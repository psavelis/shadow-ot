//! Quest Scripting
//!
//! Handles quest definitions, triggers, and progression.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Quest state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestState {
    /// Not started
    NotStarted,
    /// In progress
    InProgress,
    /// Completed
    Completed,
    /// Failed
    Failed,
}

impl Default for QuestState {
    fn default() -> Self {
        Self::NotStarted
    }
}

/// Quest objective types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestObjective {
    /// Kill a certain monster type
    Kill {
        monster: String,
        count: u32,
    },
    /// Collect items
    Collect {
        item_id: u16,
        count: u32,
    },
    /// Talk to NPC
    TalkTo {
        npc: String,
        keyword: Option<String>,
    },
    /// Reach a location
    ReachLocation {
        x: u16,
        y: u16,
        z: u8,
        radius: u8,
    },
    /// Use an item at a location
    UseItem {
        item_id: u16,
        x: u16,
        y: u16,
        z: u8,
    },
    /// Complete another quest
    CompleteQuest {
        quest_id: String,
    },
    /// Custom Lua check
    Custom {
        script: String,
    },
}

/// Quest trigger conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestTrigger {
    /// Triggered by talking to NPC
    NpcDialog {
        npc: String,
        keyword: String,
    },
    /// Triggered by entering area
    EnterArea {
        x: u16,
        y: u16,
        z: u8,
        radius: u8,
    },
    /// Triggered by killing monster
    MonsterKill {
        monster: String,
    },
    /// Triggered by using item
    ItemUse {
        item_id: u16,
        action_id: Option<u16>,
    },
    /// Triggered by level up
    LevelUp {
        level: u16,
    },
    /// Triggered by completing another quest
    QuestComplete {
        quest_id: String,
    },
    /// Triggered by time/schedule
    Scheduled {
        cron: String,
    },
}

/// Quest reward
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestReward {
    /// Experience points
    pub experience: u64,
    /// Gold
    pub gold: u32,
    /// Items
    pub items: Vec<RewardItem>,
    /// Spells unlocked
    pub spells: Vec<String>,
    /// Outfit unlocked
    pub outfit: Option<u16>,
    /// Addon unlocked
    pub addon: Option<u8>,
    /// Mount unlocked
    pub mount: Option<u16>,
    /// Storage values set
    pub storage: HashMap<u32, i32>,
}

impl Default for QuestReward {
    fn default() -> Self {
        Self {
            experience: 0,
            gold: 0,
            items: Vec::new(),
            spells: Vec::new(),
            outfit: None,
            addon: None,
            mount: None,
            storage: HashMap::new(),
        }
    }
}

impl QuestReward {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn experience(mut self, exp: u64) -> Self {
        self.experience = exp;
        self
    }

    pub fn gold(mut self, amount: u32) -> Self {
        self.gold = amount;
        self
    }

    pub fn item(mut self, item_id: u16, count: u16) -> Self {
        self.items.push(RewardItem { item_id, count });
        self
    }
}

/// Item given as reward
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardItem {
    pub item_id: u16,
    pub count: u16,
}

/// A complete quest definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestScript {
    /// Unique quest identifier
    pub id: String,
    /// Quest name
    pub name: String,
    /// Quest description
    pub description: String,
    /// Minimum level requirement
    pub min_level: u16,
    /// Maximum level (0 = no limit)
    pub max_level: u16,
    /// Required vocation (empty = any)
    pub vocations: Vec<String>,
    /// Required quests to be completed
    pub prerequisites: Vec<String>,
    /// Quest stages/missions
    pub stages: Vec<QuestStage>,
    /// Trigger to start quest
    pub start_trigger: Option<QuestTrigger>,
    /// Whether quest is repeatable
    pub repeatable: bool,
    /// Cooldown for repeatable quests (seconds)
    pub cooldown: u32,
    /// Final rewards
    pub rewards: QuestReward,
    /// Quest log group
    pub group: String,
}

impl QuestScript {
    /// Create a new quest
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: String::new(),
            min_level: 1,
            max_level: 0,
            vocations: Vec::new(),
            prerequisites: Vec::new(),
            stages: Vec::new(),
            start_trigger: None,
            repeatable: false,
            cooldown: 0,
            rewards: QuestReward::default(),
            group: "default".to_string(),
        }
    }

    /// Set description
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    /// Add a stage
    pub fn add_stage(mut self, stage: QuestStage) -> Self {
        self.stages.push(stage);
        self
    }

    /// Set rewards
    pub fn rewards(mut self, rewards: QuestReward) -> Self {
        self.rewards = rewards;
        self
    }

    /// Check if player meets requirements
    pub fn can_start(&self, player_level: u16, player_vocation: &str, completed_quests: &[String]) -> bool {
        if player_level < self.min_level {
            return false;
        }
        if self.max_level > 0 && player_level > self.max_level {
            return false;
        }
        if !self.vocations.is_empty() && !self.vocations.iter().any(|v| v == player_vocation) {
            return false;
        }
        for prereq in &self.prerequisites {
            if !completed_quests.contains(prereq) {
                return false;
            }
        }
        true
    }
}

/// A stage/mission within a quest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestStage {
    /// Stage number
    pub stage: u8,
    /// Stage name
    pub name: String,
    /// Stage description
    pub description: String,
    /// Objectives to complete
    pub objectives: Vec<QuestObjective>,
    /// Rewards for completing this stage
    pub rewards: Option<QuestReward>,
}

impl QuestStage {
    pub fn new(stage: u8, name: impl Into<String>) -> Self {
        Self {
            stage,
            name: name.into(),
            description: String::new(),
            objectives: Vec::new(),
            rewards: None,
        }
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    pub fn add_objective(mut self, objective: QuestObjective) -> Self {
        self.objectives.push(objective);
        self
    }
}

/// Player's progress in a quest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestProgress {
    /// Quest ID
    pub quest_id: String,
    /// Current state
    pub state: QuestState,
    /// Current stage
    pub current_stage: u8,
    /// Objective progress (objective index -> current count)
    pub objective_progress: HashMap<usize, u32>,
    /// When quest was started
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    /// When quest was completed
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl QuestProgress {
    pub fn new(quest_id: impl Into<String>) -> Self {
        Self {
            quest_id: quest_id.into(),
            state: QuestState::NotStarted,
            current_stage: 0,
            objective_progress: HashMap::new(),
            started_at: None,
            completed_at: None,
        }
    }

    /// Start the quest
    pub fn start(&mut self) {
        self.state = QuestState::InProgress;
        self.current_stage = 1;
        self.started_at = Some(chrono::Utc::now());
    }

    /// Complete the quest
    pub fn complete(&mut self) {
        self.state = QuestState::Completed;
        self.completed_at = Some(chrono::Utc::now());
    }

    /// Update objective progress
    pub fn update_objective(&mut self, objective_idx: usize, value: u32) {
        self.objective_progress.insert(objective_idx, value);
    }

    /// Get objective progress
    pub fn get_progress(&self, objective_idx: usize) -> u32 {
        self.objective_progress.get(&objective_idx).copied().unwrap_or(0)
    }
}

/// Manages quests and player progress
pub struct QuestManager {
    /// All quest definitions
    quests: HashMap<String, QuestScript>,
    /// Player progress (player_id -> quest_id -> progress)
    progress: HashMap<Uuid, HashMap<String, QuestProgress>>,
}

impl QuestManager {
    pub fn new() -> Self {
        Self {
            quests: HashMap::new(),
            progress: HashMap::new(),
        }
    }

    /// Register a quest
    pub fn register(&mut self, quest: QuestScript) {
        self.quests.insert(quest.id.clone(), quest);
    }

    /// Get quest definition
    pub fn get_quest(&self, id: &str) -> Option<&QuestScript> {
        self.quests.get(id)
    }

    /// Start quest for player
    pub fn start_quest(&mut self, player_id: Uuid, quest_id: &str) -> Result<(), &'static str> {
        if !self.quests.contains_key(quest_id) {
            return Err("Quest not found");
        }

        let player_progress = self.progress.entry(player_id).or_default();
        
        if player_progress.get(quest_id).map_or(false, |p| p.state == QuestState::InProgress) {
            return Err("Quest already in progress");
        }

        let mut progress = QuestProgress::new(quest_id);
        progress.start();
        player_progress.insert(quest_id.to_string(), progress);

        Ok(())
    }

    /// Get player's progress for a quest
    pub fn get_progress(&self, player_id: Uuid, quest_id: &str) -> Option<&QuestProgress> {
        self.progress.get(&player_id)?.get(quest_id)
    }

    /// Get mutable progress
    pub fn get_progress_mut(&mut self, player_id: Uuid, quest_id: &str) -> Option<&mut QuestProgress> {
        self.progress.get_mut(&player_id)?.get_mut(quest_id)
    }

    /// Check if player completed a quest
    pub fn is_completed(&self, player_id: Uuid, quest_id: &str) -> bool {
        self.get_progress(player_id, quest_id)
            .map_or(false, |p| p.state == QuestState::Completed)
    }

    /// Get all quests player can start
    pub fn available_quests(&self, player_id: Uuid, player_level: u16, player_vocation: &str) -> Vec<&QuestScript> {
        let completed: Vec<String> = self.progress
            .get(&player_id)
            .map(|p| {
                p.iter()
                    .filter(|(_, prog)| prog.state == QuestState::Completed)
                    .map(|(id, _)| id.clone())
                    .collect()
            })
            .unwrap_or_default();

        self.quests
            .values()
            .filter(|q| {
                // Not already started/completed
                !self.progress
                    .get(&player_id)
                    .map_or(false, |p| p.contains_key(&q.id))
                && q.can_start(player_level, player_vocation, &completed)
            })
            .collect()
    }

    /// Load quests from JSON
    pub fn load_from_json(&mut self, json: &str) -> Result<usize, serde_json::Error> {
        let quests: Vec<QuestScript> = serde_json::from_str(json)?;
        let count = quests.len();
        for quest in quests {
            self.register(quest);
        }
        Ok(count)
    }
}

impl Default for QuestManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quest_creation() {
        let quest = QuestScript::new("test_quest", "Test Quest")
            .description("A test quest")
            .add_stage(
                QuestStage::new(1, "Stage 1")
                    .add_objective(QuestObjective::Kill {
                        monster: "Rat".to_string(),
                        count: 10,
                    })
            )
            .rewards(QuestReward::new().experience(1000).gold(100));

        assert_eq!(quest.id, "test_quest");
        assert_eq!(quest.stages.len(), 1);
    }

    #[test]
    fn test_quest_requirements() {
        let quest = QuestScript::new("level_quest", "Level Quest");
        assert!(quest.can_start(10, "knight", &[]));

        let quest = QuestScript {
            min_level: 50,
            ..QuestScript::new("high_level", "High Level")
        };
        assert!(!quest.can_start(10, "knight", &[]));
        assert!(quest.can_start(50, "knight", &[]));
    }

    #[test]
    fn test_quest_manager() {
        let mut manager = QuestManager::new();
        let player_id = Uuid::new_v4();

        manager.register(QuestScript::new("test", "Test"));
        
        assert!(manager.start_quest(player_id, "test").is_ok());
        assert!(manager.start_quest(player_id, "test").is_err()); // Already started
        
        let progress = manager.get_progress(player_id, "test").unwrap();
        assert_eq!(progress.state, QuestState::InProgress);
    }
}
