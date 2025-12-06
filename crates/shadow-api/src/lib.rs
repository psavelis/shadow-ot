//! Shadow OT REST API
//!
//! Provides HTTP endpoints for the web frontend, admin panel, and external integrations.

pub mod auth;
pub mod error;
pub mod middleware;
pub mod routes;
pub mod state;

pub use auth::{AuthConfig, JwtClaims};
pub use error::ApiError;
pub use state::AppState;

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;
use tower_http::compression::CompressionLayer;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// API Result type
pub type ApiResult<T> = std::result::Result<T, ApiError>;

/// OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        routes::health::health_check,
        routes::auth::login,
        routes::auth::register,
        routes::auth::logout,
        routes::auth::refresh_token,
        routes::accounts::get_account,
        routes::accounts::update_account,
        routes::characters::list_characters,
        routes::characters::get_character,
        routes::characters::create_character,
        routes::characters::delete_character,
        routes::realms::list_realms,
        routes::realms::get_realm,
        routes::highscores::get_highscores,
        routes::guilds::list_guilds,
        routes::guilds::get_guild,
        routes::market::list_offers,
        routes::news::list_news,
    ),
    components(
        schemas(
            routes::auth::LoginRequest,
            routes::auth::LoginResponse,
            routes::auth::RegisterRequest,
            routes::accounts::AccountResponse,
            routes::characters::CharacterResponse,
            routes::characters::CreateCharacterRequest,
            routes::realms::RealmResponse,
            routes::highscores::HighscoreEntry,
            routes::guilds::GuildResponse,
            routes::market::MarketOffer,
            routes::news::NewsArticle,
        )
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "auth", description = "Authentication endpoints"),
        (name = "accounts", description = "Account management"),
        (name = "characters", description = "Character management"),
        (name = "realms", description = "Realm information"),
        (name = "highscores", description = "Highscore listings"),
        (name = "guilds", description = "Guild information"),
        (name = "market", description = "In-game market"),
        (name = "news", description = "News and announcements"),
    ),
    info(
        title = "Shadow OT API",
        version = "1.0.0",
        description = "REST API for Shadow OT game server",
        license(name = "MIT"),
    )
)]
pub struct ApiDoc;

/// Create the API router
pub fn create_router(state: Arc<AppState>) -> Router {
    // Build routes
    let api_routes = Router::new()
        // Health
        .route("/health", get(routes::health::health_check))
        // Auth
        .route("/auth/login", post(routes::auth::login))
        .route("/auth/register", post(routes::auth::register))
        .route("/auth/logout", post(routes::auth::logout))
        .route("/auth/refresh", post(routes::auth::refresh_token))
        .route("/auth/verify-email", post(routes::auth::verify_email))
        .route("/auth/forgot-password", post(routes::auth::forgot_password))
        .route("/auth/reset-password", post(routes::auth::reset_password))
        // Accounts
        .route("/account", get(routes::accounts::get_account))
        .route("/account", put(routes::accounts::update_account))
        .route("/account/password", put(routes::accounts::change_password))
        .route("/account/sessions", get(routes::accounts::list_sessions))
        .route("/account/sessions/:id", delete(routes::accounts::revoke_session))
        // Characters
        .route("/characters", get(routes::characters::list_characters))
        .route("/characters", post(routes::characters::create_character))
        .route("/characters/:id", get(routes::characters::get_character))
        .route("/characters/:id", delete(routes::characters::delete_character))
        .route("/characters/:id/online", get(routes::characters::get_online_status))
        // Realms
        .route("/realms", get(routes::realms::list_realms))
        .route("/realms/:id", get(routes::realms::get_realm))
        .route("/realms/:id/online", get(routes::realms::get_online_count))
        // Highscores
        .route("/highscores/:realm", get(routes::highscores::get_highscores))
        .route("/highscores/:realm/:type", get(routes::highscores::get_highscores_by_type))
        // Guilds
        .route("/guilds", get(routes::guilds::list_guilds))
        .route("/guilds/:id", get(routes::guilds::get_guild))
        .route("/guilds/:id/members", get(routes::guilds::get_guild_members))
        .route("/guilds/:id/wars", get(routes::guilds::get_guild_wars))
        // Market
        .route("/market/offers", get(routes::market::list_offers))
        .route("/market/offers/:id", get(routes::market::get_offer))
        .route("/market/history", get(routes::market::get_history))
        // News
        .route("/news", get(routes::news::list_news))
        .route("/news/:id", get(routes::news::get_article))
        // Forum
        .route("/forum/categories", get(routes::forum::list_categories))
        .route("/forum/threads", get(routes::forum::list_threads))
        .route("/forum/threads/:id", get(routes::forum::get_thread))
        .route("/forum/threads", post(routes::forum::create_thread))
        .route("/forum/threads/:id/posts", post(routes::forum::create_post))
        // Houses
        .route("/houses/:realm", get(routes::houses::list_houses))
        .route("/houses/:realm/:id", get(routes::houses::get_house))
        // Admin routes (protected)
        .route("/admin/stats", get(routes::admin::get_stats))
        .route("/admin/players/online", get(routes::admin::get_online_players))
        .route("/admin/ban", post(routes::admin::ban_account))
        .route("/admin/broadcast", post(routes::admin::broadcast_message));

    // Main router with middleware
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/api/v1", api_routes)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(state)
}

/// Start the API server
pub async fn start_server(state: Arc<AppState>, addr: &str) -> std::io::Result<()> {
    let router = create_router(state);
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("API server listening on {}", addr);

    axum::serve(listener, router).await
}
