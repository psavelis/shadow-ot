//! Wallet Connection Management
//!
//! Handle connections from various wallet providers.

use serde::{Deserialize, Serialize};
use crate::{Chain, Result, BlockchainError};

/// Types of supported wallet providers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WalletType {
    /// MetaMask browser extension
    MetaMask,
    /// WalletConnect protocol
    WalletConnect,
    /// Coinbase Wallet
    CoinbaseWallet,
    /// Rainbow Wallet
    Rainbow,
    /// Rabby Wallet
    Rabby,
    /// Trust Wallet
    TrustWallet,
    /// Phantom (Solana, but may support EVM)
    Phantom,
    /// Argent (Starknet)
    Argent,
    /// Braavos (Starknet)
    Braavos,
    /// Uniswap Wallet
    Uniswap,
    /// Ledger hardware wallet
    Ledger,
    /// Trezor hardware wallet
    Trezor,
    /// Generic injected provider
    Injected,
    /// Bitcoin-specific wallets
    Xverse,
    Leather,
    OKXWallet,
    /// Unknown/Other
    Other,
}

impl WalletType {
    /// Check if this wallet supports a given chain
    pub fn supports_chain(&self, chain: Chain) -> bool {
        match self {
            // EVM-only wallets
            WalletType::MetaMask
            | WalletType::CoinbaseWallet
            | WalletType::Rainbow
            | WalletType::Rabby
            | WalletType::TrustWallet
            | WalletType::Uniswap => chain.is_evm(),

            // Starknet-only wallets
            WalletType::Argent | WalletType::Braavos => chain.is_starknet(),

            // Bitcoin-only wallets
            WalletType::Xverse | WalletType::Leather => chain.is_bitcoin(),

            // Multi-chain wallets
            WalletType::WalletConnect | WalletType::OKXWallet => true,

            // Hardware wallets support multiple chains
            WalletType::Ledger | WalletType::Trezor => true,

            // Phantom primarily Solana but may support EVM
            WalletType::Phantom => chain.is_evm(),

            // Generic/Unknown - assume true
            WalletType::Injected | WalletType::Other => true,
        }
    }

    /// Get the wallet's display name
    pub fn display_name(&self) -> &'static str {
        match self {
            WalletType::MetaMask => "MetaMask",
            WalletType::WalletConnect => "WalletConnect",
            WalletType::CoinbaseWallet => "Coinbase Wallet",
            WalletType::Rainbow => "Rainbow",
            WalletType::Rabby => "Rabby",
            WalletType::TrustWallet => "Trust Wallet",
            WalletType::Phantom => "Phantom",
            WalletType::Argent => "Argent",
            WalletType::Braavos => "Braavos",
            WalletType::Uniswap => "Uniswap Wallet",
            WalletType::Ledger => "Ledger",
            WalletType::Trezor => "Trezor",
            WalletType::Injected => "Browser Wallet",
            WalletType::Xverse => "Xverse",
            WalletType::Leather => "Leather",
            WalletType::OKXWallet => "OKX Wallet",
            WalletType::Other => "Wallet",
        }
    }

    /// Get the wallet's icon URL (for UI)
    pub fn icon_url(&self) -> &'static str {
        match self {
            WalletType::MetaMask => "https://metamask.io/icons/icon-256x256.png",
            WalletType::WalletConnect => "https://walletconnect.com/walletconnect-logo.png",
            WalletType::CoinbaseWallet => "https://www.coinbase.com/favicon.ico",
            WalletType::Rainbow => "https://rainbow.me/favicon.ico",
            WalletType::Rabby => "https://rabby.io/favicon.ico",
            WalletType::TrustWallet => "https://trustwallet.com/favicon.ico",
            WalletType::Phantom => "https://phantom.app/favicon.ico",
            WalletType::Argent => "https://argent.xyz/favicon.ico",
            WalletType::Braavos => "https://braavos.app/favicon.ico",
            WalletType::Uniswap => "https://app.uniswap.org/favicon.ico",
            WalletType::Ledger => "https://www.ledger.com/favicon.ico",
            WalletType::Trezor => "https://trezor.io/favicon.ico",
            WalletType::Xverse => "https://www.xverse.app/favicon.ico",
            WalletType::Leather => "https://leather.io/favicon.ico",
            WalletType::OKXWallet => "https://www.okx.com/favicon.ico",
            _ => "",
        }
    }
}

impl std::fmt::Display for WalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Represents an active wallet connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletConnection {
    pub wallet_type: WalletType,
    pub chain: Chain,
    pub address: String,
    pub connected_at: chrono::DateTime<chrono::Utc>,
    pub session_topic: Option<String>, // For WalletConnect
}

impl WalletConnection {
    pub fn new(wallet_type: WalletType, chain: Chain, address: String) -> Self {
        Self {
            wallet_type,
            chain,
            address,
            connected_at: chrono::Utc::now(),
            session_topic: None,
        }
    }

    pub fn with_session(mut self, session_topic: String) -> Self {
        self.session_topic = Some(session_topic);
        self
    }
}

/// Connection request sent from client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionRequest {
    pub wallet_type: WalletType,
    pub chain: Chain,
    pub address: String,
    pub session_topic: Option<String>,
}

/// Connection manager for tracking active wallet connections
pub struct ConnectionManager {
    connections: std::sync::RwLock<std::collections::HashMap<String, WalletConnection>>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: std::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }

    /// Register a new wallet connection
    pub fn connect(&self, request: ConnectionRequest) -> Result<WalletConnection> {
        let connection = WalletConnection {
            wallet_type: request.wallet_type,
            chain: request.chain,
            address: request.address.clone(),
            connected_at: chrono::Utc::now(),
            session_topic: request.session_topic,
        };

        let mut connections = self.connections.write()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

        connections.insert(request.address, connection.clone());

        tracing::info!(
            "Wallet connected: {} ({:?}) on {:?}",
            connection.address,
            connection.wallet_type,
            connection.chain
        );

        Ok(connection)
    }

    /// Disconnect a wallet
    pub fn disconnect(&self, address: &str) -> Result<bool> {
        let mut connections = self.connections.write()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

        let removed = connections.remove(address).is_some();

        if removed {
            tracing::info!("Wallet disconnected: {}", address);
        }

        Ok(removed)
    }

    /// Get an active connection by address
    pub fn get_connection(&self, address: &str) -> Result<Option<WalletConnection>> {
        let connections = self.connections.read()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

        Ok(connections.get(address).cloned())
    }

    /// Get all active connections
    pub fn get_all_connections(&self) -> Result<Vec<WalletConnection>> {
        let connections = self.connections.read()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

        Ok(connections.values().cloned().collect())
    }

    /// Check if an address is connected
    pub fn is_connected(&self, address: &str) -> Result<bool> {
        let connections = self.connections.read()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

        Ok(connections.contains_key(address))
    }
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}
