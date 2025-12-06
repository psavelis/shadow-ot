//! Realm information endpoints

use crate::error::ApiError;
use crate::state::AppState;
use crate::ApiResult;
use axum::{extract::{Path, State}, Json};
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;

/// Realm response
#[derive(Debug, Serialize, ToSchema)]
pub struct RealmResponse {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub theme: String,
    pub pvp_type: String,
    pub game_type: String,
    pub region: String,
    pub status: String,
    pub current_players: i32,
    pub max_players: i32,
    pub experience_rate: f64,
    pub skill_rate: f64,
    pub magic_rate: f64,
    pub loot_rate: f64,
    pub premium_only: bool,
    pub transfer_locked: bool,
    pub creation_date: String,
}

/// List all realms
#[utoipa::path(
    get,
    path = "/api/v1/realms",
    responses(
        (status = 200, description = "Realm list", body = Vec<RealmResponse>)
    ),
    tag = "realms"
)]
pub async fn list_realms(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<Vec<RealmResponse>>> {
    let realms = sqlx::query_as::<_, RealmRow>(
        "SELECT * FROM realms ORDER BY name"
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(realms.into_iter().map(Into::into).collect()))
}

/// Get realm by ID
#[utoipa::path(
    get,
    path = "/api/v1/realms/{id}",
    params(
        ("id" = i32, Path, description = "Realm ID")
    ),
    responses(
        (status = 200, description = "Realm information", body = RealmResponse),
        (status = 404, description = "Realm not found")
    ),
    tag = "realms"
)]
pub async fn get_realm(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> ApiResult<Json<RealmResponse>> {
    let realm = sqlx::query_as::<_, RealmRow>(
        "SELECT * FROM realms WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound("Realm not found".to_string()))?;

    Ok(Json(realm.into()))
}

/// Get online count for realm
pub async fn get_online_count(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> ApiResult<Json<serde_json::Value>> {
    let count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM characters WHERE realm_id = $1 AND online = true"
    )
    .bind(id)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(serde_json::json!({
        "realm_id": id,
        "online_count": count
    })))
}

// Helper types

#[derive(sqlx::FromRow)]
struct RealmRow {
    id: i32,
    name: String,
    slug: String,
    description: Option<String>,
    theme: String,
    pvp_type: String,
    game_type: String,
    region: String,
    status: String,
    current_players: i32,
    max_players: i32,
    experience_rate: sqlx::types::BigDecimal,
    skill_rate: sqlx::types::BigDecimal,
    magic_rate: sqlx::types::BigDecimal,
    loot_rate: sqlx::types::BigDecimal,
    premium_only: bool,
    transfer_locked: bool,
    creation_date: chrono::NaiveDate,
}

impl From<RealmRow> for RealmResponse {
    fn from(row: RealmRow) -> Self {
        use std::str::FromStr;
        RealmResponse {
            id: row.id,
            name: row.name,
            slug: row.slug,
            description: row.description,
            theme: row.theme,
            pvp_type: row.pvp_type,
            game_type: row.game_type,
            region: row.region,
            status: row.status,
            current_players: row.current_players,
            max_players: row.max_players,
            experience_rate: f64::from_str(&row.experience_rate.to_string()).unwrap_or(1.0),
            skill_rate: f64::from_str(&row.skill_rate.to_string()).unwrap_or(1.0),
            magic_rate: f64::from_str(&row.magic_rate.to_string()).unwrap_or(1.0),
            loot_rate: f64::from_str(&row.loot_rate.to_string()).unwrap_or(1.0),
            premium_only: row.premium_only,
            transfer_locked: row.transfer_locked,
            creation_date: row.creation_date.to_string(),
        }
    }
}
