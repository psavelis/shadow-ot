//! Protocol version handling and feature flags

use serde::{Deserialize, Serialize};

/// Supported protocol versions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProtocolVersion {
    V860,
    V870,
    V960,
    V1000,
    V1050,
    V1076,
    V1098,
    V1099,
    V1100,
    V1200,
    V1220,
    V1240,
    V1260,
    V1281,
    V1290,
    V1300,
    V1310,
    Custom(u16),
}

impl ProtocolVersion {
    pub fn from_version(version: u16) -> Self {
        match version {
            860 => Self::V860,
            870 => Self::V870,
            960 => Self::V960,
            1000 => Self::V1000,
            1050 => Self::V1050,
            1076 => Self::V1076,
            1098 => Self::V1098,
            1099 => Self::V1099,
            1100 => Self::V1100,
            1200 => Self::V1200,
            1220 => Self::V1220,
            1240 => Self::V1240,
            1260 => Self::V1260,
            1281 => Self::V1281,
            1290 => Self::V1290,
            1300 => Self::V1300,
            1310 => Self::V1310,
            v => Self::Custom(v),
        }
    }

    pub fn version_number(&self) -> u16 {
        match self {
            Self::V860 => 860,
            Self::V870 => 870,
            Self::V960 => 960,
            Self::V1000 => 1000,
            Self::V1050 => 1050,
            Self::V1076 => 1076,
            Self::V1098 => 1098,
            Self::V1099 => 1099,
            Self::V1100 => 1100,
            Self::V1200 => 1200,
            Self::V1220 => 1220,
            Self::V1240 => 1240,
            Self::V1260 => 1260,
            Self::V1281 => 1281,
            Self::V1290 => 1290,
            Self::V1300 => 1300,
            Self::V1310 => 1310,
            Self::Custom(v) => *v,
        }
    }

    /// Check if this version uses XTEA encryption
    pub fn uses_xtea(&self) -> bool {
        true // All supported versions use XTEA
    }

    /// Check if this version uses RSA encryption
    pub fn uses_rsa(&self) -> bool {
        self.version_number() >= 770
    }

    /// Check if this version uses sequence numbers
    pub fn uses_sequence_numbers(&self) -> bool {
        self.version_number() >= 1050
    }

    /// Check if this version uses adler32 checksum
    pub fn uses_checksum(&self) -> bool {
        self.version_number() >= 830
    }

    /// Check if this version supports the market
    pub fn supports_market(&self) -> bool {
        self.version_number() >= 944
    }

    /// Check if this version supports mounts
    pub fn supports_mounts(&self) -> bool {
        self.version_number() >= 870
    }

    /// Check if this version supports outfits with addons
    pub fn supports_addons(&self) -> bool {
        self.version_number() >= 780
    }

    /// Check if this version supports the prey system
    pub fn supports_prey(&self) -> bool {
        self.version_number() >= 1100
    }

    /// Check if this version supports the bestiary
    pub fn supports_bestiary(&self) -> bool {
        self.version_number() >= 1150
    }

    /// Check if this version supports the analytics/cyclopedia
    pub fn supports_cyclopedia(&self) -> bool {
        self.version_number() >= 1200
    }

    /// Check if this version supports the forge system
    pub fn supports_forge(&self) -> bool {
        self.version_number() >= 1281
    }

    /// Check if this version uses extended protocol (16-bit opcodes)
    pub fn uses_extended_opcodes(&self) -> bool {
        self.version_number() >= 1100
    }

    /// Get the DAT signature for this version
    pub fn dat_signature(&self) -> u32 {
        // These are example signatures - real ones depend on actual client files
        match self.version_number() {
            860 => 0x4A1C7C5E,
            1098 => 0x57A56E77,
            1200 => 0x5C6A8000,
            1281 => 0x5F7A1000,
            1310 => 0x61AB3000,
            _ => 0x00000000,
        }
    }

    /// Get the SPR signature for this version
    pub fn spr_signature(&self) -> u32 {
        match self.version_number() {
            860 => 0x4A1C7C5F,
            1098 => 0x57A56E78,
            1200 => 0x5C6A8001,
            1281 => 0x5F7A1001,
            1310 => 0x61AB3001,
            _ => 0x00000000,
        }
    }
}

impl Default for ProtocolVersion {
    fn default() -> Self {
        Self::V1098
    }
}

impl std::fmt::Display for ProtocolVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.version_number())
    }
}
