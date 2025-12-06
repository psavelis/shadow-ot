//! Account model - user accounts spanning all realms

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// User account - one account can have characters on multiple realms
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Account {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub account_type: AccountType,
    pub premium_until: Option<DateTime<Utc>>,
    pub premium_days_purchased: i32,
    pub coins: i64,
    pub tournament_coins: i64,
    pub email_verified: bool,
    pub two_factor_enabled: bool,
    #[serde(skip_serializing)]
    pub two_factor_secret: Option<String>,
    pub last_login: Option<DateTime<Utc>>,
    pub last_ip: Option<String>,
    pub login_attempts: i32,
    pub locked_until: Option<DateTime<Utc>>,
    pub ban_until: Option<DateTime<Utc>>,
    pub ban_reason: Option<String>,
    pub ban_by: Option<Uuid>,
    pub wallet_address: Option<String>,
    pub wallet_chain: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "account_type", rename_all = "lowercase")]
pub enum AccountType {
    Player,
    Tutor,
    SeniorTutor,
    Gamemaster,
    CommunityManager,
    God,
    Admin,
}

impl Default for AccountType {
    fn default() -> Self {
        Self::Player
    }
}

/// Account session for tracking active logins
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AccountSession {
    pub id: Uuid,
    pub account_id: Uuid,
    pub session_token: String,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// Account authentication log
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AccountAuthLog {
    pub id: Uuid,
    pub account_id: Uuid,
    pub action: AuthAction,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub success: bool,
    pub failure_reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "auth_action", rename_all = "snake_case")]
pub enum AuthAction {
    Login,
    Logout,
    PasswordChange,
    PasswordReset,
    TwoFactorEnable,
    TwoFactorDisable,
    EmailChange,
    WalletConnect,
    WalletDisconnect,
}

/// Create account request
#[derive(Debug, Clone, Deserialize)]
pub struct CreateAccountRequest {
    pub email: String,
    pub username: String,
    pub password: String,
    pub wallet_address: Option<String>,
}

/// Login request
#[derive(Debug, Clone, Deserialize)]
pub struct LoginRequest {
    pub email_or_username: String,
    pub password: String,
    pub two_factor_code: Option<String>,
}
