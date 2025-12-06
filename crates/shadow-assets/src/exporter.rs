//! Asset exporter for generating modern asset formats

use crate::{
    AssetCatalog, AssetError, AssetResult, CatalogEntry, CatalogType, ClientVersion, DatFile,
    SprFile, SpriteData, SpriteSheet, SpriteType, SPRITE_SIZE,
};
use image::{ImageBuffer, Rgba, RgbaImage};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tracing::{debug, info};

/// Export format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Png,
    WebP,
    Bmp,
}

impl ExportFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            ExportFormat::Png => "png",
            ExportFormat::WebP => "webp",
            ExportFormat::Bmp => "bmp",
        }
    }
}

/// Sprite sheet configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteSheetConfig {
    pub sprites_per_row: u32,
    pub sprites_per_sheet: u32,
    pub format: String,
}

impl Default for SpriteSheetConfig {
    fn default() -> Self {
        Self {
            sprites_per_row: 64,
            sprites_per_sheet: 4096,
            format: "png".to_string(),
        }
    }
}

/// Asset exporter
pub struct AssetExporter {
    spr: SprFile,
    dat: DatFile,
    config: SpriteSheetConfig,
    output_path: String,
}

impl AssetExporter {
    /// Create new exporter
    pub fn new(spr: SprFile, dat: DatFile, config: SpriteSheetConfig, output_path: String) -> Self {
        Self {
            spr,
            dat,
            config,
            output_path,
        }
    }

    /// Export all sprites to sprite sheets
    pub fn export_sprite_sheets(&mut self) -> AssetResult<Vec<CatalogEntry>> {
        let sprite_count = self.spr.sprite_count();
        let sprites_per_sheet = self.config.sprites_per_sheet;
        let sprites_per_row = self.config.sprites_per_row;

        let sheet_count = (sprite_count + sprites_per_sheet - 1) / sprites_per_sheet;
        info!(
            "Exporting {} sprites to {} sheets",
            sprite_count, sheet_count
        );

        // Create output directory
        let sprites_dir = format!("{}/sprites", self.output_path);
        fs::create_dir_all(&sprites_dir)?;

        let mut entries = Vec::new();

        for sheet_idx in 0..sheet_count {
            let first_id = sheet_idx * sprites_per_sheet + 1;
            let last_id = ((sheet_idx + 1) * sprites_per_sheet).min(sprite_count);

            let actual_sprites = last_id - first_id + 1;
            let rows = (actual_sprites + sprites_per_row - 1) / sprites_per_row;

            let width = sprites_per_row * SPRITE_SIZE;
            let height = rows * SPRITE_SIZE;

            let mut image: RgbaImage = ImageBuffer::new(width, height);

            // Fill with transparent
            for pixel in image.pixels_mut() {
                *pixel = Rgba([0, 0, 0, 0]);
            }

            // Draw sprites
            for sprite_id in first_id..=last_id {
                let sprite = self.spr.get_sprite(sprite_id)?;
                let local_idx = sprite_id - first_id;
                let col = local_idx % sprites_per_row;
                let row = local_idx / sprites_per_row;

                let x_offset = col * SPRITE_SIZE;
                let y_offset = row * SPRITE_SIZE;

                self.draw_sprite(&mut image, &sprite, x_offset, y_offset);
            }

            // Save image
            let filename = format!("sprites-{}.{}", sheet_idx, self.config.format);
            let filepath = format!("{}/{}", sprites_dir, filename);

            image.save(&filepath)?;
            debug!("Saved sprite sheet: {}", filename);

            // Add catalog entry
            entries.push(CatalogEntry {
                type_: CatalogType::Sprite,
                file: format!("sprites/{}", filename),
                sprite_type: Some(SpriteType::Normal),
                first_sprite_id: first_id,
                last_sprite_id: last_id,
                area: Some(0),
            });
        }

        Ok(entries)
    }

    /// Draw a sprite onto an image
    fn draw_sprite(&self, image: &mut RgbaImage, sprite: &SpriteData, x_offset: u32, y_offset: u32) {
        for y in 0..SPRITE_SIZE {
            for x in 0..SPRITE_SIZE {
                let src_idx = ((y * SPRITE_SIZE + x) * 4) as usize;
                let r = sprite.pixels[src_idx];
                let g = sprite.pixels[src_idx + 1];
                let b = sprite.pixels[src_idx + 2];
                let a = sprite.pixels[src_idx + 3];

                if a > 0 {
                    let pixel = image.get_pixel_mut(x_offset + x, y_offset + y);
                    *pixel = Rgba([r, g, b, a]);
                }
            }
        }
    }

    /// Export a single sprite to PNG
    pub fn export_sprite<P: AsRef<Path>>(&mut self, sprite_id: u32, path: P) -> AssetResult<()> {
        let sprite = self.spr.get_sprite(sprite_id)?;
        let mut image: RgbaImage = ImageBuffer::new(SPRITE_SIZE, SPRITE_SIZE);

        for y in 0..SPRITE_SIZE {
            for x in 0..SPRITE_SIZE {
                let src_idx = ((y * SPRITE_SIZE + x) * 4) as usize;
                let r = sprite.pixels[src_idx];
                let g = sprite.pixels[src_idx + 1];
                let b = sprite.pixels[src_idx + 2];
                let a = sprite.pixels[src_idx + 3];
                image.put_pixel(x, y, Rgba([r, g, b, a]));
            }
        }

        image.save(path)?;
        Ok(())
    }

    /// Export item composite sprite (multiple tiles)
    pub fn export_item_composite<P: AsRef<Path>>(
        &mut self,
        item_id: u16,
        path: P,
    ) -> AssetResult<()> {
        let item = self
            .dat
            .get_item(item_id)
            .ok_or(AssetError::ItemNotFound(item_id as u32))?
            .clone();

        if item.frame_groups.is_empty() {
            return Err(AssetError::InvalidFormat(format!(
                "Item {} has no frame groups",
                item_id
            )));
        }

        let fg = &item.frame_groups[0];
        let width = fg.width as u32 * SPRITE_SIZE;
        let height = fg.height as u32 * SPRITE_SIZE;

        let mut image: RgbaImage = ImageBuffer::new(width, height);

        // Fill transparent
        for pixel in image.pixels_mut() {
            *pixel = Rgba([0, 0, 0, 0]);
        }

        // Draw each sprite tile
        for y in 0..fg.height {
            for x in 0..fg.width {
                let idx = (y as usize * fg.width as usize + x as usize) % fg.sprite_ids.len();
                let sprite_id = fg.sprite_ids[idx];

                if sprite_id > 0 {
                    let sprite = self.spr.get_sprite(sprite_id)?;
                    let x_offset = x as u32 * SPRITE_SIZE;
                    let y_offset = y as u32 * SPRITE_SIZE;

                    self.draw_sprite(&mut image, &sprite, x_offset, y_offset);
                }
            }
        }

        image.save(path)?;
        Ok(())
    }

    /// Export outfit composite with all directions
    pub fn export_outfit<P: AsRef<Path>>(
        &mut self,
        outfit_id: u16,
        path: P,
    ) -> AssetResult<()> {
        let outfit = self
            .dat
            .get_creature(outfit_id)
            .ok_or(AssetError::ItemNotFound(outfit_id as u32))?
            .clone();

        if outfit.frame_groups.is_empty() {
            return Err(AssetError::InvalidFormat(format!(
                "Outfit {} has no frame groups",
                outfit_id
            )));
        }

        let fg = &outfit.frame_groups[0];

        // Calculate image size (pattern_x directions, pattern_z addons)
        let tile_width = fg.width as u32 * SPRITE_SIZE;
        let tile_height = fg.height as u32 * SPRITE_SIZE;

        let total_width = tile_width * fg.pattern_x as u32 * fg.frames as u32;
        let total_height = tile_height * fg.pattern_z as u32;

        let mut image: RgbaImage = ImageBuffer::new(total_width, total_height);

        // Fill transparent
        for pixel in image.pixels_mut() {
            *pixel = Rgba([0, 0, 0, 0]);
        }

        // Draw each direction and frame
        let mut sprite_idx = 0;
        for z in 0..fg.pattern_z {
            for frame in 0..fg.frames {
                for x_dir in 0..fg.pattern_x {
                    for layer in 0..fg.layers {
                        for y in 0..fg.height {
                            for x in 0..fg.width {
                                if sprite_idx < fg.sprite_ids.len() {
                                    let sprite_id = fg.sprite_ids[sprite_idx];
                                    sprite_idx += 1;

                                    if sprite_id > 0 {
                                        let sprite = self.spr.get_sprite(sprite_id)?;

                                        let x_offset = (x_dir as u32 * fg.frames as u32 + frame as u32)
                                            * tile_width
                                            + x as u32 * SPRITE_SIZE;
                                        let y_offset = z as u32 * tile_height + y as u32 * SPRITE_SIZE;

                                        self.draw_sprite(&mut image, &sprite, x_offset, y_offset);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        image.save(path)?;
        Ok(())
    }

    /// Generate sprite atlas metadata
    pub fn generate_atlas_metadata(&self, entries: &[CatalogEntry]) -> AssetResult<String> {
        let metadata = serde_json::to_string_pretty(entries)
            .map_err(|e| AssetError::InvalidFormat(format!("Failed to serialize metadata: {}", e)))?;
        Ok(metadata)
    }

    /// Calculate SHA256 hash of a file
    pub fn hash_file<P: AsRef<Path>>(path: P) -> AssetResult<String> {
        let data = fs::read(path)?;
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let hash = hasher.finalize();
        Ok(hex::encode(hash))
    }

    /// Export full asset package
    pub fn export_full_package(&mut self) -> AssetResult<AssetCatalog> {
        info!("Starting full asset export to {}", self.output_path);

        // Create output directories
        fs::create_dir_all(&self.output_path)?;

        // Export sprite sheets
        let sprite_entries = self.export_sprite_sheets()?;

        // Create catalog
        let mut catalog = AssetCatalog::new(&self.output_path);

        for entry in sprite_entries {
            catalog.add_entry(entry);
        }

        // Save catalog
        let catalog_path = format!("{}/catalog.json", self.output_path);
        catalog.save_catalog(&catalog_path)?;

        info!("Asset export complete");
        Ok(catalog)
    }

    /// Get DAT reference
    pub fn dat(&self) -> &DatFile {
        &self.dat
    }

    /// Get SPR reference
    pub fn spr(&self) -> &SprFile {
        &self.spr
    }
}

/// Quick utility to convert legacy assets
pub fn convert_legacy_assets<P: AsRef<Path>>(
    spr_path: P,
    dat_path: P,
    output_path: P,
    version: ClientVersion,
) -> AssetResult<()> {
    info!("Loading SPR file...");
    let spr = SprFile::load(spr_path, version)?;

    info!("Loading DAT file...");
    let dat = DatFile::load(dat_path, version)?;

    let config = SpriteSheetConfig::default();
    let mut exporter = AssetExporter::new(
        spr,
        dat,
        config,
        output_path.as_ref().to_string_lossy().to_string(),
    );

    exporter.export_full_package()?;

    Ok(())
}
