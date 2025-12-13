//! Commands - Operations that mutate state
//!
//! Commands represent intent to change the system state.
//! They are validated and executed by command handlers.

use crate::domain::{DomainError, Gender, Vocation};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// Command to create a new character
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateCharacterCommand {
    /// Character name (3-14 alphanumeric chars)
    #[validate(length(min = 3, max = 14))]
    pub name: String,

    /// Realm ID
    pub realm_id: i32,

    /// Character gender
    #[serde(alias = "sex")]
    pub gender: Gender,

    /// Starting vocation
    pub vocation: Vocation,

    /// Character appearance look type
    #[serde(default = "default_look_type")]
    pub look_type: i32,

    /// Starting town ID
    #[serde(default = "default_town_id")]
    pub town_id: i32,
}

fn default_look_type() -> i32 {
    128
}

fn default_town_id() -> i32 {
    1
}

impl CreateCharacterCommand {
    pub fn new(
        name: String,
        realm_id: i32,
        gender: Gender,
        vocation: Vocation,
    ) -> Self {
        Self {
            name,
            realm_id,
            gender,
            vocation,
            look_type: default_look_type(),
            town_id: default_town_id(),
        }
    }

    /// Validate command according to business rules
    pub fn validate_business_rules(&self) -> Result<(), DomainError> {
        // Name validation
        if self.name.len() < 3 || self.name.len() > 14 {
            return Err(DomainError::InvalidCharacterName(
                "Character name must be 3-14 characters".to_string(),
            ));
        }

        // No consecutive spaces
        if self.name.contains("  ") {
            return Err(DomainError::InvalidCharacterName(
                "Name cannot contain consecutive spaces".to_string(),
            ));
        }

        // Must start with letter
        if !self.name.chars().next().map_or(false, |c| c.is_alphabetic()) {
            return Err(DomainError::InvalidCharacterName(
                "Name must start with a letter".to_string(),
            ));
        }

        // Only letters and spaces
        if !self.name.chars().all(|c| c.is_alphabetic() || c == ' ') {
            return Err(DomainError::InvalidCharacterName(
                "Name can only contain letters and spaces".to_string(),
            ));
        }

        Ok(())
    }
}

/// Command to delete a character
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCharacterCommand {
    pub character_id: i32,
}

impl DeleteCharacterCommand {
    pub fn new(character_id: i32) -> Self {
        Self { character_id }
    }
}

/// Command to update account email
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEmailCommand {
    #[validate(email)]
    pub email: String,
}

impl UpdateEmailCommand {
    pub fn new(email: String) -> Self {
        Self { email }
    }
}

/// Command to change password
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordCommand {
    pub current_password: String,

    #[validate(length(min = 8, max = 64))]
    pub new_password: String,
}

impl ChangePasswordCommand {
    pub fn new(current_password: String, new_password: String) -> Self {
        Self {
            current_password,
            new_password,
        }
    }
}

/// Command to login
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginCommand {
    #[validate(email)]
    pub email: String,

    pub password: String,
}

/// Command to register
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterCommand {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8, max = 64))]
    pub password: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_character_valid_name() {
        let cmd = CreateCharacterCommand::new(
            "Knight Hero".to_string(),
            1,
            Gender::Male,
            Vocation::Knight,
        );
        assert!(cmd.validate_business_rules().is_ok());
    }

    #[test]
    fn test_create_character_name_too_short() {
        let cmd = CreateCharacterCommand::new(
            "Ab".to_string(),
            1,
            Gender::Male,
            Vocation::Knight,
        );
        assert!(matches!(
            cmd.validate_business_rules(),
            Err(DomainError::InvalidCharacterName(_))
        ));
    }

    #[test]
    fn test_create_character_consecutive_spaces() {
        let cmd = CreateCharacterCommand::new(
            "Knight  Hero".to_string(),
            1,
            Gender::Male,
            Vocation::Knight,
        );
        assert!(matches!(
            cmd.validate_business_rules(),
            Err(DomainError::InvalidCharacterName(_))
        ));
    }

    #[test]
    fn test_create_character_starts_with_number() {
        let cmd = CreateCharacterCommand::new(
            "123Knight".to_string(),
            1,
            Gender::Male,
            Vocation::Knight,
        );
        assert!(matches!(
            cmd.validate_business_rules(),
            Err(DomainError::InvalidCharacterName(_))
        ));
    }

    #[test]
    fn test_create_character_special_chars() {
        let cmd = CreateCharacterCommand::new(
            "Knight@Hero".to_string(),
            1,
            Gender::Male,
            Vocation::Knight,
        );
        assert!(matches!(
            cmd.validate_business_rules(),
            Err(DomainError::InvalidCharacterName(_))
        ));
    }
}
