//! Kill statistics endpoints

use crate::state::AppState;
use crate::ApiResult;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

/// Kill type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "kill_type", rename_all = "lowercase")]
pub enum KillType {
    Pvp,
    Pve,
    Boss,
}

/// Overall kill statistics
#[derive(Debug, Serialize, ToSchema)]
pub struct KillStatistics {
    pub total_kills: i64,
    pub pvp_kills: i64,
    pub pve_kills: i64,
    pub boss_kills: i64,
    pub total_deaths: i64,
    pub pvp_deaths: i64,
    pub pve_deaths: i64,
    pub unique_killers: i64,
    pub unique_victims: i64,
    pub most_dangerous_area: Option<String>,
    pub most_killed_creature: Option<String>,
    pub last_updated: DateTime<Utc>,
}

/// Top killer entry
#[derive(Debug, Serialize, ToSchema)]
pub struct TopKiller {
    pub character_id: Uuid,
    pub character_name: String,
    pub level: i32,
    pub vocation: String,
    pub kills: i64,
    pub pvp_kills: i64,
    pub pve_kills: i64,
    pub boss_kills: i64,
    pub kill_streak: i32,
    pub realm: String,
}

#[derive(Debug, FromRow)]
struct TopKillerRow {
    character_id: Uuid,
    character_name: String,
    level: i32,
    vocation: String,
    kills: i64,
    pvp_kills: i64,
    pve_kills: i64,
    boss_kills: i64,
    kill_streak: i32,
    realm: String,
}

/// Kill entry (death record)
#[derive(Debug, Serialize, ToSchema)]
pub struct KillEntry {
    pub id: Uuid,
    pub victim_name: String,
    pub victim_level: i32,
    pub victim_vocation: String,
    pub killer_name: String,
    pub killer_level: Option<i32>,
    pub killer_type: String, // "player", "creature"
    pub kill_type: KillType,
    pub damage: i32,
    pub location: String,
    pub realm: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
struct KillEntryRow {
    id: Uuid,
    victim_name: String,
    victim_level: i32,
    victim_vocation: String,
    killer_name: String,
    killer_level: Option<i32>,
    killer_type: String,
    kill_type: KillType,
    damage: i32,
    location: String,
    realm: String,
    timestamp: DateTime<Utc>,
}

/// Boss hunter entry
#[derive(Debug, Serialize, ToSchema)]
pub struct BossHunter {
    pub character_id: Uuid,
    pub character_name: String,
    pub level: i32,
    pub vocation: String,
    pub boss_kills: i64,
    pub unique_bosses: i64,
    pub rarest_kill: Option<String>,
    pub realm: String,
}

#[derive(Debug, FromRow)]
struct BossHunterRow {
    character_id: Uuid,
    character_name: String,
    level: i32,
    vocation: String,
    boss_kills: i64,
    unique_bosses: i64,
    rarest_kill: Option<String>,
    realm: String,
}

/// Paginated kill entries response
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedKillEntries {
    pub data: Vec<KillEntry>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

/// Statistics query parameters
#[derive(Debug, Deserialize)]
pub struct StatsQuery {
    pub realm: Option<String>,
}

/// Top killers query parameters
#[derive(Debug, Deserialize)]
pub struct TopKillersQuery {
    pub realm: Option<String>,
    #[serde(rename = "type")]
    pub kill_type: Option<String>,
    pub time_range: Option<String>,
    pub limit: Option<u32>,
}

/// Recent deaths query parameters
#[derive(Debug, Deserialize)]
pub struct RecentDeathsQuery {
    pub realm: Option<String>,
    #[serde(rename = "type")]
    pub kill_type: Option<String>,
    pub limit: Option<u32>,
}

/// Character kills query parameters
#[derive(Debug, Deserialize)]
pub struct CharacterKillsQuery {
    #[serde(rename = "type")]
    pub kill_type: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

/// Get overall kill statistics
#[utoipa::path(
    get,
    path = "/api/v1/kill-statistics",
    params(
        ("realm" = Option<String>, Query, description = "Filter by realm")
    ),
    responses(
        (status = 200, description = "Kill statistics", body = KillStatistics)
    ),
    tag = "kill-statistics"
)]
pub async fn get_statistics(
    State(state): State<Arc<AppState>>,
    Query(query): Query<StatsQuery>,
) -> ApiResult<Json<KillStatistics>> {
    let stats: (i64, i64, i64, i64, i64, i64, i64, i64, i64) = sqlx::query_as(
        "SELECT 
            COUNT(*) as total_kills,
            COUNT(*) FILTER (WHERE kill_type = 'pvp') as pvp_kills,
            COUNT(*) FILTER (WHERE kill_type = 'pve') as pve_kills,
            COUNT(*) FILTER (WHERE kill_type = 'boss') as boss_kills,
            COUNT(*) as total_deaths,
            COUNT(*) FILTER (WHERE kill_type = 'pvp') as pvp_deaths,
            COUNT(*) FILTER (WHERE kill_type != 'pvp') as pve_deaths,
            COUNT(DISTINCT CASE WHEN killer_type = 'player' THEN killer_id END) as unique_killers,
            COUNT(DISTINCT victim_id) as unique_victims
         FROM kill_records
         WHERE ($1::text IS NULL OR realm = $1)"
    )
    .bind(&query.realm)
    .fetch_one(&state.db)
    .await?;

    let most_dangerous: Option<(String,)> = sqlx::query_as(
        "SELECT location FROM kill_records
         WHERE ($1::text IS NULL OR realm = $1)
         GROUP BY location
         ORDER BY COUNT(*) DESC
         LIMIT 1"
    )
    .bind(&query.realm)
    .fetch_optional(&state.db)
    .await?;

    let most_killed: Option<(String,)> = sqlx::query_as(
        "SELECT killer_name FROM kill_records
         WHERE killer_type = 'creature' AND ($1::text IS NULL OR realm = $1)
         GROUP BY killer_name
         ORDER BY COUNT(*) DESC
         LIMIT 1"
    )
    .bind(&query.realm)
    .fetch_optional(&state.db)
    .await?;

    Ok(Json(KillStatistics {
        total_kills: stats.0,
        pvp_kills: stats.1,
        pve_kills: stats.2,
        boss_kills: stats.3,
        total_deaths: stats.4,
        pvp_deaths: stats.5,
        pve_deaths: stats.6,
        unique_killers: stats.7,
        unique_victims: stats.8,
        most_dangerous_area: most_dangerous.map(|r| r.0),
        most_killed_creature: most_killed.map(|r| r.0),
        last_updated: Utc::now(),
    }))
}

/// Get top killers
#[utoipa::path(
    get,
    path = "/api/v1/kill-statistics/top-killers",
    params(
        ("realm" = Option<String>, Query, description = "Filter by realm"),
        ("type" = Option<String>, Query, description = "Kill type: pvp, pve, boss, all"),
        ("time_range" = Option<String>, Query, description = "Time range: today, week, month, all"),
        ("limit" = Option<u32>, Query, description = "Max results")
    ),
    responses(
        (status = 200, description = "Top killers list", body = Vec<TopKiller>)
    ),
    tag = "kill-statistics"
)]
pub async fn get_top_killers(
    State(state): State<Arc<AppState>>,
    Query(query): Query<TopKillersQuery>,
) -> ApiResult<Json<Vec<TopKiller>>> {
    let limit = query.limit.unwrap_or(10).min(100) as i64;
    
    let time_filter = match query.time_range.as_deref() {
        Some("today") => Some(Utc::now() - Duration::days(1)),
        Some("week") => Some(Utc::now() - Duration::weeks(1)),
        Some("month") => Some(Utc::now() - Duration::days(30)),
        _ => None,
    };

    let type_filter = match query.kill_type.as_deref() {
        Some("pvp") => Some("pvp"),
        Some("pve") => Some("pve"),
        Some("boss") => Some("boss"),
        _ => None,
    };

    let rows = sqlx::query_as::<_, TopKillerRow>(
        "WITH killer_stats AS (
            SELECT 
                kr.killer_id as character_id,
                c.name as character_name,
                c.level,
                c.vocation::text,
                COUNT(*) as kills,
                COUNT(*) FILTER (WHERE kr.kill_type = 'pvp') as pvp_kills,
                COUNT(*) FILTER (WHERE kr.kill_type = 'pve') as pve_kills,
                COUNT(*) FILTER (WHERE kr.kill_type = 'boss') as boss_kills,
                COALESCE(c.kill_streak, 0) as kill_streak,
                r.name as realm
            FROM kill_records kr
            JOIN characters c ON kr.killer_id = c.id
            JOIN realms r ON c.realm_id = r.id
            WHERE kr.killer_type = 'player'
              AND ($1::text IS NULL OR r.slug = $1)
              AND ($2::kill_type IS NULL OR kr.kill_type = $2)
              AND ($3::timestamptz IS NULL OR kr.timestamp >= $3)
            GROUP BY kr.killer_id, c.name, c.level, c.vocation, c.kill_streak, r.name
        )
        SELECT * FROM killer_stats
        ORDER BY kills DESC
        LIMIT $4"
    )
    .bind(&query.realm)
    .bind(type_filter.map(|t| KillType::from_str(t)))
    .bind(time_filter)
    .bind(limit)
    .fetch_all(&state.db)
    .await?;

    let killers = rows.into_iter().map(|r| TopKiller {
        character_id: r.character_id,
        character_name: r.character_name,
        level: r.level,
        vocation: r.vocation,
        kills: r.kills,
        pvp_kills: r.pvp_kills,
        pve_kills: r.pve_kills,
        boss_kills: r.boss_kills,
        kill_streak: r.kill_streak,
        realm: r.realm,
    }).collect();

    Ok(Json(killers))
}

/// Get recent deaths
#[utoipa::path(
    get,
    path = "/api/v1/kill-statistics/recent",
    params(
        ("realm" = Option<String>, Query, description = "Filter by realm"),
        ("type" = Option<String>, Query, description = "Kill type: pvp, pve, boss, all"),
        ("limit" = Option<u32>, Query, description = "Max results")
    ),
    responses(
        (status = 200, description = "Recent deaths list", body = Vec<KillEntry>)
    ),
    tag = "kill-statistics"
)]
pub async fn get_recent_deaths(
    State(state): State<Arc<AppState>>,
    Query(query): Query<RecentDeathsQuery>,
) -> ApiResult<Json<Vec<KillEntry>>> {
    let limit = query.limit.unwrap_or(20).min(100) as i64;

    let type_filter = match query.kill_type.as_deref() {
        Some("pvp") => Some("pvp"),
        Some("pve") => Some("pve"),
        Some("boss") => Some("boss"),
        _ => None,
    };

    let rows = sqlx::query_as::<_, KillEntryRow>(
        "SELECT 
            kr.id,
            cv.name as victim_name,
            cv.level as victim_level,
            cv.vocation::text as victim_vocation,
            COALESCE(ck.name, kr.killer_name) as killer_name,
            ck.level as killer_level,
            kr.killer_type,
            kr.kill_type,
            kr.damage,
            kr.location,
            r.name as realm,
            kr.timestamp
         FROM kill_records kr
         JOIN characters cv ON kr.victim_id = cv.id
         LEFT JOIN characters ck ON kr.killer_id = ck.id AND kr.killer_type = 'player'
         JOIN realms r ON cv.realm_id = r.id
         WHERE ($1::text IS NULL OR r.slug = $1)
           AND ($2::kill_type IS NULL OR kr.kill_type = $2)
         ORDER BY kr.timestamp DESC
         LIMIT $3"
    )
    .bind(&query.realm)
    .bind(type_filter.map(|t| KillType::from_str(t)))
    .bind(limit)
    .fetch_all(&state.db)
    .await?;

    let entries = rows.into_iter().map(|r| KillEntry {
        id: r.id,
        victim_name: r.victim_name,
        victim_level: r.victim_level,
        victim_vocation: r.victim_vocation,
        killer_name: r.killer_name,
        killer_level: r.killer_level,
        killer_type: r.killer_type,
        kill_type: r.kill_type,
        damage: r.damage,
        location: r.location,
        realm: r.realm,
        timestamp: r.timestamp,
    }).collect();

    Ok(Json(entries))
}

/// Get boss hunters
#[utoipa::path(
    get,
    path = "/api/v1/kill-statistics/boss-hunters",
    params(
        ("realm" = Option<String>, Query, description = "Filter by realm"),
        ("limit" = Option<u32>, Query, description = "Max results")
    ),
    responses(
        (status = 200, description = "Boss hunters list", body = Vec<BossHunter>)
    ),
    tag = "kill-statistics"
)]
pub async fn get_boss_hunters(
    State(state): State<Arc<AppState>>,
    Query(query): Query<StatsQuery>,
) -> ApiResult<Json<Vec<BossHunter>>> {
    let limit = 10i64; // Default limit

    let rows = sqlx::query_as::<_, BossHunterRow>(
        "WITH boss_stats AS (
            SELECT 
                kr.killer_id as character_id,
                c.name as character_name,
                c.level,
                c.vocation::text,
                COUNT(*) as boss_kills,
                COUNT(DISTINCT kr.killer_name) as unique_bosses,
                (SELECT killer_name FROM kill_records 
                 WHERE killer_type = 'creature' 
                   AND kill_type = 'boss'
                   AND killer_id = kr.killer_id
                 ORDER BY (SELECT creature_rarity FROM creatures WHERE name = killer_name) DESC NULLS LAST
                 LIMIT 1) as rarest_kill,
                r.name as realm
            FROM kill_records kr
            JOIN characters c ON kr.killer_id = c.id
            JOIN realms r ON c.realm_id = r.id
            WHERE kr.killer_type = 'player' AND kr.kill_type = 'boss'
              AND ($1::text IS NULL OR r.slug = $1)
            GROUP BY kr.killer_id, c.name, c.level, c.vocation, r.name
        )
        SELECT * FROM boss_stats
        ORDER BY boss_kills DESC, unique_bosses DESC
        LIMIT $2"
    )
    .bind(&query.realm)
    .bind(limit)
    .fetch_all(&state.db)
    .await?;

    let hunters = rows.into_iter().map(|r| BossHunter {
        character_id: r.character_id,
        character_name: r.character_name,
        level: r.level,
        vocation: r.vocation,
        boss_kills: r.boss_kills,
        unique_bosses: r.unique_bosses,
        rarest_kill: r.rarest_kill,
        realm: r.realm,
    }).collect();

    Ok(Json(hunters))
}

/// Get character kills
#[utoipa::path(
    get,
    path = "/api/v1/kill-statistics/character/{character_id}",
    params(
        ("character_id" = String, Path, description = "Character ID or UUID"),
        ("type" = Option<String>, Query, description = "Kill type filter"),
        ("page" = Option<u32>, Query, description = "Page number"),
        ("page_size" = Option<u32>, Query, description = "Results per page")
    ),
    responses(
        (status = 200, description = "Character kill history", body = PaginatedKillEntries)
    ),
    tag = "kill-statistics"
)]
pub async fn get_character_kills(
    State(state): State<Arc<AppState>>,
    Path(character_id): Path<String>,
    Query(query): Query<CharacterKillsQuery>,
) -> ApiResult<Json<PaginatedKillEntries>> {
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(20).min(100);
    let offset = (page - 1) * page_size;

    let type_filter = match query.kill_type.as_deref() {
        Some("pvp") => Some("pvp"),
        Some("pve") => Some("pve"),
        Some("boss") => Some("boss"),
        _ => None,
    };

    // Try to parse as UUID first, otherwise treat as name lookup
    let char_uuid: Option<Uuid> = character_id.parse().ok();

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM kill_records kr
         JOIN characters c ON kr.killer_id = c.id
         WHERE (c.uuid = $1 OR ($1 IS NULL AND c.name = $2))
           AND kr.killer_type = 'player'
           AND ($3::kill_type IS NULL OR kr.kill_type = $3)"
    )
    .bind(char_uuid)
    .bind(&character_id)
    .bind(type_filter.map(|t| KillType::from_str(t)))
    .fetch_one(&state.db)
    .await?;

    let rows = sqlx::query_as::<_, KillEntryRow>(
        "SELECT 
            kr.id,
            cv.name as victim_name,
            cv.level as victim_level,
            cv.vocation::text as victim_vocation,
            COALESCE(ck.name, kr.killer_name) as killer_name,
            ck.level as killer_level,
            kr.killer_type,
            kr.kill_type,
            kr.damage,
            kr.location,
            r.name as realm,
            kr.timestamp
         FROM kill_records kr
         JOIN characters cv ON kr.victim_id = cv.id
         JOIN characters ck ON kr.killer_id = ck.id
         JOIN realms r ON cv.realm_id = r.id
         WHERE (ck.uuid = $1 OR ($1::uuid IS NULL AND ck.name = $2))
           AND kr.killer_type = 'player'
           AND ($3::kill_type IS NULL OR kr.kill_type = $3)
         ORDER BY kr.timestamp DESC
         LIMIT $4 OFFSET $5"
    )
    .bind(char_uuid)
    .bind(&character_id)
    .bind(type_filter.map(|t| KillType::from_str(t)))
    .bind(page_size as i64)
    .bind(offset as i64)
    .fetch_all(&state.db)
    .await?;

    let entries: Vec<KillEntry> = rows.into_iter().map(|r| KillEntry {
        id: r.id,
        victim_name: r.victim_name,
        victim_level: r.victim_level,
        victim_vocation: r.victim_vocation,
        killer_name: r.killer_name,
        killer_level: r.killer_level,
        killer_type: r.killer_type,
        kill_type: r.kill_type,
        damage: r.damage,
        location: r.location,
        realm: r.realm,
        timestamp: r.timestamp,
    }).collect();

    Ok(Json(PaginatedKillEntries {
        data: entries,
        total: total.0,
        page,
        page_size,
        total_pages: ((total.0 as f64) / (page_size as f64)).ceil() as u32,
    }))
}

impl KillType {
    fn from_str(s: &str) -> Self {
        match s {
            "pvp" => Self::Pvp,
            "pve" => Self::Pve,
            "boss" => Self::Boss,
            _ => Self::Pve,
        }
    }
}
