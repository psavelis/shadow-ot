//! Forum endpoints

use crate::state::AppState;
use crate::ApiResult;
use axum::{extract::{Path, Query, State}, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Forum category
#[derive(Debug, Serialize)]
pub struct ForumCategory {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub position: i32,
    pub is_locked: bool,
    pub thread_count: i64,
}

/// List forum categories
pub async fn list_categories(
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<Vec<ForumCategory>>> {
    let categories = sqlx::query_as::<_, ForumCategoryRow>(
        "SELECT fc.*, (SELECT COUNT(*) FROM forum_threads WHERE category_id = fc.id) as thread_count
         FROM forum_categories fc
         ORDER BY fc.position"
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(categories.into_iter().map(|c| ForumCategory {
        id: c.id,
        name: c.name,
        description: c.description,
        position: c.position,
        is_locked: c.is_locked,
        thread_count: c.thread_count,
    }).collect()))
}

/// Forum thread
#[derive(Debug, Serialize)]
pub struct ForumThread {
    pub id: i32,
    pub category_id: i32,
    pub title: String,
    pub author_name: Option<String>,
    pub is_sticky: bool,
    pub is_locked: bool,
    pub view_count: i32,
    pub reply_count: i32,
    pub last_reply_at: Option<String>,
    pub last_reply_by: Option<String>,
    pub created_at: String,
}

/// Thread query
#[derive(Debug, Deserialize)]
pub struct ThreadQuery {
    pub category_id: Option<i32>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

/// List forum threads
pub async fn list_threads(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ThreadQuery>,
) -> ApiResult<Json<Vec<ForumThread>>> {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(20).min(50);
    let offset = (page - 1) * limit;

    let threads = sqlx::query_as::<_, ForumThreadRow>(
        "SELECT ft.*, a.email as author_name
         FROM forum_threads ft
         LEFT JOIN accounts a ON ft.author_id = a.id
         WHERE ($1::int IS NULL OR ft.category_id = $1)
         ORDER BY ft.is_sticky DESC, ft.last_reply_at DESC NULLS LAST
         LIMIT $2 OFFSET $3"
    )
    .bind(query.category_id)
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(threads.into_iter().map(Into::into).collect()))
}

/// Get thread with posts
pub async fn get_thread(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> ApiResult<Json<ThreadWithPosts>> {
    // Increment view count
    sqlx::query("UPDATE forum_threads SET view_count = view_count + 1 WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await?;

    let thread = sqlx::query_as::<_, ForumThreadRow>(
        "SELECT ft.*, a.email as author_name
         FROM forum_threads ft
         LEFT JOIN accounts a ON ft.author_id = a.id
         WHERE ft.id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(crate::error::ApiError::NotFound("Thread not found".to_string()))?;

    let posts = sqlx::query_as::<_, ForumPostRow>(
        "SELECT fp.*, a.email as author_name
         FROM forum_posts fp
         LEFT JOIN accounts a ON fp.author_id = a.id
         WHERE fp.thread_id = $1
         ORDER BY fp.created_at"
    )
    .bind(id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(ThreadWithPosts {
        thread: thread.into(),
        posts: posts.into_iter().map(Into::into).collect(),
    }))
}

/// Thread with posts
#[derive(Debug, Serialize)]
pub struct ThreadWithPosts {
    pub thread: ForumThread,
    pub posts: Vec<ForumPost>,
}

/// Forum post
#[derive(Debug, Serialize)]
pub struct ForumPost {
    pub id: i32,
    pub content: String,
    pub author_name: Option<String>,
    pub character_name: Option<String>,
    pub is_first_post: bool,
    pub edited_by: Option<String>,
    pub edited_at: Option<String>,
    pub created_at: String,
}

/// Create thread request
#[derive(Debug, Deserialize)]
pub struct CreateThreadRequest {
    pub category_id: i32,
    pub title: String,
    pub content: String,
    pub character_name: Option<String>,
}

/// Create thread
pub async fn create_thread(
    State(_state): State<Arc<AppState>>,
    axum::extract::Request(request): axum::extract::Request,
    Json(_body): Json<CreateThreadRequest>,
) -> ApiResult<Json<ForumThread>> {
    let _claims = crate::middleware::get_claims(&request)
        .ok_or(crate::error::ApiError::Unauthorized)?;

    // Implementation would create thread and first post
    Err(crate::error::ApiError::BadRequest("Not implemented".to_string()))
}

/// Create post request
#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub content: String,
    pub character_name: Option<String>,
}

/// Create post
pub async fn create_post(
    State(_state): State<Arc<AppState>>,
    axum::extract::Request(request): axum::extract::Request,
    Path(_thread_id): Path<i32>,
    Json(_body): Json<CreatePostRequest>,
) -> ApiResult<Json<ForumPost>> {
    let _claims = crate::middleware::get_claims(&request)
        .ok_or(crate::error::ApiError::Unauthorized)?;

    // Implementation would create post
    Err(crate::error::ApiError::BadRequest("Not implemented".to_string()))
}

// Helper types

#[derive(sqlx::FromRow)]
struct ForumCategoryRow {
    id: i32,
    name: String,
    description: Option<String>,
    position: i32,
    is_locked: bool,
    thread_count: i64,
}

#[derive(sqlx::FromRow)]
struct ForumThreadRow {
    id: i32,
    category_id: i32,
    title: String,
    author_name: Option<String>,
    is_sticky: bool,
    is_locked: bool,
    view_count: i32,
    reply_count: i32,
    last_reply_at: Option<chrono::DateTime<chrono::Utc>>,
    last_reply_by: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl From<ForumThreadRow> for ForumThread {
    fn from(row: ForumThreadRow) -> Self {
        ForumThread {
            id: row.id,
            category_id: row.category_id,
            title: row.title,
            author_name: row.author_name,
            is_sticky: row.is_sticky,
            is_locked: row.is_locked,
            view_count: row.view_count,
            reply_count: row.reply_count,
            last_reply_at: row.last_reply_at.map(|t| t.to_rfc3339()),
            last_reply_by: row.last_reply_by,
            created_at: row.created_at.to_rfc3339(),
        }
    }
}

#[derive(sqlx::FromRow)]
struct ForumPostRow {
    id: i32,
    content: String,
    author_name: Option<String>,
    character_name: Option<String>,
    is_first_post: bool,
    edited_by: Option<String>,
    edited_at: Option<chrono::DateTime<chrono::Utc>>,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl From<ForumPostRow> for ForumPost {
    fn from(row: ForumPostRow) -> Self {
        ForumPost {
            id: row.id,
            content: row.content,
            author_name: row.author_name,
            character_name: row.character_name,
            is_first_post: row.is_first_post,
            edited_by: row.edited_by,
            edited_at: row.edited_at.map(|t| t.to_rfc3339()),
            created_at: row.created_at.to_rfc3339(),
        }
    }
}
