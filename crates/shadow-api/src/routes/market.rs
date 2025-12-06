//! Market endpoints

use crate::state::AppState;
use crate::ApiResult;
use axum::{extract::{Path, Query, State}, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;

/// Market offer
#[derive(Debug, Serialize, ToSchema)]
pub struct MarketOffer {
    pub id: i32,
    pub item_type: i32,
    pub amount: i32,
    pub price: i64,
    pub offer_type: String,
    pub character_name: Option<String>,
    pub expires_at: String,
}

/// Market query
#[derive(Debug, Deserialize)]
pub struct MarketQuery {
    pub realm_id: Option<i32>,
    pub item_type: Option<i32>,
    pub offer_type: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

/// List market offers
#[utoipa::path(
    get,
    path = "/api/v1/market/offers",
    params(
        ("realm_id" = Option<i32>, Query, description = "Filter by realm"),
        ("item_type" = Option<i32>, Query, description = "Filter by item type"),
        ("offer_type" = Option<String>, Query, description = "Filter by offer type (buy/sell)"),
        ("page" = Option<u32>, Query, description = "Page number"),
        ("limit" = Option<u32>, Query, description = "Results per page")
    ),
    responses(
        (status = 200, description = "Market offers", body = Vec<MarketOffer>)
    ),
    tag = "market"
)]
pub async fn list_offers(
    State(state): State<Arc<AppState>>,
    Query(query): Query<MarketQuery>,
) -> ApiResult<Json<Vec<MarketOffer>>> {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(50).min(100);
    let offset = (page - 1) * limit;

    let offers = sqlx::query_as::<_, MarketOfferRow>(
        "SELECT mo.id, mo.item_type, mo.amount, mo.price, mo.offer_type, mo.expires_at,
                CASE WHEN mo.anonymous THEN NULL ELSE c.name END as character_name
         FROM market_offers mo
         LEFT JOIN characters c ON mo.character_id = c.id
         WHERE mo.expires_at > CURRENT_TIMESTAMP
           AND ($1::int IS NULL OR mo.realm_id = $1)
           AND ($2::int IS NULL OR mo.item_type = $2)
           AND ($3::text IS NULL OR mo.offer_type = $3)
         ORDER BY mo.created_at DESC
         LIMIT $4 OFFSET $5"
    )
    .bind(query.realm_id)
    .bind(query.item_type)
    .bind(&query.offer_type)
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(offers.into_iter().map(|o| MarketOffer {
        id: o.id,
        item_type: o.item_type,
        amount: o.amount,
        price: o.price,
        offer_type: o.offer_type,
        character_name: o.character_name,
        expires_at: o.expires_at.to_rfc3339(),
    }).collect()))
}

/// Get offer by ID
pub async fn get_offer(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> ApiResult<Json<MarketOffer>> {
    let offer = sqlx::query_as::<_, MarketOfferRow>(
        "SELECT mo.id, mo.item_type, mo.amount, mo.price, mo.offer_type, mo.expires_at,
                CASE WHEN mo.anonymous THEN NULL ELSE c.name END as character_name
         FROM market_offers mo
         LEFT JOIN characters c ON mo.character_id = c.id
         WHERE mo.id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(crate::error::ApiError::NotFound("Offer not found".to_string()))?;

    Ok(Json(MarketOffer {
        id: offer.id,
        item_type: offer.item_type,
        amount: offer.amount,
        price: offer.price,
        offer_type: offer.offer_type,
        character_name: offer.character_name,
        expires_at: offer.expires_at.to_rfc3339(),
    }))
}

/// Market history entry
#[derive(Debug, Serialize)]
pub struct MarketHistory {
    pub item_type: i32,
    pub amount: i32,
    pub price: i64,
    pub buyer_name: Option<String>,
    pub seller_name: Option<String>,
    pub completed_at: String,
}

/// Get market history
pub async fn get_history(
    State(state): State<Arc<AppState>>,
    Query(query): Query<MarketQuery>,
) -> ApiResult<Json<Vec<MarketHistory>>> {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(50).min(100);
    let offset = (page - 1) * limit;

    let history = sqlx::query_as::<_, MarketHistoryRow>(
        "SELECT mh.item_type, mh.amount, mh.price, mh.completed_at,
                b.name as buyer_name, s.name as seller_name
         FROM market_history mh
         LEFT JOIN characters b ON mh.buyer_id = b.id
         LEFT JOIN characters s ON mh.seller_id = s.id
         WHERE ($1::int IS NULL OR mh.realm_id = $1)
           AND ($2::int IS NULL OR mh.item_type = $2)
         ORDER BY mh.completed_at DESC
         LIMIT $3 OFFSET $4"
    )
    .bind(query.realm_id)
    .bind(query.item_type)
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(history.into_iter().map(|h| MarketHistory {
        item_type: h.item_type,
        amount: h.amount,
        price: h.price,
        buyer_name: h.buyer_name,
        seller_name: h.seller_name,
        completed_at: h.completed_at.to_rfc3339(),
    }).collect()))
}

#[derive(sqlx::FromRow)]
struct MarketOfferRow {
    id: i32,
    item_type: i32,
    amount: i32,
    price: i64,
    offer_type: String,
    character_name: Option<String>,
    expires_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow)]
struct MarketHistoryRow {
    item_type: i32,
    amount: i32,
    price: i64,
    buyer_name: Option<String>,
    seller_name: Option<String>,
    completed_at: chrono::DateTime<chrono::Utc>,
}
