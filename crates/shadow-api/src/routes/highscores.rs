//! Highscore endpoints

use crate::error::ApiError;
use crate::state::AppState;
use crate::ApiResult;
use axum::{extract::{Path, Query, State}, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;

/// Highscore entry
#[derive(Debug, Serialize, ToSchema)]
pub struct HighscoreEntry {
    pub rank: i64,
    pub name: String,
    pub vocation: i16,
    pub level: i32,
    pub value: i64,
    pub guild_name: Option<String>,
}

/// Highscore query parameters
#[derive(Debug, Deserialize)]
pub struct HighscoreQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub vocation: Option<i16>,
}

/// Get highscores
#[utoipa::path(
    get,
    path = "/api/v1/highscores/{realm}",
    params(
        ("realm" = i32, Path, description = "Realm ID"),
        ("page" = Option<u32>, Query, description = "Page number"),
        ("limit" = Option<u32>, Query, description = "Results per page"),
        ("vocation" = Option<i16>, Query, description = "Filter by vocation")
    ),
    responses(
        (status = 200, description = "Highscore list", body = Vec<HighscoreEntry>)
    ),
    tag = "highscores"
)]
pub async fn get_highscores(
    State(state): State<Arc<AppState>>,
    Path(realm): Path<i32>,
    Query(query): Query<HighscoreQuery>,
) -> ApiResult<Json<Vec<HighscoreEntry>>> {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(50).min(100);
    let offset = (page - 1) * limit;

    let entries = if let Some(vocation) = query.vocation {
        sqlx::query_as::<_, HighscoreRow>(
            "SELECT c.name, c.vocation, c.level, c.experience as value, g.name as guild_name,
                    ROW_NUMBER() OVER (ORDER BY c.experience DESC) as rank
             FROM characters c
             LEFT JOIN guild_members gm ON c.id = gm.character_id
             LEFT JOIN guilds g ON gm.guild_id = g.id
             WHERE c.realm_id = $1 AND c.vocation = $2 AND c.deletion_time IS NULL
             ORDER BY c.experience DESC
             LIMIT $3 OFFSET $4"
        )
        .bind(realm)
        .bind(vocation)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&state.db)
        .await?
    } else {
        sqlx::query_as::<_, HighscoreRow>(
            "SELECT c.name, c.vocation, c.level, c.experience as value, g.name as guild_name,
                    ROW_NUMBER() OVER (ORDER BY c.experience DESC) as rank
             FROM characters c
             LEFT JOIN guild_members gm ON c.id = gm.character_id
             LEFT JOIN guilds g ON gm.guild_id = g.id
             WHERE c.realm_id = $1 AND c.deletion_time IS NULL
             ORDER BY c.experience DESC
             LIMIT $2 OFFSET $3"
        )
        .bind(realm)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&state.db)
        .await?
    };

    Ok(Json(entries.into_iter().map(|e| HighscoreEntry {
        rank: e.rank,
        name: e.name,
        vocation: e.vocation,
        level: e.level,
        value: e.value,
        guild_name: e.guild_name,
    }).collect()))
}

/// Highscore types
#[derive(Debug, Clone, Copy)]
pub enum HighscoreType {
    Experience,
    MagicLevel,
    Fist,
    Club,
    Sword,
    Axe,
    Distance,
    Shielding,
    Fishing,
}

impl HighscoreType {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "experience" | "level" => Some(Self::Experience),
            "magic" | "magiclevel" => Some(Self::MagicLevel),
            "fist" => Some(Self::Fist),
            "club" => Some(Self::Club),
            "sword" => Some(Self::Sword),
            "axe" => Some(Self::Axe),
            "distance" => Some(Self::Distance),
            "shielding" => Some(Self::Shielding),
            "fishing" => Some(Self::Fishing),
            _ => None,
        }
    }

    fn column(&self) -> &'static str {
        match self {
            Self::Experience => "experience",
            Self::MagicLevel => "magic_level",
            Self::Fist => "skill_fist",
            Self::Club => "skill_club",
            Self::Sword => "skill_sword",
            Self::Axe => "skill_axe",
            Self::Distance => "skill_dist",
            Self::Shielding => "skill_shielding",
            Self::Fishing => "skill_fishing",
        }
    }
}

/// Get highscores by type
pub async fn get_highscores_by_type(
    State(state): State<Arc<AppState>>,
    Path((realm, highscore_type)): Path<(i32, String)>,
    Query(query): Query<HighscoreQuery>,
) -> ApiResult<Json<Vec<HighscoreEntry>>> {
    let hs_type = HighscoreType::from_str(&highscore_type)
        .ok_or(ApiError::BadRequest("Invalid highscore type".to_string()))?;

    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(50).min(100);
    let offset = (page - 1) * limit;

    let column = hs_type.column();

    // Build query dynamically (safe because column is from enum)
    let sql = format!(
        "SELECT c.name, c.vocation, c.level, c.{} as value, g.name as guild_name,
                ROW_NUMBER() OVER (ORDER BY c.{} DESC) as rank
         FROM characters c
         LEFT JOIN guild_members gm ON c.id = gm.character_id
         LEFT JOIN guilds g ON gm.guild_id = g.id
         WHERE c.realm_id = $1 AND c.deletion_time IS NULL
         ORDER BY c.{} DESC
         LIMIT $2 OFFSET $3",
        column, column, column
    );

    let entries = sqlx::query_as::<_, HighscoreRow>(&sql)
        .bind(realm)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&state.db)
        .await?;

    Ok(Json(entries.into_iter().map(|e| HighscoreEntry {
        rank: e.rank,
        name: e.name,
        vocation: e.vocation,
        level: e.level,
        value: e.value,
        guild_name: e.guild_name,
    }).collect()))
}

#[derive(sqlx::FromRow)]
struct HighscoreRow {
    name: String,
    vocation: i16,
    level: i32,
    value: i64,
    guild_name: Option<String>,
    rank: i64,
}
