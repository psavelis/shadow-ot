//! Database connection pool management

use sqlx::postgres::{PgPool, PgPoolOptions};
use redis::aio::ConnectionManager;

use crate::{DbConfig, DbError, Result};

/// Combined database pool with PostgreSQL and Redis
#[derive(Clone)]
pub struct DatabasePool {
    pub pg: PgPool,
    pub redis: ConnectionManager,
}

impl DatabasePool {
    pub fn postgres(&self) -> &PgPool {
        &self.pg
    }

    pub fn redis(&self) -> &ConnectionManager {
        &self.redis
    }
}

/// Create a new database pool
pub async fn create_pool(config: &DbConfig) -> Result<DatabasePool> {
    // Create PostgreSQL pool
    let pg = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(config.connect_timeout)
        .idle_timeout(Some(config.idle_timeout))
        .max_lifetime(Some(config.max_lifetime))
        .connect(&config.url)
        .await
        .map_err(|e| DbError::Connection(e.to_string()))?;

    // Create Redis connection manager
    let redis_client = redis::Client::open("redis://127.0.0.1:6379")
        .map_err(|e| DbError::Connection(e.to_string()))?;
    let redis = ConnectionManager::new(redis_client)
        .await
        .map_err(|e| DbError::Connection(e.to_string()))?;

    Ok(DatabasePool { pg, redis })
}
