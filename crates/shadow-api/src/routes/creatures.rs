//! Creature and bestiary endpoints

use crate::auth::JwtClaims;
use crate::state::AppState;
use crate::ApiResult;
use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

/// Creature difficulty
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "creature_difficulty", rename_all = "lowercase")]
pub enum CreatureDifficulty {
    Harmless,
    Trivial,
    Easy,
    Medium,
    Hard,
    Challenging,
}

/// Creature information
#[derive(Debug, Serialize, ToSchema)]
pub struct Creature {
    pub id: i32,
    pub name: String,
    pub race: String,
    pub description: String,
    pub experience: i32,
    pub health: i32,
    pub speed: i32,
    pub armor: i32,
    pub difficulty: CreatureDifficulty,
    pub is_boss: bool,
    pub sprite_id: i32,
    pub immunities: Vec<String>,
    pub weaknesses: Vec<String>,
    pub loot: Vec<LootItem>,
    pub abilities: Vec<String>,
    pub locations: Vec<String>,
    pub charm_points: i32,
    pub bestiary_class: String,
    pub bestiary_occurrence: String,
}

#[derive(Debug, FromRow)]
struct CreatureRow {
    id: i32,
    name: String,
    race: String,
    description: String,
    experience: i32,
    health: i32,
    speed: i32,
    armor: i32,
    difficulty: CreatureDifficulty,
    is_boss: bool,
    sprite_id: i32,
    immunities: sqlx::types::Json<Vec<String>>,
    weaknesses: sqlx::types::Json<Vec<String>>,
    abilities: sqlx::types::Json<Vec<String>>,
    locations: sqlx::types::Json<Vec<String>>,
    charm_points: i32,
    bestiary_class: String,
    bestiary_occurrence: String,
}

/// Loot item
#[derive(Debug, Serialize, ToSchema)]
pub struct LootItem {
    pub item_id: i32,
    pub item_name: String,
    pub chance: f32,
    pub max_count: i32,
}

#[derive(Debug, FromRow)]
struct LootItemRow {
    item_id: i32,
    item_name: String,
    chance: f32,
    max_count: i32,
}

/// Bestiary entry
#[derive(Debug, Serialize, ToSchema)]
pub struct BestiaryEntry {
    pub creature: Creature,
    pub kills: i32,
    pub stage: i32,
    pub completed: bool,
    pub unlocked_loot: bool,
    pub unlocked_charm: bool,
}

/// Paginated creatures response
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedCreatures {
    pub data: Vec<Creature>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

/// Creature query parameters
#[derive(Debug, Deserialize)]
pub struct CreatureQuery {
    pub race: Option<String>,
    pub difficulty: Option<String>,
    pub search: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

/// List all creatures
#[utoipa::path(
    get,
    path = "/api/v1/creatures",
    params(
        ("race" = Option<String>, Query, description = "Filter by race"),
        ("difficulty" = Option<String>, Query, description = "Filter by difficulty"),
        ("search" = Option<String>, Query, description = "Search by name"),
        ("page" = Option<u32>, Query, description = "Page number"),
        ("page_size" = Option<u32>, Query, description = "Results per page")
    ),
    responses(
        (status = 200, description = "Creatures list", body = PaginatedCreatures)
    ),
    tag = "creatures"
)]
pub async fn list_creatures(
    State(state): State<Arc<AppState>>,
    Query(query): Query<CreatureQuery>,
) -> ApiResult<Json<PaginatedCreatures>> {
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(50).min(200);
    let offset = (page - 1) * page_size;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM creatures
         WHERE ($1::text IS NULL OR race = $1)
           AND ($2::text IS NULL OR difficulty::text = $2)
           AND ($3::text IS NULL OR name ILIKE '%' || $3 || '%')"
    )
    .bind(&query.race)
    .bind(&query.difficulty)
    .bind(&query.search)
    .fetch_one(&state.db)
    .await?;

    let rows = sqlx::query_as::<_, CreatureRow>(
        "SELECT id, name, race, description, experience, health, speed, armor,
                difficulty, is_boss, sprite_id, immunities, weaknesses, abilities,
                locations, charm_points, bestiary_class, bestiary_occurrence
         FROM creatures
         WHERE ($1::text IS NULL OR race = $1)
           AND ($2::text IS NULL OR difficulty::text = $2)
           AND ($3::text IS NULL OR name ILIKE '%' || $3 || '%')
         ORDER BY name ASC
         LIMIT $4 OFFSET $5"
    )
    .bind(&query.race)
    .bind(&query.difficulty)
    .bind(&query.search)
    .bind(page_size as i64)
    .bind(offset as i64)
    .fetch_all(&state.db)
    .await?;

    let mut creatures = Vec::new();
    for row in rows {
        let loot = load_creature_loot(&state, row.id).await?;
        creatures.push(Creature {
            id: row.id,
            name: row.name,
            race: row.race,
            description: row.description,
            experience: row.experience,
            health: row.health,
            speed: row.speed,
            armor: row.armor,
            difficulty: row.difficulty,
            is_boss: row.is_boss,
            sprite_id: row.sprite_id,
            immunities: row.immunities.0,
            weaknesses: row.weaknesses.0,
            loot,
            abilities: row.abilities.0,
            locations: row.locations.0,
            charm_points: row.charm_points,
            bestiary_class: row.bestiary_class,
            bestiary_occurrence: row.bestiary_occurrence,
        });
    }

    Ok(Json(PaginatedCreatures {
        data: creatures,
        total: total.0,
        page,
        page_size,
        total_pages: ((total.0 as f64) / (page_size as f64)).ceil() as u32,
    }))
}

/// Get creature by ID
#[utoipa::path(
    get,
    path = "/api/v1/creatures/{id}",
    params(
        ("id" = i32, Path, description = "Creature ID")
    ),
    responses(
        (status = 200, description = "Creature details", body = Creature)
    ),
    tag = "creatures"
)]
pub async fn get_creature(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> ApiResult<Json<Creature>> {
    let row = sqlx::query_as::<_, CreatureRow>(
        "SELECT id, name, race, description, experience, health, speed, armor,
                difficulty, is_boss, sprite_id, immunities, weaknesses, abilities,
                locations, charm_points, bestiary_class, bestiary_occurrence
         FROM creatures
         WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(crate::error::ApiError::NotFound("Creature not found".to_string()))?;

    let loot = load_creature_loot(&state, row.id).await?;

    Ok(Json(Creature {
        id: row.id,
        name: row.name,
        race: row.race,
        description: row.description,
        experience: row.experience,
        health: row.health,
        speed: row.speed,
        armor: row.armor,
        difficulty: row.difficulty,
        is_boss: row.is_boss,
        sprite_id: row.sprite_id,
        immunities: row.immunities.0,
        weaknesses: row.weaknesses.0,
        loot,
        abilities: row.abilities.0,
        locations: row.locations.0,
        charm_points: row.charm_points,
        bestiary_class: row.bestiary_class,
        bestiary_occurrence: row.bestiary_occurrence,
    }))
}

/// Get creature by name
#[utoipa::path(
    get,
    path = "/api/v1/creatures/name/{name}",
    params(
        ("name" = String, Path, description = "Creature name")
    ),
    responses(
        (status = 200, description = "Creature details", body = Creature)
    ),
    tag = "creatures"
)]
pub async fn get_creature_by_name(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> ApiResult<Json<Creature>> {
    let row = sqlx::query_as::<_, CreatureRow>(
        "SELECT id, name, race, description, experience, health, speed, armor,
                difficulty, is_boss, sprite_id, immunities, weaknesses, abilities,
                locations, charm_points, bestiary_class, bestiary_occurrence
         FROM creatures
         WHERE LOWER(name) = LOWER($1)"
    )
    .bind(&name)
    .fetch_optional(&state.db)
    .await?
    .ok_or(crate::error::ApiError::NotFound("Creature not found".to_string()))?;

    let loot = load_creature_loot(&state, row.id).await?;

    Ok(Json(Creature {
        id: row.id,
        name: row.name,
        race: row.race,
        description: row.description,
        experience: row.experience,
        health: row.health,
        speed: row.speed,
        armor: row.armor,
        difficulty: row.difficulty,
        is_boss: row.is_boss,
        sprite_id: row.sprite_id,
        immunities: row.immunities.0,
        weaknesses: row.weaknesses.0,
        loot,
        abilities: row.abilities.0,
        locations: row.locations.0,
        charm_points: row.charm_points,
        bestiary_class: row.bestiary_class,
        bestiary_occurrence: row.bestiary_occurrence,
    }))
}

/// Get character's bestiary progress
#[utoipa::path(
    get,
    path = "/api/v1/characters/{character_id}/bestiary",
    params(
        ("character_id" = Uuid, Path, description = "Character ID")
    ),
    responses(
        (status = 200, description = "Bestiary progress", body = Vec<BestiaryEntry>)
    ),
    security(("bearer_auth" = [])),
    tag = "creatures"
)]
pub async fn get_bestiary_progress(
    State(state): State<Arc<AppState>>,
    Extension(_claims): Extension<JwtClaims>,
    Path(character_id): Path<Uuid>,
) -> ApiResult<Json<Vec<BestiaryEntry>>> {
    let rows = sqlx::query_as::<_, (i32, i32, bool, bool, bool)>(
        "SELECT creature_id, kills, completed, unlocked_loot, unlocked_charm
         FROM bestiary_progress
         WHERE character_id = (SELECT id FROM characters WHERE uuid = $1)
         ORDER BY creature_id"
    )
    .bind(character_id)
    .fetch_all(&state.db)
    .await?;

    let mut entries = Vec::new();
    for (creature_id, kills, completed, unlocked_loot, unlocked_charm) in rows {
        let creature_row = sqlx::query_as::<_, CreatureRow>(
            "SELECT id, name, race, description, experience, health, speed, armor,
                    difficulty, is_boss, sprite_id, immunities, weaknesses, abilities,
                    locations, charm_points, bestiary_class, bestiary_occurrence
             FROM creatures WHERE id = $1"
        )
        .bind(creature_id)
        .fetch_optional(&state.db)
        .await?;

        if let Some(row) = creature_row {
            let loot = load_creature_loot(&state, row.id).await?;
            let stage = calculate_bestiary_stage(kills, &row.bestiary_occurrence);

            entries.push(BestiaryEntry {
                creature: Creature {
                    id: row.id,
                    name: row.name,
                    race: row.race,
                    description: row.description,
                    experience: row.experience,
                    health: row.health,
                    speed: row.speed,
                    armor: row.armor,
                    difficulty: row.difficulty,
                    is_boss: row.is_boss,
                    sprite_id: row.sprite_id,
                    immunities: row.immunities.0,
                    weaknesses: row.weaknesses.0,
                    loot,
                    abilities: row.abilities.0,
                    locations: row.locations.0,
                    charm_points: row.charm_points,
                    bestiary_class: row.bestiary_class,
                    bestiary_occurrence: row.bestiary_occurrence,
                },
                kills,
                stage,
                completed,
                unlocked_loot,
                unlocked_charm,
            });
        }
    }

    Ok(Json(entries))
}

/// Get bestiary entry for specific creature
#[utoipa::path(
    get,
    path = "/api/v1/characters/{character_id}/bestiary/{creature_id}",
    params(
        ("character_id" = Uuid, Path, description = "Character ID"),
        ("creature_id" = i32, Path, description = "Creature ID")
    ),
    responses(
        (status = 200, description = "Bestiary entry", body = BestiaryEntry)
    ),
    security(("bearer_auth" = [])),
    tag = "creatures"
)]
pub async fn get_bestiary_entry(
    State(state): State<Arc<AppState>>,
    Extension(_claims): Extension<JwtClaims>,
    Path((character_id, creature_id)): Path<(Uuid, i32)>,
) -> ApiResult<Json<BestiaryEntry>> {
    let progress = sqlx::query_as::<_, (i32, bool, bool, bool)>(
        "SELECT kills, completed, unlocked_loot, unlocked_charm
         FROM bestiary_progress
         WHERE character_id = (SELECT id FROM characters WHERE uuid = $1)
           AND creature_id = $2"
    )
    .bind(character_id)
    .bind(creature_id)
    .fetch_optional(&state.db)
    .await?;

    let (kills, completed, unlocked_loot, unlocked_charm) = progress.unwrap_or((0, false, false, false));

    let creature_row = sqlx::query_as::<_, CreatureRow>(
        "SELECT id, name, race, description, experience, health, speed, armor,
                difficulty, is_boss, sprite_id, immunities, weaknesses, abilities,
                locations, charm_points, bestiary_class, bestiary_occurrence
         FROM creatures WHERE id = $1"
    )
    .bind(creature_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(crate::error::ApiError::NotFound("Creature not found".to_string()))?;

    let loot = load_creature_loot(&state, creature_row.id).await?;
    let stage = calculate_bestiary_stage(kills, &creature_row.bestiary_occurrence);

    Ok(Json(BestiaryEntry {
        creature: Creature {
            id: creature_row.id,
            name: creature_row.name,
            race: creature_row.race,
            description: creature_row.description,
            experience: creature_row.experience,
            health: creature_row.health,
            speed: creature_row.speed,
            armor: creature_row.armor,
            difficulty: creature_row.difficulty,
            is_boss: creature_row.is_boss,
            sprite_id: creature_row.sprite_id,
            immunities: creature_row.immunities.0,
            weaknesses: creature_row.weaknesses.0,
            loot,
            abilities: creature_row.abilities.0,
            locations: creature_row.locations.0,
            charm_points: creature_row.charm_points,
            bestiary_class: creature_row.bestiary_class,
            bestiary_occurrence: creature_row.bestiary_occurrence,
        },
        kills,
        stage,
        completed,
        unlocked_loot,
        unlocked_charm,
    }))
}

/// Helper to load creature loot
async fn load_creature_loot(state: &AppState, creature_id: i32) -> Result<Vec<LootItem>, sqlx::Error> {
    let rows = sqlx::query_as::<_, LootItemRow>(
        "SELECT cl.item_id, i.name as item_name, cl.chance, cl.max_count
         FROM creature_loot cl
         JOIN items i ON cl.item_id = i.id
         WHERE cl.creature_id = $1
         ORDER BY cl.chance DESC"
    )
    .bind(creature_id)
    .fetch_all(&state.db)
    .await?;

    Ok(rows.into_iter().map(|r| LootItem {
        item_id: r.item_id,
        item_name: r.item_name,
        chance: r.chance,
        max_count: r.max_count,
    }).collect())
}

/// Calculate bestiary stage based on kills
fn calculate_bestiary_stage(kills: i32, occurrence: &str) -> i32 {
    let thresholds = match occurrence.to_lowercase().as_str() {
        "common" => [5, 250, 500, 1000],
        "uncommon" => [5, 100, 250, 500],
        "rare" => [1, 25, 50, 100],
        "very rare" => [1, 5, 10, 25],
        _ => [5, 250, 500, 1000],
    };

    if kills >= thresholds[3] { 4 }
    else if kills >= thresholds[2] { 3 }
    else if kills >= thresholds[1] { 2 }
    else if kills >= thresholds[0] { 1 }
    else { 0 }
}
