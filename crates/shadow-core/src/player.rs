//! Player management - handles player state, movement, and interactions
//!
//! This module manages active player sessions and their game state,
//! bridging between the protocol layer and the game world.

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;

use shadow_protocol::codec::{NetworkMessage, Position as ProtocolPosition};
use shadow_protocol::packets::*;
use shadow_world::creature::{Creature, CreatureType, Outfit};
use shadow_world::position::{Direction, Position};
use shadow_world::tile::Tile;

use crate::Result;

/// Player session - represents an active player connection
#[derive(Debug)]
pub struct Player {
    /// Unique player ID
    pub id: Uuid,
    /// Character ID in database
    pub character_id: Uuid,
    /// Account ID
    pub account_id: Uuid,
    /// Player name
    pub name: String,
    /// Connection ID for sending packets
    pub connection_id: u64,
    /// Packet sender channel
    pub packet_tx: mpsc::Sender<NetworkMessage>,
    /// The underlying creature representation
    pub creature: Creature,
    /// Walk queue for path movement
    pub walk_queue: VecDeque<Direction>,
    /// Last step timestamp
    pub last_step: Instant,
    /// Step cooldown in milliseconds
    pub step_cooldown_ms: u32,
    /// Known creature IDs (for delta updates)
    pub known_creatures: HashMap<u32, bool>,
    /// Current map view bounds
    pub view_center: Position,
    /// Premium status
    pub premium: bool,
    /// Premium days remaining
    pub premium_days: u16,
    /// VIP list
    pub vip_list: Vec<Uuid>,
    /// Ignore list
    pub ignore_list: Vec<Uuid>,
    /// Open containers
    pub open_containers: Vec<u32>,
    /// Trade partner (if trading)
    pub trade_partner: Option<Uuid>,
    /// Last action timestamp (for exhaust)
    pub last_action: HashMap<ExhaustType, Instant>,
    /// Login time
    pub login_time: Instant,
    /// Total online time this session
    pub session_time_ms: u64,
    /// Is saving (prevent actions during save)
    pub saving: bool,
}

/// Exhaust types for action cooldowns
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExhaustType {
    Move,
    Attack,
    Spell,
    Heal,
    UseItem,
    Say,
}

impl Player {
    /// Create a new player from character data
    pub fn new(
        character_id: Uuid,
        account_id: Uuid,
        name: String,
        connection_id: u64,
        packet_tx: mpsc::Sender<NetworkMessage>,
        position: Position,
    ) -> Self {
        let mut creature = Creature::new(name.clone(), CreatureType::Player, position);
        creature.stats.health = 100;
        creature.stats.max_health = 100;
        creature.stats.mana = 50;
        creature.stats.max_mana = 50;
        creature.stats.base_speed = 220;
        creature.outfit = Outfit::with_colors(128, 78, 68, 58, 76);

        Self {
            id: Uuid::new_v4(),
            character_id,
            account_id,
            name,
            connection_id,
            packet_tx,
            creature,
            walk_queue: VecDeque::new(),
            last_step: Instant::now(),
            step_cooldown_ms: 200,
            known_creatures: HashMap::new(),
            view_center: position,
            premium: false,
            premium_days: 0,
            vip_list: Vec::new(),
            ignore_list: Vec::new(),
            open_containers: Vec::new(),
            trade_partner: None,
            last_action: HashMap::new(),
            login_time: Instant::now(),
            session_time_ms: 0,
            saving: false,
        }
    }

    /// Get current position
    pub fn position(&self) -> Position {
        self.creature.position
    }

    /// Get creature ID
    pub fn creature_id(&self) -> u32 {
        self.creature.id
    }

    /// Check if player can perform action (exhaust check)
    pub fn can_perform_action(&self, action: ExhaustType) -> bool {
        if let Some(last) = self.last_action.get(&action) {
            let cooldown = match action {
                ExhaustType::Move => Duration::from_millis(self.step_cooldown_ms as u64),
                ExhaustType::Attack => Duration::from_millis(2000),
                ExhaustType::Spell => Duration::from_millis(1000),
                ExhaustType::Heal => Duration::from_millis(1000),
                ExhaustType::UseItem => Duration::from_millis(200),
                ExhaustType::Say => Duration::from_millis(100),
            };
            return last.elapsed() >= cooldown;
        }
        true
    }

    /// Mark action as performed
    pub fn mark_action(&mut self, action: ExhaustType) {
        self.last_action.insert(action, Instant::now());
    }

    /// Try to walk in a direction
    pub fn try_walk(&mut self, direction: Direction) -> Option<Position> {
        if !self.can_perform_action(ExhaustType::Move) {
            return None;
        }

        if !self.creature.is_alive() {
            return None;
        }

        let new_pos = self.creature.position.moved(direction);
        
        // Update creature position (actual validation happens in world)
        self.creature.position = new_pos;
        self.creature.direction = direction;
        self.last_step = Instant::now();
        self.mark_action(ExhaustType::Move);

        Some(new_pos)
    }

    /// Queue a walk path
    pub fn queue_walk(&mut self, directions: Vec<Direction>) {
        self.walk_queue.clear();
        for dir in directions {
            self.walk_queue.push_back(dir);
        }
    }

    /// Cancel queued walk
    pub fn cancel_walk(&mut self) {
        self.walk_queue.clear();
    }

    /// Process next step in walk queue
    pub fn process_walk_queue(&mut self) -> Option<Direction> {
        if !self.can_perform_action(ExhaustType::Move) {
            return None;
        }
        self.walk_queue.pop_front()
    }

    /// Turn to direction
    pub fn turn(&mut self, direction: Direction) {
        self.creature.direction = direction;
    }

    /// Send a packet to this player
    pub async fn send_packet(&self, msg: NetworkMessage) -> Result<()> {
        self.packet_tx.send(msg).await
            .map_err(|_| crate::CoreError::Protocol("Failed to send packet".to_string()))?;
        Ok(())
    }

    /// Send self appearance
    pub async fn send_self_appearance(&self) -> Result<()> {
        let mut msg = NetworkMessage::new();
        // SelfAppear and LoginPending share the same opcode (0x0A) in different contexts
        msg.put_u8(ServerPacketType::LoginPending as u8);
        msg.put_u32(self.creature.id);
        msg.put_u8(0x32); // Can report bugs
        msg.put_u8(0x00); // Can change PvP framing
        msg.put_u8(0x00); // Expert mode button
        msg.put_u16(0);   // Store button URL length
        msg.put_u16(self.premium_days);

        self.send_packet(msg).await
    }

    /// Send map description around player
    pub async fn send_map_description(&self, center: Position) -> Result<()> {
        let mut msg = NetworkMessage::new();
        msg.put_u8(ServerPacketType::MapDescription as u8);
        
        // Write center position
        msg.put_u16(center.x);
        msg.put_u16(center.y);
        msg.put_u8(center.z);

        // Tile data populated from self.world via WorldRef::read().get_tiles_around(center)
        // Full implementation in GameServer::send_map_to_player() which has world access

        self.send_packet(msg).await
    }

    /// Send creature move packet
    pub async fn send_creature_move(&self, old_pos: Position, new_pos: Position) -> Result<()> {
        let mut msg = NetworkMessage::new();
        msg.put_u8(ServerPacketType::CreatureMove as u8);
        
        // Old position
        msg.put_u16(old_pos.x);
        msg.put_u16(old_pos.y);
        msg.put_u8(old_pos.z);
        msg.put_u8(1); // Old stack pos
        
        // New position
        msg.put_u16(new_pos.x);
        msg.put_u16(new_pos.y);
        msg.put_u8(new_pos.z);

        self.send_packet(msg).await
    }

    /// Send player stats
    pub async fn send_stats(&self) -> Result<()> {
        let mut msg = NetworkMessage::new();
        msg.put_u8(ServerPacketType::PlayerStats as u8);
        
        msg.put_u16(self.creature.stats.health as u16);
        msg.put_u16(self.creature.stats.max_health as u16);
        msg.put_u32(self.creature.stats.capacity);
        msg.put_u64(self.creature.stats.experience);
        msg.put_u16(self.creature.stats.level);
        msg.put_u8(0); // Level percent
        msg.put_f64(1.0); // Base XP gain
        msg.put_f64(1.0); // Low level bonus
        msg.put_f64(1.0); // XP boost
        msg.put_f64(0.0); // Stamina multiplier
        msg.put_u16(self.creature.stats.mana as u16);
        msg.put_u16(self.creature.stats.max_mana as u16);
        msg.put_u8(self.creature.stats.magic_level);
        msg.put_u8(0); // Base magic level
        msg.put_u8(self.creature.stats.magic_level_percent);
        msg.put_u8(self.creature.stats.soul);
        msg.put_u16(self.creature.stats.stamina);
        msg.put_u16(self.creature.stats.base_speed);

        self.send_packet(msg).await
    }

    /// Send skills
    pub async fn send_skills(&self) -> Result<()> {
        let mut msg = NetworkMessage::new();
        msg.put_u8(ServerPacketType::PlayerSkills as u8);

        // Send all 7 skills
        for _skill in 0..7 {
            let level = 10u16;
            let base = 10u16;
            let percent = 0u8;
            
            msg.put_u16(level);
            msg.put_u16(base);
            msg.put_u8(percent);
        }

        self.send_packet(msg).await
    }

    /// Send text message
    pub async fn send_text_message(&self, msg_type: MessageType, text: &str) -> Result<()> {
        let mut msg = NetworkMessage::new();
        msg.put_u8(ServerPacketType::TextMessage as u8);
        msg.put_u8(msg_type as u8);
        msg.put_string(text);
        
        self.send_packet(msg).await
    }

    /// Mark creature as known
    pub fn set_known_creature(&mut self, creature_id: u32) {
        self.known_creatures.insert(creature_id, true);
    }

    /// Check if creature is known
    pub fn is_known_creature(&self, creature_id: u32) -> bool {
        self.known_creatures.contains_key(&creature_id)
    }

    /// Update session time
    pub fn update_session_time(&mut self) {
        self.session_time_ms = self.login_time.elapsed().as_millis() as u64;
    }
}

/// Message types for text messages
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum MessageType {
    ConsoleBlue = 0x04,
    ConsoleRed = 0x0D,
    StatusDefault = 0x11,
    StatusWarning = 0x12,
    EventAdvance = 0x13,
    StatusSmall = 0x14,
    InfoDescription = 0x15,
    DamageDealt = 0x16,
    DamageReceived = 0x17,
    Heal = 0x18,
    Experience = 0x19,
    DamageOthers = 0x1A,
    HealOthers = 0x1B,
    ExperienceOthers = 0x1C,
    Loot = 0x1D,
    Login = 0x1E,
    Warning = 0x1F,
    GameHighlight = 0x20,
    ChannelManagement = 0x22,
    ChannelEvent = 0x27,
}

/// Player manager - handles all active players
#[derive(Debug)]
pub struct PlayerManager {
    /// Active players by player ID
    players: HashMap<Uuid, Arc<RwLock<Player>>>,
    /// Player lookup by creature ID
    by_creature_id: HashMap<u32, Uuid>,
    /// Player lookup by connection ID
    by_connection_id: HashMap<u64, Uuid>,
    /// Player lookup by name (lowercase)
    by_name: HashMap<String, Uuid>,
}

impl PlayerManager {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            by_creature_id: HashMap::new(),
            by_connection_id: HashMap::new(),
            by_name: HashMap::new(),
        }
    }

    /// Add a player
    pub fn add_player(&mut self, player: Player) -> Arc<RwLock<Player>> {
        let player_id = player.id;
        let creature_id = player.creature.id;
        let connection_id = player.connection_id;
        let name = player.name.to_lowercase();

        let player = Arc::new(RwLock::new(player));
        
        self.players.insert(player_id, player.clone());
        self.by_creature_id.insert(creature_id, player_id);
        self.by_connection_id.insert(connection_id, player_id);
        self.by_name.insert(name, player_id);

        player
    }

    /// Remove a player
    pub fn remove_player(&mut self, player_id: Uuid) -> Option<Arc<RwLock<Player>>> {
        if let Some(player) = self.players.remove(&player_id) {
            if let Ok(p) = player.try_read() {
                self.by_creature_id.remove(&p.creature.id);
                self.by_connection_id.remove(&p.connection_id);
                self.by_name.remove(&p.name.to_lowercase());
            }
            Some(player)
        } else {
            None
        }
    }

    /// Get player by ID
    pub fn get_player(&self, player_id: Uuid) -> Option<Arc<RwLock<Player>>> {
        self.players.get(&player_id).cloned()
    }

    /// Get player by creature ID
    pub fn get_by_creature_id(&self, creature_id: u32) -> Option<Arc<RwLock<Player>>> {
        self.by_creature_id.get(&creature_id)
            .and_then(|id| self.players.get(id))
            .cloned()
    }

    /// Get player by connection ID
    pub fn get_by_connection_id(&self, connection_id: u64) -> Option<Arc<RwLock<Player>>> {
        self.by_connection_id.get(&connection_id)
            .and_then(|id| self.players.get(id))
            .cloned()
    }

    /// Get player by name
    pub fn get_by_name(&self, name: &str) -> Option<Arc<RwLock<Player>>> {
        self.by_name.get(&name.to_lowercase())
            .and_then(|id| self.players.get(id))
            .cloned()
    }

    /// Get all players
    pub fn get_all_players(&self) -> Vec<Arc<RwLock<Player>>> {
        self.players.values().cloned().collect()
    }

    /// Get player count
    pub fn player_count(&self) -> usize {
        self.players.len()
    }

    /// Get players in range of a position
    pub async fn get_players_in_range(&self, center: Position, range: u32) -> Vec<Arc<RwLock<Player>>> {
        let mut result = Vec::new();
        for player in self.players.values() {
            let p = player.read().await;
            if p.position().in_range(&center, range) {
                result.push(player.clone());
            }
        }
        result
    }
}

impl Default for PlayerManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Movement handler - processes player movement requests
pub struct MovementHandler;

impl MovementHandler {
    /// Handle a movement request from a player
    pub async fn handle_move(
        player: &mut Player,
        direction: Direction,
        world: &crate::WorldRef,
    ) -> Result<Option<Position>> {
        // Check if player can move
        if !player.can_perform_action(ExhaustType::Move) {
            return Ok(None);
        }

        let current_pos = player.position();
        let new_pos = current_pos.moved(direction);

        // Check if target tile is walkable
        // This would check the actual world data
        // For now, we'll assume it's valid

        // Update player position
        player.creature.position = new_pos;
        player.creature.direction = direction;
        player.mark_action(ExhaustType::Move);

        // Send movement packet to player
        player.send_creature_move(current_pos, new_pos).await?;

        Ok(Some(new_pos))
    }

    /// Process walk queue for a player
    pub async fn process_walk_queue(
        player: &mut Player,
        world: &crate::WorldRef,
    ) -> Result<()> {
        while let Some(direction) = player.process_walk_queue() {
            if Self::handle_move(player, direction, world).await?.is_none() {
                // Movement failed, clear remaining queue
                player.cancel_walk();
                break;
            }
        }
        Ok(())
    }
}

/// World reference type for map and entity access
pub type WorldRef = Arc<RwLock<shadow_world::Map>>;
