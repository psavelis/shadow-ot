//! Guild repository - handles guild CRUD operations

use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::models::guild::{Guild, GuildRank, GuildMember, GuildInvite, GuildWar};
use crate::{DbError, Result};

/// Repository for guild operations
pub struct GuildRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> GuildRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Create a new guild
    pub async fn create(&self, guild: &Guild) -> Result<Guild> {
        let result = sqlx::query_as::<_, Guild>(
            r#"
            INSERT INTO guilds (
                id, realm_id, name, owner_id, description, motd, logo_url,
                balance, level, experience, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING *
            "#,
        )
        .bind(&guild.id)
        .bind(&guild.realm_id)
        .bind(&guild.name)
        .bind(&guild.owner_id)
        .bind(&guild.description)
        .bind(&guild.motd)
        .bind(&guild.logo_url)
        .bind(guild.balance)
        .bind(guild.level)
        .bind(guild.experience)
        .bind(&guild.created_at)
        .bind(&guild.updated_at)
        .fetch_one(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Find guild by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Guild>> {
        let result = sqlx::query_as::<_, Guild>(
            "SELECT * FROM guilds WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Find guild by name
    pub async fn find_by_name(&self, name: &str, realm_id: Uuid) -> Result<Option<Guild>> {
        let result = sqlx::query_as::<_, Guild>(
            "SELECT * FROM guilds WHERE LOWER(name) = LOWER($1) AND realm_id = $2"
        )
        .bind(name)
        .bind(realm_id)
        .fetch_optional(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Get all guilds for a realm
    pub async fn find_by_realm(&self, realm_id: Uuid) -> Result<Vec<Guild>> {
        let result = sqlx::query_as::<_, Guild>(
            "SELECT * FROM guilds WHERE realm_id = $1 ORDER BY name ASC"
        )
        .bind(realm_id)
        .fetch_all(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Update guild
    pub async fn update(&self, guild: &Guild) -> Result<Guild> {
        let result = sqlx::query_as::<_, Guild>(
            r#"
            UPDATE guilds SET
                name = $2, owner_id = $3, description = $4, motd = $5,
                logo_url = $6, balance = $7, level = $8, experience = $9,
                updated_at = $10
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(&guild.id)
        .bind(&guild.name)
        .bind(&guild.owner_id)
        .bind(&guild.description)
        .bind(&guild.motd)
        .bind(&guild.logo_url)
        .bind(guild.balance)
        .bind(guild.level)
        .bind(guild.experience)
        .bind(Utc::now())
        .fetch_one(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Delete guild
    pub async fn delete(&self, id: Uuid) -> Result<()> {
        // Remove all members first
        sqlx::query("DELETE FROM guild_members WHERE guild_id = $1")
            .bind(id)
            .execute(self.pool)
            .await
            .map_err(|e| DbError::Query(e.to_string()))?;

        // Remove all ranks
        sqlx::query("DELETE FROM guild_ranks WHERE guild_id = $1")
            .bind(id)
            .execute(self.pool)
            .await
            .map_err(|e| DbError::Query(e.to_string()))?;

        // Delete guild
        sqlx::query("DELETE FROM guilds WHERE id = $1")
            .bind(id)
            .execute(self.pool)
            .await
            .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(())
    }

    /// Create guild rank
    pub async fn create_rank(&self, rank: &GuildRank) -> Result<GuildRank> {
        let result = sqlx::query_as::<_, GuildRank>(
            r#"
            INSERT INTO guild_ranks (id, guild_id, name, level, permissions)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#
        )
        .bind(&rank.id)
        .bind(&rank.guild_id)
        .bind(&rank.name)
        .bind(rank.level)
        .bind(rank.permissions)
        .fetch_one(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Get guild ranks
    pub async fn get_ranks(&self, guild_id: Uuid) -> Result<Vec<GuildRank>> {
        let result = sqlx::query_as::<_, GuildRank>(
            "SELECT * FROM guild_ranks WHERE guild_id = $1 ORDER BY level DESC"
        )
        .bind(guild_id)
        .fetch_all(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Add guild member
    pub async fn add_member(&self, member: &GuildMember) -> Result<GuildMember> {
        let result = sqlx::query_as::<_, GuildMember>(
            r#"
            INSERT INTO guild_members (guild_id, character_id, rank_id, nick, joined_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#
        )
        .bind(&member.guild_id)
        .bind(&member.character_id)
        .bind(&member.rank_id)
        .bind(&member.nick)
        .bind(&member.joined_at)
        .fetch_one(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        // Update character guild info
        sqlx::query(
            "UPDATE characters SET guild_id = $1, guild_rank_id = $2, guild_nick = $3 WHERE id = $4"
        )
        .bind(&member.guild_id)
        .bind(&member.rank_id)
        .bind(&member.nick)
        .bind(&member.character_id)
        .execute(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Remove guild member
    pub async fn remove_member(&self, guild_id: Uuid, character_id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM guild_members WHERE guild_id = $1 AND character_id = $2")
            .bind(guild_id)
            .bind(character_id)
            .execute(self.pool)
            .await
            .map_err(|e| DbError::Query(e.to_string()))?;

        // Clear character guild info
        sqlx::query(
            "UPDATE characters SET guild_id = NULL, guild_rank_id = NULL, guild_nick = NULL WHERE id = $1"
        )
        .bind(character_id)
        .execute(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(())
    }

    /// Get guild members
    pub async fn get_members(&self, guild_id: Uuid) -> Result<Vec<GuildMember>> {
        let result = sqlx::query_as::<_, GuildMember>(
            "SELECT * FROM guild_members WHERE guild_id = $1"
        )
        .bind(guild_id)
        .fetch_all(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Create guild invite
    pub async fn create_invite(&self, invite: &GuildInvite) -> Result<GuildInvite> {
        let result = sqlx::query_as::<_, GuildInvite>(
            r#"
            INSERT INTO guild_invites (id, guild_id, character_id, invited_by, created_at, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#
        )
        .bind(&invite.id)
        .bind(&invite.guild_id)
        .bind(&invite.character_id)
        .bind(&invite.invited_by)
        .bind(&invite.created_at)
        .bind(&invite.expires_at)
        .fetch_one(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Get pending invites for a character
    pub async fn get_invites(&self, character_id: Uuid) -> Result<Vec<GuildInvite>> {
        let result = sqlx::query_as::<_, GuildInvite>(
            "SELECT * FROM guild_invites WHERE character_id = $1 AND expires_at > NOW()"
        )
        .bind(character_id)
        .fetch_all(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Delete invite
    pub async fn delete_invite(&self, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM guild_invites WHERE id = $1")
            .bind(id)
            .execute(self.pool)
            .await
            .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(())
    }

    /// Get member count
    pub async fn get_member_count(&self, guild_id: Uuid) -> Result<i64> {
        let result = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM guild_members WHERE guild_id = $1"
        )
        .bind(guild_id)
        .fetch_one(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }
}
