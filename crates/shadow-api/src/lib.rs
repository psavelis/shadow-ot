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
        routes::auth::enable_2fa,
        routes::auth::verify_2fa,
        routes::auth::disable_2fa,
        routes::auth::get_wallet_nonce,
        routes::auth::login_with_wallet,
        routes::auth::connect_wallet,
        routes::auth::disconnect_wallet,
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
        routes::support::list_tickets,
        routes::support::get_ticket,
        routes::support::create_ticket,
        routes::support::reply_to_ticket,
        routes::support::close_ticket,
        routes::support::get_faq,
        routes::auction::list_character_auctions,
        routes::auction::list_item_auctions,
        routes::auction::get_character_auction,
        routes::auction::get_item_auction,
        routes::auction::bid_on_character_auction,
        routes::auction::bid_on_item_auction,
        routes::auction::create_character_auction,
        routes::auction::create_item_auction,
        routes::auction::cancel_auction,
        routes::kill_statistics::get_statistics,
        routes::kill_statistics::get_top_killers,
        routes::kill_statistics::get_recent_deaths,
        routes::kill_statistics::get_boss_hunters,
        routes::kill_statistics::get_character_kills,
        routes::boosted::get_boosted_creature,
        routes::boosted::get_boosted_boss,
        routes::boosted::get_creature_history,
        routes::boosted::get_boss_history,
        routes::creatures::list_creatures,
        routes::creatures::get_creature,
        routes::creatures::get_creature_by_name,
        routes::creatures::get_bestiary_progress,
        routes::creatures::get_bestiary_entry,
        routes::achievements::list_achievements,
        routes::achievements::get_player_achievements,
        routes::achievements::get_leaderboard,
        routes::world_quests::list_world_quests,
        routes::world_quests::get_active_quests,
        routes::world_quests::get_world_quest,
        routes::world_quests::contribute_to_quest,
        routes::inventory::get_inventory_items,
        routes::inventory::get_inventory_item,
        routes::inventory::transfer_item,
        routes::inventory::list_on_market,
        routes::spells::list_spells,
        routes::spells::get_spell,
        routes::spells::get_spell_by_words,
        routes::spells::get_spells_by_vocation,
        routes::spells::get_spells_by_element,
        routes::spells::get_runes,
        routes::events::list_events,
        routes::events::get_active_events,
        routes::events::get_upcoming_events,
        routes::events::get_event,
        // NFT
        routes::nft::get_owned_nfts,
        routes::nft::get_nft,
        routes::nft::mint_nft,
        routes::nft::transfer_nft,
        routes::nft::get_marketplace,
        routes::nft::list_nft,
        routes::nft::buy_nft,
        routes::nft::cancel_listing,
        // Premium
        routes::premium::get_premium_status,
        routes::premium::purchase_premium,
        routes::premium::get_coin_packages,
        routes::premium::purchase_coins,
        routes::premium::get_premium_history,
        routes::premium::toggle_auto_renew,
        routes::premium::cancel_premium,
        // Notifications
        routes::notifications::get_notifications,
        routes::notifications::mark_notification_read,
        routes::notifications::mark_all_read,
        routes::notifications::delete_notification,
        routes::notifications::get_unread_count,
    ),
    components(
        schemas(
            routes::auth::LoginRequest,
            routes::auth::LoginResponse,
            routes::auth::RegisterRequest,
            routes::auth::Enable2FAResponse,
            routes::auth::Verify2FARequest,
            routes::auth::WalletNonceResponse,
            routes::auth::WalletLoginRequest,
            routes::auth::ConnectWalletRequest,
            routes::accounts::AccountResponse,
            routes::characters::CharacterResponse,
            routes::characters::CreateCharacterRequest,
            routes::realms::RealmResponse,
            routes::highscores::HighscoreEntry,
            routes::guilds::GuildResponse,
            routes::market::MarketOffer,
            routes::news::NewsArticle,
            routes::support::SupportTicket,
            routes::support::TicketMessage,
            routes::support::TicketCategory,
            routes::support::TicketStatus,
            routes::support::TicketPriority,
            routes::support::CreateTicketRequest,
            routes::support::ReplyTicketRequest,
            routes::support::PaginatedTickets,
            routes::support::FaqCategory,
            routes::support::FaqItem,
            routes::auction::CharacterAuction,
            routes::auction::ItemAuction,
            routes::auction::AuctionType,
            routes::auction::AuctionStatus,
            routes::auction::Vocation,
            routes::auction::CharacterSkills,
            routes::auction::BidRequest,
            routes::auction::BidResponse,
            routes::auction::CreateCharacterAuctionRequest,
            routes::auction::CreateItemAuctionRequest,
            routes::auction::PaginatedCharacterAuctions,
            routes::auction::PaginatedItemAuctions,
            routes::kill_statistics::KillStatistics,
            routes::kill_statistics::TopKiller,
            routes::kill_statistics::KillEntry,
            routes::kill_statistics::BossHunter,
            routes::kill_statistics::KillType,
            routes::kill_statistics::PaginatedKillEntries,
            routes::boosted::BoostedCreature,
            routes::boosted::BoostedBoss,
            routes::creatures::Creature,
            routes::creatures::CreatureDifficulty,
            routes::creatures::LootItem,
            routes::creatures::BestiaryEntry,
            routes::creatures::PaginatedCreatures,
            routes::achievements::Achievement,
            routes::achievements::AchievementCategory,
            routes::achievements::AchievementRarity,
            routes::achievements::PlayerAchievement,
            routes::achievements::AchievementProgress,
            routes::achievements::AchievementSummary,
            routes::achievements::AchievementLeaderboardEntry,
            routes::achievements::PaginatedAchievements,
            routes::achievements::PaginatedLeaderboard,
            routes::world_quests::WorldQuest,
            routes::world_quests::WorldQuestStatus,
            routes::world_quests::WorldQuestReward,
            routes::world_quests::TopContributor,
            routes::world_quests::ContributeRequest,
            routes::world_quests::ContributeResponse,
            routes::inventory::InventoryItem,
            routes::inventory::ItemAttributes,
            routes::inventory::Imbuement,
            routes::inventory::TransferRequest,
            routes::inventory::TransferResponse,
            routes::inventory::ListOnMarketRequest,
            routes::inventory::ListOnMarketResponse,
            routes::spells::Spell,
            routes::spells::Rune,
            routes::spells::SpellElement,
            routes::spells::SpellType,
            routes::events::GameEvent,
            routes::events::EventStatus,
            routes::events::EventType,
            routes::events::EventReward,
            routes::events::EventLocation,
            routes::events::EventRequirements,
            // NFT schemas
            routes::nft::Nft,
            routes::nft::NftMetadata,
            routes::nft::NftAttribute,
            routes::nft::NftListing,
            routes::nft::BlockchainChain,
            routes::nft::NftStatus,
            routes::nft::MintNftRequest,
            routes::nft::MintNftResponse,
            routes::nft::TransferNftRequest,
            routes::nft::TransferNftResponse,
            routes::nft::ListNftRequest,
            routes::nft::BuyNftRequest,
            routes::nft::PaginatedNfts,
            // Premium schemas
            routes::premium::PremiumStatus,
            routes::premium::PremiumPlan,
            routes::premium::PremiumTransaction,
            routes::premium::PurchasePremiumRequest,
            routes::premium::PurchasePremiumResponse,
            routes::premium::CoinPackage,
            routes::premium::PurchaseCoinsRequest,
            routes::premium::PurchaseCoinsResponse,
            routes::premium::PaginatedTransactions,
            // Notification schemas
            routes::notifications::Notification,
            routes::notifications::NotificationType,
            routes::notifications::PaginatedNotifications,
            routes::notifications::MarkReadResponse,
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
        (name = "support", description = "Support ticket system"),
        (name = "auctions", description = "Character and item auctions"),
        (name = "kill-statistics", description = "Kill statistics and death records"),
        (name = "boosted", description = "Boosted creatures and bosses"),
        (name = "creatures", description = "Creature database and bestiary"),
        (name = "achievements", description = "Achievement system"),
        (name = "world-quests", description = "World quest events"),
        (name = "inventory", description = "Character inventory management"),
        (name = "spells", description = "Spell and rune database"),
        (name = "events", description = "Game events and raids"),
        (name = "nft", description = "NFT and blockchain integration"),
        (name = "premium", description = "Premium subscriptions and coin shop"),
        (name = "notifications", description = "User notifications"),
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
        // 2FA
        .route("/auth/2fa/enable", post(routes::auth::enable_2fa))
        .route("/auth/2fa/verify", post(routes::auth::verify_2fa))
        .route("/auth/2fa/disable", post(routes::auth::disable_2fa))
        // Wallet auth
        .route("/auth/wallet/nonce/:address", get(routes::auth::get_wallet_nonce))
        .route("/auth/wallet/login", post(routes::auth::login_with_wallet))
        .route("/auth/wallet/connect", post(routes::auth::connect_wallet))
        .route("/auth/wallet/disconnect", post(routes::auth::disconnect_wallet))
        .route("/auth/resend-verification", post(routes::auth::resend_verification))
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
        // Support tickets
        .route("/support/tickets", get(routes::support::list_tickets))
        .route("/support/tickets", post(routes::support::create_ticket))
        .route("/support/tickets/:id", get(routes::support::get_ticket))
        .route("/support/tickets/:id/reply", post(routes::support::reply_to_ticket))
        .route("/support/tickets/:id/close", axum::routing::patch(routes::support::close_ticket))
        .route("/support/faq", get(routes::support::get_faq))
        // Auctions
        .route("/auctions/characters", get(routes::auction::list_character_auctions))
        .route("/auctions/characters", post(routes::auction::create_character_auction))
        .route("/auctions/characters/:id", get(routes::auction::get_character_auction))
        .route("/auctions/characters/:id/bid", post(routes::auction::bid_on_character_auction))
        .route("/auctions/items", get(routes::auction::list_item_auctions))
        .route("/auctions/items", post(routes::auction::create_item_auction))
        .route("/auctions/items/:id", get(routes::auction::get_item_auction))
        .route("/auctions/items/:id/bid", post(routes::auction::bid_on_item_auction))
        .route("/auctions/:auction_type/:id", delete(routes::auction::cancel_auction))
        // Kill statistics
        .route("/kill-statistics", get(routes::kill_statistics::get_statistics))
        .route("/kill-statistics/top-killers", get(routes::kill_statistics::get_top_killers))
        .route("/kill-statistics/recent", get(routes::kill_statistics::get_recent_deaths))
        .route("/kill-statistics/boss-hunters", get(routes::kill_statistics::get_boss_hunters))
        .route("/kill-statistics/character/:character_id", get(routes::kill_statistics::get_character_kills))
        // Boosted creatures/bosses
        .route("/boosted/creature", get(routes::boosted::get_boosted_creature))
        .route("/boosted/boss", get(routes::boosted::get_boosted_boss))
        .route("/boosted/creature/history", get(routes::boosted::get_creature_history))
        .route("/boosted/boss/history", get(routes::boosted::get_boss_history))
        // Creatures/Bestiary
        .route("/creatures", get(routes::creatures::list_creatures))
        .route("/creatures/:id", get(routes::creatures::get_creature))
        .route("/creatures/name/:name", get(routes::creatures::get_creature_by_name))
        .route("/characters/:character_id/bestiary", get(routes::creatures::get_bestiary_progress))
        .route("/characters/:character_id/bestiary/:creature_id", get(routes::creatures::get_bestiary_entry))
        // Achievements
        .route("/achievements", get(routes::achievements::list_achievements))
        .route("/achievements/player", get(routes::achievements::get_player_achievements))
        .route("/achievements/leaderboard", get(routes::achievements::get_leaderboard))
        // World Quests
        .route("/world-quests", get(routes::world_quests::list_world_quests))
        .route("/world-quests/active", get(routes::world_quests::get_active_quests))
        .route("/world-quests/:id", get(routes::world_quests::get_world_quest))
        .route("/world-quests/:id/contribute", post(routes::world_quests::contribute_to_quest))
        // Inventory
        .route("/inventory", get(routes::inventory::get_inventory_items))
        .route("/inventory/:id", get(routes::inventory::get_inventory_item))
        .route("/inventory/:id/transfer", post(routes::inventory::transfer_item))
        .route("/inventory/:id/list-on-market", post(routes::inventory::list_on_market))
        // Spells
        .route("/spells", get(routes::spells::list_spells))
        .route("/spells/runes", get(routes::spells::get_runes))
        .route("/spells/vocation/:vocation", get(routes::spells::get_spells_by_vocation))
        .route("/spells/element/:element", get(routes::spells::get_spells_by_element))
        .route("/spells/words/:words", get(routes::spells::get_spell_by_words))
        .route("/spells/:id", get(routes::spells::get_spell))
        // Events
        .route("/events", get(routes::events::list_events))
        .route("/events/active", get(routes::events::get_active_events))
        .route("/events/upcoming", get(routes::events::get_upcoming_events))
        .route("/events/:id", get(routes::events::get_event))
        // NFT
        .route("/nft/owned", get(routes::nft::get_owned_nfts))
        .route("/nft/mint", post(routes::nft::mint_nft))
        .route("/nft/buy", post(routes::nft::buy_nft))
        .route("/nft/marketplace", get(routes::nft::get_marketplace))
        .route("/nft/:chain/:token_id", get(routes::nft::get_nft))
        .route("/nft/:id/transfer", post(routes::nft::transfer_nft))
        .route("/nft/:id/list", post(routes::nft::list_nft))
        .route("/nft/:id/cancel-listing", post(routes::nft::cancel_listing))
        // Premium
        .route("/users/me/premium", get(routes::premium::get_premium_status))
        .route("/users/me/premium", delete(routes::premium::cancel_premium))
        .route("/users/me/premium/purchase", post(routes::premium::purchase_premium))
        .route("/users/me/premium/packages", get(routes::premium::get_coin_packages))
        .route("/users/me/premium/coins", post(routes::premium::purchase_coins))
        .route("/users/me/premium/history", get(routes::premium::get_premium_history))
        .route("/users/me/premium/auto-renew", post(routes::premium::toggle_auto_renew))
        // Notifications
        .route("/users/me/notifications", get(routes::notifications::get_notifications))
        .route("/users/me/notifications/count", get(routes::notifications::get_unread_count))
        .route("/users/me/notifications/read-all", post(routes::notifications::mark_all_read))
        .route("/users/me/notifications/:id/read", axum::routing::patch(routes::notifications::mark_notification_read))
        .route("/users/me/notifications/:id", delete(routes::notifications::delete_notification))
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
