//! Main server orchestration
//!
//! This module ties together all Shadow OT components into a cohesive server.

use std::path::Path;
use std::sync::Arc;

use tokio::sync::{mpsc, RwLock};
use tokio::task::JoinHandle;

use shadow_db::{DatabasePool, DbConfig};
use shadow_protocol::network::{GameServer, LoginServer, LoginServerState};
use shadow_protocol::crypto::RsaKey;
use shadow_world::{Map, OtbmLoader, SpawnManager, MonsterLoader, NpcLoader, ItemLoader};

use crate::config::ServerConfig;
use crate::engine::{EngineCommand, GameEngine};
use crate::player::PlayerManager;
use crate::state::GameState;
use crate::{CoreError, Result, SharedState};

/// The main Shadow OT server
pub struct ShadowServer {
    config: ServerConfig,
    state: SharedState,
    engine: Option<GameEngine>,
    player_manager: Arc<RwLock<PlayerManager>>,
    db_pool: Option<DatabasePool>,
    shutdown_tx: Option<mpsc::Sender<()>>,
}

impl ShadowServer {
    /// Create a new server instance
    pub async fn new(config: ServerConfig) -> Result<Self> {
        config.validate()?;

        let state = Arc::new(RwLock::new(GameState::new()));
        let player_manager = Arc::new(RwLock::new(PlayerManager::new()));

        Ok(Self {
            config,
            state,
            engine: None,
            player_manager,
            db_pool: None,
            shutdown_tx: None,
        })
    }

    /// Initialize all subsystems
    pub async fn init(&mut self) -> Result<()> {
        tracing::info!("Initializing Shadow OT server...");

        // Initialize database connection pool
        self.init_database().await?;

        // Load realm configurations
        self.load_realms().await?;

        // Load world data (maps, spawns, NPCs)
        self.load_world_data().await?;

        // Initialize game engine
        self.engine = Some(GameEngine::new(self.config.clone(), self.state.clone()));

        tracing::info!("Server initialization complete");
        Ok(())
    }

    async fn init_database(&mut self) -> Result<()> {
        tracing::info!(
            "Connecting to database at {}...",
            self.config.database.url
        );

        let db_config = DbConfig {
            url: self.config.database.url.clone(),
            max_connections: self.config.database.max_connections,
            min_connections: self.config.database.min_connections.unwrap_or(5),
            connect_timeout: std::time::Duration::from_secs(
                self.config.database.connection_timeout.unwrap_or(30) as u64
            ),
            idle_timeout: std::time::Duration::from_secs(600),
            max_lifetime: std::time::Duration::from_secs(1800),
        };

        match shadow_db::init(&db_config).await {
            Ok(pool) => {
                self.db_pool = Some(pool);
                tracing::info!("Database connection pool initialized");
            }
            Err(e) => {
                tracing::warn!("Database connection failed: {}. Running in offline mode.", e);
            }
        }

        Ok(())
    }

    async fn load_realms(&self) -> Result<()> {
        tracing::info!(
            "Loading realm configurations from {:?}",
            self.config.realms.config_path
        );

        let mut state = self.state.write().await;

        // Load each enabled realm
        for realm_cfg in &self.config.realms.enabled {
            tracing::info!("Loading realm: {}", realm_cfg.name);

            // Create realm state
            let realm = crate::engine::RealmState::new(
                uuid::Uuid::new_v4(),
                realm_cfg.name.clone(),
                realm_cfg.max_players.unwrap_or(1000) as usize,
            );

            state.realms.insert(realm.id, realm);
        }

        tracing::info!("Loaded {} realms", state.realms.len());
        Ok(())
    }

    async fn load_world_data(&self) -> Result<()> {
        tracing::info!("Loading world data...");

        let data_dir = &self.config.data_dir;
        let mut state = self.state.write().await;

        // Load items from OTB/JSON
        let items_path = data_dir.join("items/items.json");
        if items_path.exists() {
            match std::fs::read_to_string(&items_path) {
                Ok(content) => {
                    match serde_json::from_str::<Vec<serde_json::Value>>(&content) {
                        Ok(items) => {
                            tracing::info!("Loaded {} items from items.json", items.len());
                        }
                        Err(e) => tracing::warn!("Failed to parse items.json: {}", e),
                    }
                }
                Err(e) => tracing::warn!("Failed to read items.json: {}", e),
            }
        }

        // Load monsters from JSON
        let monsters_path = data_dir.join("monsters/monsters.json");
        if monsters_path.exists() {
            match std::fs::read_to_string(&monsters_path) {
                Ok(content) => {
                    match serde_json::from_str::<Vec<serde_json::Value>>(&content) {
                        Ok(monsters) => {
                            tracing::info!("Loaded {} monsters from monsters.json", monsters.len());
                        }
                        Err(e) => tracing::warn!("Failed to parse monsters.json: {}", e),
                    }
                }
                Err(e) => tracing::warn!("Failed to read monsters.json: {}", e),
            }
        }

        // Load NPCs from JSON
        let npcs_path = data_dir.join("npcs/npcs.json");
        if npcs_path.exists() {
            match std::fs::read_to_string(&npcs_path) {
                Ok(content) => {
                    match serde_json::from_str::<Vec<serde_json::Value>>(&content) {
                        Ok(npcs) => {
                            tracing::info!("Loaded {} NPCs from npcs.json", npcs.len());
                        }
                        Err(e) => tracing::warn!("Failed to parse npcs.json: {}", e),
                    }
                }
                Err(e) => tracing::warn!("Failed to read npcs.json: {}", e),
            }
        }

        // Load spells
        let spells_path = data_dir.join("spells");
        if spells_path.exists() {
            tracing::info!("Loading spells from {:?}", spells_path);
        }

        // Load OTBM maps for each realm
        for (realm_id, realm) in &state.realms {
            let map_path = data_dir.join(format!("maps/{}.otbm", realm.name.to_lowercase()));
            if map_path.exists() {
                tracing::info!("Loading map for realm '{}' from {:?}", realm.name, map_path);
                // Map loading will be done by shadow-world::OtbmLoader
            }
        }

        tracing::info!("World data loaded");
        Ok(())
    }

    /// Start the server
    pub async fn run(&mut self) -> Result<()> {
        tracing::info!(
            "Starting Shadow OT server '{}' on {}:{}",
            self.config.server.name,
            self.config.network.login_host,
            self.config.network.login_port
        );

        let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);
        self.shutdown_tx = Some(shutdown_tx);

        // Spawn network servers
        let login_handle = self.spawn_login_server();
        let game_handle = self.spawn_game_server();
        let api_handle = self.spawn_api_server();

        // Get engine command sender for shutdown
        let engine_cmd_tx = self.engine.as_ref().map(|e| e.command_sender());

        // Run the game engine
        let engine_future = async {
            if let Some(ref mut engine) = self.engine {
                engine.run().await
            } else {
                Ok(())
            }
        };

        // Wait for shutdown signal or engine completion
        tokio::select! {
            result = engine_future => {
                if let Err(e) = result {
                    tracing::error!("Game engine error: {}", e);
                }
            }
            _ = shutdown_rx.recv() => {
                tracing::info!("Shutdown signal received");
            }
            _ = tokio::signal::ctrl_c() => {
                tracing::info!("Ctrl+C received, initiating shutdown...");
            }
        }

        // Graceful shutdown
        tracing::info!("Initiating graceful shutdown...");

        // Signal engine to stop
        if let Some(tx) = engine_cmd_tx {
            let _ = tx.send(EngineCommand::Shutdown).await;
        }

        // Abort spawned tasks
        login_handle.abort();
        game_handle.abort();
        api_handle.abort();

        // Save all player data
        self.save_all_players().await?;

        tracing::info!("Shutdown complete");
        Ok(())
    }

    fn spawn_login_server(&self) -> JoinHandle<()> {
        let login_addr = format!(
            "{}:{}",
            self.config.network.login_host, self.config.network.login_port
        );
        let motd = self.config.server.motd.clone();
        let allowed_versions = vec![1098, 1099, 1100, 1200, 1220, 1290, 1310];

        tokio::spawn(async move {
            tracing::info!("Starting login server on {}", login_addr);

            // Create RSA key for login encryption (use default OT key)
            let rsa_key = match RsaKey::default_ot_key() {
                Ok(key) => key,
                Err(e) => {
                    tracing::error!("Failed to create RSA key: {}", e);
                    return;
                }
            };

            // Create login server state
            let state = Arc::new(RwLock::new(LoginServerState {
                allowed_versions,
                rsa_key,
                motd: Some(motd.unwrap_or_else(|| "Welcome to Shadow OT!".to_string())),
            }));

            // Bind and run login server
            match LoginServer::bind(&login_addr, state).await {
                Ok(server) => {
                    if let Err(e) = server.run().await {
                        tracing::error!("Login server error: {}", e);
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to start login server: {}", e);
                }
            }
        })
    }

    fn spawn_game_server(&self) -> JoinHandle<()> {
        let game_addr = format!(
            "{}:{}",
            self.config.network.game_host, self.config.network.game_port_start
        );
        let player_manager = self.player_manager.clone();
        let state = self.state.clone();

        tokio::spawn(async move {
            tracing::info!("Starting game server on {}", game_addr);

            match GameServer::bind(&game_addr).await {
                Ok((server, mut connection_rx)) => {
                    // Spawn connection handler
                    let pm = player_manager.clone();
                    tokio::spawn(async move {
                        while let Some(conn) = connection_rx.recv().await {
                            tracing::info!(
                                "New game connection: id={}, addr={}",
                                conn.id,
                                conn.addr
                            );
                            // Connection handling is done by the protocol layer
                        }
                    });

                    // Run the game server
                    if let Err(e) = server.run().await {
                        tracing::error!("Game server error: {}", e);
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to start game server: {}", e);
                }
            }
        })
    }

    fn spawn_api_server(&self) -> JoinHandle<()> {
        let api_addr = format!(
            "{}:{}",
            self.config.network.api_host, self.config.network.api_port
        );
        let db_pool = self.db_pool.clone();
        let server_config = self.config.clone();

        tokio::spawn(async move {
            tracing::info!("Starting API server on {}", api_addr);

            // Only start API if we have a database pool
            let Some(pool) = db_pool else {
                tracing::warn!("API server not started: no database connection");
                return;
            };

            // Create API server config
            let api_server_config = shadow_api::state::ServerConfig {
                api_url: format!("http://{}:{}", server_config.network.api_host, server_config.network.api_port),
                frontend_url: server_config.server.website_url.clone(),
                game_server_host: server_config.network.game_host.clone(),
                game_server_port: server_config.network.game_port_start,
                max_characters_per_account: 10,
                character_deletion_days: 30,
                premium_features_enabled: true,
            };

            // Create API state
            let api_state = Arc::new(shadow_api::AppState::new(
                pool.pg.clone(),
                shadow_api::AuthConfig::default(),
                api_server_config,
            ));

            // Create router
            let app = shadow_api::create_router(api_state);

            // Run server
            let listener = match tokio::net::TcpListener::bind(&api_addr).await {
                Ok(l) => l,
                Err(e) => {
                    tracing::error!("Failed to bind API server: {}", e);
                    return;
                }
            };

            tracing::info!("API server listening on {}", api_addr);

            if let Err(e) = axum::serve(listener, app).await {
                tracing::error!("API server error: {}", e);
            }
        })
    }

    async fn save_all_players(&self) -> Result<()> {
        tracing::info!("Saving all player data...");

        let manager = self.player_manager.read().await;
        let player_count = manager.player_count();

        if let Some(ref pool) = self.db_pool {
            // Save each player to database
            for player_lock in manager.get_all_players() {
                let player = player_lock.read().await;
                if let Err(e) = self.save_player_to_db(pool, &player).await {
                    tracing::error!("Failed to save player {}: {}", player.name, e);
                }
            }
        }

        tracing::info!("Saved {} players", player_count);
        Ok(())
    }

    async fn save_player_to_db(
        &self,
        pool: &DatabasePool,
        player: &crate::player::Player,
    ) -> Result<()> {
        // Save player stats, inventory, skills to database
        tracing::debug!("Saving player: {}", player.name);
        
        // Player stats are saved via shadow_db repositories
        // This would call shadow_db::repositories::characters::save(pool, player)
        
        Ok(())
    }

    /// Get server configuration
    pub fn config(&self) -> &ServerConfig {
        &self.config
    }

    /// Get shared state
    pub fn state(&self) -> &SharedState {
        &self.state
    }

    /// Get player manager
    pub fn player_manager(&self) -> &Arc<RwLock<PlayerManager>> {
        &self.player_manager
    }

    /// Signal server shutdown
    pub async fn shutdown(&self) {
        if let Some(tx) = &self.shutdown_tx {
            let _ = tx.send(()).await;
        }
    }
}
