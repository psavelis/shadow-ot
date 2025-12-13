//! Core error types for Shadow OT

use thiserror::Error;

/// Result type alias for core operations
pub type Result<T> = std::result::Result<T, CoreError>;

/// Core error types
#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Database error: {0}")]
    Database(#[from] shadow_db::DbError),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Realm error: {0}")]
    Realm(String),

    #[error("Player not found: {0}")]
    PlayerNotFound(uuid::Uuid),

    #[error("Character not found: {0}")]
    CharacterNotFound(uuid::Uuid),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Resource exhausted: {0}")]
    ResourceExhausted(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Scripting error: {0}")]
    Scripting(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl CoreError {
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            CoreError::Database(_) | CoreError::Network(_) | CoreError::ResourceExhausted(_)
        )
    }
}
