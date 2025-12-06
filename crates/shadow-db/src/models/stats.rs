//! Statistics and analytics models

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Daily server statistics
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DailyStats {
    pub realm_id: Uuid,
    pub date: NaiveDate,
    pub unique_logins: i64,
    pub new_accounts: i64,
    pub new_characters: i64,
    pub peak_players: i32,
    pub total_playtime_hours: i64,
    pub monsters_killed: i64,
    pub players_killed: i64,
    pub deaths: i64,
    pub gold_earned: i64,
    pub gold_spent: i64,
    pub items_traded: i64,
    pub market_volume: i64,
    pub quests_completed: i64,
    pub levels_gained: i64,
    pub created_at: DateTime<Utc>,
}

/// Player activity tracking
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PlayerActivity {
    pub character_id: Uuid,
    pub realm_id: Uuid,
    pub date: NaiveDate,
    pub playtime_minutes: i32,
    pub experience_gained: i64,
    pub monsters_killed: i32,
    pub deaths: i32,
    pub gold_earned: i64,
    pub gold_spent: i64,
}

/// Economic metrics
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EconomyMetrics {
    pub realm_id: Uuid,
    pub date: NaiveDate,
    pub total_gold_supply: i64,
    pub total_bank_gold: i64,
    pub avg_player_wealth: i64,
    pub median_player_wealth: i64,
    pub market_offers_active: i32,
    pub market_transactions: i32,
    pub inflation_rate: f64,
}

/// Anti-cheat detection log
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AntiCheatLog {
    pub id: Uuid,
    pub character_id: Uuid,
    pub account_id: Uuid,
    pub realm_id: Uuid,
    pub detection_type: DetectionType,
    pub severity: DetectionSeverity,
    pub details: serde_json::Value,
    pub action_taken: Option<String>,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "detection_type", rename_all = "snake_case")]
pub enum DetectionType {
    SpeedHack,
    Teleport,
    WallHack,
    ItemDupe,
    GoldDupe,
    BotBehavior,
    AutoHealing,
    MacroUsage,
    PacketManipulation,
    ClientModification,
    AbnormalStats,
    ImpossibleAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "detection_severity", rename_all = "lowercase")]
pub enum DetectionSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Matchmaking statistics
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MatchmakingStats {
    pub character_id: Uuid,
    pub match_type: MatchType,
    pub rating: i32,
    pub wins: i32,
    pub losses: i32,
    pub draws: i32,
    pub streak: i32,
    pub best_streak: i32,
    pub total_matches: i32,
    pub season: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "match_type", rename_all = "snake_case")]
pub enum MatchType {
    Duel1v1,
    Team2v2,
    Team3v3,
    Team5v5,
    BattleRoyale,
    CaptureTheFlag,
}

/// Match history
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MatchHistory {
    pub id: Uuid,
    pub realm_id: Uuid,
    pub match_type: MatchType,
    pub team_a: Vec<Uuid>,
    pub team_b: Vec<Uuid>,
    pub winner: Option<i32>, // 0 = draw, 1 = team A, 2 = team B
    pub score_a: i32,
    pub score_b: i32,
    pub duration_seconds: i32,
    pub rated: bool,
    pub replay_data: Option<Vec<u8>>,
    pub started_at: DateTime<Utc>,
    pub ended_at: DateTime<Utc>,
}

/// Leaderboard entry
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LeaderboardEntry {
    pub id: Uuid,
    pub leaderboard_type: LeaderboardType,
    pub realm_id: Option<Uuid>, // None = global
    pub character_id: Uuid,
    pub character_name: String,
    pub score: i64,
    pub rank: i32,
    pub season: Option<i32>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "leaderboard_type", rename_all = "snake_case")]
pub enum LeaderboardType {
    Experience,
    PvpKills,
    BossKills,
    Achievements,
    QuestsCompleted,
    WealthRanking,
    MatchmakingRating,
    TournamentPoints,
    SeasonalPoints,
}
