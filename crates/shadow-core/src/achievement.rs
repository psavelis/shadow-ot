//! Achievement System
//!
//! Tracks player achievements, unlocks, and rewards.
//! Supports various achievement types including kills, exploration,
//! quests, skills, and special achievements.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// Achievement ID
pub type AchievementId = String;

/// Achievement category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AchievementCategory {
    /// Combat achievements (kills, damage, etc.)
    Combat,
    /// Exploration achievements (discover areas, travel)
    Exploration,
    /// Quest-related achievements
    Quests,
    /// Skill-related achievements
    Skills,
    /// Social achievements (party, guild, friends)
    Social,
    /// Economic achievements (gold, trades, market)
    Economy,
    /// Collection achievements (items, outfits, mounts)
    Collection,
    /// Special/seasonal achievements
    Special,
    /// PvP achievements
    PvP,
    /// Boss-related achievements
    Bosses,
}

/// Achievement difficulty/rarity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum AchievementGrade {
    /// Common achievement (1 point)
    Common,
    /// Uncommon achievement (2 points)
    Uncommon,
    /// Rare achievement (3 points)
    Rare,
    /// Epic achievement (5 points)
    Epic,
    /// Legendary achievement (10 points)
    Legendary,
}

impl AchievementGrade {
    /// Get achievement points for this grade
    pub fn points(&self) -> u32 {
        match self {
            AchievementGrade::Common => 1,
            AchievementGrade::Uncommon => 2,
            AchievementGrade::Rare => 3,
            AchievementGrade::Epic => 5,
            AchievementGrade::Legendary => 10,
        }
    }
}

/// Achievement definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    /// Unique ID
    pub id: AchievementId,
    /// Display name
    pub name: String,
    /// Description of how to unlock
    pub description: String,
    /// Secret description (shown before unlock)
    pub secret_description: Option<String>,
    /// Category
    pub category: AchievementCategory,
    /// Difficulty grade
    pub grade: AchievementGrade,
    /// Is this a hidden/secret achievement?
    pub hidden: bool,
    /// Prerequisite achievements
    pub prerequisites: Vec<AchievementId>,
    /// Trigger conditions
    pub conditions: Vec<AchievementCondition>,
    /// Rewards for completing
    pub rewards: AchievementRewards,
    /// Special title granted
    pub title: Option<String>,
    /// Icon/sprite ID
    pub icon_id: u32,
    /// Is this achievement currently available?
    pub available: bool,
    /// Time-limited (seasonal) achievement?
    pub seasonal: bool,
    /// Season identifier if seasonal
    pub season: Option<String>,
}

/// Condition for unlocking an achievement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AchievementCondition {
    /// Kill X monsters of type Y
    KillMonster { monster_id: String, count: u32 },
    /// Kill any X monsters
    KillAnyMonster { count: u32 },
    /// Kill a specific boss
    KillBoss { boss_id: String },
    /// Complete a quest
    CompleteQuest { quest_id: String },
    /// Complete X quests
    CompleteQuestsCount { count: u32 },
    /// Reach level X
    ReachLevel { level: u32 },
    /// Reach skill level X in skill Y
    ReachSkill { skill: String, level: u32 },
    /// Discover a location
    DiscoverLocation { location_id: String },
    /// Visit X different cities
    VisitCities { count: u32 },
    /// Earn X gold total
    EarnGold { amount: u64 },
    /// Spend X gold
    SpendGold { amount: u64 },
    /// Own a house
    OwnHouse,
    /// Create or join a guild
    JoinGuild,
    /// Create a guild
    CreateGuild,
    /// Reach guild rank
    ReachGuildRank { rank: String },
    /// Win X PvP battles
    WinPvPBattles { count: u32 },
    /// Collect X different items
    CollectItems { count: u32 },
    /// Collect specific item
    CollectItem { item_id: u32 },
    /// Own X different outfits
    OwnOutfits { count: u32 },
    /// Own X different mounts
    OwnMounts { count: u32 },
    /// Play for X hours
    PlayTime { hours: u32 },
    /// Login on X different days
    LoginDays { count: u32 },
    /// Die X times
    Deaths { count: u32 },
    /// Deal X total damage
    TotalDamage { amount: u64 },
    /// Heal X total HP
    TotalHealing { amount: u64 },
    /// Custom script condition
    Custom { script_id: String },
}

/// Rewards for completing an achievement
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AchievementRewards {
    /// Experience points
    pub experience: u64,
    /// Gold coins
    pub gold: u64,
    /// Achievement points
    pub achievement_points: u32,
    /// Items (item_id, count)
    pub items: Vec<(u32, u32)>,
    /// Outfit unlocked
    pub outfit: Option<u32>,
    /// Mount unlocked
    pub mount: Option<u32>,
    /// Addon unlocked
    pub addon: Option<(u32, u8)>,
    /// Title unlocked
    pub title: Option<String>,
}

/// Player's achievement progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementProgress {
    /// Achievement ID
    pub achievement_id: AchievementId,
    /// Progress on each condition (condition_index -> current_value)
    pub condition_progress: HashMap<usize, u64>,
    /// Completed conditions
    pub completed_conditions: HashSet<usize>,
    /// Started at
    pub started_at: DateTime<Utc>,
    /// Last updated
    pub updated_at: DateTime<Utc>,
}

impl AchievementProgress {
    /// Create new progress tracker
    pub fn new(achievement_id: AchievementId) -> Self {
        let now = Utc::now();
        Self {
            achievement_id,
            condition_progress: HashMap::new(),
            completed_conditions: HashSet::new(),
            started_at: now,
            updated_at: now,
        }
    }

    /// Update progress on a condition
    pub fn update_condition(&mut self, condition_index: usize, value: u64) {
        self.condition_progress.insert(condition_index, value);
        self.updated_at = Utc::now();
    }

    /// Mark a condition as complete
    pub fn complete_condition(&mut self, condition_index: usize) {
        self.completed_conditions.insert(condition_index);
        self.updated_at = Utc::now();
    }

    /// Check if all conditions are complete
    pub fn all_complete(&self, total_conditions: usize) -> bool {
        self.completed_conditions.len() >= total_conditions
    }
}

/// Player's achievement tracker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAchievements {
    /// Character ID
    pub character_id: Uuid,
    /// Completed achievements with completion time
    pub completed: HashMap<AchievementId, DateTime<Utc>>,
    /// In-progress achievements
    pub in_progress: HashMap<AchievementId, AchievementProgress>,
    /// Total achievement points
    pub total_points: u32,
    /// Titles unlocked
    pub unlocked_titles: Vec<String>,
    /// Current displayed title
    pub current_title: Option<String>,
    /// Statistics for tracking
    pub stats: PlayerStats,
}

impl PlayerAchievements {
    /// Create new achievement tracker for a character
    pub fn new(character_id: Uuid) -> Self {
        Self {
            character_id,
            completed: HashMap::new(),
            in_progress: HashMap::new(),
            total_points: 0,
            unlocked_titles: Vec::new(),
            current_title: None,
            stats: PlayerStats::default(),
        }
    }

    /// Check if an achievement is completed
    pub fn is_completed(&self, achievement_id: &str) -> bool {
        self.completed.contains_key(achievement_id)
    }

    /// Get number of completed achievements
    pub fn completed_count(&self) -> usize {
        self.completed.len()
    }

    /// Get completion percentage (requires total achievements count)
    pub fn completion_percentage(&self, total_achievements: usize) -> f64 {
        if total_achievements == 0 {
            return 0.0;
        }
        (self.completed.len() as f64 / total_achievements as f64) * 100.0
    }

    /// Set displayed title
    pub fn set_title(&mut self, title: Option<String>) -> bool {
        match &title {
            Some(t) if !self.unlocked_titles.contains(t) => false,
            _ => {
                self.current_title = title;
                true
            }
        }
    }
}

/// Player statistics for achievement tracking
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlayerStats {
    /// Total monsters killed
    pub monsters_killed: u64,
    /// Monsters killed by type
    pub monster_kills: HashMap<String, u64>,
    /// Bosses killed
    pub bosses_killed: HashMap<String, u32>,
    /// Total quests completed
    pub quests_completed: u32,
    /// Locations discovered
    pub locations_discovered: HashSet<String>,
    /// Cities visited
    pub cities_visited: HashSet<String>,
    /// Total gold earned
    pub gold_earned: u64,
    /// Total gold spent
    pub gold_spent: u64,
    /// Items collected (unique)
    pub items_collected: HashSet<u32>,
    /// Outfits owned
    pub outfits_owned: HashSet<u32>,
    /// Mounts owned
    pub mounts_owned: HashSet<u32>,
    /// Total play time in minutes
    pub play_time_minutes: u64,
    /// Login days
    pub login_days: u32,
    /// Deaths
    pub deaths: u32,
    /// Total damage dealt
    pub damage_dealt: u64,
    /// Total healing done
    pub healing_done: u64,
    /// PvP wins
    pub pvp_wins: u32,
    /// PvP losses
    pub pvp_losses: u32,
}

/// Achievement manager
pub struct AchievementManager {
    /// All achievement definitions
    achievements: HashMap<AchievementId, Achievement>,
    /// Player achievement data
    player_data: HashMap<Uuid, PlayerAchievements>,
    /// Achievements by category (for faster lookup)
    by_category: HashMap<AchievementCategory, Vec<AchievementId>>,
}

impl AchievementManager {
    /// Create a new achievement manager
    pub fn new() -> Self {
        Self {
            achievements: HashMap::new(),
            player_data: HashMap::new(),
            by_category: HashMap::new(),
        }
    }

    /// Register an achievement
    pub fn register_achievement(&mut self, achievement: Achievement) {
        let id = achievement.id.clone();
        let category = achievement.category;
        
        self.achievements.insert(id.clone(), achievement);
        self.by_category.entry(category)
            .or_insert_with(Vec::new)
            .push(id);
    }

    /// Get achievement by ID
    pub fn get_achievement(&self, id: &str) -> Option<&Achievement> {
        self.achievements.get(id)
    }

    /// Get all achievements
    pub fn all_achievements(&self) -> impl Iterator<Item = &Achievement> {
        self.achievements.values()
    }

    /// Get achievements by category
    pub fn get_by_category(&self, category: AchievementCategory) -> Vec<&Achievement> {
        self.by_category.get(&category)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.achievements.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get or create player achievements
    pub fn get_player(&mut self, character_id: Uuid) -> &PlayerAchievements {
        self.player_data.entry(character_id)
            .or_insert_with(|| PlayerAchievements::new(character_id))
    }

    /// Get mutable player achievements
    pub fn get_player_mut(&mut self, character_id: Uuid) -> &mut PlayerAchievements {
        self.player_data.entry(character_id)
            .or_insert_with(|| PlayerAchievements::new(character_id))
    }

    /// Check and process achievement conditions for a player
    pub fn check_achievements(
        &mut self,
        character_id: Uuid,
    ) -> Vec<AchievementId> {
        let mut newly_completed = Vec::new();
        
        // Get player data
        let player = self.player_data.entry(character_id)
            .or_insert_with(|| PlayerAchievements::new(character_id));
        
        let stats = player.stats.clone();
        let completed = player.completed.keys().cloned().collect::<HashSet<_>>();
        
        // Check each achievement
        for (id, achievement) in &self.achievements {
            // Skip if already completed
            if completed.contains(id) {
                continue;
            }
            
            // Skip if not available
            if !achievement.available {
                continue;
            }
            
            // Check prerequisites
            if !achievement.prerequisites.iter().all(|prereq| completed.contains(prereq)) {
                continue;
            }
            
            // Check all conditions
            let all_met = achievement.conditions.iter().all(|cond| {
                Self::check_condition(cond, &stats)
            });
            
            if all_met {
                newly_completed.push(id.clone());
            }
        }
        
        // Complete the achievements
        for id in &newly_completed {
            self.complete_achievement(character_id, id);
        }
        
        newly_completed
    }

    /// Check if a single condition is met
    fn check_condition(condition: &AchievementCondition, stats: &PlayerStats) -> bool {
        match condition {
            AchievementCondition::KillMonster { monster_id, count } => {
                stats.monster_kills.get(monster_id).copied().unwrap_or(0) >= *count as u64
            }
            AchievementCondition::KillAnyMonster { count } => {
                stats.monsters_killed >= *count as u64
            }
            AchievementCondition::KillBoss { boss_id } => {
                stats.bosses_killed.get(boss_id).copied().unwrap_or(0) > 0
            }
            AchievementCondition::CompleteQuestsCount { count } => {
                stats.quests_completed >= *count
            }
            AchievementCondition::DiscoverLocation { location_id } => {
                stats.locations_discovered.contains(location_id)
            }
            AchievementCondition::VisitCities { count } => {
                stats.cities_visited.len() >= *count as usize
            }
            AchievementCondition::EarnGold { amount } => {
                stats.gold_earned >= *amount
            }
            AchievementCondition::SpendGold { amount } => {
                stats.gold_spent >= *amount
            }
            AchievementCondition::CollectItems { count } => {
                stats.items_collected.len() >= *count as usize
            }
            AchievementCondition::CollectItem { item_id } => {
                stats.items_collected.contains(item_id)
            }
            AchievementCondition::OwnOutfits { count } => {
                stats.outfits_owned.len() >= *count as usize
            }
            AchievementCondition::OwnMounts { count } => {
                stats.mounts_owned.len() >= *count as usize
            }
            AchievementCondition::PlayTime { hours } => {
                stats.play_time_minutes >= (*hours as u64 * 60)
            }
            AchievementCondition::LoginDays { count } => {
                stats.login_days >= *count
            }
            AchievementCondition::Deaths { count } => {
                stats.deaths >= *count
            }
            AchievementCondition::TotalDamage { amount } => {
                stats.damage_dealt >= *amount
            }
            AchievementCondition::TotalHealing { amount } => {
                stats.healing_done >= *amount
            }
            AchievementCondition::WinPvPBattles { count } => {
                stats.pvp_wins >= *count
            }
            // These require additional context checks
            AchievementCondition::ReachLevel { .. } |
            AchievementCondition::ReachSkill { .. } |
            AchievementCondition::CompleteQuest { .. } |
            AchievementCondition::OwnHouse |
            AchievementCondition::JoinGuild |
            AchievementCondition::CreateGuild |
            AchievementCondition::ReachGuildRank { .. } |
            AchievementCondition::Custom { .. } => {
                // These need to be checked with additional context
                false
            }
        }
    }

    /// Complete an achievement for a player
    pub fn complete_achievement(
        &mut self,
        character_id: Uuid,
        achievement_id: &str,
    ) -> Option<&AchievementRewards> {
        let achievement = self.achievements.get(achievement_id)?;
        let rewards = achievement.rewards.clone();
        let grade = achievement.grade;
        let title = achievement.title.clone();
        
        let player = self.get_player_mut(character_id);
        
        // Mark as completed
        player.completed.insert(achievement_id.to_string(), Utc::now());
        
        // Remove from in-progress
        player.in_progress.remove(achievement_id);
        
        // Add points
        player.total_points += grade.points();
        
        // Add title if present
        if let Some(t) = title {
            if !player.unlocked_titles.contains(&t) {
                player.unlocked_titles.push(t);
            }
        }
        
        // Return rewards reference from achievement
        self.achievements.get(achievement_id).map(|a| &a.rewards)
    }

    /// Update player stats and check for new achievements
    pub fn record_monster_kill(
        &mut self,
        character_id: Uuid,
        monster_id: &str,
        is_boss: bool,
    ) -> Vec<AchievementId> {
        let player = self.get_player_mut(character_id);
        player.stats.monsters_killed += 1;
        *player.stats.monster_kills.entry(monster_id.to_string()).or_insert(0) += 1;
        
        if is_boss {
            *player.stats.bosses_killed.entry(monster_id.to_string()).or_insert(0) += 1;
        }
        
        self.check_achievements(character_id)
    }

    /// Record gold earned
    pub fn record_gold_earned(
        &mut self,
        character_id: Uuid,
        amount: u64,
    ) -> Vec<AchievementId> {
        let player = self.get_player_mut(character_id);
        player.stats.gold_earned += amount;
        self.check_achievements(character_id)
    }

    /// Record location discovery
    pub fn record_location_discovered(
        &mut self,
        character_id: Uuid,
        location_id: &str,
    ) -> Vec<AchievementId> {
        let player = self.get_player_mut(character_id);
        player.stats.locations_discovered.insert(location_id.to_string());
        self.check_achievements(character_id)
    }

    /// Record quest completion
    pub fn record_quest_completed(
        &mut self,
        character_id: Uuid,
        _quest_id: &str,
    ) -> Vec<AchievementId> {
        let player = self.get_player_mut(character_id);
        player.stats.quests_completed += 1;
        self.check_achievements(character_id)
    }

    /// Get total achievement count
    pub fn total_achievement_count(&self) -> usize {
        self.achievements.len()
    }

    /// Get achievement leaderboard
    pub fn get_leaderboard(&self, limit: usize) -> Vec<(Uuid, u32)> {
        let mut scores: Vec<_> = self.player_data.iter()
            .map(|(id, data)| (*id, data.total_points))
            .collect();
        
        scores.sort_by(|a, b| b.1.cmp(&a.1));
        scores.truncate(limit);
        scores
    }
}

impl Default for AchievementManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Create default achievements
pub fn create_default_achievements() -> Vec<Achievement> {
    vec![
        Achievement {
            id: "first_blood".to_string(),
            name: "First Blood".to_string(),
            description: "Kill your first monster".to_string(),
            secret_description: None,
            category: AchievementCategory::Combat,
            grade: AchievementGrade::Common,
            hidden: false,
            prerequisites: Vec::new(),
            conditions: vec![AchievementCondition::KillAnyMonster { count: 1 }],
            rewards: AchievementRewards {
                experience: 100,
                achievement_points: 1,
                ..Default::default()
            },
            title: None,
            icon_id: 1,
            available: true,
            seasonal: false,
            season: None,
        },
        Achievement {
            id: "monster_hunter".to_string(),
            name: "Monster Hunter".to_string(),
            description: "Kill 1000 monsters".to_string(),
            secret_description: None,
            category: AchievementCategory::Combat,
            grade: AchievementGrade::Uncommon,
            hidden: false,
            prerequisites: vec!["first_blood".to_string()],
            conditions: vec![AchievementCondition::KillAnyMonster { count: 1000 }],
            rewards: AchievementRewards {
                experience: 5000,
                gold: 1000,
                achievement_points: 2,
                ..Default::default()
            },
            title: Some("Monster Hunter".to_string()),
            icon_id: 2,
            available: true,
            seasonal: false,
            season: None,
        },
        Achievement {
            id: "explorer".to_string(),
            name: "Explorer".to_string(),
            description: "Visit 10 different cities".to_string(),
            secret_description: None,
            category: AchievementCategory::Exploration,
            grade: AchievementGrade::Uncommon,
            hidden: false,
            prerequisites: Vec::new(),
            conditions: vec![AchievementCondition::VisitCities { count: 10 }],
            rewards: AchievementRewards {
                experience: 2500,
                achievement_points: 2,
                ..Default::default()
            },
            title: Some("the Explorer".to_string()),
            icon_id: 10,
            available: true,
            seasonal: false,
            season: None,
        },
        Achievement {
            id: "millionaire".to_string(),
            name: "Millionaire".to_string(),
            description: "Earn 1,000,000 gold in total".to_string(),
            secret_description: None,
            category: AchievementCategory::Economy,
            grade: AchievementGrade::Rare,
            hidden: false,
            prerequisites: Vec::new(),
            conditions: vec![AchievementCondition::EarnGold { amount: 1_000_000 }],
            rewards: AchievementRewards {
                experience: 10000,
                gold: 10000,
                achievement_points: 3,
                ..Default::default()
            },
            title: Some("the Wealthy".to_string()),
            icon_id: 20,
            available: true,
            seasonal: false,
            season: None,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_achievement_completion() {
        let mut manager = AchievementManager::new();
        
        // Register default achievements
        for achievement in create_default_achievements() {
            manager.register_achievement(achievement);
        }
        
        let char_id = Uuid::new_v4();
        
        // Kill a monster
        let completed = manager.record_monster_kill(char_id, "rat", false);
        assert!(completed.contains(&"first_blood".to_string()));
        
        let player = manager.get_player(char_id);
        assert!(player.is_completed("first_blood"));
    }

    #[test]
    fn test_achievement_points() {
        assert_eq!(AchievementGrade::Common.points(), 1);
        assert_eq!(AchievementGrade::Legendary.points(), 10);
    }
}
