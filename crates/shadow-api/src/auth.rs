//! Authentication utilities - JWT, password hashing, session management, HWID, 2FA

use crate::error::ApiError;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sha2::{Sha256, Digest};
use hmac::{Hmac, Mac};

/// Authentication configuration
#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_expiry_hours: i64,
    pub refresh_expiry_days: i64,
    pub session_timeout_minutes: i64,
    pub totp_issuer: String,
    pub hwid_validation_enabled: bool,
    pub max_hwid_per_account: usize,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            jwt_secret: "change-me-in-production-use-strong-secret".to_string(),
            jwt_expiry_hours: 24,
            refresh_expiry_days: 30,
            session_timeout_minutes: 60,
            totp_issuer: "ShadowOT".to_string(),
            hwid_validation_enabled: true,
            max_hwid_per_account: 3,
        }
    }
}

/// JWT Claims
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,           // Account UUID
    pub account_id: i32,
    pub email: String,
    pub account_type: String,
    pub exp: i64,
    pub iat: i64,
    pub jti: String,           // Token ID
}

impl JwtClaims {
    pub fn new(account_id: i32, account_uuid: &Uuid, email: &str, account_type: &str, expiry_hours: i64) -> Self {
        let now = Utc::now();
        Self {
            sub: account_uuid.to_string(),
            account_id,
            email: email.to_string(),
            account_type: account_type.to_string(),
            exp: (now + Duration::hours(expiry_hours)).timestamp(),
            iat: now.timestamp(),
            jti: Uuid::new_v4().to_string(),
        }
    }

    pub fn is_admin(&self) -> bool {
        self.account_type == "admin" || self.account_type == "gamemaster"
    }
}

/// Create JWT token
pub fn create_token(claims: &JwtClaims, secret: &str) -> Result<String, ApiError> {
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| ApiError::Internal)
}

/// Validate JWT token
pub fn validate_token(token: &str, secret: &str) -> Result<JwtClaims, ApiError> {
    decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| e.into())
}

/// Refresh token claims
#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: String,
    pub account_id: i32,
    pub exp: i64,
    pub iat: i64,
    pub jti: String,
}

impl RefreshClaims {
    pub fn new(account_id: i32, account_uuid: &Uuid, expiry_days: i64) -> Self {
        let now = Utc::now();
        Self {
            sub: account_uuid.to_string(),
            account_id,
            exp: (now + Duration::days(expiry_days)).timestamp(),
            iat: now.timestamp(),
            jti: Uuid::new_v4().to_string(),
        }
    }
}

/// Create refresh token
pub fn create_refresh_token(claims: &RefreshClaims, secret: &str) -> Result<String, ApiError> {
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| ApiError::Internal)
}

/// Validate refresh token
pub fn validate_refresh_token(token: &str, secret: &str) -> Result<RefreshClaims, ApiError> {
    decode::<RefreshClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| e.into())
}

/// Hash password using Argon2
pub fn hash_password(password: &str) -> Result<(String, String), ApiError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| ApiError::Internal)?
        .to_string();

    Ok((hash, salt.to_string()))
}

/// Verify password against hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, ApiError> {
    let parsed_hash = PasswordHash::new(hash).map_err(|_| ApiError::Internal)?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

/// Generate random token (for email verification, password reset, etc.)
pub fn generate_token() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let token: [u8; 32] = rng.gen();
    hex::encode(token)
}

/// Password strength validation
pub fn validate_password_strength(password: &str) -> Result<(), ApiError> {
    if password.len() < 8 {
        return Err(ApiError::Validation("Password must be at least 8 characters".to_string()));
    }

    if password.len() > 128 {
        return Err(ApiError::Validation("Password must be at most 128 characters".to_string()));
    }

    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_numeric());

    if !has_uppercase || !has_lowercase || !has_digit {
        return Err(ApiError::Validation(
            "Password must contain uppercase, lowercase, and numeric characters".to_string(),
        ));
    }

    Ok(())
}

/// Email validation
pub fn validate_email(email: &str) -> Result<(), ApiError> {
    if email.len() < 5 || email.len() > 255 {
        return Err(ApiError::Validation("Invalid email length".to_string()));
    }

    if !email.contains('@') || !email.contains('.') {
        return Err(ApiError::Validation("Invalid email format".to_string()));
    }

    Ok(())
}

/// Character name validation
pub fn validate_character_name(name: &str) -> Result<(), ApiError> {
    if name.len() < 2 || name.len() > 32 {
        return Err(ApiError::Validation("Name must be 2-32 characters".to_string()));
    }

    if !name.chars().all(|c| c.is_alphabetic() || c == ' ' || c == '\'') {
        return Err(ApiError::Validation("Name contains invalid characters".to_string()));
    }

    // No consecutive spaces
    if name.contains("  ") {
        return Err(ApiError::Validation("Name cannot have consecutive spaces".to_string()));
    }

    // Must start with letter
    if !name.chars().next().map(|c| c.is_alphabetic()).unwrap_or(false) {
        return Err(ApiError::Validation("Name must start with a letter".to_string()));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let password = "TestPassword123";
        let (hash, _salt) = hash_password(password).unwrap();

        assert!(verify_password(password, &hash).unwrap());
        assert!(!verify_password("wrong", &hash).unwrap());
    }

    #[test]
    fn test_jwt_token() {
        let uuid = Uuid::new_v4();
        let claims = JwtClaims::new(1, &uuid, "test@example.com", "normal", 24);
        let secret = "test-secret";

        let token = create_token(&claims, secret).unwrap();
        let decoded = validate_token(&token, secret).unwrap();

        assert_eq!(decoded.account_id, 1);
        assert_eq!(decoded.email, "test@example.com");
    }

    #[test]
    fn test_password_validation() {
        assert!(validate_password_strength("Abcd1234").is_ok());
        assert!(validate_password_strength("short").is_err());
        assert!(validate_password_strength("alllowercase1").is_err());
        assert!(validate_password_strength("ALLUPPERCASE1").is_err());
    }

    #[test]
    fn test_name_validation() {
        assert!(validate_character_name("John").is_ok());
        assert!(validate_character_name("Sir John").is_ok());
        assert!(validate_character_name("X").is_err());
        assert!(validate_character_name("John123").is_err());
    }
}

// ============================================================================
// Hardware ID (HWID) Validation
// ============================================================================

/// Hardware fingerprint components from the client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HwidFingerprint {
    /// CPU identifier
    pub cpu_id: String,
    /// Motherboard/BIOS identifier
    pub board_id: String,
    /// Disk serial number
    pub disk_id: String,
    /// MAC address (hashed)
    pub mac_hash: String,
    /// GPU identifier
    pub gpu_id: Option<String>,
    /// OS installation ID
    pub os_id: Option<String>,
}

impl HwidFingerprint {
    /// Generate a composite HWID from the fingerprint
    pub fn to_hwid(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(&self.cpu_id);
        hasher.update(&self.board_id);
        hasher.update(&self.disk_id);
        hasher.update(&self.mac_hash);
        if let Some(ref gpu) = self.gpu_id {
            hasher.update(gpu);
        }
        hex::encode(hasher.finalize())
    }

    /// Validate the fingerprint has required components
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.cpu_id.is_empty() || self.board_id.is_empty() || self.disk_id.is_empty() {
            return Err(ApiError::Validation("Incomplete hardware fingerprint".to_string()));
        }
        Ok(())
    }
}

/// HWID registration record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisteredHwid {
    pub id: Uuid,
    pub account_id: i32,
    pub hwid_hash: String,
    pub friendly_name: Option<String>,
    pub first_seen: chrono::DateTime<chrono::Utc>,
    pub last_seen: chrono::DateTime<chrono::Utc>,
    pub is_banned: bool,
    pub ban_reason: Option<String>,
}

/// HWID validation service
pub struct HwidValidator {
    config: AuthConfig,
    // In production, this would use database
    registered_hwids: std::sync::RwLock<std::collections::HashMap<i32, Vec<RegisteredHwid>>>,
    banned_hwids: std::sync::RwLock<std::collections::HashSet<String>>,
}

impl HwidValidator {
    pub fn new(config: AuthConfig) -> Self {
        Self {
            config,
            registered_hwids: std::sync::RwLock::new(std::collections::HashMap::new()),
            banned_hwids: std::sync::RwLock::new(std::collections::HashSet::new()),
        }
    }

    /// Validate HWID for an account
    pub fn validate(&self, account_id: i32, fingerprint: &HwidFingerprint) -> Result<HwidValidationResult, ApiError> {
        if !self.config.hwid_validation_enabled {
            return Ok(HwidValidationResult::Allowed);
        }

        fingerprint.validate()?;
        let hwid = fingerprint.to_hwid();

        // Check if HWID is globally banned
        {
            let banned = self.banned_hwids.read()
                .map_err(|_| ApiError::Internal)?;
            if banned.contains(&hwid) {
                return Ok(HwidValidationResult::Banned("HWID is banned".to_string()));
            }
        }

        // Check if HWID is registered to this account
        {
            let registered = self.registered_hwids.read()
                .map_err(|_| ApiError::Internal)?;

            if let Some(account_hwids) = registered.get(&account_id) {
                // Check if already registered
                if account_hwids.iter().any(|h| h.hwid_hash == hwid) {
                    return Ok(HwidValidationResult::Allowed);
                }

                // Check if max HWIDs reached
                if account_hwids.len() >= self.config.max_hwid_per_account {
                    return Ok(HwidValidationResult::MaxReached);
                }
            }

            // Check if HWID is registered to another account
            for (other_account, hwids) in registered.iter() {
                if *other_account != account_id {
                    if hwids.iter().any(|h| h.hwid_hash == hwid && !h.is_banned) {
                        return Ok(HwidValidationResult::AlreadyRegistered(*other_account));
                    }
                }
            }
        }

        // New HWID, needs registration
        Ok(HwidValidationResult::NeedsRegistration)
    }

    /// Register a new HWID for an account
    pub fn register(&self, account_id: i32, fingerprint: &HwidFingerprint, friendly_name: Option<&str>) -> Result<RegisteredHwid, ApiError> {
        fingerprint.validate()?;
        let hwid = fingerprint.to_hwid();

        let now = chrono::Utc::now();
        let record = RegisteredHwid {
            id: Uuid::new_v4(),
            account_id,
            hwid_hash: hwid,
            friendly_name: friendly_name.map(String::from),
            first_seen: now,
            last_seen: now,
            is_banned: false,
            ban_reason: None,
        };

        {
            let mut registered = self.registered_hwids.write()
                .map_err(|_| ApiError::Internal)?;
            registered.entry(account_id).or_default().push(record.clone());
        }

        Ok(record)
    }

    /// Ban an HWID globally
    pub fn ban_hwid(&self, hwid_hash: &str, reason: &str) -> Result<(), ApiError> {
        {
            let mut banned = self.banned_hwids.write()
                .map_err(|_| ApiError::Internal)?;
            banned.insert(hwid_hash.to_string());
        }

        // Also mark as banned in all registrations
        {
            let mut registered = self.registered_hwids.write()
                .map_err(|_| ApiError::Internal)?;

            for hwids in registered.values_mut() {
                for hwid in hwids.iter_mut() {
                    if hwid.hwid_hash == hwid_hash {
                        hwid.is_banned = true;
                        hwid.ban_reason = Some(reason.to_string());
                    }
                }
            }
        }

        Ok(())
    }

    /// Get HWIDs for an account
    pub fn get_account_hwids(&self, account_id: i32) -> Result<Vec<RegisteredHwid>, ApiError> {
        let registered = self.registered_hwids.read()
            .map_err(|_| ApiError::Internal)?;

        Ok(registered.get(&account_id).cloned().unwrap_or_default())
    }
}

/// Result of HWID validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HwidValidationResult {
    /// HWID is allowed
    Allowed,
    /// HWID needs to be registered
    NeedsRegistration,
    /// HWID is already registered to another account
    AlreadyRegistered(i32),
    /// Account has reached max HWIDs
    MaxReached,
    /// HWID is banned
    Banned(String),
}

// ============================================================================
// Two-Factor Authentication (TOTP)
// ============================================================================

/// TOTP configuration for an account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotpConfig {
    pub account_id: i32,
    pub secret: String,  // Base32 encoded
    pub enabled: bool,
    pub backup_codes: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub verified_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl TotpConfig {
    /// Generate a new TOTP configuration
    pub fn new(account_id: i32) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let secret_bytes: [u8; 20] = rng.gen();
        let secret = base32::encode(base32::Alphabet::Rfc4648 { padding: false }, &secret_bytes);

        // Generate backup codes
        let backup_codes: Vec<String> = (0..10)
            .map(|_| {
                let code: [u8; 4] = rng.gen();
                format!("{:08X}", u32::from_be_bytes(code))
            })
            .collect();

        Self {
            account_id,
            secret,
            enabled: false,
            backup_codes,
            created_at: chrono::Utc::now(),
            verified_at: None,
        }
    }

    /// Get the provisioning URI for authenticator apps
    pub fn provisioning_uri(&self, email: &str, issuer: &str) -> String {
        format!(
            "otpauth://totp/{}:{}?secret={}&issuer={}&algorithm=SHA1&digits=6&period=30",
            urlencoding::encode(issuer),
            urlencoding::encode(email),
            self.secret,
            urlencoding::encode(issuer),
        )
    }

    /// Verify a TOTP code
    pub fn verify_code(&self, code: &str) -> bool {
        // Parse the code
        let code: u32 = match code.parse() {
            Ok(c) => c,
            Err(_) => return false,
        };

        // Decode the secret
        let secret_bytes = match base32::decode(base32::Alphabet::Rfc4648 { padding: false }, &self.secret) {
            Some(b) => b,
            None => return false,
        };

        // Get current time step
        let now = chrono::Utc::now().timestamp() as u64;
        let time_step = now / 30;

        // Check current and adjacent time steps (for clock drift)
        for offset in [-1i64, 0, 1] {
            let step = (time_step as i64 + offset) as u64;
            if generate_totp(&secret_bytes, step) == code {
                return true;
            }
        }

        false
    }

    /// Use a backup code
    pub fn use_backup_code(&mut self, code: &str) -> bool {
        if let Some(pos) = self.backup_codes.iter().position(|c| c == code) {
            self.backup_codes.remove(pos);
            true
        } else {
            false
        }
    }
}

/// Generate a TOTP code for a given time step
fn generate_totp(secret: &[u8], time_step: u64) -> u32 {
    type HmacSha1 = Hmac<sha1::Sha1>;

    let time_bytes = time_step.to_be_bytes();

    let mut mac = HmacSha1::new_from_slice(secret).expect("HMAC accepts any key length");
    mac.update(&time_bytes);
    let result = mac.finalize().into_bytes();

    // Dynamic truncation
    let offset = (result[19] & 0x0f) as usize;
    let code = ((result[offset] & 0x7f) as u32) << 24
        | (result[offset + 1] as u32) << 16
        | (result[offset + 2] as u32) << 8
        | (result[offset + 3] as u32);

    code % 1_000_000
}

/// Two-factor authentication service
pub struct TwoFactorAuth {
    config: AuthConfig,
    // In production, this would use database
    totp_configs: std::sync::RwLock<std::collections::HashMap<i32, TotpConfig>>,
}

impl TwoFactorAuth {
    pub fn new(config: AuthConfig) -> Self {
        Self {
            config,
            totp_configs: std::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }

    /// Set up 2FA for an account (returns QR code URI)
    pub fn setup(&self, account_id: i32, email: &str) -> Result<(String, Vec<String>), ApiError> {
        let config = TotpConfig::new(account_id);
        let uri = config.provisioning_uri(email, &self.config.totp_issuer);
        let backup_codes = config.backup_codes.clone();

        {
            let mut configs = self.totp_configs.write()
                .map_err(|_| ApiError::Internal)?;
            configs.insert(account_id, config);
        }

        Ok((uri, backup_codes))
    }

    /// Verify and enable 2FA
    pub fn verify_and_enable(&self, account_id: i32, code: &str) -> Result<bool, ApiError> {
        let mut configs = self.totp_configs.write()
            .map_err(|_| ApiError::Internal)?;

        if let Some(config) = configs.get_mut(&account_id) {
            if config.verify_code(code) {
                config.enabled = true;
                config.verified_at = Some(chrono::Utc::now());
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Verify a 2FA code during login
    pub fn verify(&self, account_id: i32, code: &str) -> Result<bool, ApiError> {
        let mut configs = self.totp_configs.write()
            .map_err(|_| ApiError::Internal)?;

        if let Some(config) = configs.get_mut(&account_id) {
            if !config.enabled {
                return Ok(true); // 2FA not enabled, allow
            }

            // Try TOTP first
            if config.verify_code(code) {
                return Ok(true);
            }

            // Try backup code
            if config.use_backup_code(code) {
                return Ok(true);
            }

            return Ok(false);
        }

        Ok(true) // No 2FA config, allow
    }

    /// Check if 2FA is enabled for an account
    pub fn is_enabled(&self, account_id: i32) -> Result<bool, ApiError> {
        let configs = self.totp_configs.read()
            .map_err(|_| ApiError::Internal)?;

        Ok(configs.get(&account_id).map(|c| c.enabled).unwrap_or(false))
    }

    /// Disable 2FA
    pub fn disable(&self, account_id: i32, code: &str) -> Result<bool, ApiError> {
        let mut configs = self.totp_configs.write()
            .map_err(|_| ApiError::Internal)?;

        if let Some(config) = configs.get_mut(&account_id) {
            if config.verify_code(code) {
                config.enabled = false;
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Regenerate backup codes
    pub fn regenerate_backup_codes(&self, account_id: i32, code: &str) -> Result<Vec<String>, ApiError> {
        let mut configs = self.totp_configs.write()
            .map_err(|_| ApiError::Internal)?;

        if let Some(config) = configs.get_mut(&account_id) {
            if !config.verify_code(code) {
                return Err(ApiError::Unauthorized);
            }

            use rand::Rng;
            let mut rng = rand::thread_rng();
            config.backup_codes = (0..10)
                .map(|_| {
                    let code_bytes: [u8; 4] = rng.gen();
                    format!("{:08X}", u32::from_be_bytes(code_bytes))
                })
                .collect();

            return Ok(config.backup_codes.clone());
        }

        Err(ApiError::NotFound)
    }
}
