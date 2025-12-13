//! Domain errors - Business rule violations
//!
//! These errors represent violations of domain invariants and business rules.
//! They are independent of infrastructure concerns.

use std::fmt;

/// Domain-level errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainError {
    /// Entity not found
    NotFound(String),
    /// Validation failed
    ValidationFailed(String),
    /// Business rule violated
    BusinessRuleViolation(String),
    /// Conflict with existing data
    Conflict(String),
    /// Authorization failed
    Unauthorized,
    /// Forbidden action
    Forbidden,
    /// Invalid character name
    InvalidCharacterName(String),
    /// Invalid email
    InvalidEmail(String),
    /// Invalid password
    InvalidPassword(String),
    /// Character not found
    CharacterNotFound,
    /// Account not found
    AccountNotFound,
    /// Realm not found
    RealmNotFound,
    /// Character limit reached
    CharacterLimitReached,
    /// Name already taken
    NameAlreadyTaken,
    /// Email already taken
    EmailAlreadyTaken,
    /// Invalid token
    InvalidToken,
    /// Password mismatch
    PasswordMismatch,
    /// Character is online
    CharacterOnline,
    /// Database error (infrastructure leak, but necessary)
    DatabaseError(String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::NotFound(msg) => write!(f, "Not found: {}", msg),
            DomainError::ValidationFailed(msg) => write!(f, "Validation failed: {}", msg),
            DomainError::BusinessRuleViolation(msg) => write!(f, "Business rule violation: {}", msg),
            DomainError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            DomainError::Unauthorized => write!(f, "Unauthorized"),
            DomainError::Forbidden => write!(f, "Forbidden"),
            DomainError::InvalidCharacterName(msg) => write!(f, "Invalid character name: {}", msg),
            DomainError::InvalidEmail(msg) => write!(f, "Invalid email: {}", msg),
            DomainError::InvalidPassword(msg) => write!(f, "Invalid password: {}", msg),
            DomainError::CharacterNotFound => write!(f, "Character not found"),
            DomainError::AccountNotFound => write!(f, "Account not found"),
            DomainError::RealmNotFound => write!(f, "Realm not found"),
            DomainError::CharacterLimitReached => write!(f, "Character limit reached"),
            DomainError::NameAlreadyTaken => write!(f, "Name already taken"),
            DomainError::EmailAlreadyTaken => write!(f, "Email already taken"),
            DomainError::InvalidToken => write!(f, "Invalid token"),
            DomainError::PasswordMismatch => write!(f, "Password mismatch"),
            DomainError::CharacterOnline => write!(f, "Character is currently online"),
            DomainError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl std::error::Error for DomainError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = DomainError::NotFound("Character".to_string());
        assert_eq!(err.to_string(), "Not found: Character");

        let err = DomainError::ValidationFailed("Name too short".to_string());
        assert_eq!(err.to_string(), "Validation failed: Name too short");

        let err = DomainError::InvalidCharacterName("Too short".to_string());
        assert_eq!(err.to_string(), "Invalid character name: Too short");
    }

    #[test]
    fn test_error_equality() {
        let err1 = DomainError::Unauthorized;
        let err2 = DomainError::Unauthorized;
        assert_eq!(err1, err2);

        let err3 = DomainError::CharacterNotFound;
        let err4 = DomainError::CharacterNotFound;
        assert_eq!(err3, err4);
    }

    #[test]
    fn test_specific_errors() {
        assert_eq!(
            DomainError::CharacterLimitReached.to_string(),
            "Character limit reached"
        );
        assert_eq!(
            DomainError::NameAlreadyTaken.to_string(),
            "Name already taken"
        );
    }
}
