//! SPR file parser
//!
//! SPR files contain all sprite graphics in the Tibia client.
//! Format varies by version:
//! - Pre-9.6: 32-bit sprite IDs
//! - 9.6+: Extended 32-bit sprite IDs with LZMA compression
//! - 10.5+: LZMA compressed sprites

use crate::{AssetError, AssetResult, ClientVersion, Color, SPRITE_SIZE, SPRITE_BYTES};
use byteorder::{LittleEndian, ReadBytesExt};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;
use tracing::{debug, trace, warn};

/// Raw sprite data (32x32 RGBA)
#[derive(Clone)]
pub struct SpriteData {
    pub id: u32,
    pub pixels: Vec<u8>, // RGBA, 32x32 = 4096 bytes
}

impl SpriteData {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            pixels: vec![0u8; SPRITE_BYTES],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.pixels.iter().all(|&b| b == 0)
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        if x >= SPRITE_SIZE || y >= SPRITE_SIZE {
            return Color::TRANSPARENT;
        }
        let idx = ((y * SPRITE_SIZE + x) * 4) as usize;
        Color::new(
            self.pixels[idx],
            self.pixels[idx + 1],
            self.pixels[idx + 2],
            self.pixels[idx + 3],
        )
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        if x >= SPRITE_SIZE || y >= SPRITE_SIZE {
            return;
        }
        let idx = ((y * SPRITE_SIZE + x) * 4) as usize;
        self.pixels[idx] = color.r;
        self.pixels[idx + 1] = color.g;
        self.pixels[idx + 2] = color.b;
        self.pixels[idx + 3] = color.a;
    }
}

/// SPR file reader
pub struct SprFile {
    reader: BufReader<File>,
    version: ClientVersion,
    sprite_count: u32,
    sprite_offsets: Vec<u32>,
    cache: HashMap<u32, SpriteData>,
    use_lzma: bool,
    header_size: u64,
}

impl SprFile {
    /// Load SPR file
    pub fn load<P: AsRef<Path>>(path: P, version: ClientVersion) -> AssetResult<Self> {
        let file = File::open(path.as_ref())?;
        let mut reader = BufReader::new(file);

        // Read signature
        let signature = reader.read_u32::<LittleEndian>()?;
        debug!("SPR signature: 0x{:08X}", signature);

        // Read sprite count
        let sprite_count = if version.supports_extended_sprites() {
            reader.read_u32::<LittleEndian>()?
        } else {
            reader.read_u16::<LittleEndian>()? as u32
        };

        debug!("SPR sprite count: {}", sprite_count);

        let header_size = if version.supports_extended_sprites() { 8 } else { 6 };

        // Read sprite offsets
        let mut sprite_offsets = Vec::with_capacity(sprite_count as usize);
        for _ in 0..sprite_count {
            let offset = reader.read_u32::<LittleEndian>()?;
            sprite_offsets.push(offset);
        }

        Ok(Self {
            reader,
            version,
            sprite_count,
            sprite_offsets,
            cache: HashMap::new(),
            use_lzma: version.uses_lzma(),
            header_size,
        })
    }

    /// Get sprite count
    pub fn sprite_count(&self) -> u32 {
        self.sprite_count
    }

    /// Get sprite by ID
    pub fn get_sprite(&mut self, id: u32) -> AssetResult<SpriteData> {
        // Check cache first
        if let Some(sprite) = self.cache.get(&id) {
            return Ok(sprite.clone());
        }

        // Validate ID
        if id == 0 || id > self.sprite_count {
            return Err(AssetError::SpriteNotFound(id));
        }

        let offset = self.sprite_offsets[(id - 1) as usize];
        if offset == 0 {
            // Empty sprite
            return Ok(SpriteData::new(id));
        }

        // Seek to sprite data
        self.reader.seek(SeekFrom::Start(offset as u64))?;

        let sprite = if self.use_lzma {
            self.read_lzma_sprite(id)?
        } else {
            self.read_raw_sprite(id)?
        };

        // Cache the sprite
        self.cache.insert(id, sprite.clone());

        Ok(sprite)
    }

    /// Read raw (uncompressed) sprite
    fn read_raw_sprite(&mut self, id: u32) -> AssetResult<SpriteData> {
        // Skip color key (3 bytes RGB for transparency)
        let _r = self.reader.read_u8()?;
        let _g = self.reader.read_u8()?;
        let _b = self.reader.read_u8()?;

        // Read sprite data size
        let data_size = self.reader.read_u16::<LittleEndian>()? as usize;

        if data_size == 0 {
            return Ok(SpriteData::new(id));
        }

        // Read compressed pixel data
        let mut compressed = vec![0u8; data_size];
        self.reader.read_exact(&mut compressed)?;

        // Decode RLE sprite data
        let mut sprite = SpriteData::new(id);
        self.decode_rle_sprite(&compressed, &mut sprite)?;

        Ok(sprite)
    }

    /// Read LZMA compressed sprite
    fn read_lzma_sprite(&mut self, id: u32) -> AssetResult<SpriteData> {
        // Skip color key
        let _r = self.reader.read_u8()?;
        let _g = self.reader.read_u8()?;
        let _b = self.reader.read_u8()?;

        // Read compressed size
        let compressed_size = self.reader.read_u32::<LittleEndian>()? as usize;

        if compressed_size == 0 {
            return Ok(SpriteData::new(id));
        }

        // Read decompressed size
        let decompressed_size = self.reader.read_u32::<LittleEndian>()? as usize;

        // Read LZMA data
        let mut compressed = vec![0u8; compressed_size];
        self.reader.read_exact(&mut compressed)?;

        // Decompress LZMA
        let decompressed = self.decompress_lzma(&compressed, decompressed_size)?;

        // Decode the decompressed data
        let mut sprite = SpriteData::new(id);

        // LZMA sprites are stored as raw BGRA
        if decompressed.len() >= SPRITE_BYTES {
            for i in 0..(SPRITE_SIZE * SPRITE_SIZE) as usize {
                let src_idx = i * 4;
                let b = decompressed[src_idx];
                let g = decompressed[src_idx + 1];
                let r = decompressed[src_idx + 2];
                let a = decompressed[src_idx + 3];

                let dst_idx = i * 4;
                sprite.pixels[dst_idx] = r;
                sprite.pixels[dst_idx + 1] = g;
                sprite.pixels[dst_idx + 2] = b;
                sprite.pixels[dst_idx + 3] = a;
            }
        }

        Ok(sprite)
    }

    /// Decompress LZMA data
    fn decompress_lzma(&self, compressed: &[u8], expected_size: usize) -> AssetResult<Vec<u8>> {
        use std::io::Cursor;

        // LZMA header format for Tibia sprites
        let mut cursor = Cursor::new(compressed);

        let mut decompressed = Vec::with_capacity(expected_size);

        match lzma_rs::lzma_decompress(&mut cursor, &mut decompressed) {
            Ok(_) => Ok(decompressed),
            Err(e) => Err(AssetError::DecompressionFailed(e.to_string())),
        }
    }

    /// Decode RLE encoded sprite
    fn decode_rle_sprite(&self, data: &[u8], sprite: &mut SpriteData) -> AssetResult<()> {
        let mut data_idx = 0;
        let mut pixel_idx = 0;
        let total_pixels = (SPRITE_SIZE * SPRITE_SIZE) as usize;

        while data_idx < data.len() && pixel_idx < total_pixels {
            // Read transparent pixel count
            if data_idx + 1 >= data.len() {
                break;
            }
            let transparent_count = u16::from_le_bytes([data[data_idx], data[data_idx + 1]]) as usize;
            data_idx += 2;

            // Skip transparent pixels (already initialized to 0)
            pixel_idx += transparent_count;

            if pixel_idx >= total_pixels {
                break;
            }

            // Read colored pixel count
            if data_idx + 1 >= data.len() {
                break;
            }
            let colored_count = u16::from_le_bytes([data[data_idx], data[data_idx + 1]]) as usize;
            data_idx += 2;

            // Read colored pixels (RGB format)
            for _ in 0..colored_count {
                if data_idx + 2 >= data.len() || pixel_idx >= total_pixels {
                    break;
                }

                let r = data[data_idx];
                let g = data[data_idx + 1];
                let b = data[data_idx + 2];
                data_idx += 3;

                let dst_idx = pixel_idx * 4;
                sprite.pixels[dst_idx] = r;
                sprite.pixels[dst_idx + 1] = g;
                sprite.pixels[dst_idx + 2] = b;
                sprite.pixels[dst_idx + 3] = 255; // Opaque

                pixel_idx += 1;
            }
        }

        Ok(())
    }

    /// Clear sprite cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Get all sprite IDs
    pub fn sprite_ids(&self) -> impl Iterator<Item = u32> {
        1..=self.sprite_count
    }

    /// Check if sprite exists
    pub fn has_sprite(&self, id: u32) -> bool {
        if id == 0 || id > self.sprite_count {
            return false;
        }
        self.sprite_offsets[(id - 1) as usize] != 0
    }

    /// Preload sprites into cache
    pub fn preload(&mut self, ids: &[u32]) -> AssetResult<()> {
        for &id in ids {
            if !self.cache.contains_key(&id) {
                if let Ok(sprite) = self.get_sprite(id) {
                    self.cache.insert(id, sprite);
                }
            }
        }
        Ok(())
    }

    /// Export sprite to RGBA buffer
    pub fn export_rgba(&mut self, id: u32) -> AssetResult<Vec<u8>> {
        let sprite = self.get_sprite(id)?;
        Ok(sprite.pixels)
    }
}

/// Sprite sheet builder for efficient atlas creation
pub struct SpriteSheetBuilder {
    sprites: Vec<(u32, SpriteData)>,
    width: u32,
    height: u32,
}

impl SpriteSheetBuilder {
    pub fn new() -> Self {
        Self {
            sprites: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn add_sprite(&mut self, id: u32, sprite: SpriteData) {
        self.sprites.push((id, sprite));
    }

    pub fn build(&self, sprites_per_row: u32) -> (Vec<u8>, u32, u32, HashMap<u32, (u32, u32)>) {
        let rows = (self.sprites.len() as u32 + sprites_per_row - 1) / sprites_per_row;
        let width = sprites_per_row * SPRITE_SIZE;
        let height = rows * SPRITE_SIZE;

        let mut atlas = vec![0u8; (width * height * 4) as usize];
        let mut positions = HashMap::new();

        for (idx, (id, sprite)) in self.sprites.iter().enumerate() {
            let col = (idx as u32) % sprites_per_row;
            let row = (idx as u32) / sprites_per_row;
            let x = col * SPRITE_SIZE;
            let y = row * SPRITE_SIZE;

            positions.insert(*id, (x, y));

            // Copy sprite pixels to atlas
            for sy in 0..SPRITE_SIZE {
                for sx in 0..SPRITE_SIZE {
                    let src_idx = ((sy * SPRITE_SIZE + sx) * 4) as usize;
                    let dst_idx = (((y + sy) * width + (x + sx)) * 4) as usize;

                    atlas[dst_idx..dst_idx + 4].copy_from_slice(&sprite.pixels[src_idx..src_idx + 4]);
                }
            }
        }

        (atlas, width, height, positions)
    }
}

impl Default for SpriteSheetBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sprite_data() {
        let mut sprite = SpriteData::new(1);
        assert!(sprite.is_empty());

        sprite.set_pixel(0, 0, Color::WHITE);
        assert!(!sprite.is_empty());

        let pixel = sprite.get_pixel(0, 0);
        assert_eq!(pixel, Color::WHITE);
    }

    #[test]
    fn test_color_rgb565() {
        let color = Color::from_rgb565(0xFFFF);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 255);
        assert_eq!(color.b, 255);
    }
}
