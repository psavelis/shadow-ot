//! Forum models - community discussion boards

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Forum category
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ForumCategory {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub slug: String,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub order_index: i32,
    pub parent_id: Option<Uuid>,
    pub realm_id: Option<Uuid>, // None = global, Some = realm-specific
    pub min_level_to_view: Option<i32>,
    pub min_level_to_post: Option<i32>,
    pub staff_only: bool,
    pub locked: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Forum thread
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ForumThread {
    pub id: Uuid,
    pub category_id: Uuid,
    pub author_id: Uuid,
    pub author_character_id: Option<Uuid>,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub views: i64,
    pub reply_count: i32,
    pub last_reply_id: Option<Uuid>,
    pub last_reply_at: Option<DateTime<Utc>>,
    pub last_reply_by: Option<Uuid>,
    pub pinned: bool,
    pub locked: bool,
    pub hidden: bool,
    pub poll_id: Option<Uuid>,
    pub edited_at: Option<DateTime<Utc>>,
    pub edited_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Forum reply
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ForumReply {
    pub id: Uuid,
    pub thread_id: Uuid,
    pub author_id: Uuid,
    pub author_character_id: Option<Uuid>,
    pub content: String,
    pub reply_to_id: Option<Uuid>, // For nested replies
    pub likes: i32,
    pub hidden: bool,
    pub edited_at: Option<DateTime<Utc>>,
    pub edited_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Forum poll
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ForumPoll {
    pub id: Uuid,
    pub thread_id: Uuid,
    pub question: String,
    pub multiple_choice: bool,
    pub max_choices: i32,
    pub show_results_before_vote: bool,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Forum poll option
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ForumPollOption {
    pub id: Uuid,
    pub poll_id: Uuid,
    pub text: String,
    pub votes: i32,
    pub order_index: i32,
}

/// Forum poll vote
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ForumPollVote {
    pub poll_id: Uuid,
    pub option_id: Uuid,
    pub account_id: Uuid,
    pub created_at: DateTime<Utc>,
}

/// Forum thread subscription
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ForumSubscription {
    pub thread_id: Uuid,
    pub account_id: Uuid,
    pub notify_email: bool,
    pub created_at: DateTime<Utc>,
}

/// Forum like
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ForumLike {
    pub reply_id: Uuid,
    pub account_id: Uuid,
    pub created_at: DateTime<Utc>,
}

/// Forum moderation action
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ForumModAction {
    pub id: Uuid,
    pub moderator_id: Uuid,
    pub target_type: ForumTargetType,
    pub target_id: Uuid,
    pub action: ForumAction,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "forum_target_type", rename_all = "lowercase")]
pub enum ForumTargetType {
    Thread,
    Reply,
    Category,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "forum_action", rename_all = "snake_case")]
pub enum ForumAction {
    Pin,
    Unpin,
    Lock,
    Unlock,
    Hide,
    Unhide,
    Delete,
    Move,
    Edit,
    Warn,
}

/// News article
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NewsArticle {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub content: String,
    pub category: NewsCategory,
    pub featured_image: Option<String>,
    pub realm_id: Option<Uuid>, // None = all realms
    pub published: bool,
    pub published_at: Option<DateTime<Utc>>,
    pub views: i64,
    pub allow_comments: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "news_category", rename_all = "lowercase")]
pub enum NewsCategory {
    Update,
    Event,
    Maintenance,
    Community,
    Development,
    Announcement,
}

/// News comment
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NewsComment {
    pub id: Uuid,
    pub article_id: Uuid,
    pub author_id: Uuid,
    pub author_character_id: Option<Uuid>,
    pub content: String,
    pub likes: i32,
    pub hidden: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
