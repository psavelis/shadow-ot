//! Premium subscription and coin shop endpoints

use crate::auth::JwtClaims;
use crate::state::AppState;
use crate::ApiResult;
use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

/// Premium plan type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, sqlx::Type, PartialEq)]
#[sqlx(type_name = "premium_plan", rename_all = "lowercase")]
pub enum PremiumPlan {
    Monthly,
    Quarterly,
    Yearly,
}

/// Premium status response
#[derive(Debug, Serialize, ToSchema)]
pub struct PremiumStatus {
    pub active: bool,
    pub plan: Option<PremiumPlan>,
    pub expires_at: Option<DateTime<Utc>>,
    pub coins: i64,
    pub days_remaining: Option<i32>,
    pub auto_renew: bool,
}

#[derive(Debug, FromRow)]
struct PremiumRow {
    premium_until: Option<DateTime<Utc>>,
    premium_plan: Option<String>,
    coins: i64,
    auto_renew: bool,
}

/// Transaction type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "transaction_type", rename_all = "lowercase")]
pub enum TransactionType {
    Subscription,
    Coins,
    CoinsPurchase,
    Refund,
}

/// Transaction status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "transaction_status", rename_all = "lowercase")]
pub enum TransactionStatus {
    Completed,
    Pending,
    Failed,
}

/// Premium transaction history entry
#[derive(Debug, Serialize, ToSchema)]
pub struct PremiumTransaction {
    pub id: Uuid,
    pub transaction_type: String,
    pub description: String,
    pub amount: f64,
    pub currency: String,
    pub date: DateTime<Utc>,
    pub status: String,
}

#[derive(Debug, FromRow)]
struct TransactionRow {
    id: Uuid,
    transaction_type: String,
    description: String,
    amount: f64,
    currency: String,
    created_at: DateTime<Utc>,
    status: String,
}

/// Purchase premium request
#[derive(Debug, Deserialize, ToSchema)]
pub struct PurchasePremiumRequest {
    pub plan: String,
}

/// Purchase premium response
#[derive(Debug, Serialize, ToSchema)]
pub struct PurchasePremiumResponse {
    pub success: bool,
    pub expires_at: DateTime<Utc>,
    pub transaction_id: Uuid,
}

/// Coin package
#[derive(Debug, Serialize, ToSchema)]
pub struct CoinPackage {
    pub id: i32,
    pub name: String,
    pub coins: i64,
    pub bonus_coins: i64,
    pub price: f64,
    pub currency: String,
    pub popular: bool,
}

/// Purchase coins request
#[derive(Debug, Deserialize, ToSchema)]
pub struct PurchaseCoinsRequest {
    pub package_id: i32,
}

/// Purchase coins response
#[derive(Debug, Serialize, ToSchema)]
pub struct PurchaseCoinsResponse {
    pub success: bool,
    pub coins_added: i64,
    pub new_balance: i64,
    pub transaction_id: Uuid,
}

/// Query params for history
#[derive(Debug, Deserialize)]
pub struct HistoryQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

/// Paginated transactions
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedTransactions {
    pub data: Vec<PremiumTransaction>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

/// Get premium status
#[utoipa::path(
    get,
    path = "/api/v1/users/me/premium",
    responses(
        (status = 200, description = "Premium status", body = PremiumStatus)
    ),
    security(("bearer_auth" = [])),
    tag = "premium"
)]
pub async fn get_premium_status(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
) -> ApiResult<Json<PremiumStatus>> {
    let row: Option<PremiumRow> = sqlx::query_as(
        "SELECT premium_until, premium_plan, COALESCE(coins, 0) as coins, COALESCE(auto_renew, false) as auto_renew
         FROM accounts WHERE id = $1"
    )
    .bind(&claims.sub)
    .fetch_optional(&state.db)
    .await?;

    let status = match row {
        Some(r) => {
            let now = Utc::now();
            let active = r.premium_until.map(|t| t > now).unwrap_or(false);
            let days_remaining = r.premium_until.map(|t| {
                if t > now {
                    (t - now).num_days() as i32
                } else {
                    0
                }
            });

            let plan = if active {
                r.premium_plan.and_then(|p| match p.to_lowercase().as_str() {
                    "monthly" => Some(PremiumPlan::Monthly),
                    "quarterly" => Some(PremiumPlan::Quarterly),
                    "yearly" => Some(PremiumPlan::Yearly),
                    _ => None,
                })
            } else {
                None
            };

            PremiumStatus {
                active,
                plan,
                expires_at: if active { r.premium_until } else { None },
                coins: r.coins,
                days_remaining: if active { days_remaining } else { None },
                auto_renew: r.auto_renew && active,
            }
        }
        None => PremiumStatus {
            active: false,
            plan: None,
            expires_at: None,
            coins: 0,
            days_remaining: None,
            auto_renew: false,
        },
    };

    Ok(Json(status))
}

/// Purchase premium subscription
#[utoipa::path(
    post,
    path = "/api/v1/users/me/premium/purchase",
    request_body = PurchasePremiumRequest,
    responses(
        (status = 200, description = "Premium purchased", body = PurchasePremiumResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "premium"
)]
pub async fn purchase_premium(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
    Json(request): Json<PurchasePremiumRequest>,
) -> ApiResult<Json<PurchasePremiumResponse>> {
    let (days, price): (i64, f64) = match request.plan.to_lowercase().as_str() {
        "monthly" => (30, 9.99),
        "quarterly" => (90, 24.99),
        "yearly" => (365, 79.99),
        _ => return Err(crate::error::ApiError::BadRequest("Invalid plan".to_string())),
    };

    // Get current premium status
    let current: Option<(Option<DateTime<Utc>>,)> = sqlx::query_as(
        "SELECT premium_until FROM accounts WHERE id = $1"
    )
    .bind(&claims.sub)
    .fetch_optional(&state.db)
    .await?;

    let now = Utc::now();
    let current_expires = current.and_then(|c| c.0).unwrap_or(now);
    let start_from = if current_expires > now { current_expires } else { now };
    let new_expires = start_from + Duration::days(days);

    let tx_id = Uuid::new_v4();

    // Update account
    sqlx::query(
        "UPDATE accounts SET premium_until = $2, premium_plan = $3 WHERE id = $1"
    )
    .bind(&claims.sub)
    .bind(new_expires)
    .bind(&request.plan.to_lowercase())
    .execute(&state.db)
    .await?;

    // Record transaction
    sqlx::query(
        "INSERT INTO premium_transactions (id, account_id, transaction_type, description, amount, currency, status, created_at)
         VALUES ($1, $2, 'subscription', $3, $4, 'USD', 'completed', CURRENT_TIMESTAMP)"
    )
    .bind(tx_id)
    .bind(&claims.sub)
    .bind(format!("Premium {} subscription", request.plan))
    .bind(price)
    .execute(&state.db)
    .await?;

    Ok(Json(PurchasePremiumResponse {
        success: true,
        expires_at: new_expires,
        transaction_id: tx_id,
    }))
}

/// Get coin packages
#[utoipa::path(
    get,
    path = "/api/v1/users/me/premium/packages",
    responses(
        (status = 200, description = "Available coin packages", body = Vec<CoinPackage>)
    ),
    tag = "premium"
)]
pub async fn get_coin_packages(
    State(_state): State<Arc<AppState>>,
) -> ApiResult<Json<Vec<CoinPackage>>> {
    // Return available coin packages
    let packages = vec![
        CoinPackage {
            id: 1,
            name: "Starter Pack".to_string(),
            coins: 250,
            bonus_coins: 0,
            price: 4.99,
            currency: "USD".to_string(),
            popular: false,
        },
        CoinPackage {
            id: 2,
            name: "Adventurer Pack".to_string(),
            coins: 750,
            bonus_coins: 50,
            price: 14.99,
            currency: "USD".to_string(),
            popular: true,
        },
        CoinPackage {
            id: 3,
            name: "Hero Pack".to_string(),
            coins: 1500,
            bonus_coins: 150,
            price: 29.99,
            currency: "USD".to_string(),
            popular: false,
        },
        CoinPackage {
            id: 4,
            name: "Legend Pack".to_string(),
            coins: 3000,
            bonus_coins: 500,
            price: 49.99,
            currency: "USD".to_string(),
            popular: false,
        },
        CoinPackage {
            id: 5,
            name: "Ultimate Pack".to_string(),
            coins: 7500,
            bonus_coins: 1500,
            price: 99.99,
            currency: "USD".to_string(),
            popular: false,
        },
    ];

    Ok(Json(packages))
}

/// Purchase coins
#[utoipa::path(
    post,
    path = "/api/v1/users/me/premium/coins",
    request_body = PurchaseCoinsRequest,
    responses(
        (status = 200, description = "Coins purchased", body = PurchaseCoinsResponse)
    ),
    security(("bearer_auth" = [])),
    tag = "premium"
)]
pub async fn purchase_coins(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
    Json(request): Json<PurchaseCoinsRequest>,
) -> ApiResult<Json<PurchaseCoinsResponse>> {
    // Package definitions
    let (coins, bonus, price) = match request.package_id {
        1 => (250i64, 0i64, 4.99f64),
        2 => (750, 50, 14.99),
        3 => (1500, 150, 29.99),
        4 => (3000, 500, 49.99),
        5 => (7500, 1500, 99.99),
        _ => return Err(crate::error::ApiError::NotFound("Package not found".to_string())),
    };

    let total_coins = coins + bonus;
    let tx_id = Uuid::new_v4();

    // Update coins
    let result: (i64,) = sqlx::query_as(
        "UPDATE accounts SET coins = COALESCE(coins, 0) + $2 WHERE id = $1 RETURNING coins"
    )
    .bind(&claims.sub)
    .bind(total_coins)
    .fetch_one(&state.db)
    .await?;

    // Record transaction
    sqlx::query(
        "INSERT INTO premium_transactions (id, account_id, transaction_type, description, amount, currency, status, created_at)
         VALUES ($1, $2, 'coins', $3, $4, 'USD', 'completed', CURRENT_TIMESTAMP)"
    )
    .bind(tx_id)
    .bind(&claims.sub)
    .bind(format!("Purchased {} coins (+ {} bonus)", coins, bonus))
    .bind(price)
    .execute(&state.db)
    .await?;

    Ok(Json(PurchaseCoinsResponse {
        success: true,
        coins_added: total_coins,
        new_balance: result.0,
        transaction_id: tx_id,
    }))
}

/// Get premium transaction history
#[utoipa::path(
    get,
    path = "/api/v1/users/me/premium/history",
    params(
        ("page" = Option<u32>, Query, description = "Page number"),
        ("page_size" = Option<u32>, Query, description = "Items per page")
    ),
    responses(
        (status = 200, description = "Transaction history", body = PaginatedTransactions)
    ),
    security(("bearer_auth" = [])),
    tag = "premium"
)]
pub async fn get_premium_history(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
    Query(query): Query<HistoryQuery>,
) -> ApiResult<Json<PaginatedTransactions>> {
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(20).min(100);
    let offset = (page - 1) * page_size;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM premium_transactions WHERE account_id = $1"
    )
    .bind(&claims.sub)
    .fetch_one(&state.db)
    .await?;

    let rows: Vec<TransactionRow> = sqlx::query_as(
        "SELECT id, transaction_type, description, amount, currency, created_at, status
         FROM premium_transactions
         WHERE account_id = $1
         ORDER BY created_at DESC
         LIMIT $2 OFFSET $3"
    )
    .bind(&claims.sub)
    .bind(page_size as i64)
    .bind(offset as i64)
    .fetch_all(&state.db)
    .await?;

    let transactions = rows.into_iter().map(|r| PremiumTransaction {
        id: r.id,
        transaction_type: r.transaction_type,
        description: r.description,
        amount: r.amount,
        currency: r.currency,
        date: r.created_at,
        status: r.status,
    }).collect();

    Ok(Json(PaginatedTransactions {
        data: transactions,
        total: total.0,
        page,
        page_size,
        total_pages: ((total.0 as f64) / (page_size as f64)).ceil() as u32,
    }))
}

/// Toggle auto-renewal
#[utoipa::path(
    post,
    path = "/api/v1/users/me/premium/auto-renew",
    responses(
        (status = 200, description = "Auto-renewal toggled")
    ),
    security(("bearer_auth" = [])),
    tag = "premium"
)]
pub async fn toggle_auto_renew(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
) -> ApiResult<Json<serde_json::Value>> {
    sqlx::query(
        "UPDATE accounts SET auto_renew = NOT COALESCE(auto_renew, false) WHERE id = $1"
    )
    .bind(&claims.sub)
    .execute(&state.db)
    .await?;

    let result: (bool,) = sqlx::query_as(
        "SELECT COALESCE(auto_renew, false) FROM accounts WHERE id = $1"
    )
    .bind(&claims.sub)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(serde_json::json!({
        "success": true,
        "auto_renew": result.0
    })))
}

/// Cancel premium subscription
#[utoipa::path(
    delete,
    path = "/api/v1/users/me/premium",
    responses(
        (status = 200, description = "Premium cancelled")
    ),
    security(("bearer_auth" = [])),
    tag = "premium"
)]
pub async fn cancel_premium(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<JwtClaims>,
) -> ApiResult<Json<serde_json::Value>> {
    // Don't remove existing time, just disable auto-renew
    sqlx::query(
        "UPDATE accounts SET auto_renew = false WHERE id = $1"
    )
    .bind(&claims.sub)
    .execute(&state.db)
    .await?;

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Auto-renewal cancelled. Your premium will remain active until expiration."
    })))
}
