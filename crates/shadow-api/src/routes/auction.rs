//! Auction endpoints for character and item auctions

use crate::auth::JwtClaims;
use crate::response::SuccessResponse;
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

/// Auction type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, sqlx::Type, PartialEq)]
#[sqlx(type_name = "auction_type", rename_all = "lowercase")]
pub enum AuctionType {
    Character,
    Item,
}

/// Auction status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, sqlx::Type, PartialEq)]
#[sqlx(type_name = "auction_status", rename_all = "lowercase")]
pub enum AuctionStatus {
    Active,
    Ended,
    Cancelled,
    Won,
}

/// Character vocation for character auctions
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "vocation", rename_all = "lowercase")]
pub enum Vocation {
    None,
    Knight,
    Paladin,
    Sorcerer,
    Druid,
    EliteKnight,
    RoyalPaladin,
    MasterSorcerer,
    ElderDruid,
}

/// Character auction details
#[derive(Debug, Serialize, ToSchema)]
pub struct CharacterAuction {
    pub id: Uuid,
    pub character_name: String,
    pub level: i32,
    pub vocation: Vocation,
    pub skills: CharacterSkills,
    pub current_bid: i64,
    pub min_bid: i64,
    pub bid_increment: i64,
    pub bid_count: i32,
    pub ends_at: DateTime<Utc>,
    pub status: AuctionStatus,
}

#[derive(Debug, FromRow)]
struct CharacterAuctionRow {
    id: Uuid,
    character_name: String,
    level: i32,
    vocation: Vocation,
    skill_fist: i32,
    skill_club: i32,
    skill_sword: i32,
    skill_axe: i32,
    skill_distance: i32,
    skill_shielding: i32,
    skill_fishing: i32,
    skill_magic: i32,
    current_bid: i64,
    min_bid: i64,
    bid_increment: i64,
    bid_count: i32,
    ends_at: DateTime<Utc>,
    status: AuctionStatus,
}

/// Character skills
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CharacterSkills {
    pub fist: i32,
    pub club: i32,
    pub sword: i32,
    pub axe: i32,
    pub distance: i32,
    pub shielding: i32,
    pub fishing: i32,
    pub magic: i32,
}

/// Item auction details
#[derive(Debug, Serialize, ToSchema)]
pub struct ItemAuction {
    pub id: Uuid,
    pub item_id: i32,
    pub item_name: String,
    pub item_count: i32,
    pub is_nft: bool,
    pub nft_token_id: Option<String>,
    pub current_bid: i64,
    pub min_bid: i64,
    pub bid_increment: i64,
    pub bid_count: i32,
    pub ends_at: DateTime<Utc>,
    pub status: AuctionStatus,
    pub seller_name: String,
}

#[derive(Debug, FromRow)]
struct ItemAuctionRow {
    id: Uuid,
    item_id: i32,
    item_name: String,
    item_count: i32,
    is_nft: bool,
    nft_token_id: Option<String>,
    current_bid: i64,
    min_bid: i64,
    bid_increment: i64,
    bid_count: i32,
    ends_at: DateTime<Utc>,
    status: AuctionStatus,
    seller_name: String,
}

/// Auction query parameters
#[derive(Debug, Deserialize)]
pub struct AuctionQuery {
    pub auction_type: Option<AuctionType>,
    pub status: Option<AuctionStatus>,
    pub min_level: Option<i32>,
    pub max_level: Option<i32>,
    pub vocation: Option<Vocation>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

/// Bid request
#[derive(Debug, Deserialize, ToSchema)]
pub struct BidRequest {
    pub amount: i64,
}

/// Create character auction request
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateCharacterAuctionRequest {
    pub character_id: Uuid,
    pub min_bid: i64,
    pub bid_increment: i64,
    pub duration_hours: i32,
}

/// Create item auction request
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateItemAuctionRequest {
    pub item_id: i32,
    pub item_count: i32,
    pub min_bid: i64,
    pub bid_increment: i64,
    pub duration_hours: i32,
}

/// Paginated auctions response
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedCharacterAuctions {
    pub data: Vec<CharacterAuction>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

/// Paginated item auctions response
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedItemAuctions {
    pub data: Vec<ItemAuction>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

/// Bid response
#[derive(Debug, Serialize, ToSchema)]
pub struct BidResponse {
    pub success: bool,
    pub new_bid: i64,
    pub bid_count: i32,
}

/// List character auctions
#[utoipa::path(
    get,
    path = "/api/v1/auctions/characters",
    params(
        ("status" = Option<AuctionStatus>, Query, description = "Filter by status"),
        ("min_level" = Option<i32>, Query, description = "Minimum character level"),
        ("max_level" = Option<i32>, Query, description = "Maximum character level"),
        ("vocation" = Option<Vocation>, Query, description = "Filter by vocation"),
        ("page" = Option<u32>, Query, description = "Page number"),
        ("page_size" = Option<u32>, Query, description = "Results per page")
    ),
    responses(
        (status = 200, description = "Character auctions", body = PaginatedCharacterAuctions)
    ),
    tag = "auctions"
)]
pub async fn list_character_auctions(
    State(state): State<Arc<AppState>>,
    Query(query): Query<AuctionQuery>,
) -> ApiResult<Json<PaginatedCharacterAuctions>> {
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(20).min(100);
    let offset = (page - 1) * page_size;

    let status = query.status.unwrap_or(AuctionStatus::Active);

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM character_auctions 
         WHERE status = $1
           AND ($2::integer IS NULL OR level >= $2)
           AND ($3::integer IS NULL OR level <= $3)
           AND ($4::vocation IS NULL OR vocation = $4)"
    )
    .bind(status)
    .bind(query.min_level)
    .bind(query.max_level)
    .bind(query.vocation)
    .fetch_one(&state.db)
    .await?;

    let order_clause = match query.sort_by.as_deref() {
        Some("level") => "level",
        Some("bid") => "current_bid",
        Some("ends") => "ends_at",
        _ => "ends_at",
    };

    let order_dir = match query.sort_order.as_deref() {
        Some("asc") => "ASC",
        _ => "DESC",
    };

    let sql = format!(
        "SELECT id, character_name, level, vocation, 
                skill_fist, skill_club, skill_sword, skill_axe, 
                skill_distance, skill_shielding, skill_fishing, skill_magic,
                current_bid, min_bid, bid_increment, bid_count, ends_at, status
         FROM character_auctions
         WHERE status = $1
           AND ($2::integer IS NULL OR level >= $2)
           AND ($3::integer IS NULL OR level <= $3)
           AND ($4::vocation IS NULL OR vocation = $4)
         ORDER BY {} {}
         LIMIT $5 OFFSET $6",
        order_clause, order_dir
    );

    let rows = sqlx::query_as::<_, CharacterAuctionRow>(&sql)
        .bind(status)
        .bind(query.min_level)
        .bind(query.max_level)
        .bind(query.vocation)
        .bind(page_size as i64)
        .bind(offset as i64)
        .fetch_all(&state.db)
        .await?;

    let auctions: Vec<CharacterAuction> = rows.into_iter().map(|r| CharacterAuction {
        id: r.id,
        character_name: r.character_name,
        level: r.level,
        vocation: r.vocation,
        skills: CharacterSkills {
            fist: r.skill_fist,
            club: r.skill_club,
            sword: r.skill_sword,
            axe: r.skill_axe,
            distance: r.skill_distance,
            shielding: r.skill_shielding,
            fishing: r.skill_fishing,
            magic: r.skill_magic,
        },
        current_bid: r.current_bid,
        min_bid: r.min_bid,
        bid_increment: r.bid_increment,
        bid_count: r.bid_count,
        ends_at: r.ends_at,
        status: r.status,
    }).collect();

    Ok(Json(PaginatedCharacterAuctions {
        data: auctions,
        total: total.0,
        page,
        page_size,
        total_pages: ((total.0 as f64) / (page_size as f64)).ceil() as u32,
    }))
}

/// List item auctions
#[utoipa::path(
    get,
    path = "/api/v1/auctions/items",
    params(
        ("status" = Option<AuctionStatus>, Query, description = "Filter by status"),
        ("page" = Option<u32>, Query, description = "Page number"),
        ("page_size" = Option<u32>, Query, description = "Results per page")
    ),
    responses(
        (status = 200, description = "Item auctions", body = PaginatedItemAuctions)
    ),
    tag = "auctions"
)]
pub async fn list_item_auctions(
    State(state): State<Arc<AppState>>,
    Query(query): Query<AuctionQuery>,
) -> ApiResult<Json<PaginatedItemAuctions>> {
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(20).min(100);
    let offset = (page - 1) * page_size;

    let status = query.status.unwrap_or(AuctionStatus::Active);

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM item_auctions WHERE status = $1"
    )
    .bind(status)
    .fetch_one(&state.db)
    .await?;

    let order_clause = match query.sort_by.as_deref() {
        Some("bid") => "current_bid",
        Some("ends") => "ends_at",
        Some("name") => "item_name",
        _ => "ends_at",
    };

    let order_dir = match query.sort_order.as_deref() {
        Some("asc") => "ASC",
        _ => "DESC",
    };

    let sql = format!(
        "SELECT id, item_id, item_name, item_count, is_nft, nft_token_id,
                current_bid, min_bid, bid_increment, bid_count, ends_at, status, seller_name
         FROM item_auctions
         WHERE status = $1
         ORDER BY {} {}
         LIMIT $2 OFFSET $3",
        order_clause, order_dir
    );

    let rows = sqlx::query_as::<_, ItemAuctionRow>(&sql)
        .bind(status)
        .bind(page_size as i64)
        .bind(offset as i64)
        .fetch_all(&state.db)
        .await?;

    let auctions: Vec<ItemAuction> = rows.into_iter().map(|r| ItemAuction {
        id: r.id,
        item_id: r.item_id,
        item_name: r.item_name,
        item_count: r.item_count,
        is_nft: r.is_nft,
        nft_token_id: r.nft_token_id,
        current_bid: r.current_bid,
        min_bid: r.min_bid,
        bid_increment: r.bid_increment,
        bid_count: r.bid_count,
        ends_at: r.ends_at,
        status: r.status,
        seller_name: r.seller_name,
    }).collect();

    Ok(Json(PaginatedItemAuctions {
        data: auctions,
        total: total.0,
        page,
        page_size,
        total_pages: ((total.0 as f64) / (page_size as f64)).ceil() as u32,
    }))
}

/// Get a specific character auction
#[utoipa::path(
    get,
    path = "/api/v1/auctions/characters/{id}",
    params(
        ("id" = Uuid, Path, description = "Auction ID")
    ),
    responses(
        (status = 200, description = "Auction details", body = CharacterAuction)
    ),
    tag = "auctions"
)]
pub async fn get_character_auction(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<CharacterAuction>> {
    let row = sqlx::query_as::<_, CharacterAuctionRow>(
        "SELECT id, character_name, level, vocation, 
                skill_fist, skill_club, skill_sword, skill_axe, 
                skill_distance, skill_shielding, skill_fishing, skill_magic,
                current_bid, min_bid, bid_increment, bid_count, ends_at, status
         FROM character_auctions
         WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(crate::error::ApiError::NotFound("Auction not found".to_string()))?;

    Ok(Json(CharacterAuction {
        id: row.id,
        character_name: row.character_name,
        level: row.level,
        vocation: row.vocation,
        skills: CharacterSkills {
            fist: row.skill_fist,
            club: row.skill_club,
            sword: row.skill_sword,
            axe: row.skill_axe,
            distance: row.skill_distance,
            shielding: row.skill_shielding,
            fishing: row.skill_fishing,
            magic: row.skill_magic,
        },
        current_bid: row.current_bid,
        min_bid: row.min_bid,
        bid_increment: row.bid_increment,
        bid_count: row.bid_count,
        ends_at: row.ends_at,
        status: row.status,
    }))
}

/// Get a specific item auction
#[utoipa::path(
    get,
    path = "/api/v1/auctions/items/{id}",
    params(
        ("id" = Uuid, Path, description = "Auction ID")
    ),
    responses(
        (status = 200, description = "Auction details", body = ItemAuction)
    ),
    tag = "auctions"
)]
pub async fn get_item_auction(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ItemAuction>> {
    let row = sqlx::query_as::<_, ItemAuctionRow>(
        "SELECT id, item_id, item_name, item_count, is_nft, nft_token_id,
                current_bid, min_bid, bid_increment, bid_count, ends_at, status, seller_name
         FROM item_auctions
         WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(crate::error::ApiError::NotFound("Auction not found".to_string()))?;

    Ok(Json(ItemAuction {
        id: row.id,
        item_id: row.item_id,
        item_name: row.item_name,
        item_count: row.item_count,
        is_nft: row.is_nft,
        nft_token_id: row.nft_token_id,
        current_bid: row.current_bid,
        min_bid: row.min_bid,
        bid_increment: row.bid_increment,
        bid_count: row.bid_count,
        ends_at: row.ends_at,
        status: row.status,
        seller_name: row.seller_name,
    }))
}

/// Bid on a character auction
#[utoipa::path(
    post,
    path = "/api/v1/auctions/characters/{id}/bid",
    params(
        ("id" = Uuid, Path, description = "Auction ID")
    ),
    request_body = BidRequest,
    responses(
        (status = 200, description = "Bid placed", body = BidResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "auctions"
)]
pub async fn bid_on_character_auction(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
    Path(id): Path<Uuid>,
    Json(req): Json<BidRequest>,
) -> ApiResult<Json<BidResponse>> {
    // Start transaction
    let mut tx = state.db.begin().await?;

    // Lock and fetch auction
    let auction = sqlx::query_as::<_, CharacterAuctionRow>(
        "SELECT id, character_name, level, vocation, 
                skill_fist, skill_club, skill_sword, skill_axe, 
                skill_distance, skill_shielding, skill_fishing, skill_magic,
                current_bid, min_bid, bid_increment, bid_count, ends_at, status
         FROM character_auctions
         WHERE id = $1
         FOR UPDATE"
    )
    .bind(id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(crate::error::ApiError::NotFound("Auction not found".to_string()))?;

    if auction.status != AuctionStatus::Active {
        return Err(crate::error::ApiError::BadRequest("Auction is not active".to_string()));
    }

    if auction.ends_at < Utc::now() {
        return Err(crate::error::ApiError::BadRequest("Auction has ended".to_string()));
    }

    let min_required = if auction.bid_count == 0 {
        auction.min_bid
    } else {
        auction.current_bid + auction.bid_increment
    };

    if req.amount < min_required {
        return Err(crate::error::ApiError::BadRequest(
            format!("Bid must be at least {} coins", min_required)
        ));
    }

    // Check bidder has enough balance
    let balance: (i64,) = sqlx::query_as(
        "SELECT coins FROM accounts WHERE id = $1"
    )
    .bind(&claims.sub)
    .fetch_one(&mut *tx)
    .await?;

    if balance.0 < req.amount {
        return Err(crate::error::ApiError::BadRequest("Insufficient coins".to_string()));
    }

    // Refund previous bidder if any
    if auction.bid_count > 0 {
        sqlx::query(
            "UPDATE accounts SET coins = coins + $1
             WHERE id = (SELECT bidder_id FROM auction_bids 
                        WHERE auction_id = $2 AND auction_type = 'character'
                        ORDER BY amount DESC LIMIT 1)"
        )
        .bind(auction.current_bid)
        .bind(id)
        .execute(&mut *tx)
        .await?;
    }

    // Deduct from bidder
    sqlx::query("UPDATE accounts SET coins = coins - $1 WHERE id = $2")
        .bind(req.amount)
        .bind(&claims.sub)
        .execute(&mut *tx)
        .await?;

    // Record bid
    sqlx::query(
        "INSERT INTO auction_bids (id, auction_id, auction_type, bidder_id, amount, created_at)
         VALUES ($1, $2, 'character', $3, $4, $5)"
    )
    .bind(Uuid::new_v4())
    .bind(id)
    .bind(&claims.sub)
    .bind(req.amount)
    .bind(Utc::now())
    .execute(&mut *tx)
    .await?;

    // Update auction
    let new_count = auction.bid_count + 1;
    sqlx::query(
        "UPDATE character_auctions SET current_bid = $1, bid_count = $2 WHERE id = $3"
    )
    .bind(req.amount)
    .bind(new_count)
    .bind(id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(Json(BidResponse {
        success: true,
        new_bid: req.amount,
        bid_count: new_count,
    }))
}

/// Bid on an item auction
#[utoipa::path(
    post,
    path = "/api/v1/auctions/items/{id}/bid",
    params(
        ("id" = Uuid, Path, description = "Auction ID")
    ),
    request_body = BidRequest,
    responses(
        (status = 200, description = "Bid placed", body = BidResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "auctions"
)]
pub async fn bid_on_item_auction(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
    Path(id): Path<Uuid>,
    Json(req): Json<BidRequest>,
) -> ApiResult<Json<BidResponse>> {
    let mut tx = state.db.begin().await?;

    let auction = sqlx::query_as::<_, ItemAuctionRow>(
        "SELECT id, item_id, item_name, item_count, is_nft, nft_token_id,
                current_bid, min_bid, bid_increment, bid_count, ends_at, status, seller_name
         FROM item_auctions
         WHERE id = $1
         FOR UPDATE"
    )
    .bind(id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(crate::error::ApiError::NotFound("Auction not found".to_string()))?;

    if auction.status != AuctionStatus::Active {
        return Err(crate::error::ApiError::BadRequest("Auction is not active".to_string()));
    }

    if auction.ends_at < Utc::now() {
        return Err(crate::error::ApiError::BadRequest("Auction has ended".to_string()));
    }

    let min_required = if auction.bid_count == 0 {
        auction.min_bid
    } else {
        auction.current_bid + auction.bid_increment
    };

    if req.amount < min_required {
        return Err(crate::error::ApiError::BadRequest(
            format!("Bid must be at least {} coins", min_required)
        ));
    }

    // Check balance
    let balance: (i64,) = sqlx::query_as(
        "SELECT coins FROM accounts WHERE id = $1"
    )
    .bind(&claims.sub)
    .fetch_one(&mut *tx)
    .await?;

    if balance.0 < req.amount {
        return Err(crate::error::ApiError::BadRequest("Insufficient coins".to_string()));
    }

    // Refund previous bidder
    if auction.bid_count > 0 {
        sqlx::query(
            "UPDATE accounts SET coins = coins + $1
             WHERE id = (SELECT bidder_id FROM auction_bids 
                        WHERE auction_id = $2 AND auction_type = 'item'
                        ORDER BY amount DESC LIMIT 1)"
        )
        .bind(auction.current_bid)
        .bind(id)
        .execute(&mut *tx)
        .await?;
    }

    // Deduct from bidder
    sqlx::query("UPDATE accounts SET coins = coins - $1 WHERE id = $2")
        .bind(req.amount)
        .bind(&claims.sub)
        .execute(&mut *tx)
        .await?;

    // Record bid
    sqlx::query(
        "INSERT INTO auction_bids (id, auction_id, auction_type, bidder_id, amount, created_at)
         VALUES ($1, $2, 'item', $3, $4, $5)"
    )
    .bind(Uuid::new_v4())
    .bind(id)
    .bind(&claims.sub)
    .bind(req.amount)
    .bind(Utc::now())
    .execute(&mut *tx)
    .await?;

    // Update auction
    let new_count = auction.bid_count + 1;
    sqlx::query(
        "UPDATE item_auctions SET current_bid = $1, bid_count = $2 WHERE id = $3"
    )
    .bind(req.amount)
    .bind(new_count)
    .bind(id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(Json(BidResponse {
        success: true,
        new_bid: req.amount,
        bid_count: new_count,
    }))
}

/// Create a character auction
#[utoipa::path(
    post,
    path = "/api/v1/auctions/characters",
    request_body = CreateCharacterAuctionRequest,
    responses(
        (status = 201, description = "Auction created", body = CharacterAuction)
    ),
    security(("bearer_auth" = [])),
    tag = "auctions"
)]
pub async fn create_character_auction(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
    Json(req): Json<CreateCharacterAuctionRequest>,
) -> ApiResult<Json<CharacterAuction>> {
    // Verify character belongs to user
    let character: Option<(String, i32, Vocation, i32, i32, i32, i32, i32, i32, i32, i32)> = sqlx::query_as(
        "SELECT name, level, vocation, skill_fist, skill_club, skill_sword, skill_axe,
                skill_distance, skill_shielding, skill_fishing, magic_level
         FROM characters
         WHERE id = $1 AND account_id = $2"
    )
    .bind(req.character_id)
    .bind(&claims.sub)
    .fetch_optional(&state.db)
    .await?;

    let char = character.ok_or(
        crate::error::ApiError::NotFound("Character not found".to_string())
    )?;

    // Check no active auction exists
    let existing: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM character_auctions WHERE character_id = $1 AND status = 'active'"
    )
    .bind(req.character_id)
    .fetch_one(&state.db)
    .await?;

    if existing.0 > 0 {
        return Err(crate::error::ApiError::BadRequest(
            "Character already has an active auction".to_string()
        ));
    }

    let auction_id = Uuid::new_v4();
    let ends_at = Utc::now() + chrono::Duration::hours(req.duration_hours as i64);

    sqlx::query(
        "INSERT INTO character_auctions 
         (id, character_id, character_name, level, vocation, 
          skill_fist, skill_club, skill_sword, skill_axe, 
          skill_distance, skill_shielding, skill_fishing, skill_magic,
          current_bid, min_bid, bid_increment, bid_count, ends_at, status, seller_id)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)"
    )
    .bind(auction_id)
    .bind(req.character_id)
    .bind(&char.0)
    .bind(char.1)
    .bind(char.2)
    .bind(char.3)
    .bind(char.4)
    .bind(char.5)
    .bind(char.6)
    .bind(char.7)
    .bind(char.8)
    .bind(char.9)
    .bind(char.10)
    .bind(0i64)
    .bind(req.min_bid)
    .bind(req.bid_increment)
    .bind(0i32)
    .bind(ends_at)
    .bind(AuctionStatus::Active)
    .bind(&claims.sub)
    .execute(&state.db)
    .await?;

    Ok(Json(CharacterAuction {
        id: auction_id,
        character_name: char.0,
        level: char.1,
        vocation: char.2,
        skills: CharacterSkills {
            fist: char.3,
            club: char.4,
            sword: char.5,
            axe: char.6,
            distance: char.7,
            shielding: char.8,
            fishing: char.9,
            magic: char.10,
        },
        current_bid: 0,
        min_bid: req.min_bid,
        bid_increment: req.bid_increment,
        bid_count: 0,
        ends_at,
        status: AuctionStatus::Active,
    }))
}

/// Create an item auction
#[utoipa::path(
    post,
    path = "/api/v1/auctions/items",
    request_body = CreateItemAuctionRequest,
    responses(
        (status = 201, description = "Auction created", body = ItemAuction)
    ),
    security(("bearer_auth" = [])),
    tag = "auctions"
)]
pub async fn create_item_auction(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
    Json(req): Json<CreateItemAuctionRequest>,
) -> ApiResult<Json<ItemAuction>> {
    // Verify user owns the item
    let item: Option<(String, Option<String>)> = sqlx::query_as(
        "SELECT i.name, ui.nft_token_id
         FROM user_items ui
         JOIN items i ON i.id = ui.item_id
         WHERE ui.item_id = $1 AND ui.account_id = $2 AND ui.count >= $3"
    )
    .bind(req.item_id)
    .bind(&claims.sub)
    .bind(req.item_count)
    .fetch_optional(&state.db)
    .await?;

    let (item_name, nft_token_id) = item.ok_or(
        crate::error::ApiError::NotFound("Item not found or insufficient quantity".to_string())
    )?;

    // Get seller name
    let seller: (String,) = sqlx::query_as(
        "SELECT name FROM accounts WHERE id = $1"
    )
    .bind(&claims.sub)
    .fetch_one(&state.db)
    .await?;

    let auction_id = Uuid::new_v4();
    let ends_at = Utc::now() + chrono::Duration::hours(req.duration_hours as i64);
    let is_nft = nft_token_id.is_some();

    // Deduct items from user
    sqlx::query(
        "UPDATE user_items SET count = count - $1 WHERE item_id = $2 AND account_id = $3"
    )
    .bind(req.item_count)
    .bind(req.item_id)
    .bind(&claims.sub)
    .execute(&state.db)
    .await?;

    sqlx::query(
        "INSERT INTO item_auctions 
         (id, item_id, item_name, item_count, is_nft, nft_token_id,
          current_bid, min_bid, bid_increment, bid_count, ends_at, status, seller_id, seller_name)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)"
    )
    .bind(auction_id)
    .bind(req.item_id)
    .bind(&item_name)
    .bind(req.item_count)
    .bind(is_nft)
    .bind(&nft_token_id)
    .bind(0i64)
    .bind(req.min_bid)
    .bind(req.bid_increment)
    .bind(0i32)
    .bind(ends_at)
    .bind(AuctionStatus::Active)
    .bind(&claims.sub)
    .bind(&seller.0)
    .execute(&state.db)
    .await?;

    Ok(Json(ItemAuction {
        id: auction_id,
        item_id: req.item_id,
        item_name,
        item_count: req.item_count,
        is_nft,
        nft_token_id,
        current_bid: 0,
        min_bid: req.min_bid,
        bid_increment: req.bid_increment,
        bid_count: 0,
        ends_at,
        status: AuctionStatus::Active,
        seller_name: seller.0,
    }))
}

/// Cancel an auction (only if no bids)
#[utoipa::path(
    delete,
    path = "/api/v1/auctions/{auction_type}/{id}",
    params(
        ("auction_type" = String, Path, description = "Auction type: 'characters' or 'items'"),
        ("id" = Uuid, Path, description = "Auction ID")
    ),
    responses(
        (status = 200, description = "Auction cancelled")
    ),
    security(("bearer_auth" = [])),
    tag = "auctions"
)]
pub async fn cancel_auction(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
    Path((auction_type, id)): Path<(String, Uuid)>,
) -> ApiResult<Json<SuccessResponse>> {
    match auction_type.as_str() {
        "characters" => {
            let result = sqlx::query(
                "UPDATE character_auctions SET status = $1 
                 WHERE id = $2 AND seller_id = $3 AND bid_count = 0 AND status = 'active'"
            )
            .bind(AuctionStatus::Cancelled)
            .bind(id)
            .bind(&claims.sub)
            .execute(&state.db)
            .await?;

            if result.rows_affected() == 0 {
                return Err(crate::error::ApiError::BadRequest(
                    "Cannot cancel: auction not found, has bids, or already ended".to_string()
                ));
            }
        }
        "items" => {
            // Get item info first to return to user
            let auction: Option<(i32, i32, String)> = sqlx::query_as(
                "SELECT item_id, item_count, seller_id 
                 FROM item_auctions 
                 WHERE id = $1 AND seller_id = $2 AND bid_count = 0 AND status = 'active'"
            )
            .bind(id)
            .bind(&claims.sub)
            .fetch_optional(&state.db)
            .await?;

            let (item_id, item_count, _seller) = auction.ok_or(
                crate::error::ApiError::BadRequest(
                    "Cannot cancel: auction not found, has bids, or already ended".to_string()
                )
            )?;

            // Return items to seller
            sqlx::query(
                "UPDATE user_items SET count = count + $1 
                 WHERE item_id = $2 AND account_id = $3"
            )
            .bind(item_count)
            .bind(item_id)
            .bind(&claims.sub)
            .execute(&state.db)
            .await?;

            // Cancel auction
            sqlx::query(
                "UPDATE item_auctions SET status = $1 WHERE id = $2"
            )
            .bind(AuctionStatus::Cancelled)
            .bind(id)
            .execute(&state.db)
            .await?;
        }
        _ => {
            return Err(crate::error::ApiError::BadRequest("Invalid auction type".to_string()));
        }
    }

    Ok(Json(SuccessResponse::ok("Auction cancelled")))
}
