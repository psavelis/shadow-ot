//! Game engine - the heart of Shadow OT
//!
//! Manages the game loop, coordinates all subsystems, and handles game state updates.

use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::{broadcast, mpsc, RwLock};
use tokio::time::interval;

use crate::events::{GameEvent, RealmStatus};
use crate::state::GameState;
use crate::{RealmId, ServerConfig, SharedState, TICK_RATE_MS};

/// Command sent to the game engine
#[derive(Debug)]
pub enum EngineCommand {
    Shutdown,
    SaveAll,
    ReloadConfig,
    ReloadRealm(RealmId),
    BroadcastMessage(String),
    SetRealmStatus(RealmId, RealmStatus),
}

/// The main game engine
pub struct GameEngine {
    config: ServerConfig,
    state: SharedState,
    event_tx: broadcast::Sender<GameEvent>,
    command_rx: mpsc::Receiver<EngineCommand>,
    command_tx: mpsc::Sender<EngineCommand>,
    running: Arc<RwLock<bool>>,
    tick_count: u64,
    last_save: Instant,
}

impl GameEngine {
    /// Create a new game engine instance
    pub fn new(config: ServerConfig, state: SharedState) -> Self {
        let (event_tx, _) = broadcast::channel(10000);
        let (command_tx, command_rx) = mpsc::channel(100);

        Self {
            config,
            state,
            event_tx,
            command_rx,
            command_tx,
            running: Arc::new(RwLock::new(false)),
            tick_count: 0,
            last_save: Instant::now(),
        }
    }

    /// Get a command sender for external control
    pub fn command_sender(&self) -> mpsc::Sender<EngineCommand> {
        self.command_tx.clone()
    }

    /// Get an event subscriber
    pub fn event_subscriber(&self) -> broadcast::Receiver<GameEvent> {
        self.event_tx.subscribe()
    }

    /// Get event broadcaster
    pub fn event_broadcaster(&self) -> broadcast::Sender<GameEvent> {
        self.event_tx.clone()
    }

    /// Start the game engine main loop
    pub async fn run(&mut self) -> crate::Result<()> {
        tracing::info!("Starting Shadow OT game engine");

        {
            let mut running = self.running.write().await;
            *running = true;
        }

        let mut tick_interval = interval(Duration::from_millis(TICK_RATE_MS));
        let save_interval = Duration::from_secs(self.config.server.save_interval_minutes as u64 * 60);

        loop {
            tokio::select! {
                _ = tick_interval.tick() => {
                    self.tick().await?;
                }
                Some(cmd) = self.command_rx.recv() => {
                    match cmd {
                        EngineCommand::Shutdown => {
                            tracing::info!("Shutdown command received");
                            break;
                        }
                        EngineCommand::SaveAll => {
                            self.save_all().await?;
                        }
                        EngineCommand::ReloadConfig => {
                            self.reload_config().await?;
                        }
                        EngineCommand::ReloadRealm(realm_id) => {
                            self.reload_realm(realm_id).await?;
                        }
                        EngineCommand::BroadcastMessage(msg) => {
                            self.broadcast_message(&msg).await?;
                        }
                        EngineCommand::SetRealmStatus(realm_id, status) => {
                            self.set_realm_status(realm_id, status).await?;
                        }
                    }
                }
            }

            // Periodic save
            if self.last_save.elapsed() >= save_interval {
                self.save_all().await?;
                self.last_save = Instant::now();
            }
        }

        // Graceful shutdown
        self.shutdown().await?;

        {
            let mut running = self.running.write().await;
            *running = false;
        }

        Ok(())
    }

    /// Execute a single game tick
    async fn tick(&mut self) -> crate::Result<()> {
        self.tick_count += 1;

        let mut state = self.state.write().await;

        // Update all realms
        for realm in state.realms.values_mut() {
            if realm.status == RealmStatus::Online {
                // Process pending actions for all players in this realm
                self.process_realm_tick(realm).await?;
            }
        }

        // Process global systems
        if self.tick_count % 20 == 0 {
            // Every second (20 ticks)
            self.process_regeneration(&mut state).await?;
        }

        if self.tick_count % 100 == 0 {
            // Every 5 seconds
            self.process_creature_ai(&mut state).await?;
        }

        if self.tick_count % 1200 == 0 {
            // Every minute
            self.process_respawns(&mut state).await?;
            self.update_metrics(&state).await?;
        }

        Ok(())
    }

    async fn process_realm_tick(&self, _realm: &mut RealmState) -> crate::Result<()> {
        // Process player movements
        // Process combat
        // Process spell effects
        // Process item decay
        // Process environment effects
        Ok(())
    }

    async fn process_regeneration(&self, _state: &mut GameState) -> crate::Result<()> {
        // Health regeneration
        // Mana regeneration
        // Soul regeneration
        // Stamina updates
        Ok(())
    }

    async fn process_creature_ai(&self, _state: &mut GameState) -> crate::Result<()> {
        // Monster pathfinding
        // Monster targeting
        // Monster special abilities
        // NPC schedules
        Ok(())
    }

    async fn process_respawns(&self, _state: &mut GameState) -> crate::Result<()> {
        // Monster respawns
        // Item respawns
        // Event triggers
        Ok(())
    }

    async fn update_metrics(&self, state: &GameState) -> crate::Result<()> {
        let total_players: usize = state.realms.values().map(|r| r.player_count).sum();
        tracing::debug!(
            "Tick {} - {} players online across {} realms",
            self.tick_count,
            total_players,
            state.realms.len()
        );
        Ok(())
    }

    async fn save_all(&self) -> crate::Result<()> {
        tracing::info!("Saving all game data...");
        let state = self.state.read().await;

        // Save player data
        // Save house data
        // Save guild data
        // Save market data

        tracing::info!("Save complete - {} realms saved", state.realms.len());
        Ok(())
    }

    async fn reload_config(&mut self) -> crate::Result<()> {
        tracing::info!("Reloading server configuration...");
        // Hot-reload configuration
        Ok(())
    }

    async fn reload_realm(&self, realm_id: RealmId) -> crate::Result<()> {
        tracing::info!("Reloading realm {}", realm_id);
        // Hot-reload realm configuration
        Ok(())
    }

    async fn broadcast_message(&self, message: &str) -> crate::Result<()> {
        let event = GameEvent::GlobalBroadcast(crate::events::GlobalBroadcastEvent {
            message: message.to_string(),
            broadcast_type: crate::events::BroadcastType::ServerWide,
            sender: Some("Server".to_string()),
            timestamp: chrono::Utc::now(),
        });
        let _ = self.event_tx.send(event);
        Ok(())
    }

    async fn set_realm_status(&self, realm_id: RealmId, status: RealmStatus) -> crate::Result<()> {
        let mut state = self.state.write().await;
        if let Some(realm) = state.realms.get_mut(&realm_id) {
            let old_status = realm.status;
            realm.status = status;

            let event = GameEvent::RealmStatusChange(crate::events::RealmStatusEvent {
                realm_id,
                realm_name: realm.name.clone(),
                old_status,
                new_status: status,
                reason: "Admin action".to_string(),
                timestamp: chrono::Utc::now(),
            });
            let _ = self.event_tx.send(event);
        }
        Ok(())
    }

    async fn shutdown(&self) -> crate::Result<()> {
        tracing::info!("Shutting down game engine...");

        // Broadcast shutdown message
        self.broadcast_message("Server is shutting down...").await?;

        // Save all data
        self.save_all().await?;

        // Disconnect all players gracefully
        let state = self.state.read().await;
        for realm in state.realms.values() {
            tracing::info!("Disconnecting {} players from {}", realm.player_count, realm.name);
        }

        tracing::info!("Game engine shutdown complete");
        Ok(())
    }
}

/// State of a single realm
#[derive(Debug)]
pub struct RealmState {
    pub id: RealmId,
    pub name: String,
    pub status: RealmStatus,
    pub player_count: usize,
    pub max_players: usize,
    pub uptime_seconds: u64,
}

impl RealmState {
    pub fn new(id: RealmId, name: String, max_players: usize) -> Self {
        Self {
            id,
            name,
            status: RealmStatus::Offline,
            player_count: 0,
            max_players,
            uptime_seconds: 0,
        }
    }
}
