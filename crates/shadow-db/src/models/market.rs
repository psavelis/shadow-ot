//! Market model - in-game trading

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Market offer
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MarketOffer {
    pub id: Uuid,
    pub realm_id: Uuid,
    pub character_id: Uuid,
    pub offer_type: MarketOfferType,
    pub item_type_id: i32,
    pub amount: i32,
    pub price: i64,
    pub anonymous: bool,
    pub status: MarketOfferStatus,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "market_offer_type", rename_all = "lowercase")]
pub enum MarketOfferType {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "market_offer_status", rename_all = "lowercase")]
pub enum MarketOfferStatus {
    Active,
    Completed,
    Cancelled,
    Expired,
}

/// Market transaction history
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MarketTransaction {
    pub id: Uuid,
    pub realm_id: Uuid,
    pub offer_id: Uuid,
    pub buyer_id: Uuid,
    pub seller_id: Uuid,
    pub item_type_id: i32,
    pub amount: i32,
    pub price: i64,
    pub created_at: DateTime<Utc>,
}

/// Market statistics per item
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MarketStats {
    pub realm_id: Uuid,
    pub item_type_id: i32,
    pub date: chrono::NaiveDate,
    pub avg_buy_price: i64,
    pub avg_sell_price: i64,
    pub total_traded: i64,
    pub buy_offers: i32,
    pub sell_offers: i32,
}

/// Cross-realm market offer (for linked realms)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CrossRealmOffer {
    pub id: Uuid,
    pub source_realm_id: Uuid,
    pub target_realm_ids: Vec<Uuid>,
    pub character_id: Uuid,
    pub offer_type: MarketOfferType,
    pub item_type_id: i32,
    pub amount: i32,
    pub price: i64,
    pub conversion_rate: f64,
    pub status: MarketOfferStatus,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
