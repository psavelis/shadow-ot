//! Realm Instance
//!
//! Represents a running realm with its state.

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{RealmConfig, RealmError, RealmInfo, RealmStatus, RealmType};

/// A running realm instance
pub struct RealmInstance {
    /// Realm info
    pub info: RealmInfo,
    /// Configuration
    pub config: RealmConfig,
    /// Online players (character_id -> account_id)
    pub online_players: HashMap<Uuid, Uuid>,
    /// Player sessions (character_id -> session)
    pub sessions: HashMap<Uuid, PlayerSession>,
    /// Is accepting connections
    pub accepting_connections: bool,
    /// Last save time
    pub last_save: DateTime<Utc>,
    /// Uptime start
    pub started_at: DateTime<Utc>,
}

/// Player session in realm
#[derive(Debug, Clone)]
pub struct PlayerSession {
    pub character_id: Uuid,
    pub account_id: Uuid,
    pub character_name: String,
    pub level: u32,
    pub vocation: String,
    pub connected_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub ip_address: String,
}

impl RealmInstance {
    /// Create a new realm instance
    pub fn new(name: &str, config: RealmConfig) -> Self {
        let now = Utc::now();
        let realm_type = config.realm_type;
        
        Self {
            info: RealmInfo {
                id: Uuid::new_v4(),
                name: name.to_string(),
                description: String::new(),
                realm_type,
                status: RealmStatus::Starting,
                region: "default".to_string(),
                online_count: 0,
                max_players: config.max_players,
                exp_rate: config.experience.exp_rate,
                loot_rate: config.economy.loot_rate,
                skill_rate: config.experience.skill_rate,
                pvp_enabled: config.pvp.enabled,
                premium_required: false,
                level_cap: config.experience.level_cap,
                created_at: now,
                last_online: None,
                season_end: None,
                featured: false,
            },
            config,
            online_players: HashMap::new(),
            sessions: HashMap::new(),
            accepting_connections: false,
            last_save: now,
            started_at: now,
        }
    }

    /// Start the realm
    pub fn start(&mut self) -> Result<(), RealmError> {
        self.info.status = RealmStatus::Online;
        self.info.last_online = Some(Utc::now());
        self.accepting_connections = true;
        Ok(())
    }

    /// Stop the realm
    pub fn stop(&mut self) {
        self.accepting_connections = false;
        self.info.status = RealmStatus::ShuttingDown;
        
        // Kick all players
        self.online_players.clear();
        self.sessions.clear();
        
        self.info.status = RealmStatus::Maintenance;
    }

    /// Add a player to the realm
    pub fn add_player(
        &mut self,
        character_id: Uuid,
        account_id: Uuid,
        character_name: &str,
        level: u32,
        vocation: &str,
        ip: &str,
    ) -> Result<(), RealmError> {
        if !self.accepting_connections {
            return Err(RealmError::Offline);
        }

        if self.online_players.len() >= self.config.max_players as usize {
            return Err(RealmError::Full);
        }

        let now = Utc::now();
        let session = PlayerSession {
            character_id,
            account_id,
            character_name: character_name.to_string(),
            level,
            vocation: vocation.to_string(),
            connected_at: now,
            last_activity: now,
            ip_address: ip.to_string(),
        };

        self.online_players.insert(character_id, account_id);
        self.sessions.insert(character_id, session);
        self.info.online_count = self.online_players.len() as u32;

        Ok(())
    }

    /// Remove a player from the realm
    pub fn remove_player(&mut self, character_id: Uuid) {
        self.online_players.remove(&character_id);
        self.sessions.remove(&character_id);
        self.info.online_count = self.online_players.len() as u32;
    }

    /// Get uptime in seconds
    pub fn uptime(&self) -> i64 {
        (Utc::now() - self.started_at).num_seconds()
    }

    /// Check if character is online
    pub fn is_online(&self, character_id: Uuid) -> bool {
        self.online_players.contains_key(&character_id)
    }

    /// Get session for character
    pub fn get_session(&self, character_id: Uuid) -> Option<&PlayerSession> {
        self.sessions.get(&character_id)
    }

    /// Update session activity
    pub fn update_activity(&mut self, character_id: Uuid) {
        if let Some(session) = self.sessions.get_mut(&character_id) {
            session.last_activity = Utc::now();
        }
    }

    /// Get online count
    pub fn online_count(&self) -> usize {
        self.online_players.len()
    }

    /// Is realm full
    pub fn is_full(&self) -> bool {
        self.online_players.len() >= self.config.max_players as usize
    }

    /// Broadcast a message to all players
    pub fn broadcast(&self, _message: &str) {
        // Would send to all connected clients
        // Implementation depends on networking layer
    }
}
