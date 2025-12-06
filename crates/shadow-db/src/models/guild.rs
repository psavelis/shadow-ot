//! Guild model - player guilds and wars

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Player guild
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Guild {
    pub id: Uuid,
    pub realm_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub motd: Option<String>,
    pub owner_id: Uuid,
    pub logo_url: Option<String>,
    pub logo_checksum: Option<String>,
    pub creation_date: DateTime<Utc>,
    pub balance: i64,
    pub level: i32,
    pub experience: i64,
    pub member_count: i32,
    pub max_members: i32,
    pub guild_hall_id: Option<i32>,
    pub is_recruiting: bool,
    pub min_level_to_join: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Guild rank
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GuildRank {
    pub id: Uuid,
    pub guild_id: Uuid,
    pub name: String,
    pub level: i32, // 1 = lowest, 3 = leader
    pub permissions: i64, // Bitmask of permissions
    pub order_index: i32,
    pub created_at: DateTime<Utc>,
}

/// Guild membership
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GuildMember {
    pub guild_id: Uuid,
    pub character_id: Uuid,
    pub rank_id: Uuid,
    pub nick: Option<String>,
    pub joined_at: DateTime<Utc>,
}

/// Guild invitation
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GuildInvite {
    pub id: Uuid,
    pub guild_id: Uuid,
    pub character_id: Uuid,
    pub invited_by: Uuid,
    pub message: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// Guild war
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GuildWar {
    pub id: Uuid,
    pub realm_id: Uuid,
    pub guild_a_id: Uuid,
    pub guild_b_id: Uuid,
    pub status: GuildWarStatus,
    pub war_type: GuildWarType,
    pub kills_a: i32,
    pub kills_b: i32,
    pub kill_limit: i32, // 0 = no limit
    pub fee: i64,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
    pub winner_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "guild_war_status", rename_all = "snake_case")]
pub enum GuildWarStatus {
    Pending,
    Active,
    Ended,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "guild_war_type", rename_all = "snake_case")]
pub enum GuildWarType {
    Normal,
    Kills,
    Timed,
}

/// Guild war kill
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GuildWarKill {
    pub id: Uuid,
    pub war_id: Uuid,
    pub killer_id: Uuid,
    pub killer_guild_id: Uuid,
    pub victim_id: Uuid,
    pub victim_guild_id: Uuid,
    pub pos_x: i32,
    pub pos_y: i32,
    pub pos_z: i32,
    pub killed_at: DateTime<Utc>,
}

/// Guild log entry
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GuildLog {
    pub id: Uuid,
    pub guild_id: Uuid,
    pub action: GuildLogAction,
    pub actor_id: Option<Uuid>,
    pub target_id: Option<Uuid>,
    pub details: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "guild_log_action", rename_all = "snake_case")]
pub enum GuildLogAction {
    Created,
    DescriptionChanged,
    MotdChanged,
    MemberJoined,
    MemberLeft,
    MemberKicked,
    MemberPromoted,
    MemberDemoted,
    RankCreated,
    RankDeleted,
    RankModified,
    WarDeclared,
    WarAccepted,
    WarEnded,
    WarSurrender,
    Deposit,
    Withdraw,
    GuildHallBought,
}

/// Guild permissions (bitmask values)
pub mod permissions {
    pub const INVITE: i64 = 1 << 0;
    pub const KICK: i64 = 1 << 1;
    pub const PROMOTE: i64 = 1 << 2;
    pub const DEMOTE: i64 = 1 << 3;
    pub const EDIT_MOTD: i64 = 1 << 4;
    pub const EDIT_DESCRIPTION: i64 = 1 << 5;
    pub const MANAGE_RANKS: i64 = 1 << 6;
    pub const MANAGE_WARS: i64 = 1 << 7;
    pub const WITHDRAW_GOLD: i64 = 1 << 8;
    pub const DEPOSIT_GOLD: i64 = 1 << 9;
    pub const USE_GUILD_HALL: i64 = 1 << 10;
    pub const EDIT_GUILD_HALL: i64 = 1 << 11;
    pub const DISBAND: i64 = 1 << 12;
}
