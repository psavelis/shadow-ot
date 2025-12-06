//! Blockchain configuration

use serde::{Deserialize, Serialize};
use crate::Chain;
use crate::chains::evm::EvmChainConfig;
use crate::chains::starknet::StarknetChainConfig;
use crate::chains::bitcoin::BitcoinChainConfig;

/// Main blockchain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainConfig {
    pub enabled: bool,
    pub primary_chain: Chain,
    pub enabled_chains: Vec<Chain>,
    /// EVM chain configurations
    pub evm_chains: Vec<EvmChainConfig>,
    /// Starknet chain configurations
    pub starknet_chains: Vec<StarknetChainConfig>,
    /// Bitcoin chain configurations
    pub bitcoin_chains: Vec<BitcoinChainConfig>,
    pub ethereum: Option<EvmConfig>,
    pub polygon: Option<EvmConfig>,
    pub starknet: Option<StarknetConfig>,
    pub bitcoin: Option<BitcoinConfig>,
    pub ipfs: IpfsConfig,
    pub bridge: BridgeConfig,
    pub contracts: ContractsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvmConfig {
    pub rpc_url: String,
    pub ws_url: Option<String>,
    pub chain_id: u64,
    pub private_key: Option<String>,
    pub gas_price_multiplier: f64,
    pub confirmation_blocks: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarknetConfig {
    pub rpc_url: String,
    pub chain_id: String,
    pub account_address: Option<String>,
    pub private_key: Option<String>,
    pub max_fee_multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitcoinConfig {
    pub rpc_url: String,
    pub rpc_user: String,
    pub rpc_password: String,
    pub network: String, // mainnet, testnet, regtest
    pub wallet_name: Option<String>,
    pub ordinals_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpfsConfig {
    pub gateway_url: String,
    pub api_url: String,
    pub pin_service: Option<PinServiceConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinServiceConfig {
    pub provider: String, // pinata, infura, web3.storage
    pub api_key: String,
    pub api_secret: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeConfig {
    pub enabled: bool,
    pub relayer_url: Option<String>,
    pub fee_percentage: f64,
    pub min_confirmations: u64,
    pub supported_routes: Vec<BridgeRoute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeRoute {
    pub source: Chain,
    pub target: Chain,
    pub enabled: bool,
    pub fee_override: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractsConfig {
    pub ethereum_nft: Option<String>,
    pub polygon_nft: Option<String>,
    pub starknet_nft: Option<String>,
    pub ethereum_bridge: Option<String>,
    pub polygon_bridge: Option<String>,
    pub starknet_bridge: Option<String>,
    pub ethereum_marketplace: Option<String>,
    pub polygon_marketplace: Option<String>,
    pub starknet_marketplace: Option<String>,
}

impl Default for BlockchainConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            primary_chain: Chain::Polygon,
            enabled_chains: vec![Chain::Polygon, Chain::Starknet],
            ethereum: None,
            polygon: Some(EvmConfig {
                rpc_url: "https://polygon-rpc.com".to_string(),
                ws_url: None,
                chain_id: 137,
                private_key: None,
                gas_price_multiplier: 1.2,
                confirmation_blocks: 5,
            }),
            starknet: Some(StarknetConfig {
                rpc_url: "https://starknet-mainnet.public.blastapi.io".to_string(),
                chain_id: "SN_MAIN".to_string(),
                account_address: None,
                private_key: None,
                max_fee_multiplier: 1.5,
            }),
            bitcoin: None,
            evm_chains: vec![],
            starknet_chains: vec![],
            bitcoin_chains: vec![],
            ipfs: IpfsConfig {
                gateway_url: "https://ipfs.io/ipfs/".to_string(),
                api_url: "https://api.pinata.cloud".to_string(),
                pin_service: None,
            },
            bridge: BridgeConfig {
                enabled: true,
                relayer_url: None,
                fee_percentage: 0.5,
                min_confirmations: 12,
                supported_routes: vec![
                    BridgeRoute {
                        source: Chain::Ethereum,
                        target: Chain::Polygon,
                        enabled: true,
                        fee_override: None,
                    },
                    BridgeRoute {
                        source: Chain::Polygon,
                        target: Chain::Starknet,
                        enabled: true,
                        fee_override: None,
                    },
                    BridgeRoute {
                        source: Chain::Starknet,
                        target: Chain::Polygon,
                        enabled: true,
                        fee_override: None,
                    },
                ],
            },
            contracts: ContractsConfig {
                ethereum_nft: None,
                polygon_nft: None,
                starknet_nft: None,
                ethereum_bridge: None,
                polygon_bridge: None,
                starknet_bridge: None,
                ethereum_marketplace: None,
                polygon_marketplace: None,
                starknet_marketplace: None,
            },
        }
    }
}
