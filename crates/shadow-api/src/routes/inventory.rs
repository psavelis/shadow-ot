//! Inventory management endpoints

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

/// Inventory item
#[derive(Debug, Serialize, ToSchema)]
pub struct InventoryItem {
    pub id: Uuid,
    pub item_id: i32,
    pub name: String,
    pub description: String,
    pub count: i32,
    pub slot: i32,
    pub container_id: Option<Uuid>,
    pub attributes: ItemAttributes,
    pub tradeable: bool,
    pub droppable: bool,
    pub stackable: bool,
    pub weight: f32,
    pub acquired_at: DateTime<Utc>,
}

/// Item attributes
#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
pub struct ItemAttributes {
    pub attack: Option<i32>,
    pub defense: Option<i32>,
    pub armor: Option<i32>,
    pub charges: Option<i32>,
    pub duration: Option<i32>,
    pub imbuements: Vec<Imbuement>,
}

/// Imbuement on item
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Imbuement {
    pub id: i32,
    pub name: String,
    pub tier: i32,
    pub remaining_hours: f32,
}

#[derive(Debug, FromRow)]
struct InventoryItemRow {
    id: Uuid,
    item_id: i32,
    name: String,
    description: String,
    count: i32,
    slot: i32,
    container_id: Option<Uuid>,
    tradeable: bool,
    droppable: bool,
    stackable: bool,
    weight: f32,
    attack: Option<i32>,
    defense: Option<i32>,
    armor: Option<i32>,
    charges: Option<i32>,
    duration: Option<i32>,
    acquired_at: DateTime<Utc>,
}

/// Query parameters
#[derive(Debug, Deserialize)]
pub struct InventoryQuery {
    pub character_id: Option<Uuid>,
    pub slot: Option<i32>,
    pub container_id: Option<Uuid>,
}

/// Transfer request
#[derive(Debug, Deserialize, ToSchema)]
pub struct TransferRequest {
    pub to_character_id: Uuid,
    pub count: Option<i32>,
}

/// Transfer response
#[derive(Debug, Serialize, ToSchema)]
pub struct TransferResponse {
    pub success: bool,
    pub message: String,
    pub transferred_count: i32,
}

/// List on market request
#[derive(Debug, Deserialize, ToSchema)]
pub struct ListOnMarketRequest {
    pub price: i64,
    pub count: Option<i32>,
    pub anonymous: Option<bool>,
}

/// List on market response
#[derive(Debug, Serialize, ToSchema)]
pub struct ListOnMarketResponse {
    pub success: bool,
    pub offer_id: Uuid,
    pub message: String,
}

/// Get inventory items
#[utoipa::path(
    get,
    path = "/api/v1/inventory",
    params(
        ("character_id" = Option<Uuid>, Query, description = "Character ID"),
        ("slot" = Option<i32>, Query, description = "Filter by slot"),
        ("container_id" = Option<Uuid>, Query, description = "Filter by container")
    ),
    responses(
        (status = 200, description = "Inventory items", body = Vec<InventoryItem>)
    ),
    security(("bearer_auth" = [])),
    tag = "inventory"
)]
pub async fn get_inventory_items(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
    Query(query): Query<InventoryQuery>,
) -> ApiResult<Json<Vec<InventoryItem>>> {
    // Get character ID, either from query or first character of account
    let character_id = match query.character_id {
        Some(id) => id,
        None => {
            let row: Option<(Uuid,)> = sqlx::query_as(
                "SELECT uuid FROM characters WHERE account_id = $1 ORDER BY last_login DESC LIMIT 1"
            )
            .bind(&claims.sub)
            .fetch_optional(&state.db)
            .await?;
            row.ok_or(crate::error::ApiError::NotFound("No characters found".to_string()))?.0
        }
    };

    // Verify ownership
    let owns: Option<(i32,)> = sqlx::query_as(
        "SELECT 1 FROM characters WHERE uuid = $1 AND account_id = $2"
    )
    .bind(character_id)
    .bind(&claims.sub)
    .fetch_optional(&state.db)
    .await?;

    if owns.is_none() {
        return Err(crate::error::ApiError::Forbidden("Not your character".to_string()));
    }

    let rows = sqlx::query_as::<_, InventoryItemRow>(
        "SELECT i.id, i.item_id, it.name, COALESCE(it.description, '') as description, 
                i.count, i.slot, i.container_id, it.tradeable, it.droppable, it.stackable,
                COALESCE(it.weight, 0) as weight,
                i.attack, i.defense, i.armor, i.charges, i.duration,
                i.acquired_at
         FROM character_inventory i
         JOIN items it ON it.id = i.item_id
         WHERE i.character_id = $1
           AND ($2::int IS NULL OR i.slot = $2)
           AND ($3::uuid IS NULL OR i.container_id = $3)
         ORDER BY i.slot, i.id"
    )
    .bind(character_id)
    .bind(query.slot)
    .bind(query.container_id)
    .fetch_all(&state.db)
    .await?;

    let mut items = Vec::new();
    for row in rows {
        let imbuements = load_imbuements(&state, row.id).await?;

        items.push(InventoryItem {
            id: row.id,
            item_id: row.item_id,
            name: row.name,
            description: row.description,
            count: row.count,
            slot: row.slot,
            container_id: row.container_id,
            attributes: ItemAttributes {
                attack: row.attack,
                defense: row.defense,
                armor: row.armor,
                charges: row.charges,
                duration: row.duration,
                imbuements,
            },
            tradeable: row.tradeable,
            droppable: row.droppable,
            stackable: row.stackable,
            weight: row.weight,
            acquired_at: row.acquired_at,
        });
    }

    Ok(Json(items))
}

/// Get specific inventory item
#[utoipa::path(
    get,
    path = "/api/v1/inventory/{id}",
    params(
        ("id" = Uuid, Path, description = "Inventory item ID")
    ),
    responses(
        (status = 200, description = "Inventory item details", body = InventoryItem)
    ),
    security(("bearer_auth" = [])),
    tag = "inventory"
)]
pub async fn get_inventory_item(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<InventoryItem>> {
    let row = sqlx::query_as::<_, InventoryItemRow>(
        "SELECT i.id, i.item_id, it.name, COALESCE(it.description, '') as description,
                i.count, i.slot, i.container_id, it.tradeable, it.droppable, it.stackable,
                COALESCE(it.weight, 0) as weight,
                i.attack, i.defense, i.armor, i.charges, i.duration,
                i.acquired_at
         FROM character_inventory i
         JOIN items it ON it.id = i.item_id
         JOIN characters c ON c.uuid = i.character_id
         WHERE i.id = $1 AND c.account_id = $2"
    )
    .bind(id)
    .bind(&claims.sub)
    .fetch_optional(&state.db)
    .await?
    .ok_or(crate::error::ApiError::NotFound("Item not found".to_string()))?;

    let imbuements = load_imbuements(&state, row.id).await?;

    Ok(Json(InventoryItem {
        id: row.id,
        item_id: row.item_id,
        name: row.name,
        description: row.description,
        count: row.count,
        slot: row.slot,
        container_id: row.container_id,
        attributes: ItemAttributes {
            attack: row.attack,
            defense: row.defense,
            armor: row.armor,
            charges: row.charges,
            duration: row.duration,
            imbuements,
        },
        tradeable: row.tradeable,
        droppable: row.droppable,
        stackable: row.stackable,
        weight: row.weight,
        acquired_at: row.acquired_at,
    }))
}

/// Transfer item to another character
#[utoipa::path(
    post,
    path = "/api/v1/inventory/{id}/transfer",
    params(
        ("id" = Uuid, Path, description = "Inventory item ID")
    ),
    request_body = TransferRequest,
    responses(
        (status = 200, description = "Transfer completed", body = TransferResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "inventory"
)]
pub async fn transfer_item(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
    Path(id): Path<Uuid>,
    Json(request): Json<TransferRequest>,
) -> ApiResult<Json<TransferResponse>> {
    // Verify ownership and get item details
    let item: Option<(Uuid, i32, i32, bool)> = sqlx::query_as(
        "SELECT i.character_id, i.item_id, i.count, it.tradeable
         FROM character_inventory i
         JOIN items it ON it.id = i.item_id
         JOIN characters c ON c.uuid = i.character_id
         WHERE i.id = $1 AND c.account_id = $2"
    )
    .bind(id)
    .bind(&claims.sub)
    .fetch_optional(&state.db)
    .await?;

    let (from_char_id, item_id, available_count, tradeable) = item
        .ok_or(crate::error::ApiError::NotFound("Item not found".to_string()))?;

    if !tradeable {
        return Err(crate::error::ApiError::BadRequest("Item is not tradeable".to_string()));
    }

    // Verify target character exists and is different
    let target_exists: Option<(Uuid,)> = sqlx::query_as(
        "SELECT uuid FROM characters WHERE uuid = $1"
    )
    .bind(request.to_character_id)
    .fetch_optional(&state.db)
    .await?;

    if target_exists.is_none() {
        return Err(crate::error::ApiError::NotFound("Target character not found".to_string()));
    }

    if request.to_character_id == from_char_id {
        return Err(crate::error::ApiError::BadRequest("Cannot transfer to same character".to_string()));
    }

    let transfer_count = request.count.unwrap_or(available_count).min(available_count);

    // Begin transaction
    let mut tx = state.db.begin().await?;

    // Reduce or remove from source
    if transfer_count >= available_count {
        sqlx::query("DELETE FROM character_inventory WHERE id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await?;
    } else {
        sqlx::query("UPDATE character_inventory SET count = count - $2 WHERE id = $1")
            .bind(id)
            .bind(transfer_count)
            .execute(&mut *tx)
            .await?;
    }

    // Add to target (or stack if exists)
    sqlx::query(
        "INSERT INTO character_inventory (id, character_id, item_id, count, slot, acquired_at)
         VALUES (gen_random_uuid(), $1, $2, $3, 
                 (SELECT COALESCE(MAX(slot), 0) + 1 FROM character_inventory WHERE character_id = $1),
                 CURRENT_TIMESTAMP)
         ON CONFLICT (character_id, item_id, slot) DO UPDATE SET count = character_inventory.count + $3"
    )
    .bind(request.to_character_id)
    .bind(item_id)
    .bind(transfer_count)
    .execute(&mut *tx)
    .await?;

    // Log transfer
    sqlx::query(
        "INSERT INTO item_transfers (item_id, from_character_id, to_character_id, count, transferred_at)
         VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)"
    )
    .bind(item_id)
    .bind(from_char_id)
    .bind(request.to_character_id)
    .bind(transfer_count)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(Json(TransferResponse {
        success: true,
        message: format!("Transferred {} item(s) successfully", transfer_count),
        transferred_count: transfer_count,
    }))
}

/// List item on market
#[utoipa::path(
    post,
    path = "/api/v1/inventory/{id}/list-on-market",
    params(
        ("id" = Uuid, Path, description = "Inventory item ID")
    ),
    request_body = ListOnMarketRequest,
    responses(
        (status = 200, description = "Listed on market", body = ListOnMarketResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "inventory"
)]
pub async fn list_on_market(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
    Path(id): Path<Uuid>,
    Json(request): Json<ListOnMarketRequest>,
) -> ApiResult<Json<ListOnMarketResponse>> {
    // Verify ownership and get item details
    let item: Option<(Uuid, i32, String, i32, bool)> = sqlx::query_as(
        "SELECT i.character_id, i.item_id, it.name, i.count, it.tradeable
         FROM character_inventory i
         JOIN items it ON it.id = i.item_id
         JOIN characters c ON c.uuid = i.character_id
         WHERE i.id = $1 AND c.account_id = $2"
    )
    .bind(id)
    .bind(&claims.sub)
    .fetch_optional(&state.db)
    .await?;

    let (char_id, item_id, item_name, available_count, tradeable) = item
        .ok_or(crate::error::ApiError::NotFound("Item not found".to_string()))?;

    if !tradeable {
        return Err(crate::error::ApiError::BadRequest("Item is not tradeable".to_string()));
    }

    let list_count = request.count.unwrap_or(available_count).min(available_count);

    // Begin transaction
    let mut tx = state.db.begin().await?;

    // Remove from inventory
    if list_count >= available_count {
        sqlx::query("DELETE FROM character_inventory WHERE id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await?;
    } else {
        sqlx::query("UPDATE character_inventory SET count = count - $2 WHERE id = $1")
            .bind(id)
            .bind(list_count)
            .execute(&mut *tx)
            .await?;
    }

    // Create market offer
    let offer_id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO market_offers (id, seller_id, item_id, count, price, anonymous, listed_at, expires_at, status)
         VALUES ($1, $2, $3, $4, $5, $6, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP + INTERVAL '7 days', 'active')"
    )
    .bind(offer_id)
    .bind(char_id)
    .bind(item_id)
    .bind(list_count)
    .bind(request.price)
    .bind(request.anonymous.unwrap_or(false))
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(Json(ListOnMarketResponse {
        success: true,
        offer_id,
        message: format!("Listed {} x{} for {} gold", item_name, list_count, request.price),
    }))
}

/// Helper to load item imbuements
async fn load_imbuements(state: &AppState, inventory_id: Uuid) -> Result<Vec<Imbuement>, sqlx::Error> {
    let rows: Vec<(i32, String, i32, f32)> = sqlx::query_as(
        "SELECT imb.id, imb.name, ii.tier, ii.remaining_hours
         FROM inventory_imbuements ii
         JOIN imbuements imb ON imb.id = ii.imbuement_id
         WHERE ii.inventory_id = $1"
    )
    .bind(inventory_id)
    .fetch_all(&state.db)
    .await?;

    Ok(rows.into_iter().map(|(id, name, tier, remaining_hours)| Imbuement {
        id,
        name,
        tier,
        remaining_hours,
    }).collect())
}
