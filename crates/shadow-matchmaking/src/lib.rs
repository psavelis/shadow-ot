//! Shadow OT Matchmaking System
//!
//! Handles competitive matchmaking, arena battles, tournaments,
//! and ranked play systems.

pub mod arena;
pub mod queue;
pub mod rating;
pub mod tournament;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

pub use arena::{Arena, ArenaManager, ArenaMatch};
pub use queue::{MatchmakingQueue, QueueEntry};
pub use rating::{PlayerRating, RatingSystem};
pub use tournament::{Tournament, TournamentManager};

/// Matchmaking errors
#[derive(Debug, Error)]
pub enum MatchmakingError {
    #[error("Player already in queue")]
    AlreadyInQueue,
    
    #[error("Player not found in queue")]
    NotInQueue,
    
    #[error("Match not found")]
    MatchNotFound,
    
    #[error("Arena not found")]
    ArenaNotFound,
    
    #[error("Tournament not found")]
    TournamentNotFound,
    
    #[error("Tournament is full")]
    TournamentFull,
    
    #[error("Invalid team size")]
    InvalidTeamSize,
    
    #[error("Level requirement not met")]
    LevelRequirementNotMet,
    
    #[error("Already in match")]
    AlreadyInMatch,
    
    #[error("Cooldown active")]
    CooldownActive,
    
    #[error("Database error: {0}")]
    DatabaseError(String),
}

/// Match types available
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MatchType {
    /// 1v1 duel
    Duel,
    /// 2v2 team battle
    Team2v2,
    /// 3v3 team battle
    Team3v3,
    /// 5v5 team battle
    Team5v5,
    /// Free-for-all (last man standing)
    FreeForAll,
    /// Capture the flag
    CaptureTheFlag,
    /// King of the hill
    KingOfTheHill,
    /// Battle royale
    BattleRoyale,
    /// Custom match
    Custom,
}

impl MatchType {
    /// Get team size for this match type
    pub fn team_size(&self) -> usize {
        match self {
            MatchType::Duel => 1,
            MatchType::Team2v2 => 2,
            MatchType::Team3v3 => 3,
            MatchType::Team5v5 => 5,
            MatchType::FreeForAll => 1,
            MatchType::CaptureTheFlag => 5,
            MatchType::KingOfTheHill => 3,
            MatchType::BattleRoyale => 1,
            MatchType::Custom => 1,
        }
    }

    /// Get number of teams
    pub fn team_count(&self) -> usize {
        match self {
            MatchType::Duel => 2,
            MatchType::Team2v2 => 2,
            MatchType::Team3v3 => 2,
            MatchType::Team5v5 => 2,
            MatchType::FreeForAll => 10,
            MatchType::CaptureTheFlag => 2,
            MatchType::KingOfTheHill => 3,
            MatchType::BattleRoyale => 20,
            MatchType::Custom => 2,
        }
    }
}

/// Match result
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatchResult {
    /// Match is still in progress
    InProgress,
    /// Team 1 won
    Team1Win,
    /// Team 2 won
    Team2Win,
    /// Draw/tie
    Draw,
    /// Match was cancelled
    Cancelled,
}

/// Player match statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MatchStats {
    /// Kills
    pub kills: u32,
    /// Deaths
    pub deaths: u32,
    /// Assists
    pub assists: u32,
    /// Damage dealt
    pub damage_dealt: u64,
    /// Damage taken
    pub damage_taken: u64,
    /// Healing done
    pub healing_done: u64,
    /// Objectives completed
    pub objectives: u32,
}

impl MatchStats {
    /// Calculate KDA ratio
    pub fn kda(&self) -> f64 {
        let deaths = self.deaths.max(1) as f64;
        (self.kills as f64 + self.assists as f64 * 0.5) / deaths
    }
}

/// A participant in a match
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchParticipant {
    /// Character ID
    pub character_id: Uuid,
    /// Character name
    pub character_name: String,
    /// Team number
    pub team: u8,
    /// Stats for this match
    pub stats: MatchStats,
    /// Rating before match
    pub rating_before: i32,
    /// Rating change
    pub rating_change: i32,
    /// Left early/disconnected
    pub left_early: bool,
}

/// Matchmaking configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchmakingConfig {
    /// Enable matchmaking
    pub enabled: bool,
    /// Maximum queue time before expanding search (seconds)
    pub max_queue_time: u64,
    /// Rating range for matching
    pub rating_range: i32,
    /// Rating range expansion per minute
    pub rating_expansion: i32,
    /// Maximum rating range
    pub max_rating_range: i32,
    /// Minimum level to queue
    pub min_level: u32,
    /// Cooldown between matches (seconds)
    pub match_cooldown: u64,
    /// Enable cross-realm matching
    pub cross_realm: bool,
}

impl Default for MatchmakingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_queue_time: 300,
            rating_range: 100,
            rating_expansion: 50,
            max_rating_range: 500,
            min_level: 50,
            match_cooldown: 30,
            cross_realm: true,
        }
    }
}

/// Season information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Season {
    /// Season ID
    pub id: u32,
    /// Season name
    pub name: String,
    /// When season started
    pub start_date: DateTime<Utc>,
    /// When season ends
    pub end_date: DateTime<Utc>,
    /// Is current season
    pub is_active: bool,
    /// Rewards for ranks
    pub rewards: HashMap<Rank, SeasonReward>,
}

/// Competitive ranks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Rank {
    Unranked,
    Bronze1,
    Bronze2,
    Bronze3,
    Silver1,
    Silver2,
    Silver3,
    Gold1,
    Gold2,
    Gold3,
    Platinum1,
    Platinum2,
    Platinum3,
    Diamond1,
    Diamond2,
    Diamond3,
    Master,
    Grandmaster,
    Legend,
}

impl Rank {
    /// Get rank from rating
    pub fn from_rating(rating: i32) -> Self {
        match rating {
            r if r < 0 => Rank::Unranked,
            0..=199 => Rank::Bronze1,
            200..=399 => Rank::Bronze2,
            400..=599 => Rank::Bronze3,
            600..=799 => Rank::Silver1,
            800..=999 => Rank::Silver2,
            1000..=1199 => Rank::Silver3,
            1200..=1399 => Rank::Gold1,
            1400..=1599 => Rank::Gold2,
            1600..=1799 => Rank::Gold3,
            1800..=1999 => Rank::Platinum1,
            2000..=2199 => Rank::Platinum2,
            2200..=2399 => Rank::Platinum3,
            2400..=2599 => Rank::Diamond1,
            2600..=2799 => Rank::Diamond2,
            2800..=2999 => Rank::Diamond3,
            3000..=3199 => Rank::Master,
            3200..=3499 => Rank::Grandmaster,
            _ => Rank::Legend,
        }
    }

    /// Get minimum rating for this rank
    pub fn min_rating(&self) -> i32 {
        match self {
            Rank::Unranked => 0,
            Rank::Bronze1 => 0,
            Rank::Bronze2 => 200,
            Rank::Bronze3 => 400,
            Rank::Silver1 => 600,
            Rank::Silver2 => 800,
            Rank::Silver3 => 1000,
            Rank::Gold1 => 1200,
            Rank::Gold2 => 1400,
            Rank::Gold3 => 1600,
            Rank::Platinum1 => 1800,
            Rank::Platinum2 => 2000,
            Rank::Platinum3 => 2200,
            Rank::Diamond1 => 2400,
            Rank::Diamond2 => 2600,
            Rank::Diamond3 => 2800,
            Rank::Master => 3000,
            Rank::Grandmaster => 3200,
            Rank::Legend => 3500,
        }
    }

    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Rank::Unranked => "Unranked",
            Rank::Bronze1 => "Bronze I",
            Rank::Bronze2 => "Bronze II",
            Rank::Bronze3 => "Bronze III",
            Rank::Silver1 => "Silver I",
            Rank::Silver2 => "Silver II",
            Rank::Silver3 => "Silver III",
            Rank::Gold1 => "Gold I",
            Rank::Gold2 => "Gold II",
            Rank::Gold3 => "Gold III",
            Rank::Platinum1 => "Platinum I",
            Rank::Platinum2 => "Platinum II",
            Rank::Platinum3 => "Platinum III",
            Rank::Diamond1 => "Diamond I",
            Rank::Diamond2 => "Diamond II",
            Rank::Diamond3 => "Diamond III",
            Rank::Master => "Master",
            Rank::Grandmaster => "Grandmaster",
            Rank::Legend => "Legend",
        }
    }
}

/// Season reward
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonReward {
    /// Title granted
    pub title: Option<String>,
    /// Mount ID granted
    pub mount: Option<u32>,
    /// Outfit addon granted
    pub outfit_addon: Option<(u32, u8)>,
    /// Gold reward
    pub gold: u64,
    /// Premium currency
    pub premium_currency: u32,
}

/// Main matchmaking system
pub struct MatchmakingSystem {
    /// Configuration
    config: MatchmakingConfig,
    /// Matchmaking queues by type
    queues: HashMap<MatchType, MatchmakingQueue>,
    /// Arena manager
    arenas: ArenaManager,
    /// Rating system
    ratings: RatingSystem,
    /// Tournament manager
    tournaments: TournamentManager,
    /// Active matches
    active_matches: HashMap<Uuid, ArenaMatch>,
    /// Player match history
    match_history: HashMap<Uuid, Vec<Uuid>>,
}

impl MatchmakingSystem {
    /// Create a new matchmaking system
    pub fn new(config: MatchmakingConfig) -> Self {
        let mut queues = HashMap::new();
        
        // Create queues for each match type
        for match_type in [
            MatchType::Duel,
            MatchType::Team2v2,
            MatchType::Team3v3,
            MatchType::Team5v5,
        ] {
            queues.insert(match_type, MatchmakingQueue::new(match_type));
        }

        Self {
            config,
            queues,
            arenas: ArenaManager::new(),
            ratings: RatingSystem::new(),
            tournaments: TournamentManager::new(),
            active_matches: HashMap::new(),
            match_history: HashMap::new(),
        }
    }

    /// Queue a player for matchmaking
    pub fn queue_player(
        &mut self,
        character_id: Uuid,
        character_name: &str,
        level: u32,
        match_type: MatchType,
    ) -> Result<(), MatchmakingError> {
        if !self.config.enabled {
            return Err(MatchmakingError::CooldownActive);
        }

        if level < self.config.min_level {
            return Err(MatchmakingError::LevelRequirementNotMet);
        }

        // Check if already in a match
        if self.is_in_match(character_id) {
            return Err(MatchmakingError::AlreadyInMatch);
        }

        let queue = self.queues.get_mut(&match_type)
            .ok_or(MatchmakingError::NotInQueue)?;

        let rating = self.ratings.get_rating(character_id).rating;

        queue.add_player(character_id, character_name, rating)
    }

    /// Remove a player from queue
    pub fn dequeue_player(
        &mut self,
        character_id: Uuid,
        match_type: MatchType,
    ) -> Result<(), MatchmakingError> {
        let queue = self.queues.get_mut(&match_type)
            .ok_or(MatchmakingError::NotInQueue)?;

        queue.remove_player(character_id)
    }

    /// Check if player is in any match
    pub fn is_in_match(&self, character_id: Uuid) -> bool {
        self.active_matches.values().any(|m| {
            m.participants.iter().any(|p| p.character_id == character_id)
        })
    }

    /// Process matchmaking queues
    pub fn process_queues(&mut self) -> Vec<ArenaMatch> {
        let mut new_matches = Vec::new();

        for (match_type, queue) in &mut self.queues {
            while let Some(matched) = queue.try_match(&self.config) {
                let arena = self.arenas.get_available_arena(*match_type);
                
                if let Some(arena_id) = arena {
                    let match_id = Uuid::new_v4();
                    let arena_match = ArenaMatch::new(
                        match_id,
                        arena_id,
                        *match_type,
                        matched,
                    );
                    
                    self.active_matches.insert(match_id, arena_match.clone());
                    new_matches.push(arena_match);
                }
            }
        }

        new_matches
    }

    /// End a match and process results
    pub fn end_match(
        &mut self,
        match_id: Uuid,
        result: MatchResult,
    ) -> Result<Vec<(Uuid, i32)>, MatchmakingError> {
        let arena_match = self.active_matches.remove(&match_id)
            .ok_or(MatchmakingError::MatchNotFound)?;

        // Calculate rating changes
        let rating_changes = self.ratings.process_match(&arena_match, result);

        // Record match in history
        for participant in &arena_match.participants {
            self.match_history
                .entry(participant.character_id)
                .or_insert_with(Vec::new)
                .push(match_id);
        }

        // Return arena
        self.arenas.release_arena(arena_match.arena_id);

        Ok(rating_changes)
    }

    /// Get player's current rank
    pub fn get_rank(&self, character_id: Uuid) -> Rank {
        let rating = self.ratings.get_rating(character_id);
        Rank::from_rating(rating.rating)
    }

    /// Get leaderboard
    pub fn get_leaderboard(&self, match_type: MatchType, limit: usize) -> Vec<(Uuid, i32)> {
        self.ratings.get_leaderboard(match_type, limit)
    }

    /// Get queue statistics
    pub fn get_queue_stats(&self, match_type: MatchType) -> Option<QueueStats> {
        self.queues.get(&match_type).map(|q| q.get_stats())
    }
}

/// Queue statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueStats {
    /// Players in queue
    pub players_in_queue: usize,
    /// Average wait time (seconds)
    pub avg_wait_time: f64,
    /// Matches made in last hour
    pub matches_last_hour: u32,
}
