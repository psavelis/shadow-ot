//! Cross-Chain Bridge Module
//!
//! Handles bridging assets between different blockchains.

pub mod queue;
pub mod verifier;

pub use queue::{BridgeQueue, BridgeQueueStats};
pub use verifier::{BridgeVerifier, VerificationResult};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{AssetType, BridgeRequest, BridgeStatus, Chain, Result, BlockchainError};

/// Configuration for the bridge service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeConfig {
    /// Supported bridge routes (source -> targets)
    pub routes: Vec<BridgeRoute>,
    /// Minimum confirmations required on source chain
    pub min_confirmations: u64,
    /// Bridge fee in basis points (100 = 1%)
    pub fee_bps: u16,
    /// Maximum pending bridges per user
    pub max_pending_per_user: usize,
    /// Bridge timeout in seconds
    pub timeout_secs: u64,
}

impl Default for BridgeConfig {
    fn default() -> Self {
        Self {
            routes: vec![
                BridgeRoute::new(Chain::Ethereum, Chain::Polygon),
                BridgeRoute::new(Chain::Ethereum, Chain::Starknet),
                BridgeRoute::new(Chain::Ethereum, Chain::Base),
                BridgeRoute::new(Chain::Polygon, Chain::Ethereum),
                BridgeRoute::new(Chain::Polygon, Chain::Starknet),
                BridgeRoute::new(Chain::Starknet, Chain::Ethereum),
                BridgeRoute::new(Chain::Starknet, Chain::Polygon),
                BridgeRoute::new(Chain::Bitcoin, Chain::Spark),
                BridgeRoute::new(Chain::Spark, Chain::Bitcoin),
            ],
            min_confirmations: 12,
            fee_bps: 50, // 0.5%
            max_pending_per_user: 5,
            timeout_secs: 3600, // 1 hour
        }
    }
}

/// A supported bridge route
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeRoute {
    pub source: Chain,
    pub target: Chain,
    pub enabled: bool,
    pub min_amount: Option<u64>,
    pub max_amount: Option<u64>,
}

impl BridgeRoute {
    pub fn new(source: Chain, target: Chain) -> Self {
        Self {
            source,
            target,
            enabled: true,
            min_amount: None,
            max_amount: None,
        }
    }

    pub fn with_limits(mut self, min: u64, max: u64) -> Self {
        self.min_amount = Some(min);
        self.max_amount = Some(max);
        self
    }

    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
}

/// Bridge transaction record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeTransaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub request: BridgeRequest,
    pub source_tx_hash: Option<String>,
    pub target_tx_hash: Option<String>,
    pub fee_amount: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl BridgeTransaction {
    pub fn new(user_id: Uuid, request: BridgeRequest) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            request,
            source_tx_hash: None,
            target_tx_hash: None,
            fee_amount: 0,
            created_at: now,
            updated_at: now,
            completed_at: None,
        }
    }

    pub fn set_source_tx(&mut self, tx_hash: &str) {
        self.source_tx_hash = Some(tx_hash.to_string());
        self.updated_at = chrono::Utc::now();
    }

    pub fn set_target_tx(&mut self, tx_hash: &str) {
        self.target_tx_hash = Some(tx_hash.to_string());
        self.updated_at = chrono::Utc::now();
    }

    pub fn complete(&mut self) {
        self.request.status = BridgeStatus::Completed;
        self.completed_at = Some(chrono::Utc::now());
        self.updated_at = chrono::Utc::now();
    }

    pub fn fail(&mut self) {
        self.request.status = BridgeStatus::Failed;
        self.updated_at = chrono::Utc::now();
    }

    pub fn is_complete(&self) -> bool {
        self.request.status == BridgeStatus::Completed
    }

    pub fn is_failed(&self) -> bool {
        self.request.status == BridgeStatus::Failed
    }

    pub fn is_pending(&self) -> bool {
        !self.is_complete() && !self.is_failed()
    }
}

/// Bridge service managing cross-chain transfers
pub struct BridgeService {
    config: BridgeConfig,
    transactions: std::sync::RwLock<std::collections::HashMap<Uuid, BridgeTransaction>>,
    user_pending: std::sync::RwLock<std::collections::HashMap<Uuid, Vec<Uuid>>>,
}

impl BridgeService {
    pub fn new(config: BridgeConfig) -> Self {
        Self {
            config,
            transactions: std::sync::RwLock::new(std::collections::HashMap::new()),
            user_pending: std::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }

    /// Check if a route is supported
    pub fn is_route_supported(&self, source: Chain, target: Chain) -> bool {
        self.config.routes.iter().any(|r| {
            r.enabled && r.source == source && r.target == target
        })
    }

    /// Get supported target chains for a source
    pub fn get_supported_targets(&self, source: Chain) -> Vec<Chain> {
        self.config.routes.iter()
            .filter(|r| r.enabled && r.source == source)
            .map(|r| r.target)
            .collect()
    }

    /// Initiate a bridge request
    pub fn initiate(
        &self,
        user_id: Uuid,
        token_id: &str,
        source_chain: Chain,
        target_chain: Chain,
        source_address: &str,
        target_address: &str,
        asset: AssetType,
    ) -> Result<BridgeTransaction> {
        // Check route support
        if !self.is_route_supported(source_chain, target_chain) {
            return Err(BlockchainError::UnsupportedBridgeRoute(source_chain, target_chain));
        }

        // Check user's pending count
        {
            let user_pending = self.user_pending.read()
                .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

            if let Some(pending) = user_pending.get(&user_id) {
                if pending.len() >= self.config.max_pending_per_user {
                    return Err(BlockchainError::TooManyPendingBridges);
                }
            }
        }

        // Create bridge request
        let request = BridgeRequest {
            request_id: Uuid::new_v4(),
            token_id: token_id.to_string(),
            source_chain,
            target_chain,
            owner_address_source: source_address.to_string(),
            owner_address_target: target_address.to_string(),
            asset,
            status: BridgeStatus::Pending,
            created_at: chrono::Utc::now(),
        };

        let transaction = BridgeTransaction::new(user_id, request);
        let tx_id = transaction.id;

        // Store transaction
        {
            let mut transactions = self.transactions.write()
                .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;
            transactions.insert(tx_id, transaction.clone());
        }

        // Track user's pending bridges
        {
            let mut user_pending = self.user_pending.write()
                .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;
            user_pending.entry(user_id).or_default().push(tx_id);
        }

        tracing::info!(
            "Bridge initiated: {} from {:?} to {:?} for user {}",
            token_id,
            source_chain,
            target_chain,
            user_id
        );

        Ok(transaction)
    }

    /// Get a bridge transaction
    pub fn get(&self, id: Uuid) -> Result<Option<BridgeTransaction>> {
        let transactions = self.transactions.read()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;
        Ok(transactions.get(&id).cloned())
    }

    /// Get user's bridge transactions
    pub fn get_user_bridges(&self, user_id: Uuid) -> Result<Vec<BridgeTransaction>> {
        let transactions = self.transactions.read()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

        Ok(transactions
            .values()
            .filter(|t| t.user_id == user_id)
            .cloned()
            .collect())
    }

    /// Update a bridge transaction
    pub fn update(&self, transaction: BridgeTransaction) -> Result<()> {
        let id = transaction.id;
        let user_id = transaction.user_id;
        let is_complete = transaction.is_complete() || transaction.is_failed();

        {
            let mut transactions = self.transactions.write()
                .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;
            transactions.insert(id, transaction);
        }

        // Remove from pending if complete
        if is_complete {
            let mut user_pending = self.user_pending.write()
                .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

            if let Some(pending) = user_pending.get_mut(&user_id) {
                pending.retain(|i| *i != id);
            }
        }

        Ok(())
    }

    /// Get bridge statistics
    pub fn stats(&self) -> Result<BridgeStats> {
        let transactions = self.transactions.read()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

        let total = transactions.len();
        let pending = transactions.values().filter(|t| t.is_pending()).count();
        let completed = transactions.values().filter(|t| t.is_complete()).count();
        let failed = transactions.values().filter(|t| t.is_failed()).count();

        Ok(BridgeStats {
            total,
            pending,
            completed,
            failed,
        })
    }

    /// Clean up old completed/failed transactions
    pub fn cleanup(&self, older_than: chrono::Duration) -> Result<usize> {
        let cutoff = chrono::Utc::now() - older_than;

        let mut transactions = self.transactions.write()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

        let before = transactions.len();
        transactions.retain(|_, t| {
            t.is_pending() || t.updated_at > cutoff
        });

        Ok(before - transactions.len())
    }
}

/// Bridge statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeStats {
    pub total: usize,
    pub pending: usize,
    pub completed: usize,
    pub failed: usize,
}
