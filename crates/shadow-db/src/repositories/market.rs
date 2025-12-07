//! Market repository - handles market/auction operations

use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};

use crate::models::market::{MarketOffer, MarketOfferType, MarketOfferStatus, MarketTransaction};
use crate::models::{OfferType, OfferState, MarketHistory}; // Type aliases
use crate::{DbError, Result};

/// Repository for market operations
pub struct MarketRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> MarketRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Create a new market offer
    pub async fn create_offer(&self, offer: &MarketOffer) -> Result<MarketOffer> {
        let result = sqlx::query_as::<_, MarketOffer>(
            r#"
            INSERT INTO market_offers (
                id, realm_id, character_id, item_type_id, amount, price,
                offer_type, status, anonymous, created_at, expires_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING *
            "#,
        )
        .bind(&offer.id)
        .bind(&offer.realm_id)
        .bind(&offer.character_id)
        .bind(offer.item_type_id)
        .bind(offer.amount)
        .bind(offer.price)
        .bind(&offer.offer_type)
        .bind(&offer.status)
        .bind(offer.anonymous)
        .bind(&offer.created_at)
        .bind(&offer.expires_at)
        .fetch_one(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Find offer by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<MarketOffer>> {
        let result = sqlx::query_as::<_, MarketOffer>(
            "SELECT * FROM market_offers WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Get all active offers for an item
    pub async fn find_by_item(
        &self, 
        realm_id: Uuid, 
        item_id: i32, 
        offer_type: Option<OfferType>
    ) -> Result<Vec<MarketOffer>> {
        let base_query = r#"
            SELECT * FROM market_offers 
            WHERE realm_id = $1 
            AND item_id = $2 
            AND state = 'active'
            AND expires_at > NOW()
        "#;
        
        let query = if offer_type.is_some() {
            format!("{} AND offer_type = $3 ORDER BY price ASC", base_query)
        } else {
            format!("{} ORDER BY price ASC", base_query)
        };

        let result = if let Some(ot) = offer_type {
            sqlx::query_as::<_, MarketOffer>(&query)
                .bind(realm_id)
                .bind(item_id)
                .bind(ot)
                .fetch_all(self.pool)
                .await
        } else {
            sqlx::query_as::<_, MarketOffer>(&query)
                .bind(realm_id)
                .bind(item_id)
                .fetch_all(self.pool)
                .await
        };

        result.map_err(|e| DbError::Query(e.to_string()))
    }

    /// Get all offers by a character
    pub async fn find_by_character(&self, character_id: Uuid) -> Result<Vec<MarketOffer>> {
        let result = sqlx::query_as::<_, MarketOffer>(
            r#"
            SELECT * FROM market_offers 
            WHERE character_id = $1 AND state = 'active'
            ORDER BY created_at DESC
            "#
        )
        .bind(character_id)
        .fetch_all(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Browse market by category (item type range)
    pub async fn browse(
        &self,
        realm_id: Uuid,
        item_id_min: i32,
        item_id_max: i32,
        offer_type: OfferType,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<MarketOffer>> {
        let result = sqlx::query_as::<_, MarketOffer>(
            r#"
            SELECT * FROM market_offers 
            WHERE realm_id = $1 
            AND item_id >= $2 AND item_id <= $3
            AND offer_type = $4
            AND state = 'active'
            AND expires_at > NOW()
            ORDER BY price ASC
            LIMIT $5 OFFSET $6
            "#
        )
        .bind(realm_id)
        .bind(item_id_min)
        .bind(item_id_max)
        .bind(offer_type)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Cancel an offer
    pub async fn cancel_offer(&self, id: Uuid, character_id: Uuid) -> Result<bool> {
        let result = sqlx::query(
            r#"
            UPDATE market_offers 
            SET state = 'cancelled', updated_at = NOW()
            WHERE id = $1 AND character_id = $2 AND state = 'active'
            "#
        )
        .bind(id)
        .bind(character_id)
        .execute(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    /// Accept an offer (buy/sell)
    pub async fn accept_offer(
        &self, 
        id: Uuid, 
        buyer_id: Uuid, 
        amount: i32
    ) -> Result<MarketOffer> {
        // Start transaction
        let mut tx = self.pool.begin().await
            .map_err(|e| DbError::Query(e.to_string()))?;

        // Get and lock the offer
        let offer = sqlx::query_as::<_, MarketOffer>(
            "SELECT * FROM market_offers WHERE id = $1 AND state = 'active' FOR UPDATE"
        )
        .bind(id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?
        .ok_or_else(|| DbError::NotFound("Offer not found or not active".to_string()))?;

        if amount > offer.amount {
            return Err(DbError::Validation("Amount exceeds available".to_string()));
        }

        // Update offer
        let new_amount = offer.amount - amount;
        let new_state = if new_amount == 0 { OfferState::Completed } else { OfferState::Active };

        sqlx::query(
            r#"
            UPDATE market_offers 
            SET amount = $2, state = $3, updated_at = NOW()
            WHERE id = $1
            "#
        )
        .bind(id)
        .bind(new_amount)
        .bind(new_state)
        .execute(&mut *tx)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        // Record history
        let history_id = Uuid::new_v4();
        sqlx::query(
            r#"
            INSERT INTO market_history (
                id, realm_id, item_id, amount, price, seller_id, buyer_id, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())
            "#
        )
        .bind(history_id)
        .bind(&offer.realm_id)
        .bind(offer.item_type_id)
        .bind(amount)
        .bind(offer.price)
        .bind(&offer.character_id)
        .bind(buyer_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        tx.commit().await
            .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(offer)
    }

    /// Expire old offers
    pub async fn expire_offers(&self) -> Result<u64> {
        let result = sqlx::query(
            r#"
            UPDATE market_offers 
            SET state = 'expired', updated_at = NOW()
            WHERE state = 'active' AND expires_at < NOW()
            "#
        )
        .execute(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result.rows_affected())
    }

    /// Get market history for an item
    pub async fn get_history(
        &self,
        realm_id: Uuid,
        item_id: i32,
        days: i32,
    ) -> Result<Vec<MarketHistory>> {
        let result = sqlx::query_as::<_, MarketHistory>(
            r#"
            SELECT * FROM market_history 
            WHERE realm_id = $1 
            AND item_id = $2 
            AND created_at > NOW() - INTERVAL '1 day' * $3
            ORDER BY created_at DESC
            "#
        )
        .bind(realm_id)
        .bind(item_id)
        .bind(days)
        .fetch_all(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Get average price for an item
    pub async fn get_average_price(&self, realm_id: Uuid, item_id: i32, days: i32) -> Result<Option<i64>> {
        let result = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT AVG(price)::bigint FROM market_history 
            WHERE realm_id = $1 
            AND item_id = $2 
            AND created_at > NOW() - INTERVAL '1 day' * $3
            "#
        )
        .bind(realm_id)
        .bind(item_id)
        .bind(days)
        .fetch_optional(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Count active offers by character
    pub async fn count_by_character(&self, character_id: Uuid) -> Result<i64> {
        let result = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM market_offers WHERE character_id = $1 AND state = 'active'"
        )
        .bind(character_id)
        .fetch_one(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Get best buy price for an item
    pub async fn get_best_buy_price(&self, realm_id: Uuid, item_id: i32) -> Result<Option<i64>> {
        let result = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT MAX(price) FROM market_offers 
            WHERE realm_id = $1 
            AND item_id = $2 
            AND offer_type = 'buy'
            AND state = 'active'
            AND expires_at > NOW()
            "#
        )
        .bind(realm_id)
        .bind(item_id)
        .fetch_optional(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Get best sell price for an item
    pub async fn get_best_sell_price(&self, realm_id: Uuid, item_id: i32) -> Result<Option<i64>> {
        let result = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT MIN(price) FROM market_offers 
            WHERE realm_id = $1 
            AND item_id = $2 
            AND offer_type = 'sell'
            AND state = 'active'
            AND expires_at > NOW()
            "#
        )
        .bind(realm_id)
        .bind(item_id)
        .fetch_optional(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }
}
