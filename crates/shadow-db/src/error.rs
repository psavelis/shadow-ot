//! Database error types

use thiserror::Error;

pub type Result<T> = std::result::Result<T, DbError>;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database connection error: {0}")]
    Connection(String),

    #[error("Query error: {0}")]
    Query(String),

    #[error("Record not found: {0}")]
    NotFound(String),

    #[error("Duplicate record: {0}")]
    Duplicate(String),

    #[error("Migration error: {0}")]
    Migration(String),

    #[error("Cache error: {0}")]
    Cache(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Transaction error: {0}")]
    Transaction(String),

    #[error("SQL error: {0}")]
    Sql(#[from] sqlx::Error),
}

impl From<redis::RedisError> for DbError {
    fn from(err: redis::RedisError) -> Self {
        DbError::Cache(err.to_string())
    }
}
