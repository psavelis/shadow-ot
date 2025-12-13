//! Output ports (Driving ports) - Infrastructure interfaces
//!
//! These ports define what the application needs from infrastructure.
//! They are implemented by secondary adapters (database, cache, etc.)

use crate::domain::{Character, DomainError, Gender, Vocation};
use async_trait::async_trait;

/// Character repository - persistence abstraction
#[async_trait]
pub trait CharacterRepository: Send + Sync {
    /// Find all characters for an account
    async fn find_by_account_id(&self, account_id: i32) -> Result<Vec<Character>, DomainError>;

    /// Find a character by ID
    async fn find_by_id(&self, id: i32) -> Result<Option<Character>, DomainError>;

    /// Check if a character name is available
    async fn is_name_available(&self, name: &str) -> Result<bool, DomainError>;

    /// Count characters for an account
    async fn count_by_account_id(&self, account_id: i32) -> Result<i64, DomainError>;

    /// Create a new character
    async fn create(
        &self,
        account_id: i32,
        realm_id: i32,
        name: &str,
        gender: Gender,
        vocation: Vocation,
        look_type: i32,
        town_id: i32,
    ) -> Result<i32, DomainError>;

    /// Schedule character for deletion
    async fn schedule_deletion(&self, id: i32, days: i32) -> Result<(), DomainError>;

    /// Check if character is online
    async fn is_online(&self, id: i32) -> Result<Option<bool>, DomainError>;

    /// Verify character ownership
    async fn verify_ownership(&self, character_id: i32, account_id: i32) -> Result<bool, DomainError>;
}

/// Account repository - persistence abstraction
#[async_trait]
pub trait AccountRepository: Send + Sync {
    /// Find account by ID
    async fn find_by_id(&self, id: i32) -> Result<Option<crate::domain::Account>, DomainError>;

    /// Find account by email
    async fn find_by_email(&self, email: &str) -> Result<Option<crate::domain::Account>, DomainError>;

    /// Check if email is taken
    async fn is_email_taken(&self, email: &str, exclude_id: Option<i32>) -> Result<bool, DomainError>;

    /// Update account email
    async fn update_email(&self, id: i32, email: &str) -> Result<(), DomainError>;

    /// Get password hash for account
    async fn get_password_hash(&self, id: i32) -> Result<String, DomainError>;

    /// Update password hash
    async fn update_password(&self, id: i32, hash: &str, salt: &str) -> Result<(), DomainError>;
}

/// Realm repository - persistence abstraction
#[async_trait]
pub trait RealmRepository: Send + Sync {
    /// Check if realm exists
    async fn exists(&self, id: i32) -> Result<bool, DomainError>;

    /// Get realm by ID
    async fn find_by_id(&self, id: i32) -> Result<Option<Realm>, DomainError>;

    /// Get online count for realm
    async fn get_online_count(&self, id: i32) -> Result<i64, DomainError>;
}

/// Realm entity (simplified for repository)
#[derive(Debug, Clone)]
pub struct Realm {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

/// Password hasher - security abstraction
pub trait PasswordHasher: Send + Sync {
    /// Hash a password
    fn hash(&self, password: &str) -> Result<(String, String), DomainError>;

    /// Verify a password against a hash
    fn verify(&self, password: &str, hash: &str) -> Result<bool, DomainError>;
}

/// Token generator - authentication abstraction
#[async_trait]
pub trait TokenGenerator: Send + Sync {
    /// Generate access token
    fn generate_access_token(&self, account_id: i32) -> Result<String, DomainError>;

    /// Generate refresh token
    fn generate_refresh_token(&self, account_id: i32) -> Result<String, DomainError>;

    /// Validate and decode refresh token
    fn validate_refresh_token(&self, token: &str) -> Result<i32, DomainError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Verify traits are object-safe
    fn _assert_char_repo_object_safe(_: &dyn CharacterRepository) {}
    fn _assert_account_repo_object_safe(_: &dyn AccountRepository) {}
    fn _assert_realm_repo_object_safe(_: &dyn RealmRepository) {}
    fn _assert_hasher_object_safe(_: &dyn PasswordHasher) {}
    fn _assert_token_gen_object_safe(_: &dyn TokenGenerator) {}
}
