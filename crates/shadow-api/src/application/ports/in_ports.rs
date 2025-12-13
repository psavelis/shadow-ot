//! Input ports (Driven ports) - Use case interfaces
//!
//! These ports define the application's capabilities that can be
//! invoked by primary adapters (HTTP handlers, CLI, etc.)

use crate::application::commands::CreateCharacterCommand;
use crate::domain::{Character, DomainError, Gender};
use async_trait::async_trait;

/// Character management use cases
#[async_trait]
pub trait CharacterService: Send + Sync {
    /// List all characters for an account
    async fn list_characters(&self, account_id: i32) -> Result<Vec<Character>, DomainError>;

    /// Get a character by ID
    async fn get_character(&self, id: i32) -> Result<Character, DomainError>;

    /// Create a new character
    async fn create_character(
        &self,
        account_id: i32,
        command: CreateCharacterCommand,
    ) -> Result<Character, DomainError>;

    /// Delete a character (soft delete with grace period)
    async fn delete_character(&self, account_id: i32, character_id: i32) -> Result<i32, DomainError>;

    /// Check if a character is online
    async fn is_online(&self, character_id: i32) -> Result<bool, DomainError>;
}

/// Account management use cases
#[async_trait]
pub trait AccountService: Send + Sync {
    /// Get account by ID
    async fn get_account(&self, account_id: i32) -> Result<crate::domain::Account, DomainError>;

    /// Update account email
    async fn update_email(&self, account_id: i32, new_email: &str) -> Result<(), DomainError>;

    /// Change account password
    async fn change_password(
        &self,
        account_id: i32,
        current_password: &str,
        new_password: &str,
    ) -> Result<(), DomainError>;
}

/// Authentication use cases
#[async_trait]
pub trait AuthService: Send + Sync {
    /// Authenticate with email and password
    async fn authenticate(&self, email: &str, password: &str) -> Result<AuthResult, DomainError>;

    /// Register a new account
    async fn register(&self, email: &str, password: &str) -> Result<AuthResult, DomainError>;

    /// Refresh access token
    async fn refresh_token(&self, refresh_token: &str) -> Result<AuthResult, DomainError>;

    /// Logout (invalidate tokens)
    async fn logout(&self, account_id: i32) -> Result<(), DomainError>;
}

/// Authentication result
#[derive(Debug, Clone)]
pub struct AuthResult {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub account_id: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Verify traits are object-safe
    fn _assert_object_safe(_: &dyn CharacterService) {}
    fn _assert_account_object_safe(_: &dyn AccountService) {}
    fn _assert_auth_object_safe(_: &dyn AuthService) {}
}
