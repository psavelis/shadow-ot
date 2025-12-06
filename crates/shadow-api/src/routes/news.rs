//! News endpoints

use crate::state::AppState;
use crate::ApiResult;
use axum::{extract::{Path, Query, State}, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;

/// News article
#[derive(Debug, Serialize, ToSchema)]
pub struct NewsArticle {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub category: String,
    pub author_name: Option<String>,
    pub view_count: i32,
    pub featured: bool,
    pub published_at: Option<String>,
}

/// News query
#[derive(Debug, Deserialize)]
pub struct NewsQuery {
    pub category: Option<String>,
    pub featured: Option<bool>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

/// List news articles
#[utoipa::path(
    get,
    path = "/api/v1/news",
    params(
        ("category" = Option<String>, Query, description = "Filter by category"),
        ("featured" = Option<bool>, Query, description = "Filter by featured"),
        ("page" = Option<u32>, Query, description = "Page number"),
        ("limit" = Option<u32>, Query, description = "Results per page")
    ),
    responses(
        (status = 200, description = "News articles", body = Vec<NewsArticle>)
    ),
    tag = "news"
)]
pub async fn list_news(
    State(state): State<Arc<AppState>>,
    Query(query): Query<NewsQuery>,
) -> ApiResult<Json<Vec<NewsArticle>>> {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(10).min(50);
    let offset = (page - 1) * limit;

    let articles = sqlx::query_as::<_, NewsRow>(
        "SELECT n.id, n.title, n.content, n.category, n.view_count, n.featured, n.published_at,
                a.email as author_name
         FROM news_articles n
         LEFT JOIN accounts a ON n.author_id = a.id
         WHERE n.is_published = true
           AND ($1::text IS NULL OR n.category = $1)
           AND ($2::bool IS NULL OR n.featured = $2)
         ORDER BY n.featured DESC, n.published_at DESC
         LIMIT $3 OFFSET $4"
    )
    .bind(&query.category)
    .bind(query.featured)
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(articles.into_iter().map(Into::into).collect()))
}

/// Get news article by ID
pub async fn get_article(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> ApiResult<Json<NewsArticle>> {
    // Increment view count
    sqlx::query("UPDATE news_articles SET view_count = view_count + 1 WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await?;

    let article = sqlx::query_as::<_, NewsRow>(
        "SELECT n.id, n.title, n.content, n.category, n.view_count, n.featured, n.published_at,
                a.email as author_name
         FROM news_articles n
         LEFT JOIN accounts a ON n.author_id = a.id
         WHERE n.id = $1 AND n.is_published = true"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(crate::error::ApiError::NotFound("Article not found".to_string()))?;

    Ok(Json(article.into()))
}

#[derive(sqlx::FromRow)]
struct NewsRow {
    id: i32,
    title: String,
    content: String,
    category: String,
    view_count: i32,
    featured: bool,
    published_at: Option<chrono::DateTime<chrono::Utc>>,
    author_name: Option<String>,
}

impl From<NewsRow> for NewsArticle {
    fn from(row: NewsRow) -> Self {
        NewsArticle {
            id: row.id,
            title: row.title,
            content: row.content,
            category: row.category,
            author_name: row.author_name,
            view_count: row.view_count,
            featured: row.featured,
            published_at: row.published_at.map(|t| t.to_rfc3339()),
        }
    }
}
