//! Queries - Operations that read state
//!
//! Queries represent intent to retrieve information without side effects.
//! They follow CQRS (Command Query Responsibility Segregation).

use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

/// Query to get characters for an account
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetCharactersQuery {
    /// Optional realm filter
    pub realm_id: Option<i32>,
}

impl GetCharactersQuery {
    pub fn new(realm_id: Option<i32>) -> Self {
        Self { realm_id }
    }

    pub fn all() -> Self {
        Self { realm_id: None }
    }
}

/// Query to get a specific character
#[derive(Debug, Clone, Serialize, Deserialize, IntoParams)]
pub struct GetCharacterQuery {
    /// Character ID
    pub id: i32,
}

impl GetCharacterQuery {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}

/// Query to check character online status
#[derive(Debug, Clone, Serialize, Deserialize, IntoParams)]
pub struct GetCharacterOnlineStatusQuery {
    /// Character ID
    pub id: i32,
}

impl GetCharacterOnlineStatusQuery {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}

/// Query to get account details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccountQuery {
    /// Account ID (usually from auth context)
    pub id: i32,
}

impl GetAccountQuery {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}

/// Query for paginated lists
#[derive(Debug, Clone, Serialize, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaginationQuery {
    /// Page number (1-indexed)
    #[serde(default = "default_page")]
    pub page: i32,

    /// Items per page
    #[serde(default = "default_per_page")]
    pub per_page: i32,
}

fn default_page() -> i32 {
    1
}

fn default_per_page() -> i32 {
    20
}

impl Default for PaginationQuery {
    fn default() -> Self {
        Self {
            page: default_page(),
            per_page: default_per_page(),
        }
    }
}

impl PaginationQuery {
    pub fn new(page: i32, per_page: i32) -> Self {
        Self { page, per_page }
    }

    /// Calculate offset for SQL queries
    pub fn offset(&self) -> i64 {
        ((self.page.max(1) - 1) * self.per_page) as i64
    }

    /// Get limit for SQL queries
    pub fn limit(&self) -> i64 {
        self.per_page.clamp(1, 100) as i64
    }
}

/// Query response with pagination metadata
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
}

impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, total: i64, query: &PaginationQuery) -> Self {
        let total_pages = ((total as f64) / (query.per_page as f64)).ceil() as i32;
        Self {
            items,
            total,
            page: query.page,
            per_page: query.per_page,
            total_pages,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_offset() {
        let query = PaginationQuery::new(1, 20);
        assert_eq!(query.offset(), 0);

        let query = PaginationQuery::new(2, 20);
        assert_eq!(query.offset(), 20);

        let query = PaginationQuery::new(3, 10);
        assert_eq!(query.offset(), 20);
    }

    #[test]
    fn test_pagination_limit_clamped() {
        let query = PaginationQuery::new(1, 200);
        assert_eq!(query.limit(), 100);

        let query = PaginationQuery::new(1, 0);
        assert_eq!(query.limit(), 1);
    }

    #[test]
    fn test_pagination_negative_page() {
        let query = PaginationQuery::new(-1, 20);
        assert_eq!(query.offset(), 0);
    }

    #[test]
    fn test_paginated_response_total_pages() {
        let query = PaginationQuery::new(1, 10);
        let response: PaginatedResponse<()> = PaginatedResponse::new(vec![], 25, &query);
        assert_eq!(response.total_pages, 3);
    }
}
