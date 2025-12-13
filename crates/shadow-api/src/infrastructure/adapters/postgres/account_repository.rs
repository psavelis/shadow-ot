//! PostgreSQL Account Repository implementation

use crate::application::ports::out_ports::AccountRepository;
use crate::domain::{Account, AccountId, DomainError};
use async_trait::async_trait;
use sqlx::PgPool;

/// PostgreSQL implementation of AccountRepository
pub struct PostgresAccountRepository {
    pool: PgPool,
}

impl PostgresAccountRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct AccountRow {
    id: i32,
    email: String,
    premium_until: Option<chrono::DateTime<chrono::Utc>>,
    coins: i32,
}

impl From<AccountRow> for Account {
    fn from(row: AccountRow) -> Self {
        Account {
            id: AccountId::new(row.id),
            email: row.email,
            premium_until: row.premium_until,
            coins: row.coins,
        }
    }
}

#[async_trait]
impl AccountRepository for PostgresAccountRepository {
    async fn find_by_id(&self, id: i32) -> Result<Option<Account>, DomainError> {
        let row = sqlx::query_as::<_, AccountRow>(
            "SELECT id, email, premium_until, coins FROM accounts WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(row.map(Into::into))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<Account>, DomainError> {
        let row = sqlx::query_as::<_, AccountRow>(
            "SELECT id, email, premium_until, coins FROM accounts WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(row.map(Into::into))
    }

    async fn is_email_taken(&self, email: &str, exclude_id: Option<i32>) -> Result<bool, DomainError> {
        let exists = match exclude_id {
            Some(id) => {
                sqlx::query_scalar::<_, bool>(
                    "SELECT EXISTS(SELECT 1 FROM accounts WHERE email = $1 AND id != $2)",
                )
                .bind(email)
                .bind(id)
                .fetch_one(&self.pool)
                .await
            }
            None => {
                sqlx::query_scalar::<_, bool>(
                    "SELECT EXISTS(SELECT 1 FROM accounts WHERE email = $1)",
                )
                .bind(email)
                .fetch_one(&self.pool)
                .await
            }
        }
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(exists)
    }

    async fn update_email(&self, id: i32, email: &str) -> Result<(), DomainError> {
        sqlx::query("UPDATE accounts SET email = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2")
            .bind(email)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn get_password_hash(&self, id: i32) -> Result<String, DomainError> {
        let hash = sqlx::query_scalar::<_, String>(
            "SELECT password_hash FROM accounts WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?
        .ok_or(DomainError::AccountNotFound)?;

        Ok(hash)
    }

    async fn update_password(&self, id: i32, hash: &str, salt: &str) -> Result<(), DomainError> {
        sqlx::query(
            "UPDATE accounts SET password_hash = $1, password_salt = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3",
        )
        .bind(hash)
        .bind(salt)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_row_conversion() {
        let row = AccountRow {
            id: 1,
            email: "test@example.com".to_string(),
            premium_until: None,
            coins: 100,
        };

        let account: Account = row.into();
        
        assert_eq!(account.id.value(), 1);
        assert_eq!(account.email, "test@example.com");
        assert_eq!(account.coins, 100);
        assert!(!account.is_premium());
    }

    #[test]
    fn test_account_with_premium() {
        let future_date = chrono::Utc::now() + chrono::Duration::days(30);
        let row = AccountRow {
            id: 1,
            email: "premium@example.com".to_string(),
            premium_until: Some(future_date),
            coins: 500,
        };

        let account: Account = row.into();
        
        assert!(account.is_premium());
    }
}
