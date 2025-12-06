//! Bosstiary System Implementation
//!
//! The Bosstiary tracks boss kills and provides rewards for defeating bosses.
//! Similar to the Bestiary but specifically for boss creatures.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Boss difficulty tiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum BossDifficulty {
    /// Easy bosses (mini-bosses, event bosses)
    Bane,
    /// Medium difficulty
    Archfoe,
    /// Hard bosses (world bosses)
    Nemesis,
}

impl BossDifficulty {
    /// Kills required to complete this difficulty tier
    pub fn kills_required(&self) -> u32 {
        match self {
            BossDifficulty::Bane => 5,
            BossDifficulty::Archfoe => 10,
            BossDifficulty::Nemesis => 20,
        }
    }

    /// Points awarded per kill
    pub fn points_per_kill(&self) -> u32 {
        match self {
            BossDifficulty::Bane => 25,
            BossDifficulty::Archfoe => 50,
            BossDifficulty::Nemesis => 100,
        }
    }

    /// Charm points awarded upon completion
    pub fn completion_charm_points(&self) -> u32 {
        match self {
            BossDifficulty::Bane => 50,
            BossDifficulty::Archfoe => 100,
            BossDifficulty::Nemesis => 250,
        }
    }
}

/// A boss entry in the Bosstiary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BossEntry {
    /// Unique boss creature ID
    pub boss_id: u32,
    /// Boss name
    pub name: String,
    /// Boss difficulty tier
    pub difficulty: BossDifficulty,
    /// Description/lore text
    pub description: String,
    /// Boss location hints
    pub locations: Vec<String>,
    /// Respawn time in hours (None for triggered bosses)
    pub respawn_hours: Option<u32>,
    /// Whether this boss is from a quest
    pub quest_related: bool,
    /// Minimum level to fight
    pub min_level: u16,
    /// Associated realm (if realm-specific)
    pub realm_id: Option<Uuid>,
    /// Special loot this boss can drop
    pub notable_loot: Vec<String>,
}

/// Player's progress on a specific boss
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BossProgress {
    pub boss_id: u32,
    pub kill_count: u32,
    pub first_kill: Option<DateTime<Utc>>,
    pub last_kill: Option<DateTime<Utc>>,
    pub is_completed: bool,
    /// Personal best kill time in seconds (for timed encounters)
    pub best_time: Option<u32>,
}

impl BossProgress {
    pub fn new(boss_id: u32) -> Self {
        Self {
            boss_id,
            kill_count: 0,
            first_kill: None,
            last_kill: None,
            is_completed: false,
            best_time: None,
        }
    }

    /// Record a kill
    pub fn record_kill(&mut self, kills_required: u32, kill_time: Option<u32>) {
        let now = Utc::now();

        if self.first_kill.is_none() {
            self.first_kill = Some(now);
        }
        self.last_kill = Some(now);
        self.kill_count += 1;

        if self.kill_count >= kills_required {
            self.is_completed = true;
        }

        // Update best time if provided and better
        if let Some(time) = kill_time {
            if self.best_time.map(|bt| time < bt).unwrap_or(true) {
                self.best_time = Some(time);
            }
        }
    }

    /// Get progress percentage (0.0 - 1.0)
    pub fn progress_percent(&self, kills_required: u32) -> f32 {
        (self.kill_count as f32 / kills_required as f32).min(1.0)
    }
}

/// Player's full Bosstiary state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerBosstiary {
    pub player_id: Uuid,
    /// Progress for each boss
    pub boss_progress: HashMap<u32, BossProgress>,
    /// Total bosstiary points earned
    pub total_points: u64,
    /// Total bosses completed
    pub bosses_completed: u32,
    /// Charm points earned from bosstiary
    pub charm_points_earned: u64,
    /// Tracked boss (shows spawn timer)
    pub tracked_boss: Option<u32>,
}

impl PlayerBosstiary {
    pub fn new(player_id: Uuid) -> Self {
        Self {
            player_id,
            boss_progress: HashMap::new(),
            total_points: 0,
            bosses_completed: 0,
            charm_points_earned: 0,
            tracked_boss: None,
        }
    }

    /// Record a boss kill
    pub fn record_kill(&mut self, boss: &BossEntry, kill_time: Option<u32>) -> BossKillResult {
        let progress = self.boss_progress
            .entry(boss.boss_id)
            .or_insert_with(|| BossProgress::new(boss.boss_id));

        let was_completed = progress.is_completed;
        let kills_required = boss.difficulty.kills_required();

        progress.record_kill(kills_required, kill_time);

        // Calculate rewards
        let points_earned = boss.difficulty.points_per_kill();
        self.total_points += points_earned as u64;

        let mut charm_points = 0;
        if progress.is_completed && !was_completed {
            // First time completing this boss
            self.bosses_completed += 1;
            charm_points = boss.difficulty.completion_charm_points();
            self.charm_points_earned += charm_points as u64;
        }

        BossKillResult {
            boss_id: boss.boss_id,
            kill_count: progress.kill_count,
            kills_required,
            points_earned,
            charm_points,
            newly_completed: progress.is_completed && !was_completed,
            is_new_best_time: kill_time.map(|t| Some(t) == progress.best_time).unwrap_or(false),
        }
    }

    /// Get unlocked information level for a boss
    pub fn info_level(&self, boss_id: u32, kills_required: u32) -> BossInfoLevel {
        match self.boss_progress.get(&boss_id) {
            None => BossInfoLevel::Unknown,
            Some(progress) => {
                let percent = progress.progress_percent(kills_required);
                if progress.is_completed {
                    BossInfoLevel::Complete
                } else if percent >= 0.5 {
                    BossInfoLevel::Detailed
                } else if percent > 0.0 {
                    BossInfoLevel::Basic
                } else {
                    BossInfoLevel::Unknown
                }
            }
        }
    }

    /// Set tracked boss
    pub fn track_boss(&mut self, boss_id: Option<u32>) {
        self.tracked_boss = boss_id;
    }

    /// Get completion percentage
    pub fn completion_percentage(&self, total_bosses: u32) -> f32 {
        if total_bosses == 0 {
            return 0.0;
        }
        (self.bosses_completed as f32 / total_bosses as f32) * 100.0
    }
}

/// Result of recording a boss kill
#[derive(Debug, Clone)]
pub struct BossKillResult {
    pub boss_id: u32,
    pub kill_count: u32,
    pub kills_required: u32,
    pub points_earned: u32,
    pub charm_points: u32,
    pub newly_completed: bool,
    pub is_new_best_time: bool,
}

/// Information level unlocked for a boss
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BossInfoLevel {
    /// No kills yet
    Unknown,
    /// Some kills - basic info
    Basic,
    /// Half kills - detailed info
    Detailed,
    /// Completed - full info + rewards
    Complete,
}

/// Bosstiary manager
pub struct BosstiaryManager {
    /// Boss definitions
    bosses: HashMap<u32, BossEntry>,
    /// Player bosstiary data
    player_data: HashMap<Uuid, PlayerBosstiary>,
    /// Boss spawn timers (boss_id -> next spawn time)
    spawn_timers: HashMap<u32, DateTime<Utc>>,
}

impl BosstiaryManager {
    pub fn new() -> Self {
        Self {
            bosses: HashMap::new(),
            player_data: HashMap::new(),
            spawn_timers: HashMap::new(),
        }
    }

    /// Register a boss in the bosstiary
    pub fn register_boss(&mut self, boss: BossEntry) {
        self.bosses.insert(boss.boss_id, boss);
    }

    /// Get boss entry
    pub fn get_boss(&self, boss_id: u32) -> Option<&BossEntry> {
        self.bosses.get(&boss_id)
    }

    /// Get all bosses
    pub fn all_bosses(&self) -> impl Iterator<Item = &BossEntry> {
        self.bosses.values()
    }

    /// Get bosses by difficulty
    pub fn bosses_by_difficulty(&self, difficulty: BossDifficulty) -> Vec<&BossEntry> {
        self.bosses
            .values()
            .filter(|b| b.difficulty == difficulty)
            .collect()
    }

    /// Get or create player bosstiary
    pub fn get_or_create(&mut self, player_id: Uuid) -> &mut PlayerBosstiary {
        self.player_data
            .entry(player_id)
            .or_insert_with(|| PlayerBosstiary::new(player_id))
    }

    /// Record a boss kill
    pub fn record_kill(&mut self, player_id: Uuid, boss_id: u32, kill_time: Option<u32>) -> Option<BossKillResult> {
        let boss = self.bosses.get(&boss_id)?.clone();
        let player_data = self.get_or_create(player_id);
        Some(player_data.record_kill(&boss, kill_time))
    }

    /// Update spawn timer for a boss
    pub fn set_spawn_timer(&mut self, boss_id: u32, next_spawn: DateTime<Utc>) {
        self.spawn_timers.insert(boss_id, next_spawn);
    }

    /// Get next spawn time for a boss
    pub fn get_spawn_timer(&self, boss_id: u32) -> Option<DateTime<Utc>> {
        self.spawn_timers.get(&boss_id).copied()
    }

    /// Calculate respawn time from now
    pub fn calculate_next_spawn(&self, boss_id: u32) -> Option<DateTime<Utc>> {
        let boss = self.bosses.get(&boss_id)?;
        let hours = boss.respawn_hours?;
        Some(Utc::now() + chrono::Duration::hours(hours as i64))
    }

    /// Get total boss count
    pub fn total_bosses(&self) -> u32 {
        self.bosses.len() as u32
    }

    /// Get leaderboard for a specific boss (by kill count)
    pub fn boss_leaderboard(&self, boss_id: u32, limit: usize) -> Vec<(Uuid, u32)> {
        let mut entries: Vec<_> = self.player_data
            .iter()
            .filter_map(|(pid, data)| {
                data.boss_progress.get(&boss_id).map(|p| (*pid, p.kill_count))
            })
            .collect();

        entries.sort_by(|a, b| b.1.cmp(&a.1));
        entries.truncate(limit);
        entries
    }
}

impl Default for BosstiaryManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boss_difficulty_kills() {
        assert_eq!(BossDifficulty::Bane.kills_required(), 5);
        assert_eq!(BossDifficulty::Nemesis.kills_required(), 20);
    }

    #[test]
    fn test_boss_progress() {
        let mut progress = BossProgress::new(1);
        progress.record_kill(5, None);
        progress.record_kill(5, None);

        assert_eq!(progress.kill_count, 2);
        assert!(!progress.is_completed);

        for _ in 0..3 {
            progress.record_kill(5, None);
        }

        assert!(progress.is_completed);
    }
}
