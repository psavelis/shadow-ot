//! Shadow OT Asset Pipeline
//!
//! This crate handles loading and parsing of Tibia client assets:
//! - SPR (Sprite) files - Contains all game sprites
//! - DAT (Data) files - Contains item/creature/effect definitions
//! - PNG exports - For modern client support
//! - OTB (Open Tibia Binary) - Item database

pub mod spr;
pub mod dat;
pub mod otb;
pub mod sprite;
pub mod appearance;
pub mod catalog;
pub mod exporter;

pub use spr::{SprFile, SpriteData};
pub use dat::{DatFile, ThingType, ThingCategory};
pub use otb::{OtbFile, OtbItem, ItemFlags};
pub use sprite::{Sprite, SpriteSheet, Animation, FrameGroup};
pub use appearance::{Appearance, AppearanceFlags, Light, Market};
pub use catalog::{AssetCatalog, CatalogEntry};
pub use exporter::{AssetExporter, ExportFormat};

use thiserror::Error;

/// Asset errors
#[derive(Error, Debug)]
pub enum AssetError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid file format: {0}")]
    InvalidFormat(String),

    #[error("Unsupported version: {0}")]
    UnsupportedVersion(u32),

    #[error("Sprite not found: {0}")]
    SpriteNotFound(u32),

    #[error("Item not found: {0}")]
    ItemNotFound(u32),

    #[error("Decompression failed: {0}")]
    DecompressionFailed(String),

    #[error("Invalid sprite data at ID {0}")]
    InvalidSpriteData(u32),

    #[error("Image error: {0}")]
    Image(#[from] image::ImageError),
}

pub type AssetResult<T> = Result<T, AssetError>;

/// Supported client versions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientVersion {
    V740,
    V750,
    V760,
    V770,
    V780,
    V790,
    V792,
    V800,
    V810,
    V820,
    V830,
    V840,
    V850,
    V854,
    V860,
    V870,
    V900,
    V910,
    V920,
    V940,
    V944,
    V953,
    V954,
    V960,
    V961,
    V963,
    V970,
    V980,
    V981,
    V1000,
    V1010,
    V1020,
    V1030,
    V1031,
    V1035,
    V1036,
    V1037,
    V1038,
    V1050,
    V1051,
    V1052,
    V1053,
    V1054,
    V1055,
    V1056,
    V1057,
    V1058,
    V1060,
    V1061,
    V1062,
    V1063,
    V1064,
    V1070,
    V1072,
    V1073,
    V1074,
    V1075,
    V1076,
    V1077,
    V1078,
    V1079,
    V1080,
    V1081,
    V1090,
    V1091,
    V1092,
    V1093,
    V1094,
    V1095,
    V1096,
    V1097,
    V1098,
    V1099,
    V1100,
    V1200,
    V1220,
    V1240,
    V1250,
    V1260,
    V1270,
    V1280,
    V1290,
    V1300,
    V1310,
    V1320,
    Unknown,
}

impl ClientVersion {
    pub fn from_dat_signature(sig: u32) -> Self {
        match sig {
            0x439D5A33 => ClientVersion::V740,
            0x41BF05E7 => ClientVersion::V750,
            0x439D5A33 => ClientVersion::V760,
            0x422A2280 => ClientVersion::V770,
            0x41B8B49D => ClientVersion::V780,
            0x416D2A22 => ClientVersion::V790,
            0x41F2A06F => ClientVersion::V792,
            0x46A29261 => ClientVersion::V800,
            0x4783C0E0 => ClientVersion::V810,
            0x4A10CB12 => ClientVersion::V820,
            0x4A3C4F2B => ClientVersion::V830,
            0x4D2A3D0F => ClientVersion::V840,
            0x4E0F68C8 => ClientVersion::V850,
            0x57BBE02D => ClientVersion::V1000,
            0x57E20FA2 => ClientVersion::V1010,
            0x580B60D4 => ClientVersion::V1020,
            0x582D71A0 => ClientVersion::V1031,
            _ => ClientVersion::Unknown,
        }
    }

    pub fn supports_extended_sprites(&self) -> bool {
        matches!(
            self,
            ClientVersion::V960
                | ClientVersion::V961
                | ClientVersion::V963
                | ClientVersion::V970
                | ClientVersion::V980
                | ClientVersion::V981
                | ClientVersion::V1000
                | ClientVersion::V1010
                | ClientVersion::V1020
                | ClientVersion::V1030
                | ClientVersion::V1031
                | ClientVersion::V1035
                | ClientVersion::V1036
                | ClientVersion::V1037
                | ClientVersion::V1038
                | ClientVersion::V1050
                | ClientVersion::V1051
                | ClientVersion::V1052
                | ClientVersion::V1053
                | ClientVersion::V1054
                | ClientVersion::V1055
                | ClientVersion::V1056
                | ClientVersion::V1057
                | ClientVersion::V1058
                | ClientVersion::V1060
                | ClientVersion::V1061
                | ClientVersion::V1062
                | ClientVersion::V1063
                | ClientVersion::V1064
                | ClientVersion::V1070
                | ClientVersion::V1072
                | ClientVersion::V1073
                | ClientVersion::V1074
                | ClientVersion::V1075
                | ClientVersion::V1076
                | ClientVersion::V1077
                | ClientVersion::V1078
                | ClientVersion::V1079
                | ClientVersion::V1080
                | ClientVersion::V1081
                | ClientVersion::V1090
                | ClientVersion::V1091
                | ClientVersion::V1092
                | ClientVersion::V1093
                | ClientVersion::V1094
                | ClientVersion::V1095
                | ClientVersion::V1096
                | ClientVersion::V1097
                | ClientVersion::V1098
                | ClientVersion::V1099
                | ClientVersion::V1100
                | ClientVersion::V1200
                | ClientVersion::V1220
                | ClientVersion::V1240
                | ClientVersion::V1250
                | ClientVersion::V1260
                | ClientVersion::V1270
                | ClientVersion::V1280
                | ClientVersion::V1290
                | ClientVersion::V1300
                | ClientVersion::V1310
                | ClientVersion::V1320
        )
    }

    pub fn uses_lzma(&self) -> bool {
        matches!(
            self,
            ClientVersion::V1050
                | ClientVersion::V1051
                | ClientVersion::V1052
                | ClientVersion::V1053
                | ClientVersion::V1054
                | ClientVersion::V1055
                | ClientVersion::V1056
                | ClientVersion::V1057
                | ClientVersion::V1058
                | ClientVersion::V1060
                | ClientVersion::V1061
                | ClientVersion::V1062
                | ClientVersion::V1063
                | ClientVersion::V1064
                | ClientVersion::V1070
                | ClientVersion::V1072
                | ClientVersion::V1073
                | ClientVersion::V1074
                | ClientVersion::V1075
                | ClientVersion::V1076
                | ClientVersion::V1077
                | ClientVersion::V1078
                | ClientVersion::V1079
                | ClientVersion::V1080
                | ClientVersion::V1081
                | ClientVersion::V1090
                | ClientVersion::V1091
                | ClientVersion::V1092
                | ClientVersion::V1093
                | ClientVersion::V1094
                | ClientVersion::V1095
                | ClientVersion::V1096
                | ClientVersion::V1097
                | ClientVersion::V1098
                | ClientVersion::V1099
                | ClientVersion::V1100
                | ClientVersion::V1200
                | ClientVersion::V1220
                | ClientVersion::V1240
                | ClientVersion::V1250
                | ClientVersion::V1260
                | ClientVersion::V1270
                | ClientVersion::V1280
                | ClientVersion::V1290
                | ClientVersion::V1300
                | ClientVersion::V1310
                | ClientVersion::V1320
        )
    }
}

/// RGBA color
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const TRANSPARENT: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
    pub const MAGENTA: Color = Color { r: 255, g: 0, b: 255, a: 255 };

    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_rgb565(value: u16) -> Self {
        let r = ((value >> 11) & 0x1F) as u8;
        let g = ((value >> 5) & 0x3F) as u8;
        let b = (value & 0x1F) as u8;
        Self {
            r: (r << 3) | (r >> 2),
            g: (g << 2) | (g >> 4),
            b: (b << 3) | (b >> 2),
            a: 255,
        }
    }

    pub fn to_rgba(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

/// Standard sprite dimensions
pub const SPRITE_SIZE: u32 = 32;
pub const SPRITE_PIXELS: usize = (SPRITE_SIZE * SPRITE_SIZE) as usize;
pub const SPRITE_BYTES: usize = SPRITE_PIXELS * 4; // RGBA
