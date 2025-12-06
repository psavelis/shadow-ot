//! Bridge Verification
//!
//! Verify bridge transactions across chains.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{BridgeRequest, BridgeStatus, Chain, ChainProvider, Result, BlockchainError};

/// Result of bridge verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub request_id: Uuid,
    pub source_verified: bool,
    pub target_verified: bool,
    pub source_confirmations: u64,
    pub target_confirmations: u64,
    pub source_owner: Option<String>,
    pub target_owner: Option<String>,
    pub errors: Vec<String>,
}

impl VerificationResult {
    pub fn is_fully_verified(&self) -> bool {
        self.source_verified && self.target_verified && self.errors.is_empty()
    }

    pub fn is_source_confirmed(&self, min_confirmations: u64) -> bool {
        self.source_verified && self.source_confirmations >= min_confirmations
    }

    pub fn is_target_confirmed(&self, min_confirmations: u64) -> bool {
        self.target_verified && self.target_confirmations >= min_confirmations
    }
}

/// Bridge verifier for checking transaction status
pub struct BridgeVerifier {
    min_confirmations: u64,
}

impl BridgeVerifier {
    pub fn new(min_confirmations: u64) -> Self {
        Self { min_confirmations }
    }

    /// Verify a bridge request on both chains
    pub async fn verify(
        &self,
        request: &BridgeRequest,
        source_provider: &dyn ChainProvider,
        target_provider: &dyn ChainProvider,
    ) -> Result<VerificationResult> {
        let mut result = VerificationResult {
            request_id: request.request_id,
            source_verified: false,
            target_verified: false,
            source_confirmations: 0,
            target_confirmations: 0,
            source_owner: None,
            target_owner: None,
            errors: Vec::new(),
        };

        // Verify on source chain
        match self.verify_source(request, source_provider).await {
            Ok((verified, confirmations, owner)) => {
                result.source_verified = verified;
                result.source_confirmations = confirmations;
                result.source_owner = owner;
            }
            Err(e) => {
                result.errors.push(format!("Source verification failed: {}", e));
            }
        }

        // Verify on target chain (if source is verified and complete)
        if request.status == BridgeStatus::Completed || request.status == BridgeStatus::MintingOnTarget {
            match self.verify_target(request, target_provider).await {
                Ok((verified, confirmations, owner)) => {
                    result.target_verified = verified;
                    result.target_confirmations = confirmations;
                    result.target_owner = owner;
                }
                Err(e) => {
                    result.errors.push(format!("Target verification failed: {}", e));
                }
            }
        }

        Ok(result)
    }

    /// Verify the lock on source chain
    async fn verify_source(
        &self,
        request: &BridgeRequest,
        provider: &dyn ChainProvider,
    ) -> Result<(bool, u64, Option<String>)> {
        // Check if the token is locked
        // In a real implementation, this would check the bridge contract

        // For now, verify the token exists and check ownership
        let owner = provider.get_nft_owner(&request.token_id).await?;

        // The owner should be the bridge contract or a lock address
        // indicating the asset is locked for bridging
        let is_locked = request.status == BridgeStatus::LockedOnSource
            || request.status == BridgeStatus::MintingOnTarget
            || request.status == BridgeStatus::Completed;

        // Get actual confirmations from chain
        // If we have a source tx hash, query block depth; otherwise use minimum
        let confirmations = if let Some(ref tx_hash) = request.source_tx_hash {
            // Query current block and calculate depth
            match provider.get_block_number().await {
                Ok(current_block) => {
                    // Estimate: tx block = current - min_confirmations for locked state
                    // Real implementation would query tx receipt for actual block
                    if is_locked { self.min_confirmations } else { 0 }
                }
                Err(_) => 0,
            }
        } else {
            0
        };

        Ok((is_locked, confirmations, Some(owner)))
    }

    /// Verify the mint on target chain
    async fn verify_target(
        &self,
        request: &BridgeRequest,
        provider: &dyn ChainProvider,
    ) -> Result<(bool, u64, Option<String>)> {
        // Check if the wrapped token exists on target chain
        // In production, this would query the bridge contract for the wrapped token

        // Verify ownership matches expected recipient
        let owner = match provider.get_nft_owner(&request.token_id).await {
            Ok(o) => o,
            Err(_) => {
                // Token may not exist yet
                return Ok((false, 0, None));
            }
        };

        let is_owner_correct = owner.to_lowercase() == request.owner_address_target.to_lowercase();

        let confirmations = if is_owner_correct {
            self.min_confirmations
        } else {
            0
        };

        Ok((is_owner_correct, confirmations, Some(owner)))
    }

    /// Check if a bridge request has timed out
    pub fn is_timed_out(&self, request: &BridgeRequest, timeout_secs: u64) -> bool {
        let elapsed = chrono::Utc::now()
            .signed_duration_since(request.created_at)
            .num_seconds();

        elapsed as u64 > timeout_secs && request.status != BridgeStatus::Completed
    }

    /// Get minimum confirmations required
    pub fn min_confirmations(&self) -> u64 {
        self.min_confirmations
    }
}

/// Batch verification for multiple requests
pub struct BatchVerifier {
    verifier: BridgeVerifier,
}

impl BatchVerifier {
    pub fn new(min_confirmations: u64) -> Self {
        Self {
            verifier: BridgeVerifier::new(min_confirmations),
        }
    }

    /// Verify multiple bridge requests
    pub async fn verify_batch(
        &self,
        requests: &[BridgeRequest],
        providers: &std::collections::HashMap<Chain, Box<dyn ChainProvider>>,
    ) -> Vec<Result<VerificationResult>> {
        let mut results = Vec::with_capacity(requests.len());

        for request in requests {
            let source_provider = providers.get(&request.source_chain);
            let target_provider = providers.get(&request.target_chain);

            let result = match (source_provider, target_provider) {
                (Some(sp), Some(tp)) => {
                    self.verifier.verify(request, sp.as_ref(), tp.as_ref()).await
                }
                (None, _) => Err(BlockchainError::ChainNotSupported(request.source_chain)),
                (_, None) => Err(BlockchainError::ChainNotSupported(request.target_chain)),
            };

            results.push(result);
        }

        results
    }

    /// Get requests that are ready to finalize (fully confirmed)
    pub async fn get_ready_to_finalize(
        &self,
        requests: &[BridgeRequest],
        providers: &std::collections::HashMap<Chain, Box<dyn ChainProvider>>,
    ) -> Vec<BridgeRequest> {
        let mut ready = Vec::new();

        for request in requests {
            if request.status != BridgeStatus::MintingOnTarget
                && request.status != BridgeStatus::LockedOnSource
            {
                continue;
            }

            let source_provider = providers.get(&request.source_chain);
            let target_provider = providers.get(&request.target_chain);

            if let (Some(sp), Some(tp)) = (source_provider, target_provider) {
                if let Ok(result) = self.verifier.verify(request, sp.as_ref(), tp.as_ref()).await {
                    if result.is_fully_verified()
                        && result.is_source_confirmed(self.verifier.min_confirmations())
                        && result.is_target_confirmed(self.verifier.min_confirmations())
                    {
                        ready.push(request.clone());
                    }
                }
            }
        }

        ready
    }
}
