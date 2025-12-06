//! Application state shared across handlers

use crate::auth::AuthConfig;
use redis::aio::ConnectionManager;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    /// Database connection pool
    pub db: PgPool,
    /// Authentication configuration
    pub auth_config: AuthConfig,
    /// Redis cache (if available)
    pub cache: Option<Arc<RwLock<CacheState>>>,
    /// Server configuration
    pub config: ServerConfig,
}

impl AppState {
    pub fn new(db: PgPool, auth_config: AuthConfig, config: ServerConfig) -> Self {
        Self {
            db,
            auth_config,
            cache: None,
            config,
        }
    }

    pub fn with_cache(mut self, cache: CacheState) -> Self {
        self.cache = Some(Arc::new(RwLock::new(cache)));
        self
    }
}

/// Server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub api_url: String,
    pub frontend_url: String,
    pub game_server_host: String,
    pub game_server_port: u16,
    pub max_characters_per_account: u8,
    pub character_deletion_days: u8,
    pub premium_features_enabled: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            api_url: "http://localhost:8080".to_string(),
            frontend_url: "http://localhost:3000".to_string(),
            game_server_host: "127.0.0.1".to_string(),
            game_server_port: 7172,
            max_characters_per_account: 10,
            character_deletion_days: 30,
            premium_features_enabled: true,
        }
    }
}

/// Cache state using Redis for sessions and rate limiting
pub struct CacheState {
    /// Redis connection manager for cache operations
    pub redis: ConnectionManager,
}

impl CacheState {
    /// Create a new cache state with Redis connection
    pub async fn new(redis_url: &str) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(redis_url)?;
        let redis = ConnectionManager::new(client).await?;
        Ok(Self { redis })
    }

    /// Create from existing connection manager
    pub fn from_connection(redis: ConnectionManager) -> Self {
        Self { redis }
    }

    /// Get a value from cache
    pub async fn get(&self, key: &str) -> Option<String> {
        use redis::AsyncCommands;
        let mut conn = self.redis.clone();
        conn.get(key).await.ok()
    }

    /// Set a value in cache with expiration
    pub async fn set_ex(&self, key: &str, value: &str, seconds: u64) -> Result<(), redis::RedisError> {
        use redis::AsyncCommands;
        let mut conn = self.redis.clone();
        conn.set_ex(key, value, seconds).await
    }

    /// Delete a key from cache
    pub async fn del(&self, key: &str) -> Result<(), redis::RedisError> {
        use redis::AsyncCommands;
        let mut conn = self.redis.clone();
        conn.del(key).await
    }

    /// Increment a counter (for rate limiting)
    pub async fn incr(&self, key: &str) -> Result<i64, redis::RedisError> {
        use redis::AsyncCommands;
        let mut conn = self.redis.clone();
        conn.incr(key, 1).await
    }

    /// Set expiration on a key
    pub async fn expire(&self, key: &str, seconds: i64) -> Result<bool, redis::RedisError> {
        use redis::AsyncCommands;
        let mut conn = self.redis.clone();
        conn.expire(key, seconds).await
    }
}
