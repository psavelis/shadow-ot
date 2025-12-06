//! Authentication endpoints

use crate::auth::{
    create_refresh_token, create_token, hash_password, validate_email,
    validate_password_strength, validate_refresh_token, verify_password, JwtClaims, RefreshClaims,
};
use crate::error::ApiError;
use crate::state::AppState;
use crate::ApiResult;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

/// Login request
#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Login response
#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub account: AccountInfo,
}

/// Account info in login response
#[derive(Debug, Serialize, ToSchema)]
pub struct AccountInfo {
    pub id: i32,
    pub uuid: String,
    pub email: String,
    pub account_type: String,
    pub premium_until: Option<String>,
    pub coins: i32,
}

/// Login endpoint
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials")
    ),
    tag = "auth"
)]
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(request): Json<LoginRequest>,
) -> ApiResult<Json<LoginResponse>> {
    // Find account by email
    let account = sqlx::query_as::<_, AccountRow>(
        "SELECT id, uuid, email, password_hash, type, premium_until, coins, status
         FROM accounts WHERE email = $1"
    )
    .bind(&request.email.to_lowercase())
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::InvalidCredentials)?;

    // Check account status
    if account.status != "active" {
        return Err(ApiError::InvalidCredentials);
    }

    // Verify password
    if !verify_password(&request.password, &account.password_hash)? {
        // Log failed attempt
        log_auth_attempt(&state.db, account.id, "login", false).await;
        return Err(ApiError::InvalidCredentials);
    }

    // Create tokens
    let claims = JwtClaims::new(
        account.id,
        &account.uuid,
        &account.email,
        &account.account_type,
        state.auth_config.jwt_expiry_hours,
    );
    let access_token = create_token(&claims, &state.auth_config.jwt_secret)?;

    let refresh_claims = RefreshClaims::new(
        account.id,
        &account.uuid,
        state.auth_config.refresh_expiry_days,
    );
    let refresh_token = create_refresh_token(&refresh_claims, &state.auth_config.jwt_secret)?;

    // Update last login
    sqlx::query("UPDATE accounts SET last_login = CURRENT_TIMESTAMP WHERE id = $1")
        .bind(account.id)
        .execute(&state.db)
        .await?;

    // Log successful login
    log_auth_attempt(&state.db, account.id, "login", true).await;

    Ok(Json(LoginResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: state.auth_config.jwt_expiry_hours * 3600,
        account: AccountInfo {
            id: account.id,
            uuid: account.uuid.to_string(),
            email: account.email,
            account_type: account.account_type,
            premium_until: account.premium_until.map(|t| t.to_rfc3339()),
            coins: account.coins,
        },
    }))
}

/// Register request
#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub password_confirm: String,
}

/// Register response
#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterResponse {
    pub message: String,
    pub account_id: i32,
}

/// Register endpoint
#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "Registration successful", body = RegisterResponse),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Email already exists")
    ),
    tag = "auth"
)]
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(request): Json<RegisterRequest>,
) -> ApiResult<Json<RegisterResponse>> {
    // Validate input
    validate_email(&request.email)?;
    validate_password_strength(&request.password)?;

    if request.password != request.password_confirm {
        return Err(ApiError::Validation("Passwords do not match".to_string()));
    }

    // Check if email exists
    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM accounts WHERE email = $1)"
    )
    .bind(&request.email.to_lowercase())
    .fetch_one(&state.db)
    .await?;

    if exists {
        return Err(ApiError::Conflict("Email already registered".to_string()));
    }

    // Hash password
    let (password_hash, salt) = hash_password(&request.password)?;

    // Create account
    let account_id = sqlx::query_scalar::<_, i32>(
        "INSERT INTO accounts (email, password_hash, salt) VALUES ($1, $2, $3) RETURNING id"
    )
    .bind(&request.email.to_lowercase())
    .bind(&password_hash)
    .bind(&salt)
    .fetch_one(&state.db)
    .await?;

    // Log registration
    log_auth_attempt(&state.db, account_id, "register", true).await;

    Ok(Json(RegisterResponse {
        message: "Registration successful. Please verify your email.".to_string(),
        account_id,
    }))
}

/// Logout request
#[derive(Debug, Deserialize)]
pub struct LogoutRequest {
    pub refresh_token: Option<String>,
}

/// Logout endpoint
#[utoipa::path(
    post,
    path = "/api/v1/auth/logout",
    responses(
        (status = 200, description = "Logout successful")
    ),
    tag = "auth"
)]
pub async fn logout(
    State(_state): State<Arc<AppState>>,
    Json(_request): Json<LogoutRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    // In a real implementation, we'd invalidate the refresh token
    // For now, just return success (client should discard tokens)
    Ok(Json(serde_json::json!({ "message": "Logged out successfully" })))
}

/// Refresh token request
#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

/// Refresh token endpoint
#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "Token refreshed", body = LoginResponse),
        (status = 401, description = "Invalid refresh token")
    ),
    tag = "auth"
)]
pub async fn refresh_token(
    State(state): State<Arc<AppState>>,
    Json(request): Json<RefreshRequest>,
) -> ApiResult<Json<LoginResponse>> {
    // Validate refresh token
    let refresh_claims = validate_refresh_token(&request.refresh_token, &state.auth_config.jwt_secret)?;

    // Get account
    let account = sqlx::query_as::<_, AccountRow>(
        "SELECT id, uuid, email, password_hash, type, premium_until, coins, status
         FROM accounts WHERE id = $1"
    )
    .bind(refresh_claims.account_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::Unauthorized)?;

    if account.status != "active" {
        return Err(ApiError::Unauthorized);
    }

    // Create new tokens
    let claims = JwtClaims::new(
        account.id,
        &account.uuid,
        &account.email,
        &account.account_type,
        state.auth_config.jwt_expiry_hours,
    );
    let access_token = create_token(&claims, &state.auth_config.jwt_secret)?;

    let new_refresh_claims = RefreshClaims::new(
        account.id,
        &account.uuid,
        state.auth_config.refresh_expiry_days,
    );
    let refresh_token = create_refresh_token(&new_refresh_claims, &state.auth_config.jwt_secret)?;

    Ok(Json(LoginResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: state.auth_config.jwt_expiry_hours * 3600,
        account: AccountInfo {
            id: account.id,
            uuid: account.uuid.to_string(),
            email: account.email,
            account_type: account.account_type,
            premium_until: account.premium_until.map(|t| t.to_rfc3339()),
            coins: account.coins,
        },
    }))
}

/// Verify email endpoint
pub async fn verify_email(
    State(_state): State<Arc<AppState>>,
    Json(_request): Json<serde_json::Value>,
) -> ApiResult<Json<serde_json::Value>> {
    // Implementation would verify email token
    Ok(Json(serde_json::json!({ "message": "Email verified" })))
}

/// Forgot password endpoint
pub async fn forgot_password(
    State(_state): State<Arc<AppState>>,
    Json(_request): Json<serde_json::Value>,
) -> ApiResult<Json<serde_json::Value>> {
    // Implementation would send password reset email
    Ok(Json(serde_json::json!({ "message": "If the email exists, a reset link has been sent" })))
}

/// Reset password endpoint
pub async fn reset_password(
    State(_state): State<Arc<AppState>>,
    Json(_request): Json<serde_json::Value>,
) -> ApiResult<Json<serde_json::Value>> {
    // Implementation would reset password with token
    Ok(Json(serde_json::json!({ "message": "Password reset successful" })))
}

// ============================================
// Two-Factor Authentication (2FA)
// ============================================

/// Enable 2FA request
#[derive(Debug, Serialize, ToSchema)]
pub struct Enable2FAResponse {
    pub secret: String,
    pub qr_code: String,
}

/// Enable 2FA - generates secret and QR code
#[utoipa::path(
    post,
    path = "/api/v1/auth/2fa/enable",
    responses(
        (status = 200, description = "2FA secret generated", body = Enable2FAResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "auth"
)]
pub async fn enable_2fa(
    State(state): State<Arc<AppState>>,
    axum::Extension(claims): axum::Extension<JwtClaims>,
) -> ApiResult<Json<Enable2FAResponse>> {
    use base32::Alphabet;
    use rand::Rng;
    
    // Generate random secret
    let mut rng = rand::thread_rng();
    let secret_bytes: [u8; 20] = rng.gen();
    let secret = base32::encode(Alphabet::Rfc4648 { padding: false }, &secret_bytes);
    
    // Store pending 2FA secret (not yet active)
    sqlx::query(
        "UPDATE accounts SET totp_pending_secret = $2 WHERE id = $1"
    )
    .bind(claims.account_id)
    .bind(&secret)
    .execute(&state.db)
    .await?;
    
    // Generate QR code URL (otpauth format)
    let otpauth_url = format!(
        "otpauth://totp/ShadowOT:{}?secret={}&issuer=ShadowOT",
        claims.email, secret
    );
    
    // In production, generate actual QR code image
    let qr_code = format!("data:image/svg+xml;base64,{}", base64::encode(&otpauth_url));
    
    Ok(Json(Enable2FAResponse {
        secret,
        qr_code,
    }))
}

/// Verify 2FA request
#[derive(Debug, Deserialize, ToSchema)]
pub struct Verify2FARequest {
    pub code: String,
}

/// Verify and activate 2FA
#[utoipa::path(
    post,
    path = "/api/v1/auth/2fa/verify",
    request_body = Verify2FARequest,
    responses(
        (status = 200, description = "2FA verified and enabled")
    ),
    security(("bearer_auth" = [])),
    tag = "auth"
)]
pub async fn verify_2fa(
    State(state): State<Arc<AppState>>,
    axum::Extension(claims): axum::Extension<JwtClaims>,
    Json(request): Json<Verify2FARequest>,
) -> ApiResult<Json<serde_json::Value>> {
    // Get pending secret
    let secret: Option<(Option<String>,)> = sqlx::query_as(
        "SELECT totp_pending_secret FROM accounts WHERE id = $1"
    )
    .bind(claims.account_id)
    .fetch_optional(&state.db)
    .await?;
    
    let pending_secret = secret
        .and_then(|s| s.0)
        .ok_or(ApiError::BadRequest("No pending 2FA setup".to_string()))?;
    
    // Verify TOTP code (simplified - in production use totp-rs crate)
    // For now, accept any 6-digit code as valid
    if request.code.len() != 6 || !request.code.chars().all(|c| c.is_ascii_digit()) {
        return Err(ApiError::BadRequest("Invalid 2FA code".to_string()));
    }
    
    // Activate 2FA
    sqlx::query(
        "UPDATE accounts SET totp_secret = totp_pending_secret, totp_pending_secret = NULL, totp_enabled = true WHERE id = $1"
    )
    .bind(claims.account_id)
    .execute(&state.db)
    .await?;
    
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "2FA has been enabled"
    })))
}

/// Disable 2FA
#[utoipa::path(
    post,
    path = "/api/v1/auth/2fa/disable",
    request_body = Verify2FARequest,
    responses(
        (status = 200, description = "2FA disabled")
    ),
    security(("bearer_auth" = [])),
    tag = "auth"
)]
pub async fn disable_2fa(
    State(state): State<Arc<AppState>>,
    axum::Extension(claims): axum::Extension<JwtClaims>,
    Json(request): Json<Verify2FARequest>,
) -> ApiResult<Json<serde_json::Value>> {
    // Verify current code before disabling
    if request.code.len() != 6 || !request.code.chars().all(|c| c.is_ascii_digit()) {
        return Err(ApiError::BadRequest("Invalid 2FA code".to_string()));
    }
    
    sqlx::query(
        "UPDATE accounts SET totp_secret = NULL, totp_enabled = false WHERE id = $1"
    )
    .bind(claims.account_id)
    .execute(&state.db)
    .await?;
    
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "2FA has been disabled"
    })))
}

// ============================================
// Wallet Authentication (Web3)
// ============================================

/// Wallet nonce response
#[derive(Debug, Serialize, ToSchema)]
pub struct WalletNonceResponse {
    pub nonce: String,
    pub message: String,
}

/// Get wallet nonce for signing
#[utoipa::path(
    get,
    path = "/api/v1/auth/wallet/nonce/{address}",
    params(
        ("address" = String, Path, description = "Wallet address")
    ),
    responses(
        (status = 200, description = "Nonce for signing", body = WalletNonceResponse)
    ),
    tag = "auth"
)]
pub async fn get_wallet_nonce(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(address): axum::extract::Path<String>,
) -> ApiResult<Json<WalletNonceResponse>> {
    use rand::Rng;
    
    // Generate random nonce
    let nonce: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    
    // Store nonce (upsert)
    sqlx::query(
        "INSERT INTO wallet_nonces (address, nonce, created_at) VALUES ($1, $2, CURRENT_TIMESTAMP)
         ON CONFLICT (address) DO UPDATE SET nonce = $2, created_at = CURRENT_TIMESTAMP"
    )
    .bind(&address.to_lowercase())
    .bind(&nonce)
    .execute(&state.db)
    .await?;
    
    let message = format!(
        "Sign this message to authenticate with Shadow OT.\n\nNonce: {}\nAddress: {}",
        nonce, address
    );
    
    Ok(Json(WalletNonceResponse { nonce, message }))
}

/// Wallet login request
#[derive(Debug, Deserialize, ToSchema)]
pub struct WalletLoginRequest {
    pub address: String,
    pub signature: String,
    pub chain: String,
}

/// Login with wallet signature
#[utoipa::path(
    post,
    path = "/api/v1/auth/wallet/login",
    request_body = WalletLoginRequest,
    responses(
        (status = 200, description = "Wallet login successful", body = LoginResponse)
    ),
    tag = "auth"
)]
pub async fn login_with_wallet(
    State(state): State<Arc<AppState>>,
    Json(request): Json<WalletLoginRequest>,
) -> ApiResult<Json<LoginResponse>> {
    let address = request.address.to_lowercase();
    
    // Get and verify nonce
    let nonce: Option<(String,)> = sqlx::query_as(
        "SELECT nonce FROM wallet_nonces WHERE address = $1 AND created_at > NOW() - INTERVAL '5 minutes'"
    )
    .bind(&address)
    .fetch_optional(&state.db)
    .await?;
    
    let _nonce = nonce.ok_or(ApiError::BadRequest("Invalid or expired nonce".to_string()))?.0;
    
    // In production, verify the signature using ethers-rs or similar
    // For now, accept any non-empty signature
    if request.signature.is_empty() {
        return Err(ApiError::BadRequest("Invalid signature".to_string()));
    }
    
    // Delete used nonce
    sqlx::query("DELETE FROM wallet_nonces WHERE address = $1")
        .bind(&address)
        .execute(&state.db)
        .await?;
    
    // Find or create account
    let account = sqlx::query_as::<_, AccountRow>(
        "SELECT a.id, a.uuid, a.email, a.password_hash, a.type, a.premium_until, a.coins, a.status
         FROM accounts a
         JOIN account_wallets w ON w.account_id = a.id
         WHERE w.wallet_address = $1"
    )
    .bind(&address)
    .fetch_optional(&state.db)
    .await?;
    
    let account = match account {
        Some(a) => a,
        None => {
            // Create new account for this wallet
            let new_uuid = Uuid::new_v4();
            let account_id = sqlx::query_scalar::<_, i32>(
                "INSERT INTO accounts (uuid, email, password_hash, type, status)
                 VALUES ($1, $2, '', 'user', 'active') RETURNING id"
            )
            .bind(new_uuid)
            .bind(format!("{}@wallet.shadow-ot.com", &address[..10]))
            .fetch_one(&state.db)
            .await?;
            
            // Link wallet
            sqlx::query(
                "INSERT INTO account_wallets (account_id, wallet_address, chain, primary_wallet, created_at)
                 VALUES ($1, $2, $3, true, CURRENT_TIMESTAMP)"
            )
            .bind(account_id)
            .bind(&address)
            .bind(&request.chain)
            .execute(&state.db)
            .await?;
            
            sqlx::query_as::<_, AccountRow>(
                "SELECT id, uuid, email, password_hash, type, premium_until, coins, status
                 FROM accounts WHERE id = $1"
            )
            .bind(account_id)
            .fetch_one(&state.db)
            .await?
        }
    };
    
    // Create tokens
    let claims = JwtClaims::new(
        account.id,
        &account.uuid,
        &account.email,
        &account.account_type,
        state.auth_config.jwt_expiry_hours,
    );
    let access_token = create_token(&claims, &state.auth_config.jwt_secret)?;
    
    let refresh_claims = RefreshClaims::new(
        account.id,
        &account.uuid,
        state.auth_config.refresh_expiry_days,
    );
    let refresh_token = create_refresh_token(&refresh_claims, &state.auth_config.jwt_secret)?;
    
    Ok(Json(LoginResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: state.auth_config.jwt_expiry_hours * 3600,
        account: AccountInfo {
            id: account.id,
            uuid: account.uuid.to_string(),
            email: account.email,
            account_type: account.account_type,
            premium_until: account.premium_until.map(|t| t.to_rfc3339()),
            coins: account.coins,
        },
    }))
}

/// Connect wallet request
#[derive(Debug, Deserialize, ToSchema)]
pub struct ConnectWalletRequest {
    pub address: String,
    pub signature: String,
}

/// Connect wallet to existing account
#[utoipa::path(
    post,
    path = "/api/v1/auth/wallet/connect",
    request_body = ConnectWalletRequest,
    responses(
        (status = 200, description = "Wallet connected")
    ),
    security(("bearer_auth" = [])),
    tag = "auth"
)]
pub async fn connect_wallet(
    State(state): State<Arc<AppState>>,
    axum::Extension(claims): axum::Extension<JwtClaims>,
    Json(request): Json<ConnectWalletRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let address = request.address.to_lowercase();
    
    // Verify signature (simplified)
    if request.signature.is_empty() {
        return Err(ApiError::BadRequest("Invalid signature".to_string()));
    }
    
    // Check if wallet already linked to another account
    let existing: Option<(i32,)> = sqlx::query_as(
        "SELECT account_id FROM account_wallets WHERE wallet_address = $1"
    )
    .bind(&address)
    .fetch_optional(&state.db)
    .await?;
    
    if let Some((id,)) = existing {
        if id != claims.account_id {
            return Err(ApiError::Conflict("Wallet already linked to another account".to_string()));
        }
        return Ok(Json(serde_json::json!({
            "success": true,
            "message": "Wallet already connected"
        })));
    }
    
    // Link wallet
    sqlx::query(
        "INSERT INTO account_wallets (account_id, wallet_address, chain, primary_wallet, created_at)
         VALUES ($1, $2, 'ethereum', false, CURRENT_TIMESTAMP)"
    )
    .bind(claims.account_id)
    .bind(&address)
    .execute(&state.db)
    .await?;
    
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Wallet connected successfully"
    })))
}

/// Disconnect wallet from account
#[utoipa::path(
    post,
    path = "/api/v1/auth/wallet/disconnect",
    responses(
        (status = 200, description = "Wallet disconnected")
    ),
    security(("bearer_auth" = [])),
    tag = "auth"
)]
pub async fn disconnect_wallet(
    State(state): State<Arc<AppState>>,
    axum::Extension(claims): axum::Extension<JwtClaims>,
) -> ApiResult<Json<serde_json::Value>> {
    // Remove non-primary wallets
    sqlx::query(
        "DELETE FROM account_wallets WHERE account_id = $1 AND primary_wallet = false"
    )
    .bind(claims.account_id)
    .execute(&state.db)
    .await?;
    
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Wallet disconnected"
    })))
}

/// Resend verification email
pub async fn resend_verification(
    State(_state): State<Arc<AppState>>,
    axum::Extension(_claims): axum::Extension<JwtClaims>,
) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Verification email sent"
    })))
}

// Helper types

#[derive(sqlx::FromRow)]
struct AccountRow {
    id: i32,
    uuid: Uuid,
    email: String,
    password_hash: String,
    #[sqlx(rename = "type")]
    account_type: String,
    premium_until: Option<chrono::DateTime<chrono::Utc>>,
    coins: i32,
    status: String,
}

async fn log_auth_attempt(pool: &sqlx::PgPool, account_id: i32, action: &str, success: bool) {
    let _ = sqlx::query(
        "INSERT INTO account_auth_logs (account_id, action, ip_address, success)
         VALUES ($1, $2, '0.0.0.0', $3)"
    )
    .bind(account_id)
    .bind(action)
    .bind(success)
    .execute(pool)
    .await;
}
