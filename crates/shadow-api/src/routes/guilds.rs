//! Guild endpoints

use crate::error::ApiError;
use crate::state::AppState;
use crate::ApiResult;
use axum::{extract::{Path, Query, State}, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;

/// Guild response
#[derive(Debug, Serialize, ToSchema)]
pub struct GuildResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub motd: Option<String>,
    pub logo_url: Option<String>,
    pub level: i32,
    pub owner_name: Option<String>,
    pub member_count: i64,
    pub creation_date: String,
}

/// Guild query
#[derive(Debug, Deserialize)]
pub struct GuildQuery {
    pub realm_id: Option<i32>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

/// List guilds
#[utoipa::path(
    get,
    path = "/api/v1/guilds",
    params(
        ("realm_id" = Option<i32>, Query, description = "Filter by realm"),
        ("page" = Option<u32>, Query, description = "Page number"),
        ("limit" = Option<u32>, Query, description = "Results per page")
    ),
    responses(
        (status = 200, description = "Guild list", body = Vec<GuildResponse>)
    ),
    tag = "guilds"
)]
pub async fn list_guilds(
    State(state): State<Arc<AppState>>,
    Query(query): Query<GuildQuery>,
) -> ApiResult<Json<Vec<GuildResponse>>> {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(50).min(100);
    let offset = (page - 1) * limit;

    let guilds = if let Some(realm_id) = query.realm_id {
        sqlx::query_as::<_, GuildRow>(
            "SELECT g.*, c.name as owner_name,
                    (SELECT COUNT(*) FROM guild_members WHERE guild_id = g.id) as member_count
             FROM guilds g
             LEFT JOIN characters c ON g.owner_id = c.id
             WHERE g.realm_id = $1
             ORDER BY g.level DESC, g.name
             LIMIT $2 OFFSET $3"
        )
        .bind(realm_id)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&state.db)
        .await?
    } else {
        sqlx::query_as::<_, GuildRow>(
            "SELECT g.*, c.name as owner_name,
                    (SELECT COUNT(*) FROM guild_members WHERE guild_id = g.id) as member_count
             FROM guilds g
             LEFT JOIN characters c ON g.owner_id = c.id
             ORDER BY g.level DESC, g.name
             LIMIT $1 OFFSET $2"
        )
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&state.db)
        .await?
    };

    Ok(Json(guilds.into_iter().map(Into::into).collect()))
}

/// Get guild by ID
#[utoipa::path(
    get,
    path = "/api/v1/guilds/{id}",
    params(
        ("id" = i32, Path, description = "Guild ID")
    ),
    responses(
        (status = 200, description = "Guild information", body = GuildResponse),
        (status = 404, description = "Guild not found")
    ),
    tag = "guilds"
)]
pub async fn get_guild(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> ApiResult<Json<GuildResponse>> {
    let guild = sqlx::query_as::<_, GuildRow>(
        "SELECT g.*, c.name as owner_name,
                (SELECT COUNT(*) FROM guild_members WHERE guild_id = g.id) as member_count
         FROM guilds g
         LEFT JOIN characters c ON g.owner_id = c.id
         WHERE g.id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound("Guild not found".to_string()))?;

    Ok(Json(guild.into()))
}

/// Guild member
#[derive(Debug, Serialize)]
pub struct GuildMember {
    pub character_id: i32,
    pub name: String,
    pub rank_name: String,
    pub level: i32,
    pub vocation: i16,
    pub online: bool,
    pub nick: Option<String>,
    pub joined_at: String,
}

/// Get guild members
pub async fn get_guild_members(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> ApiResult<Json<Vec<GuildMember>>> {
    let members = sqlx::query_as::<_, GuildMemberRow>(
        "SELECT gm.character_id, c.name, gr.name as rank_name, c.level, c.vocation, c.online, gm.nick, gm.joined_at
         FROM guild_members gm
         JOIN characters c ON gm.character_id = c.id
         LEFT JOIN guild_ranks gr ON gm.rank_id = gr.id
         WHERE gm.guild_id = $1
         ORDER BY gr.level DESC, c.name"
    )
    .bind(id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(members.into_iter().map(|m| GuildMember {
        character_id: m.character_id,
        name: m.name,
        rank_name: m.rank_name.unwrap_or_else(|| "Member".to_string()),
        level: m.level,
        vocation: m.vocation,
        online: m.online,
        nick: m.nick,
        joined_at: m.joined_at.to_rfc3339(),
    }).collect()))
}

/// Guild war
#[derive(Debug, Serialize)]
pub struct GuildWar {
    pub id: i32,
    pub opponent_name: String,
    pub opponent_id: i32,
    pub status: String,
    pub guild_frags: i32,
    pub opponent_frags: i32,
    pub frag_limit: i32,
    pub started_at: Option<String>,
}

/// Get guild wars
pub async fn get_guild_wars(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> ApiResult<Json<Vec<GuildWar>>> {
    let wars = sqlx::query_as::<_, GuildWarRow>(
        "SELECT gw.*,
                CASE WHEN gw.guild1_id = $1 THEN g2.name ELSE g1.name END as opponent_name,
                CASE WHEN gw.guild1_id = $1 THEN gw.guild2_id ELSE gw.guild1_id END as opponent_id,
                CASE WHEN gw.guild1_id = $1 THEN gw.guild1_frags ELSE gw.guild2_frags END as guild_frags,
                CASE WHEN gw.guild1_id = $1 THEN gw.guild2_frags ELSE gw.guild1_frags END as opponent_frags
         FROM guild_wars gw
         JOIN guilds g1 ON gw.guild1_id = g1.id
         JOIN guilds g2 ON gw.guild2_id = g2.id
         WHERE gw.guild1_id = $1 OR gw.guild2_id = $1
         ORDER BY gw.created_at DESC"
    )
    .bind(id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(wars.into_iter().map(|w| GuildWar {
        id: w.id,
        opponent_name: w.opponent_name,
        opponent_id: w.opponent_id,
        status: w.status,
        guild_frags: w.guild_frags,
        opponent_frags: w.opponent_frags,
        frag_limit: w.frag_limit,
        started_at: w.started_at.map(|t| t.to_rfc3339()),
    }).collect()))
}

// Helper types

#[derive(sqlx::FromRow)]
struct GuildRow {
    id: i32,
    name: String,
    description: Option<String>,
    motd: Option<String>,
    logo_url: Option<String>,
    level: i32,
    owner_name: Option<String>,
    member_count: i64,
    creation_date: chrono::NaiveDate,
}

impl From<GuildRow> for GuildResponse {
    fn from(row: GuildRow) -> Self {
        GuildResponse {
            id: row.id,
            name: row.name,
            description: row.description,
            motd: row.motd,
            logo_url: row.logo_url,
            level: row.level,
            owner_name: row.owner_name,
            member_count: row.member_count,
            creation_date: row.creation_date.to_string(),
        }
    }
}

#[derive(sqlx::FromRow)]
struct GuildMemberRow {
    character_id: i32,
    name: String,
    rank_name: Option<String>,
    level: i32,
    vocation: i16,
    online: bool,
    nick: Option<String>,
    joined_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow)]
struct GuildWarRow {
    id: i32,
    status: String,
    frag_limit: i32,
    opponent_name: String,
    opponent_id: i32,
    guild_frags: i32,
    opponent_frags: i32,
    started_at: Option<chrono::DateTime<chrono::Utc>>,
}
