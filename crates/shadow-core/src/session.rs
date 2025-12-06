//! Player session management

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{CharacterId, PlayerId, RealmId};

/// Represents an active player session
#[derive(Debug, Clone)]
pub struct PlayerSession {
    pub session_id: Uuid,
    pub player_id: PlayerId,
    pub character_id: Option<CharacterId>,
    pub realm_id: Option<RealmId>,
    pub ip_address: String,
    pub connected_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub protocol_version: u16,
    pub client_version: String,
    pub state: SessionState,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SessionState {
    /// Just connected, not yet authenticated
    Connected,
    /// Authenticated, selecting character
    Authenticated,
    /// In game, playing
    InGame,
    /// Disconnecting
    Disconnecting,
}

impl PlayerSession {
    pub fn new(ip_address: String, protocol_version: u16) -> Self {
        let now = Utc::now();
        Self {
            session_id: Uuid::new_v4(),
            player_id: Uuid::nil(),
            character_id: None,
            realm_id: None,
            ip_address,
            connected_at: now,
            last_activity: now,
            protocol_version,
            client_version: String::new(),
            state: SessionState::Connected,
        }
    }

    pub fn authenticate(&mut self, player_id: PlayerId) {
        self.player_id = player_id;
        self.state = SessionState::Authenticated;
        self.touch();
    }

    pub fn enter_game(&mut self, character_id: CharacterId, realm_id: RealmId) {
        self.character_id = Some(character_id);
        self.realm_id = Some(realm_id);
        self.state = SessionState::InGame;
        self.touch();
    }

    pub fn touch(&mut self) {
        self.last_activity = Utc::now();
    }

    pub fn duration(&self) -> chrono::Duration {
        Utc::now() - self.connected_at
    }

    pub fn idle_duration(&self) -> chrono::Duration {
        Utc::now() - self.last_activity
    }

    pub fn is_idle(&self, max_idle_seconds: i64) -> bool {
        self.idle_duration().num_seconds() > max_idle_seconds
    }
}
