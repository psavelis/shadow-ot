//! Global game state management

use std::collections::HashMap;

use crate::engine::RealmState;
use crate::RealmId;

/// Global game state shared across all components
#[derive(Debug, Default)]
pub struct GameState {
    /// All active realms
    pub realms: HashMap<RealmId, RealmState>,
    /// Server start time
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Total unique logins today
    pub daily_unique_logins: usize,
    /// Peak concurrent players
    pub peak_players: usize,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            realms: HashMap::new(),
            started_at: Some(chrono::Utc::now()),
            daily_unique_logins: 0,
            peak_players: 0,
        }
    }

    pub fn total_players(&self) -> usize {
        self.realms.values().map(|r| r.player_count).sum()
    }

    pub fn update_peak(&mut self) {
        let current = self.total_players();
        if current > self.peak_players {
            self.peak_players = current;
        }
    }

    pub fn uptime(&self) -> chrono::Duration {
        self.started_at
            .map(|start| chrono::Utc::now() - start)
            .unwrap_or_default()
    }
}
