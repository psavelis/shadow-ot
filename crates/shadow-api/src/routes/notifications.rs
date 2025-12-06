//! User notification endpoints

use crate::auth::JwtClaims;
use crate::state::AppState;
use crate::ApiResult;
use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

/// Notification type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "notification_type", rename_all = "lowercase")]
pub enum NotificationType {
    Levelup,
    Trade,
    Achievement,
    Guild,
    System,
    Message,
    Friend,
    Death,
    Market,
}

/// User notification
#[derive(Debug, Serialize, ToSchema)]
pub struct Notification {
    pub id: Uuid,
    pub notification_type: NotificationType,
    pub title: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub read: bool,
    pub action_url: Option<String>,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, FromRow)]
struct NotificationRow {
    id: Uuid,
    notification_type: NotificationType,
    title: String,
    message: String,
    created_at: DateTime<Utc>,
    read_at: Option<DateTime<Utc>>,
    action_url: Option<String>,
    data: Option<serde_json::Value>,
}

/// Query parameters for notifications
#[derive(Debug, Deserialize)]
pub struct NotificationQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub unread_only: Option<bool>,
    pub notification_type: Option<String>,
}

/// Paginated notifications
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedNotifications {
    pub data: Vec<Notification>,
    pub total: i64,
    pub unread_count: i64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

/// Mark read response
#[derive(Debug, Serialize, ToSchema)]
pub struct MarkReadResponse {
    pub success: bool,
    pub marked_count: i64,
}

/// Get user notifications
#[utoipa::path(
    get,
    path = "/api/v1/users/me/notifications",
    params(
        ("page" = Option<u32>, Query, description = "Page number"),
        ("page_size" = Option<u32>, Query, description = "Items per page"),
        ("unread_only" = Option<bool>, Query, description = "Filter to unread only"),
        ("notification_type" = Option<String>, Query, description = "Filter by type")
    ),
    responses(
        (status = 200, description = "User notifications", body = PaginatedNotifications)
    ),
    security(("bearer_auth" = [])),
    tag = "notifications"
)]
pub async fn get_notifications(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
    Query(query): Query<NotificationQuery>,
) -> ApiResult<Json<PaginatedNotifications>> {
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(20).min(100);
    let offset = (page - 1) * page_size;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM notifications 
         WHERE account_id = $1
           AND ($2::boolean IS NULL OR ($2 = true AND read_at IS NULL))
           AND ($3::text IS NULL OR notification_type::text = $3)"
    )
    .bind(&claims.sub)
    .bind(query.unread_only)
    .bind(&query.notification_type)
    .fetch_one(&state.db)
    .await?;

    let unread: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM notifications WHERE account_id = $1 AND read_at IS NULL"
    )
    .bind(&claims.sub)
    .fetch_one(&state.db)
    .await?;

    let rows: Vec<NotificationRow> = sqlx::query_as(
        "SELECT id, notification_type, title, message, created_at, read_at, action_url, data
         FROM notifications
         WHERE account_id = $1
           AND ($2::boolean IS NULL OR ($2 = true AND read_at IS NULL))
           AND ($3::text IS NULL OR notification_type::text = $3)
         ORDER BY created_at DESC
         LIMIT $4 OFFSET $5"
    )
    .bind(&claims.sub)
    .bind(query.unread_only)
    .bind(&query.notification_type)
    .bind(page_size as i64)
    .bind(offset as i64)
    .fetch_all(&state.db)
    .await?;

    let notifications = rows.into_iter().map(|r| Notification {
        id: r.id,
        notification_type: r.notification_type,
        title: r.title,
        message: r.message,
        timestamp: r.created_at,
        read: r.read_at.is_some(),
        action_url: r.action_url,
        data: r.data,
    }).collect();

    Ok(Json(PaginatedNotifications {
        data: notifications,
        total: total.0,
        unread_count: unread.0,
        page,
        page_size,
        total_pages: ((total.0 as f64) / (page_size as f64)).ceil() as u32,
    }))
}

/// Mark notification as read
#[utoipa::path(
    patch,
    path = "/api/v1/users/me/notifications/{id}/read",
    params(
        ("id" = Uuid, Path, description = "Notification ID")
    ),
    responses(
        (status = 200, description = "Notification marked as read")
    ),
    security(("bearer_auth" = [])),
    tag = "notifications"
)]
pub async fn mark_notification_read(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    sqlx::query(
        "UPDATE notifications SET read_at = CURRENT_TIMESTAMP
         WHERE id = $1 AND account_id = $2 AND read_at IS NULL"
    )
    .bind(id)
    .bind(&claims.sub)
    .execute(&state.db)
    .await?;

    Ok(Json(serde_json::json!({
        "success": true
    })))
}

/// Mark all notifications as read
#[utoipa::path(
    post,
    path = "/api/v1/users/me/notifications/read-all",
    responses(
        (status = 200, description = "All notifications marked as read", body = MarkReadResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "notifications"
)]
pub async fn mark_all_read(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
) -> ApiResult<Json<MarkReadResponse>> {
    let result = sqlx::query(
        "UPDATE notifications SET read_at = CURRENT_TIMESTAMP
         WHERE account_id = $1 AND read_at IS NULL"
    )
    .bind(&claims.sub)
    .execute(&state.db)
    .await?;

    Ok(Json(MarkReadResponse {
        success: true,
        marked_count: result.rows_affected() as i64,
    }))
}

/// Delete notification
#[utoipa::path(
    delete,
    path = "/api/v1/users/me/notifications/{id}",
    params(
        ("id" = Uuid, Path, description = "Notification ID")
    ),
    responses(
        (status = 200, description = "Notification deleted")
    ),
    security(("bearer_auth" = [])),
    tag = "notifications"
)]
pub async fn delete_notification(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    sqlx::query(
        "DELETE FROM notifications WHERE id = $1 AND account_id = $2"
    )
    .bind(id)
    .bind(&claims.sub)
    .execute(&state.db)
    .await?;

    Ok(Json(serde_json::json!({
        "success": true
    })))
}

/// Get unread notification count
#[utoipa::path(
    get,
    path = "/api/v1/users/me/notifications/count",
    responses(
        (status = 200, description = "Unread notification count")
    ),
    security(("bearer_auth" = [])),
    tag = "notifications"
)]
pub async fn get_unread_count(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
) -> ApiResult<Json<serde_json::Value>> {
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM notifications WHERE account_id = $1 AND read_at IS NULL"
    )
    .bind(&claims.sub)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(serde_json::json!({
        "count": count.0
    })))
}
