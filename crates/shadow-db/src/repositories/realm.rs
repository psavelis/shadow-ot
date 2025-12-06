//! Realm repository - handles realm CRUD operations

use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::models::realm::{Realm, RealmStatus};
use crate::{DbError, Result};

/// Repository for realm operations
pub struct RealmRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> RealmRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Find realm by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Realm>> {
        let result = sqlx::query_as::<_, Realm>(
            "SELECT * FROM realms WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Find realm by name
    pub async fn find_by_name(&self, name: &str) -> Result<Option<Realm>> {
        let result = sqlx::query_as::<_, Realm>(
            "SELECT * FROM realms WHERE LOWER(name) = LOWER($1)"
        )
        .bind(name)
        .fetch_optional(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Find realm by slug
    pub async fn find_by_slug(&self, slug: &str) -> Result<Option<Realm>> {
        let result = sqlx::query_as::<_, Realm>(
            "SELECT * FROM realms WHERE LOWER(slug) = LOWER($1)"
        )
        .bind(slug)
        .fetch_optional(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Get all realms
    pub async fn find_all(&self) -> Result<Vec<Realm>> {
        let result = sqlx::query_as::<_, Realm>(
            "SELECT * FROM realms ORDER BY name ASC"
        )
        .fetch_all(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Get all active realms
    pub async fn find_active(&self) -> Result<Vec<Realm>> {
        let result = sqlx::query_as::<_, Realm>(
            "SELECT * FROM realms WHERE status = 'online' ORDER BY name ASC"
        )
        .fetch_all(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Update realm status
    pub async fn set_status(&self, id: Uuid, status: RealmStatus) -> Result<()> {
        sqlx::query(
            "UPDATE realms SET status = $2, updated_at = NOW() WHERE id = $1"
        )
        .bind(id)
        .bind(status)
        .execute(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(())
    }

    /// Update player count
    pub async fn update_player_count(&self, id: Uuid, current_players: i32) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE realms SET 
                current_players = $2,
                peak_players = GREATEST(peak_players, $2),
                updated_at = NOW()
            WHERE id = $1
            "#
        )
        .bind(id)
        .bind(current_players)
        .execute(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(())
    }

    /// Get online player count for a realm
    pub async fn get_player_count(&self, realm_id: Uuid) -> Result<i64> {
        let result = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM characters WHERE realm_id = $1 AND online = true"
        )
        .bind(realm_id)
        .fetch_one(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Get total player count across all realms
    pub async fn get_total_player_count(&self) -> Result<i64> {
        let result = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM characters WHERE online = true"
        )
        .fetch_one(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Update realm rates
    pub async fn update_rates(
        &self,
        id: Uuid,
        exp_rate: f64,
        skill_rate: f64,
        magic_rate: f64,
        loot_rate: f64,
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE realms SET 
                rate_experience = $2, 
                rate_skill = $3, 
                rate_magic = $4, 
                rate_loot = $5,
                updated_at = NOW()
            WHERE id = $1
            "#
        )
        .bind(id)
        .bind(exp_rate)
        .bind(skill_rate)
        .bind(magic_rate)
        .bind(loot_rate)
        .execute(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(())
    }

    /// Update last save timestamp
    pub async fn update_last_save(&self, id: Uuid) -> Result<()> {
        sqlx::query(
            "UPDATE realms SET last_save = NOW(), updated_at = NOW() WHERE id = $1"
        )
        .bind(id)
        .execute(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(())
    }

    /// Update uptime
    pub async fn update_uptime(&self, id: Uuid, uptime_seconds: i64) -> Result<()> {
        sqlx::query(
            "UPDATE realms SET uptime_seconds = $2, updated_at = NOW() WHERE id = $1"
        )
        .bind(id)
        .bind(uptime_seconds)
        .execute(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(())
    }

    /// Get realms by region
    pub async fn find_by_region(&self, region: &str) -> Result<Vec<Realm>> {
        let result = sqlx::query_as::<_, Realm>(
            "SELECT * FROM realms WHERE region = $1 ORDER BY name ASC"
        )
        .bind(region)
        .fetch_all(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Get seasonal realms
    pub async fn find_seasonal(&self) -> Result<Vec<Realm>> {
        let result = sqlx::query_as::<_, Realm>(
            r#"
            SELECT * FROM realms 
            WHERE is_seasonal = true 
            AND season_end > NOW()
            ORDER BY name ASC
            "#
        )
        .fetch_all(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }
}
