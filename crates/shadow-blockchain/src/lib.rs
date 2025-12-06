//! Shadow OT Blockchain Integration
//!
//! Multi-chain blockchain support for Shadow OT, enabling:
//! - NFT minting of in-game assets across multiple chains
//! - Cross-chain asset bridges (Starknet, Ethereum, Polygon, Bitcoin/Spark)
//! - Web3 wallet authentication
//! - On-chain marketplace and trading
//! - Play-to-earn mechanics with native tokens

pub mod chains;
pub mod nft;
pub mod bridge;
pub mod wallet;
pub mod error;
pub mod config;

pub use error::{BlockchainError, Result};
pub use config::BlockchainConfig;
pub use chains::{create_providers, EvmProvider, StarknetProvider, BitcoinProvider};
pub use wallet::{WalletAuth, WalletAuthChallenge, WalletAuthResult, WalletConnection, WalletType, UserWallet, WalletManager};
pub use nft::{ShadowNft, NftCollection, NftManager, MetadataBuilder, MetadataGenerator, MintQueue, MintRequest, MintStatus, NftStorage, StoredNft};
pub use bridge::{BridgeService, BridgeConfig as BridgeServiceConfig, BridgeRoute as ServiceBridgeRoute, BridgeTransaction, BridgeStats, BridgeQueue, BridgeQueueStats, BridgeVerifier, VerificationResult};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Supported blockchain networks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Chain {
    /// Ethereum mainnet
    Ethereum,
    /// Ethereum Sepolia testnet
    EthereumSepolia,
    /// Polygon (Matic) mainnet
    Polygon,
    /// Polygon Mumbai testnet
    PolygonMumbai,
    /// Starknet mainnet
    Starknet,
    /// Starknet Goerli testnet
    StarknetGoerli,
    /// Starknet Sepolia testnet
    StarknetSepolia,
    /// Bitcoin mainnet (via Ordinals/Inscriptions)
    Bitcoin,
    /// Bitcoin testnet
    BitcoinTestnet,
    /// Spark (Bitcoin L2)
    Spark,
    /// Base (Coinbase L2)
    Base,
    /// Arbitrum One
    Arbitrum,
}

impl Chain {
    pub fn chain_id(&self) -> u64 {
        match self {
            Chain::Ethereum => 1,
            Chain::EthereumSepolia => 11155111,
            Chain::Polygon => 137,
            Chain::PolygonMumbai => 80001,
            Chain::Starknet => 0x534e5f4d41494e, // SN_MAIN
            Chain::StarknetGoerli => 0x534e5f474f45524c49, // SN_GOERLI
            Chain::StarknetSepolia => 0x534e5f5345504f4c4941, // SN_SEPOLIA
            Chain::Bitcoin => 0, // Bitcoin doesn't use chain IDs
            Chain::BitcoinTestnet => 0,
            Chain::Spark => 1000, // Placeholder
            Chain::Base => 8453,
            Chain::Arbitrum => 42161,
        }
    }

    pub fn is_evm(&self) -> bool {
        matches!(
            self,
            Chain::Ethereum
                | Chain::EthereumSepolia
                | Chain::Polygon
                | Chain::PolygonMumbai
                | Chain::Base
                | Chain::Arbitrum
        )
    }

    pub fn is_starknet(&self) -> bool {
        matches!(
            self,
            Chain::Starknet | Chain::StarknetGoerli | Chain::StarknetSepolia
        )
    }

    pub fn is_bitcoin(&self) -> bool {
        matches!(self, Chain::Bitcoin | Chain::BitcoinTestnet | Chain::Spark)
    }

    pub fn is_testnet(&self) -> bool {
        matches!(
            self,
            Chain::EthereumSepolia
                | Chain::PolygonMumbai
                | Chain::StarknetGoerli
                | Chain::StarknetSepolia
                | Chain::BitcoinTestnet
        )
    }
}

/// Asset types that can be minted as NFTs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetType {
    /// In-game item (weapon, armor, etc.)
    Item {
        item_id: u32,
        name: String,
        rarity: Rarity,
        attributes: Vec<ItemAttribute>,
    },
    /// Character outfit
    Outfit {
        outfit_id: u32,
        name: String,
        addons: u8,
    },
    /// Mount
    Mount {
        mount_id: u32,
        name: String,
    },
    /// House deed
    House {
        house_id: u32,
        name: String,
        realm_id: Uuid,
        size: u32,
        location: String,
    },
    /// Guild emblem/banner
    GuildAsset {
        guild_id: Uuid,
        asset_type: GuildAssetType,
    },
    /// Achievement badge
    Achievement {
        achievement_id: u32,
        name: String,
        points: u32,
    },
    /// Special event item
    EventItem {
        event_id: Uuid,
        item_id: u32,
        name: String,
        event_name: String,
    },
    /// Land/territory in a realm
    Territory {
        realm_id: Uuid,
        coordinates: (u16, u16, u16, u16), // x1, y1, x2, y2
        name: String,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
    Unique,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemAttribute {
    pub name: String,
    pub value: AttributeValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttributeValue {
    Number(i64),
    String(String),
    Boolean(bool),
    Range { min: i64, max: i64, current: i64 },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GuildAssetType {
    Emblem,
    Banner,
    Territory,
    Treasury,
}

/// NFT metadata following ERC-721/ERC-1155 standards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftMetadata {
    pub name: String,
    pub description: String,
    pub image: String,
    pub external_url: Option<String>,
    pub animation_url: Option<String>,
    pub attributes: Vec<NftAttribute>,
    pub properties: NftProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftAttribute {
    pub trait_type: String,
    pub value: serde_json::Value,
    pub display_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftProperties {
    pub game_id: String,
    pub realm_id: Option<Uuid>,
    pub asset_type: String,
    pub original_chain: Chain,
    pub bridged_chains: Vec<Chain>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub shadow_ot_version: String,
}

/// Trait for chain-specific implementations
#[async_trait]
pub trait ChainProvider: Send + Sync {
    /// Get the chain this provider handles
    fn chain(&self) -> Chain;

    /// Check if the provider is connected and healthy
    async fn health_check(&self) -> Result<bool>;

    /// Get the current block number
    async fn get_block_number(&self) -> Result<u64>;

    /// Mint an NFT on this chain
    async fn mint_nft(
        &self,
        to: &str,
        metadata: &NftMetadata,
        asset: &AssetType,
    ) -> Result<MintResult>;

    /// Transfer an NFT
    async fn transfer_nft(
        &self,
        token_id: &str,
        from: &str,
        to: &str,
    ) -> Result<TransferResult>;

    /// Get NFT owner
    async fn get_nft_owner(&self, token_id: &str) -> Result<String>;

    /// Verify wallet signature for authentication
    async fn verify_signature(
        &self,
        message: &str,
        signature: &str,
        address: &str,
    ) -> Result<bool>;

    /// Lock an asset for cross-chain bridging
    async fn lock_for_bridge(&self, token_id: &str, owner: &str) -> Result<String>;

    /// Unlock an asset after failed bridge
    async fn unlock_from_bridge(&self, token_id: &str, owner: &str) -> Result<String>;
}

/// Result of minting an NFT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintResult {
    pub chain: Chain,
    pub token_id: String,
    pub transaction_hash: String,
    pub contract_address: String,
    pub metadata_uri: String,
    pub minted_at: chrono::DateTime<chrono::Utc>,
}

/// Result of transferring an NFT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferResult {
    pub chain: Chain,
    pub token_id: String,
    pub transaction_hash: String,
    pub from: String,
    pub to: String,
    pub transferred_at: chrono::DateTime<chrono::Utc>,
}

/// Cross-chain bridge request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeRequest {
    pub request_id: Uuid,
    pub token_id: String,
    pub source_chain: Chain,
    pub target_chain: Chain,
    pub owner_address_source: String,
    pub owner_address_target: String,
    pub asset: AssetType,
    pub status: BridgeStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum BridgeStatus {
    Pending,
    LockingOnSource,
    LockedOnSource,
    MintingOnTarget,
    Completed,
    Failed,
    Cancelled,
}

/// Initialize the blockchain service with all configured chains
pub async fn init(config: BlockchainConfig) -> Result<BlockchainService> {
    BlockchainService::new(config).await
}

/// Main blockchain service coordinating all chain providers
pub struct BlockchainService {
    config: BlockchainConfig,
    providers: std::collections::HashMap<Chain, Box<dyn ChainProvider>>,
}

impl BlockchainService {
    pub async fn new(config: BlockchainConfig) -> Result<Self> {
        let providers = std::collections::HashMap::new();
        // Providers will be initialized based on config

        Ok(Self { config, providers })
    }

    /// Get a provider for a specific chain
    pub fn provider(&self, chain: Chain) -> Option<&dyn ChainProvider> {
        self.providers.get(&chain).map(|p| p.as_ref())
    }

    /// Mint an asset as NFT on multiple chains simultaneously
    pub async fn multi_chain_mint(
        &self,
        to_addresses: std::collections::HashMap<Chain, String>,
        metadata: &NftMetadata,
        asset: &AssetType,
    ) -> Result<Vec<MintResult>> {
        let mut results = Vec::new();

        for (chain, address) in to_addresses {
            if let Some(provider) = self.provider(chain) {
                match provider.mint_nft(&address, metadata, asset).await {
                    Ok(result) => results.push(result),
                    Err(e) => {
                        tracing::error!("Failed to mint on {:?}: {}", chain, e);
                    }
                }
            }
        }

        Ok(results)
    }

    /// Bridge an asset from one chain to another
    pub async fn bridge_asset(&self, mut request: BridgeRequest) -> Result<BridgeRequest> {
        tracing::info!(
            "Bridging asset {} from {:?} to {:?}",
            request.token_id,
            request.source_chain,
            request.target_chain
        );

        // Update status to locking
        request.status = BridgeStatus::LockingOnSource;

        // Get source chain provider
        let source_provider = self
            .provider(request.source_chain)
            .ok_or(BlockchainError::ChainNotSupported(request.source_chain))?;

        // Get target chain provider
        let target_provider = self
            .provider(request.target_chain)
            .ok_or(BlockchainError::ChainNotSupported(request.target_chain))?;

        // Step 1: Lock the asset on source chain
        match source_provider
            .lock_for_bridge(&request.token_id, &request.owner_address_source)
            .await
        {
            Ok(_) => {
                request.status = BridgeStatus::LockedOnSource;
                tracing::info!("Asset locked on source chain");
            }
            Err(e) => {
                request.status = BridgeStatus::Failed;
                tracing::error!("Failed to lock asset: {}", e);
                return Err(e);
            }
        }

        // Step 2: Mint wrapped asset on target chain
        request.status = BridgeStatus::MintingOnTarget;

        // Create metadata for bridged asset
        let metadata = NftMetadata {
            name: format!("Bridged {}", request.token_id),
            description: format!(
                "Asset bridged from {:?} to {:?}",
                request.source_chain, request.target_chain
            ),
            image: String::new(),
            external_url: None,
            animation_url: None,
            attributes: vec![
                NftAttribute {
                    trait_type: "original_chain".to_string(),
                    value: serde_json::Value::String(format!("{:?}", request.source_chain)),
                    display_type: None,
                },
                NftAttribute {
                    trait_type: "original_token_id".to_string(),
                    value: serde_json::Value::String(request.token_id.clone()),
                    display_type: None,
                },
            ],
            properties: NftProperties {
                game_id: "shadow-ot".to_string(),
                realm_id: None,
                asset_type: format!("{:?}", request.asset),
                original_chain: request.source_chain,
                bridged_chains: vec![request.target_chain],
                created_at: chrono::Utc::now(),
                shadow_ot_version: env!("CARGO_PKG_VERSION").to_string(),
            },
        };

        match target_provider
            .mint_nft(&request.owner_address_target, &metadata, &request.asset)
            .await
        {
            Ok(result) => {
                request.status = BridgeStatus::Completed;
                tracing::info!(
                    "Bridge complete! New token ID on target: {}",
                    result.token_id
                );
            }
            Err(e) => {
                request.status = BridgeStatus::Failed;
                // Attempt to unlock on source chain
                let _ = source_provider
                    .unlock_from_bridge(&request.token_id, &request.owner_address_source)
                    .await;
                tracing::error!("Failed to mint on target chain: {}", e);
                return Err(e);
            }
        }

        Ok(request)
    }
}
