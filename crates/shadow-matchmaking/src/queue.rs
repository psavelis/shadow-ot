//! Matchmaking Queue Module
//!
//! Handles player queuing and match creation logic.

use chrono::{DateTime, Duration, Utc};
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

use crate::{MatchType, MatchmakingConfig, MatchmakingError, MatchParticipant, MatchStats, QueueStats};

/// A player in the queue
#[derive(Debug, Clone)]
pub struct QueueEntry {
    /// Character ID
    pub character_id: Uuid,
    /// Character name
    pub character_name: String,
    /// Player rating
    pub rating: i32,
    /// When they joined the queue
    pub joined_at: DateTime<Utc>,
    /// Team ID (for pre-made teams)
    pub team_id: Option<Uuid>,
}

impl QueueEntry {
    /// Create a new queue entry
    pub fn new(character_id: Uuid, character_name: &str, rating: i32) -> Self {
        Self {
            character_id,
            character_name: character_name.to_string(),
            rating,
            joined_at: Utc::now(),
            team_id: None,
        }
    }

    /// How long this player has been waiting
    pub fn wait_time(&self) -> Duration {
        Utc::now() - self.joined_at
    }

    /// Get expanded rating range based on wait time
    pub fn expanded_range(&self, config: &MatchmakingConfig) -> i32 {
        let minutes_waiting = self.wait_time().num_minutes() as i32;
        let expansion = minutes_waiting * config.rating_expansion;
        (config.rating_range + expansion).min(config.max_rating_range)
    }
}

/// Matchmaking queue for a specific match type
pub struct MatchmakingQueue {
    /// Match type
    match_type: MatchType,
    /// Players in queue (sorted by join time)
    queue: VecDeque<QueueEntry>,
    /// Quick lookup by character ID
    player_lookup: HashMap<Uuid, usize>,
    /// Statistics
    matches_created: u32,
    total_wait_time: Duration,
    matches_in_last_hour: Vec<DateTime<Utc>>,
}

impl MatchmakingQueue {
    /// Create a new matchmaking queue
    pub fn new(match_type: MatchType) -> Self {
        Self {
            match_type,
            queue: VecDeque::new(),
            player_lookup: HashMap::new(),
            matches_created: 0,
            total_wait_time: Duration::zero(),
            matches_in_last_hour: Vec::new(),
        }
    }

    /// Add a player to the queue
    pub fn add_player(
        &mut self,
        character_id: Uuid,
        character_name: &str,
        rating: i32,
    ) -> Result<(), MatchmakingError> {
        if self.player_lookup.contains_key(&character_id) {
            return Err(MatchmakingError::AlreadyInQueue);
        }

        let entry = QueueEntry::new(character_id, character_name, rating);
        let index = self.queue.len();
        self.queue.push_back(entry);
        self.player_lookup.insert(character_id, index);

        Ok(())
    }

    /// Remove a player from the queue
    pub fn remove_player(&mut self, character_id: Uuid) -> Result<(), MatchmakingError> {
        if let Some(&index) = self.player_lookup.get(&character_id) {
            self.queue.remove(index);
            self.player_lookup.remove(&character_id);
            
            // Rebuild lookup after removal
            self.rebuild_lookup();
            Ok(())
        } else {
            Err(MatchmakingError::NotInQueue)
        }
    }

    /// Rebuild the player lookup table
    fn rebuild_lookup(&mut self) {
        self.player_lookup.clear();
        for (i, entry) in self.queue.iter().enumerate() {
            self.player_lookup.insert(entry.character_id, i);
        }
    }

    /// Try to create a match from the queue
    pub fn try_match(&mut self, config: &MatchmakingConfig) -> Option<Vec<MatchParticipant>> {
        let team_size = self.match_type.team_size();
        let team_count = self.match_type.team_count();
        let players_needed = team_size * team_count;

        if self.queue.len() < players_needed {
            return None;
        }

        // Find compatible players
        let mut matched = Vec::new();
        let mut matched_indices = Vec::new();

        // Start with the player who has waited longest
        if let Some(anchor) = self.queue.front() {
            let anchor_rating = anchor.rating;
            let range = anchor.expanded_range(config);

            for (i, entry) in self.queue.iter().enumerate() {
                let diff = (entry.rating - anchor_rating).abs();
                if diff <= range {
                    matched.push(entry.clone());
                    matched_indices.push(i);

                    if matched.len() >= players_needed {
                        break;
                    }
                }
            }
        }

        if matched.len() >= players_needed {
            // Remove matched players from queue
            matched_indices.sort_by(|a, b| b.cmp(a)); // Sort descending to remove from end first
            for index in matched_indices {
                if let Some(entry) = self.queue.remove(index) {
                    self.total_wait_time = self.total_wait_time + entry.wait_time();
                }
            }
            self.rebuild_lookup();

            // Create participants with team assignments
            let participants = self.create_participants(matched, team_size);

            // Update stats
            self.matches_created += 1;
            self.matches_in_last_hour.push(Utc::now());

            return Some(participants);
        }

        None
    }

    /// Create match participants from matched players
    fn create_participants(
        &self,
        matched: Vec<QueueEntry>,
        team_size: usize,
    ) -> Vec<MatchParticipant> {
        matched.into_iter().enumerate().map(|(i, entry)| {
            let team = (i / team_size) as u8;
            MatchParticipant {
                character_id: entry.character_id,
                character_name: entry.character_name,
                team,
                stats: MatchStats::default(),
                rating_before: entry.rating,
                rating_change: 0,
                left_early: false,
            }
        }).collect()
    }

    /// Get queue statistics
    pub fn get_stats(&self) -> QueueStats {
        // Clean up old entries
        let hour_ago = Utc::now() - Duration::hours(1);
        let recent_matches = self.matches_in_last_hour.iter()
            .filter(|&t| *t > hour_ago)
            .count();

        let avg_wait = if self.matches_created > 0 {
            self.total_wait_time.num_seconds() as f64 / self.matches_created as f64
        } else {
            0.0
        };

        QueueStats {
            players_in_queue: self.queue.len(),
            avg_wait_time: avg_wait,
            matches_last_hour: recent_matches as u32,
        }
    }

    /// Get number of players in queue
    pub fn queue_size(&self) -> usize {
        self.queue.len()
    }

    /// Check if player is in queue
    pub fn is_in_queue(&self, character_id: Uuid) -> bool {
        self.player_lookup.contains_key(&character_id)
    }
}
