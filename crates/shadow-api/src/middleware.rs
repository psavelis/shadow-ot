//! API middleware - authentication, rate limiting, logging

use crate::auth::{validate_token, JwtClaims};
use crate::error::ApiError;
use crate::state::AppState;
use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

/// Extract JWT claims from request
pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Result<Response, ApiError> {
    // Get authorization header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    // Extract token
    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => &header[7..],
        _ => return Err(ApiError::Unauthorized),
    };

    // Validate token
    let claims = validate_token(token, &state.auth_config.jwt_secret)?;

    // Store claims in request extensions
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

/// Optional authentication - doesn't fail if no token
pub async fn optional_auth_middleware(
    State(state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Response {
    // Get authorization header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    // Extract and validate token if present
    if let Some(header) = auth_header {
        if header.starts_with("Bearer ") {
            let token = &header[7..];
            if let Ok(claims) = validate_token(token, &state.auth_config.jwt_secret) {
                request.extensions_mut().insert(claims);
            }
        }
    }

    next.run(request).await
}

/// Admin-only middleware
pub async fn admin_middleware(
    State(state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Result<Response, ApiError> {
    // Get claims (must be authenticated first)
    let claims = request
        .extensions()
        .get::<JwtClaims>()
        .ok_or(ApiError::Unauthorized)?;

    if !claims.is_admin() {
        return Err(ApiError::Forbidden);
    }

    Ok(next.run(request).await)
}

/// Rate limiting state (simplified - use tower-governor or similar in production)
pub struct RateLimitState {
    requests: std::collections::HashMap<String, Vec<i64>>,
    limit: usize,
    window_secs: i64,
}

impl RateLimitState {
    pub fn new(limit: usize, window_secs: i64) -> Self {
        Self {
            requests: std::collections::HashMap::new(),
            limit,
            window_secs,
        }
    }

    pub fn check(&mut self, key: &str) -> bool {
        let now = chrono::Utc::now().timestamp();
        let window_start = now - self.window_secs;

        let timestamps = self.requests.entry(key.to_string()).or_default();

        // Remove old timestamps
        timestamps.retain(|&t| t > window_start);

        // Check limit
        if timestamps.len() >= self.limit {
            return false;
        }

        // Add current request
        timestamps.push(now);
        true
    }
}

/// Extract authenticated user claims
pub fn get_claims(request: &Request) -> Option<&JwtClaims> {
    request.extensions().get::<JwtClaims>()
}

/// Require authentication in handler
#[macro_export]
macro_rules! require_auth {
    ($request:expr) => {
        match $crate::middleware::get_claims(&$request) {
            Some(claims) => claims,
            None => return Err($crate::error::ApiError::Unauthorized),
        }
    };
}
