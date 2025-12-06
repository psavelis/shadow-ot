//! Spell endpoints

use crate::state::AppState;
use crate::ApiResult;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use utoipa::ToSchema;

/// Spell element types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "spell_element", rename_all = "lowercase")]
pub enum SpellElement {
    Fire,
    Ice,
    Earth,
    Energy,
    Holy,
    Death,
    Physical,
    Healing,
}

/// Spell type categories
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "spell_type", rename_all = "lowercase")]
pub enum SpellType {
    Instant,
    Rune,
    Conjure,
    Support,
    Special,
}

/// Vocation enum for spells
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "spell_vocation", rename_all = "lowercase")]
pub enum SpellVocation {
    None,
    Knight,
    Paladin,
    Sorcerer,
    Druid,
    All,
}

/// Spell information
#[derive(Debug, Serialize, ToSchema)]
pub struct Spell {
    pub id: i32,
    pub name: String,
    pub words: String,
    pub description: String,
    pub spell_type: SpellType,
    pub element: SpellElement,
    pub vocations: Vec<String>,
    pub level_required: i32,
    pub mana_cost: i32,
    pub soul_cost: i32,
    pub cooldown: i32,
    pub group_cooldown: i32,
    pub premium: bool,
    pub damage_formula: Option<String>,
    pub healing_formula: Option<String>,
    pub area_effect: Option<String>,
    pub icon: Option<String>,
    pub animation_id: Option<i32>,
}

#[derive(Debug, FromRow)]
struct SpellRow {
    id: i32,
    name: String,
    words: String,
    description: String,
    spell_type: SpellType,
    element: SpellElement,
    level_required: i32,
    mana_cost: i32,
    soul_cost: i32,
    cooldown: i32,
    group_cooldown: i32,
    premium: bool,
    damage_formula: Option<String>,
    healing_formula: Option<String>,
    area_effect: Option<String>,
    icon: Option<String>,
    animation_id: Option<i32>,
}

/// Rune information
#[derive(Debug, Serialize, ToSchema)]
pub struct Rune {
    pub id: i32,
    pub name: String,
    pub spell_id: i32,
    pub level_required: i32,
    pub magic_level_required: i32,
    pub charges: i32,
    pub element: SpellElement,
    pub vocations: Vec<String>,
    pub premium: bool,
    pub description: String,
    pub icon: Option<String>,
}

#[derive(Debug, FromRow)]
struct RuneRow {
    id: i32,
    name: String,
    spell_id: i32,
    level_required: i32,
    magic_level_required: i32,
    charges: i32,
    element: SpellElement,
    premium: bool,
    description: String,
    icon: Option<String>,
}

/// Query parameters for spells
#[derive(Debug, Deserialize)]
pub struct SpellQuery {
    pub element: Option<String>,
    pub spell_type: Option<String>,
    pub vocation: Option<String>,
    pub premium: Option<bool>,
    pub search: Option<String>,
}

/// List all spells
#[utoipa::path(
    get,
    path = "/api/v1/spells",
    params(
        ("element" = Option<String>, Query, description = "Filter by element"),
        ("spell_type" = Option<String>, Query, description = "Filter by type"),
        ("vocation" = Option<String>, Query, description = "Filter by vocation"),
        ("premium" = Option<bool>, Query, description = "Filter by premium status"),
        ("search" = Option<String>, Query, description = "Search by name or words")
    ),
    responses(
        (status = 200, description = "Spells list", body = Vec<Spell>)
    ),
    tag = "spells"
)]
pub async fn list_spells(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SpellQuery>,
) -> ApiResult<Json<Vec<Spell>>> {
    let rows = sqlx::query_as::<_, SpellRow>(
        "SELECT id, name, words, description, spell_type, element, level_required,
                mana_cost, soul_cost, cooldown, group_cooldown, premium,
                damage_formula, healing_formula, area_effect, icon, animation_id
         FROM spells
         WHERE ($1::text IS NULL OR element::text = $1)
           AND ($2::text IS NULL OR spell_type::text = $2)
           AND ($3::bool IS NULL OR premium = $3)
           AND ($4::text IS NULL OR LOWER(name) LIKE LOWER('%' || $4 || '%') OR LOWER(words) LIKE LOWER('%' || $4 || '%'))
         ORDER BY level_required, name"
    )
    .bind(&query.element)
    .bind(&query.spell_type)
    .bind(query.premium)
    .bind(&query.search)
    .fetch_all(&state.db)
    .await?;

    let mut spells = Vec::new();
    for row in rows {
        let vocations = load_spell_vocations(&state, row.id).await?;
        
        // Apply vocation filter if specified
        if let Some(ref voc) = query.vocation {
            if !vocations.iter().any(|v| v.to_lowercase() == voc.to_lowercase()) && !vocations.contains(&"all".to_string()) {
                continue;
            }
        }

        spells.push(Spell {
            id: row.id,
            name: row.name,
            words: row.words,
            description: row.description,
            spell_type: row.spell_type,
            element: row.element,
            vocations,
            level_required: row.level_required,
            mana_cost: row.mana_cost,
            soul_cost: row.soul_cost,
            cooldown: row.cooldown,
            group_cooldown: row.group_cooldown,
            premium: row.premium,
            damage_formula: row.damage_formula,
            healing_formula: row.healing_formula,
            area_effect: row.area_effect,
            icon: row.icon,
            animation_id: row.animation_id,
        });
    }

    Ok(Json(spells))
}

/// Get spell by ID
#[utoipa::path(
    get,
    path = "/api/v1/spells/{id}",
    params(
        ("id" = i32, Path, description = "Spell ID")
    ),
    responses(
        (status = 200, description = "Spell details", body = Spell)
    ),
    tag = "spells"
)]
pub async fn get_spell(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> ApiResult<Json<Spell>> {
    let row = sqlx::query_as::<_, SpellRow>(
        "SELECT id, name, words, description, spell_type, element, level_required,
                mana_cost, soul_cost, cooldown, group_cooldown, premium,
                damage_formula, healing_formula, area_effect, icon, animation_id
         FROM spells WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(crate::error::ApiError::NotFound("Spell not found".to_string()))?;

    let vocations = load_spell_vocations(&state, row.id).await?;

    Ok(Json(Spell {
        id: row.id,
        name: row.name,
        words: row.words,
        description: row.description,
        spell_type: row.spell_type,
        element: row.element,
        vocations,
        level_required: row.level_required,
        mana_cost: row.mana_cost,
        soul_cost: row.soul_cost,
        cooldown: row.cooldown,
        group_cooldown: row.group_cooldown,
        premium: row.premium,
        damage_formula: row.damage_formula,
        healing_formula: row.healing_formula,
        area_effect: row.area_effect,
        icon: row.icon,
        animation_id: row.animation_id,
    }))
}

/// Get spell by magic words
#[utoipa::path(
    get,
    path = "/api/v1/spells/words/{words}",
    params(
        ("words" = String, Path, description = "Spell magic words")
    ),
    responses(
        (status = 200, description = "Spell details", body = Spell)
    ),
    tag = "spells"
)]
pub async fn get_spell_by_words(
    State(state): State<Arc<AppState>>,
    Path(words): Path<String>,
) -> ApiResult<Json<Spell>> {
    let row = sqlx::query_as::<_, SpellRow>(
        "SELECT id, name, words, description, spell_type, element, level_required,
                mana_cost, soul_cost, cooldown, group_cooldown, premium,
                damage_formula, healing_formula, area_effect, icon, animation_id
         FROM spells WHERE LOWER(words) = LOWER($1)"
    )
    .bind(&words)
    .fetch_optional(&state.db)
    .await?
    .ok_or(crate::error::ApiError::NotFound("Spell not found".to_string()))?;

    let vocations = load_spell_vocations(&state, row.id).await?;

    Ok(Json(Spell {
        id: row.id,
        name: row.name,
        words: row.words,
        description: row.description,
        spell_type: row.spell_type,
        element: row.element,
        vocations,
        level_required: row.level_required,
        mana_cost: row.mana_cost,
        soul_cost: row.soul_cost,
        cooldown: row.cooldown,
        group_cooldown: row.group_cooldown,
        premium: row.premium,
        damage_formula: row.damage_formula,
        healing_formula: row.healing_formula,
        area_effect: row.area_effect,
        icon: row.icon,
        animation_id: row.animation_id,
    }))
}

/// Get spells by vocation
#[utoipa::path(
    get,
    path = "/api/v1/spells/vocation/{vocation}",
    params(
        ("vocation" = String, Path, description = "Vocation name")
    ),
    responses(
        (status = 200, description = "Spells for vocation", body = Vec<Spell>)
    ),
    tag = "spells"
)]
pub async fn get_spells_by_vocation(
    State(state): State<Arc<AppState>>,
    Path(vocation): Path<String>,
) -> ApiResult<Json<Vec<Spell>>> {
    let rows = sqlx::query_as::<_, SpellRow>(
        "SELECT DISTINCT s.id, s.name, s.words, s.description, s.spell_type, s.element, s.level_required,
                s.mana_cost, s.soul_cost, s.cooldown, s.group_cooldown, s.premium,
                s.damage_formula, s.healing_formula, s.area_effect, s.icon, s.animation_id
         FROM spells s
         JOIN spell_vocations sv ON sv.spell_id = s.id
         WHERE LOWER(sv.vocation) = LOWER($1) OR LOWER(sv.vocation) = 'all'
         ORDER BY s.level_required, s.name"
    )
    .bind(&vocation)
    .fetch_all(&state.db)
    .await?;

    let mut spells = Vec::new();
    for row in rows {
        let vocations = load_spell_vocations(&state, row.id).await?;
        spells.push(Spell {
            id: row.id,
            name: row.name,
            words: row.words,
            description: row.description,
            spell_type: row.spell_type,
            element: row.element,
            vocations,
            level_required: row.level_required,
            mana_cost: row.mana_cost,
            soul_cost: row.soul_cost,
            cooldown: row.cooldown,
            group_cooldown: row.group_cooldown,
            premium: row.premium,
            damage_formula: row.damage_formula,
            healing_formula: row.healing_formula,
            area_effect: row.area_effect,
            icon: row.icon,
            animation_id: row.animation_id,
        });
    }

    Ok(Json(spells))
}

/// Get spells by element
#[utoipa::path(
    get,
    path = "/api/v1/spells/element/{element}",
    params(
        ("element" = String, Path, description = "Spell element")
    ),
    responses(
        (status = 200, description = "Spells with element", body = Vec<Spell>)
    ),
    tag = "spells"
)]
pub async fn get_spells_by_element(
    State(state): State<Arc<AppState>>,
    Path(element): Path<String>,
) -> ApiResult<Json<Vec<Spell>>> {
    let rows = sqlx::query_as::<_, SpellRow>(
        "SELECT id, name, words, description, spell_type, element, level_required,
                mana_cost, soul_cost, cooldown, group_cooldown, premium,
                damage_formula, healing_formula, area_effect, icon, animation_id
         FROM spells WHERE LOWER(element::text) = LOWER($1)
         ORDER BY level_required, name"
    )
    .bind(&element)
    .fetch_all(&state.db)
    .await?;

    let mut spells = Vec::new();
    for row in rows {
        let vocations = load_spell_vocations(&state, row.id).await?;
        spells.push(Spell {
            id: row.id,
            name: row.name,
            words: row.words,
            description: row.description,
            spell_type: row.spell_type,
            element: row.element,
            vocations,
            level_required: row.level_required,
            mana_cost: row.mana_cost,
            soul_cost: row.soul_cost,
            cooldown: row.cooldown,
            group_cooldown: row.group_cooldown,
            premium: row.premium,
            damage_formula: row.damage_formula,
            healing_formula: row.healing_formula,
            area_effect: row.area_effect,
            icon: row.icon,
            animation_id: row.animation_id,
        });
    }

    Ok(Json(spells))
}

/// Get all runes
#[utoipa::path(
    get,
    path = "/api/v1/spells/runes",
    responses(
        (status = 200, description = "Runes list", body = Vec<Rune>)
    ),
    tag = "spells"
)]
pub async fn get_runes(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<Vec<Rune>>> {
    let rows = sqlx::query_as::<_, RuneRow>(
        "SELECT id, name, spell_id, level_required, magic_level_required, charges,
                element, premium, description, icon
         FROM runes ORDER BY level_required, name"
    )
    .fetch_all(&state.db)
    .await?;

    let mut runes = Vec::new();
    for row in rows {
        let vocations = load_rune_vocations(&state, row.id).await?;
        runes.push(Rune {
            id: row.id,
            name: row.name,
            spell_id: row.spell_id,
            level_required: row.level_required,
            magic_level_required: row.magic_level_required,
            charges: row.charges,
            element: row.element,
            vocations,
            premium: row.premium,
            description: row.description,
            icon: row.icon,
        });
    }

    Ok(Json(runes))
}

/// Helper to load spell vocations
async fn load_spell_vocations(state: &AppState, spell_id: i32) -> Result<Vec<String>, sqlx::Error> {
    let rows: Vec<(String,)> = sqlx::query_as(
        "SELECT vocation FROM spell_vocations WHERE spell_id = $1"
    )
    .bind(spell_id)
    .fetch_all(&state.db)
    .await?;

    Ok(rows.into_iter().map(|(v,)| v).collect())
}

/// Helper to load rune vocations
async fn load_rune_vocations(state: &AppState, rune_id: i32) -> Result<Vec<String>, sqlx::Error> {
    let rows: Vec<(String,)> = sqlx::query_as(
        "SELECT vocation FROM rune_vocations WHERE rune_id = $1"
    )
    .bind(rune_id)
    .fetch_all(&state.db)
    .await?;

    Ok(rows.into_iter().map(|(v,)| v).collect())
}
