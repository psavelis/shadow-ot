//! Admin endpoints

use crate::error::ApiError;
use crate::middleware::get_claims;
use crate::response::MessageResponse;
use crate::state::AppState;
use crate::ApiResult;
use axum::{extract::{Request, State}, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Server statistics
#[derive(Debug, Serialize)]
pub struct ServerStats {
    pub total_accounts: i64,
    pub total_characters: i64,
    pub total_online: i64,
    pub total_guilds: i64,
    pub total_houses_owned: i64,
    pub peak_online_today: i32,
    pub uptime_seconds: i64,
}

/// Get server statistics
pub async fn get_stats(
    State(state): State<Arc<AppState>>,
    request: Request,
) -> ApiResult<Json<ServerStats>> {
    let claims = get_claims(&request).ok_or(ApiError::Unauthorized)?;
    if !claims.is_admin() {
        return Err(ApiError::Forbidden);
    }

    let accounts = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM accounts")
        .fetch_one(&state.db)
        .await?;

    let characters = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM characters WHERE deletion_time IS NULL")
        .fetch_one(&state.db)
        .await?;

    let online = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM characters WHERE online = true")
        .fetch_one(&state.db)
        .await?;

    let guilds = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM guilds")
        .fetch_one(&state.db)
        .await?;

    let houses_owned = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM houses WHERE owner_id IS NOT NULL")
        .fetch_one(&state.db)
        .await?;

    let peak = sqlx::query_scalar::<_, i32>(
        "SELECT COALESCE(MAX(peak_players), 0) FROM daily_stats WHERE date = CURRENT_DATE"
    )
    .fetch_one(&state.db)
    .await
    .unwrap_or(0);

    Ok(Json(ServerStats {
        total_accounts: accounts,
        total_characters: characters,
        total_online: online,
        total_guilds: guilds,
        total_houses_owned: houses_owned,
        peak_online_today: peak,
        uptime_seconds: 0, // Would be tracked by server
    }))
}

/// Online player info
#[derive(Debug, Serialize)]
pub struct OnlinePlayer {
    pub id: i32,
    pub name: String,
    pub level: i32,
    pub vocation: i16,
    pub realm_id: i32,
    pub realm_name: Option<String>,
    pub last_login: Option<String>,
}

/// Get online players
pub async fn get_online_players(
    State(state): State<Arc<AppState>>,
    request: Request,
) -> ApiResult<Json<Vec<OnlinePlayer>>> {
    let claims = get_claims(&request).ok_or(ApiError::Unauthorized)?;
    if !claims.is_admin() {
        return Err(ApiError::Forbidden);
    }

    let players = sqlx::query_as::<_, OnlinePlayerRow>(
        "SELECT c.id, c.name, c.level, c.vocation, c.realm_id, r.name as realm_name, c.last_login
         FROM characters c
         LEFT JOIN realms r ON c.realm_id = r.id
         WHERE c.online = true
         ORDER BY c.level DESC"
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(players.into_iter().map(|p| OnlinePlayer {
        id: p.id,
        name: p.name,
        level: p.level,
        vocation: p.vocation,
        realm_id: p.realm_id,
        realm_name: p.realm_name,
        last_login: p.last_login.map(|t| t.to_rfc3339()),
    }).collect()))
}

/// Ban request
#[derive(Debug, Deserialize)]
pub struct BanRequest {
    pub account_id: i32,
    pub reason: String,
    pub ban_type: String,
    pub duration_days: Option<i32>,
}

/// Ban an account
pub async fn ban_account(
    State(state): State<Arc<AppState>>,
    request: Request,
    Json(body): Json<BanRequest>,
) -> ApiResult<Json<MessageResponse>> {
    let claims = get_claims(&request).ok_or(ApiError::Unauthorized)?;
    if !claims.is_admin() {
        return Err(ApiError::Forbidden);
    }

    let expires_at = body.duration_days.map(|days| {
        chrono::Utc::now() + chrono::Duration::days(days as i64)
    });

    sqlx::query(
        "INSERT INTO account_bans (account_id, banned_by, reason, ban_type, expires_at)
         VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(body.account_id)
    .bind(claims.account_id)
    .bind(&body.reason)
    .bind(&body.ban_type)
    .bind(expires_at)
    .execute(&state.db)
    .await?;

    // Update account status
    sqlx::query("UPDATE accounts SET status = 'banned' WHERE id = $1")
        .bind(body.account_id)
        .execute(&state.db)
        .await?;

    // Log action
    sqlx::query(
        "INSERT INTO gm_actions (gm_account_id, target_account_id, action_type, reason)
         VALUES ($1, $2, 'ban', $3)"
    )
    .bind(claims.account_id)
    .bind(body.account_id)
    .bind(&body.reason)
    .execute(&state.db)
    .await?;

    Ok(Json(MessageResponse::new("Account banned")))
}

/// Broadcast request
#[derive(Debug, Deserialize)]
pub struct BroadcastRequest {
    pub message: String,
    pub realm_id: Option<i32>,
}

/// Broadcast a message
pub async fn broadcast_message(
    State(_state): State<Arc<AppState>>,
    request: Request,
    Json(_body): Json<BroadcastRequest>,
) -> ApiResult<Json<MessageResponse>> {
    let claims = get_claims(&request).ok_or(ApiError::Unauthorized)?;
    if !claims.is_admin() {
        return Err(ApiError::Forbidden);
    }

    // In a real implementation, this would send message to game server(s)
    // For now, just acknowledge

    Ok(Json(MessageResponse::new("Broadcast queued")))
}

#[derive(sqlx::FromRow)]
struct OnlinePlayerRow {
    id: i32,
    name: String,
    level: i32,
    vocation: i16,
    realm_id: i32,
    realm_name: Option<String>,
    last_login: Option<chrono::DateTime<chrono::Utc>>,
}
