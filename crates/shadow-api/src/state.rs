//! Application state shared across handlers

use crate::auth::AuthConfig;
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

/// Cache state for Redis or in-memory cache
pub struct CacheState {
    // Placeholder for redis client or in-memory cache
    _placeholder: (),
}

impl CacheState {
    pub fn new() -> Self {
        Self { _placeholder: () }
    }
}

impl Default for CacheState {
    fn default() -> Self {
        Self::new()
    }
}
