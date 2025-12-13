//! PostgreSQL Realm Repository implementation

use crate::application::ports::out_ports::{Realm, RealmRepository};
use crate::domain::DomainError;
use async_trait::async_trait;
use sqlx::PgPool;

/// PostgreSQL implementation of RealmRepository
pub struct PostgresRealmRepository {
    pool: PgPool,
}

impl PostgresRealmRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct RealmRow {
    id: i32,
    name: String,
    slug: String,
}

impl From<RealmRow> for Realm {
    fn from(row: RealmRow) -> Self {
        Realm {
            id: row.id,
            name: row.name,
            slug: row.slug,
        }
    }
}

#[async_trait]
impl RealmRepository for PostgresRealmRepository {
    async fn exists(&self, id: i32) -> Result<bool, DomainError> {
        let exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM realms WHERE id = $1)",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(exists)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Realm>, DomainError> {
        let row = sqlx::query_as::<_, RealmRow>(
            "SELECT id, name, slug FROM realms WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(row.map(Into::into))
    }

    async fn get_online_count(&self, id: i32) -> Result<i64, DomainError> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM characters WHERE realm_id = $1 AND online = true",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_realm_row_conversion() {
        let row = RealmRow {
            id: 1,
            name: "Aetheria".to_string(),
            slug: "aetheria".to_string(),
        };

        let realm: Realm = row.into();
        
        assert_eq!(realm.id, 1);
        assert_eq!(realm.name, "Aetheria");
        assert_eq!(realm.slug, "aetheria");
    }
}
