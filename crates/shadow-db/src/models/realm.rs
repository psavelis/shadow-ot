//! Realm model - multi-realm server management

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Realm (server world)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Realm {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub theme: RealmTheme,
    pub status: RealmStatus,

    // Network
    pub host: String,
    pub port: i32,
    pub region: String, // eu-west, us-east, etc.

    // Configuration
    pub protocol_version: i32,
    pub pvp_type: PvpType,
    pub premium_type: PremiumType,
    pub transfer_type: TransferType,

    // Rates
    pub rate_experience: f64,
    pub rate_skill: f64,
    pub rate_loot: f64,
    pub rate_magic: f64,
    pub rate_spawn: f64,

    // Limits
    pub max_players: i32,
    pub current_players: i32,
    pub peak_players: i32,

    // Features
    pub features: serde_json::Value, // Feature flags as JSON

    // Statistics
    pub total_accounts: i64,
    pub total_characters: i64,
    pub total_guilds: i64,
    pub uptime_seconds: i64,
    pub last_save: Option<DateTime<Utc>>,

    // Seasonal
    pub is_seasonal: bool,
    pub season_start: Option<DateTime<Utc>>,
    pub season_end: Option<DateTime<Utc>>,
    pub season_name: Option<String>,

    // Branding
    pub logo_url: Option<String>,
    pub banner_url: Option<String>,
    pub primary_color: String,
    pub secondary_color: String,
    pub tagline: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "realm_theme", rename_all = "lowercase")]
pub enum RealmTheme {
    Mythic,
    Dark,
    Classic,
    War,
    Creative,
    Seasonal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "realm_status", rename_all = "lowercase")]
pub enum RealmStatus {
    Online,
    Offline,
    Maintenance,
    Starting,
    Stopping,
    Locked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "pvp_type", rename_all = "snake_case")]
pub enum PvpType {
    Open,
    Optional,
    Hardcore,
    RetroOpen,
    RetroHardcore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "premium_type", rename_all = "lowercase")]
pub enum PremiumType {
    Free,
    Premium,
    Mixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "transfer_type", rename_all = "lowercase")]
pub enum TransferType {
    Locked,
    Incoming,
    Outgoing,
    Open,
}

/// Realm event
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RealmEvent {
    pub id: Uuid,
    pub realm_id: Uuid,
    pub name: String,
    pub description: String,
    pub event_type: RealmEventType,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub config: serde_json::Value,
    pub active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "realm_event_type", rename_all = "snake_case")]
pub enum RealmEventType {
    DoubleExp,
    DoubleLoot,
    DoubleSkill,
    RapidRespawn,
    WorldBoss,
    InvasionEvent,
    Tournament,
    SeasonalEvent,
    Custom,
}

/// Realm highscores
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RealmHighscore {
    pub realm_id: Uuid,
    pub category: HighscoreCategory,
    pub character_id: Uuid,
    pub character_name: String,
    pub value: i64,
    pub rank: i32,
    pub previous_rank: Option<i32>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "highscore_category", rename_all = "snake_case")]
pub enum HighscoreCategory {
    Level,
    MagicLevel,
    SkillFist,
    SkillClub,
    SkillSword,
    SkillAxe,
    SkillDistance,
    SkillShielding,
    SkillFishing,
    Achievements,
    BossPoints,
    CharmPoints,
    Loyalty,
}

/// Cross-realm link for trading/events
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RealmLink {
    pub id: Uuid,
    pub realm_a_id: Uuid,
    pub realm_b_id: Uuid,
    pub link_type: RealmLinkType,
    pub enabled: bool,
    pub gold_conversion_rate: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "realm_link_type", rename_all = "snake_case")]
pub enum RealmLinkType {
    Trading,
    Events,
    Tournaments,
    Full,
}

/// Realm maintenance schedule
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RealmMaintenance {
    pub id: Uuid,
    pub realm_id: Option<Uuid>, // None = all realms
    pub maintenance_type: MaintenanceType,
    pub scheduled_start: DateTime<Utc>,
    pub scheduled_end: DateTime<Utc>,
    pub actual_start: Option<DateTime<Utc>>,
    pub actual_end: Option<DateTime<Utc>>,
    pub message: String,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "maintenance_type", rename_all = "lowercase")]
pub enum MaintenanceType {
    Scheduled,
    Emergency,
    Update,
    Hotfix,
}
