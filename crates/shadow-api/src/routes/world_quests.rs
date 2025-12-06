//! World quest endpoints

use crate::auth::JwtClaims;
use crate::state::AppState;
use crate::ApiResult;
use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

/// World quest status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "world_quest_status", rename_all = "lowercase")]
pub enum WorldQuestStatus {
    Active,
    Completed,
    Failed,
}

/// World quest information
#[derive(Debug, Serialize, ToSchema)]
pub struct WorldQuest {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub realm: String,
    pub status: WorldQuestStatus,
    pub required_progress: i64,
    pub current_progress: i64,
    pub progress_percentage: f32,
    pub contributor_count: i32,
    pub rewards: Vec<WorldQuestReward>,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub top_contributors: Vec<TopContributor>,
}

#[derive(Debug, FromRow)]
struct WorldQuestRow {
    id: Uuid,
    name: String,
    description: String,
    realm: String,
    status: WorldQuestStatus,
    required_progress: i64,
    current_progress: i64,
    contributor_count: i32,
    starts_at: DateTime<Utc>,
    ends_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
}

/// World quest reward
#[derive(Debug, Serialize, ToSchema)]
pub struct WorldQuestReward {
    pub reward_type: String,
    pub item_id: Option<i32>,
    pub item_name: Option<String>,
    pub amount: i32,
    pub description: String,
}

#[derive(Debug, FromRow)]
struct RewardRow {
    reward_type: String,
    item_id: Option<i32>,
    item_name: Option<String>,
    amount: i32,
    description: String,
}

/// Top contributor
#[derive(Debug, Serialize, ToSchema)]
pub struct TopContributor {
    pub character_id: Uuid,
    pub character_name: String,
    pub contribution: i64,
}

#[derive(Debug, FromRow)]
struct ContributorRow {
    character_id: Uuid,
    character_name: String,
    contribution: i64,
}

/// Query parameters
#[derive(Debug, Deserialize)]
pub struct WorldQuestQuery {
    pub realm: Option<String>,
    pub status: Option<String>,
}

/// Contribution request
#[derive(Debug, Deserialize, ToSchema)]
pub struct ContributeRequest {
    pub amount: i64,
}

/// Contribution response
#[derive(Debug, Serialize, ToSchema)]
pub struct ContributeResponse {
    pub success: bool,
    pub new_total: i64,
    pub your_contribution: i64,
}

/// List world quests
#[utoipa::path(
    get,
    path = "/api/v1/world-quests",
    params(
        ("realm" = Option<String>, Query, description = "Filter by realm"),
        ("status" = Option<String>, Query, description = "Filter by status")
    ),
    responses(
        (status = 200, description = "World quests list", body = Vec<WorldQuest>)
    ),
    tag = "world-quests"
)]
pub async fn list_world_quests(
    State(state): State<Arc<AppState>>,
    Query(query): Query<WorldQuestQuery>,
) -> ApiResult<Json<Vec<WorldQuest>>> {
    let rows = sqlx::query_as::<_, WorldQuestRow>(
        "SELECT id, name, description, realm, status, required_progress, current_progress,
                contributor_count, starts_at, ends_at, completed_at
         FROM world_quests
         WHERE ($1::text IS NULL OR realm = $1)
           AND ($2::world_quest_status IS NULL OR status = $2)
         ORDER BY starts_at DESC"
    )
    .bind(&query.realm)
    .bind(query.status.as_ref().map(|s| match s.as_str() {
        "active" => WorldQuestStatus::Active,
        "completed" => WorldQuestStatus::Completed,
        "failed" => WorldQuestStatus::Failed,
        _ => WorldQuestStatus::Active,
    }))
    .fetch_all(&state.db)
    .await?;

    let mut quests = Vec::new();
    for row in rows {
        let rewards = load_quest_rewards(&state, row.id).await?;
        let top_contributors = load_top_contributors(&state, row.id).await?;
        let progress_pct = if row.required_progress > 0 {
            (row.current_progress as f32 / row.required_progress as f32) * 100.0
        } else {
            0.0
        };

        quests.push(WorldQuest {
            id: row.id,
            name: row.name,
            description: row.description,
            realm: row.realm,
            status: row.status,
            required_progress: row.required_progress,
            current_progress: row.current_progress,
            progress_percentage: progress_pct.min(100.0),
            contributor_count: row.contributor_count,
            rewards,
            starts_at: row.starts_at,
            ends_at: row.ends_at,
            completed_at: row.completed_at,
            top_contributors,
        });
    }

    Ok(Json(quests))
}

/// Get active world quests
#[utoipa::path(
    get,
    path = "/api/v1/world-quests/active",
    params(
        ("realm" = Option<String>, Query, description = "Filter by realm")
    ),
    responses(
        (status = 200, description = "Active world quests", body = Vec<WorldQuest>)
    ),
    tag = "world-quests"
)]
pub async fn get_active_quests(
    State(state): State<Arc<AppState>>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> ApiResult<Json<Vec<WorldQuest>>> {
    let realm = params.get("realm");

    let rows = sqlx::query_as::<_, WorldQuestRow>(
        "SELECT id, name, description, realm, status, required_progress, current_progress,
                contributor_count, starts_at, ends_at, completed_at
         FROM world_quests
         WHERE status = 'active'
           AND ($1::text IS NULL OR realm = $1)
           AND starts_at <= NOW() AND ends_at > NOW()
         ORDER BY ends_at ASC"
    )
    .bind(realm)
    .fetch_all(&state.db)
    .await?;

    let mut quests = Vec::new();
    for row in rows {
        let rewards = load_quest_rewards(&state, row.id).await?;
        let top_contributors = load_top_contributors(&state, row.id).await?;
        let progress_pct = if row.required_progress > 0 {
            (row.current_progress as f32 / row.required_progress as f32) * 100.0
        } else {
            0.0
        };

        quests.push(WorldQuest {
            id: row.id,
            name: row.name,
            description: row.description,
            realm: row.realm,
            status: row.status,
            required_progress: row.required_progress,
            current_progress: row.current_progress,
            progress_percentage: progress_pct.min(100.0),
            contributor_count: row.contributor_count,
            rewards,
            starts_at: row.starts_at,
            ends_at: row.ends_at,
            completed_at: row.completed_at,
            top_contributors,
        });
    }

    Ok(Json(quests))
}

/// Get world quest by ID
#[utoipa::path(
    get,
    path = "/api/v1/world-quests/{id}",
    params(
        ("id" = Uuid, Path, description = "World quest ID")
    ),
    responses(
        (status = 200, description = "World quest details", body = WorldQuest)
    ),
    tag = "world-quests"
)]
pub async fn get_world_quest(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<WorldQuest>> {
    let row = sqlx::query_as::<_, WorldQuestRow>(
        "SELECT id, name, description, realm, status, required_progress, current_progress,
                contributor_count, starts_at, ends_at, completed_at
         FROM world_quests
         WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(crate::error::ApiError::NotFound("World quest not found".to_string()))?;

    let rewards = load_quest_rewards(&state, row.id).await?;
    let top_contributors = load_top_contributors(&state, row.id).await?;
    let progress_pct = if row.required_progress > 0 {
        (row.current_progress as f32 / row.required_progress as f32) * 100.0
    } else {
        0.0
    };

    Ok(Json(WorldQuest {
        id: row.id,
        name: row.name,
        description: row.description,
        realm: row.realm,
        status: row.status,
        required_progress: row.required_progress,
        current_progress: row.current_progress,
        progress_percentage: progress_pct.min(100.0),
        contributor_count: row.contributor_count,
        rewards,
        starts_at: row.starts_at,
        ends_at: row.ends_at,
        completed_at: row.completed_at,
        top_contributors,
    }))
}

/// Contribute to world quest
#[utoipa::path(
    post,
    path = "/api/v1/world-quests/{id}/contribute",
    params(
        ("id" = Uuid, Path, description = "World quest ID")
    ),
    request_body = ContributeRequest,
    responses(
        (status = 200, description = "Contribution recorded", body = ContributeResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "world-quests"
)]
pub async fn contribute_to_quest(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
    Path(id): Path<Uuid>,
    Json(request): Json<ContributeRequest>,
) -> ApiResult<Json<ContributeResponse>> {
    // Get active character for this account
    let character: Option<(i32, Uuid)> = sqlx::query_as(
        "SELECT id, uuid FROM characters WHERE account_id = $1 ORDER BY last_login DESC LIMIT 1"
    )
    .bind(&claims.sub)
    .fetch_optional(&state.db)
    .await?;

    let (char_id, char_uuid) = character
        .ok_or(crate::error::ApiError::NotFound("No characters found".to_string()))?;

    // Check quest is active
    let quest_status: Option<(WorldQuestStatus,)> = sqlx::query_as(
        "SELECT status FROM world_quests WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?;

    match quest_status {
        Some((WorldQuestStatus::Active,)) => {},
        _ => return Err(crate::error::ApiError::BadRequest("Quest is not active".to_string())),
    }

    // Record contribution
    sqlx::query(
        "INSERT INTO world_quest_contributions (quest_id, character_id, amount)
         VALUES ($1, $2, $3)
         ON CONFLICT (quest_id, character_id) DO UPDATE SET
            amount = world_quest_contributions.amount + $3,
            contributed_at = CURRENT_TIMESTAMP"
    )
    .bind(id)
    .bind(char_id)
    .bind(request.amount)
    .execute(&state.db)
    .await?;

    // Update quest progress
    let new_progress: (i64,) = sqlx::query_as(
        "UPDATE world_quests SET 
            current_progress = current_progress + $2,
            contributor_count = (SELECT COUNT(DISTINCT character_id) FROM world_quest_contributions WHERE quest_id = $1)
         WHERE id = $1
         RETURNING current_progress"
    )
    .bind(id)
    .bind(request.amount)
    .fetch_one(&state.db)
    .await?;

    // Get player's total contribution
    let your_contribution: (i64,) = sqlx::query_as(
        "SELECT amount FROM world_quest_contributions WHERE quest_id = $1 AND character_id = $2"
    )
    .bind(id)
    .bind(char_id)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(ContributeResponse {
        success: true,
        new_total: new_progress.0,
        your_contribution: your_contribution.0,
    }))
}

/// Helper to load quest rewards
async fn load_quest_rewards(state: &AppState, quest_id: Uuid) -> Result<Vec<WorldQuestReward>, sqlx::Error> {
    let rows = sqlx::query_as::<_, RewardRow>(
        "SELECT reward_type, item_id, 
                (SELECT name FROM items WHERE id = item_id) as item_name,
                amount, description
         FROM world_quest_rewards
         WHERE quest_id = $1
         ORDER BY id"
    )
    .bind(quest_id)
    .fetch_all(&state.db)
    .await?;

    Ok(rows.into_iter().map(|r| WorldQuestReward {
        reward_type: r.reward_type,
        item_id: r.item_id,
        item_name: r.item_name,
        amount: r.amount,
        description: r.description,
    }).collect())
}

/// Helper to load top contributors
async fn load_top_contributors(state: &AppState, quest_id: Uuid) -> Result<Vec<TopContributor>, sqlx::Error> {
    let rows = sqlx::query_as::<_, ContributorRow>(
        "SELECT c.uuid as character_id, c.name as character_name, wqc.amount as contribution
         FROM world_quest_contributions wqc
         JOIN characters c ON c.id = wqc.character_id
         WHERE wqc.quest_id = $1
         ORDER BY wqc.amount DESC
         LIMIT 10"
    )
    .bind(quest_id)
    .fetch_all(&state.db)
    .await?;

    Ok(rows.into_iter().map(|r| TopContributor {
        character_id: r.character_id,
        character_name: r.character_name,
        contribution: r.contribution,
    }).collect())
}
