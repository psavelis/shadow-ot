//! Shadow OT Realm System
//!
//! Manages multiple game realms (servers) with different configurations,
//! rulesets, and themes. Each realm operates independently but can
//! share account data and allow cross-realm features.

pub mod config;
pub mod instance;
pub mod manager;
pub mod transfer;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

pub use config::RealmConfig;
pub use instance::RealmInstance;
pub use manager::RealmManager;
pub use transfer::CrossRealmTransfer;

/// Realm errors
#[derive(Debug, Error)]
pub enum RealmError {
    #[error("Realm not found: {0}")]
    NotFound(Uuid),
    
    #[error("Realm is offline")]
    Offline,
    
    #[error("Realm is full")]
    Full,
    
    #[error("Character already exists in realm")]
    CharacterExists,
    
    #[error("Transfer not allowed")]
    TransferNotAllowed,
    
    #[error("Cross-realm feature disabled")]
    CrossRealmDisabled,
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
}

/// Realm type/theme
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RealmType {
    /// Standard PvE realm
    PvE,
    /// Open PvP realm
    PvP,
    /// Hardcore (permadeath) realm
    Hardcore,
    /// Retro/classic rules realm
    Retro,
    /// Seasonal realm with time limit
    Seasonal,
    /// Experimental/test realm
    Experimental,
    /// Tournament realm
    Tournament,
    /// Custom/modded realm
    Custom,
}

/// Realm status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RealmStatus {
    /// Realm is online and accepting players
    Online,
    /// Realm is offline for maintenance
    Maintenance,
    /// Realm is online but locked (no new players)
    Locked,
    /// Realm is starting up
    Starting,
    /// Realm is shutting down
    ShuttingDown,
    /// Realm crashed
    Crashed,
}

/// Realm information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealmInfo {
    /// Unique realm ID
    pub id: Uuid,
    /// Realm name
    pub name: String,
    /// Description
    pub description: String,
    /// Realm type
    pub realm_type: RealmType,
    /// Current status
    pub status: RealmStatus,
    /// Server location/region
    pub region: String,
    /// Current online players
    pub online_count: u32,
    /// Maximum players
    pub max_players: u32,
    /// Experience rate multiplier
    pub exp_rate: f64,
    /// Loot rate multiplier
    pub loot_rate: f64,
    /// Skill rate multiplier
    pub skill_rate: f64,
    /// PvP enabled
    pub pvp_enabled: bool,
    /// Premium required
    pub premium_required: bool,
    /// Level cap
    pub level_cap: u32,
    /// When realm was created
    pub created_at: DateTime<Utc>,
    /// Last online time
    pub last_online: Option<DateTime<Utc>>,
    /// Season end date (for seasonal realms)
    pub season_end: Option<DateTime<Utc>>,
    /// Featured/recommended
    pub featured: bool,
}

impl RealmInfo {
    /// Check if realm is accepting new players
    pub fn is_available(&self) -> bool {
        matches!(self.status, RealmStatus::Online) && 
        self.online_count < self.max_players
    }

    /// Get population level (low/medium/high)
    pub fn population_level(&self) -> &'static str {
        let ratio = self.online_count as f64 / self.max_players as f64;
        if ratio < 0.3 {
            "low"
        } else if ratio < 0.7 {
            "medium"
        } else {
            "high"
        }
    }

    /// Check if realm is seasonal and still active
    pub fn is_season_active(&self) -> bool {
        match self.season_end {
            Some(end) => Utc::now() < end,
            None => true,
        }
    }
}

/// Player's realm data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerRealmData {
    /// Account ID
    pub account_id: Uuid,
    /// Characters in this realm
    pub characters: Vec<Uuid>,
    /// First joined this realm
    pub joined_at: DateTime<Utc>,
    /// Last played in this realm
    pub last_played: DateTime<Utc>,
    /// Total playtime in this realm (minutes)
    pub playtime_minutes: u64,
    /// Realm-specific achievements
    pub realm_achievements: Vec<String>,
}

/// Cross-realm friend entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossRealmFriend {
    /// Account ID of friend
    pub account_id: Uuid,
    /// Friend's display name
    pub display_name: String,
    /// Currently online
    pub online: bool,
    /// Current realm if online
    pub current_realm: Option<Uuid>,
    /// Current character if online
    pub current_character: Option<String>,
}

/// Realm announcement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealmAnnouncement {
    /// Announcement ID
    pub id: Uuid,
    /// Title
    pub title: String,
    /// Content
    pub content: String,
    /// Priority (higher = more important)
    pub priority: u8,
    /// When posted
    pub posted_at: DateTime<Utc>,
    /// Expires at
    pub expires_at: Option<DateTime<Utc>>,
    /// Is pinned
    pub pinned: bool,
}

/// Server-wide message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GlobalMessage {
    /// System announcement
    Announcement(String),
    /// Server shutdown warning
    ShutdownWarning { minutes: u32 },
    /// Server restart notification
    RestartNotification { reason: String },
    /// Event start notification
    EventStart { event_name: String },
    /// Event end notification
    EventEnd { event_name: String },
    /// World boss spawn
    WorldBoss { boss_name: String, location: String },
    /// Double XP announcement
    DoubleXP { duration_hours: u32 },
}

/// Realm selection response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealmListResponse {
    /// Available realms
    pub realms: Vec<RealmInfo>,
    /// Recommended realm for new players
    pub recommended: Option<Uuid>,
    /// Realms with player's characters
    pub player_realms: Vec<Uuid>,
    /// Last played realm
    pub last_realm: Option<Uuid>,
}
