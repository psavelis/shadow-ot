//! Character repository - handles character CRUD operations

use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::models::character::{
    Character, CharacterSkill, CharacterSpell, CharacterDeath, CharacterStorage,
    Vocation, Sex, SkullType, SkillType,
};
use crate::{DbError, Result};

/// Repository for character operations
pub struct CharacterRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> CharacterRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Create a new character
    pub async fn create(&self, character: &Character) -> Result<Character> {
        let result = sqlx::query_as::<_, Character>(
            r#"
            INSERT INTO characters (
                id, account_id, realm_id, name, vocation, promoted, sex,
                level, experience, health, health_max, mana, mana_max,
                capacity, cap_max, soul, stamina, magic_level, magic_level_exp,
                pos_x, pos_y, pos_z, town_id,
                look_type, look_head, look_body, look_legs, look_feet, look_addons, look_mount,
                skull_type, skull_until, frags, frag_time,
                balance, bank_balance, guild_id, guild_rank_id, guild_nick,
                house_id, blessings, online, last_login, last_logout,
                deletion_date, deleted_by, total_playtime, login_count,
                deaths, kills_players, kills_monsters,
                prey_wildcard, prey_bonus_rerolls, charm_points,
                created_at, updated_at
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13,
                $14, $15, $16, $17, $18, $19, $20, $21, $22, $23,
                $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34,
                $35, $36, $37, $38, $39, $40, $41, $42, $43, $44,
                $45, $46, $47, $48, $49, $50, $51, $52, $53, $54, $55
            )
            RETURNING *
            "#,
        )
        .bind(&character.id)
        .bind(&character.account_id)
        .bind(&character.realm_id)
        .bind(&character.name)
        .bind(&character.vocation)
        .bind(character.promoted)
        .bind(&character.sex)
        .bind(character.level)
        .bind(character.experience)
        .bind(character.health)
        .bind(character.health_max)
        .bind(character.mana)
        .bind(character.mana_max)
        .bind(character.capacity)
        .bind(character.cap_max)
        .bind(character.soul)
        .bind(character.stamina)
        .bind(character.magic_level)
        .bind(character.magic_level_exp)
        .bind(character.pos_x)
        .bind(character.pos_y)
        .bind(character.pos_z)
        .bind(character.town_id)
        .bind(character.look_type)
        .bind(character.look_head)
        .bind(character.look_body)
        .bind(character.look_legs)
        .bind(character.look_feet)
        .bind(character.look_addons)
        .bind(character.look_mount)
        .bind(&character.skull_type)
        .bind(&character.skull_until)
        .bind(character.frags)
        .bind(&character.frag_time)
        .bind(character.balance)
        .bind(character.bank_balance)
        .bind(&character.guild_id)
        .bind(&character.guild_rank_id)
        .bind(&character.guild_nick)
        .bind(character.house_id)
        .bind(character.blessings)
        .bind(character.online)
        .bind(&character.last_login)
        .bind(&character.last_logout)
        .bind(&character.deletion_date)
        .bind(&character.deleted_by)
        .bind(character.total_playtime)
        .bind(character.login_count)
        .bind(character.deaths)
        .bind(character.kills_players)
        .bind(character.kills_monsters)
        .bind(character.prey_wildcard)
        .bind(character.prey_bonus_rerolls)
        .bind(character.charm_points)
        .bind(&character.created_at)
        .bind(&character.updated_at)
        .fetch_one(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Find character by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Character>> {
        let result = sqlx::query_as::<_, Character>(
            "SELECT * FROM characters WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Find character by name
    pub async fn find_by_name(&self, name: &str) -> Result<Option<Character>> {
        let result = sqlx::query_as::<_, Character>(
            "SELECT * FROM characters WHERE LOWER(name) = LOWER($1)"
        )
        .bind(name)
        .fetch_optional(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Find all characters for an account
    pub async fn find_by_account(&self, account_id: Uuid) -> Result<Vec<Character>> {
        let result = sqlx::query_as::<_, Character>(
            r#"
            SELECT * FROM characters 
            WHERE account_id = $1 AND deletion_date IS NULL
            ORDER BY level DESC, name ASC
            "#
        )
        .bind(account_id)
        .fetch_all(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Find all characters for an account in a realm
    pub async fn find_by_account_and_realm(&self, account_id: Uuid, realm_id: Uuid) -> Result<Vec<Character>> {
        let result = sqlx::query_as::<_, Character>(
            r#"
            SELECT * FROM characters 
            WHERE account_id = $1 AND realm_id = $2 AND deletion_date IS NULL
            ORDER BY level DESC, name ASC
            "#
        )
        .bind(account_id)
        .bind(realm_id)
        .fetch_all(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Update character
    pub async fn update(&self, character: &Character) -> Result<Character> {
        let result = sqlx::query_as::<_, Character>(
            r#"
            UPDATE characters SET
                vocation = $2, promoted = $3, level = $4, experience = $5,
                health = $6, health_max = $7, mana = $8, mana_max = $9,
                capacity = $10, cap_max = $11, soul = $12, stamina = $13,
                magic_level = $14, magic_level_exp = $15,
                pos_x = $16, pos_y = $17, pos_z = $18, town_id = $19,
                look_type = $20, look_head = $21, look_body = $22,
                look_legs = $23, look_feet = $24, look_addons = $25, look_mount = $26,
                skull_type = $27, skull_until = $28, frags = $29, frag_time = $30,
                balance = $31, bank_balance = $32,
                guild_id = $33, guild_rank_id = $34, guild_nick = $35,
                house_id = $36, blessings = $37, online = $38,
                last_login = $39, last_logout = $40,
                total_playtime = $41, login_count = $42, deaths = $43,
                kills_players = $44, kills_monsters = $45,
                prey_wildcard = $46, prey_bonus_rerolls = $47, charm_points = $48,
                updated_at = $49
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(&character.id)
        .bind(&character.vocation)
        .bind(character.promoted)
        .bind(character.level)
        .bind(character.experience)
        .bind(character.health)
        .bind(character.health_max)
        .bind(character.mana)
        .bind(character.mana_max)
        .bind(character.capacity)
        .bind(character.cap_max)
        .bind(character.soul)
        .bind(character.stamina)
        .bind(character.magic_level)
        .bind(character.magic_level_exp)
        .bind(character.pos_x)
        .bind(character.pos_y)
        .bind(character.pos_z)
        .bind(character.town_id)
        .bind(character.look_type)
        .bind(character.look_head)
        .bind(character.look_body)
        .bind(character.look_legs)
        .bind(character.look_feet)
        .bind(character.look_addons)
        .bind(character.look_mount)
        .bind(&character.skull_type)
        .bind(&character.skull_until)
        .bind(character.frags)
        .bind(&character.frag_time)
        .bind(character.balance)
        .bind(character.bank_balance)
        .bind(&character.guild_id)
        .bind(&character.guild_rank_id)
        .bind(&character.guild_nick)
        .bind(character.house_id)
        .bind(character.blessings)
        .bind(character.online)
        .bind(&character.last_login)
        .bind(&character.last_logout)
        .bind(character.total_playtime)
        .bind(character.login_count)
        .bind(character.deaths)
        .bind(character.kills_players)
        .bind(character.kills_monsters)
        .bind(character.prey_wildcard)
        .bind(character.prey_bonus_rerolls)
        .bind(character.charm_points)
        .bind(Utc::now())
        .fetch_one(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Delete character (soft delete)
    pub async fn delete(&self, id: Uuid, deleted_by: Option<Uuid>) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE characters 
            SET deletion_date = NOW(), deleted_by = $2, updated_at = NOW() 
            WHERE id = $1
            "#
        )
        .bind(id)
        .bind(deleted_by)
        .execute(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(())
    }

    /// Set character online status
    pub async fn set_online(&self, id: Uuid, online: bool) -> Result<()> {
        let query = if online {
            r#"
            UPDATE characters 
            SET online = true, last_login = NOW(), login_count = login_count + 1, updated_at = NOW() 
            WHERE id = $1
            "#
        } else {
            r#"
            UPDATE characters 
            SET online = false, last_logout = NOW(), updated_at = NOW() 
            WHERE id = $1
            "#
        };

        sqlx::query(query)
            .bind(id)
            .execute(self.pool)
            .await
            .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(())
    }

    /// Save character position
    pub async fn save_position(&self, id: Uuid, x: i32, y: i32, z: i32) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE characters 
            SET pos_x = $2, pos_y = $3, pos_z = $4, updated_at = NOW() 
            WHERE id = $1
            "#
        )
        .bind(id)
        .bind(x)
        .bind(y)
        .bind(z)
        .execute(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(())
    }

    /// Get character skills
    pub async fn get_skills(&self, character_id: Uuid) -> Result<Vec<CharacterSkill>> {
        let result = sqlx::query_as::<_, CharacterSkill>(
            "SELECT * FROM character_skills WHERE character_id = $1"
        )
        .bind(character_id)
        .fetch_all(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Save character skill
    pub async fn save_skill(&self, skill: &CharacterSkill) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO character_skills (character_id, skill_type, level, tries)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (character_id, skill_type) 
            DO UPDATE SET level = $3, tries = $4
            "#
        )
        .bind(&skill.character_id)
        .bind(&skill.skill_type)
        .bind(skill.level)
        .bind(skill.tries)
        .execute(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(())
    }

    /// Get character spells
    pub async fn get_spells(&self, character_id: Uuid) -> Result<Vec<CharacterSpell>> {
        let result = sqlx::query_as::<_, CharacterSpell>(
            "SELECT * FROM character_spells WHERE character_id = $1"
        )
        .bind(character_id)
        .fetch_all(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Learn a spell
    pub async fn learn_spell(&self, character_id: Uuid, spell_id: i32) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO character_spells (character_id, spell_id, learned_at)
            VALUES ($1, $2, NOW())
            ON CONFLICT (character_id, spell_id) DO NOTHING
            "#
        )
        .bind(character_id)
        .bind(spell_id)
        .execute(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(())
    }

    /// Record a death
    pub async fn record_death(&self, death: &CharacterDeath) -> Result<CharacterDeath> {
        let result = sqlx::query_as::<_, CharacterDeath>(
            r#"
            INSERT INTO character_deaths (
                id, character_id, realm_id, level, killed_by, is_player,
                mostdamage_by, mostdamage_is_player, unjustified, time
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#
        )
        .bind(&death.id)
        .bind(&death.character_id)
        .bind(&death.realm_id)
        .bind(death.level)
        .bind(&death.killed_by)
        .bind(death.is_player)
        .bind(&death.mostdamage_by)
        .bind(death.mostdamage_is_player)
        .bind(death.unjustified)
        .bind(&death.time)
        .fetch_one(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        // Increment death counter
        sqlx::query(
            "UPDATE characters SET deaths = deaths + 1, updated_at = NOW() WHERE id = $1"
        )
        .bind(&death.character_id)
        .execute(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Get recent deaths
    pub async fn get_deaths(&self, character_id: Uuid, limit: i32) -> Result<Vec<CharacterDeath>> {
        let result = sqlx::query_as::<_, CharacterDeath>(
            r#"
            SELECT * FROM character_deaths 
            WHERE character_id = $1 
            ORDER BY time DESC 
            LIMIT $2
            "#
        )
        .bind(character_id)
        .bind(limit)
        .fetch_all(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Get/set storage value
    pub async fn get_storage(&self, character_id: Uuid, key: &str) -> Result<Option<i64>> {
        let result = sqlx::query_scalar::<_, i64>(
            "SELECT value FROM character_storage WHERE character_id = $1 AND key = $2"
        )
        .bind(character_id)
        .bind(key)
        .fetch_optional(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    pub async fn set_storage(&self, character_id: Uuid, key: &str, value: i64) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO character_storage (character_id, key, value)
            VALUES ($1, $2, $3)
            ON CONFLICT (character_id, key) DO UPDATE SET value = $3
            "#
        )
        .bind(character_id)
        .bind(key)
        .bind(value)
        .execute(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(())
    }

    /// Get highscores
    pub async fn get_highscores(
        &self,
        realm_id: Option<Uuid>,
        vocation: Option<Vocation>,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Character>> {
        let mut query = String::from(
            "SELECT * FROM characters WHERE deletion_date IS NULL"
        );
        
        if realm_id.is_some() {
            query.push_str(" AND realm_id = $3");
        }
        if vocation.is_some() {
            query.push_str(" AND vocation = $4");
        }
        
        query.push_str(" ORDER BY level DESC, experience DESC LIMIT $1 OFFSET $2");

        // Simplified - in production, use dynamic query builder
        let result = sqlx::query_as::<_, Character>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(self.pool)
            .await
            .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }

    /// Count characters by realm
    pub async fn count_by_realm(&self, realm_id: Uuid) -> Result<i64> {
        let result = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM characters WHERE realm_id = $1 AND deletion_date IS NULL"
        )
        .bind(realm_id)
        .fetch_one(self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(result)
    }
}
