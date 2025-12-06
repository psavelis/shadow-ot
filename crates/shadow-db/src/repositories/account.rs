//! Account repository - handles account CRUD operations

use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::models::account::{Account, AccountSession, AccountType};
use crate::{DbError, Result};

/// Repository for account operations
pub struct AccountRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> AccountRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Create a new account
    pub async fn create(&self, account: &Account) -> Result<Account> {
        let result = sqlx::query_as::<_, Account>(
            r#"
            INSERT INTO accounts (
                id, email, username, password_hash, account_type,
                premium_until, premium_days_purchased, coins, tournament_coins,
                email_verified, two_factor_enabled, two_factor_secret,
                last_login, last_ip, login_attempts, locked_until,
                ban_until, ban_reason, ban_by, wallet_address, wallet_chain,
                created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23)
            RETURNING *
            "#,
        )
        .bind(&account.id)
        .bind(&account.email)
        .bind(&account.username)
        .bind(&account.password_hash)
        .bind(&account.account_type)
        .bind(&account.premium_until)
        .bind(account.premium_days_purchased)
        .bind(account.coins)
        .bind(account.tournament_coins)
        .bind(account.email_verified)
        .bind(account.two_factor_enabled)
        .bind(&account.two_factor_secret)
        .bind(&account.last_login)
        .bind(&account.last_ip)
        .bind(account.login_attempts)
        .bind(&account.locked_until)
        .bind(&account.ban_until)
        .bind(&account.ban_reason)
        .bind(&account.ban_by)
        .bind(&account.wallet_address)
        .bind(&account.wallet_chain)
        .bind(&account.created_at)
        .bind(&account.updated_at)
        .fetch_one(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Find account by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Account>> {
        let result = sqlx::query_as::<_, Account>(
            "SELECT * FROM accounts WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Find account by username
    pub async fn find_by_username(&self, username: &str) -> Result<Option<Account>> {
        let result = sqlx::query_as::<_, Account>(
            "SELECT * FROM accounts WHERE LOWER(username) = LOWER($1)"
        )
        .bind(username)
        .fetch_optional(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Find account by email
    pub async fn find_by_email(&self, email: &str) -> Result<Option<Account>> {
        let result = sqlx::query_as::<_, Account>(
            "SELECT * FROM accounts WHERE LOWER(email) = LOWER($1)"
        )
        .bind(email)
        .fetch_optional(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Find account by username or email
    pub async fn find_by_username_or_email(&self, identifier: &str) -> Result<Option<Account>> {
        let result = sqlx::query_as::<_, Account>(
            "SELECT * FROM accounts WHERE LOWER(username) = LOWER($1) OR LOWER(email) = LOWER($1)"
        )
        .bind(identifier)
        .fetch_optional(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Update account
    pub async fn update(&self, account: &Account) -> Result<Account> {
        let result = sqlx::query_as::<_, Account>(
            r#"
            UPDATE accounts SET
                email = $2,
                username = $3,
                password_hash = $4,
                account_type = $5,
                premium_until = $6,
                premium_days_purchased = $7,
                coins = $8,
                tournament_coins = $9,
                email_verified = $10,
                two_factor_enabled = $11,
                two_factor_secret = $12,
                last_login = $13,
                last_ip = $14,
                login_attempts = $15,
                locked_until = $16,
                ban_until = $17,
                ban_reason = $18,
                ban_by = $19,
                wallet_address = $20,
                wallet_chain = $21,
                updated_at = $22
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(&account.id)
        .bind(&account.email)
        .bind(&account.username)
        .bind(&account.password_hash)
        .bind(&account.account_type)
        .bind(&account.premium_until)
        .bind(account.premium_days_purchased)
        .bind(account.coins)
        .bind(account.tournament_coins)
        .bind(account.email_verified)
        .bind(account.two_factor_enabled)
        .bind(&account.two_factor_secret)
        .bind(&account.last_login)
        .bind(&account.last_ip)
        .bind(account.login_attempts)
        .bind(&account.locked_until)
        .bind(&account.ban_until)
        .bind(&account.ban_reason)
        .bind(&account.ban_by)
        .bind(&account.wallet_address)
        .bind(&account.wallet_chain)
        .bind(Utc::now())
        .fetch_one(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Verify password (returns account if valid)
    pub async fn verify_credentials(&self, identifier: &str, password_hash: &str) -> Result<Option<Account>> {
        let result = sqlx::query_as::<_, Account>(
            r#"
            SELECT * FROM accounts 
            WHERE (LOWER(username) = LOWER($1) OR LOWER(email) = LOWER($1))
            AND password_hash = $2 
            AND ban_until IS NULL
            "#
        )
        .bind(identifier)
        .bind(password_hash)
        .fetch_optional(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Check if account is banned
    pub async fn is_banned(&self, account_id: Uuid) -> Result<bool> {
        let result = sqlx::query_scalar::<_, bool>(
            "SELECT ban_until IS NOT NULL AND ban_until > NOW() FROM accounts WHERE id = $1"
        )
        .bind(account_id)
        .fetch_optional(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result.unwrap_or(false))
    }

    /// Ban an account
    pub async fn ban(&self, account_id: Uuid, banned_by: Option<Uuid>, reason: &str, until: Option<DateTime<Utc>>) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE accounts SET
                ban_until = $2,
                ban_reason = $3,
                ban_by = $4,
                updated_at = NOW()
            WHERE id = $1
            "#
        )
        .bind(account_id)
        .bind(until)
        .bind(reason)
        .bind(banned_by)
        .execute(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(())
    }

    /// Unban an account
    pub async fn unban(&self, account_id: Uuid) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE accounts SET
                ban_until = NULL,
                ban_reason = NULL,
                ban_by = NULL,
                updated_at = NOW()
            WHERE id = $1
            "#
        )
        .bind(account_id)
        .execute(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(())
    }

    /// Create a session
    pub async fn create_session(&self, session: &AccountSession) -> Result<AccountSession> {
        let result = sqlx::query_as::<_, AccountSession>(
            r#"
            INSERT INTO account_sessions (
                id, account_id, session_token, ip_address, user_agent, expires_at, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#
        )
        .bind(&session.id)
        .bind(&session.account_id)
        .bind(&session.session_token)
        .bind(&session.ip_address)
        .bind(&session.user_agent)
        .bind(&session.expires_at)
        .bind(&session.created_at)
        .fetch_one(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Find session by token
    pub async fn find_session(&self, token: &str) -> Result<Option<AccountSession>> {
        let result = sqlx::query_as::<_, AccountSession>(
            r#"
            SELECT * FROM account_sessions 
            WHERE session_token = $1 
            AND expires_at > NOW()
            "#
        )
        .bind(token)
        .fetch_optional(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Invalidate session
    pub async fn invalidate_session(&self, token: &str) -> Result<()> {
        sqlx::query(
            "DELETE FROM account_sessions WHERE session_token = $1"
        )
        .bind(token)
        .execute(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(())
    }

    /// Invalidate all sessions for an account
    pub async fn invalidate_all_sessions(&self, account_id: Uuid) -> Result<u64> {
        let result = sqlx::query(
            "DELETE FROM account_sessions WHERE account_id = $1"
        )
        .bind(account_id)
        .execute(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result.rows_affected())
    }

    /// Update premium status
    pub async fn update_premium(&self, account_id: Uuid, premium_until: Option<DateTime<Utc>>) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE accounts 
            SET premium_until = $2, updated_at = NOW() 
            WHERE id = $1
            "#
        )
        .bind(account_id)
        .bind(premium_until)
        .execute(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(())
    }

    /// Add coins to account
    pub async fn add_coins(&self, account_id: Uuid, amount: i64) -> Result<i64> {
        let result = sqlx::query_scalar::<_, i64>(
            r#"
            UPDATE accounts 
            SET coins = coins + $2, updated_at = NOW() 
            WHERE id = $1
            RETURNING coins
            "#
        )
        .bind(account_id)
        .bind(amount)
        .fetch_one(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Record login attempt
    pub async fn record_login_attempt(&self, account_id: Uuid, ip: &str, success: bool) -> Result<()> {
        if success {
            sqlx::query(
                r#"
                UPDATE accounts SET
                    last_login = NOW(),
                    last_ip = $2,
                    login_attempts = 0,
                    locked_until = NULL,
                    updated_at = NOW()
                WHERE id = $1
                "#
            )
            .bind(account_id)
            .bind(ip)
            .execute(self.pool)
            .await
            .map_err(|e| DbError::Query(e.to_string()))?;
        } else {
            // Increment failed attempts
            sqlx::query(
                r#"
                UPDATE accounts SET
                    login_attempts = login_attempts + 1,
                    locked_until = CASE 
                        WHEN login_attempts >= 4 THEN NOW() + INTERVAL '15 minutes'
                        ELSE locked_until
                    END,
                    updated_at = NOW()
                WHERE id = $1
                "#
            )
            .bind(account_id)
            .execute(self.pool)
            .await
            .map_err(|e| DbError::Query(e.to_string()))?;
        }

        Ok(())
    }

    /// Get online player count
    pub async fn get_online_count(&self) -> Result<i64> {
        let result = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM characters WHERE online = true"
        )
        .fetch_one(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }
}
