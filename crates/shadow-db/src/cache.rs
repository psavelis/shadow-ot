//! Redis caching layer

use redis::AsyncCommands;
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

use crate::{DbError, Result};

/// Cache key prefixes
pub mod keys {
    pub const SESSION: &str = "shadow:session:";
    pub const PLAYER: &str = "shadow:player:";
    pub const CHARACTER: &str = "shadow:character:";
    pub const REALM: &str = "shadow:realm:";
    pub const ONLINE: &str = "shadow:online:";
    pub const RATE_LIMIT: &str = "shadow:ratelimit:";
}

/// Cache operations
pub struct Cache {
    redis: redis::aio::ConnectionManager,
}

impl Cache {
    pub fn new(redis: redis::aio::ConnectionManager) -> Self {
        Self { redis }
    }

    /// Get a value from cache
    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>> {
        let mut conn = self.redis.clone();
        let value: Option<String> = conn.get(key).await?;

        match value {
            Some(json) => {
                let parsed = serde_json::from_str(&json)
                    .map_err(|e| DbError::Serialization(e.to_string()))?;
                Ok(Some(parsed))
            }
            None => Ok(None),
        }
    }

    /// Set a value in cache with TTL
    pub async fn set<T: Serialize>(&self, key: &str, value: &T, ttl: Duration) -> Result<()> {
        let mut conn = self.redis.clone();
        let json = serde_json::to_string(value)
            .map_err(|e| DbError::Serialization(e.to_string()))?;
        conn.set_ex(key, json, ttl.as_secs()).await?;
        Ok(())
    }

    /// Delete a key from cache
    pub async fn delete(&self, key: &str) -> Result<()> {
        let mut conn = self.redis.clone();
        conn.del(key).await?;
        Ok(())
    }

    /// Check if a key exists
    pub async fn exists(&self, key: &str) -> Result<bool> {
        let mut conn = self.redis.clone();
        let exists: bool = conn.exists(key).await?;
        Ok(exists)
    }

    /// Increment a counter
    pub async fn incr(&self, key: &str) -> Result<i64> {
        let mut conn = self.redis.clone();
        let value: i64 = conn.incr(key, 1).await?;
        Ok(value)
    }

    /// Set expiry on a key
    pub async fn expire(&self, key: &str, ttl: Duration) -> Result<()> {
        let mut conn = self.redis.clone();
        conn.expire(key, ttl.as_secs() as i64).await?;
        Ok(())
    }

    /// Add to a set
    pub async fn sadd(&self, key: &str, member: &str) -> Result<()> {
        let mut conn = self.redis.clone();
        conn.sadd(key, member).await?;
        Ok(())
    }

    /// Remove from a set
    pub async fn srem(&self, key: &str, member: &str) -> Result<()> {
        let mut conn = self.redis.clone();
        conn.srem(key, member).await?;
        Ok(())
    }

    /// Get all members of a set
    pub async fn smembers(&self, key: &str) -> Result<Vec<String>> {
        let mut conn = self.redis.clone();
        let members: Vec<String> = conn.smembers(key).await?;
        Ok(members)
    }

    /// Check rate limit
    pub async fn check_rate_limit(
        &self,
        key: &str,
        max_requests: u32,
        window: Duration,
    ) -> Result<bool> {
        let full_key = format!("{}{}", keys::RATE_LIMIT, key);
        let mut conn = self.redis.clone();

        let count: i64 = conn.incr(&full_key, 1).await?;

        if count == 1 {
            conn.expire(&full_key, window.as_secs() as i64).await?;
        }

        Ok(count <= max_requests as i64)
    }
}
