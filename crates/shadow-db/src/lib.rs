//! Shadow OT Database Layer
//!
//! Complete database abstraction with PostgreSQL for persistent storage
//! and Redis for caching/sessions.

pub mod error;
pub mod models;
pub mod repositories;
pub mod migrations;
pub mod cache;
pub mod pool;

pub use error::{DbError, Result};
pub use pool::{DatabasePool, create_pool};

use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

/// Database configuration
#[derive(Debug, Clone)]
pub struct DbConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: Duration,
    pub idle_timeout: Duration,
    pub max_lifetime: Duration,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            url: "postgres://shadow:shadow@localhost:5432/shadow_ot".to_string(),
            max_connections: 100,
            min_connections: 10,
            connect_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(600),
            max_lifetime: Duration::from_secs(1800),
        }
    }
}

/// Initialize database with migrations
pub async fn init(config: &DbConfig) -> Result<DatabasePool> {
    tracing::info!("Initializing database connection pool...");

    let pool = create_pool(config).await?;

    // Run migrations
    tracing::info!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool.pg)
        .await
        .map_err(|e| DbError::Migration(e.to_string()))?;

    tracing::info!("Database initialization complete");
    Ok(pool)
}
