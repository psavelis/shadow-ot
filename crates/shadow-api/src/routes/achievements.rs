//! Achievement endpoints

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

/// Achievement category
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "achievement_category", rename_all = "lowercase")]
pub enum AchievementCategory {
    Exploration,
    Combat,
    Social,
    Economy,
    Collection,
    Special,
}

/// Achievement rarity
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "achievement_rarity", rename_all = "lowercase")]
pub enum AchievementRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

/// Achievement definition
#[derive(Debug, Serialize, ToSchema)]
pub struct Achievement {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub category: AchievementCategory,
    pub points: i32,
    pub rarity: AchievementRarity,
    pub secret: bool,
    pub icon: Option<String>,
    pub requirements: Option<String>,
}

#[derive(Debug, FromRow)]
struct AchievementRow {
    id: i32,
    name: String,
    description: String,
    category: AchievementCategory,
    points: i32,
    rarity: AchievementRarity,
    secret: bool,
    icon: Option<String>,
    requirements: Option<String>,
}

/// Player achievement with unlock status
#[derive(Debug, Serialize, ToSchema)]
pub struct PlayerAchievement {
    pub achievement: Achievement,
    pub unlocked: bool,
    pub unlocked_at: Option<DateTime<Utc>>,
    pub progress: Option<AchievementProgress>,
}

/// Achievement progress
#[derive(Debug, Serialize, ToSchema)]
pub struct AchievementProgress {
    pub current: i32,
    pub required: i32,
}

/// Achievement summary response
#[derive(Debug, Serialize, ToSchema)]
pub struct AchievementSummary {
    pub achievements: Vec<PlayerAchievement>,
    pub total_points: i32,
    pub completed_count: i32,
    pub total_count: i32,
}

/// Leaderboard entry
#[derive(Debug, Serialize, ToSchema)]
pub struct AchievementLeaderboardEntry {
    pub rank: i32,
    pub character_id: Uuid,
    pub character_name: String,
    pub level: i32,
    pub vocation: String,
    pub points: i32,
    pub completed_count: i32,
}

#[derive(Debug, FromRow)]
struct LeaderboardRow {
    character_id: Uuid,
    character_name: String,
    level: i32,
    vocation: String,
    points: i64,
    completed_count: i64,
}

/// Paginated achievements response
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedAchievements {
    pub data: Vec<Achievement>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

/// Paginated leaderboard response
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedLeaderboard {
    pub data: Vec<AchievementLeaderboardEntry>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

/// Achievement query parameters
#[derive(Debug, Deserialize)]
pub struct AchievementQuery {
    pub category: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

/// List all achievements
#[utoipa::path(
    get,
    path = "/api/v1/achievements",
    params(
        ("category" = Option<String>, Query, description = "Filter by category"),
        ("page" = Option<u32>, Query, description = "Page number"),
        ("page_size" = Option<u32>, Query, description = "Results per page")
    ),
    responses(
        (status = 200, description = "Achievements list", body = PaginatedAchievements)
    ),
    tag = "achievements"
)]
pub async fn list_achievements(
    State(state): State<Arc<AppState>>,
    Query(query): Query<AchievementQuery>,
) -> ApiResult<Json<PaginatedAchievements>> {
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(50).min(200);
    let offset = (page - 1) * page_size;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM achievements
         WHERE ($1::text IS NULL OR category::text = $1)"
    )
    .bind(&query.category)
    .fetch_one(&state.db)
    .await?;

    let rows = sqlx::query_as::<_, AchievementRow>(
        "SELECT id, name, description, category, points, rarity, secret, icon, requirements
         FROM achievements
         WHERE ($1::text IS NULL OR category::text = $1)
         ORDER BY points DESC, name ASC
         LIMIT $2 OFFSET $3"
    )
    .bind(&query.category)
    .bind(page_size as i64)
    .bind(offset as i64)
    .fetch_all(&state.db)
    .await?;

    let achievements = rows.into_iter().map(|r| Achievement {
        id: r.id,
        name: r.name,
        description: r.description,
        category: r.category,
        points: r.points,
        rarity: r.rarity,
        secret: r.secret,
        icon: r.icon,
        requirements: r.requirements,
    }).collect();

    Ok(Json(PaginatedAchievements {
        data: achievements,
        total: total.0,
        page,
        page_size,
        total_pages: ((total.0 as f64) / (page_size as f64)).ceil() as u32,
    }))
}

/// Get player achievements
#[utoipa::path(
    get,
    path = "/api/v1/achievements/player",
    params(
        ("characterId" = Option<Uuid>, Query, description = "Character ID (optional, defaults to first character)")
    ),
    responses(
        (status = 200, description = "Player achievements", body = AchievementSummary)
    ),
    security(("bearer_auth" = [])),
    tag = "achievements"
)]
pub async fn get_player_achievements(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> ApiResult<Json<AchievementSummary>> {
    // Get character ID from params or default to first character
    let character_id: Option<Uuid> = params.get("characterId")
        .and_then(|s| s.parse().ok());

    let char_id = if let Some(id) = character_id {
        id
    } else {
        // Get first character for this account
        let first_char: Option<(Uuid,)> = sqlx::query_as(
            "SELECT uuid FROM characters WHERE account_id = $1 ORDER BY created_at LIMIT 1"
        )
        .bind(&claims.sub)
        .fetch_optional(&state.db)
        .await?;

        first_char.ok_or(crate::error::ApiError::NotFound("No characters found".to_string()))?.0
    };

    // Get all achievements
    let all_achievements = sqlx::query_as::<_, AchievementRow>(
        "SELECT id, name, description, category, points, rarity, secret, icon, requirements
         FROM achievements ORDER BY category, points DESC"
    )
    .fetch_all(&state.db)
    .await?;

    // Get player's unlocked achievements
    let unlocked: Vec<(i32, DateTime<Utc>)> = sqlx::query_as(
        "SELECT achievement_id, unlocked_at FROM character_achievements
         WHERE character_id = (SELECT id FROM characters WHERE uuid = $1)"
    )
    .bind(char_id)
    .fetch_all(&state.db)
    .await?;

    let unlocked_map: std::collections::HashMap<i32, DateTime<Utc>> = 
        unlocked.into_iter().collect();

    let mut player_achievements = Vec::new();
    let mut total_points = 0;
    let mut completed_count = 0;

    for row in all_achievements {
        let unlocked = unlocked_map.get(&row.id);
        
        if unlocked.is_some() {
            total_points += row.points;
            completed_count += 1;
        }

        player_achievements.push(PlayerAchievement {
            achievement: Achievement {
                id: row.id,
                name: row.name,
                description: row.description,
                category: row.category,
                points: row.points,
                rarity: row.rarity,
                secret: row.secret,
                icon: row.icon,
                requirements: row.requirements,
            },
            unlocked: unlocked.is_some(),
            unlocked_at: unlocked.copied(),
            progress: None, // Could be loaded from progress table
        });
    }

    Ok(Json(AchievementSummary {
        total_count: player_achievements.len() as i32,
        achievements: player_achievements,
        total_points,
        completed_count,
    }))
}

/// Get achievement leaderboard
#[utoipa::path(
    get,
    path = "/api/v1/achievements/leaderboard",
    params(
        ("page" = Option<u32>, Query, description = "Page number"),
        ("page_size" = Option<u32>, Query, description = "Results per page")
    ),
    responses(
        (status = 200, description = "Achievement leaderboard", body = PaginatedLeaderboard)
    ),
    tag = "achievements"
)]
pub async fn get_leaderboard(
    State(state): State<Arc<AppState>>,
    Query(query): Query<AchievementQuery>,
) -> ApiResult<Json<PaginatedLeaderboard>> {
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(50).min(100);
    let offset = (page - 1) * page_size;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(DISTINCT character_id) FROM character_achievements"
    )
    .fetch_one(&state.db)
    .await?;

    let rows = sqlx::query_as::<_, LeaderboardRow>(
        "SELECT 
            c.uuid as character_id,
            c.name as character_name,
            c.level,
            c.vocation::text as vocation,
            COALESCE(SUM(a.points), 0) as points,
            COUNT(ca.achievement_id) as completed_count
         FROM characters c
         LEFT JOIN character_achievements ca ON ca.character_id = c.id
         LEFT JOIN achievements a ON a.id = ca.achievement_id
         GROUP BY c.id, c.uuid, c.name, c.level, c.vocation
         ORDER BY points DESC, completed_count DESC
         LIMIT $1 OFFSET $2"
    )
    .bind(page_size as i64)
    .bind(offset as i64)
    .fetch_all(&state.db)
    .await?;

    let entries: Vec<AchievementLeaderboardEntry> = rows.into_iter()
        .enumerate()
        .map(|(i, r)| AchievementLeaderboardEntry {
            rank: (offset + i as u32 + 1) as i32,
            character_id: r.character_id,
            character_name: r.character_name,
            level: r.level,
            vocation: r.vocation,
            points: r.points as i32,
            completed_count: r.completed_count as i32,
        })
        .collect();

    Ok(Json(PaginatedLeaderboard {
        data: entries,
        total: total.0,
        page,
        page_size,
        total_pages: ((total.0 as f64) / (page_size as f64)).ceil() as u32,
    }))
}
