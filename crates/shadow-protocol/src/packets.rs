//! Tibia protocol packet definitions
//!
//! Complete packet definitions for all Tibia protocol versions,
//! supporting features from 8.6 to 12.x+

use serde::{Deserialize, Serialize};
use crate::codec::{NetworkMessage, Position};
use crate::{ProtocolError, Result};

// ============================================================================
// LOGIN SERVER PACKETS
// ============================================================================

/// Client -> Server: Initial login request
#[derive(Debug, Clone)]
pub struct LoginPacket {
    pub protocol_version: u16,
    pub client_version: u16,
    pub dat_signature: u32,
    pub spr_signature: u32,
    pub pic_signature: u32,
    pub preview_state: u8,
    pub xtea_key: [u32; 4],
    pub account_name: String,
    pub password: String,
    pub auth_token: Option<String>,
    pub stay_logged_in: bool,
}

impl LoginPacket {
    pub fn parse(msg: &mut NetworkMessage) -> Result<Self> {
        let protocol_version = msg.get_u16()?;
        let client_version = msg.get_u16()?;
        let dat_signature = msg.get_u32()?;
        let spr_signature = msg.get_u32()?;
        let pic_signature = msg.get_u32()?;
        let preview_state = msg.get_u8()?;

        let xtea_key = [
            msg.get_u32()?,
            msg.get_u32()?,
            msg.get_u32()?,
            msg.get_u32()?,
        ];

        let account_name = msg.get_string()?;
        let password = msg.get_string()?;

        let auth_token = if msg.remaining() > 0 {
            Some(msg.get_string()?)
        } else {
            None
        };

        let stay_logged_in = if msg.remaining() > 0 {
            msg.get_u8()? != 0
        } else {
            false
        };

        Ok(Self {
            protocol_version,
            client_version,
            dat_signature,
            spr_signature,
            pic_signature,
            preview_state,
            xtea_key,
            account_name,
            password,
            auth_token,
            stay_logged_in,
        })
    }
}

/// Server -> Client: Login error
#[derive(Debug, Clone)]
pub struct LoginErrorPacket {
    pub message: String,
}

impl LoginErrorPacket {
    pub fn write(&self, msg: &mut NetworkMessage) {
        msg.put_u8(0x0A);
        msg.put_string(&self.message);
    }
}

/// Server -> Client: Character list
#[derive(Debug, Clone)]
pub struct CharacterListPacket {
    pub motd: Option<String>,
    pub session_key: String,
    pub characters: Vec<CharacterEntry>,
    pub premium_days: u16,
    pub premium_until: u32,
}

#[derive(Debug, Clone)]
pub struct CharacterEntry {
    pub name: String,
    pub realm: String,
    pub realm_host: String,
    pub realm_port: u16,
    pub preview_state: u8,
}

impl CharacterListPacket {
    pub fn write(&self, msg: &mut NetworkMessage) {
        // MOTD
        if let Some(ref motd) = self.motd {
            msg.put_u8(0x14);
            msg.put_string(motd);
        }

        // Session key
        msg.put_u8(0x28);
        msg.put_string(&self.session_key);

        // Character list
        msg.put_u8(0x64);
        msg.put_u8(self.characters.len() as u8);

        for char in &self.characters {
            msg.put_string(&char.realm);
            msg.put_string(&char.realm_host);
            msg.put_u16(char.realm_port);
            msg.put_u8(char.preview_state);
        }

        msg.put_u8(self.characters.len() as u8);
        for char in &self.characters {
            msg.put_string(&char.name);
        }

        // Premium
        msg.put_u8(0x00); // No premium type flag for now
        msg.put_u16(self.premium_days);
    }
}

// ============================================================================
// GAME SERVER PACKETS - CLIENT TO SERVER
// ============================================================================

/// Packet opcodes from client
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ClientPacketType {
    Logout = 0x14,
    Ping = 0x1D,
    PingBack = 0x1E,

    // Movement
    MoveNorth = 0x65,
    MoveEast = 0x66,
    MoveSouth = 0x67,
    MoveWest = 0x68,
    StopMove = 0x69,
    MoveNorthEast = 0x6A,
    MoveSouthEast = 0x6B,
    MoveSouthWest = 0x6C,
    MoveNorthWest = 0x6D,

    // Turns
    TurnNorth = 0x6F,
    TurnEast = 0x70,
    TurnSouth = 0x71,
    TurnWest = 0x72,

    // Item operations
    MoveItem = 0x78,
    LookAt = 0x8C,
    LookAtCreature = 0x8D,
    UseItem = 0x82,
    UseItemEx = 0x83,
    UseOnCreature = 0x84,
    RotateItem = 0x85,

    // Container
    CloseContainer = 0x87,
    UpContainer = 0x88,
    BrowseField = 0xCB,

    // Chat
    Say = 0x96,
    ChannelList = 0x97,
    OpenChannel = 0x98,
    CloseChannel = 0x99,
    PrivateMessage = 0x9A,

    // Combat
    Attack = 0xA1,
    Follow = 0xA2,
    CancelAttack = 0xBE,
    SetModes = 0xA0,

    // Party
    PartyInvite = 0xA3,
    PartyJoin = 0xA4,
    PartyRevokeInvite = 0xA5,
    PartyPassLeadership = 0xA6,
    PartyLeave = 0xA7,
    PartyShareExp = 0xA8,

    // Trade
    TradeRequest = 0x7D,
    TradeInspect = 0x7E,
    TradeAccept = 0x7F,
    TradeClose = 0x80,

    // Market
    MarketLeave = 0xF4,
    MarketBrowse = 0xF5,
    MarketCreate = 0xF6,
    MarketCancel = 0xF7,
    MarketAccept = 0xF8,

    // Quest
    QuestLog = 0xF0,
    QuestLine = 0xF1,

    // Other
    Outfit = 0xD3,
    Mount = 0xD4,
    AddBuddy = 0xDC,
    RemoveBuddy = 0xDD,
    EditBuddy = 0xDE,
    BugReport = 0xE6,
    ViolationReport = 0xE7,
    RuleViolation = 0xF2,

    // Bestiary & Prey (modern versions)
    BestiaryInfo = 0xD5,
    PreyAction = 0xEB,

    // Store
    StoreOpen = 0xFA,
    StoreBrowse = 0xFB,
    StoreBuy = 0xFC,

    Unknown = 0xFF,
}

impl From<u8> for ClientPacketType {
    fn from(value: u8) -> Self {
        match value {
            0x14 => Self::Logout,
            0x1D => Self::Ping,
            0x1E => Self::PingBack,
            0x65 => Self::MoveNorth,
            0x66 => Self::MoveEast,
            0x67 => Self::MoveSouth,
            0x68 => Self::MoveWest,
            0x69 => Self::StopMove,
            0x6A => Self::MoveNorthEast,
            0x6B => Self::MoveSouthEast,
            0x6C => Self::MoveSouthWest,
            0x6D => Self::MoveNorthWest,
            0x6F => Self::TurnNorth,
            0x70 => Self::TurnEast,
            0x71 => Self::TurnSouth,
            0x72 => Self::TurnWest,
            0x78 => Self::MoveItem,
            0x82 => Self::UseItem,
            0x83 => Self::UseItemEx,
            0x84 => Self::UseOnCreature,
            0x85 => Self::RotateItem,
            0x87 => Self::CloseContainer,
            0x88 => Self::UpContainer,
            0x8C => Self::LookAt,
            0x8D => Self::LookAtCreature,
            0x96 => Self::Say,
            0x97 => Self::ChannelList,
            0x98 => Self::OpenChannel,
            0x99 => Self::CloseChannel,
            0x9A => Self::PrivateMessage,
            0xA0 => Self::SetModes,
            0xA1 => Self::Attack,
            0xA2 => Self::Follow,
            0xA3 => Self::PartyInvite,
            0xA4 => Self::PartyJoin,
            0xA5 => Self::PartyRevokeInvite,
            0xA6 => Self::PartyPassLeadership,
            0xA7 => Self::PartyLeave,
            0xA8 => Self::PartyShareExp,
            0x7D => Self::TradeRequest,
            0x7E => Self::TradeInspect,
            0x7F => Self::TradeAccept,
            0x80 => Self::TradeClose,
            0xBE => Self::CancelAttack,
            0xCB => Self::BrowseField,
            0xD3 => Self::Outfit,
            0xD4 => Self::Mount,
            0xD5 => Self::BestiaryInfo,
            0xDC => Self::AddBuddy,
            0xDD => Self::RemoveBuddy,
            0xDE => Self::EditBuddy,
            0xE6 => Self::BugReport,
            0xE7 => Self::ViolationReport,
            0xEB => Self::PreyAction,
            0xF0 => Self::QuestLog,
            0xF1 => Self::QuestLine,
            0xF2 => Self::RuleViolation,
            0xF4 => Self::MarketLeave,
            0xF5 => Self::MarketBrowse,
            0xF6 => Self::MarketCreate,
            0xF7 => Self::MarketCancel,
            0xF8 => Self::MarketAccept,
            0xFA => Self::StoreOpen,
            0xFB => Self::StoreBrowse,
            0xFC => Self::StoreBuy,
            _ => Self::Unknown,
        }
    }
}

// ============================================================================
// GAME SERVER PACKETS - SERVER TO CLIENT
// ============================================================================

/// Packet opcodes from server
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ServerPacketType {
    // Login
    LoginPending = 0x0A,
    LoginError = 0x14,
    LoginAdvice = 0x15,
    LoginWait = 0x16,
    LoginSuccess = 0x17,
    LoginToken = 0x0B,
    Challenge = 0x1F,

    // Game
    // SelfAppear uses the same opcode as LoginPending (0x0A) in different contexts
    Ping = 0x1D,
    PingBack = 0x1E,

    // Map
    MapDescription = 0x64,
    MapNorth = 0x65,
    MapEast = 0x66,
    MapSouth = 0x67,
    MapWest = 0x68,

    // Tile updates
    TileUpdate = 0x69,
    TileAddThing = 0x6A,
    TileTransformThing = 0x6B,
    TileRemoveThing = 0x6C,

    // Creature
    CreatureMove = 0x6D,
    ContainerOpen = 0x6E,
    ContainerClose = 0x6F,
    ContainerAddItem = 0x70,
    ContainerUpdateItem = 0x71,
    ContainerRemoveItem = 0x72,

    // Inventory
    InventorySetSlot = 0x78,
    InventoryClearSlot = 0x79,

    // NPC Trade
    NpcOpenTrade = 0x7A,
    NpcPlayerGoods = 0x7B,
    NpcCloseTrade = 0x7C,

    // Player Trade
    TradeOwn = 0x7D,
    TradeCounter = 0x7E,
    TradeClose = 0x7F,

    // Ambient
    AmbientLight = 0x82,
    GraphicEffect = 0x83,
    TextEffect = 0x84,
    MissileEffect = 0x85,

    // Creature updates
    CreatureHealth = 0x8C,
    CreatureLight = 0x8D,
    CreatureOutfit = 0x8E,
    CreatureSpeed = 0x8F,
    CreatureSkull = 0x90,
    CreatureShield = 0x91,
    CreatureUnpass = 0x92,
    CreatureMarks = 0x93,
    CreatureHelpers = 0x94,
    CreatureType = 0x95,

    // Player
    PlayerStats = 0xA0,
    PlayerSkills = 0xA1,
    PlayerIcons = 0xA2,
    PlayerCancelTarget = 0xA3,
    PlayerSpellDelay = 0xA4,
    PlayerSpellGroupDelay = 0xA5,
    PlayerMultiUseDelay = 0xA6,

    // Chat
    Talk = 0xAA,
    ChannelList = 0xAB,
    OpenChannel = 0xAC,
    OpenPrivateChannel = 0xAD,
    RuleViolationChannel = 0xAE,
    RuleViolationRemove = 0xAF,
    RuleViolationCancel = 0xB0,
    RuleViolationLock = 0xB1,
    OpenOwnChannel = 0xB2,
    CloseChannel = 0xB3,
    TextMessage = 0xB4,
    CancelWalk = 0xB5,
    WalkWait = 0xB6,

    // Floor change
    FloorChangeUp = 0xBE,
    FloorChangeDown = 0xBF,

    // Outfit window
    OutfitWindow = 0xC8,

    // VIP
    VipAdd = 0xD2,
    VipState = 0xD3,
    VipLogout = 0xD4,

    // Quest
    QuestLog = 0xF0,
    QuestLine = 0xF1,

    // Market
    MarketEnter = 0xF6,
    MarketLeave = 0xF7,
    MarketBrowse = 0xF8,
    MarketDetail = 0xF9,

    // Store
    StoreCategories = 0xFB,
    StoreOffers = 0xFC,
    StoreTransactionHistory = 0xFD,
    StoreSuccess = 0xFE,
    StoreError = 0xE0,

    Unknown = 0xFF,
}

// ============================================================================
// MOVEMENT PACKETS
// ============================================================================

#[derive(Debug, Clone, Copy)]
pub struct MovePacket {
    pub direction: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
    NorthEast = 4,
    SouthEast = 5,
    SouthWest = 6,
    NorthWest = 7,
}

impl Direction {
    pub fn from_packet_type(packet_type: ClientPacketType) -> Option<Self> {
        match packet_type {
            ClientPacketType::MoveNorth => Some(Self::North),
            ClientPacketType::MoveEast => Some(Self::East),
            ClientPacketType::MoveSouth => Some(Self::South),
            ClientPacketType::MoveWest => Some(Self::West),
            ClientPacketType::MoveNorthEast => Some(Self::NorthEast),
            ClientPacketType::MoveSouthEast => Some(Self::SouthEast),
            ClientPacketType::MoveSouthWest => Some(Self::SouthWest),
            ClientPacketType::MoveNorthWest => Some(Self::NorthWest),
            _ => None,
        }
    }

    pub fn offset(&self) -> (i32, i32) {
        match self {
            Self::North => (0, -1),
            Self::East => (1, 0),
            Self::South => (0, 1),
            Self::West => (-1, 0),
            Self::NorthEast => (1, -1),
            Self::SouthEast => (1, 1),
            Self::SouthWest => (-1, 1),
            Self::NorthWest => (-1, -1),
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
            Self::NorthEast => Self::SouthWest,
            Self::SouthEast => Self::NorthWest,
            Self::SouthWest => Self::NorthEast,
            Self::NorthWest => Self::SouthEast,
        }
    }
}

// ============================================================================
// CHAT PACKETS
// ============================================================================

#[derive(Debug, Clone)]
pub struct SayPacket {
    pub message_type: MessageType,
    pub channel_id: u16,
    pub receiver: String,
    pub text: String,
}

impl SayPacket {
    pub fn parse(msg: &mut NetworkMessage) -> Result<Self> {
        let message_type = MessageType::from(msg.get_u8()?);
        let channel_id = match message_type {
            MessageType::Channel | MessageType::ChannelManagement => msg.get_u16()?,
            _ => 0,
        };
        let receiver = match message_type {
            MessageType::Private | MessageType::PrivateRed | MessageType::GamemasterPrivate => {
                msg.get_string()?
            }
            _ => String::new(),
        };
        let text = msg.get_string()?;

        Ok(Self {
            message_type,
            channel_id,
            receiver,
            text,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum MessageType {
    Say = 1,
    Whisper = 2,
    Yell = 3,
    Private = 4,
    Channel = 5,
    ChannelManagement = 6,
    GamemasterBroadcast = 9,
    GamemasterChannel = 10,
    GamemasterPrivate = 11,
    PrivateRed = 12,
    MonsterSay = 36,
    MonsterYell = 37,
    NpcFrom = 38,
    NpcFromBlock = 39,
}

impl From<u8> for MessageType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Say,
            2 => Self::Whisper,
            3 => Self::Yell,
            4 => Self::Private,
            5 => Self::Channel,
            6 => Self::ChannelManagement,
            9 => Self::GamemasterBroadcast,
            10 => Self::GamemasterChannel,
            11 => Self::GamemasterPrivate,
            12 => Self::PrivateRed,
            36 => Self::MonsterSay,
            37 => Self::MonsterYell,
            38 => Self::NpcFrom,
            39 => Self::NpcFromBlock,
            _ => Self::Say,
        }
    }
}

// ============================================================================
// COMBAT PACKETS
// ============================================================================

#[derive(Debug, Clone, Copy)]
pub struct AttackPacket {
    pub creature_id: u32,
    pub sequence: u32,
}

impl AttackPacket {
    pub fn parse(msg: &mut NetworkMessage) -> Result<Self> {
        Ok(Self {
            sequence: msg.get_u32()?,
            creature_id: msg.get_u32()?,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SetModesPacket {
    pub fight_mode: FightMode,
    pub chase_mode: ChaseMode,
    pub secure_mode: bool,
    pub pvp_mode: PvpMode,
}

impl SetModesPacket {
    pub fn parse(msg: &mut NetworkMessage) -> Result<Self> {
        Ok(Self {
            fight_mode: FightMode::from(msg.get_u8()?),
            chase_mode: ChaseMode::from(msg.get_u8()?),
            secure_mode: msg.get_u8()? != 0,
            pvp_mode: PvpMode::from(msg.get_u8()?),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum FightMode {
    Offensive = 1,
    Balanced = 2,
    Defensive = 3,
}

impl From<u8> for FightMode {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Offensive,
            2 => Self::Balanced,
            3 => Self::Defensive,
            _ => Self::Balanced,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum ChaseMode {
    Stand = 0,
    Chase = 1,
}

impl From<u8> for ChaseMode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Stand,
            1 => Self::Chase,
            _ => Self::Stand,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum PvpMode {
    Dove = 0,
    WhiteHand = 1,
    YellowHand = 2,
    RedFist = 3,
}

impl From<u8> for PvpMode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Dove,
            1 => Self::WhiteHand,
            2 => Self::YellowHand,
            3 => Self::RedFist,
            _ => Self::Dove,
        }
    }
}

// ============================================================================
// ITEM PACKETS
// ============================================================================

#[derive(Debug, Clone)]
pub struct MoveItemPacket {
    pub from_position: Position,
    pub from_sprite_id: u16,
    pub from_stack_pos: u8,
    pub to_position: Position,
    pub count: u8,
}

impl MoveItemPacket {
    pub fn parse(msg: &mut NetworkMessage) -> Result<Self> {
        Ok(Self {
            from_position: Position::read(msg)?,
            from_sprite_id: msg.get_u16()?,
            from_stack_pos: msg.get_u8()?,
            to_position: Position::read(msg)?,
            count: msg.get_u8()?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct UseItemPacket {
    pub position: Position,
    pub sprite_id: u16,
    pub stack_pos: u8,
    pub index: u8,
}

impl UseItemPacket {
    pub fn parse(msg: &mut NetworkMessage) -> Result<Self> {
        Ok(Self {
            position: Position::read(msg)?,
            sprite_id: msg.get_u16()?,
            stack_pos: msg.get_u8()?,
            index: msg.get_u8()?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct UseItemExPacket {
    pub from_position: Position,
    pub from_sprite_id: u16,
    pub from_stack_pos: u8,
    pub to_position: Position,
    pub to_sprite_id: u16,
    pub to_stack_pos: u8,
}

impl UseItemExPacket {
    pub fn parse(msg: &mut NetworkMessage) -> Result<Self> {
        Ok(Self {
            from_position: Position::read(msg)?,
            from_sprite_id: msg.get_u16()?,
            from_stack_pos: msg.get_u8()?,
            to_position: Position::read(msg)?,
            to_sprite_id: msg.get_u16()?,
            to_stack_pos: msg.get_u8()?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct LookAtPacket {
    pub position: Position,
    pub sprite_id: u16,
    pub stack_pos: u8,
}

impl LookAtPacket {
    pub fn parse(msg: &mut NetworkMessage) -> Result<Self> {
        Ok(Self {
            position: Position::read(msg)?,
            sprite_id: msg.get_u16()?,
            stack_pos: msg.get_u8()?,
        })
    }
}
