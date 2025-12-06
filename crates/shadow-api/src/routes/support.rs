//! Support ticket endpoints

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

/// Support ticket category
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "ticket_category", rename_all = "lowercase")]
pub enum TicketCategory {
    Technical,
    Billing,
    Account,
    Report,
    Other,
}

/// Support ticket status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "ticket_status", rename_all = "lowercase")]
pub enum TicketStatus {
    Open,
    Pending,
    Resolved,
    Closed,
}

/// Support ticket priority
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "ticket_priority", rename_all = "lowercase")]
pub enum TicketPriority {
    Low,
    Medium,
    High,
    Urgent,
}

/// Support ticket message
#[derive(Debug, Serialize, ToSchema)]
pub struct TicketMessage {
    pub id: Uuid,
    pub content: String,
    pub author_type: String,
    pub created_at: DateTime<Utc>,
    pub attachments: Vec<String>,
}

#[derive(Debug, FromRow)]
struct TicketMessageRow {
    id: Uuid,
    content: String,
    author_type: String,
    created_at: DateTime<Utc>,
}

/// Support ticket
#[derive(Debug, Serialize, ToSchema)]
pub struct SupportTicket {
    pub id: Uuid,
    pub subject: String,
    pub category: TicketCategory,
    pub status: TicketStatus,
    pub priority: TicketPriority,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub messages: Vec<TicketMessage>,
}

#[derive(Debug, FromRow)]
struct TicketRow {
    id: Uuid,
    subject: String,
    category: TicketCategory,
    status: TicketStatus,
    priority: TicketPriority,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

/// Ticket query parameters
#[derive(Debug, Deserialize)]
pub struct TicketQuery {
    pub status: Option<TicketStatus>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

/// Create ticket request
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTicketRequest {
    pub subject: String,
    pub category: TicketCategory,
    pub message: String,
}

/// Reply to ticket request
#[derive(Debug, Deserialize, ToSchema)]
pub struct ReplyTicketRequest {
    pub message: String,
}

/// Paginated response
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedTickets {
    pub data: Vec<SupportTicket>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

/// FAQ item
#[derive(Debug, Serialize, ToSchema)]
pub struct FaqItem {
    pub question: String,
    pub answer: String,
}

/// FAQ category
#[derive(Debug, Serialize, ToSchema)]
pub struct FaqCategory {
    pub category: String,
    pub items: Vec<FaqItem>,
}

/// List user's support tickets
#[utoipa::path(
    get,
    path = "/api/v1/support/tickets",
    params(
        ("status" = Option<TicketStatus>, Query, description = "Filter by status"),
        ("page" = Option<u32>, Query, description = "Page number"),
        ("page_size" = Option<u32>, Query, description = "Results per page")
    ),
    responses(
        (status = 200, description = "User's tickets", body = PaginatedTickets)
    ),
    security(("bearer_auth" = [])),
    tag = "support"
)]
pub async fn list_tickets(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
    Query(query): Query<TicketQuery>,
) -> ApiResult<Json<PaginatedTickets>> {
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(20).min(100);
    let offset = (page - 1) * page_size;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM support_tickets 
         WHERE account_id = $1
           AND ($2::ticket_status IS NULL OR status = $2)"
    )
    .bind(&claims.sub)
    .bind(query.status)
    .fetch_one(&state.db)
    .await?;

    let rows = sqlx::query_as::<_, TicketRow>(
        "SELECT id, subject, category, status, priority, created_at, updated_at
         FROM support_tickets
         WHERE account_id = $1
           AND ($2::ticket_status IS NULL OR status = $2)
         ORDER BY updated_at DESC
         LIMIT $3 OFFSET $4"
    )
    .bind(&claims.sub)
    .bind(query.status)
    .bind(page_size as i64)
    .bind(offset as i64)
    .fetch_all(&state.db)
    .await?;

    let mut tickets = Vec::new();
    for row in rows {
        let messages = load_ticket_messages(&state, row.id).await?;
        tickets.push(SupportTicket {
            id: row.id,
            subject: row.subject,
            category: row.category,
            status: row.status,
            priority: row.priority,
            created_at: row.created_at,
            updated_at: row.updated_at,
            messages,
        });
    }

    Ok(Json(PaginatedTickets {
        data: tickets,
        total: total.0,
        page,
        page_size,
        total_pages: ((total.0 as f64) / (page_size as f64)).ceil() as u32,
    }))
}

/// Get a specific ticket
#[utoipa::path(
    get,
    path = "/api/v1/support/tickets/{id}",
    params(
        ("id" = Uuid, Path, description = "Ticket ID")
    ),
    responses(
        (status = 200, description = "Ticket details", body = SupportTicket)
    ),
    security(("bearer_auth" = [])),
    tag = "support"
)]
pub async fn get_ticket(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<SupportTicket>> {
    let row = sqlx::query_as::<_, TicketRow>(
        "SELECT id, subject, category, status, priority, created_at, updated_at
         FROM support_tickets
         WHERE id = $1 AND account_id = $2"
    )
    .bind(id)
    .bind(&claims.sub)
    .fetch_optional(&state.db)
    .await?
    .ok_or(crate::error::ApiError::NotFound("Ticket not found".to_string()))?;

    let messages = load_ticket_messages(&state, row.id).await?;

    Ok(Json(SupportTicket {
        id: row.id,
        subject: row.subject,
        category: row.category,
        status: row.status,
        priority: row.priority,
        created_at: row.created_at,
        updated_at: row.updated_at,
        messages,
    }))
}

/// Create a new support ticket
#[utoipa::path(
    post,
    path = "/api/v1/support/tickets",
    request_body = CreateTicketRequest,
    responses(
        (status = 201, description = "Ticket created", body = SupportTicket)
    ),
    security(("bearer_auth" = [])),
    tag = "support"
)]
pub async fn create_ticket(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
    Json(req): Json<CreateTicketRequest>,
) -> ApiResult<Json<SupportTicket>> {
    let ticket_id = Uuid::new_v4();
    let now = Utc::now();

    // Create ticket
    sqlx::query(
        "INSERT INTO support_tickets (id, account_id, subject, category, status, priority, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
    )
    .bind(ticket_id)
    .bind(&claims.sub)
    .bind(&req.subject)
    .bind(req.category)
    .bind(TicketStatus::Open)
    .bind(TicketPriority::Medium)
    .bind(now)
    .bind(now)
    .execute(&state.db)
    .await?;

    // Add initial message
    let message_id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO ticket_messages (id, ticket_id, content, author_type, created_at)
         VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(message_id)
    .bind(ticket_id)
    .bind(&req.message)
    .bind("user")
    .bind(now)
    .execute(&state.db)
    .await?;

    Ok(Json(SupportTicket {
        id: ticket_id,
        subject: req.subject,
        category: req.category,
        status: TicketStatus::Open,
        priority: TicketPriority::Medium,
        created_at: now,
        updated_at: now,
        messages: vec![TicketMessage {
            id: message_id,
            content: req.message,
            author_type: "user".to_string(),
            created_at: now,
            attachments: vec![],
        }],
    }))
}

/// Reply to a ticket
#[utoipa::path(
    post,
    path = "/api/v1/support/tickets/{id}/reply",
    params(
        ("id" = Uuid, Path, description = "Ticket ID")
    ),
    request_body = ReplyTicketRequest,
    responses(
        (status = 200, description = "Reply added", body = SupportTicket)
    ),
    security(("bearer_auth" = [])),
    tag = "support"
)]
pub async fn reply_to_ticket(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
    Path(id): Path<Uuid>,
    Json(req): Json<ReplyTicketRequest>,
) -> ApiResult<Json<SupportTicket>> {
    // Verify ticket belongs to user
    let ticket = sqlx::query_as::<_, TicketRow>(
        "SELECT id, subject, category, status, priority, created_at, updated_at
         FROM support_tickets
         WHERE id = $1 AND account_id = $2"
    )
    .bind(id)
    .bind(&claims.sub)
    .fetch_optional(&state.db)
    .await?
    .ok_or(crate::error::ApiError::NotFound("Ticket not found".to_string()))?;

    if ticket.status == TicketStatus::Closed {
        return Err(crate::error::ApiError::BadRequest("Cannot reply to closed ticket".to_string()));
    }

    let now = Utc::now();
    let message_id = Uuid::new_v4();

    // Add message
    sqlx::query(
        "INSERT INTO ticket_messages (id, ticket_id, content, author_type, created_at)
         VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(message_id)
    .bind(id)
    .bind(&req.message)
    .bind("user")
    .bind(now)
    .execute(&state.db)
    .await?;

    // Update ticket timestamp and status
    sqlx::query(
        "UPDATE support_tickets SET updated_at = $1, status = $2 WHERE id = $3"
    )
    .bind(now)
    .bind(TicketStatus::Open)
    .bind(id)
    .execute(&state.db)
    .await?;

    let messages = load_ticket_messages(&state, id).await?;

    Ok(Json(SupportTicket {
        id: ticket.id,
        subject: ticket.subject,
        category: ticket.category,
        status: TicketStatus::Open,
        priority: ticket.priority,
        created_at: ticket.created_at,
        updated_at: now,
        messages,
    }))
}

/// Close a ticket
#[utoipa::path(
    patch,
    path = "/api/v1/support/tickets/{id}/close",
    params(
        ("id" = Uuid, Path, description = "Ticket ID")
    ),
    responses(
        (status = 200, description = "Ticket closed")
    ),
    security(("bearer_auth" = [])),
    tag = "support"
)]
pub async fn close_ticket(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let result = sqlx::query(
        "UPDATE support_tickets SET status = $1, updated_at = $2
         WHERE id = $3 AND account_id = $4"
    )
    .bind(TicketStatus::Closed)
    .bind(Utc::now())
    .bind(id)
    .bind(&claims.sub)
    .execute(&state.db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(crate::error::ApiError::NotFound("Ticket not found".to_string()));
    }

    Ok(Json(serde_json::json!({ "success": true })))
}

/// Get FAQ
#[utoipa::path(
    get,
    path = "/api/v1/support/faq",
    responses(
        (status = 200, description = "FAQ categories", body = Vec<FaqCategory>)
    ),
    tag = "support"
)]
pub async fn get_faq(
    State(_state): State<Arc<AppState>>,
) -> ApiResult<Json<Vec<FaqCategory>>> {
    // Static FAQ for now - could be database-driven
    Ok(Json(vec![
        FaqCategory {
            category: "Getting Started".to_string(),
            items: vec![
                FaqItem {
                    question: "How do I create an account?".to_string(),
                    answer: "Click the 'Create Account' button on the homepage and follow the registration process.".to_string(),
                },
                FaqItem {
                    question: "How do I download the client?".to_string(),
                    answer: "Visit our Downloads page to get the latest client for your operating system.".to_string(),
                },
                FaqItem {
                    question: "Which realm should I choose?".to_string(),
                    answer: "Each realm has different rates and PvP rules. Check our Realms page for details on each server.".to_string(),
                },
            ],
        },
        FaqCategory {
            category: "Account & Security".to_string(),
            items: vec![
                FaqItem {
                    question: "How do I enable two-factor authentication?".to_string(),
                    answer: "Go to Account Settings > Security and follow the 2FA setup wizard.".to_string(),
                },
                FaqItem {
                    question: "I forgot my password, what do I do?".to_string(),
                    answer: "Click 'Forgot Password' on the login page and enter your email to receive a reset link.".to_string(),
                },
                FaqItem {
                    question: "How do I link my wallet?".to_string(),
                    answer: "Navigate to Account > Wallet Integration and connect your Web3 wallet.".to_string(),
                },
            ],
        },
        FaqCategory {
            category: "Gameplay".to_string(),
            items: vec![
                FaqItem {
                    question: "How does the Forge system work?".to_string(),
                    answer: "The Forge allows you to upgrade items using Dust and Cores. Higher tiers grant bonus stats.".to_string(),
                },
                FaqItem {
                    question: "What are Hunting Tasks?".to_string(),
                    answer: "Hunting Tasks are repeatable quests to kill specific monsters for rewards and Task Points.".to_string(),
                },
                FaqItem {
                    question: "How do I join a guild?".to_string(),
                    answer: "You can apply to guilds through the Guild page or receive an invitation from a guild leader.".to_string(),
                },
            ],
        },
        FaqCategory {
            category: "Premium & Shop".to_string(),
            items: vec![
                FaqItem {
                    question: "What benefits does Premium give?".to_string(),
                    answer: "Premium includes bonus XP, access to exclusive areas, priority login, and more perks.".to_string(),
                },
                FaqItem {
                    question: "How do I purchase coins?".to_string(),
                    answer: "Visit the Shop page and select a coin package. We accept crypto and traditional payments.".to_string(),
                },
                FaqItem {
                    question: "Are NFT items tradeable?".to_string(),
                    answer: "Yes! NFT items can be traded on supported marketplaces or transferred between accounts.".to_string(),
                },
            ],
        },
    ]))
}

/// Helper to load messages for a ticket
async fn load_ticket_messages(state: &AppState, ticket_id: Uuid) -> Result<Vec<TicketMessage>, sqlx::Error> {
    let rows = sqlx::query_as::<_, TicketMessageRow>(
        "SELECT id, content, author_type, created_at
         FROM ticket_messages
         WHERE ticket_id = $1
         ORDER BY created_at ASC"
    )
    .bind(ticket_id)
    .fetch_all(&state.db)
    .await?;

    Ok(rows.into_iter().map(|r| TicketMessage {
        id: r.id,
        content: r.content,
        author_type: r.author_type,
        created_at: r.created_at,
        attachments: vec![],
    }).collect())
}
