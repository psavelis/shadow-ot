//! Game event endpoints

use crate::state::AppState;
use crate::ApiResult;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

/// Event status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "event_status", rename_all = "lowercase")]
pub enum EventStatus {
    Upcoming,
    Active,
    Ended,
    Cancelled,
}

/// Event type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "event_type", rename_all = "lowercase")]
pub enum EventType {
    WorldBoss,
    DoubleExp,
    DoubleSkill,
    DoubleLoot,
    Raid,
    Competition,
    Seasonal,
    Special,
}

/// Game event information
#[derive(Debug, Serialize, ToSchema)]
pub struct GameEvent {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub event_type: EventType,
    pub status: EventStatus,
    pub realm: Option<String>,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    pub rewards: Vec<EventReward>,
    pub location: Option<EventLocation>,
    pub requirements: Option<EventRequirements>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
struct EventRow {
    id: Uuid,
    name: String,
    description: String,
    event_type: EventType,
    status: EventStatus,
    realm: Option<String>,
    starts_at: DateTime<Utc>,
    ends_at: DateTime<Utc>,
    location_x: Option<i32>,
    location_y: Option<i32>,
    location_z: Option<i32>,
    location_name: Option<String>,
    min_level: Option<i32>,
    max_players: Option<i32>,
    created_at: DateTime<Utc>,
}

/// Event reward
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EventReward {
    pub reward_type: String,
    pub item_id: Option<i32>,
    pub item_name: Option<String>,
    pub amount: i32,
    pub description: String,
    pub tier: Option<String>,
}

#[derive(Debug, FromRow)]
struct RewardRow {
    reward_type: String,
    item_id: Option<i32>,
    item_name: Option<String>,
    amount: i32,
    description: String,
    tier: Option<String>,
}

/// Event location
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EventLocation {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub name: Option<String>,
}

/// Event requirements
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EventRequirements {
    pub min_level: Option<i32>,
    pub max_players: Option<i32>,
}

/// Query parameters
#[derive(Debug, Deserialize)]
pub struct EventQuery {
    pub status: Option<String>,
    pub event_type: Option<String>,
    pub realm: Option<String>,
}

/// List all events
#[utoipa::path(
    get,
    path = "/api/v1/events",
    params(
        ("status" = Option<String>, Query, description = "Filter by status"),
        ("event_type" = Option<String>, Query, description = "Filter by type"),
        ("realm" = Option<String>, Query, description = "Filter by realm")
    ),
    responses(
        (status = 200, description = "Events list", body = Vec<GameEvent>)
    ),
    tag = "events"
)]
pub async fn list_events(
    State(state): State<Arc<AppState>>,
    Query(query): Query<EventQuery>,
) -> ApiResult<Json<Vec<GameEvent>>> {
    let rows = sqlx::query_as::<_, EventRow>(
        "SELECT id, name, description, event_type, status, realm, starts_at, ends_at,
                location_x, location_y, location_z, location_name, min_level, max_players, created_at
         FROM game_events
         WHERE ($1::text IS NULL OR status::text = $1)
           AND ($2::text IS NULL OR event_type::text = $2)
           AND ($3::text IS NULL OR realm = $3 OR realm IS NULL)
         ORDER BY starts_at DESC"
    )
    .bind(&query.status)
    .bind(&query.event_type)
    .bind(&query.realm)
    .fetch_all(&state.db)
    .await?;

    let mut events = Vec::new();
    for row in rows {
        let rewards = load_event_rewards(&state, row.id).await?;
        events.push(build_event(row, rewards));
    }

    Ok(Json(events))
}

/// Get active events
#[utoipa::path(
    get,
    path = "/api/v1/events/active",
    responses(
        (status = 200, description = "Active events", body = Vec<GameEvent>)
    ),
    tag = "events"
)]
pub async fn get_active_events(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<Vec<GameEvent>>> {
    let rows = sqlx::query_as::<_, EventRow>(
        "SELECT id, name, description, event_type, status, realm, starts_at, ends_at,
                location_x, location_y, location_z, location_name, min_level, max_players, created_at
         FROM game_events
         WHERE status = 'active' AND starts_at <= NOW() AND ends_at > NOW()
         ORDER BY ends_at ASC"
    )
    .fetch_all(&state.db)
    .await?;

    let mut events = Vec::new();
    for row in rows {
        let rewards = load_event_rewards(&state, row.id).await?;
        events.push(build_event(row, rewards));
    }

    Ok(Json(events))
}

/// Get upcoming events
#[utoipa::path(
    get,
    path = "/api/v1/events/upcoming",
    responses(
        (status = 200, description = "Upcoming events", body = Vec<GameEvent>)
    ),
    tag = "events"
)]
pub async fn get_upcoming_events(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<Vec<GameEvent>>> {
    let rows = sqlx::query_as::<_, EventRow>(
        "SELECT id, name, description, event_type, status, realm, starts_at, ends_at,
                location_x, location_y, location_z, location_name, min_level, max_players, created_at
         FROM game_events
         WHERE status = 'upcoming' AND starts_at > NOW()
         ORDER BY starts_at ASC"
    )
    .fetch_all(&state.db)
    .await?;

    let mut events = Vec::new();
    for row in rows {
        let rewards = load_event_rewards(&state, row.id).await?;
        events.push(build_event(row, rewards));
    }

    Ok(Json(events))
}

/// Get event by ID
#[utoipa::path(
    get,
    path = "/api/v1/events/{id}",
    params(
        ("id" = Uuid, Path, description = "Event ID")
    ),
    responses(
        (status = 200, description = "Event details", body = GameEvent)
    ),
    tag = "events"
)]
pub async fn get_event(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<GameEvent>> {
    let row = sqlx::query_as::<_, EventRow>(
        "SELECT id, name, description, event_type, status, realm, starts_at, ends_at,
                location_x, location_y, location_z, location_name, min_level, max_players, created_at
         FROM game_events WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(crate::error::ApiError::NotFound("Event not found".to_string()))?;

    let rewards = load_event_rewards(&state, row.id).await?;

    Ok(Json(build_event(row, rewards)))
}

/// Helper to build GameEvent from row
fn build_event(row: EventRow, rewards: Vec<EventReward>) -> GameEvent {
    let location = if row.location_x.is_some() {
        Some(EventLocation {
            x: row.location_x.unwrap_or(0),
            y: row.location_y.unwrap_or(0),
            z: row.location_z.unwrap_or(0),
            name: row.location_name,
        })
    } else {
        None
    };

    let requirements = if row.min_level.is_some() || row.max_players.is_some() {
        Some(EventRequirements {
            min_level: row.min_level,
            max_players: row.max_players,
        })
    } else {
        None
    };

    GameEvent {
        id: row.id,
        name: row.name,
        description: row.description,
        event_type: row.event_type,
        status: row.status,
        realm: row.realm,
        starts_at: row.starts_at,
        ends_at: row.ends_at,
        rewards,
        location,
        requirements,
        created_at: row.created_at,
    }
}

/// Helper to load event rewards
async fn load_event_rewards(state: &AppState, event_id: Uuid) -> Result<Vec<EventReward>, sqlx::Error> {
    let rows = sqlx::query_as::<_, RewardRow>(
        "SELECT reward_type, item_id, 
                (SELECT name FROM items WHERE id = item_id) as item_name,
                amount, description, tier
         FROM event_rewards
         WHERE event_id = $1
         ORDER BY id"
    )
    .bind(event_id)
    .fetch_all(&state.db)
    .await?;

    Ok(rows.into_iter().map(|r| EventReward {
        reward_type: r.reward_type,
        item_id: r.item_id,
        item_name: r.item_name,
        amount: r.amount,
        description: r.description,
        tier: r.tier,
    }).collect())
}
