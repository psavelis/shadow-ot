//! Domain layer tests
//!
//! Comprehensive tests for domain entities, value objects, and business rules.

#[cfg(test)]
mod value_objects_tests {
    use crate::domain::{Gender, Vocation};

    #[test]
    fn test_gender_protocol_conversion() {
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
    fn test_gender_display() {
        assert_eq!(format!("{}", Gender::Male), "male");
        assert_eq!(format!("{}", Gender::Female), "female");
    }

    #[test]
    fn test_gender_default() {
        assert_eq!(Gender::default(), Gender::Male);
    }

    #[test]
    fn test_vocation_from_i16() {
        assert_eq!(Vocation::from_i16(0), Some(Vocation::None));
        assert_eq!(Vocation::from_i16(1), Some(Vocation::Sorcerer));
        assert_eq!(Vocation::from_i16(2), Some(Vocation::Druid));
        assert_eq!(Vocation::from_i16(3), Some(Vocation::Paladin));
        assert_eq!(Vocation::from_i16(4), Some(Vocation::Knight));
        assert_eq!(Vocation::from_i16(5), Some(Vocation::MasterSorcerer));
        assert_eq!(Vocation::from_i16(6), Some(Vocation::ElderDruid));
        assert_eq!(Vocation::from_i16(7), Some(Vocation::RoyalPaladin));
        assert_eq!(Vocation::from_i16(8), Some(Vocation::EliteKnight));
        assert_eq!(Vocation::from_i16(99), None);
    }

    #[test]
    fn test_vocation_to_i16() {
        assert_eq!(Vocation::None.to_i16(), 0);
        assert_eq!(Vocation::Knight.to_i16(), 4);
        assert_eq!(Vocation::EliteKnight.to_i16(), 8);
    }

    #[test]
    fn test_vocation_is_promoted() {
        assert!(!Vocation::None.is_promoted());
        assert!(!Vocation::Knight.is_promoted());
        assert!(!Vocation::Sorcerer.is_promoted());
        assert!(Vocation::MasterSorcerer.is_promoted());
        assert!(Vocation::ElderDruid.is_promoted());
        assert!(Vocation::RoyalPaladin.is_promoted());
        assert!(Vocation::EliteKnight.is_promoted());
    }

    #[test]
    fn test_vocation_base_vocation() {
        assert_eq!(Vocation::MasterSorcerer.base_vocation(), Vocation::Sorcerer);
        assert_eq!(Vocation::ElderDruid.base_vocation(), Vocation::Druid);
        assert_eq!(Vocation::RoyalPaladin.base_vocation(), Vocation::Paladin);
        assert_eq!(Vocation::EliteKnight.base_vocation(), Vocation::Knight);
        assert_eq!(Vocation::Knight.base_vocation(), Vocation::Knight);
    }
}

#[cfg(test)]
mod entities_tests {
    use crate::domain::{Account, AccountId, Character, CharacterId, Gender, Vocation};
    use chrono::{Duration, Utc};

    fn create_test_character() -> Character {
        Character {
            id: CharacterId::new(1),
            account_id: 100,
            realm_id: 1,
            name: "TestKnight".to_string(),
            gender: Gender::Male,
            vocation: Vocation::Knight,
            level: 50,
            experience: 1_000_000,
            health: 500,
            max_health: 500,
            mana: 100,
            max_mana: 100,
            look_type: 128,
            look_head: 0,
            look_body: 0,
            look_legs: 0,
            look_feet: 0,
            look_addons: 0,
            town_id: 1,
            balance: 1000,
            bank_balance: 5000,
            online: false,
        }
    }

    fn create_test_account() -> Account {
        Account {
            id: AccountId::new(1),
            email: "test@example.com".to_string(),
            premium_until: None,
            coins: 100,
        }
    }

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
    fn test_character_can_be_deleted_when_offline() {
        let mut character = create_test_character();
        character.online = false;
        character.level = 50;
        assert!(character.can_be_deleted());
    }

    #[test]
    fn test_character_cannot_be_deleted_when_online() {
        let mut character = create_test_character();
        character.online = true;
        assert!(!character.can_be_deleted());
    }

    #[test]
    fn test_character_cannot_be_deleted_high_level() {
        let mut character = create_test_character();
        character.level = 150;
        character.online = false;
        assert!(!character.can_be_deleted());
    }

    #[test]
    fn test_character_total_wealth() {
        let character = create_test_character();
        assert_eq!(character.total_wealth(), 6000);
    }

    #[test]
    fn test_character_is_rookie() {
        let mut character = create_test_character();
        character.vocation = Vocation::None;
        assert!(character.is_rookie());

        character.vocation = Vocation::Knight;
        assert!(!character.is_rookie());
    }

    #[test]
    fn test_account_is_premium_with_future_date() {
        let mut account = create_test_account();
        account.premium_until = Some(Utc::now() + Duration::days(30));
        assert!(account.is_premium());
    }

    #[test]
    fn test_account_is_not_premium_with_past_date() {
        let mut account = create_test_account();
        account.premium_until = Some(Utc::now() - Duration::days(1));
        assert!(!account.is_premium());
    }

    #[test]
    fn test_account_is_not_premium_with_none() {
        let account = create_test_account();
        assert!(!account.is_premium());
    }

    #[test]
    fn test_account_has_coins() {
        let account = create_test_account();
        assert!(account.has_coins(50));
        assert!(account.has_coins(100));
        assert!(!account.has_coins(101));
    }

    #[test]
    fn test_account_can_create_character() {
        let account = create_test_account();
        assert!(account.can_create_character(0, 5));
        assert!(account.can_create_character(4, 5));
        assert!(!account.can_create_character(5, 5));
        assert!(!account.can_create_character(6, 5));
    }
}

#[cfg(test)]
mod errors_tests {
    use crate::domain::DomainError;

    #[test]
    fn test_error_display() {
        assert_eq!(
            DomainError::NotFound("Test".to_string()).to_string(),
            "Not found: Test"
        );
        assert_eq!(
            DomainError::CharacterNotFound.to_string(),
            "Character not found"
        );
        assert_eq!(
            DomainError::AccountNotFound.to_string(),
            "Account not found"
        );
        assert_eq!(
            DomainError::CharacterLimitReached.to_string(),
            "Character limit reached"
        );
    }

    #[test]
    fn test_error_equality() {
        assert_eq!(DomainError::Unauthorized, DomainError::Unauthorized);
        assert_eq!(DomainError::CharacterNotFound, DomainError::CharacterNotFound);
        assert_ne!(DomainError::Unauthorized, DomainError::Forbidden);
    }
}
