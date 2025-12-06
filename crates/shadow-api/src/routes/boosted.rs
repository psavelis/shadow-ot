//! Boosted creature and boss endpoints

use crate::state::AppState;
use crate::ApiResult;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use utoipa::ToSchema;

/// Boosted creature
#[derive(Debug, Serialize, ToSchema)]
pub struct BoostedCreature {
    pub id: i32,
    pub name: String,
    pub race: String,
    pub sprite_id: i32,
    pub experience_bonus: i32,
    pub loot_bonus: i32,
    pub date: NaiveDate,
}

#[derive(Debug, FromRow)]
struct BoostedCreatureRow {
    id: i32,
    name: String,
    race: String,
    sprite_id: i32,
    experience_bonus: i32,
    loot_bonus: i32,
    date: NaiveDate,
}

/// Boosted boss
#[derive(Debug, Serialize, ToSchema)]
pub struct BoostedBoss {
    pub id: i32,
    pub name: String,
    pub race: String,
    pub sprite_id: i32,
    pub experience_bonus: i32,
    pub loot_bonus: i32,
    pub date: NaiveDate,
}

#[derive(Debug, FromRow)]
struct BoostedBossRow {
    id: i32,
    name: String,
    race: String,
    sprite_id: i32,
    experience_bonus: i32,
    loot_bonus: i32,
    date: NaiveDate,
}

/// History query parameters
#[derive(Debug, Deserialize)]
pub struct HistoryQuery {
    pub days: Option<i32>,
}

/// Get today's boosted creature
#[utoipa::path(
    get,
    path = "/api/v1/boosted/creature",
    responses(
        (status = 200, description = "Today's boosted creature", body = BoostedCreature)
    ),
    tag = "boosted"
)]
pub async fn get_boosted_creature(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<BoostedCreature>> {
    let today = Utc::now().date_naive();
    
    let row = sqlx::query_as::<_, BoostedCreatureRow>(
        "SELECT c.id, c.name, c.race, c.sprite_id, bc.experience_bonus, bc.loot_bonus, bc.date
         FROM boosted_creatures bc
         JOIN creatures c ON bc.creature_id = c.id
         WHERE bc.date = $1"
    )
    .bind(today)
    .fetch_optional(&state.db)
    .await?
    .ok_or(crate::error::ApiError::NotFound("No boosted creature today".to_string()))?;

    Ok(Json(BoostedCreature {
        id: row.id,
        name: row.name,
        race: row.race,
        sprite_id: row.sprite_id,
        experience_bonus: row.experience_bonus,
        loot_bonus: row.loot_bonus,
        date: row.date,
    }))
}

/// Get today's boosted boss
#[utoipa::path(
    get,
    path = "/api/v1/boosted/boss",
    responses(
        (status = 200, description = "Today's boosted boss", body = BoostedBoss)
    ),
    tag = "boosted"
)]
pub async fn get_boosted_boss(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<BoostedBoss>> {
    let today = Utc::now().date_naive();
    
    let row = sqlx::query_as::<_, BoostedBossRow>(
        "SELECT c.id, c.name, c.race, c.sprite_id, bb.experience_bonus, bb.loot_bonus, bb.date
         FROM boosted_bosses bb
         JOIN creatures c ON bb.boss_id = c.id
         WHERE bb.date = $1"
    )
    .bind(today)
    .fetch_optional(&state.db)
    .await?
    .ok_or(crate::error::ApiError::NotFound("No boosted boss today".to_string()))?;

    Ok(Json(BoostedBoss {
        id: row.id,
        name: row.name,
        race: row.race,
        sprite_id: row.sprite_id,
        experience_bonus: row.experience_bonus,
        loot_bonus: row.loot_bonus,
        date: row.date,
    }))
}

/// Get boosted creature history
#[utoipa::path(
    get,
    path = "/api/v1/boosted/creature/history",
    params(
        ("days" = Option<i32>, Query, description = "Number of days of history")
    ),
    responses(
        (status = 200, description = "Boosted creature history", body = Vec<BoostedCreature>)
    ),
    tag = "boosted"
)]
pub async fn get_creature_history(
    State(state): State<Arc<AppState>>,
    Query(query): Query<HistoryQuery>,
) -> ApiResult<Json<Vec<BoostedCreature>>> {
    let days = query.days.unwrap_or(30).min(365);
    
    let rows = sqlx::query_as::<_, BoostedCreatureRow>(
        "SELECT c.id, c.name, c.race, c.sprite_id, bc.experience_bonus, bc.loot_bonus, bc.date
         FROM boosted_creatures bc
         JOIN creatures c ON bc.creature_id = c.id
         WHERE bc.date >= CURRENT_DATE - $1::integer
         ORDER BY bc.date DESC"
    )
    .bind(days)
    .fetch_all(&state.db)
    .await?;

    let creatures = rows.into_iter().map(|r| BoostedCreature {
        id: r.id,
        name: r.name,
        race: r.race,
        sprite_id: r.sprite_id,
        experience_bonus: r.experience_bonus,
        loot_bonus: r.loot_bonus,
        date: r.date,
    }).collect();

    Ok(Json(creatures))
}

/// Get boosted boss history
#[utoipa::path(
    get,
    path = "/api/v1/boosted/boss/history",
    params(
        ("days" = Option<i32>, Query, description = "Number of days of history")
    ),
    responses(
        (status = 200, description = "Boosted boss history", body = Vec<BoostedBoss>)
    ),
    tag = "boosted"
)]
pub async fn get_boss_history(
    State(state): State<Arc<AppState>>,
    Query(query): Query<HistoryQuery>,
) -> ApiResult<Json<Vec<BoostedBoss>>> {
    let days = query.days.unwrap_or(30).min(365);
    
    let rows = sqlx::query_as::<_, BoostedBossRow>(
        "SELECT c.id, c.name, c.race, c.sprite_id, bb.experience_bonus, bb.loot_bonus, bb.date
         FROM boosted_bosses bb
         JOIN creatures c ON bb.boss_id = c.id
         WHERE bb.date >= CURRENT_DATE - $1::integer
         ORDER BY bb.date DESC"
    )
    .bind(days)
    .fetch_all(&state.db)
    .await?;

    let bosses = rows.into_iter().map(|r| BoostedBoss {
        id: r.id,
        name: r.name,
        race: r.race,
        sprite_id: r.sprite_id,
        experience_bonus: r.experience_bonus,
        loot_bonus: r.loot_bonus,
        date: r.date,
    }).collect();

    Ok(Json(bosses))
}
