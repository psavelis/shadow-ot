//! Shared response types for strongly-typed API responses

use serde::Serialize;
use utoipa::ToSchema;

/// Generic message response for simple confirmations
#[derive(Debug, Serialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}

impl MessageResponse {
    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }
}

/// Success response with boolean flag and message
#[derive(Debug, Serialize, ToSchema)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
}

impl SuccessResponse {
    pub fn new(success: bool, message: impl Into<String>) -> Self {
        Self { success, message: message.into() }
    }

    pub fn ok(message: impl Into<String>) -> Self {
        Self::new(true, message)
    }
}

/// Response for delete operations
#[derive(Debug, Serialize, ToSchema)]
pub struct DeletedResponse {
    pub deleted: bool,
    pub message: String,
}

impl DeletedResponse {
    pub fn new(message: impl Into<String>) -> Self {
        Self { deleted: true, message: message.into() }
    }
}

/// Response for 2FA setup with secret and QR code
#[derive(Debug, Serialize, ToSchema)]
pub struct TwoFactorSetupResponse {
    pub secret: String,
    pub qr_code: String,
    pub message: String,
}

/// Response for 2FA verification
#[derive(Debug, Serialize, ToSchema)]
pub struct TwoFactorVerifyResponse {
    pub verified: bool,
    pub backup_codes: Vec<String>,
    pub message: String,
}

/// Response for wallet nonce request
#[derive(Debug, Serialize, ToSchema)]
pub struct WalletNonceResponse {
    pub nonce: String,
    pub message: String,
}

/// Response for wallet connection
#[derive(Debug, Serialize, ToSchema)]
pub struct WalletConnectResponse {
    pub success: bool,
    pub wallet_address: String,
    pub message: String,
}

/// Response for unread count queries
#[derive(Debug, Serialize, ToSchema)]
pub struct UnreadCountResponse {
    pub count: i64,
}

impl UnreadCountResponse {
    pub fn new(count: i64) -> Self {
        Self { count }
    }
}

/// Response for online status queries
#[derive(Debug, Serialize, ToSchema)]
pub struct OnlineStatusResponse {
    pub online: bool,
}

impl OnlineStatusResponse {
    pub fn new(online: bool) -> Self {
        Self { online }
    }
}

/// Response for realm online count
#[derive(Debug, Serialize, ToSchema)]
pub struct RealmOnlineCountResponse {
    pub realm_id: i32,
    pub online_count: i64,
}

/// Response for auto-renew toggle
#[derive(Debug, Serialize, ToSchema)]
pub struct AutoRenewResponse {
    pub success: bool,
    pub auto_renew: bool,
}
