//! PostgreSQL Character Repository implementation

use crate::application::ports::out_ports::CharacterRepository;
use crate::domain::{Character, CharacterId, DomainError, Gender, Vocation};
use async_trait::async_trait;
use sqlx::PgPool;

/// PostgreSQL implementation of CharacterRepository
pub struct PostgresCharacterRepository {
    pool: PgPool,
}

impl PostgresCharacterRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct CharacterRow {
    id: i32,
    account_id: i32,
    realm_id: i32,
    name: String,
    sex: i16,
    vocation: i16,
    level: i32,
    experience: i64,
    health: i32,
    max_health: i32,
    mana: i32,
    max_mana: i32,
    look_type: i32,
    look_head: i16,
    look_body: i16,
    look_legs: i16,
    look_feet: i16,
    look_addons: i16,
    town_id: i32,
    balance: i64,
    bank_balance: i64,
    online: bool,
}

impl From<CharacterRow> for Character {
    fn from(row: CharacterRow) -> Self {
        Character {
            id: CharacterId::new(row.id),
            account_id: row.account_id,
            realm_id: row.realm_id,
            name: row.name,
            gender: Gender::from_protocol_value(row.sex).unwrap_or_default(),
            vocation: Vocation::from_i16(row.vocation).unwrap_or_default(),
            level: row.level,
            experience: row.experience,
            health: row.health,
            max_health: row.max_health,
            mana: row.mana,
            max_mana: row.max_mana,
            look_type: row.look_type,
            look_head: row.look_head,
            look_body: row.look_body,
            look_legs: row.look_legs,
            look_feet: row.look_feet,
            look_addons: row.look_addons,
            town_id: row.town_id,
            balance: row.balance,
            bank_balance: row.bank_balance,
            online: row.online,
        }
    }
}

#[async_trait]
impl CharacterRepository for PostgresCharacterRepository {
    async fn find_by_account_id(&self, account_id: i32) -> Result<Vec<Character>, DomainError> {
        let rows = sqlx::query_as::<_, CharacterRow>(
            r#"
            SELECT id, account_id, realm_id, name, sex, vocation, level, experience,
                   health, max_health, mana, max_mana, look_type, look_head, look_body,
                   look_legs, look_feet, look_addons, town_id, balance, bank_balance, online
            FROM characters
            WHERE account_id = $1 AND deletion_time IS NULL
            ORDER BY level DESC
            "#,
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Character>, DomainError> {
        let row = sqlx::query_as::<_, CharacterRow>(
            r#"
            SELECT id, account_id, realm_id, name, sex, vocation, level, experience,
                   health, max_health, mana, max_mana, look_type, look_head, look_body,
                   look_legs, look_feet, look_addons, town_id, balance, bank_balance, online
            FROM characters
            WHERE id = $1 AND deletion_time IS NULL
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(row.map(Into::into))
    }

    async fn is_name_available(&self, name: &str) -> Result<bool, DomainError> {
        let exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM characters WHERE LOWER(name) = LOWER($1))",
        )
        .bind(name)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(!exists)
    }

    async fn count_by_account_id(&self, account_id: i32) -> Result<i64, DomainError> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM characters WHERE account_id = $1 AND deletion_time IS NULL",
        )
        .bind(account_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(count)
    }

    async fn create(
        &self,
        account_id: i32,
        realm_id: i32,
        name: &str,
        gender: Gender,
        vocation: Vocation,
        look_type: i32,
        town_id: i32,
    ) -> Result<i32, DomainError> {
        let id = sqlx::query_scalar::<_, i32>(
            r#"
            INSERT INTO characters (account_id, realm_id, name, sex, vocation, look_type, town_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id
            "#,
        )
        .bind(account_id)
        .bind(realm_id)
        .bind(name)
        .bind(gender.to_protocol_value())
        .bind(vocation.to_i16())
        .bind(look_type)
        .bind(town_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(id)
    }

    async fn schedule_deletion(&self, id: i32, days: i32) -> Result<(), DomainError> {
        sqlx::query(
            "UPDATE characters SET deletion_time = CURRENT_TIMESTAMP + INTERVAL '1 day' * $1 WHERE id = $2",
        )
        .bind(days)
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn is_online(&self, id: i32) -> Result<Option<bool>, DomainError> {
        let online = sqlx::query_scalar::<_, bool>("SELECT online FROM characters WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(online)
    }

    async fn verify_ownership(&self, character_id: i32, account_id: i32) -> Result<bool, DomainError> {
        let owns = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM characters WHERE id = $1 AND account_id = $2 AND deletion_time IS NULL)",
        )
        .bind(character_id)
        .bind(account_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(owns)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Integration tests would require a test database
    // For unit testing, we'd use mock implementations

    #[test]
    fn test_character_row_conversion() {
        let row = CharacterRow {
            id: 1,
            account_id: 100,
            realm_id: 1,
            name: "TestKnight".to_string(),
            sex: 1, // Male
            vocation: 4, // Knight
            level: 50,
            experience: 1000000,
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
        };

        let character: Character = row.into();
        
        assert_eq!(character.id.value(), 1);
        assert_eq!(character.name, "TestKnight");
        assert_eq!(character.gender, Gender::Male);
        assert_eq!(character.vocation, Vocation::Knight);
        assert_eq!(character.level, 50);
    }
}
