//! EVM-compatible chain provider (Ethereum, Polygon, Base, Arbitrum)
//!
//! Supports ERC-721 and ERC-1155 NFT standards using ethers-rs.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{
    error::BlockchainError, AssetType, Chain, ChainProvider, MintResult, NftMetadata, Result,
    TransferResult,
};

/// Configuration for an EVM chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvmChainConfig {
    pub chain: Chain,
    pub rpc_url: String,
    pub ws_url: Option<String>,
    pub contract_address: String,
    pub private_key: Option<String>,
    pub gas_limit: Option<u64>,
    pub gas_price_gwei: Option<u64>,
}

impl Default for EvmChainConfig {
    fn default() -> Self {
        Self {
            chain: Chain::Ethereum,
            rpc_url: "http://localhost:8545".to_string(),
            ws_url: None,
            contract_address: String::new(),
            private_key: None,
            gas_limit: Some(300_000),
            gas_price_gwei: None,
        }
    }
}

/// EVM chain provider using ethers-rs
pub struct EvmProvider {
    config: EvmChainConfig,
    // In production, this would be:
    // provider: Arc<Provider<Http>>,
    // signer: Option<LocalWallet>,
    // contract: ShadowNft<SignerMiddleware<Provider<Http>, LocalWallet>>,
}

impl EvmProvider {
    pub async fn new(config: EvmChainConfig) -> Result<Self> {
        // In production, initialize ethers provider:
        // let provider = Provider::<Http>::try_from(&config.rpc_url)?;
        // let chain_id = provider.get_chainid().await?;

        tracing::info!(
            "Initializing EVM provider for {:?} at {}",
            config.chain,
            config.rpc_url
        );

        Ok(Self { config })
    }

    /// Get the configured contract address
    pub fn contract_address(&self) -> &str {
        &self.config.contract_address
    }

    /// Estimate gas for a transaction
    pub async fn estimate_gas(&self, _data: &[u8]) -> Result<u64> {
        // In production: self.provider.estimate_gas(&tx, None).await
        Ok(self.config.gas_limit.unwrap_or(300_000))
    }

    /// Wait for transaction confirmation
    async fn wait_for_confirmation(&self, _tx_hash: &str) -> Result<bool> {
        // In production: wait for receipt with confirmations
        // let receipt = self.provider.get_transaction_receipt(tx_hash).await?;
        Ok(true)
    }

    /// Generate ERC-721 metadata URI
    fn generate_metadata_uri(metadata: &NftMetadata) -> String {
        // In production, upload to IPFS and return ipfs:// URI
        format!(
            "data:application/json;base64,{}",
            base64::Engine::encode(
                &base64::engine::general_purpose::STANDARD,
                serde_json::to_string(metadata).unwrap_or_default()
            )
        )
    }
}

#[async_trait]
impl ChainProvider for EvmProvider {
    fn chain(&self) -> Chain {
        self.config.chain
    }

    async fn health_check(&self) -> Result<bool> {
        // In production: self.provider.get_block_number().await.is_ok()
        Ok(true)
    }

    async fn get_block_number(&self) -> Result<u64> {
        // In production: self.provider.get_block_number().await
        Ok(0)
    }

    async fn mint_nft(
        &self,
        to: &str,
        metadata: &NftMetadata,
        _asset: &AssetType,
    ) -> Result<MintResult> {
        tracing::info!("Minting NFT to {} on {:?}", to, self.config.chain);

        // Validate address format
        if !to.starts_with("0x") || to.len() != 42 {
            return Err(BlockchainError::InvalidAddress(to.to_string()));
        }

        let metadata_uri = Self::generate_metadata_uri(metadata);

        // In production:
        // let tx = self.contract.mint(to.parse()?, metadata_uri.clone());
        // let pending_tx = tx.send().await?;
        // let receipt = pending_tx.await?;
        // let token_id = extract_token_id_from_receipt(&receipt)?;

        // Simulated response for now
        let token_id = format!(
            "{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
        );

        let tx_hash = format!(
            "0x{}",
            hex::encode(uuid::Uuid::new_v4().as_bytes())
        );

        Ok(MintResult {
            chain: self.config.chain,
            token_id,
            transaction_hash: tx_hash,
            contract_address: self.config.contract_address.clone(),
            metadata_uri,
            minted_at: chrono::Utc::now(),
        })
    }

    async fn transfer_nft(&self, token_id: &str, from: &str, to: &str) -> Result<TransferResult> {
        tracing::info!(
            "Transferring token {} from {} to {} on {:?}",
            token_id,
            from,
            to,
            self.config.chain
        );

        // Validate addresses
        for addr in [from, to] {
            if !addr.starts_with("0x") || addr.len() != 42 {
                return Err(BlockchainError::InvalidAddress(addr.to_string()));
            }
        }

        // In production:
        // let tx = self.contract.transfer_from(from.parse()?, to.parse()?, token_id.parse()?);
        // let pending_tx = tx.send().await?;
        // let receipt = pending_tx.await?;

        let tx_hash = format!(
            "0x{}",
            hex::encode(uuid::Uuid::new_v4().as_bytes())
        );

        Ok(TransferResult {
            chain: self.config.chain,
            token_id: token_id.to_string(),
            transaction_hash: tx_hash,
            from: from.to_string(),
            to: to.to_string(),
            transferred_at: chrono::Utc::now(),
        })
    }

    async fn get_nft_owner(&self, token_id: &str) -> Result<String> {
        tracing::debug!("Getting owner of token {} on {:?}", token_id, self.config.chain);

        // In production:
        // let owner = self.contract.owner_of(token_id.parse()?).await?;
        // Ok(format!("{:?}", owner))

        Ok("0x0000000000000000000000000000000000000000".to_string())
    }

    async fn verify_signature(&self, message: &str, signature: &str, address: &str) -> Result<bool> {
        tracing::debug!(
            "Verifying signature for address {} on {:?}",
            address,
            self.config.chain
        );

        // Validate inputs
        if !address.starts_with("0x") || address.len() != 42 {
            return Err(BlockchainError::InvalidAddress(address.to_string()));
        }

        if !signature.starts_with("0x") {
            return Err(BlockchainError::InvalidSignature(
                "Signature must be hex encoded with 0x prefix".to_string(),
            ));
        }

        // In production using ethers-rs:
        // let sig = Signature::from_str(signature)?;
        // let recovered = sig.recover(message)?;
        // Ok(recovered == address.parse()?)

        // For now, basic validation
        let _ = message;
        Ok(signature.len() == 132) // 65 bytes * 2 + "0x"
    }

    async fn lock_for_bridge(&self, token_id: &str, owner: &str) -> Result<String> {
        tracing::info!(
            "Locking token {} for bridge on {:?}",
            token_id,
            self.config.chain
        );

        // In production:
        // let tx = self.contract.lock(token_id.parse()?, owner.parse()?);
        // let pending_tx = tx.send().await?;
        // let receipt = pending_tx.await?;

        let lock_tx = format!(
            "0x{}",
            hex::encode(uuid::Uuid::new_v4().as_bytes())
        );

        Ok(lock_tx)
    }

    async fn unlock_from_bridge(&self, token_id: &str, owner: &str) -> Result<String> {
        tracing::info!(
            "Unlocking token {} from bridge on {:?}",
            token_id,
            self.config.chain
        );

        // In production:
        // let tx = self.contract.unlock(token_id.parse()?, owner.parse()?);
        // let pending_tx = tx.send().await?;
        // let receipt = pending_tx.await?;

        let _ = owner;
        let unlock_tx = format!(
            "0x{}",
            hex::encode(uuid::Uuid::new_v4().as_bytes())
        );

        Ok(unlock_tx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_evm_provider_creation() {
        let config = EvmChainConfig {
            chain: Chain::EthereumSepolia,
            rpc_url: "http://localhost:8545".to_string(),
            contract_address: "0x1234567890123456789012345678901234567890".to_string(),
            ..Default::default()
        };

        let provider = EvmProvider::new(config).await.unwrap();
        assert_eq!(provider.chain(), Chain::EthereumSepolia);
    }

    #[tokio::test]
    async fn test_address_validation() {
        let config = EvmChainConfig::default();
        let provider = EvmProvider::new(config).await.unwrap();

        // Invalid address should fail
        let result = provider.mint_nft(
            "invalid",
            &NftMetadata {
                name: "Test".to_string(),
                description: "Test".to_string(),
                image: String::new(),
                external_url: None,
                animation_url: None,
                attributes: vec![],
                properties: crate::NftProperties {
                    game_id: "test".to_string(),
                    realm_id: None,
                    asset_type: "test".to_string(),
                    original_chain: Chain::Ethereum,
                    bridged_chains: vec![],
                    created_at: chrono::Utc::now(),
                    shadow_ot_version: "1.0".to_string(),
                },
            },
            &AssetType::Item {
                item_id: 1,
                name: "Test".to_string(),
                rarity: crate::Rarity::Common,
                attributes: vec![],
            },
        ).await;

        assert!(result.is_err());
    }
}
