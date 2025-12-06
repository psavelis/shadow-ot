//! Server configuration management
//!
//! Handles loading, validation, and hot-reloading of server configuration.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub server: ServerSettings,
    pub network: NetworkSettings,
    pub database: DatabaseSettings,
    pub redis: RedisSettings,
    pub realms: RealmsSettings,
    pub security: SecuritySettings,
    pub monitoring: MonitoringSettings,
    pub features: FeatureFlags,
    #[serde(default = "default_data_dir")]
    pub data_dir: PathBuf,
}

fn default_data_dir() -> PathBuf {
    PathBuf::from("data")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSettings {
    pub name: String,
    pub motd: Option<String>,
    pub save_interval_minutes: u32,
    pub max_players_global: usize,
    pub owner_name: String,
    pub owner_email: String,
    pub website_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSettings {
    pub login_host: String,
    pub login_port: u16,
    pub game_host: String,
    pub game_port_start: u16,
    pub game_port_end: u16,
    pub api_host: String,
    pub api_port: u16,
    pub websocket_port: u16,
    pub max_connections_per_ip: u32,
    pub connection_timeout_seconds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSettings {
    pub url: String,
    pub max_connections: u32,
    #[serde(default)]
    pub min_connections: Option<u32>,
    #[serde(default)]
    pub connection_timeout: Option<u32>,
    #[serde(default)]
    pub idle_timeout_seconds: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisSettings {
    pub url: String,
    pub pool_size: u32,
    pub session_prefix: String,
    pub cache_prefix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealmsSettings {
    pub config_path: PathBuf,
    pub default_realm: String,
    pub hot_reload: bool,
    pub max_realms: usize,
    #[serde(default)]
    pub enabled: Vec<RealmConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealmConfig {
    pub name: String,
    pub max_players: Option<usize>,
    pub experience_rate: Option<f32>,
    pub loot_rate: Option<f32>,
    pub skill_rate: Option<f32>,
    pub pvp_enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySettings {
    pub jwt_secret: String,
    pub jwt_expiry_hours: u32,
    pub password_min_length: usize,
    pub max_login_attempts: u32,
    pub lockout_duration_minutes: u32,
    pub enable_2fa: bool,
    pub allowed_client_versions: Vec<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringSettings {
    pub prometheus_enabled: bool,
    pub prometheus_port: u16,
    pub tracing_enabled: bool,
    pub tracing_endpoint: Option<String>,
    pub log_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlags {
    pub multi_realm: bool,
    pub cross_realm_trading: bool,
    pub seasonal_events: bool,
    pub matchmaking: bool,
    pub anti_cheat: bool,
    pub custom_sprites: bool,
    pub lua_scripting: bool,
    pub user_submitted_content: bool,
    pub houses: bool,
    pub guilds: bool,
    pub party_system: bool,
    pub market: bool,
    pub achievements: bool,
    pub bestiary: bool,
    pub prey_system: bool,
    pub forge_system: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            server: ServerSettings {
                name: "Shadow OT".to_string(),
                motd: Some("Welcome to Shadow OT - The Ultimate Open Tibia Experience!".to_string()),
                save_interval_minutes: 5,
                max_players_global: 10000,
                owner_name: "Shadow Team".to_string(),
                owner_email: "admin@shadow-ot.com".to_string(),
                website_url: "https://shadow-ot.com".to_string(),
            },
            network: NetworkSettings {
                login_host: "0.0.0.0".to_string(),
                login_port: 7171,
                game_host: "0.0.0.0".to_string(),
                game_port_start: 7172,
                game_port_end: 7180,
                api_host: "0.0.0.0".to_string(),
                api_port: 8080,
                websocket_port: 8081,
                max_connections_per_ip: 10,
                connection_timeout_seconds: 60,
            },
            database: DatabaseSettings {
                url: "postgres://shadow:shadow@localhost:5432/shadow_ot".to_string(),
                max_connections: 100,
                min_connections: Some(10),
                connection_timeout: Some(30),
                idle_timeout_seconds: Some(600),
            },
            redis: RedisSettings {
                url: "redis://localhost:6379".to_string(),
                pool_size: 50,
                session_prefix: "shadow:session:".to_string(),
                cache_prefix: "shadow:cache:".to_string(),
            },
            realms: RealmsSettings {
                config_path: PathBuf::from("realms"),
                default_realm: "shadowveil".to_string(),
                hot_reload: true,
                max_realms: 20,
                enabled: vec![
                    RealmConfig {
                        name: "shadowveil".to_string(),
                        max_players: Some(1000),
                        experience_rate: Some(1.0),
                        loot_rate: Some(1.0),
                        skill_rate: Some(1.0),
                        pvp_enabled: Some(true),
                    },
                ],
            },
            security: SecuritySettings {
                jwt_secret: "CHANGE_ME_IN_PRODUCTION".to_string(),
                jwt_expiry_hours: 24,
                password_min_length: 8,
                max_login_attempts: 5,
                lockout_duration_minutes: 15,
                enable_2fa: true,
                allowed_client_versions: vec![1098, 1099, 1100, 1200, 1281, 1310],
            },
            monitoring: MonitoringSettings {
                prometheus_enabled: true,
                prometheus_port: 9090,
                tracing_enabled: true,
                tracing_endpoint: None,
                log_level: "info".to_string(),
            },
            features: FeatureFlags {
                multi_realm: true,
                cross_realm_trading: true,
                seasonal_events: true,
                matchmaking: true,
                anti_cheat: true,
                custom_sprites: true,
                lua_scripting: true,
                user_submitted_content: true,
                houses: true,
                guilds: true,
                party_system: true,
                market: true,
                achievements: true,
                bestiary: true,
                prey_system: true,
                forge_system: true,
            },
            data_dir: PathBuf::from("data"),
        }
    }
}

impl ServerConfig {
    /// Load configuration from file
    pub fn from_file(path: &std::path::Path) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| crate::CoreError::Config(format!("Failed to read config: {}", e)))?;
        toml::from_str(&content)
            .map_err(|e| crate::CoreError::Config(format!("Failed to parse config: {}", e)))
    }

    /// Load configuration from environment variables
    pub fn from_env() -> crate::Result<Self> {
        let mut config = Self::default();

        if let Ok(url) = std::env::var("DATABASE_URL") {
            config.database.url = url;
        }
        if let Ok(url) = std::env::var("REDIS_URL") {
            config.redis.url = url;
        }
        if let Ok(secret) = std::env::var("JWT_SECRET") {
            config.security.jwt_secret = secret;
        }
        if let Ok(level) = std::env::var("LOG_LEVEL") {
            config.monitoring.log_level = level;
        }

        Ok(config)
    }

    /// Validate configuration
    pub fn validate(&self) -> crate::Result<()> {
        if self.security.jwt_secret == "CHANGE_ME_IN_PRODUCTION" {
            tracing::warn!("Using default JWT secret - change this in production!");
        }
        if self.network.game_port_end < self.network.game_port_start {
            return Err(crate::CoreError::Config(
                "game_port_end must be >= game_port_start".to_string(),
            ));
        }
        Ok(())
    }
}
