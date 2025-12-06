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
