//! Chain-specific provider implementations
//!
//! Each chain has its own provider implementing the ChainProvider trait.

pub mod evm;
pub mod starknet;
pub mod bitcoin;

pub use evm::EvmProvider;
pub use starknet::StarknetProvider;
pub use bitcoin::BitcoinProvider;

use crate::{Chain, ChainProvider, BlockchainConfig, Result, BlockchainError};
use std::collections::HashMap;

/// Create all configured chain providers
pub async fn create_providers(
    config: &BlockchainConfig,
) -> Result<HashMap<Chain, Box<dyn ChainProvider>>> {
    let mut providers: HashMap<Chain, Box<dyn ChainProvider>> = HashMap::new();

    // EVM chains
    for chain_config in &config.evm_chains {
        let chain = chain_config.chain;
        let provider = EvmProvider::new(chain_config.clone()).await?;
        providers.insert(chain, Box::new(provider));
    }

    // Starknet chains
    for chain_config in &config.starknet_chains {
        let chain = chain_config.chain;
        let provider = StarknetProvider::new(chain_config.clone()).await?;
        providers.insert(chain, Box::new(provider));
    }

    // Bitcoin-based chains
    for chain_config in &config.bitcoin_chains {
        let chain = chain_config.chain;
        let provider = BitcoinProvider::new(chain_config.clone()).await?;
        providers.insert(chain, Box::new(provider));
    }

    Ok(providers)
}
