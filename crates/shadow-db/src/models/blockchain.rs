//! Blockchain models - NFT assets, wallets, transactions

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Linked wallet address
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WalletLink {
    pub id: Uuid,
    pub account_id: Uuid,
    pub chain: BlockchainChain,
    pub address: String,
    pub primary_wallet: bool,
    pub verified: bool,
    pub verification_message: Option<String>,
    pub verification_signature: Option<String>,
    pub linked_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "blockchain_chain", rename_all = "lowercase")]
pub enum BlockchainChain {
    Ethereum,
    Polygon,
    Starknet,
    Bitcoin,
    Spark,
    Base,
    Arbitrum,
}

/// NFT asset record
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NftAsset {
    pub id: Uuid,
    pub owner_account_id: Option<Uuid>,
    pub chain: BlockchainChain,
    pub contract_address: String,
    pub token_id: String,
    pub asset_type: NftAssetType,
    pub game_asset_id: Option<Uuid>, // Reference to in-game item, house, etc.
    pub metadata_uri: String,
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub rarity: Option<NftRarity>,
    pub attributes: serde_json::Value,
    pub minted_at: DateTime<Utc>,
    pub minted_tx_hash: String,
    pub last_transfer_at: Option<DateTime<Utc>>,
    pub last_transfer_tx_hash: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "nft_asset_type", rename_all = "snake_case")]
pub enum NftAssetType {
    Item,
    Outfit,
    Mount,
    House,
    GuildEmblem,
    Achievement,
    Territory,
    EventItem,
    Character,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "nft_rarity", rename_all = "lowercase")]
pub enum NftRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
    Unique,
}

/// Cross-chain bridge request
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BridgeRequest {
    pub id: Uuid,
    pub nft_asset_id: Uuid,
    pub requester_account_id: Uuid,
    pub source_chain: BlockchainChain,
    pub target_chain: BlockchainChain,
    pub source_address: String,
    pub target_address: String,
    pub status: BridgeStatus,
    pub lock_tx_hash: Option<String>,
    pub mint_tx_hash: Option<String>,
    pub fee_paid: String,
    pub error_message: Option<String>,
    pub requested_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "bridge_status", rename_all = "snake_case")]
pub enum BridgeStatus {
    Pending,
    LockingOnSource,
    LockedOnSource,
    MintingOnTarget,
    Completed,
    Failed,
    Cancelled,
    Refunding,
    Refunded,
}

/// NFT marketplace listing
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NftListing {
    pub id: Uuid,
    pub nft_asset_id: Uuid,
    pub seller_account_id: Uuid,
    pub chain: BlockchainChain,
    pub price: String, // Wei/smallest unit
    pub currency: String, // ETH, MATIC, STRK, etc.
    pub listing_type: NftListingType,
    pub auction_end_time: Option<DateTime<Utc>>,
    pub highest_bid: Option<String>,
    pub highest_bidder_id: Option<Uuid>,
    pub status: NftListingStatus,
    pub tx_hash: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "nft_listing_type", rename_all = "snake_case")]
pub enum NftListingType {
    FixedPrice,
    Auction,
    DutchAuction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "nft_listing_status", rename_all = "lowercase")]
pub enum NftListingStatus {
    Active,
    Sold,
    Cancelled,
    Expired,
}

/// NFT transaction record
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NftTransaction {
    pub id: Uuid,
    pub nft_asset_id: Uuid,
    pub chain: BlockchainChain,
    pub tx_hash: String,
    pub tx_type: NftTxType,
    pub from_address: String,
    pub to_address: String,
    pub from_account_id: Option<Uuid>,
    pub to_account_id: Option<Uuid>,
    pub price: Option<String>,
    pub currency: Option<String>,
    pub gas_used: Option<String>,
    pub block_number: i64,
    pub confirmed: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "nft_tx_type", rename_all = "lowercase")]
pub enum NftTxType {
    Mint,
    Transfer,
    Sale,
    Bridge,
    Burn,
}

/// Token balance (for in-game currency tokens)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TokenBalance {
    pub account_id: Uuid,
    pub chain: BlockchainChain,
    pub token_address: String,
    pub balance: String,
    pub updated_at: DateTime<Utc>,
}

/// Airdrop/reward claim
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AirdropClaim {
    pub id: Uuid,
    pub account_id: Uuid,
    pub campaign_id: Uuid,
    pub chain: BlockchainChain,
    pub token_address: String,
    pub amount: String,
    pub merkle_proof: Vec<String>,
    pub claimed: bool,
    pub claim_tx_hash: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub claimed_at: Option<DateTime<Utc>>,
}
