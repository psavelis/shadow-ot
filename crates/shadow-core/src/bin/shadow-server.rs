//! Shadow OT Server
//!
//! Main entry point for the Shadow OT game server.
//! This binary starts all server components: login server, game server, and API.

use std::env;
use std::path::PathBuf;

use shadow_core::{ServerConfig, ShadowServer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    init_logging();

    // Print banner
    print_banner();

    // Load configuration
    let config = load_config()?;

    // Initialize and start server
    tracing::info!("Initializing Shadow OT server...");
    let mut server = ShadowServer::new(config).await?;

    // Initialize all subsystems
    server.init().await?;

    // Start the server
    tracing::info!("Starting Shadow OT server...");
    server.run().await?;

    tracing::info!("Shadow OT server shutdown complete");
    Ok(())
}

fn init_logging() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new("info,shadow_core=debug,shadow_protocol=debug")
    });

    tracing_subscriber::registry()
        .with(filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true),
        )
        .init();
}

fn print_banner() {
    println!(
        r#"
 ███████╗██╗  ██╗ █████╗ ██████╗  ██████╗ ██╗    ██╗     ██████╗ ████████╗
 ██╔════╝██║  ██║██╔══██╗██╔══██╗██╔═══██╗██║    ██║    ██╔═══██╗╚══██╔══╝
 ███████╗███████║███████║██║  ██║██║   ██║██║ █╗ ██║    ██║   ██║   ██║   
 ╚════██║██╔══██║██╔══██║██║  ██║██║   ██║██║███╗██║    ██║   ██║   ██║   
 ███████║██║  ██║██║  ██║██████╔╝╚██████╔╝╚███╔███╔╝    ╚██████╔╝   ██║   
 ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═════╝  ╚═════╝  ╚══╝╚══╝      ╚═════╝    ╚═╝   
                                                                           
        The Ultimate Open Tibia Server - v{}
        
"#,
        env!("CARGO_PKG_VERSION")
    );
}

fn load_config() -> anyhow::Result<ServerConfig> {
    // Try to load from environment variable first
    let config_path = env::var("SHADOW_CONFIG")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            // Default paths to try
            let paths = [
                PathBuf::from("config.toml"),
                PathBuf::from("shadow.toml"),
                PathBuf::from("/etc/shadow-ot/config.toml"),
            ];

            for path in &paths {
                if path.exists() {
                    return path.clone();
                }
            }

            PathBuf::from("config.toml")
        });

    tracing::info!("Loading configuration from {:?}", config_path);

    if config_path.exists() {
        Ok(ServerConfig::from_file(&config_path)?)
    } else {
        tracing::warn!("Configuration file not found, using defaults");
        Ok(ServerConfig::default())
    }
}
