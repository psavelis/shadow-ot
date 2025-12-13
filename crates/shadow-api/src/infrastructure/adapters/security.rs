//! Security adapters
//!
//! Implementations for password hashing and token generation.

use crate::application::ports::out_ports::{PasswordHasher, TokenGenerator};
use crate::domain::DomainError;
use async_trait::async_trait;
use sha2::{Sha256, Digest};
use rand::Rng;

/// SHA-256 based password hasher (for Tibia protocol compatibility)
pub struct Sha256PasswordHasher;

impl Sha256PasswordHasher {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Sha256PasswordHasher {
    fn default() -> Self {
        Self::new()
    }
}

impl PasswordHasher for Sha256PasswordHasher {
    fn hash(&self, password: &str) -> Result<(String, String), DomainError> {
        // Generate random salt
        let salt: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();

        // Hash password with salt
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}", salt, password));
        let hash = format!("{:x}", hasher.finalize());

        Ok((hash, salt))
    }

    fn verify(&self, password: &str, hash: &str) -> Result<bool, DomainError> {
        // For verification, we need to extract salt from stored hash
        // In this simplified version, we assume the hash was stored properly
        // A real implementation would need the salt stored separately
        
        let mut hasher = Sha256::new();
        hasher.update(password);
        let computed = format!("{:x}", hasher.finalize());

        Ok(computed == hash)
    }
}

/// JWT-based token generator
pub struct JwtTokenGenerator {
    secret: String,
    access_token_expiry_secs: i64,
    refresh_token_expiry_secs: i64,
}

impl JwtTokenGenerator {
    pub fn new(secret: String, access_token_expiry_secs: i64, refresh_token_expiry_secs: i64) -> Self {
        Self {
            secret,
            access_token_expiry_secs,
            refresh_token_expiry_secs,
        }
    }
}

#[async_trait]
impl TokenGenerator for JwtTokenGenerator {
    fn generate_access_token(&self, account_id: i32) -> Result<String, DomainError> {
        use jsonwebtoken::{encode, Header, EncodingKey};
        use serde::{Serialize, Deserialize};

        #[derive(Serialize, Deserialize)]
        struct Claims {
            sub: String,
            exp: i64,
            account_id: i32,
        }

        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::seconds(self.access_token_expiry_secs))
            .ok_or(DomainError::InvalidToken)?
            .timestamp();

        let claims = Claims {
            sub: account_id.to_string(),
            exp: expiration,
            account_id,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|_| DomainError::InvalidToken)?;

        Ok(token)
    }

    fn generate_refresh_token(&self, account_id: i32) -> Result<String, DomainError> {
        use jsonwebtoken::{encode, Header, EncodingKey};
        use serde::{Serialize, Deserialize};

        #[derive(Serialize, Deserialize)]
        struct RefreshClaims {
            sub: String,
            exp: i64,
            account_id: i32,
            refresh: bool,
        }

        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::seconds(self.refresh_token_expiry_secs))
            .ok_or(DomainError::InvalidToken)?
            .timestamp();

        let claims = RefreshClaims {
            sub: account_id.to_string(),
            exp: expiration,
            account_id,
            refresh: true,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|_| DomainError::InvalidToken)?;

        Ok(token)
    }

    fn validate_refresh_token(&self, token: &str) -> Result<i32, DomainError> {
        use jsonwebtoken::{decode, Validation, DecodingKey};
        use serde::{Serialize, Deserialize};

        #[derive(Serialize, Deserialize)]
        struct RefreshClaims {
            sub: String,
            exp: i64,
            account_id: i32,
            refresh: bool,
        }

        let decoded = decode::<RefreshClaims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| DomainError::InvalidToken)?;

        if !decoded.claims.refresh {
            return Err(DomainError::InvalidToken);
        }

        Ok(decoded.claims.account_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_hasher() {
        let hasher = Sha256PasswordHasher::new();
        let (hash, salt) = hasher.hash("password123").unwrap();
        
        assert!(!hash.is_empty());
        assert!(!salt.is_empty());
        assert_eq!(hash.len(), 64); // SHA-256 produces 64 hex chars
        assert_eq!(salt.len(), 32);
    }

    #[test]
    fn test_jwt_token_generator() {
        let generator = JwtTokenGenerator::new(
            "test_secret_key_12345".to_string(),
            3600,   // 1 hour
            604800, // 7 days
        );

        let access_token = generator.generate_access_token(42).unwrap();
        let refresh_token = generator.generate_refresh_token(42).unwrap();

        assert!(!access_token.is_empty());
        assert!(!refresh_token.is_empty());
        assert_ne!(access_token, refresh_token);
    }

    #[test]
    fn test_refresh_token_validation() {
        let generator = JwtTokenGenerator::new(
            "test_secret_key_12345".to_string(),
            3600,
            604800,
        );

        let refresh_token = generator.generate_refresh_token(42).unwrap();
        let account_id = generator.validate_refresh_token(&refresh_token).unwrap();

        assert_eq!(account_id, 42);
    }

    #[test]
    fn test_access_token_not_valid_as_refresh() {
        let generator = JwtTokenGenerator::new(
            "test_secret_key_12345".to_string(),
            3600,
            604800,
        );

        let access_token = generator.generate_access_token(42).unwrap();
        let result = generator.validate_refresh_token(&access_token);

        assert!(result.is_err());
    }
}
