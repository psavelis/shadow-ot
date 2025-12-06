//! Blockchain error types

use thiserror::Error;
use crate::Chain;

pub type Result<T> = std::result::Result<T, BlockchainError>;

#[derive(Error, Debug)]
pub enum BlockchainError {
    #[error("Chain not supported: {0:?}")]
    ChainNotSupported(Chain),

    #[error("Chain not configured: {0:?}")]
    ChainNotConfigured(Chain),

    #[error("Provider error on {chain:?}: {message}")]
    Provider { chain: Chain, message: String },

    #[error("Contract error: {0}")]
    Contract(String),

    #[error("Transaction failed: {0}")]
    TransactionFailed(String),

    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    #[error("Invalid signature: {0}")]
    InvalidSignature(String),

    #[error("NFT not found: {0}")]
    NftNotFound(String),

    #[error("Insufficient funds: need {needed}, have {available}")]
    InsufficientFunds { needed: String, available: String },

    #[error("Bridge error: {0}")]
    Bridge(String),

    #[error("Unsupported bridge route: {0:?} to {1:?}")]
    UnsupportedBridgeRoute(Chain, Chain),

    #[error("Too many pending bridge requests")]
    TooManyPendingBridges,

    #[error("Queue is full")]
    QueueFull,

    #[error("IPFS error: {0}")]
    Ipfs(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Rate limited on {0:?}")]
    RateLimited(Chain),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Timeout waiting for {0}")]
    Timeout(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<reqwest::Error> for BlockchainError {
    fn from(err: reqwest::Error) -> Self {
        BlockchainError::Network(err.to_string())
    }
}

impl From<serde_json::Error> for BlockchainError {
    fn from(err: serde_json::Error) -> Self {
        BlockchainError::Serialization(err.to_string())
    }
}
