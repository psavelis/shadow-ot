//! Web3 Wallet Integration
//!
//! Multi-chain wallet support for authentication and transactions.

pub mod auth;
pub mod connect;

pub use auth::{WalletAuth, WalletAuthChallenge, WalletAuthResult};
pub use connect::{WalletConnection, WalletType};

use crate::{Chain, Result, BlockchainError};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Wallet information associated with a user account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserWallet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub chain: Chain,
    pub address: String,
    pub wallet_type: WalletType,
    pub is_primary: bool,
    pub verified_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_used_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl UserWallet {
    pub fn new(user_id: Uuid, chain: Chain, address: String, wallet_type: WalletType) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            chain,
            address,
            wallet_type,
            is_primary: false,
            verified_at: None,
            created_at: chrono::Utc::now(),
            last_used_at: None,
        }
    }

    pub fn verify(&mut self) {
        self.verified_at = Some(chrono::Utc::now());
    }

    pub fn is_verified(&self) -> bool {
        self.verified_at.is_some()
    }

    pub fn touch(&mut self) {
        self.last_used_at = Some(chrono::Utc::now());
    }
}

/// Wallet manager for handling user wallets
pub struct WalletManager {
    // In production, would include database pool
    wallets: std::sync::RwLock<std::collections::HashMap<Uuid, Vec<UserWallet>>>,
}

impl WalletManager {
    pub fn new() -> Self {
        Self {
            wallets: std::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }

    /// Add a wallet for a user
    pub fn add_wallet(&self, wallet: UserWallet) -> Result<()> {
        let mut wallets = self.wallets.write()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

        wallets
            .entry(wallet.user_id)
            .or_default()
            .push(wallet);

        Ok(())
    }

    /// Get all wallets for a user
    pub fn get_user_wallets(&self, user_id: Uuid) -> Result<Vec<UserWallet>> {
        let wallets = self.wallets.read()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

        Ok(wallets.get(&user_id).cloned().unwrap_or_default())
    }

    /// Get a user's primary wallet for a chain
    pub fn get_primary_wallet(&self, user_id: Uuid, chain: Chain) -> Result<Option<UserWallet>> {
        let wallets = self.get_user_wallets(user_id)?;

        Ok(wallets
            .into_iter()
            .find(|w| w.chain == chain && w.is_primary))
    }

    /// Set a wallet as primary for its chain
    pub fn set_primary(&self, user_id: Uuid, wallet_id: Uuid) -> Result<()> {
        let mut wallets = self.wallets.write()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

        if let Some(user_wallets) = wallets.get_mut(&user_id) {
            // Find the wallet to make primary and get its chain
            let chain = user_wallets
                .iter()
                .find(|w| w.id == wallet_id)
                .map(|w| w.chain);

            if let Some(chain) = chain {
                // Unset other wallets of the same chain
                for wallet in user_wallets.iter_mut() {
                    if wallet.chain == chain {
                        wallet.is_primary = wallet.id == wallet_id;
                    }
                }
            }
        }

        Ok(())
    }

    /// Remove a wallet
    pub fn remove_wallet(&self, user_id: Uuid, wallet_id: Uuid) -> Result<bool> {
        let mut wallets = self.wallets.write()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

        if let Some(user_wallets) = wallets.get_mut(&user_id) {
            let len_before = user_wallets.len();
            user_wallets.retain(|w| w.id != wallet_id);
            return Ok(user_wallets.len() < len_before);
        }

        Ok(false)
    }
}

impl Default for WalletManager {
    fn default() -> Self {
        Self::new()
    }
}
