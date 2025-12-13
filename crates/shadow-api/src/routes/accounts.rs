//! Account management endpoints

use crate::error::ApiError;
use crate::middleware::get_claims;
use crate::response::MessageResponse;
use crate::state::AppState;
use crate::ApiResult;
use axum::{extract::{Request, State}, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;

/// Account response
#[derive(Debug, Serialize, ToSchema)]
pub struct AccountResponse {
    pub id: i32,
    pub uuid: String,
    pub email: String,
    pub account_type: String,
    pub premium_until: Option<String>,
    pub coins: i32,
    pub tournament_coins: i32,
    pub email_verified: bool,
    pub two_factor_enabled: bool,
    pub created_at: String,
    pub last_login: Option<String>,
}

/// Get current account
#[utoipa::path(
    get,
    path = "/api/v1/account",
    responses(
        (status = 200, description = "Account information", body = AccountResponse),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "accounts"
)]
pub async fn get_account(
    State(state): State<Arc<AppState>>,
    request: Request,
) -> ApiResult<Json<AccountResponse>> {
    let claims = get_claims(&request).ok_or(ApiError::Unauthorized)?;

    let account = sqlx::query_as::<_, AccountRow>(
        "SELECT id, uuid, email, type, premium_until, coins, tournament_coins,
                email_verified, two_factor_enabled, created_at, last_login
         FROM accounts WHERE id = $1"
    )
    .bind(claims.account_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound("Account not found".to_string()))?;

    Ok(Json(AccountResponse {
        id: account.id,
        uuid: account.uuid.to_string(),
        email: account.email,
        account_type: account.account_type,
        premium_until: account.premium_until.map(|t| t.to_rfc3339()),
        coins: account.coins,
        tournament_coins: account.tournament_coins,
        email_verified: account.email_verified,
        two_factor_enabled: account.two_factor_enabled,
        created_at: account.created_at.to_rfc3339(),
        last_login: account.last_login.map(|t| t.to_rfc3339()),
    }))
}

/// Update account request
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateAccountRequest {
    pub email: Option<String>,
}

/// Update account
#[utoipa::path(
    put,
    path = "/api/v1/account",
    request_body = UpdateAccountRequest,
    responses(
        (status = 200, description = "Account updated", body = AccountResponse),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "accounts"
)]
pub async fn update_account(
    State(state): State<Arc<AppState>>,
    request: Request,
    Json(body): Json<UpdateAccountRequest>,
) -> ApiResult<Json<AccountResponse>> {
    let claims = get_claims(&request).ok_or(ApiError::Unauthorized)?;

    if let Some(email) = &body.email {
        crate::auth::validate_email(email)?;

        // Check if email is taken
        let exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM accounts WHERE email = $1 AND id != $2)"
        )
        .bind(email.to_lowercase())
        .bind(claims.account_id)
        .fetch_one(&state.db)
        .await?;

        if exists {
            return Err(ApiError::Conflict("Email already in use".to_string()));
        }

        sqlx::query("UPDATE accounts SET email = $1, email_verified = false WHERE id = $2")
            .bind(email.to_lowercase())
            .bind(claims.account_id)
            .execute(&state.db)
            .await?;
    }

    // Return updated account
    get_account(State(state), request).await
}

/// Change password request
#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
    pub new_password_confirm: String,
}

/// Change password
pub async fn change_password(
    State(state): State<Arc<AppState>>,
    request: Request,
    Json(body): Json<ChangePasswordRequest>,
) -> ApiResult<Json<MessageResponse>> {
    let claims = get_claims(&request).ok_or(ApiError::Unauthorized)?;

    // Validate new password
    crate::auth::validate_password_strength(&body.new_password)?;

    if body.new_password != body.new_password_confirm {
        return Err(ApiError::Validation("Passwords do not match".to_string()));
    }

    // Get current password hash
    let hash = sqlx::query_scalar::<_, String>(
        "SELECT password_hash FROM accounts WHERE id = $1"
    )
    .bind(claims.account_id)
    .fetch_one(&state.db)
    .await?;

    // Verify current password
    if !crate::auth::verify_password(&body.current_password, &hash)? {
        return Err(ApiError::InvalidCredentials);
    }

    // Hash new password
    let (new_hash, new_salt) = crate::auth::hash_password(&body.new_password)?;

    // Update password
    sqlx::query("UPDATE accounts SET password_hash = $1, salt = $2 WHERE id = $3")
        .bind(&new_hash)
        .bind(&new_salt)
        .bind(claims.account_id)
        .execute(&state.db)
        .await?;

    Ok(Json(MessageResponse::new("Password changed successfully")))
}

/// Session info
#[derive(Debug, Serialize)]
pub struct SessionInfo {
    pub id: i32,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub created_at: String,
    pub last_activity: String,
}

/// List active sessions
pub async fn list_sessions(
    State(state): State<Arc<AppState>>,
    request: Request,
) -> ApiResult<Json<Vec<SessionInfo>>> {
    let claims = get_claims(&request).ok_or(ApiError::Unauthorized)?;

    let sessions = sqlx::query_as::<_, SessionRow>(
        "SELECT id, ip_address, user_agent, created_at, last_activity
         FROM account_sessions
         WHERE account_id = $1 AND revoked = false AND expires_at > CURRENT_TIMESTAMP
         ORDER BY last_activity DESC"
    )
    .bind(claims.account_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(sessions.into_iter().map(|s| SessionInfo {
        id: s.id,
        ip_address: s.ip_address.to_string(),
        user_agent: s.user_agent,
        created_at: s.created_at.to_rfc3339(),
        last_activity: s.last_activity.to_rfc3339(),
    }).collect()))
}

/// Revoke a session
pub async fn revoke_session(
    State(state): State<Arc<AppState>>,
    request: Request,
    axum::extract::Path(session_id): axum::extract::Path<i32>,
) -> ApiResult<Json<MessageResponse>> {
    let claims = get_claims(&request).ok_or(ApiError::Unauthorized)?;

    let result = sqlx::query(
        "UPDATE account_sessions SET revoked = true WHERE id = $1 AND account_id = $2"
    )
    .bind(session_id)
    .bind(claims.account_id)
    .execute(&state.db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound("Session not found".to_string()));
    }

    Ok(Json(MessageResponse::new("Session revoked")))
}

// Helper types

#[derive(sqlx::FromRow)]
struct AccountRow {
    id: i32,
    uuid: uuid::Uuid,
    email: String,
    #[sqlx(rename = "type")]
    account_type: String,
    premium_until: Option<chrono::DateTime<chrono::Utc>>,
    coins: i32,
    tournament_coins: i32,
    email_verified: bool,
    two_factor_enabled: bool,
    created_at: chrono::DateTime<chrono::Utc>,
    last_login: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(sqlx::FromRow)]
struct SessionRow {
    id: i32,
    ip_address: std::net::IpAddr,
    user_agent: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
    last_activity: chrono::DateTime<chrono::Utc>,
}
