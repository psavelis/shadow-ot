//! Social models - friends, VIP, chat channels

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Friend/VIP list entry
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Friendship {
    pub id: Uuid,
    pub account_id: Uuid,
    pub friend_account_id: Uuid,
    pub friend_name: String,
    pub description: Option<String>,
    pub icon: i32,
    pub notify: bool,
    pub created_at: DateTime<Utc>,
}

/// Chat channel
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ChatChannel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub channel_type: ChannelType,
    pub owner_id: Option<Uuid>, // For private channels
    pub guild_id: Option<Uuid>, // For guild channels
    pub party_id: Option<Uuid>, // For party channels
    pub password: Option<String>,
    pub min_level: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "channel_type", rename_all = "lowercase")]
pub enum ChannelType {
    Public,
    Private,
    Guild,
    Party,
    Trade,
    Help,
    Gamemaster,
    Tutor,
    RuleViolation,
}

/// Chat channel membership
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ChannelMember {
    pub channel_id: i32,
    pub character_id: Uuid,
    pub invited: bool,
    pub muted_until: Option<DateTime<Utc>>,
    pub joined_at: DateTime<Utc>,
}

/// Private message
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PrivateMessage {
    pub id: Uuid,
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
    pub content: String,
    pub read: bool,
    pub deleted_by_sender: bool,
    pub deleted_by_receiver: bool,
    pub created_at: DateTime<Utc>,
}

/// Party
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Party {
    pub id: Uuid,
    pub realm_id: Uuid,
    pub leader_id: Uuid,
    pub share_experience: bool,
    pub created_at: DateTime<Utc>,
}

/// Party member
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PartyMember {
    pub party_id: Uuid,
    pub character_id: Uuid,
    pub joined_at: DateTime<Utc>,
}

/// Party invitation
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PartyInvite {
    pub id: Uuid,
    pub party_id: Uuid,
    pub inviter_id: Uuid,
    pub invitee_id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// Report/ticket
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Report {
    pub id: Uuid,
    pub realm_id: Uuid,
    pub reporter_id: Uuid,
    pub reported_id: Option<Uuid>,
    pub report_type: ReportType,
    pub category: ReportCategory,
    pub description: String,
    pub status: ReportStatus,
    pub assigned_to: Option<Uuid>,
    pub resolution: Option<String>,
    pub pos_x: Option<i32>,
    pub pos_y: Option<i32>,
    pub pos_z: Option<i32>,
    pub evidence: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "report_type", rename_all = "snake_case")]
pub enum ReportType {
    BugReport,
    PlayerReport,
    Suggestion,
    Support,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "report_category", rename_all = "snake_case")]
pub enum ReportCategory {
    Harassment,
    Cheating,
    Botting,
    RealMoneyTrading,
    NameViolation,
    Scamming,
    GameBug,
    WebsiteBug,
    Exploit,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "report_status", rename_all = "lowercase")]
pub enum ReportStatus {
    Open,
    Investigating,
    Resolved,
    Closed,
    Rejected,
}
