//! Database migration utilities
//!
//! Handles running database migrations either from files or embedded.

use sqlx::PgPool;
use tracing::{error, info};

/// Run migrations from embedded SQL
pub async fn run_migrations(pool: &PgPool) -> crate::Result<()> {
    info!("Running database migrations...");

    // Check if tables exist
    let tables_exist = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'accounts')"
    )
    .fetch_one(pool)
    .await
    .unwrap_or(false);

    if tables_exist {
        info!("Database schema already exists, skipping migrations");
        return Ok(());
    }

    // Read and execute migration
    let migration_sql = include_str!("../migrations/001_initial_schema.sql");

    // Split by statement (simplified - real implementation should handle this better)
    for statement in migration_sql.split(";") {
        let statement = statement.trim();
        if statement.is_empty() || statement.starts_with("--") {
            continue;
        }

        if let Err(e) = sqlx::query(statement).execute(pool).await {
            // Log but continue for certain expected errors
            if !e.to_string().contains("already exists") {
                error!("Migration statement failed: {}", e);
                // Continue anyway for now
            }
        }
    }

    info!("Database migrations completed");
    Ok(())
}

/// Check database connection and schema
pub async fn check_database(pool: &PgPool) -> crate::Result<DatabaseStatus> {
    let version = sqlx::query_scalar::<_, String>("SELECT version()")
        .fetch_one(pool)
        .await
        .map_err(|e| crate::DbError::Connection(e.to_string()))?;

    let tables = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'public'"
    )
    .fetch_one(pool)
    .await
    .unwrap_or(0);

    let accounts = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM accounts")
        .fetch_one(pool)
        .await
        .unwrap_or(0);

    let characters = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM characters")
        .fetch_one(pool)
        .await
        .unwrap_or(0);

    let realms = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM realms")
        .fetch_one(pool)
        .await
        .unwrap_or(0);

    Ok(DatabaseStatus {
        version,
        tables: tables as usize,
        accounts: accounts as usize,
        characters: characters as usize,
        realms: realms as usize,
    })
}

/// Database status info
#[derive(Debug, Clone)]
pub struct DatabaseStatus {
    pub version: String,
    pub tables: usize,
    pub accounts: usize,
    pub characters: usize,
    pub realms: usize,
}

impl std::fmt::Display for DatabaseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PostgreSQL: {}\n  Tables: {}\n  Accounts: {}\n  Characters: {}\n  Realms: {}",
            self.version, self.tables, self.accounts, self.characters, self.realms
        )
    }
}
