//! House repository - handles house CRUD operations

use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::models::house::{House, HouseAccess, HouseBid, AccessLevel};
use crate::{DbError, Result};

/// Repository for house operations
pub struct HouseRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> HouseRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Find house by ID
    pub async fn find_by_id(&self, id: i32, realm_id: Uuid) -> Result<Option<House>> {
        let result = sqlx::query_as::<_, House>(
            "SELECT * FROM houses WHERE id = $1 AND realm_id = $2"
        )
        .bind(id)
        .bind(realm_id)
        .fetch_optional(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Find house by name
    pub async fn find_by_name(&self, name: &str, realm_id: Uuid) -> Result<Option<House>> {
        let result = sqlx::query_as::<_, House>(
            "SELECT * FROM houses WHERE LOWER(name) = LOWER($1) AND realm_id = $2"
        )
        .bind(name)
        .bind(realm_id)
        .fetch_optional(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Get all houses for a realm
    pub async fn find_by_realm(&self, realm_id: Uuid) -> Result<Vec<House>> {
        let result = sqlx::query_as::<_, House>(
            "SELECT * FROM houses WHERE realm_id = $1 ORDER BY name ASC"
        )
        .bind(realm_id)
        .fetch_all(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Get houses owned by a character
    pub async fn find_by_owner(&self, owner_id: Uuid) -> Result<Vec<House>> {
        let result = sqlx::query_as::<_, House>(
            "SELECT * FROM houses WHERE owner_id = $1 ORDER BY name ASC"
        )
        .bind(owner_id)
        .fetch_all(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Get available houses (no owner or auction)
    pub async fn find_available(&self, realm_id: Uuid) -> Result<Vec<House>> {
        let result = sqlx::query_as::<_, House>(
            r#"
            SELECT * FROM houses 
            WHERE realm_id = $1 AND owner_id IS NULL
            ORDER BY rent ASC
            "#
        )
        .bind(realm_id)
        .fetch_all(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Update house owner
    pub async fn set_owner(&self, house_id: i32, realm_id: Uuid, owner_id: Option<Uuid>) -> Result<()> {
        let now = if owner_id.is_some() { Some(Utc::now()) } else { None };
        
        sqlx::query(
            r#"
            UPDATE houses 
            SET owner_id = $3, paid_until = $4, updated_at = NOW() 
            WHERE id = $1 AND realm_id = $2
            "#
        )
        .bind(house_id)
        .bind(realm_id)
        .bind(owner_id)
        .bind(now)
        .execute(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        // Clear access list when owner changes
        if owner_id.is_none() {
            sqlx::query("DELETE FROM house_access WHERE house_id = $1")
                .bind(house_id)
                .execute(self.pool)
                .await
                .map_err(|e| DbError::Query(e.to_string()))?;
        }

        Ok(())
    }

    /// Update rent paid until
    pub async fn update_rent(&self, house_id: i32, realm_id: Uuid, paid_until: DateTime<Utc>) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE houses 
            SET paid_until = $3, updated_at = NOW() 
            WHERE id = $1 AND realm_id = $2
            "#
        )
        .bind(house_id)
        .bind(realm_id)
        .bind(paid_until)
        .execute(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(())
    }

    /// Get house access list
    pub async fn get_access_list(&self, house_id: i32) -> Result<Vec<HouseAccess>> {
        let result = sqlx::query_as::<_, HouseAccess>(
            "SELECT * FROM house_access WHERE house_id = $1"
        )
        .bind(house_id)
        .fetch_all(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Add house access
    pub async fn add_access(&self, access: &HouseAccess) -> Result<HouseAccess> {
        let result = sqlx::query_as::<_, HouseAccess>(
            r#"
            INSERT INTO house_access (house_id, character_id, access_level, granted_by, granted_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (house_id, character_id) 
            DO UPDATE SET access_level = $3, granted_by = $4, granted_at = $5
            RETURNING *
            "#
        )
        .bind(access.house_id)
        .bind(&access.character_id)
        .bind(&access.access_level)
        .bind(&access.granted_by)
        .bind(&access.granted_at)
        .fetch_one(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Remove house access
    pub async fn remove_access(&self, house_id: i32, character_id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM house_access WHERE house_id = $1 AND character_id = $2")
            .bind(house_id)
            .bind(character_id)
            .execute(self.pool)
            .await
            .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(())
    }

    /// Check if character has access to house
    pub async fn has_access(&self, house_id: i32, character_id: Uuid) -> Result<Option<AccessLevel>> {
        // First check if owner
        let house = sqlx::query_as::<_, House>(
            "SELECT * FROM houses WHERE id = $1"
        )
        .bind(house_id)
        .fetch_optional(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        if let Some(house) = house {
            if house.owner_id == Some(character_id) {
                return Ok(Some(AccessLevel::Owner));
            }
        }

        // Check access list
        let result = sqlx::query_scalar::<_, String>(
            r#"
            SELECT access_level::text FROM house_access 
            WHERE house_id = $1 AND character_id = $2
            "#
        )
        .bind(house_id)
        .bind(character_id)
        .fetch_optional(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result.map(|s| match s.as_str() {
            "guest" => AccessLevel::Guest,
            "subowner" => AccessLevel::SubOwner,
            _ => AccessLevel::Guest,
        }))
    }

    /// Create house bid
    pub async fn create_bid(&self, bid: &HouseBid) -> Result<HouseBid> {
        let result = sqlx::query_as::<_, HouseBid>(
            r#"
            INSERT INTO house_bids (id, house_id, character_id, amount, created_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#
        )
        .bind(&bid.id)
        .bind(bid.house_id)
        .bind(&bid.character_id)
        .bind(bid.amount)
        .bind(&bid.created_at)
        .fetch_one(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Get highest bid for a house
    pub async fn get_highest_bid(&self, house_id: i32) -> Result<Option<HouseBid>> {
        let result = sqlx::query_as::<_, HouseBid>(
            r#"
            SELECT * FROM house_bids 
            WHERE house_id = $1 
            ORDER BY amount DESC 
            LIMIT 1
            "#
        )
        .bind(house_id)
        .fetch_optional(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Delete all bids for a house (after auction ends)
    pub async fn clear_bids(&self, house_id: i32) -> Result<()> {
        sqlx::query("DELETE FROM house_bids WHERE house_id = $1")
            .bind(house_id)
            .execute(self.pool)
            .await
            .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(())
    }

    /// Get houses with overdue rent
    pub async fn find_overdue(&self, realm_id: Uuid) -> Result<Vec<House>> {
        let result = sqlx::query_as::<_, House>(
            r#"
            SELECT * FROM houses 
            WHERE realm_id = $1 
            AND owner_id IS NOT NULL 
            AND paid_until < NOW()
            "#
        )
        .bind(realm_id)
        .fetch_all(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }
}
