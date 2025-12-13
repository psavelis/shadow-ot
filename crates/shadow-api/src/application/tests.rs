//! Application layer tests
//!
//! Tests for commands, queries, and ports.

#[cfg(test)]
mod commands_tests {
    use crate::application::commands::CreateCharacterCommand;
    use crate::domain::{DomainError, Gender, Vocation};

    #[test]
    fn test_create_character_command_valid() {
        let cmd = CreateCharacterCommand::new(
            "Knight Hero".to_string(),
            1,
            Gender::Male,
            Vocation::Knight,
        );
        assert!(cmd.validate_business_rules().is_ok());
    }

    #[test]
    fn test_create_character_command_name_too_short() {
        let cmd = CreateCharacterCommand::new(
            "Ab".to_string(),
            1,
            Gender::Male,
            Vocation::Knight,
        );
        let result = cmd.validate_business_rules();
        assert!(matches!(result, Err(DomainError::InvalidCharacterName(_))));
    }

    #[test]
    fn test_create_character_command_name_too_long() {
        let cmd = CreateCharacterCommand::new(
            "This Name Is Way Too Long".to_string(),
            1,
            Gender::Male,
            Vocation::Knight,
        );
        let result = cmd.validate_business_rules();
        assert!(matches!(result, Err(DomainError::InvalidCharacterName(_))));
    }

    #[test]
    fn test_create_character_command_consecutive_spaces() {
        let cmd = CreateCharacterCommand::new(
            "Knight  Hero".to_string(),
            1,
            Gender::Male,
            Vocation::Knight,
        );
        let result = cmd.validate_business_rules();
        assert!(matches!(result, Err(DomainError::InvalidCharacterName(_))));
    }

    #[test]
    fn test_create_character_command_starts_with_number() {
        let cmd = CreateCharacterCommand::new(
            "123Knight".to_string(),
            1,
            Gender::Male,
            Vocation::Knight,
        );
        let result = cmd.validate_business_rules();
        assert!(matches!(result, Err(DomainError::InvalidCharacterName(_))));
    }

    #[test]
    fn test_create_character_command_special_characters() {
        let cmd = CreateCharacterCommand::new(
            "Knight@Hero".to_string(),
            1,
            Gender::Male,
            Vocation::Knight,
        );
        let result = cmd.validate_business_rules();
        assert!(matches!(result, Err(DomainError::InvalidCharacterName(_))));
    }

    #[test]
    fn test_create_character_command_single_word() {
        let cmd = CreateCharacterCommand::new(
            "Knight".to_string(),
            1,
            Gender::Male,
            Vocation::Knight,
        );
        assert!(cmd.validate_business_rules().is_ok());
    }

    #[test]
    fn test_create_character_command_with_space() {
        let cmd = CreateCharacterCommand::new(
            "Dark Knight".to_string(),
            1,
            Gender::Female,
            Vocation::Knight,
        );
        assert!(cmd.validate_business_rules().is_ok());
    }
}

#[cfg(test)]
mod queries_tests {
    use crate::application::queries::{PaginatedResponse, PaginationQuery};

    #[test]
    fn test_pagination_offset_first_page() {
        let query = PaginationQuery::new(1, 20);
        assert_eq!(query.offset(), 0);
    }

    #[test]
    fn test_pagination_offset_second_page() {
        let query = PaginationQuery::new(2, 20);
        assert_eq!(query.offset(), 20);
    }

    #[test]
    fn test_pagination_offset_custom_page_size() {
        let query = PaginationQuery::new(3, 10);
        assert_eq!(query.offset(), 20);
    }

    #[test]
    fn test_pagination_limit_normal() {
        let query = PaginationQuery::new(1, 20);
        assert_eq!(query.limit(), 20);
    }

    #[test]
    fn test_pagination_limit_clamped_high() {
        let query = PaginationQuery::new(1, 200);
        assert_eq!(query.limit(), 100);
    }

    #[test]
    fn test_pagination_limit_clamped_low() {
        let query = PaginationQuery::new(1, 0);
        assert_eq!(query.limit(), 1);
    }

    #[test]
    fn test_pagination_negative_page() {
        let query = PaginationQuery::new(-1, 20);
        assert_eq!(query.offset(), 0);
    }

    #[test]
    fn test_pagination_default() {
        let query = PaginationQuery::default();
        assert_eq!(query.page, 1);
        assert_eq!(query.per_page, 20);
    }

    #[test]
    fn test_paginated_response_total_pages() {
        let query = PaginationQuery::new(1, 10);
        let response: PaginatedResponse<i32> = PaginatedResponse::new(vec![1, 2, 3], 25, &query);
        assert_eq!(response.total_pages, 3);
        assert_eq!(response.total, 25);
        assert_eq!(response.items.len(), 3);
    }

    #[test]
    fn test_paginated_response_exact_division() {
        let query = PaginationQuery::new(1, 10);
        let response: PaginatedResponse<i32> = PaginatedResponse::new(vec![], 20, &query);
        assert_eq!(response.total_pages, 2);
    }

    #[test]
    fn test_paginated_response_empty() {
        let query = PaginationQuery::new(1, 10);
        let response: PaginatedResponse<i32> = PaginatedResponse::new(vec![], 0, &query);
        assert_eq!(response.total_pages, 0);
    }
}
