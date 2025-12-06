//! House endpoints

use crate::state::AppState;
use crate::ApiResult;
use axum::{extract::{Path, Query, State}, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// House info
#[derive(Debug, Serialize)]
pub struct HouseInfo {
    pub id: i32,
    pub name: String,
    pub owner_name: Option<String>,
    pub town_id: i32,
    pub rent: i32,
    pub size: i32,
    pub beds: i32,
    pub paid_until: Option<String>,
}

/// House query
#[derive(Debug, Deserialize)]
pub struct HouseQuery {
    pub town_id: Option<i32>,
    pub available: Option<bool>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

/// List houses
pub async fn list_houses(
    State(state): State<Arc<AppState>>,
    Path(realm): Path<i32>,
    Query(query): Query<HouseQuery>,
) -> ApiResult<Json<Vec<HouseInfo>>> {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(50).min(100);
    let offset = (page - 1) * limit;

    let houses = sqlx::query_as::<_, HouseRow>(
        "SELECT h.*, c.name as owner_name
         FROM houses h
         LEFT JOIN characters c ON h.owner_id = c.id
         WHERE h.realm_id = $1
           AND ($2::int IS NULL OR h.town_id = $2)
           AND ($3::bool IS NULL OR (h.owner_id IS NULL) = $3)
         ORDER BY h.town_id, h.name
         LIMIT $4 OFFSET $5"
    )
    .bind(realm)
    .bind(query.town_id)
    .bind(query.available)
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(houses.into_iter().map(Into::into).collect()))
}

/// Get house by ID
pub async fn get_house(
    State(state): State<Arc<AppState>>,
    Path((realm, id)): Path<(i32, i32)>,
) -> ApiResult<Json<HouseInfo>> {
    let house = sqlx::query_as::<_, HouseRow>(
        "SELECT h.*, c.name as owner_name
         FROM houses h
         LEFT JOIN characters c ON h.owner_id = c.id
         WHERE h.realm_id = $1 AND h.id = $2"
    )
    .bind(realm)
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(crate::error::ApiError::NotFound("House not found".to_string()))?;

    Ok(Json(house.into()))
}

#[derive(sqlx::FromRow)]
struct HouseRow {
    id: i32,
    name: String,
    owner_name: Option<String>,
    town_id: i32,
    rent: i32,
    size: i32,
    beds: i32,
    paid_until: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<HouseRow> for HouseInfo {
    fn from(row: HouseRow) -> Self {
        HouseInfo {
            id: row.id,
            name: row.name,
            owner_name: row.owner_name,
            town_id: row.town_id,
            rent: row.rent,
            size: row.size,
            beds: row.beds,
            paid_until: row.paid_until.map(|t| t.to_rfc3339()),
        }
    }
}
