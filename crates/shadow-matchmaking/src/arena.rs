//! Arena Module
//!
//! Handles arena instances and match management.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{MatchParticipant, MatchResult, MatchStats, MatchType};

/// Arena definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arena {
    /// Arena ID
    pub id: Uuid,
    /// Arena name
    pub name: String,
    /// Map ID for this arena
    pub map_id: String,
    /// Spawn points for team 1
    pub team1_spawns: Vec<(i32, i32, i32)>,
    /// Spawn points for team 2
    pub team2_spawns: Vec<(i32, i32, i32)>,
    /// Match types this arena supports
    pub supported_types: Vec<MatchType>,
    /// Maximum match duration (seconds)
    pub max_duration: u64,
    /// Is currently in use
    pub in_use: bool,
    /// Is enabled
    pub enabled: bool,
}

impl Arena {
    /// Create a new arena
    pub fn new(
        name: &str,
        map_id: &str,
        supported_types: Vec<MatchType>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            map_id: map_id.to_string(),
            team1_spawns: Vec::new(),
            team2_spawns: Vec::new(),
            supported_types,
            max_duration: 600, // 10 minutes default
            in_use: false,
            enabled: true,
        }
    }

    /// Check if arena supports a match type
    pub fn supports(&self, match_type: MatchType) -> bool {
        self.supported_types.contains(&match_type)
    }

    /// Check if arena is available
    pub fn is_available(&self) -> bool {
        self.enabled && !self.in_use
    }
}

/// An active arena match
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArenaMatch {
    /// Match ID
    pub id: Uuid,
    /// Arena ID
    pub arena_id: Uuid,
    /// Match type
    pub match_type: MatchType,
    /// Participants
    pub participants: Vec<MatchParticipant>,
    /// Match start time
    pub started_at: DateTime<Utc>,
    /// Match end time (if ended)
    pub ended_at: Option<DateTime<Utc>>,
    /// Current result
    pub result: MatchResult,
    /// Team 1 score
    pub team1_score: u32,
    /// Team 2 score
    pub team2_score: u32,
    /// Match events log
    pub events: Vec<MatchEvent>,
}

impl ArenaMatch {
    /// Create a new arena match
    pub fn new(
        id: Uuid,
        arena_id: Uuid,
        match_type: MatchType,
        participants: Vec<MatchParticipant>,
    ) -> Self {
        Self {
            id,
            arena_id,
            match_type,
            participants,
            started_at: Utc::now(),
            ended_at: None,
            result: MatchResult::InProgress,
            team1_score: 0,
            team2_score: 0,
            events: Vec::new(),
        }
    }

    /// Record a kill
    pub fn record_kill(
        &mut self,
        killer_id: Uuid,
        victim_id: Uuid,
    ) {
        // Update killer stats
        if let Some(killer) = self.participants.iter_mut()
            .find(|p| p.character_id == killer_id)
        {
            killer.stats.kills += 1;
        }

        // Update victim stats
        if let Some(victim) = self.participants.iter_mut()
            .find(|p| p.character_id == victim_id)
        {
            victim.stats.deaths += 1;
        }

        // Update scores
        if let (Some(killer), Some(victim)) = (
            self.get_participant(killer_id),
            self.get_participant(victim_id),
        ) {
            if killer.team != victim.team {
                if killer.team == 0 {
                    self.team1_score += 1;
                } else {
                    self.team2_score += 1;
                }
            }
        }

        // Log event
        self.events.push(MatchEvent::Kill {
            killer: killer_id,
            victim: victim_id,
            timestamp: Utc::now(),
        });
    }

    /// Record damage
    pub fn record_damage(&mut self, dealer_id: Uuid, target_id: Uuid, amount: u64) {
        if let Some(dealer) = self.participants.iter_mut()
            .find(|p| p.character_id == dealer_id)
        {
            dealer.stats.damage_dealt += amount;
        }

        if let Some(target) = self.participants.iter_mut()
            .find(|p| p.character_id == target_id)
        {
            target.stats.damage_taken += amount;
        }
    }

    /// Record healing
    pub fn record_healing(&mut self, healer_id: Uuid, amount: u64) {
        if let Some(healer) = self.participants.iter_mut()
            .find(|p| p.character_id == healer_id)
        {
            healer.stats.healing_done += amount;
        }
    }

    /// Get participant by ID
    pub fn get_participant(&self, character_id: Uuid) -> Option<&MatchParticipant> {
        self.participants.iter().find(|p| p.character_id == character_id)
    }

    /// Get team members
    pub fn get_team(&self, team: u8) -> Vec<&MatchParticipant> {
        self.participants.iter()
            .filter(|p| p.team == team)
            .collect()
    }

    /// End the match
    pub fn end(&mut self, result: MatchResult) {
        self.result = result;
        self.ended_at = Some(Utc::now());
    }

    /// Check if match is over
    pub fn is_over(&self) -> bool {
        !matches!(self.result, MatchResult::InProgress)
    }

    /// Get match duration
    pub fn duration(&self) -> chrono::Duration {
        match self.ended_at {
            Some(end) => end - self.started_at,
            None => Utc::now() - self.started_at,
        }
    }

    /// Get MVP (most valuable player)
    pub fn get_mvp(&self) -> Option<&MatchParticipant> {
        self.participants.iter()
            .max_by(|a, b| {
                let a_score = a.stats.kills * 3 + a.stats.assists;
                let b_score = b.stats.kills * 3 + b.stats.assists;
                a_score.cmp(&b_score)
            })
    }
}

/// Match events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchEvent {
    Kill {
        killer: Uuid,
        victim: Uuid,
        timestamp: DateTime<Utc>,
    },
    Objective {
        player: Uuid,
        objective_type: String,
        timestamp: DateTime<Utc>,
    },
    Disconnect {
        player: Uuid,
        timestamp: DateTime<Utc>,
    },
    Reconnect {
        player: Uuid,
        timestamp: DateTime<Utc>,
    },
}

/// Arena manager
pub struct ArenaManager {
    /// All arenas
    arenas: HashMap<Uuid, Arena>,
    /// Active matches
    active_matches: HashMap<Uuid, Uuid>, // arena_id -> match_id
}

impl ArenaManager {
    /// Create a new arena manager
    pub fn new() -> Self {
        let mut manager = Self {
            arenas: HashMap::new(),
            active_matches: HashMap::new(),
        };

        // Add default arenas
        manager.add_default_arenas();
        manager
    }

    /// Add default arenas
    fn add_default_arenas(&mut self) {
        let duel_arena = Arena {
            id: Uuid::new_v4(),
            name: "Duel Arena".to_string(),
            map_id: "arena_duel_1".to_string(),
            team1_spawns: vec![(100, 100, 7)],
            team2_spawns: vec![(110, 100, 7)],
            supported_types: vec![MatchType::Duel],
            max_duration: 300,
            in_use: false,
            enabled: true,
        };
        self.arenas.insert(duel_arena.id, duel_arena);

        let team_arena = Arena {
            id: Uuid::new_v4(),
            name: "Team Arena".to_string(),
            map_id: "arena_team_1".to_string(),
            team1_spawns: vec![(100, 100, 7), (101, 100, 7), (102, 100, 7)],
            team2_spawns: vec![(110, 100, 7), (111, 100, 7), (112, 100, 7)],
            supported_types: vec![MatchType::Team2v2, MatchType::Team3v3],
            max_duration: 600,
            in_use: false,
            enabled: true,
        };
        self.arenas.insert(team_arena.id, team_arena);
    }

    /// Register an arena
    pub fn register_arena(&mut self, arena: Arena) {
        self.arenas.insert(arena.id, arena);
    }

    /// Get an available arena for a match type
    pub fn get_available_arena(&mut self, match_type: MatchType) -> Option<Uuid> {
        for arena in self.arenas.values_mut() {
            if arena.is_available() && arena.supports(match_type) {
                arena.in_use = true;
                return Some(arena.id);
            }
        }
        None
    }

    /// Release an arena after a match
    pub fn release_arena(&mut self, arena_id: Uuid) {
        if let Some(arena) = self.arenas.get_mut(&arena_id) {
            arena.in_use = false;
        }
        self.active_matches.remove(&arena_id);
    }

    /// Get arena by ID
    pub fn get_arena(&self, arena_id: Uuid) -> Option<&Arena> {
        self.arenas.get(&arena_id)
    }

    /// Get all arenas
    pub fn all_arenas(&self) -> impl Iterator<Item = &Arena> {
        self.arenas.values()
    }
}

impl Default for ArenaManager {
    fn default() -> Self {
        Self::new()
    }
}
