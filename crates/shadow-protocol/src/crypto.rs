//! Cryptographic functions for Tibia protocol
//!
//! Implements XTEA encryption/decryption and RSA for key exchange

use crate::{ProtocolError, Result, XTEA_KEY_SIZE};

/// XTEA encryption/decryption
pub struct Xtea {
    key: [u32; XTEA_KEY_SIZE],
}

impl Xtea {
    /// Number of rounds for XTEA (standard is 32)
    const ROUNDS: u32 = 32;
    /// XTEA delta constant
    const DELTA: u32 = 0x9E3779B9;

    /// Create a new XTEA cipher with the given key
    pub fn new(key: [u32; XTEA_KEY_SIZE]) -> Self {
        Self { key }
    }

    /// Encrypt data in place (data length must be multiple of 8)
    pub fn encrypt(&self, data: &mut [u8]) -> Result<()> {
        if data.len() % 8 != 0 {
            return Err(ProtocolError::Encryption(
                "Data length must be multiple of 8 for XTEA".to_string(),
            ));
        }

        for chunk in data.chunks_exact_mut(8) {
            self.encrypt_block(chunk);
        }
        Ok(())
    }

    /// Decrypt data in place (data length must be multiple of 8)
    pub fn decrypt(&self, data: &mut [u8]) -> Result<()> {
        if data.len() % 8 != 0 {
            return Err(ProtocolError::Decryption(
                "Data length must be multiple of 8 for XTEA".to_string(),
            ));
        }

        for chunk in data.chunks_exact_mut(8) {
            self.decrypt_block(chunk);
        }
        Ok(())
    }

    fn encrypt_block(&self, block: &mut [u8]) {
        let mut v0 = u32::from_le_bytes([block[0], block[1], block[2], block[3]]);
        let mut v1 = u32::from_le_bytes([block[4], block[5], block[6], block[7]]);
        let mut sum: u32 = 0;

        for _ in 0..Self::ROUNDS {
            v0 = v0.wrapping_add(
                ((v1 << 4) ^ (v1 >> 5)).wrapping_add(v1) ^ sum.wrapping_add(self.key[(sum & 3) as usize]),
            );
            sum = sum.wrapping_add(Self::DELTA);
            v1 = v1.wrapping_add(
                ((v0 << 4) ^ (v0 >> 5)).wrapping_add(v0)
                    ^ sum.wrapping_add(self.key[((sum >> 11) & 3) as usize]),
            );
        }

        block[0..4].copy_from_slice(&v0.to_le_bytes());
        block[4..8].copy_from_slice(&v1.to_le_bytes());
    }

    fn decrypt_block(&self, block: &mut [u8]) {
        let mut v0 = u32::from_le_bytes([block[0], block[1], block[2], block[3]]);
        let mut v1 = u32::from_le_bytes([block[4], block[5], block[6], block[7]]);
        let mut sum: u32 = Self::DELTA.wrapping_mul(Self::ROUNDS);

        for _ in 0..Self::ROUNDS {
            v1 = v1.wrapping_sub(
                ((v0 << 4) ^ (v0 >> 5)).wrapping_add(v0)
                    ^ sum.wrapping_add(self.key[((sum >> 11) & 3) as usize]),
            );
            sum = sum.wrapping_sub(Self::DELTA);
            v0 = v0.wrapping_sub(
                ((v1 << 4) ^ (v1 >> 5)).wrapping_add(v1) ^ sum.wrapping_add(self.key[(sum & 3) as usize]),
            );
        }

        block[0..4].copy_from_slice(&v0.to_le_bytes());
        block[4..8].copy_from_slice(&v1.to_le_bytes());
    }
}

/// RSA key pair for protocol encryption
pub struct RsaKey {
    n: rsa::BigUint,
    d: rsa::BigUint,
    e: rsa::BigUint,
}

impl RsaKey {
    /// Create RSA key from modulus and exponents
    pub fn new(n: &str, e: &str, d: &str) -> Result<Self> {
        use std::str::FromStr;

        let n = rsa::BigUint::from_str(n).map_err(|_| ProtocolError::InvalidRsaKey)?;
        let e = rsa::BigUint::from_str(e).map_err(|_| ProtocolError::InvalidRsaKey)?;
        let d = rsa::BigUint::from_str(d).map_err(|_| ProtocolError::InvalidRsaKey)?;

        Ok(Self { n, e, d })
    }

    /// Create default OT RSA key
    pub fn default_ot_key() -> Result<Self> {
        Self::new(
            crate::RSA_MODULUS,
            crate::RSA_EXPONENT,
            crate::RSA_PRIVATE_EXPONENT,
        )
    }

    /// Decrypt RSA encrypted data (128 bytes)
    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        if data.len() != 128 {
            return Err(ProtocolError::Decryption(format!(
                "RSA block must be 128 bytes, got {}",
                data.len()
            )));
        }

        // Convert to BigUint (big-endian for RSA)
        let c = rsa::BigUint::from_bytes_be(data);

        // m = c^d mod n
        let m = c.modpow(&self.d, &self.n);

        // Convert back to bytes, pad to 128 bytes
        let mut result = m.to_bytes_be();
        while result.len() < 128 {
            result.insert(0, 0);
        }

        Ok(result)
    }

    /// Encrypt data with RSA (public key operation)
    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        if data.len() > 128 {
            return Err(ProtocolError::Encryption(
                "Data too large for RSA encryption".to_string(),
            ));
        }

        // Pad to 128 bytes
        let mut padded = vec![0u8; 128 - data.len()];
        padded.extend_from_slice(data);

        // Convert to BigUint
        let m = rsa::BigUint::from_bytes_be(&padded);

        // c = m^e mod n
        let c = m.modpow(&self.e, &self.n);

        // Convert back to bytes
        let mut result = c.to_bytes_be();
        while result.len() < 128 {
            result.insert(0, 0);
        }

        Ok(result)
    }
}

/// Adler32 checksum calculation
pub fn adler32(data: &[u8]) -> u32 {
    let mut a: u32 = 1;
    let mut b: u32 = 0;

    for &byte in data {
        a = (a + byte as u32) % 65521;
        b = (b + a) % 65521;
    }

    (b << 16) | a
}

/// Verify adler32 checksum
pub fn verify_adler32(data: &[u8], expected: u32) -> bool {
    adler32(data) == expected
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xtea_roundtrip() {
        let key = [0x12345678, 0x9ABCDEF0, 0x13579BDF, 0x2468ACE0];
        let xtea = Xtea::new(key);

        let original = b"Hello World!1234";
        let mut data = original.to_vec();

        xtea.encrypt(&mut data).unwrap();
        assert_ne!(&data[..], original);

        xtea.decrypt(&mut data).unwrap();
        assert_eq!(&data[..], original);
    }

    #[test]
    fn test_adler32() {
        let data = b"Wikipedia";
        assert_eq!(adler32(data), 0x11E60398);
    }
}
