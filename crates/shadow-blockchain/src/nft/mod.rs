//! NFT Management Module
//!
//! Handles NFT minting, metadata generation, and asset tracking.

pub mod metadata;
pub mod minting;
pub mod storage;

pub use metadata::{MetadataBuilder, MetadataGenerator};
pub use minting::{MintQueue, MintRequest, MintStatus};
pub use storage::{NftStorage, StoredNft};

use crate::{AssetType, Chain, MintResult, NftMetadata, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents an NFT in the Shadow OT system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowNft {
    /// Internal ID
    pub id: Uuid,
    /// On-chain token ID
    pub token_id: String,
    /// Chain where this NFT exists
    pub chain: Chain,
    /// Contract address
    pub contract_address: String,
    /// Current owner's address
    pub owner_address: String,
    /// Owner's user ID (if known)
    pub owner_user_id: Option<Uuid>,
    /// The in-game asset this NFT represents
    pub asset: AssetType,
    /// NFT metadata
    pub metadata: NftMetadata,
    /// Mint result from blockchain
    pub mint_result: MintResult,
    /// Whether this NFT is locked (e.g., for bridging)
    pub is_locked: bool,
    /// Chains this asset has been bridged to
    pub bridged_to: Vec<Chain>,
    /// Created timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last updated timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl ShadowNft {
    pub fn new(
        asset: AssetType,
        metadata: NftMetadata,
        mint_result: MintResult,
        owner_address: String,
    ) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            token_id: mint_result.token_id.clone(),
            chain: mint_result.chain,
            contract_address: mint_result.contract_address.clone(),
            owner_address,
            owner_user_id: None,
            asset,
            metadata,
            mint_result,
            is_locked: false,
            bridged_to: vec![],
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_user(mut self, user_id: Uuid) -> Self {
        self.owner_user_id = Some(user_id);
        self
    }

    pub fn lock(&mut self) {
        self.is_locked = true;
        self.updated_at = chrono::Utc::now();
    }

    pub fn unlock(&mut self) {
        self.is_locked = false;
        self.updated_at = chrono::Utc::now();
    }

    pub fn add_bridge(&mut self, chain: Chain) {
        if !self.bridged_to.contains(&chain) {
            self.bridged_to.push(chain);
            self.updated_at = chrono::Utc::now();
        }
    }
}

/// NFT collection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftCollection {
    pub id: Uuid,
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub chain: Chain,
    pub contract_address: String,
    pub base_uri: String,
    pub max_supply: Option<u64>,
    pub minted_count: u64,
    pub royalty_bps: u16, // Basis points (100 = 1%)
    pub royalty_recipient: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl NftCollection {
    pub fn new(
        name: &str,
        symbol: &str,
        description: &str,
        chain: Chain,
        contract_address: &str,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            symbol: symbol.to_string(),
            description: description.to_string(),
            chain,
            contract_address: contract_address.to_string(),
            base_uri: String::new(),
            max_supply: None,
            minted_count: 0,
            royalty_bps: 250, // 2.5% default
            royalty_recipient: String::new(),
            created_at: chrono::Utc::now(),
        }
    }

    pub fn with_base_uri(mut self, uri: &str) -> Self {
        self.base_uri = uri.to_string();
        self
    }

    pub fn with_max_supply(mut self, supply: u64) -> Self {
        self.max_supply = Some(supply);
        self
    }

    pub fn with_royalty(mut self, bps: u16, recipient: &str) -> Self {
        self.royalty_bps = bps;
        self.royalty_recipient = recipient.to_string();
        self
    }

    pub fn can_mint(&self) -> bool {
        match self.max_supply {
            Some(max) => self.minted_count < max,
            None => true,
        }
    }

    pub fn increment_minted(&mut self) {
        self.minted_count += 1;
    }
}

/// Manages NFT collections across chains
pub struct NftManager {
    collections: std::sync::RwLock<std::collections::HashMap<Uuid, NftCollection>>,
    nfts: std::sync::RwLock<std::collections::HashMap<Uuid, ShadowNft>>,
}

impl NftManager {
    pub fn new() -> Self {
        Self {
            collections: std::sync::RwLock::new(std::collections::HashMap::new()),
            nfts: std::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }

    /// Register a new NFT collection
    pub fn register_collection(&self, collection: NftCollection) -> Result<()> {
        let mut collections = self.collections.write()
            .map_err(|_| crate::BlockchainError::InternalError("Lock poisoned".into()))?;

        collections.insert(collection.id, collection);
        Ok(())
    }

    /// Get a collection by ID
    pub fn get_collection(&self, id: Uuid) -> Result<Option<NftCollection>> {
        let collections = self.collections.read()
            .map_err(|_| crate::BlockchainError::InternalError("Lock poisoned".into()))?;

        Ok(collections.get(&id).cloned())
    }

    /// Get collections for a chain
    pub fn get_chain_collections(&self, chain: Chain) -> Result<Vec<NftCollection>> {
        let collections = self.collections.read()
            .map_err(|_| crate::BlockchainError::InternalError("Lock poisoned".into()))?;

        Ok(collections
            .values()
            .filter(|c| c.chain == chain)
            .cloned()
            .collect())
    }

    /// Store a minted NFT
    pub fn store_nft(&self, nft: ShadowNft) -> Result<()> {
        let mut nfts = self.nfts.write()
            .map_err(|_| crate::BlockchainError::InternalError("Lock poisoned".into()))?;

        nfts.insert(nft.id, nft);
        Ok(())
    }

    /// Get an NFT by ID
    pub fn get_nft(&self, id: Uuid) -> Result<Option<ShadowNft>> {
        let nfts = self.nfts.read()
            .map_err(|_| crate::BlockchainError::InternalError("Lock poisoned".into()))?;

        Ok(nfts.get(&id).cloned())
    }

    /// Get NFTs owned by a user
    pub fn get_user_nfts(&self, user_id: Uuid) -> Result<Vec<ShadowNft>> {
        let nfts = self.nfts.read()
            .map_err(|_| crate::BlockchainError::InternalError("Lock poisoned".into()))?;

        Ok(nfts
            .values()
            .filter(|n| n.owner_user_id == Some(user_id))
            .cloned()
            .collect())
    }

    /// Get NFTs by address
    pub fn get_address_nfts(&self, address: &str) -> Result<Vec<ShadowNft>> {
        let nfts = self.nfts.read()
            .map_err(|_| crate::BlockchainError::InternalError("Lock poisoned".into()))?;

        Ok(nfts
            .values()
            .filter(|n| n.owner_address == address)
            .cloned()
            .collect())
    }
}

impl Default for NftManager {
    fn default() -> Self {
        Self::new()
    }
}
