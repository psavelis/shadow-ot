//! Login server implementation

use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;
use tokio_util::codec::Framed;

use shadow_db::{DatabasePool, repositories::{AccountRepository, CharacterRepository, RealmRepository}};

use crate::codec::{NetworkMessage, TibiaCodec};
use crate::crypto::RsaKey;
use crate::packets::{CharacterEntry, CharacterListPacket, LoginErrorPacket, LoginPacket};
use crate::version::ProtocolVersion;
use crate::{Result, ProtocolError};

/// Login server state
pub struct LoginServerState {
    pub allowed_versions: Vec<u16>,
    pub rsa_key: RsaKey,
    pub motd: Option<String>,
    pub db_pool: Option<DatabasePool>,
}

/// Login server handling initial client connections
pub struct LoginServer {
    listener: TcpListener,
    state: Arc<RwLock<LoginServerState>>,
}

impl LoginServer {
    /// Create a new login server
    pub async fn bind(addr: &str, state: Arc<RwLock<LoginServerState>>) -> Result<Self> {
        let listener = TcpListener::bind(addr)
            .await
            .map_err(|e| ProtocolError::Connection(e.to_string()))?;

        tracing::info!("Login server listening on {}", addr);

        Ok(Self { listener, state })
    }

    /// Run the login server accept loop
    pub async fn run(&self) -> Result<()> {
        loop {
            match self.listener.accept().await {
                Ok((socket, addr)) => {
                    let state = self.state.clone();
                    tokio::spawn(async move {
                        if let Err(e) = handle_login_connection(socket, addr, state).await {
                            tracing::error!("Login connection error from {}: {}", addr, e);
                        }
                    });
                }
                Err(e) => {
                    tracing::error!("Failed to accept connection: {}", e);
                }
            }
        }
    }
}

/// Handle a single login connection
async fn handle_login_connection(
    socket: TcpStream,
    addr: SocketAddr,
    state: Arc<RwLock<LoginServerState>>,
) -> Result<()> {
    tracing::debug!("New login connection from {}", addr);

    // Set TCP_NODELAY for lower latency
    socket.set_nodelay(true).ok();

    // Create codec without encryption initially
    let version = ProtocolVersion::default();
    let mut codec = TibiaCodec::new(version);
    let mut framed = Framed::new(socket, codec);

    // Read the first packet (should contain RSA encrypted data)
    use futures::StreamExt;
    let Some(result) = framed.next().await else {
        return Err(ProtocolError::Connection("Connection closed".to_string()));
    };

    let msg = result?;

    // The first packet is RSA encrypted
    // In a real implementation, we'd decrypt with RSA first
    // For now, we'll parse directly

    // Parse login packet
    let login = parse_login_packet(msg, &state).await?;

    // Validate version
    {
        let state = state.read().await;
        if !state.allowed_versions.contains(&login.protocol_version) {
            let error = LoginErrorPacket {
                message: format!(
                    "Your client version {} is not supported. Please use a supported client.",
                    login.protocol_version
                ),
            };
            let mut response = NetworkMessage::new();
            error.write(&mut response);
            // Send error response
            return Ok(());
        }
    }

    // Set XTEA key for encrypted communication
    // codec.set_xtea_key(login.xtea_key);

    // Authenticate user with real database
    let auth_result = authenticate_user(&login.account_name, &login.password, &state).await;

    match auth_result {
        Ok((characters, premium_days, premium_until)) => {
            let state = state.read().await;
            let response = CharacterListPacket {
                motd: state.motd.clone(),
                session_key: generate_session_key(),
                characters,
                premium_days,
                premium_until,
            };

            let mut msg = NetworkMessage::new();
            response.write(&mut msg);

            // Send response (would use framed.send() with actual codec)
            tracing::info!("Login successful for account: {}", login.account_name);
        }
        Err(error_msg) => {
            let error = LoginErrorPacket {
                message: error_msg,
            };
            let mut msg = NetworkMessage::new();
            error.write(&mut msg);
            tracing::warn!("Login failed for account: {} - {}", login.account_name, error.message);
        }
    }

    Ok(())
}

/// Parse login packet from raw message
async fn parse_login_packet(
    mut msg: NetworkMessage,
    state: &Arc<RwLock<LoginServerState>>,
) -> Result<LoginPacket> {
    // Skip first byte (packet type) if present
    let packet_type = msg.get_u8()?;
    if packet_type != 0x01 {
        return Err(ProtocolError::InvalidPacket(format!(
            "Expected login packet (0x01), got 0x{:02X}",
            packet_type
        )));
    }

    // OS info (2 bytes)
    let _os = msg.get_u16()?;

    // Protocol version
    let protocol_version = msg.get_u16()?;

    // Client version
    let client_version = if protocol_version >= 1076 {
        msg.get_u32()? as u16
    } else {
        protocol_version
    };

    // Content revision (newer clients)
    let _content_revision = if protocol_version >= 1076 {
        msg.get_u16()?
    } else {
        0
    };

    // Preview state
    let preview_state = if protocol_version >= 1076 {
        msg.get_u8()?
    } else {
        0
    };

    // Signatures
    let dat_signature = msg.get_u32()?;
    let spr_signature = msg.get_u32()?;
    let pic_signature = msg.get_u32()?;

    // RSA encrypted block (128 bytes)
    let rsa_block = msg.get_bytes(128)?;

    // Decrypt RSA block
    let state = state.read().await;
    let decrypted = state.rsa_key.decrypt(&rsa_block)?;

    // Parse decrypted data
    let mut decrypted_msg = NetworkMessage::from_bytes(decrypted.into());

    // First byte should be 0
    let first_byte = decrypted_msg.get_u8()?;
    if first_byte != 0 {
        return Err(ProtocolError::InvalidPacket("Invalid RSA decryption".to_string()));
    }

    // XTEA key
    let xtea_key = [
        decrypted_msg.get_u32()?,
        decrypted_msg.get_u32()?,
        decrypted_msg.get_u32()?,
        decrypted_msg.get_u32()?,
    ];

    // Account name and password
    let account_name = decrypted_msg.get_string()?;
    let password = decrypted_msg.get_string()?;

    // Auth token (optional, for 2FA)
    let auth_token = if decrypted_msg.remaining() > 2 {
        Some(decrypted_msg.get_string()?)
    } else {
        None
    };

    // Stay logged in flag
    let stay_logged_in = if decrypted_msg.remaining() > 0 {
        decrypted_msg.get_u8()? != 0
    } else {
        false
    };

    Ok(LoginPacket {
        protocol_version,
        client_version,
        dat_signature,
        spr_signature,
        pic_signature,
        preview_state,
        xtea_key,
        account_name,
        password,
        auth_token,
        stay_logged_in,
    })
}

/// Authenticate user credentials against real database
async fn authenticate_user(
    account_name: &str,
    password: &str,
    state: &Arc<RwLock<LoginServerState>>,
) -> std::result::Result<(Vec<CharacterEntry>, u32, u64), String> {
    if account_name.is_empty() || password.is_empty() {
        return Err("Account name and password are required.".to_string());
    }

    // Get database pool from state
    let db_pool = {
        let state = state.read().await;
        state.db_pool.clone()
    };

    let Some(db_pool) = db_pool else {
        tracing::warn!("Database not available - login will fail");
        return Err("Server database is unavailable. Please try again later.".to_string());
    };

    // Hash the password (SHA-256 for compatibility with common OT implementations)
    let password_hash = sha256_hash(password);

    // Verify credentials against database
    let account_repo = AccountRepository::new(db_pool.postgres());
    let account = match account_repo.verify_credentials(account_name, &password_hash).await {
        Ok(Some(acc)) => acc,
        Ok(None) => {
            tracing::debug!("Invalid credentials for account: {}", account_name);
            return Err("Invalid account name or password.".to_string());
        }
        Err(e) => {
            tracing::error!("Database error during authentication: {}", e);
            return Err("Authentication failed. Please try again.".to_string());
        }
    };

    // Check if account is locked
    if let Some(locked_until) = &account.locked_until {
        if *locked_until > chrono::Utc::now() {
            return Err(format!(
                "Your account is temporarily locked. Try again after {}.",
                locked_until.format("%Y-%m-%d %H:%M:%S UTC")
            ));
        }
    }

    // Check if 2FA is required but token not provided
    if account.two_factor_enabled {
        // In a real implementation, we'd validate the 2FA token here
        // For now, log that 2FA is required
        tracing::info!("Account {} has 2FA enabled", account.username);
    }

    // Fetch characters for this account
    let char_repo = CharacterRepository::new(db_pool.postgres());
    let characters = match char_repo.find_by_account(account.id).await {
        Ok(chars) => chars,
        Err(e) => {
            tracing::error!("Failed to fetch characters for account {}: {}", account.id, e);
            return Err("Failed to retrieve character list.".to_string());
        }
    };

    // Fetch realm information for character entries
    let realm_repo = RealmRepository::new(db_pool.postgres());
    let realms = match realm_repo.find_all().await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("Failed to fetch realms: {}", e);
            return Err("Failed to retrieve realm information.".to_string());
        }
    };

    // Build character entries with realm info
    let character_entries: Vec<CharacterEntry> = characters
        .iter()
        .filter_map(|char| {
            let realm = realms.iter().find(|r| r.id == char.realm_id)?;
            Some(CharacterEntry {
                name: char.name.clone(),
                realm: realm.name.clone(),
                realm_host: realm.host.clone(),
                realm_port: realm.port as u16,
                preview_state: if realm.is_seasonal { 1 } else { 0 },
            })
        })
        .collect();

    // Calculate premium info
    let premium_days = account.premium_days_purchased.unwrap_or(0) as u32;
    let premium_until = account
        .premium_until
        .map(|dt| dt.timestamp() as u64)
        .unwrap_or(0);

    // Update last login info
    if let Err(e) = update_last_login(&account_repo, &account).await {
        tracing::warn!("Failed to update last login for account {}: {}", account.id, e);
    }

    tracing::info!(
        "Account {} logged in with {} characters",
        account.username,
        character_entries.len()
    );

    Ok((character_entries, premium_days, premium_until))
}

/// Update account's last login timestamp and IP
async fn update_last_login(
    repo: &AccountRepository<'_>,
    account: &shadow_db::models::account::Account,
) -> std::result::Result<(), String> {
    let mut updated = account.clone();
    updated.last_login = Some(chrono::Utc::now());
    updated.login_attempts = 0; // Reset failed attempts on successful login
    repo.update(&updated).await.map_err(|e| e.to_string())?;
    Ok(())
}

/// SHA-256 hash for password (compatible with most OT servers)
fn sha256_hash(input: &str) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

/// Generate a unique session key
fn generate_session_key() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let key: [u8; 32] = rng.gen();
    hex::encode(key)
}
