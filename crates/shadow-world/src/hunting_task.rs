//! Hunting Task System (Killing in the Name of...)
//!
//! Implements Tibia's task hunting system where players:
//! - Receive tasks to kill specific monsters
//! - Track progress towards kill counts
//! - Earn rewards and access to boss monsters
//! - Progress through task points for ranks

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Task difficulty tier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TaskDifficulty {
    /// Entry level tasks (e.g., Trolls, Goblins)
    Paw,
    /// Mid level tasks (e.g., Minotaurs, Dwarfs)
    Antler,
    /// Advanced tasks (e.g., Dragons, Demons)
    Horn,
    /// Boss tasks
    Trophy,
}

impl TaskDifficulty {
    /// Get the minimum level for this difficulty
    pub fn min_level(&self) -> u16 {
        match self {
            Self::Paw => 8,
            Self::Antler => 40,
            Self::Horn => 80,
            Self::Trophy => 130,
        }
    }

    /// Get task points rewarded
    pub fn task_points(&self) -> u32 {
        match self {
            Self::Paw => 1,
            Self::Antler => 2,
            Self::Horn => 3,
            Self::Trophy => 5,
        }
    }

    /// Get experience reward multiplier
    pub fn experience_multiplier(&self) -> f32 {
        match self {
            Self::Paw => 1.0,
            Self::Antler => 2.0,
            Self::Horn => 4.0,
            Self::Trophy => 8.0,
        }
    }

    /// Get gold reward base
    pub fn gold_base(&self) -> u64 {
        match self {
            Self::Paw => 1000,
            Self::Antler => 5000,
            Self::Horn => 20000,
            Self::Trophy => 100000,
        }
    }
}

/// Task rank (determines available tasks)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TaskRank {
    /// New hunter (0 points)
    Huntsman,
    /// First rank (100 points)
    RangerKnight,
    /// Second rank (500 points)
    BigGameHunter,
    /// Third rank (1000 points)
    TrophyHunter,
    /// Fourth rank (2500 points)
    Elite,
}

impl TaskRank {
    /// Get rank from points
    pub fn from_points(points: u32) -> Self {
        match points {
            0..=99 => Self::Huntsman,
            100..=499 => Self::RangerKnight,
            500..=999 => Self::BigGameHunter,
            1000..=2499 => Self::TrophyHunter,
            _ => Self::Elite,
        }
    }

    /// Get points needed for next rank
    pub fn points_for_next(&self) -> Option<u32> {
        match self {
            Self::Huntsman => Some(100),
            Self::RangerKnight => Some(500),
            Self::BigGameHunter => Some(1000),
            Self::TrophyHunter => Some(2500),
            Self::Elite => None,
        }
    }

    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Huntsman => "Huntsman",
            Self::RangerKnight => "Ranger Knight",
            Self::BigGameHunter => "Big Game Hunter",
            Self::TrophyHunter => "Trophy Hunter",
            Self::Elite => "Elite Hunter",
        }
    }

    /// Get outfit addon unlocked at this rank
    pub fn addon_unlocked(&self) -> Option<u8> {
        match self {
            Self::RangerKnight => Some(1),
            Self::TrophyHunter => Some(2),
            Self::Elite => Some(3),
            _ => None,
        }
    }

    /// Get boss access granted at this rank
    pub fn boss_access(&self) -> Vec<&'static str> {
        match self {
            Self::RangerKnight => vec!["Bloodcrab"],
            Self::BigGameHunter => vec!["Bloodcrab", "Deathstrike"],
            Self::TrophyHunter => vec!["Bloodcrab", "Deathstrike", "Shardhead"],
            Self::Elite => vec!["Bloodcrab", "Deathstrike", "Shardhead", "Thul"],
            _ => vec![],
        }
    }
}

/// A hunting task definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HuntingTask {
    /// Task ID
    pub id: u32,
    /// Monster race name
    pub monster_name: String,
    /// Monster race IDs that count
    pub race_ids: Vec<u16>,
    /// Task difficulty
    pub difficulty: TaskDifficulty,
    /// Number of kills required
    pub kill_count: u32,
    /// Minimum level to accept
    pub min_level: u16,
    /// Experience reward
    pub experience_reward: u64,
    /// Gold reward
    pub gold_reward: u64,
    /// Item rewards (item_id, count)
    pub item_rewards: Vec<(u16, u32)>,
    /// Whether task is repeatable
    pub repeatable: bool,
    /// Cooldown in hours (if repeatable)
    pub cooldown_hours: u32,
    /// Unlocks boss
    pub unlocks_boss: Option<String>,
    /// Description
    pub description: String,
    /// Hunting locations hint
    pub location_hint: String,
}

impl HuntingTask {
    /// Create a new hunting task
    pub fn new(
        id: u32,
        monster_name: impl Into<String>,
        race_ids: Vec<u16>,
        difficulty: TaskDifficulty,
        kill_count: u32,
    ) -> Self {
        let exp = (kill_count as u64) * (difficulty.experience_multiplier() as u64) * 100;
        let gold = difficulty.gold_base() * (kill_count as u64 / 100);

        Self {
            id,
            monster_name: monster_name.into(),
            race_ids,
            difficulty,
            kill_count,
            min_level: difficulty.min_level(),
            experience_reward: exp,
            gold_reward: gold,
            item_rewards: Vec::new(),
            repeatable: true,
            cooldown_hours: 20,
            unlocks_boss: None,
            description: String::new(),
            location_hint: String::new(),
        }
    }

    /// Set custom experience reward
    pub fn with_experience(mut self, exp: u64) -> Self {
        self.experience_reward = exp;
        self
    }

    /// Set custom gold reward
    pub fn with_gold(mut self, gold: u64) -> Self {
        self.gold_reward = gold;
        self
    }

    /// Add item reward
    pub fn with_item(mut self, item_id: u16, count: u32) -> Self {
        self.item_rewards.push((item_id, count));
        self
    }

    /// Set boss unlock
    pub fn with_boss(mut self, boss_name: impl Into<String>) -> Self {
        self.unlocks_boss = Some(boss_name.into());
        self
    }

    /// Set description
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    /// Set location hint
    pub fn with_location(mut self, loc: impl Into<String>) -> Self {
        self.location_hint = loc.into();
        self
    }
}

/// Player's active task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveTask {
    /// Task ID
    pub task_id: u32,
    /// Current kill count
    pub kills: u32,
    /// Started time
    pub started_at: DateTime<Utc>,
}

impl ActiveTask {
    /// Create new active task
    pub fn new(task_id: u32) -> Self {
        Self {
            task_id,
            kills: 0,
            started_at: Utc::now(),
        }
    }

    /// Record a kill
    pub fn record_kill(&mut self) {
        self.kills += 1;
    }

    /// Check if complete for given requirement
    pub fn is_complete(&self, required: u32) -> bool {
        self.kills >= required
    }

    /// Get progress percentage
    pub fn progress_percent(&self, required: u32) -> f32 {
        (self.kills as f32 / required as f32 * 100.0).min(100.0)
    }
}

/// Task completion record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCompletion {
    /// Task ID
    pub task_id: u32,
    /// Completed time
    pub completed_at: DateTime<Utc>,
    /// Can repeat after (if repeatable)
    pub cooldown_until: Option<DateTime<Utc>>,
}

/// Player's task progress
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlayerTaskProgress {
    /// Player ID
    pub player_id: u32,
    /// Total task points earned
    pub task_points: u32,
    /// Current rank
    pub rank: TaskRank,
    /// Active tasks (max 3)
    pub active_tasks: Vec<ActiveTask>,
    /// Completed tasks history
    pub completed_tasks: Vec<TaskCompletion>,
    /// Total tasks completed
    pub total_completed: u32,
    /// Bosses unlocked
    pub bosses_unlocked: Vec<String>,
}

impl PlayerTaskProgress {
    /// Create new player progress
    pub fn new(player_id: u32) -> Self {
        Self {
            player_id,
            task_points: 0,
            rank: TaskRank::Huntsman,
            active_tasks: Vec::new(),
            completed_tasks: Vec::new(),
            total_completed: 0,
            bosses_unlocked: Vec::new(),
        }
    }

    /// Check if can accept a new task
    pub fn can_accept_task(&self) -> bool {
        self.active_tasks.len() < 3
    }

    /// Check if player meets level requirement
    pub fn meets_level_requirement(&self, task: &HuntingTask, player_level: u16) -> bool {
        player_level >= task.min_level
    }

    /// Check if task is on cooldown
    pub fn is_task_on_cooldown(&self, task_id: u32) -> bool {
        let now = Utc::now();
        self.completed_tasks.iter().any(|c| {
            c.task_id == task_id
                && c.cooldown_until.map(|until| until > now).unwrap_or(false)
        })
    }

    /// Accept a task
    pub fn accept_task(&mut self, task_id: u32) -> bool {
        if !self.can_accept_task() {
            return false;
        }

        if self.active_tasks.iter().any(|t| t.task_id == task_id) {
            return false;
        }

        self.active_tasks.push(ActiveTask::new(task_id));
        true
    }

    /// Cancel a task
    pub fn cancel_task(&mut self, task_id: u32) -> bool {
        let initial_len = self.active_tasks.len();
        self.active_tasks.retain(|t| t.task_id != task_id);
        self.active_tasks.len() < initial_len
    }

    /// Record kill for task
    pub fn record_kill(&mut self, race_id: u16, tasks: &HashMap<u32, HuntingTask>) -> Vec<u32> {
        let mut completed = Vec::new();

        for task in &mut self.active_tasks {
            if let Some(task_def) = tasks.get(&task.task_id) {
                if task_def.race_ids.contains(&race_id) {
                    task.record_kill();
                    if task.is_complete(task_def.kill_count) {
                        completed.push(task.task_id);
                    }
                }
            }
        }

        completed
    }

    /// Complete a task
    pub fn complete_task(&mut self, task: &HuntingTask) -> bool {
        let active_pos = self.active_tasks.iter().position(|t| t.task_id == task.id);
        if active_pos.is_none() {
            return false;
        }

        let active = self.active_tasks.remove(active_pos.unwrap());
        if !active.is_complete(task.kill_count) {
            self.active_tasks.push(active);
            return false;
        }

        // Add points
        self.task_points += task.difficulty.task_points();
        self.total_completed += 1;

        // Update rank
        self.rank = TaskRank::from_points(self.task_points);

        // Record completion
        let cooldown_until = if task.repeatable {
            Some(Utc::now() + chrono::Duration::hours(task.cooldown_hours as i64))
        } else {
            None
        };

        self.completed_tasks.push(TaskCompletion {
            task_id: task.id,
            completed_at: Utc::now(),
            cooldown_until,
        });

        // Unlock boss if applicable
        if let Some(ref boss) = task.unlocks_boss {
            if !self.bosses_unlocked.contains(boss) {
                self.bosses_unlocked.push(boss.clone());
            }
        }

        true
    }

    /// Get active task by ID
    pub fn get_active_task(&self, task_id: u32) -> Option<&ActiveTask> {
        self.active_tasks.iter().find(|t| t.task_id == task_id)
    }

    /// Get points needed for next rank
    pub fn points_to_next_rank(&self) -> Option<u32> {
        self.rank.points_for_next().map(|next| {
            if next > self.task_points {
                next - self.task_points
            } else {
                0
            }
        })
    }
}

impl Default for TaskRank {
    fn default() -> Self {
        Self::Huntsman
    }
}

/// Task giver NPC info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskGiver {
    /// NPC name
    pub name: String,
    /// Position
    pub position: (u16, u16, u8),
    /// Difficulties this NPC handles
    pub difficulties: Vec<TaskDifficulty>,
    /// Greeting message
    pub greeting: String,
}

/// Hunting task manager
#[derive(Debug)]
pub struct TaskManager {
    /// Task definitions
    tasks: HashMap<u32, HuntingTask>,
    /// Tasks by difficulty
    tasks_by_difficulty: HashMap<TaskDifficulty, Vec<u32>>,
    /// Tasks by race ID
    tasks_by_race: HashMap<u16, Vec<u32>>,
    /// Player progress
    player_progress: HashMap<u32, PlayerTaskProgress>,
    /// Task givers
    task_givers: Vec<TaskGiver>,
}

impl TaskManager {
    /// Create new task manager
    pub fn new() -> Self {
        let mut manager = Self {
            tasks: HashMap::new(),
            tasks_by_difficulty: HashMap::new(),
            tasks_by_race: HashMap::new(),
            player_progress: HashMap::new(),
            task_givers: Vec::new(),
        };

        // Initialize default tasks
        manager.init_default_tasks();
        manager
    }

    /// Initialize default hunting tasks
    fn init_default_tasks(&mut self) {
        // Paw difficulty (entry level)
        self.register_task(HuntingTask::new(1, "Trolls", vec![10, 11], TaskDifficulty::Paw, 300)
            .with_description("Hunt down the trolls that terrorize travelers.")
            .with_location("Thais trolls, Edron trolls"));

        self.register_task(HuntingTask::new(2, "Goblins", vec![20, 21], TaskDifficulty::Paw, 300)
            .with_description("Clear out goblin camps.")
            .with_location("Femor Hills, Kazordoon"));

        self.register_task(HuntingTask::new(3, "Rotworms", vec![30], TaskDifficulty::Paw, 400)
            .with_description("Cull the rotworm population.")
            .with_location("Darashia rotworms, Venore swamps"));

        self.register_task(HuntingTask::new(4, "Cyclops", vec![40], TaskDifficulty::Paw, 500)
            .with_description("Defeat cyclops in their lairs.")
            .with_location("Mount Sternum, Cyclopolis"));

        // Antler difficulty (mid level)
        self.register_task(HuntingTask::new(10, "Minotaurs", vec![50, 51, 52], TaskDifficulty::Antler, 400)
            .with_description("Brave the minotaur mazes.")
            .with_location("Mintwallin, Minotaur Pyramid"));

        self.register_task(HuntingTask::new(11, "Giant Spiders", vec![60], TaskDifficulty::Antler, 400)
            .with_description("Clear spider nests.")
            .with_location("Port Hope, Plains of Havoc"));

        self.register_task(HuntingTask::new(12, "Dragons", vec![70], TaskDifficulty::Antler, 500)
            .with_description("Slay the fire-breathing beasts.")
            .with_location("Thais dragon lair, Darashia dragons")
            .with_boss("Deathstrike"));

        // Horn difficulty (advanced)
        self.register_task(HuntingTask::new(20, "Dragon Lords", vec![80], TaskDifficulty::Horn, 400)
            .with_description("Hunt the dragon lords.")
            .with_location("Goroma, Razachai"));

        self.register_task(HuntingTask::new(21, "Hydras", vec![90], TaskDifficulty::Horn, 400)
            .with_description("Defeat the many-headed hydras.")
            .with_location("Hydra spawn Talahu"));

        self.register_task(HuntingTask::new(22, "Demons", vec![100], TaskDifficulty::Horn, 666)
            .with_description("Vanquish the demons of the deep.")
            .with_location("Demon spawn Edron, Goroma")
            .with_boss("Shardhead"));

        // Trophy difficulty (boss tasks)
        self.register_task(HuntingTask::new(30, "Hellspawns", vec![110], TaskDifficulty::Trophy, 600)
            .with_description("Destroy the hellspawn legions.")
            .with_location("Oramond, Roshamuul"));

        self.register_task(HuntingTask::new(31, "Grim Reapers", vec![120], TaskDifficulty::Trophy, 500)
            .with_description("End the grim reapers.")
            .with_location("Drefia, Oramond")
            .with_boss("Thul"));
    }

    /// Register a task
    pub fn register_task(&mut self, task: HuntingTask) {
        let id = task.id;
        let difficulty = task.difficulty;

        // Index by race
        for &race_id in &task.race_ids {
            self.tasks_by_race.entry(race_id).or_default().push(id);
        }

        // Index by difficulty
        self.tasks_by_difficulty.entry(difficulty).or_default().push(id);

        self.tasks.insert(id, task);
    }

    /// Get task by ID
    pub fn get_task(&self, task_id: u32) -> Option<&HuntingTask> {
        self.tasks.get(&task_id)
    }

    /// Get tasks by difficulty
    pub fn get_tasks_by_difficulty(&self, difficulty: TaskDifficulty) -> Vec<&HuntingTask> {
        self.tasks_by_difficulty
            .get(&difficulty)
            .map(|ids| ids.iter().filter_map(|id| self.tasks.get(id)).collect())
            .unwrap_or_default()
    }

    /// Get available tasks for player
    pub fn get_available_tasks(&self, player_id: u32, player_level: u16) -> Vec<&HuntingTask> {
        let progress = self.player_progress.get(&player_id);

        self.tasks.values()
            .filter(|task| {
                // Check level requirement
                if player_level < task.min_level {
                    return false;
                }

                // Check cooldown
                if let Some(prog) = progress {
                    if prog.is_task_on_cooldown(task.id) {
                        return false;
                    }
                    // Check if already active
                    if prog.active_tasks.iter().any(|t| t.task_id == task.id) {
                        return false;
                    }
                }

                true
            })
            .collect()
    }

    /// Get or create player progress
    pub fn get_player_progress(&mut self, player_id: u32) -> &mut PlayerTaskProgress {
        self.player_progress
            .entry(player_id)
            .or_insert_with(|| PlayerTaskProgress::new(player_id))
    }

    /// Get player progress (read-only)
    pub fn get_progress(&self, player_id: u32) -> Option<&PlayerTaskProgress> {
        self.player_progress.get(&player_id)
    }

    /// Accept task for player
    pub fn accept_task(&mut self, player_id: u32, task_id: u32, player_level: u16) -> bool {
        // Check task exists and player meets requirements
        let task = match self.tasks.get(&task_id) {
            Some(t) => t.clone(),
            None => return false,
        };

        if player_level < task.min_level {
            return false;
        }

        let progress = self.get_player_progress(player_id);
        
        if progress.is_task_on_cooldown(task_id) {
            return false;
        }

        progress.accept_task(task_id)
    }

    /// Record kill
    pub fn record_kill(&mut self, player_id: u32, race_id: u16) -> Vec<TaskCompletionEvent> {
        let tasks = &self.tasks;
        let progress = match self.player_progress.get_mut(&player_id) {
            Some(p) => p,
            None => return Vec::new(),
        };

        let completed_ids = progress.record_kill(race_id, tasks);
        
        completed_ids.iter()
            .filter_map(|&task_id| {
                self.tasks.get(&task_id).map(|task| TaskCompletionEvent {
                    task_id,
                    task_name: task.monster_name.clone(),
                    experience: task.experience_reward,
                    gold: task.gold_reward,
                    task_points: task.difficulty.task_points(),
                })
            })
            .collect()
    }

    /// Complete task and claim rewards
    pub fn complete_task(&mut self, player_id: u32, task_id: u32) -> Option<TaskRewards> {
        let task = self.tasks.get(&task_id)?.clone();
        let progress = self.player_progress.get_mut(&player_id)?;

        if !progress.complete_task(&task) {
            return None;
        }

        Some(TaskRewards {
            experience: task.experience_reward,
            gold: task.gold_reward,
            task_points: task.difficulty.task_points(),
            items: task.item_rewards.clone(),
            boss_unlocked: task.unlocks_boss.clone(),
            new_rank: if progress.rank > TaskRank::from_points(progress.task_points - task.difficulty.task_points()) {
                Some(progress.rank)
            } else {
                None
            },
        })
    }

    /// Register task giver NPC
    pub fn register_task_giver(&mut self, giver: TaskGiver) {
        self.task_givers.push(giver);
    }

    /// Get task givers
    pub fn get_task_givers(&self) -> &[TaskGiver] {
        &self.task_givers
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Task completion event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCompletionEvent {
    pub task_id: u32,
    pub task_name: String,
    pub experience: u64,
    pub gold: u64,
    pub task_points: u32,
}

/// Task rewards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskRewards {
    pub experience: u64,
    pub gold: u64,
    pub task_points: u32,
    pub items: Vec<(u16, u32)>,
    pub boss_unlocked: Option<String>,
    pub new_rank: Option<TaskRank>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_rank() {
        assert_eq!(TaskRank::from_points(0), TaskRank::Huntsman);
        assert_eq!(TaskRank::from_points(100), TaskRank::RangerKnight);
        assert_eq!(TaskRank::from_points(2500), TaskRank::Elite);
    }

    #[test]
    fn test_player_progress() {
        let mut progress = PlayerTaskProgress::new(1);
        
        assert!(progress.can_accept_task());
        assert!(progress.accept_task(1));
        assert!(progress.accept_task(2));
        assert!(progress.accept_task(3));
        assert!(!progress.can_accept_task()); // Max 3 tasks
    }

    #[test]
    fn test_task_manager() {
        let mut manager = TaskManager::new();
        
        // Accept task
        assert!(manager.accept_task(1, 1, 50));
        
        // Record kills
        for _ in 0..300 {
            manager.record_kill(1, 10);
        }

        // Check completion
        let progress = manager.get_progress(1).unwrap();
        let active = progress.get_active_task(1).unwrap();
        assert!(active.is_complete(300));
    }
}
