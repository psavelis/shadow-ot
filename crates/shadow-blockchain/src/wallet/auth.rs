//! Wallet Authentication
//!
//! Sign-in with Web3 wallets using message signing.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};

use crate::{Chain, Result, BlockchainError};

/// Challenge for wallet authentication (SIWE-style)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletAuthChallenge {
    pub id: Uuid,
    pub chain: Chain,
    pub address: String,
    pub nonce: String,
    pub message: String,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub uri: String,
    pub domain: String,
}

impl WalletAuthChallenge {
    /// Create a new authentication challenge
    pub fn new(chain: Chain, address: &str, domain: &str, uri: &str) -> Self {
        let nonce = Self::generate_nonce();
        let issued_at = Utc::now();
        let expires_at = issued_at + Duration::minutes(5);

        let message = Self::format_message(
            domain,
            &address,
            uri,
            &nonce,
            issued_at,
            chain,
        );

        Self {
            id: Uuid::new_v4(),
            chain,
            address: address.to_string(),
            nonce,
            message,
            issued_at,
            expires_at,
            uri: uri.to_string(),
            domain: domain.to_string(),
        }
    }

    /// Generate a cryptographically secure nonce
    fn generate_nonce() -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let bytes: [u8; 16] = rng.gen();
        hex::encode(bytes)
    }

    /// Format the message according to EIP-4361 (Sign-In with Ethereum)
    fn format_message(
        domain: &str,
        address: &str,
        uri: &str,
        nonce: &str,
        issued_at: DateTime<Utc>,
        chain: Chain,
    ) -> String {
        format!(
            "{domain} wants you to sign in with your {chain_name} account:\n\
            {address}\n\n\
            Welcome to Shadow OT! Sign this message to verify your wallet ownership.\n\n\
            URI: {uri}\n\
            Version: 1\n\
            Chain ID: {chain_id}\n\
            Nonce: {nonce}\n\
            Issued At: {issued_at}",
            domain = domain,
            chain_name = match chain {
                Chain::Starknet | Chain::StarknetGoerli | Chain::StarknetSepolia => "Starknet",
                Chain::Bitcoin | Chain::BitcoinTestnet | Chain::Spark => "Bitcoin",
                _ => "Ethereum",
            },
            address = address,
            uri = uri,
            chain_id = chain.chain_id(),
            nonce = nonce,
            issued_at = issued_at.format("%Y-%m-%dT%H:%M:%SZ"),
        )
    }

    /// Check if the challenge has expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}

/// Result of wallet authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletAuthResult {
    pub user_id: Option<Uuid>,
    pub address: String,
    pub chain: Chain,
    pub verified: bool,
    pub is_new_user: bool,
    pub session_token: Option<String>,
}

/// Wallet authentication service
pub struct WalletAuth {
    pending_challenges: std::sync::RwLock<std::collections::HashMap<Uuid, WalletAuthChallenge>>,
    domain: String,
    uri: String,
}

impl WalletAuth {
    pub fn new(domain: &str, uri: &str) -> Self {
        Self {
            pending_challenges: std::sync::RwLock::new(std::collections::HashMap::new()),
            domain: domain.to_string(),
            uri: uri.to_string(),
        }
    }

    /// Create a new authentication challenge
    pub fn create_challenge(&self, chain: Chain, address: &str) -> Result<WalletAuthChallenge> {
        let challenge = WalletAuthChallenge::new(chain, address, &self.domain, &self.uri);

        let mut challenges = self.pending_challenges.write()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

        // Clean up expired challenges
        challenges.retain(|_, c| !c.is_expired());

        // Store the new challenge
        challenges.insert(challenge.id, challenge.clone());

        tracing::info!(
            "Created auth challenge {} for {} on {:?}",
            challenge.id,
            address,
            chain
        );

        Ok(challenge)
    }

    /// Verify a signed challenge
    pub async fn verify_challenge(
        &self,
        challenge_id: Uuid,
        signature: &str,
        provider: &dyn crate::ChainProvider,
    ) -> Result<WalletAuthResult> {
        // Get the challenge
        let challenge = {
            let challenges = self.pending_challenges.read()
                .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

            challenges.get(&challenge_id).cloned()
                .ok_or(BlockchainError::InvalidSignature("Challenge not found".into()))?
        };

        // Check expiration
        if challenge.is_expired() {
            return Err(BlockchainError::InvalidSignature("Challenge expired".into()));
        }

        // Verify the signature
        let verified = provider
            .verify_signature(&challenge.message, signature, &challenge.address)
            .await?;

        if !verified {
            return Err(BlockchainError::InvalidSignature("Signature verification failed".into()));
        }

        // Remove the used challenge
        {
            let mut challenges = self.pending_challenges.write()
                .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;
            challenges.remove(&challenge_id);
        }

        tracing::info!(
            "Verified wallet auth for {} on {:?}",
            challenge.address,
            challenge.chain
        );

        Ok(WalletAuthResult {
            user_id: None, // Will be filled by the caller
            address: challenge.address,
            chain: challenge.chain,
            verified: true,
            is_new_user: false, // Will be determined by the caller
            session_token: None, // Will be generated by the caller
        })
    }

    /// Get a pending challenge
    pub fn get_challenge(&self, challenge_id: Uuid) -> Result<Option<WalletAuthChallenge>> {
        let challenges = self.pending_challenges.read()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

        Ok(challenges.get(&challenge_id).cloned())
    }

    /// Clean up expired challenges
    pub fn cleanup_expired(&self) -> Result<usize> {
        let mut challenges = self.pending_challenges.write()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

        let before = challenges.len();
        challenges.retain(|_, c| !c.is_expired());
        let removed = before - challenges.len();

        if removed > 0 {
            tracing::debug!("Cleaned up {} expired wallet auth challenges", removed);
        }

        Ok(removed)
    }
}
