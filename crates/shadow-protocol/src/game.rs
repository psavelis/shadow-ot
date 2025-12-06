//! Game server implementation - handles in-game protocol

use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, RwLock};

use crate::codec::{NetworkMessage, Position, TibiaCodec};
use crate::packets::*;
use crate::version::ProtocolVersion;
use crate::{ProtocolError, Result};

/// Game server handling in-game connections
pub struct GameServer {
    listener: TcpListener,
    connection_tx: mpsc::Sender<GameConnection>,
}

/// Represents an active game connection
pub struct GameConnection {
    pub id: u64,
    pub addr: SocketAddr,
    pub player_id: Option<uuid::Uuid>,
    pub character_id: Option<uuid::Uuid>,
    pub protocol_version: ProtocolVersion,
    pub xtea_key: [u32; 4],
    pub packet_tx: mpsc::Sender<NetworkMessage>,
}

/// Events emitted by game connections
#[derive(Debug)]
pub enum GameEvent {
    Connected(u64, SocketAddr),
    Disconnected(u64),
    Packet(u64, ClientPacketType, NetworkMessage),
    Error(u64, String),
}

impl GameServer {
    /// Create a new game server
    pub async fn bind(addr: &str) -> Result<(Self, mpsc::Receiver<GameConnection>)> {
        let listener = TcpListener::bind(addr)
            .await
            .map_err(|e| ProtocolError::Connection(e.to_string()))?;

        let (connection_tx, connection_rx) = mpsc::channel(1000);

        tracing::info!("Game server listening on {}", addr);

        Ok((Self { listener, connection_tx }, connection_rx))
    }

    /// Run the game server accept loop
    pub async fn run(&self) -> Result<()> {
        let mut connection_id: u64 = 0;

        loop {
            match self.listener.accept().await {
                Ok((socket, addr)) => {
                    connection_id += 1;
                    let tx = self.connection_tx.clone();

                    tokio::spawn(async move {
                        if let Err(e) = handle_game_connection(socket, addr, connection_id, tx).await
                        {
                            tracing::error!(
                                "Game connection error from {} (id: {}): {}",
                                addr,
                                connection_id,
                                e
                            );
                        }
                    });
                }
                Err(e) => {
                    tracing::error!("Failed to accept game connection: {}", e);
                }
            }
        }
    }
}

/// Handle a single game connection
async fn handle_game_connection(
    socket: TcpStream,
    addr: SocketAddr,
    connection_id: u64,
    connection_tx: mpsc::Sender<GameConnection>,
) -> Result<()> {
    tracing::debug!("New game connection from {} (id: {})", addr, connection_id);

    // Set TCP_NODELAY for minimal latency (critical for game server)
    socket.set_nodelay(true).ok();

    // Set socket keepalive
    let socket_ref = socket2::SockRef::from(&socket);
    let keepalive = socket2::TcpKeepalive::new()
        .with_time(std::time::Duration::from_secs(60))
        .with_interval(std::time::Duration::from_secs(10));
    socket_ref.set_tcp_keepalive(&keepalive).ok();

    // Create channel for sending packets to this connection
    let (packet_tx, mut packet_rx) = mpsc::channel::<NetworkMessage>(100);

    // Split socket for concurrent read/write
    let (read_half, write_half) = socket.into_split();

    // Create codec
    let version = ProtocolVersion::default();
    let codec = TibiaCodec::new(version);

    // Initial connection state (player_id/character_id assigned after login)
    let connection = GameConnection {
        id: connection_id,
        addr,
        player_id: None,
        character_id: None,
        protocol_version: version,
        xtea_key: [0; 4],
        packet_tx: packet_tx.clone(),
    };

    // Notify about new connection
    connection_tx.send(connection).await.ok();

    // Spawn writer task
    let writer_handle = tokio::spawn(async move {
        use futures::SinkExt;
        use tokio_util::codec::FramedWrite;

        let mut framed_write = FramedWrite::new(write_half, codec);

        while let Some(msg) = packet_rx.recv().await {
            if let Err(e) = framed_write.send(msg).await {
                tracing::error!("Error sending packet to {}: {}", addr, e);
                break;
            }
        }
    });

    // Reader loop
    use futures::StreamExt;
    use tokio_util::codec::FramedRead;

    let read_codec = TibiaCodec::new(version);
    let mut framed_read = FramedRead::new(read_half, read_codec);

    while let Some(result) = framed_read.next().await {
        match result {
            Ok(mut msg) => {
                if let Err(e) = process_game_packet(&mut msg, connection_id, &packet_tx).await {
                    tracing::error!(
                        "Error processing packet from {} (id: {}): {}",
                        addr,
                        connection_id,
                        e
                    );
                }
            }
            Err(e) => {
                tracing::error!(
                    "Error reading packet from {} (id: {}): {}",
                    addr,
                    connection_id,
                    e
                );
                break;
            }
        }
    }

    // Cleanup
    writer_handle.abort();
    tracing::debug!("Game connection closed from {} (id: {})", addr, connection_id);

    Ok(())
}

/// Process an incoming game packet
async fn process_game_packet(
    msg: &mut NetworkMessage,
    connection_id: u64,
    packet_tx: &mpsc::Sender<NetworkMessage>,
) -> Result<()> {
    let opcode = msg.get_u8()?;
    let packet_type = ClientPacketType::from(opcode);

    match packet_type {
        ClientPacketType::Ping => {
            // Respond with ping back
            let mut response = NetworkMessage::new();
            response.put_u8(ServerPacketType::PingBack as u8);
            packet_tx.send(response).await.ok();
        }

        ClientPacketType::PingBack => {
            // Client responded to our ping, update latency measurement
        }

        ClientPacketType::Logout => {
            tracing::debug!("Player logout request from connection {}", connection_id);
            // Handle logout
        }

        // Movement packets
        ClientPacketType::MoveNorth
        | ClientPacketType::MoveEast
        | ClientPacketType::MoveSouth
        | ClientPacketType::MoveWest
        | ClientPacketType::MoveNorthEast
        | ClientPacketType::MoveSouthEast
        | ClientPacketType::MoveSouthWest
        | ClientPacketType::MoveNorthWest => {
            if let Some(direction) = Direction::from_packet_type(packet_type) {
                // Process movement
                tracing::trace!(
                    "Move {:?} from connection {}",
                    direction,
                    connection_id
                );
            }
        }

        ClientPacketType::StopMove => {
            // Cancel walk queue
        }

        // Turn packets
        ClientPacketType::TurnNorth
        | ClientPacketType::TurnEast
        | ClientPacketType::TurnSouth
        | ClientPacketType::TurnWest => {
            // Process turn
        }

        // Item operations
        ClientPacketType::MoveItem => {
            let packet = MoveItemPacket::parse(msg)?;
            tracing::trace!(
                "Move item from {:?} to {:?} (count: {})",
                packet.from_position,
                packet.to_position,
                packet.count
            );
        }

        ClientPacketType::UseItem => {
            let packet = UseItemPacket::parse(msg)?;
            tracing::trace!("Use item at {:?}", packet.position);
        }

        ClientPacketType::UseItemEx => {
            let packet = UseItemExPacket::parse(msg)?;
            tracing::trace!(
                "Use item ex from {:?} to {:?}",
                packet.from_position,
                packet.to_position
            );
        }

        ClientPacketType::LookAt => {
            let packet = LookAtPacket::parse(msg)?;
            tracing::trace!("Look at {:?}", packet.position);
        }

        // Chat
        ClientPacketType::Say => {
            let packet = SayPacket::parse(msg)?;
            tracing::debug!(
                "Chat {:?}: {}",
                packet.message_type,
                packet.text
            );
        }

        // Combat
        ClientPacketType::Attack => {
            let packet = AttackPacket::parse(msg)?;
            tracing::trace!("Attack creature {}", packet.creature_id);
        }

        ClientPacketType::Follow => {
            let creature_id = msg.get_u32()?;
            tracing::trace!("Follow creature {}", creature_id);
        }

        ClientPacketType::CancelAttack => {
            tracing::trace!("Cancel attack");
        }

        ClientPacketType::SetModes => {
            let packet = SetModesPacket::parse(msg)?;
            tracing::trace!(
                "Set modes: fight={:?}, chase={:?}, secure={}, pvp={:?}",
                packet.fight_mode,
                packet.chase_mode,
                packet.secure_mode,
                packet.pvp_mode
            );
        }

        // Channel operations
        ClientPacketType::ChannelList => {
            // Send channel list
        }

        ClientPacketType::OpenChannel => {
            let channel_id = msg.get_u16()?;
            tracing::trace!("Open channel {}", channel_id);
        }

        ClientPacketType::CloseChannel => {
            let channel_id = msg.get_u16()?;
            tracing::trace!("Close channel {}", channel_id);
        }

        // Party
        ClientPacketType::PartyInvite => {
            let creature_id = msg.get_u32()?;
            tracing::trace!("Party invite to creature {}", creature_id);
        }

        ClientPacketType::PartyJoin => {
            let creature_id = msg.get_u32()?;
            tracing::trace!("Party join from creature {}", creature_id);
        }

        ClientPacketType::PartyLeave => {
            tracing::trace!("Party leave");
        }

        // Trade
        ClientPacketType::TradeRequest => {
            let pos = Position::read(msg)?;
            let sprite_id = msg.get_u16()?;
            let stack_pos = msg.get_u8()?;
            let creature_id = msg.get_u32()?;
            tracing::trace!(
                "Trade request item at {:?} with creature {}",
                pos,
                creature_id
            );
        }

        ClientPacketType::TradeAccept | ClientPacketType::TradeClose => {
            // Handle trade accept/close
        }

        // Market
        ClientPacketType::MarketBrowse => {
            let category = msg.get_u16()?;
            tracing::trace!("Market browse category {}", category);
        }

        ClientPacketType::MarketCreate => {
            let offer_type = msg.get_u8()?;
            let item_id = msg.get_u16()?;
            let amount = msg.get_u16()?;
            let price = msg.get_u32()?;
            let is_anonymous = msg.get_u8()? != 0;
            tracing::trace!(
                "Market create: type={}, item={}, amount={}, price={}",
                offer_type,
                item_id,
                amount,
                price
            );
        }

        // Outfit
        ClientPacketType::Outfit => {
            let looktype = msg.get_u16()?;
            let head = msg.get_u8()?;
            let body = msg.get_u8()?;
            let legs = msg.get_u8()?;
            let feet = msg.get_u8()?;
            let addons = msg.get_u8()?;
            tracing::trace!(
                "Change outfit: type={}, colors={}/{}/{}/{}, addons={}",
                looktype,
                head,
                body,
                legs,
                feet,
                addons
            );
        }

        ClientPacketType::Mount => {
            let mount_id = msg.get_u16()?;
            tracing::trace!("Mount/unmount: {}", mount_id);
        }

        // Quest
        ClientPacketType::QuestLog => {
            // Send quest log
        }

        ClientPacketType::QuestLine => {
            let quest_id = msg.get_u16()?;
            tracing::trace!("Quest line for quest {}", quest_id);
        }

        // Bestiary (modern clients)
        ClientPacketType::BestiaryInfo => {
            let creature_id = msg.get_u16()?;
            tracing::trace!("Bestiary info for creature {}", creature_id);
        }

        // Prey (modern clients)
        ClientPacketType::PreyAction => {
            let slot = msg.get_u8()?;
            let action = msg.get_u8()?;
            tracing::trace!("Prey action: slot={}, action={}", slot, action);
        }

        // Store
        ClientPacketType::StoreOpen => {
            // Open store
        }

        ClientPacketType::StoreBrowse => {
            let category = msg.get_string()?;
            tracing::trace!("Store browse category: {}", category);
        }

        ClientPacketType::StoreBuy => {
            let offer_id = msg.get_u32()?;
            let product_type = msg.get_u8()?;
            tracing::trace!("Store buy offer {} type {}", offer_id, product_type);
        }

        // VIP
        ClientPacketType::AddBuddy => {
            let name = msg.get_string()?;
            tracing::trace!("Add buddy: {}", name);
        }

        ClientPacketType::RemoveBuddy => {
            let buddy_id = msg.get_u32()?;
            tracing::trace!("Remove buddy: {}", buddy_id);
        }

        _ => {
            tracing::debug!(
                "Unhandled packet type 0x{:02X} from connection {}",
                opcode,
                connection_id
            );
        }
    }

    Ok(())
}

/// Build and send map description to client
pub fn build_map_description(
    center: Position,
    width: u8,
    height: u8,
) -> NetworkMessage {
    let mut msg = NetworkMessage::new();
    msg.put_u8(ServerPacketType::MapDescription as u8);

    center.write(&mut msg);

    // Map tiles are populated by the caller via shadow_world::Map::get_tiles_around()
    // The caller adds tile data after receiving this base packet structure

    msg
}

/// Build creature spawn packet
pub fn build_creature_spawn(
    creature_id: u32,
    position: Position,
    name: &str,
    health_percent: u8,
    direction: Direction,
    outfit: &Outfit,
) -> NetworkMessage {
    let mut msg = NetworkMessage::new();
    msg.put_u8(ServerPacketType::TileAddThing as u8);

    position.write(&mut msg);

    // Creature data
    msg.put_u8(0x61); // New creature marker
    msg.put_u32(0); // Known creature removal
    msg.put_u32(creature_id);
    msg.put_u8(0x00); // Creature type (player)
    msg.put_string(name);
    msg.put_u8(health_percent);
    msg.put_u8(direction as u8);

    // Outfit
    outfit.write(&mut msg);

    // Light
    msg.put_u8(0); // Light level
    msg.put_u8(0); // Light color

    // Speed
    msg.put_u16(220); // Base speed

    // Skull and shield
    msg.put_u8(0); // Skull
    msg.put_u8(0); // Party shield

    // Guild emblem (if applicable)
    msg.put_u8(0);

    // Creature type flags
    msg.put_u8(0); // Not unpassable
    msg.put_u8(0); // Speech bubble

    // Mark (for modern clients)
    msg.put_u8(0xFF);

    // Helpers (party)
    msg.put_u16(0);

    msg
}

/// Outfit data structure
#[derive(Debug, Clone, Default)]
pub struct Outfit {
    pub look_type: u16,
    pub look_head: u8,
    pub look_body: u8,
    pub look_legs: u8,
    pub look_feet: u8,
    pub look_addons: u8,
    pub look_mount: u16,
}

impl Outfit {
    pub fn write(&self, msg: &mut NetworkMessage) {
        msg.put_u16(self.look_type);
        if self.look_type != 0 {
            msg.put_u8(self.look_head);
            msg.put_u8(self.look_body);
            msg.put_u8(self.look_legs);
            msg.put_u8(self.look_feet);
            msg.put_u8(self.look_addons);
        }
        msg.put_u16(self.look_mount);
    }

    pub fn read(msg: &mut NetworkMessage) -> Result<Self> {
        let look_type = msg.get_u16()?;
        let (look_head, look_body, look_legs, look_feet, look_addons) = if look_type != 0 {
            (
                msg.get_u8()?,
                msg.get_u8()?,
                msg.get_u8()?,
                msg.get_u8()?,
                msg.get_u8()?,
            )
        } else {
            (0, 0, 0, 0, 0)
        };
        let look_mount = msg.get_u16()?;

        Ok(Self {
            look_type,
            look_head,
            look_body,
            look_legs,
            look_feet,
            look_addons,
            look_mount,
        })
    }
}

/// Build player stats packet
pub fn build_player_stats(
    health: u32,
    max_health: u32,
    free_capacity: u32,
    total_capacity: u32,
    experience: u64,
    level: u16,
    level_percent: u8,
    mana: u32,
    max_mana: u32,
    magic_level: u8,
    magic_level_base: u8,
    magic_level_percent: u8,
    soul: u8,
    stamina: u16,
    speed: u16,
    fed: u16,
    offline_training_time: u16,
    exp_bonus: u16,
) -> NetworkMessage {
    let mut msg = NetworkMessage::new();
    msg.put_u8(ServerPacketType::PlayerStats as u8);

    msg.put_u16(health as u16);
    msg.put_u16(max_health as u16);
    msg.put_u32(free_capacity);
    msg.put_u32(total_capacity);
    msg.put_u64(experience);
    msg.put_u16(level);
    msg.put_u8(level_percent);
    msg.put_u16(100); // Experience bonus rate
    msg.put_u16(mana as u16);
    msg.put_u16(max_mana as u16);
    msg.put_u8(magic_level);
    msg.put_u8(magic_level_base);
    msg.put_u8(magic_level_percent);
    msg.put_u8(soul);
    msg.put_u16(stamina);
    msg.put_u16(speed);
    msg.put_u16(fed);
    msg.put_u16(offline_training_time);
    msg.put_u16(exp_bonus);
    msg.put_u16(0); // Store coin balance
    msg.put_u16(0); // Store bonus coins

    msg
}

/// Build text message packet
pub fn build_text_message(message_type: TextMessageType, text: &str) -> NetworkMessage {
    let mut msg = NetworkMessage::new();
    msg.put_u8(ServerPacketType::TextMessage as u8);
    msg.put_u8(message_type as u8);
    msg.put_string(text);
    msg
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum TextMessageType {
    // Console messages
    ConsoleRed = 18,
    ConsoleOrange = 19,
    ConsoleOrange2 = 20,
    Warning = 21,
    EventAdvance = 22,
    EventDefault = 23,
    StatusDefault = 24,
    InfoDescription = 25,
    StatusSmall = 26,
    ConsoleBlue = 27,

    // Game messages
    Damage = 17,
    Healing = 18,
    Experience = 19,
    DamageDealt = 20,
    DamageReceived = 21,
    DamageOther = 22,
    HealingOther = 23,
    ExperienceOther = 24,
    Loot = 31,
    Login = 28,
    Guild = 29,
}
