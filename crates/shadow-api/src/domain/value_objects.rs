//! Value objects - Immutable domain primitives with validation
//!
//! Value objects encapsulate domain rules and provide type safety.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Gender representation for characters
///
/// Note: The underlying protocol uses `sex` (0=female, 1=male) for Tibia compatibility.
/// We expose `gender` in the API for modern, inclusive terminology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
}

impl Gender {
    /// Convert to protocol value (Tibia compatibility)
    /// 0 = female, 1 = male
    #[inline]
    pub fn to_protocol_value(self) -> i16 {
        match self {
            Gender::Male => 1,
            Gender::Female => 0,
        }
    }

    /// Create from protocol value
    pub fn from_protocol_value(value: i16) -> Option<Self> {
        match value {
            0 => Some(Gender::Female),
            1 => Some(Gender::Male),
            _ => None,
        }
    }

    /// Get the default look type for this gender
    pub fn default_look_type(self) -> i32 {
        match self {
            Gender::Male => 128,
            Gender::Female => 136,
        }
    }
}

impl Default for Gender {
    fn default() -> Self {
        Gender::Male
    }
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gender::Male => write!(f, "male"),
            Gender::Female => write!(f, "female"),
        }
    }
}

/// Character vocation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum Vocation {
    None = 0,
    Sorcerer = 1,
    Druid = 2,
    Paladin = 3,
    Knight = 4,
    MasterSorcerer = 5,
    ElderDruid = 6,
    RoyalPaladin = 7,
    EliteKnight = 8,
}

impl Vocation {
    pub fn from_i16(value: i16) -> Option<Self> {
        match value {
            0 => Some(Vocation::None),
            1 => Some(Vocation::Sorcerer),
            2 => Some(Vocation::Druid),
            3 => Some(Vocation::Paladin),
            4 => Some(Vocation::Knight),
            5 => Some(Vocation::MasterSorcerer),
            6 => Some(Vocation::ElderDruid),
            7 => Some(Vocation::RoyalPaladin),
            8 => Some(Vocation::EliteKnight),
            _ => None,
        }
    }

    pub fn to_i16(self) -> i16 {
        self as i16
    }

    pub fn is_promoted(self) -> bool {
        matches!(
            self,
            Vocation::MasterSorcerer
                | Vocation::ElderDruid
                | Vocation::RoyalPaladin
                | Vocation::EliteKnight
        )
    }

    pub fn base_vocation(self) -> Self {
        match self {
            Vocation::MasterSorcerer => Vocation::Sorcerer,
            Vocation::ElderDruid => Vocation::Druid,
            Vocation::RoyalPaladin => Vocation::Paladin,
            Vocation::EliteKnight => Vocation::Knight,
            other => other,
        }
    }
}

impl Default for Vocation {
    fn default() -> Self {
        Vocation::None
    }
}

/// Character name with validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CharacterName(String);

impl CharacterName {
    /// Create a new character name with validation
    pub fn new(name: impl Into<String>) -> Result<Self, &'static str> {
        let name = name.into();
        Self::validate(&name)?;
        Ok(Self(name))
    }

    /// Validate character name rules
    fn validate(name: &str) -> Result<(), &'static str> {
        if name.len() < 2 {
            return Err("Name must be at least 2 characters");
        }
        if name.len() > 29 {
            return Err("Name must be at most 29 characters");
        }
        if !name.chars().all(|c| c.is_ascii_alphabetic() || c == ' ' || c == '-' || c == '\'') {
            return Err("Name contains invalid characters");
        }
        if name.starts_with(' ') || name.ends_with(' ') {
            return Err("Name cannot start or end with space");
        }
        if name.contains("  ") {
            return Err("Name cannot contain consecutive spaces");
        }
        // Check for reserved words
        let lower = name.to_lowercase();
        if lower.contains("admin") || lower.contains("gamemaster") || lower.contains("gm ") {
            return Err("Name contains reserved words");
        }
        Ok(())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl std::fmt::Display for CharacterName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Email address with validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Email(String);

impl Email {
    pub fn new(email: impl Into<String>) -> Result<Self, &'static str> {
        let email = email.into().to_lowercase();
        Self::validate(&email)?;
        Ok(Self(email))
    }

    fn validate(email: &str) -> Result<(), &'static str> {
        if !email.contains('@') {
            return Err("Invalid email format");
        }
        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 {
            return Err("Invalid email format");
        }
        if parts[0].is_empty() || parts[1].is_empty() {
            return Err("Invalid email format");
        }
        if !parts[1].contains('.') {
            return Err("Invalid email domain");
        }
        Ok(())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod gender_tests {
        use super::*;

        #[test]
        fn test_gender_to_protocol_value() {
            assert_eq!(Gender::Male.to_protocol_value(), 1);
            assert_eq!(Gender::Female.to_protocol_value(), 0);
        }

        #[test]
        fn test_gender_from_protocol_value() {
            assert_eq!(Gender::from_protocol_value(0), Some(Gender::Female));
            assert_eq!(Gender::from_protocol_value(1), Some(Gender::Male));
            assert_eq!(Gender::from_protocol_value(2), None);
            assert_eq!(Gender::from_protocol_value(-1), None);
        }

        #[test]
        fn test_gender_default_look_type() {
            assert_eq!(Gender::Male.default_look_type(), 128);
            assert_eq!(Gender::Female.default_look_type(), 136);
        }

        #[test]
        fn test_gender_serialization() {
            assert_eq!(serde_json::to_string(&Gender::Male).unwrap(), "\"male\"");
            assert_eq!(serde_json::to_string(&Gender::Female).unwrap(), "\"female\"");
        }

        #[test]
        fn test_gender_deserialization() {
            assert_eq!(serde_json::from_str::<Gender>("\"male\"").unwrap(), Gender::Male);
            assert_eq!(serde_json::from_str::<Gender>("\"female\"").unwrap(), Gender::Female);
        }
    }

    mod vocation_tests {
        use super::*;

        #[test]
        fn test_vocation_from_i16() {
            assert_eq!(Vocation::from_i16(0), Some(Vocation::None));
            assert_eq!(Vocation::from_i16(4), Some(Vocation::Knight));
            assert_eq!(Vocation::from_i16(8), Some(Vocation::EliteKnight));
            assert_eq!(Vocation::from_i16(99), None);
        }

        #[test]
        fn test_vocation_is_promoted() {
            assert!(!Vocation::Knight.is_promoted());
            assert!(Vocation::EliteKnight.is_promoted());
        }

        #[test]
        fn test_vocation_base_vocation() {
            assert_eq!(Vocation::EliteKnight.base_vocation(), Vocation::Knight);
            assert_eq!(Vocation::Knight.base_vocation(), Vocation::Knight);
        }
    }

    mod character_name_tests {
        use super::*;

        #[test]
        fn test_valid_names() {
            assert!(CharacterName::new("John").is_ok());
            assert!(CharacterName::new("John Doe").is_ok());
            assert!(CharacterName::new("O'Brien").is_ok());
            assert!(CharacterName::new("Mary-Jane").is_ok());
        }

        #[test]
        fn test_invalid_names() {
            assert!(CharacterName::new("A").is_err()); // too short
            assert!(CharacterName::new(" John").is_err()); // starts with space
            assert!(CharacterName::new("John  Doe").is_err()); // consecutive spaces
            assert!(CharacterName::new("Admin123").is_err()); // invalid chars
            assert!(CharacterName::new("GM John").is_err()); // reserved
        }
    }

    mod email_tests {
        use super::*;

        #[test]
        fn test_valid_emails() {
            assert!(Email::new("test@example.com").is_ok());
            assert!(Email::new("user.name@domain.co.uk").is_ok());
        }

        #[test]
        fn test_invalid_emails() {
            assert!(Email::new("invalid").is_err());
            assert!(Email::new("@domain.com").is_err());
            assert!(Email::new("user@").is_err());
            assert!(Email::new("user@domain").is_err());
        }

        #[test]
        fn test_email_lowercase() {
            let email = Email::new("TEST@EXAMPLE.COM").unwrap();
            assert_eq!(email.as_str(), "test@example.com");
        }
    }
}
