//! Bitcoin and Bitcoin L2 chain provider
//!
//! Supports Bitcoin Ordinals/Inscriptions and Spark L2 for NFT minting.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{
    error::BlockchainError, AssetType, Chain, ChainProvider, MintResult, NftMetadata, Result,
    TransferResult,
};

/// Configuration for Bitcoin-based chains
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitcoinChainConfig {
    pub chain: Chain,
    pub rpc_url: String,
    pub rpc_user: Option<String>,
    pub rpc_password: Option<String>,
    pub network: BitcoinNetwork,
    pub inscription_fee_rate: u64,
    pub wallet_name: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BitcoinNetwork {
    Mainnet,
    Testnet,
    Regtest,
    Signet,
}

impl Default for BitcoinChainConfig {
    fn default() -> Self {
        Self {
            chain: Chain::BitcoinTestnet,
            rpc_url: "http://localhost:18332".to_string(),
            rpc_user: None,
            rpc_password: None,
            network: BitcoinNetwork::Testnet,
            inscription_fee_rate: 10,
            wallet_name: None,
        }
    }
}

/// Bitcoin chain provider for Ordinals/Inscriptions
pub struct BitcoinProvider {
    config: BitcoinChainConfig,
    // In production:
    // client: bitcoincore_rpc::Client,
    // ord_client: ord::Client, // For ordinals
}

impl BitcoinProvider {
    pub async fn new(config: BitcoinChainConfig) -> Result<Self> {
        tracing::info!(
            "Initializing Bitcoin provider for {:?} at {}",
            config.chain,
            config.rpc_url
        );

        // In production:
        // let auth = Auth::UserPass(user, password);
        // let client = Client::new(&config.rpc_url, auth)?;

        Ok(Self { config })
    }

    /// Validate a Bitcoin address
    fn validate_address(address: &str, _network: BitcoinNetwork) -> Result<()> {
        // Basic validation - in production use bitcoin crate
        if address.is_empty() {
            return Err(BlockchainError::InvalidAddress("Empty address".to_string()));
        }

        // Mainnet addresses start with 1, 3, or bc1
        // Testnet addresses start with m, n, 2, or tb1
        let valid_prefixes = ["1", "3", "bc1", "m", "n", "2", "tb1"];
        if !valid_prefixes.iter().any(|p| address.starts_with(p)) {
            return Err(BlockchainError::InvalidAddress(format!(
                "Invalid Bitcoin address prefix: {}",
                address
            )));
        }

        Ok(())
    }

    /// Create an inscription (Ordinal NFT)
    async fn create_inscription(&self, metadata: &NftMetadata, _to: &str) -> Result<String> {
        // In production, this would:
        // 1. Create the inscription content (JSON metadata)
        // 2. Build a commit transaction
        // 3. Build a reveal transaction
        // 4. Broadcast both transactions
        // 5. Return the inscription ID

        let inscription_content = serde_json::to_vec(metadata)
            .map_err(|e| BlockchainError::SerializationError(e.to_string()))?;

        // Generate a mock inscription ID (txid:index format)
        let txid = hex::encode(uuid::Uuid::new_v4().as_bytes());
        let inscription_id = format!("{}i0", txid);

        tracing::info!(
            "Created inscription {} ({} bytes)",
            inscription_id,
            inscription_content.len()
        );

        Ok(inscription_id)
    }

    /// Transfer an inscription to a new address
    async fn transfer_inscription(&self, inscription_id: &str, to: &str) -> Result<String> {
        // In production:
        // 1. Find the UTXO containing the inscription
        // 2. Create a transaction sending it to the new address
        // 3. Sign and broadcast

        tracing::info!("Transferring inscription {} to {}", inscription_id, to);

        let txid = hex::encode(uuid::Uuid::new_v4().as_bytes());
        Ok(txid)
    }
}

#[async_trait]
impl ChainProvider for BitcoinProvider {
    fn chain(&self) -> Chain {
        self.config.chain
    }

    async fn health_check(&self) -> Result<bool> {
        // In production: client.get_blockchain_info().is_ok()
        Ok(true)
    }

    async fn get_block_number(&self) -> Result<u64> {
        // In production: client.get_block_count()
        Ok(0)
    }

    async fn mint_nft(
        &self,
        to: &str,
        metadata: &NftMetadata,
        _asset: &AssetType,
    ) -> Result<MintResult> {
        tracing::info!("Minting Ordinal inscription to {} on {:?}", to, self.config.chain);

        Self::validate_address(to, self.config.network)?;

        let inscription_id = self.create_inscription(metadata, to).await?;

        // The "contract address" for Bitcoin is the inscription protocol identifier
        let contract_address = "ordinals".to_string();

        // For Bitcoin, metadata is embedded in the inscription itself
        let metadata_uri = format!("ord://{}", inscription_id);

        Ok(MintResult {
            chain: self.config.chain,
            token_id: inscription_id.clone(),
            transaction_hash: inscription_id.split('i').next().unwrap_or("").to_string(),
            contract_address,
            metadata_uri,
            minted_at: chrono::Utc::now(),
        })
    }

    async fn transfer_nft(&self, token_id: &str, from: &str, to: &str) -> Result<TransferResult> {
        tracing::info!(
            "Transferring inscription {} from {} to {} on {:?}",
            token_id,
            from,
            to,
            self.config.chain
        );

        Self::validate_address(from, self.config.network)?;
        Self::validate_address(to, self.config.network)?;

        let txid = self.transfer_inscription(token_id, to).await?;

        Ok(TransferResult {
            chain: self.config.chain,
            token_id: token_id.to_string(),
            transaction_hash: txid,
            from: from.to_string(),
            to: to.to_string(),
            transferred_at: chrono::Utc::now(),
        })
    }

    async fn get_nft_owner(&self, token_id: &str) -> Result<String> {
        tracing::debug!("Getting owner of inscription {} on Bitcoin", token_id);

        // In production:
        // 1. Parse the inscription ID to get txid:index
        // 2. Find the current UTXO containing this inscription
        // 3. Return the address of that UTXO

        Ok("unknown".to_string())
    }

    async fn verify_signature(&self, message: &str, signature: &str, address: &str) -> Result<bool> {
        tracing::debug!(
            "Verifying Bitcoin signature for address {}",
            address
        );

        Self::validate_address(address, self.config.network)?;

        // In production using bitcoin crate:
        // let sig = MessageSignature::from_base64(signature)?;
        // let secp = Secp256k1::verification_only();
        // sig.is_signed_by_address(&secp, address, message)?

        let _ = message;
        let _ = signature;

        // Bitcoin signatures are base64 encoded
        Ok(true)
    }

    async fn lock_for_bridge(&self, token_id: &str, owner: &str) -> Result<String> {
        tracing::info!(
            "Locking inscription {} for bridge on {:?}",
            token_id,
            self.config.chain
        );

        // For Bitcoin, locking means sending to a multisig or HTLC address
        // In production:
        // 1. Create a P2SH address with bridge parameters
        // 2. Transfer the inscription to that address

        let _ = owner;
        let lock_txid = hex::encode(uuid::Uuid::new_v4().as_bytes());

        Ok(lock_txid)
    }

    async fn unlock_from_bridge(&self, token_id: &str, owner: &str) -> Result<String> {
        tracing::info!(
            "Unlocking inscription {} from bridge on {:?}",
            token_id,
            self.config.chain
        );

        // Release from the locking script back to the original owner
        let _ = owner;
        let unlock_txid = hex::encode(uuid::Uuid::new_v4().as_bytes());

        Ok(unlock_txid)
    }
}

/// Spark L2 specific functionality
pub mod spark {
    use super::*;

    /// Spark-specific configuration extending Bitcoin config
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SparkConfig {
        pub base: BitcoinChainConfig,
        pub spark_rpc_url: String,
        pub state_channel_address: Option<String>,
    }

    impl Default for SparkConfig {
        fn default() -> Self {
            Self {
                base: BitcoinChainConfig {
                    chain: Chain::Spark,
                    ..Default::default()
                },
                spark_rpc_url: "http://localhost:9000".to_string(),
                state_channel_address: None,
            }
        }
    }

    /// Spark L2 provides faster, cheaper transactions for gaming
    pub struct SparkProvider {
        inner: BitcoinProvider,
        _spark_rpc: String,
    }

    impl SparkProvider {
        pub async fn new(config: SparkConfig) -> Result<Self> {
            let inner = BitcoinProvider::new(config.base).await?;
            Ok(Self {
                inner,
                _spark_rpc: config.spark_rpc_url,
            })
        }
    }

    #[async_trait]
    impl ChainProvider for SparkProvider {
        fn chain(&self) -> Chain {
            Chain::Spark
        }

        async fn health_check(&self) -> Result<bool> {
            // Check both Spark node and underlying Bitcoin
            self.inner.health_check().await
        }

        async fn get_block_number(&self) -> Result<u64> {
            // Spark has its own block height
            Ok(0)
        }

        async fn mint_nft(
            &self,
            to: &str,
            metadata: &NftMetadata,
            asset: &AssetType,
        ) -> Result<MintResult> {
            // Spark mints are done in state channels for speed
            // Final settlement happens on Bitcoin
            let mut result = self.inner.mint_nft(to, metadata, asset).await?;
            result.chain = Chain::Spark;
            Ok(result)
        }

        async fn transfer_nft(
            &self,
            token_id: &str,
            from: &str,
            to: &str,
        ) -> Result<TransferResult> {
            let mut result = self.inner.transfer_nft(token_id, from, to).await?;
            result.chain = Chain::Spark;
            Ok(result)
        }

        async fn get_nft_owner(&self, token_id: &str) -> Result<String> {
            self.inner.get_nft_owner(token_id).await
        }

        async fn verify_signature(
            &self,
            message: &str,
            signature: &str,
            address: &str,
        ) -> Result<bool> {
            self.inner.verify_signature(message, signature, address).await
        }

        async fn lock_for_bridge(&self, token_id: &str, owner: &str) -> Result<String> {
            self.inner.lock_for_bridge(token_id, owner).await
        }

        async fn unlock_from_bridge(&self, token_id: &str, owner: &str) -> Result<String> {
            self.inner.unlock_from_bridge(token_id, owner).await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bitcoin_provider_creation() {
        let config = BitcoinChainConfig::default();
        let provider = BitcoinProvider::new(config).await.unwrap();
        assert!(provider.chain().is_bitcoin());
    }

    #[test]
    fn test_address_validation() {
        // Valid mainnet addresses
        assert!(BitcoinProvider::validate_address("1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2", BitcoinNetwork::Mainnet).is_ok());
        assert!(BitcoinProvider::validate_address("bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq", BitcoinNetwork::Mainnet).is_ok());

        // Valid testnet addresses
        assert!(BitcoinProvider::validate_address("tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx", BitcoinNetwork::Testnet).is_ok());

        // Invalid addresses
        assert!(BitcoinProvider::validate_address("", BitcoinNetwork::Mainnet).is_err());
        assert!(BitcoinProvider::validate_address("invalid", BitcoinNetwork::Mainnet).is_err());
    }
}
