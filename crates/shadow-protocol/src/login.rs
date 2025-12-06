//! Login server implementation

use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;
use tokio_util::codec::Framed;

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

    // Authenticate user (placeholder - real implementation would check database)
    let auth_result = authenticate_user(&login.account_name, &login.password).await;

    match auth_result {
        Ok(characters) => {
            let state = state.read().await;
            let response = CharacterListPacket {
                motd: state.motd.clone(),
                session_key: generate_session_key(),
                characters,
                premium_days: 0,
                premium_until: 0,
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

/// Authenticate user credentials (placeholder)
async fn authenticate_user(
    account_name: &str,
    password: &str,
) -> std::result::Result<Vec<CharacterEntry>, String> {
    // This would be replaced with actual database lookup
    if account_name.is_empty() || password.is_empty() {
        return Err("Account name and password are required.".to_string());
    }

    // Placeholder character list
    Ok(vec![
        CharacterEntry {
            name: "TestCharacter".to_string(),
            realm: "Shadowveil".to_string(),
            realm_host: "127.0.0.1".to_string(),
            realm_port: 7172,
            preview_state: 0,
        },
    ])
}

/// Generate a unique session key
fn generate_session_key() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let key: [u8; 32] = rng.gen();
    hex::encode(key)
}
