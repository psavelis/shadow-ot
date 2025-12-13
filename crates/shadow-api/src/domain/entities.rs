//! Domain entities - Core business objects with identity
//!
//! Entities have a unique identity and lifecycle. They encapsulate
//! business rules and invariants.

use super::value_objects::{CharacterName, Gender, Vocation};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Strongly-typed Character ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CharacterId(i32);

impl CharacterId {
    pub fn new(id: i32) -> Self {
        Self(id)
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}

/// Strongly-typed Account ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AccountId(i32);

impl AccountId {
    pub fn new(id: i32) -> Self {
        Self(id)
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}

/// Character entity - represents a player's in-game character
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Character {
    pub id: CharacterId,
    pub account_id: i32,
    pub realm_id: i32,
    pub name: String,
    pub gender: Gender,
    pub vocation: Vocation,
    pub level: i32,
    pub experience: i64,
    pub health: i32,
    pub max_health: i32,
    pub mana: i32,
    pub max_mana: i32,
    pub look_type: i32,
    pub look_head: i16,
    pub look_body: i16,
    pub look_legs: i16,
    pub look_feet: i16,
    pub look_addons: i16,
    pub town_id: i32,
    pub balance: i64,
    pub bank_balance: i64,
    pub online: bool,
}

impl Character {
    /// Check if character can be deleted (business rule)
    pub fn can_be_deleted(&self) -> bool {
        !self.online && self.level < 100 // Example rule
    }

    /// Calculate character's total wealth
    pub fn total_wealth(&self) -> i64 {
        self.balance + self.bank_balance
    }

    /// Check if character is a rookie
    pub fn is_rookie(&self) -> bool {
        self.vocation == Vocation::None
    }
}

/// Command to create a new character
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateCharacterCommand {
    pub name: String,
    pub gender: Gender,
    pub vocation: Vocation,
    pub realm_id: i32,
}

impl CreateCharacterCommand {
    /// Validate the command
    pub fn validate(&self) -> Result<CharacterName, &'static str> {
        CharacterName::new(&self.name)
    }
}

/// Account entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: AccountId,
    pub email: String,
    pub premium_until: Option<DateTime<Utc>>,
    pub coins: i32,
}

impl Account {
    /// Check if account has active premium
    pub fn is_premium(&self) -> bool {
        self.premium_until
            .map(|until| until > Utc::now())
            .unwrap_or(false)
    }

    /// Check if account has enough coins
    pub fn has_coins(&self, amount: i32) -> bool {
        self.coins >= amount
    }

    /// Check if account can create more characters
    pub fn can_create_character(&self, current_count: i64, max_per_account: i64) -> bool {
        current_count < max_per_account
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_id() {
        let id = CharacterId::new(42);
        assert_eq!(id.value(), 42);
    }

    #[test]
    fn test_account_id() {
        let id = AccountId::new(100);
        assert_eq!(id.value(), 100);
    }

    #[test]
    fn test_create_character_command_validation() {
        let valid = CreateCharacterCommand {
            name: "Test Character".to_string(),
            gender: Gender::Male,
            vocation: Vocation::Knight,
            realm_id: 1,
        };
        assert!(valid.validate().is_ok());

        let invalid = CreateCharacterCommand {
            name: "A".to_string(), // too short
            gender: Gender::Female,
            vocation: Vocation::Druid,
            realm_id: 1,
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_account_is_premium() {
        let account = Account {
            id: AccountId::new(1),
            email: "test@example.com".to_string(),
            premium_until: Some(Utc::now() + chrono::Duration::days(30)),
            coins: 100,
        };
        assert!(account.is_premium());

        let expired = Account {
            id: AccountId::new(2),
            email: "expired@example.com".to_string(),
            premium_until: Some(Utc::now() - chrono::Duration::days(1)),
            coins: 50,
        };
        assert!(!expired.is_premium());

        let no_premium = Account {
            id: AccountId::new(3),
            email: "free@example.com".to_string(),
            premium_until: None,
            coins: 0,
        };
        assert!(!no_premium.is_premium());
    }

    #[test]
    fn test_account_has_coins() {
        let account = Account {
            id: AccountId::new(1),
            email: "test@example.com".to_string(),
            premium_until: None,
            coins: 100,
        };
        
        assert!(account.has_coins(50));
        assert!(account.has_coins(100));
        assert!(!account.has_coins(101));
    }
}
