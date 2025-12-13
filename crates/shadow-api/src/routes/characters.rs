//! Character management endpoints

use crate::error::ApiError;
use crate::middleware::get_claims;
use crate::response::{MessageResponse, OnlineStatusResponse};
use crate::state::AppState;
use crate::domain::{Gender, Vocation};
use crate::ApiResult;
use axum::{extract::{Path, Request, State}, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;

/// Character response
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CharacterResponse {
    pub id: i32,
    pub name: String,
    /// Character gender (male/female)
    #[serde(rename = "gender")]
    pub gender: String,
    pub vocation: i16,
    pub level: i32,
    pub experience: i64,
    pub health: i32,
    pub max_health: i32,
    pub mana: i32,
    pub max_mana: i32,
    pub look_type: i32,
    pub look_head: i16,
    pub look_body: i16,
    pub look_legs: i16,
    pub look_feet: i16,
    pub look_addons: i16,
    pub town_id: i32,
    pub balance: i64,
    pub bank_balance: i64,
    pub online: bool,
    pub realm_id: i32,
    pub realm_name: Option<String>,
    pub last_login: Option<String>,
    pub created_at: String,
}

/// List characters for account
#[utoipa::path(
    get,
    path = "/api/v1/characters",
    responses(
        (status = 200, description = "Character list", body = Vec<CharacterResponse>),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = [])),
    tag = "characters"
)]
pub async fn list_characters(
    State(state): State<Arc<AppState>>,
    request: Request,
) -> ApiResult<Json<Vec<CharacterResponse>>> {
    let claims = get_claims(&request).ok_or(ApiError::Unauthorized)?;

    let characters = sqlx::query_as::<_, CharacterRow>(
        "SELECT c.*, r.name as realm_name
         FROM characters c
         LEFT JOIN realms r ON c.realm_id = r.id
         WHERE c.account_id = $1 AND c.deletion_time IS NULL
         ORDER BY c.level DESC"
    )
    .bind(claims.account_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(characters.into_iter().map(Into::into).collect()))
}

/// Get character by ID
#[utoipa::path(
    get,
    path = "/api/v1/characters/{id}",
    params(
        ("id" = i32, Path, description = "Character ID")
    ),
    responses(
        (status = 200, description = "Character information", body = CharacterResponse),
        (status = 404, description = "Character not found")
    ),
    tag = "characters"
)]
pub async fn get_character(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> ApiResult<Json<CharacterResponse>> {
    let character = sqlx::query_as::<_, CharacterRow>(
        "SELECT c.*, r.name as realm_name
         FROM characters c
         LEFT JOIN realms r ON c.realm_id = r.id
         WHERE c.id = $1 AND c.deletion_time IS NULL"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound("Character not found".to_string()))?;

    Ok(Json(character.into()))
}

/// Create character request
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateCharacterRequest {
    pub name: String,
    /// Character gender (male/female)
    #[serde(alias = "sex")]
    pub gender: Gender,
    pub vocation: Vocation,
    pub realm_id: i32,
}

/// Create character
#[utoipa::path(
    post,
    path = "/api/v1/characters",
    request_body = CreateCharacterRequest,
    responses(
        (status = 201, description = "Character created", body = CharacterResponse),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Name already exists")
    ),
    security(("bearer_auth" = [])),
    tag = "characters"
)]
pub async fn create_character(
    State(state): State<Arc<AppState>>,
    request: Request,
    Json(body): Json<CreateCharacterRequest>,
) -> ApiResult<Json<CharacterResponse>> {
    let claims = get_claims(&request).ok_or(ApiError::Unauthorized)?;

    // Validate name
    crate::auth::validate_character_name(&body.name)?;

    // Check realm exists
    let realm_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM realms WHERE id = $1)"
    )
    .bind(body.realm_id)
    .fetch_one(&state.db)
    .await?;

    if !realm_exists {
        return Err(ApiError::NotFound("Realm not found".to_string()));
    }

    // Check character limit
    let char_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM characters WHERE account_id = $1 AND deletion_time IS NULL"
    )
    .bind(claims.account_id)
    .fetch_one(&state.db)
    .await?;

    if char_count >= state.config.max_characters_per_account as i64 {
        return Err(ApiError::BadRequest("Character limit reached".to_string()));
    }

    // Check name availability
    let name_taken = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM characters WHERE LOWER(name) = LOWER($1))"
    )
    .bind(&body.name)
    .fetch_one(&state.db)
    .await?;

    if name_taken {
        return Err(ApiError::Conflict("Name already taken".to_string()));
    }

    // Get starting town (from realm default or first town)
    let town_id = 1; // Default town

    // Get look type based on gender
    let look_type = body.gender.default_look_type();

    // Create character
    let id = sqlx::query_scalar::<_, i32>(
        "INSERT INTO characters (account_id, realm_id, name, sex, vocation, look_type, town_id)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING id"
    )
    .bind(claims.account_id)
    .bind(body.realm_id)
    .bind(&body.name)
    .bind(body.gender.to_protocol_value())
    .bind(body.vocation.to_i16())
    .bind(look_type)
    .bind(town_id)
    .fetch_one(&state.db)
    .await?;

    // Fetch created character
    get_character(State(state), Path(id)).await
}

/// Delete character
#[utoipa::path(
    delete,
    path = "/api/v1/characters/{id}",
    params(
        ("id" = i32, Path, description = "Character ID")
    ),
    responses(
        (status = 200, description = "Character scheduled for deletion"),
        (status = 404, description = "Character not found")
    ),
    security(("bearer_auth" = [])),
    tag = "characters"
)]
pub async fn delete_character(
    State(state): State<Arc<AppState>>,
    request: Request,
    Path(id): Path<i32>,
) -> ApiResult<Json<MessageResponse>> {
    let claims = get_claims(&request).ok_or(ApiError::Unauthorized)?;

    // Verify ownership
    let owns = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM characters WHERE id = $1 AND account_id = $2 AND deletion_time IS NULL)"
    )
    .bind(id)
    .bind(claims.account_id)
    .fetch_one(&state.db)
    .await?;

    if !owns {
        return Err(ApiError::NotFound("Character not found".to_string()));
    }

    // Schedule deletion
    let deletion_days = state.config.character_deletion_days as i32;
    sqlx::query(
        "UPDATE characters SET deletion_time = CURRENT_TIMESTAMP + INTERVAL '1 day' * $1 WHERE id = $2"
    )
    .bind(deletion_days)
    .bind(id)
    .execute(&state.db)
    .await?;

    Ok(Json(MessageResponse::new(format!("Character will be deleted in {} days", deletion_days))))
}

/// Get online status
pub async fn get_online_status(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> ApiResult<Json<OnlineStatusResponse>> {
    let online = sqlx::query_scalar::<_, bool>(
        "SELECT online FROM characters WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::NotFound("Character not found".to_string()))?;

    Ok(Json(OnlineStatusResponse::new(online)))
}

// Helper types

#[derive(sqlx::FromRow)]
struct CharacterRow {
    id: i32,
    name: String,
    sex: i16, // DB still uses "sex" column
    vocation: i16,
    level: i32,
    experience: i64,
    health: i32,
    max_health: i32,
    mana: i32,
    max_mana: i32,
    look_type: i32,
    look_head: i16,
    look_body: i16,
    look_legs: i16,
    look_feet: i16,
    look_addons: i16,
    town_id: i32,
    balance: i64,
    bank_balance: i64,
    online: bool,
    realm_id: i32,
    realm_name: Option<String>,
    last_login: Option<chrono::DateTime<chrono::Utc>>,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl From<CharacterRow> for CharacterResponse {
    fn from(row: CharacterRow) -> Self {
        // Convert DB sex (0=female, 1=male) to gender string
        let gender = Gender::from_protocol_value(row.sex)
            .unwrap_or_default()
            .to_string();

        CharacterResponse {
            id: row.id,
            name: row.name,
            gender,
            vocation: row.vocation,
            level: row.level,
            experience: row.experience,
            health: row.health,
            max_health: row.max_health,
            mana: row.mana,
            max_mana: row.max_mana,
            look_type: row.look_type,
            look_head: row.look_head,
            look_body: row.look_body,
            look_legs: row.look_legs,
            look_feet: row.look_feet,
            look_addons: row.look_addons,
            town_id: row.town_id,
            balance: row.balance,
            bank_balance: row.bank_balance,
            online: row.online,
            realm_id: row.realm_id,
            realm_name: row.realm_name,
            last_login: row.last_login.map(|t| t.to_rfc3339()),
            created_at: row.created_at.to_rfc3339(),
        }
    }
}
