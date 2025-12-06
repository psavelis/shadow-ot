//! NFT and blockchain endpoints

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

/// Blockchain chain type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "blockchain_chain", rename_all = "lowercase")]
pub enum BlockchainChain {
    Ethereum,
    Polygon,
    Starknet,
    Bitcoin,
    Base,
    Arbitrum,
}

/// NFT status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "nft_status", rename_all = "lowercase")]
pub enum NftStatus {
    Minted,
    Listed,
    Transferred,
    Burned,
}

/// NFT information
#[derive(Debug, Serialize, ToSchema)]
pub struct Nft {
    pub id: Uuid,
    pub token_id: String,
    pub chain: BlockchainChain,
    pub contract_address: String,
    pub owner_address: String,
    pub item_id: Option<i32>,
    pub item_name: Option<String>,
    pub nft_type: String,
    pub rarity: String,
    pub metadata: NftMetadata,
    pub status: NftStatus,
    pub minted_at: DateTime<Utc>,
    pub last_transfer_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow)]
struct NftRow {
    id: Uuid,
    token_id: String,
    chain: BlockchainChain,
    contract_address: String,
    owner_address: String,
    item_id: Option<i32>,
    item_name: Option<String>,
    nft_type: String,
    rarity: String,
    metadata_name: String,
    metadata_description: String,
    metadata_image: String,
    status: NftStatus,
    minted_at: DateTime<Utc>,
    last_transfer_at: Option<DateTime<Utc>>,
}

/// NFT metadata
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NftMetadata {
    pub name: String,
    pub description: String,
    pub image: String,
    pub animation_url: Option<String>,
    pub external_url: Option<String>,
    pub attributes: Vec<NftAttribute>,
}

/// NFT attribute
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NftAttribute {
    pub trait_type: String,
    pub value: serde_json::Value,
    pub display_type: Option<String>,
}

/// NFT marketplace listing
#[derive(Debug, Serialize, ToSchema)]
pub struct NftListing {
    pub nft: Nft,
    pub price: String,
    pub currency: String,
    pub seller: String,
    pub listed_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Query parameters
#[derive(Debug, Deserialize)]
pub struct NftQuery {
    pub chain: Option<String>,
    pub nft_type: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

/// Mint NFT request
#[derive(Debug, Deserialize, ToSchema)]
pub struct MintNftRequest {
    pub item_id: i32,
    pub chain: String,
}

/// Mint NFT response
#[derive(Debug, Serialize, ToSchema)]
pub struct MintNftResponse {
    pub success: bool,
    pub tx_hash: String,
    pub nft_id: Uuid,
    pub token_id: String,
}

/// Transfer NFT request
#[derive(Debug, Deserialize, ToSchema)]
pub struct TransferNftRequest {
    pub to_address: String,
}

/// Transfer NFT response
#[derive(Debug, Serialize, ToSchema)]
pub struct TransferNftResponse {
    pub success: bool,
    pub tx_hash: String,
}

/// List NFT for sale request
#[derive(Debug, Deserialize, ToSchema)]
pub struct ListNftRequest {
    pub price: String,
    pub currency: Option<String>,
}

/// Buy NFT request
#[derive(Debug, Deserialize, ToSchema)]
pub struct BuyNftRequest {
    pub nft_id: Uuid,
}

/// Paginated NFT response
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedNfts {
    pub data: Vec<Nft>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

/// Get owned NFTs
#[utoipa::path(
    get,
    path = "/api/v1/nft/owned",
    responses(
        (status = 200, description = "Owned NFTs", body = Vec<Nft>)
    ),
    security(("bearer_auth" = [])),
    tag = "nft"
)]
pub async fn get_owned_nfts(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
) -> ApiResult<Json<Vec<Nft>>> {
    // Get wallet address for user
    let wallet: Option<(String,)> = sqlx::query_as(
        "SELECT wallet_address FROM account_wallets WHERE account_id = $1 LIMIT 1"
    )
    .bind(&claims.sub)
    .fetch_optional(&state.db)
    .await?;

    let wallet_address = wallet.map(|w| w.0).unwrap_or_default();

    if wallet_address.is_empty() {
        return Ok(Json(Vec::new()));
    }

    let rows = sqlx::query_as::<_, NftRow>(
        "SELECT n.id, n.token_id, n.chain, n.contract_address, n.owner_address,
                n.item_id, i.name as item_name, n.nft_type, n.rarity,
                n.metadata_name, n.metadata_description, n.metadata_image,
                n.status, n.minted_at, n.last_transfer_at
         FROM nfts n
         LEFT JOIN items i ON i.id = n.item_id
         WHERE n.owner_address = $1
         ORDER BY n.minted_at DESC"
    )
    .bind(&wallet_address)
    .fetch_all(&state.db)
    .await?;

    let nfts = rows.into_iter().map(|r| build_nft(r)).collect();
    Ok(Json(nfts))
}

/// Get NFT by token ID
#[utoipa::path(
    get,
    path = "/api/v1/nft/{chain}/{token_id}",
    params(
        ("chain" = String, Path, description = "Blockchain chain"),
        ("token_id" = String, Path, description = "Token ID")
    ),
    responses(
        (status = 200, description = "NFT details", body = Nft)
    ),
    tag = "nft"
)]
pub async fn get_nft(
    State(state): State<Arc<AppState>>,
    Path((chain, token_id)): Path<(String, String)>,
) -> ApiResult<Json<Nft>> {
    let row = sqlx::query_as::<_, NftRow>(
        "SELECT n.id, n.token_id, n.chain, n.contract_address, n.owner_address,
                n.item_id, i.name as item_name, n.nft_type, n.rarity,
                n.metadata_name, n.metadata_description, n.metadata_image,
                n.status, n.minted_at, n.last_transfer_at
         FROM nfts n
         LEFT JOIN items i ON i.id = n.item_id
         WHERE n.chain::text = $1 AND n.token_id = $2"
    )
    .bind(&chain.to_lowercase())
    .bind(&token_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(crate::error::ApiError::NotFound("NFT not found".to_string()))?;

    Ok(Json(build_nft(row)))
}

/// Mint NFT from in-game item
#[utoipa::path(
    post,
    path = "/api/v1/nft/mint",
    request_body = MintNftRequest,
    responses(
        (status = 200, description = "NFT minted", body = MintNftResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "nft"
)]
pub async fn mint_nft(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
    Json(request): Json<MintNftRequest>,
) -> ApiResult<Json<MintNftResponse>> {
    // Get wallet address
    let wallet: Option<(String,)> = sqlx::query_as(
        "SELECT wallet_address FROM account_wallets WHERE account_id = $1 LIMIT 1"
    )
    .bind(&claims.sub)
    .fetch_optional(&state.db)
    .await?;

    let wallet_address = wallet
        .ok_or(crate::error::ApiError::BadRequest("No wallet connected".to_string()))?.0;

    // Get item details
    let item: Option<(i32, String, String)> = sqlx::query_as(
        "SELECT id, name, COALESCE(rarity, 'common') FROM items WHERE id = $1"
    )
    .bind(request.item_id)
    .fetch_optional(&state.db)
    .await?;

    let (item_id, item_name, rarity) = item
        .ok_or(crate::error::ApiError::NotFound("Item not found".to_string()))?;

    // Generate token ID and create NFT record
    let token_id = format!("SOT-{}-{}", item_id, Uuid::new_v4().to_string()[..8].to_uppercase());
    let nft_id = Uuid::new_v4();

    let chain = match request.chain.to_lowercase().as_str() {
        "ethereum" => BlockchainChain::Ethereum,
        "polygon" => BlockchainChain::Polygon,
        "starknet" => BlockchainChain::Starknet,
        "base" => BlockchainChain::Base,
        "arbitrum" => BlockchainChain::Arbitrum,
        _ => BlockchainChain::Polygon, // Default to Polygon for lower fees
    };

    // Insert NFT record
    sqlx::query(
        "INSERT INTO nfts (id, token_id, chain, contract_address, owner_address, item_id,
                           nft_type, rarity, metadata_name, metadata_description, metadata_image,
                           status, minted_at)
         VALUES ($1, $2, $3, $4, $5, $6, 'item', $7, $8, $9, $10, 'minted', CURRENT_TIMESTAMP)"
    )
    .bind(nft_id)
    .bind(&token_id)
    .bind(chain)
    .bind("0x...") // Contract address would come from config
    .bind(&wallet_address)
    .bind(item_id)
    .bind(&rarity)
    .bind(&item_name)
    .bind(format!("Shadow OT {} - A legendary in-game item", item_name))
    .bind(format!("https://assets.shadow-ot.com/items/{}.png", item_id))
    .execute(&state.db)
    .await?;

    // In production, this would trigger actual blockchain minting
    let tx_hash = format!("0x{}", hex::encode(&Uuid::new_v4().as_bytes()[..16]));

    Ok(Json(MintNftResponse {
        success: true,
        tx_hash,
        nft_id,
        token_id,
    }))
}

/// Transfer NFT to another address
#[utoipa::path(
    post,
    path = "/api/v1/nft/transfer",
    request_body = TransferNftRequest,
    responses(
        (status = 200, description = "Transfer initiated", body = TransferNftResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "nft"
)]
pub async fn transfer_nft(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
    Path(nft_id): Path<Uuid>,
    Json(request): Json<TransferNftRequest>,
) -> ApiResult<Json<TransferNftResponse>> {
    // Verify ownership
    let wallet: Option<(String,)> = sqlx::query_as(
        "SELECT wallet_address FROM account_wallets WHERE account_id = $1 LIMIT 1"
    )
    .bind(&claims.sub)
    .fetch_optional(&state.db)
    .await?;

    let wallet_address = wallet
        .ok_or(crate::error::ApiError::BadRequest("No wallet connected".to_string()))?.0;

    let nft_owner: Option<(String,)> = sqlx::query_as(
        "SELECT owner_address FROM nfts WHERE id = $1"
    )
    .bind(nft_id)
    .fetch_optional(&state.db)
    .await?;

    let owner = nft_owner
        .ok_or(crate::error::ApiError::NotFound("NFT not found".to_string()))?.0;

    if owner.to_lowercase() != wallet_address.to_lowercase() {
        return Err(crate::error::ApiError::Forbidden("Not the owner".to_string()));
    }

    // Update ownership
    sqlx::query(
        "UPDATE nfts SET owner_address = $2, last_transfer_at = CURRENT_TIMESTAMP, status = 'transferred'
         WHERE id = $1"
    )
    .bind(nft_id)
    .bind(&request.to_address)
    .execute(&state.db)
    .await?;

    // Log transfer
    sqlx::query(
        "INSERT INTO nft_transfers (nft_id, from_address, to_address, transferred_at)
         VALUES ($1, $2, $3, CURRENT_TIMESTAMP)"
    )
    .bind(nft_id)
    .bind(&wallet_address)
    .bind(&request.to_address)
    .execute(&state.db)
    .await?;

    let tx_hash = format!("0x{}", hex::encode(&Uuid::new_v4().as_bytes()[..16]));

    Ok(Json(TransferNftResponse {
        success: true,
        tx_hash,
    }))
}

/// Get NFT marketplace listings
#[utoipa::path(
    get,
    path = "/api/v1/nft/marketplace",
    params(
        ("chain" = Option<String>, Query, description = "Filter by chain"),
        ("page" = Option<u32>, Query, description = "Page number"),
        ("page_size" = Option<u32>, Query, description = "Page size")
    ),
    responses(
        (status = 200, description = "Marketplace listings", body = PaginatedNfts)
    ),
    tag = "nft"
)]
pub async fn get_marketplace(
    State(state): State<Arc<AppState>>,
    Query(query): Query<NftQuery>,
) -> ApiResult<Json<PaginatedNfts>> {
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(20).min(100);
    let offset = (page - 1) * page_size;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM nfts WHERE status = 'listed'
         AND ($1::text IS NULL OR chain::text = $1)"
    )
    .bind(&query.chain)
    .fetch_one(&state.db)
    .await?;

    let rows = sqlx::query_as::<_, NftRow>(
        "SELECT n.id, n.token_id, n.chain, n.contract_address, n.owner_address,
                n.item_id, i.name as item_name, n.nft_type, n.rarity,
                n.metadata_name, n.metadata_description, n.metadata_image,
                n.status, n.minted_at, n.last_transfer_at
         FROM nfts n
         LEFT JOIN items i ON i.id = n.item_id
         WHERE n.status = 'listed'
           AND ($1::text IS NULL OR n.chain::text = $1)
         ORDER BY n.minted_at DESC
         LIMIT $2 OFFSET $3"
    )
    .bind(&query.chain)
    .bind(page_size as i64)
    .bind(offset as i64)
    .fetch_all(&state.db)
    .await?;

    let nfts = rows.into_iter().map(|r| build_nft(r)).collect();

    Ok(Json(PaginatedNfts {
        data: nfts,
        total: total.0,
        page,
        page_size,
        total_pages: ((total.0 as f64) / (page_size as f64)).ceil() as u32,
    }))
}

/// List NFT for sale
#[utoipa::path(
    post,
    path = "/api/v1/nft/list",
    request_body = ListNftRequest,
    responses(
        (status = 200, description = "NFT listed for sale")
    ),
    security(("bearer_auth" = [])),
    tag = "nft"
)]
pub async fn list_nft(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
    Path(nft_id): Path<Uuid>,
    Json(request): Json<ListNftRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    // Verify ownership (similar to transfer)
    let wallet: Option<(String,)> = sqlx::query_as(
        "SELECT wallet_address FROM account_wallets WHERE account_id = $1 LIMIT 1"
    )
    .bind(&claims.sub)
    .fetch_optional(&state.db)
    .await?;

    let wallet_address = wallet
        .ok_or(crate::error::ApiError::BadRequest("No wallet connected".to_string()))?.0;

    // Update NFT status
    sqlx::query(
        "UPDATE nfts SET status = 'listed', listing_price = $2, listing_currency = $3
         WHERE id = $1 AND owner_address = $4"
    )
    .bind(nft_id)
    .bind(&request.price)
    .bind(request.currency.as_deref().unwrap_or("ETH"))
    .bind(&wallet_address)
    .execute(&state.db)
    .await?;

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "NFT listed for sale"
    })))
}

/// Buy NFT from marketplace
#[utoipa::path(
    post,
    path = "/api/v1/nft/buy",
    request_body = BuyNftRequest,
    responses(
        (status = 200, description = "NFT purchased")
    ),
    security(("bearer_auth" = [])),
    tag = "nft"
)]
pub async fn buy_nft(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
    Json(request): Json<BuyNftRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let wallet: Option<(String,)> = sqlx::query_as(
        "SELECT wallet_address FROM account_wallets WHERE account_id = $1 LIMIT 1"
    )
    .bind(&claims.sub)
    .fetch_optional(&state.db)
    .await?;

    let wallet_address = wallet
        .ok_or(crate::error::ApiError::BadRequest("No wallet connected".to_string()))?.0;

    // Update ownership
    sqlx::query(
        "UPDATE nfts SET owner_address = $2, status = 'transferred', last_transfer_at = CURRENT_TIMESTAMP
         WHERE id = $1 AND status = 'listed'"
    )
    .bind(request.nft_id)
    .bind(&wallet_address)
    .execute(&state.db)
    .await?;

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "NFT purchased successfully"
    })))
}

/// Cancel NFT listing
#[utoipa::path(
    post,
    path = "/api/v1/nft/cancel-listing",
    responses(
        (status = 200, description = "Listing cancelled")
    ),
    security(("bearer_auth" = [])),
    tag = "nft"
)]
pub async fn cancel_listing(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
    Path(nft_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let wallet: Option<(String,)> = sqlx::query_as(
        "SELECT wallet_address FROM account_wallets WHERE account_id = $1 LIMIT 1"
    )
    .bind(&claims.sub)
    .fetch_optional(&state.db)
    .await?;

    let wallet_address = wallet
        .ok_or(crate::error::ApiError::BadRequest("No wallet connected".to_string()))?.0;

    sqlx::query(
        "UPDATE nfts SET status = 'minted', listing_price = NULL
         WHERE id = $1 AND owner_address = $2 AND status = 'listed'"
    )
    .bind(nft_id)
    .bind(&wallet_address)
    .execute(&state.db)
    .await?;

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Listing cancelled"
    })))
}

/// Helper to build NFT from row
fn build_nft(row: NftRow) -> Nft {
    Nft {
        id: row.id,
        token_id: row.token_id,
        chain: row.chain,
        contract_address: row.contract_address,
        owner_address: row.owner_address,
        item_id: row.item_id,
        item_name: row.item_name,
        nft_type: row.nft_type,
        rarity: row.rarity,
        metadata: NftMetadata {
            name: row.metadata_name,
            description: row.metadata_description,
            image: row.metadata_image,
            animation_url: None,
            external_url: None,
            attributes: Vec::new(),
        },
        status: row.status,
        minted_at: row.minted_at,
        last_transfer_at: row.last_transfer_at,
    }
}
