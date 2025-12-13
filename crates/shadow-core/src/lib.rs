//! Shadow OT Core Engine
//!
//! This is the heart of the Shadow OT server, coordinating all game systems,
//! managing the game loop, and orchestrating communication between subsystems.

pub mod achievement;
pub mod bank;
pub mod config;
pub mod cyclopedia;
pub mod death;
pub mod engine;
pub mod error;
pub mod events;
pub mod geolocation;
pub mod guild;
pub mod party;
pub mod player;
pub mod scheduler;
pub mod server;
pub mod session;
pub mod state;
pub mod trade;
pub mod vip;

use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

pub use achievement::{Achievement, AchievementManager, PlayerAchievements};
pub use bank::{BankAccount, BankManager};
pub use config::ServerConfig;
pub use cyclopedia::{Cyclopedia, CyclopediaManager, CyclopediaCategory};
pub use death::{BlessingType, DeathManager, DeathPenalty, PlayerBlessings, SkullType};
pub use engine::GameEngine;
pub use error::{CoreError, Result};
pub use geolocation::{GeoLocation, GeoService, GeoConfig, ServerRegion};
pub use guild::{Guild, GuildManager, GuildMember, GuildRank};
pub use party::{Party, PartyManager};
pub use server::ShadowServer;
pub use session::PlayerSession;
pub use state::GameState;
pub use trade::{TradeManager, TradeState};
pub use vip::{VipManager, VipStatus, VipTier};

/// Server-wide unique identifier
pub type ServerId = Uuid;
/// Player unique identifier
pub type PlayerId = Uuid;
/// Character unique identifier
pub type CharacterId = Uuid;
/// Realm unique identifier
pub type RealmId = Uuid;

/// Game tick rate (50ms = 20 ticks per second, matching Tibia's server tick)
pub const TICK_RATE_MS: u64 = 50;
/// Maximum players per realm instance
pub const MAX_PLAYERS_PER_REALM: usize = 1000;
/// Protocol version range supported
pub const SUPPORTED_PROTOCOL_MIN: u16 = 860;
pub const SUPPORTED_PROTOCOL_MAX: u16 = 1310;

/// Shared server state across all components
pub type SharedState = Arc<RwLock<GameState>>;
/// Event broadcast channel for server-wide events
pub type EventBroadcast = broadcast::Sender<events::GameEvent>;
/// World reference type for map and entity access
pub type WorldRef = Arc<RwLock<shadow_world::Map>>;

/// Server capabilities flags
#[derive(Debug, Clone, Copy)]
pub struct Capabilities {
    pub multi_realm: bool,
    pub cross_realm_trading: bool,
    pub seasonal_events: bool,
    pub matchmaking: bool,
    pub anti_cheat: bool,
    pub custom_sprites: bool,
    pub lua_scripting: bool,
    pub user_submitted_content: bool,
}

impl Default for Capabilities {
    fn default() -> Self {
        Self {
            multi_realm: true,
            cross_realm_trading: true,
            seasonal_events: true,
            matchmaking: true,
            anti_cheat: true,
            custom_sprites: true,
            lua_scripting: true,
            user_submitted_content: true,
        }
    }
}

/// Initialize the Shadow OT core with default configuration
pub async fn init() -> Result<ShadowServer> {
    init_with_config(ServerConfig::default()).await
}

/// Initialize the Shadow OT core with custom configuration
pub async fn init_with_config(config: ServerConfig) -> Result<ShadowServer> {
    tracing::info!("Initializing Shadow OT Core v{}", env!("CARGO_PKG_VERSION"));
    ShadowServer::new(config).await
}
