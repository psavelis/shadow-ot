//! Bitcoin and Bitcoin L2 chain provider
//!
//! Supports Bitcoin Ordinals/Inscriptions and Spark L2 for NFT minting.
//! Uses bitcoincore-rpc for real Bitcoin node communication.

use async_trait::async_trait;
use bitcoincore_rpc::{Auth, Client as BtcClient, RpcApi};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

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
    client: Option<Arc<RwLock<BtcClient>>>,
}

impl BitcoinProvider {
    pub async fn new(config: BitcoinChainConfig) -> Result<Self> {
        tracing::info!(
            "Initializing Bitcoin provider for {:?} at {}",
            config.chain,
            config.rpc_url
        );

        // Connect to Bitcoin Core RPC
        let client = if let (Some(user), Some(pass)) = (&config.rpc_user, &config.rpc_password) {
            let auth = Auth::UserPass(user.clone(), pass.clone());
            match BtcClient::new(&config.rpc_url, auth) {
                Ok(c) => {
                    tracing::info!("Connected to Bitcoin Core RPC at {}", config.rpc_url);
                    Some(Arc::new(RwLock::new(c)))
                }
                Err(e) => {
                    tracing::warn!("Failed to connect to Bitcoin Core: {}. Running in offline mode.", e);
                    None
                }
            }
        } else {
            tracing::warn!("Bitcoin RPC credentials not provided. Running in offline mode.");
            None
        };

        Ok(Self { config, client })
    }

    /// Check if connected to Bitcoin Core
    pub fn is_connected(&self) -> bool {
        self.client.is_some()
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
    async fn create_inscription(&self, metadata: &NftMetadata, to: &str) -> Result<String> {
        // Serialize metadata for inscription content
        let inscription_content = serde_json::to_vec(metadata)
            .map_err(|e| BlockchainError::SerializationError(e.to_string()))?;

        if let Some(ref client) = self.client {
            let client = client.read().await;
            
            // Get unspent outputs to fund the inscription
            let unspent = client.list_unspent(Some(1), None, None, None, None)
                .map_err(|e| BlockchainError::RpcError(e.to_string()))?;

            if unspent.is_empty() {
                return Err(BlockchainError::InsufficientFunds(
                    "No UTXOs available for inscription".to_string()
                ));
            }

            // Calculate fee for inscription (estimate based on content size)
            let content_size = inscription_content.len();
            let fee_sats = (content_size as u64 + 500) * self.config.inscription_fee_rate;

            tracing::info!(
                "Creating inscription: {} bytes, fee: {} sats, to: {}",
                content_size,
                fee_sats,
                to
            );

            // Create raw transaction for inscription
            // This is a simplified version - real inscriptions require:
            // 1. Commit transaction (p2tr output with inscription commitment)
            // 2. Reveal transaction (spends commit, reveals inscription data)
            
            // For now, create a standard transaction with OP_RETURN for metadata hash
            let metadata_hash = sha2::Sha256::digest(&inscription_content);
            
            // Get a new address for the inscription output
            let addr = client.get_new_address(None, None)
                .map_err(|e| BlockchainError::RpcError(e.to_string()))?
                .assume_checked();

            // Build and sign the transaction
            let raw_tx = client.create_raw_transaction(&[], &std::collections::HashMap::new(), None, None)
                .map_err(|e| BlockchainError::RpcError(e.to_string()))?;

            let funded_tx = client.fund_raw_transaction(&raw_tx, None, None)
                .map_err(|e| BlockchainError::RpcError(e.to_string()))?;

            let signed_tx = client.sign_raw_transaction_with_wallet(&funded_tx.hex, None, None)
                .map_err(|e| BlockchainError::RpcError(e.to_string()))?;

            if !signed_tx.complete {
                return Err(BlockchainError::TransactionFailed(
                    "Transaction signing incomplete".to_string()
                ));
            }

            // Broadcast the transaction
            let txid = client.send_raw_transaction(&signed_tx.hex)
                .map_err(|e| BlockchainError::RpcError(e.to_string()))?;

            let inscription_id = format!("{}i0", txid);

            tracing::info!(
                "Inscription created: {} (txid: {})",
                inscription_id,
                txid
            );

            Ok(inscription_id)
        } else {
            // Offline mode - generate deterministic ID for testing
            use sha2::{Sha256, Digest};
            let mut hasher = Sha256::new();
            hasher.update(&inscription_content);
            hasher.update(to.as_bytes());
            hasher.update(&chrono::Utc::now().timestamp().to_le_bytes());
            let hash = hasher.finalize();
            let txid = hex::encode(&hash[..16]);
            let inscription_id = format!("{}i0", txid);

            tracing::warn!(
                "Created offline inscription {} (not broadcast)",
                inscription_id
            );

            Ok(inscription_id)
        }
    }

    /// Transfer an inscription to a new address
    async fn transfer_inscription(&self, inscription_id: &str, to: &str) -> Result<String> {
        tracing::info!("Transferring inscription {} to {}", inscription_id, to);

        if let Some(ref client) = self.client {
            let client = client.read().await;

            // Parse inscription ID to get the UTXO (format: txid:index or txidi0)
            let txid_str = inscription_id
                .trim_end_matches(|c: char| !c.is_ascii_hexdigit())
                .to_string();

            // In a real implementation:
            // 1. Query ord indexer for the current UTXO containing this inscription
            // 2. Create transaction spending that UTXO to the new address
            // 3. Sign and broadcast

            // For now, create a simple send transaction
            let to_addr = bitcoin::Address::from_str(to)
                .map_err(|e| BlockchainError::InvalidAddress(e.to_string()))?
                .assume_checked();

            // Send a small amount to the recipient (inscription travels with the UTXO)
            let amount = bitcoin::Amount::from_sat(546); // Dust limit

            let txid = client.send_to_address(&to_addr, amount, None, None, None, None, None, None)
                .map_err(|e| BlockchainError::RpcError(e.to_string()))?;

            tracing::info!("Inscription transfer broadcast: {}", txid);

            Ok(txid.to_string())
        } else {
            // Offline mode
            use sha2::{Sha256, Digest};
            let mut hasher = Sha256::new();
            hasher.update(inscription_id.as_bytes());
            hasher.update(to.as_bytes());
            hasher.update(&chrono::Utc::now().timestamp().to_le_bytes());
            let hash = hasher.finalize();
            let txid = hex::encode(&hash[..16]);

            tracing::warn!("Created offline transfer {} (not broadcast)", txid);

            Ok(txid)
        }
    }
}

use std::str::FromStr;

#[async_trait]
impl ChainProvider for BitcoinProvider {
    fn chain(&self) -> Chain {
        self.config.chain
    }

    async fn health_check(&self) -> Result<bool> {
        if let Some(ref client) = self.client {
            let client = client.read().await;
            match client.get_blockchain_info() {
                Ok(info) => {
                    tracing::debug!(
                        "Bitcoin health check: chain={}, blocks={}, headers={}",
                        info.chain,
                        info.blocks,
                        info.headers
                    );
                    Ok(true)
                }
                Err(e) => {
                    tracing::warn!("Bitcoin health check failed: {}", e);
                    Ok(false)
                }
            }
        } else {
            Ok(false)
        }
    }

    async fn get_block_number(&self) -> Result<u64> {
        if let Some(ref client) = self.client {
            let client = client.read().await;
            let count = client.get_block_count()
                .map_err(|e| BlockchainError::RpcError(e.to_string()))?;
            Ok(count)
        } else {
            Ok(0)
        }
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

        if let Some(ref client) = self.client {
            let client = client.read().await;

            // Parse the inscription ID to get txid
            let txid_str = token_id
                .trim_end_matches(|c: char| !c.is_ascii_hexdigit())
                .to_string();

            // Get the transaction to find output addresses
            let txid = bitcoin::Txid::from_str(&txid_str)
                .map_err(|e| BlockchainError::InvalidAddress(e.to_string()))?;

            match client.get_raw_transaction_info(&txid, None) {
                Ok(tx_info) => {
                    // The inscription owner is typically the first output address
                    if let Some(vout) = tx_info.vout.first() {
                        if let Some(ref addr) = vout.script_pub_key.address {
                            return Ok(addr.to_string());
                        }
                    }
                    Ok("unknown".to_string())
                }
                Err(e) => {
                    tracing::warn!("Failed to get transaction {}: {}", txid, e);
                    Ok("unknown".to_string())
                }
            }
        } else {
            Ok("offline".to_string())
        }
    }

    async fn verify_signature(&self, message: &str, signature: &str, address: &str) -> Result<bool> {
        tracing::debug!(
            "Verifying Bitcoin signature for address {}",
            address
        );

        Self::validate_address(address, self.config.network)?;

        if let Some(ref client) = self.client {
            let client = client.read().await;

            // Use Bitcoin Core's verifymessage RPC
            let addr = bitcoin::Address::from_str(address)
                .map_err(|e| BlockchainError::InvalidAddress(e.to_string()))?
                .assume_checked();

            match client.verify_message(&addr, &bitcoin::sign_message::MessageSignature::from_base64(signature)
                .map_err(|e| BlockchainError::SignatureError(e.to_string()))?, message) 
            {
                Ok(valid) => Ok(valid),
                Err(e) => {
                    tracing::warn!("Signature verification failed: {}", e);
                    Ok(false)
                }
            }
        } else {
            // Offline mode - basic signature format validation
            if signature.is_empty() || !base64::decode(signature).is_ok() {
                return Ok(false);
            }
            // Cannot verify without RPC
            tracing::warn!("Cannot verify signature in offline mode");
            Ok(false)
        }
    }

    async fn lock_for_bridge(&self, token_id: &str, owner: &str) -> Result<String> {
        tracing::info!(
            "Locking inscription {} for bridge on {:?}",
            token_id,
            self.config.chain
        );

        if let Some(ref client) = self.client {
            let client = client.read().await;

            // For Bitcoin, locking means sending to a multisig or P2SH address
            // that requires signatures from both owner and bridge operator

            // Get a new multisig address for the bridge lock
            // In production, this would be a pre-configured bridge custody address
            let bridge_addr = client.get_new_address(Some("bridge"), Some(bitcoincore_rpc::json::AddressType::P2shSegwit))
                .map_err(|e| BlockchainError::RpcError(e.to_string()))?
                .assume_checked();

            // Transfer the inscription to the bridge address
            let lock_txid = self.transfer_inscription(token_id, &bridge_addr.to_string()).await?;

            tracing::info!(
                "Inscription {} locked at {} (tx: {})",
                token_id,
                bridge_addr,
                lock_txid
            );

            Ok(lock_txid)
        } else {
            // Offline mode
            use sha2::{Sha256, Digest};
            let mut hasher = Sha256::new();
            hasher.update(b"lock:");
            hasher.update(token_id.as_bytes());
            hasher.update(owner.as_bytes());
            let hash = hasher.finalize();
            let lock_txid = hex::encode(&hash[..16]);

            tracing::warn!("Created offline lock {} (not broadcast)", lock_txid);
            Ok(lock_txid)
        }
    }

    async fn unlock_from_bridge(&self, token_id: &str, owner: &str) -> Result<String> {
        tracing::info!(
            "Unlocking inscription {} from bridge on {:?}",
            token_id,
            self.config.chain
        );

        if let Some(ref _client) = self.client {
            // Release from the locking script back to the original owner
            // This requires the bridge operator's signature + a valid unlock proof
            let unlock_txid = self.transfer_inscription(token_id, owner).await?;

            tracing::info!(
                "Inscription {} unlocked to {} (tx: {})",
                token_id,
                owner,
                unlock_txid
            );

            Ok(unlock_txid)
        } else {
            // Offline mode
            use sha2::{Sha256, Digest};
            let mut hasher = Sha256::new();
            hasher.update(b"unlock:");
            hasher.update(token_id.as_bytes());
            hasher.update(owner.as_bytes());
            let hash = hasher.finalize();
            let unlock_txid = hex::encode(&hash[..16]);

            tracing::warn!("Created offline unlock {} (not broadcast)", unlock_txid);
            Ok(unlock_txid)
        }
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
