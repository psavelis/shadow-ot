//! House model - player housing system

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// In-game house
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct House {
    pub id: i32,
    pub realm_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub town_id: i32,
    pub owner_id: Option<Uuid>,
    pub paid_until: Option<DateTime<Utc>>,
    pub rent: i64,
    pub size: i32, // SQM
    pub beds: i32,
    pub entry_x: i32,
    pub entry_y: i32,
    pub entry_z: i32,
    pub guild_hall: bool,
    pub last_warning: Option<DateTime<Utc>>,
    pub bid_id: Option<Uuid>,
    pub highest_bid: i64,
    pub bid_end: Option<DateTime<Utc>>,
    pub is_nft: bool,
    pub nft_token_id: Option<String>,
    pub nft_chain: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// House access list
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct HouseAccess {
    pub house_id: i32,
    pub character_id: Uuid,
    pub access_type: HouseAccessType,
    pub granted_by: Uuid,
    pub granted_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "house_access_type", rename_all = "lowercase")]
pub enum HouseAccessType {
    Guest,
    SubOwner,
}

/// House auction bid
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct HouseBid {
    pub id: Uuid,
    pub house_id: i32,
    pub character_id: Uuid,
    pub amount: i64,
    pub status: HouseBidStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "house_bid_status", rename_all = "lowercase")]
pub enum HouseBidStatus {
    Active,
    Won,
    Lost,
    Cancelled,
}

/// House transfer history
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct HouseTransfer {
    pub id: Uuid,
    pub house_id: i32,
    pub from_id: Option<Uuid>,
    pub to_id: Option<Uuid>,
    pub transfer_type: HouseTransferType,
    pub price: i64,
    pub transferred_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "house_transfer_type", rename_all = "snake_case")]
pub enum HouseTransferType {
    Auction,
    DirectSale,
    Gift,
    Eviction,
    AdminAction,
    NftTransfer,
}
