//! NFT Storage and Persistence
//!
//! In-memory and database storage for NFT records.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

use crate::{Chain, MintResult, NftMetadata, Result, BlockchainError};

/// A stored NFT record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredNft {
    pub id: Uuid,
    pub token_id: String,
    pub chain: Chain,
    pub contract_address: String,
    pub owner_address: String,
    pub owner_user_id: Option<Uuid>,
    pub metadata_uri: String,
    pub metadata: Option<NftMetadata>,
    pub is_locked: bool,
    pub lock_reason: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl StoredNft {
    pub fn from_mint_result(result: MintResult, owner_address: &str) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            token_id: result.token_id,
            chain: result.chain,
            contract_address: result.contract_address,
            owner_address: owner_address.to_string(),
            owner_user_id: None,
            metadata_uri: result.metadata_uri,
            metadata: None,
            is_locked: false,
            lock_reason: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_user(mut self, user_id: Uuid) -> Self {
        self.owner_user_id = Some(user_id);
        self
    }

    pub fn with_metadata(mut self, metadata: NftMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn lock(&mut self, reason: &str) {
        self.is_locked = true;
        self.lock_reason = Some(reason.to_string());
        self.updated_at = chrono::Utc::now();
    }

    pub fn unlock(&mut self) {
        self.is_locked = false;
        self.lock_reason = None;
        self.updated_at = chrono::Utc::now();
    }

    pub fn transfer(&mut self, new_owner: &str, new_user_id: Option<Uuid>) {
        self.owner_address = new_owner.to_string();
        self.owner_user_id = new_user_id;
        self.updated_at = chrono::Utc::now();
    }
}

/// In-memory NFT storage
pub struct NftStorage {
    /// NFTs by internal ID
    by_id: std::sync::RwLock<HashMap<Uuid, StoredNft>>,
    /// Token ID to internal ID mapping per chain
    by_token: std::sync::RwLock<HashMap<(Chain, String), Uuid>>,
    /// User ID to NFT IDs mapping
    by_user: std::sync::RwLock<HashMap<Uuid, Vec<Uuid>>>,
    /// Address to NFT IDs mapping
    by_address: std::sync::RwLock<HashMap<String, Vec<Uuid>>>,
}

impl NftStorage {
    pub fn new() -> Self {
        Self {
            by_id: std::sync::RwLock::new(HashMap::new()),
            by_token: std::sync::RwLock::new(HashMap::new()),
            by_user: std::sync::RwLock::new(HashMap::new()),
            by_address: std::sync::RwLock::new(HashMap::new()),
        }
    }

    /// Store a new NFT
    pub fn store(&self, nft: StoredNft) -> Result<()> {
        let id = nft.id;
        let chain = nft.chain;
        let token_id = nft.token_id.clone();
        let address = nft.owner_address.clone();
        let user_id = nft.owner_user_id;

        // Store by ID
        {
            let mut by_id = self.by_id.write()
                .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;
            by_id.insert(id, nft);
        }

        // Index by token
        {
            let mut by_token = self.by_token.write()
                .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;
            by_token.insert((chain, token_id), id);
        }

        // Index by address
        {
            let mut by_address = self.by_address.write()
                .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;
            by_address.entry(address).or_default().push(id);
        }

        // Index by user if present
        if let Some(uid) = user_id {
            let mut by_user = self.by_user.write()
                .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;
            by_user.entry(uid).or_default().push(id);
        }

        Ok(())
    }

    /// Get an NFT by internal ID
    pub fn get(&self, id: Uuid) -> Result<Option<StoredNft>> {
        let by_id = self.by_id.read()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;
        Ok(by_id.get(&id).cloned())
    }

    /// Get an NFT by chain and token ID
    pub fn get_by_token(&self, chain: Chain, token_id: &str) -> Result<Option<StoredNft>> {
        let by_token = self.by_token.read()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

        if let Some(id) = by_token.get(&(chain, token_id.to_string())) {
            return self.get(*id);
        }

        Ok(None)
    }

    /// Get all NFTs owned by a user
    pub fn get_by_user(&self, user_id: Uuid) -> Result<Vec<StoredNft>> {
        let by_user = self.by_user.read()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

        let ids = by_user.get(&user_id).cloned().unwrap_or_default();

        let by_id = self.by_id.read()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

        Ok(ids.iter().filter_map(|id| by_id.get(id).cloned()).collect())
    }

    /// Get all NFTs owned by an address
    pub fn get_by_address(&self, address: &str) -> Result<Vec<StoredNft>> {
        let by_address = self.by_address.read()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

        let ids = by_address.get(address).cloned().unwrap_or_default();

        let by_id = self.by_id.read()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

        Ok(ids.iter().filter_map(|id| by_id.get(id).cloned()).collect())
    }

    /// Update an NFT
    pub fn update(&self, nft: StoredNft) -> Result<()> {
        let mut by_id = self.by_id.write()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

        if by_id.contains_key(&nft.id) {
            by_id.insert(nft.id, nft);
            Ok(())
        } else {
            Err(BlockchainError::NftNotFound(nft.id.to_string()))
        }
    }

    /// Record a transfer (updates ownership indices)
    pub fn record_transfer(
        &self,
        id: Uuid,
        old_address: &str,
        new_address: &str,
        new_user_id: Option<Uuid>,
    ) -> Result<()> {
        // Update the NFT record
        {
            let mut by_id = self.by_id.write()
                .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

            if let Some(nft) = by_id.get_mut(&id) {
                let old_user = nft.owner_user_id;
                nft.transfer(new_address, new_user_id);

                // Update user index
                if old_user != new_user_id {
                    drop(by_id);

                    let mut by_user = self.by_user.write()
                        .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

                    // Remove from old user
                    if let Some(uid) = old_user {
                        if let Some(ids) = by_user.get_mut(&uid) {
                            ids.retain(|i| *i != id);
                        }
                    }

                    // Add to new user
                    if let Some(uid) = new_user_id {
                        by_user.entry(uid).or_default().push(id);
                    }
                }
            } else {
                return Err(BlockchainError::NftNotFound(id.to_string()));
            }
        }

        // Update address index
        {
            let mut by_address = self.by_address.write()
                .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

            // Remove from old address
            if let Some(ids) = by_address.get_mut(old_address) {
                ids.retain(|i| *i != id);
            }

            // Add to new address
            by_address.entry(new_address.to_string()).or_default().push(id);
        }

        Ok(())
    }

    /// Get total count
    pub fn count(&self) -> Result<usize> {
        let by_id = self.by_id.read()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;
        Ok(by_id.len())
    }

    /// Get counts by chain
    pub fn count_by_chain(&self) -> Result<HashMap<Chain, usize>> {
        let by_id = self.by_id.read()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

        let mut counts = HashMap::new();
        for nft in by_id.values() {
            *counts.entry(nft.chain).or_insert(0) += 1;
        }

        Ok(counts)
    }
}

impl Default for NftStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nft_storage() {
        let storage = NftStorage::new();

        let mint_result = MintResult {
            chain: Chain::Polygon,
            token_id: "123".to_string(),
            transaction_hash: "0x123".to_string(),
            contract_address: "0xabc".to_string(),
            metadata_uri: "ipfs://test".to_string(),
            minted_at: chrono::Utc::now(),
        };

        let user_id = Uuid::new_v4();
        let nft = StoredNft::from_mint_result(mint_result, "0xowner")
            .with_user(user_id);

        let id = nft.id;

        storage.store(nft).unwrap();

        // Get by ID
        let found = storage.get(id).unwrap().unwrap();
        assert_eq!(found.token_id, "123");

        // Get by token
        let found = storage.get_by_token(Chain::Polygon, "123").unwrap().unwrap();
        assert_eq!(found.id, id);

        // Get by user
        let user_nfts = storage.get_by_user(user_id).unwrap();
        assert_eq!(user_nfts.len(), 1);

        // Count
        assert_eq!(storage.count().unwrap(), 1);
    }
}
