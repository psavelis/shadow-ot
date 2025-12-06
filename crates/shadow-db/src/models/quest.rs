//! Quest model - quests and missions

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Quest definition
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Quest {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub story: Option<String>,
    pub min_level: i32,
    pub max_level: Option<i32>,
    pub premium_only: bool,
    pub repeatable: bool,
    pub cooldown_hours: Option<i32>,
    pub group_size_min: Option<i32>,
    pub group_size_max: Option<i32>,
    pub reward_experience: i64,
    pub reward_gold: i64,
    pub reward_items: serde_json::Value, // Array of item rewards
    pub reward_outfits: serde_json::Value,
    pub reward_mounts: serde_json::Value,
    pub reward_achievements: serde_json::Value,
    pub category: QuestCategory,
    pub difficulty: QuestDifficulty,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "quest_category", rename_all = "lowercase")]
pub enum QuestCategory {
    Main,
    Side,
    Daily,
    Weekly,
    World,
    Event,
    Tutorial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "quest_difficulty", rename_all = "lowercase")]
pub enum QuestDifficulty {
    Easy,
    Medium,
    Hard,
    Expert,
    Legendary,
}

/// Quest mission (sub-tasks within a quest)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct QuestMission {
    pub id: i32,
    pub quest_id: i32,
    pub order_index: i32,
    pub name: String,
    pub description: String,
    pub objective_type: MissionObjective,
    pub objective_target: String, // NPC name, creature ID, item ID, etc.
    pub objective_count: i32,
    pub hidden: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "mission_objective", rename_all = "snake_case")]
pub enum MissionObjective {
    KillCreature,
    CollectItem,
    TalkToNpc,
    ReachLocation,
    UseItem,
    DeliverItem,
    EscortNpc,
    SurviveWaves,
    SolveRiddle,
    Custom,
}

/// Character quest progress
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CharacterQuest {
    pub character_id: Uuid,
    pub quest_id: i32,
    pub status: QuestStatus,
    pub current_mission: i32,
    pub mission_progress: i32,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub last_completion: Option<DateTime<Utc>>,
    pub completion_count: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "quest_status", rename_all = "lowercase")]
pub enum QuestStatus {
    NotStarted,
    InProgress,
    Completed,
    Failed,
}

/// Quest storage (for complex quest state)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CharacterQuestStorage {
    pub character_id: Uuid,
    pub quest_id: i32,
    pub key: String,
    pub value: String,
}
