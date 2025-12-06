//! Asset catalog for managing sprite sheets and appearances

use crate::{Appearance, AppearanceCategory, AssetError, AssetResult, SpriteSheet};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tracing::info;

/// Asset catalog entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogEntry {
    pub type_: CatalogType,
    pub file: String,
    pub sprite_type: Option<SpriteType>,
    pub first_sprite_id: u32,
    pub last_sprite_id: u32,
    pub area: Option<u32>,
}

/// Catalog types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CatalogType {
    Sprite,
    Appearance,
}

/// Sprite types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpriteType {
    Normal = 0,
    Lzma = 1,
}

/// Asset catalog for managing sprites and appearances
#[derive(Debug)]
pub struct AssetCatalog {
    entries: Vec<CatalogEntry>,
    appearances: HashMap<u32, Appearance>,
    items: HashMap<u32, Appearance>,
    outfits: HashMap<u32, Appearance>,
    effects: HashMap<u32, Appearance>,
    missiles: HashMap<u32, Appearance>,
    sprite_sheets: HashMap<String, SpriteSheet>,
    base_path: String,
}

impl AssetCatalog {
    /// Create new asset catalog
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        Self {
            entries: Vec::new(),
            appearances: HashMap::new(),
            items: HashMap::new(),
            outfits: HashMap::new(),
            effects: HashMap::new(),
            missiles: HashMap::new(),
            sprite_sheets: HashMap::new(),
            base_path: base_path.as_ref().to_string_lossy().to_string(),
        }
    }

    /// Load catalog from JSON file
    pub fn load_catalog<P: AsRef<Path>>(&mut self, catalog_path: P) -> AssetResult<()> {
        let content = std::fs::read_to_string(catalog_path.as_ref())?;
        self.entries = serde_json::from_str(&content)
            .map_err(|e| AssetError::InvalidFormat(format!("Invalid catalog JSON: {}", e)))?;

        info!("Loaded {} catalog entries", self.entries.len());
        Ok(())
    }

    /// Save catalog to JSON file
    pub fn save_catalog<P: AsRef<Path>>(&self, catalog_path: P) -> AssetResult<()> {
        let content = serde_json::to_string_pretty(&self.entries)
            .map_err(|e| AssetError::InvalidFormat(format!("Failed to serialize catalog: {}", e)))?;
        std::fs::write(catalog_path, content)?;
        Ok(())
    }

    /// Add catalog entry
    pub fn add_entry(&mut self, entry: CatalogEntry) {
        self.entries.push(entry);
    }

    /// Get sprite sheet for sprite ID
    pub fn get_sprite_sheet(&self, sprite_id: u32) -> Option<(&String, &CatalogEntry)> {
        for entry in &self.entries {
            if entry.type_ == CatalogType::Sprite
                && sprite_id >= entry.first_sprite_id
                && sprite_id <= entry.last_sprite_id
            {
                return Some((&entry.file, entry));
            }
        }
        None
    }

    /// Register appearance
    pub fn register_appearance(&mut self, appearance: Appearance) {
        let id = appearance.id;
        match appearance.category {
            AppearanceCategory::Object => {
                self.items.insert(id, appearance.clone());
            }
            AppearanceCategory::Outfit => {
                self.outfits.insert(id, appearance.clone());
            }
            AppearanceCategory::Effect => {
                self.effects.insert(id, appearance.clone());
            }
            AppearanceCategory::Missile => {
                self.missiles.insert(id, appearance.clone());
            }
        }
        self.appearances.insert(id, appearance);
    }

    /// Get item by ID
    pub fn get_item(&self, id: u32) -> Option<&Appearance> {
        self.items.get(&id)
    }

    /// Get outfit by ID
    pub fn get_outfit(&self, id: u32) -> Option<&Appearance> {
        self.outfits.get(&id)
    }

    /// Get effect by ID
    pub fn get_effect(&self, id: u32) -> Option<&Appearance> {
        self.effects.get(&id)
    }

    /// Get missile by ID
    pub fn get_missile(&self, id: u32) -> Option<&Appearance> {
        self.missiles.get(&id)
    }

    /// Get any appearance by ID
    pub fn get_appearance(&self, id: u32) -> Option<&Appearance> {
        self.appearances.get(&id)
    }

    /// Get all items
    pub fn items(&self) -> impl Iterator<Item = &Appearance> {
        self.items.values()
    }

    /// Get all outfits
    pub fn outfits(&self) -> impl Iterator<Item = &Appearance> {
        self.outfits.values()
    }

    /// Get all effects
    pub fn effects(&self) -> impl Iterator<Item = &Appearance> {
        self.effects.values()
    }

    /// Get all missiles
    pub fn missiles(&self) -> impl Iterator<Item = &Appearance> {
        self.missiles.values()
    }

    /// Register sprite sheet
    pub fn register_sprite_sheet(&mut self, name: String, sheet: SpriteSheet) {
        self.sprite_sheets.insert(name, sheet);
    }

    /// Get sprite sheet by name
    pub fn get_sprite_sheet_by_name(&self, name: &str) -> Option<&SpriteSheet> {
        self.sprite_sheets.get(name)
    }

    /// Get base path
    pub fn base_path(&self) -> &str {
        &self.base_path
    }

    /// Get full path for a file
    pub fn full_path(&self, file: &str) -> String {
        format!("{}/{}", self.base_path, file)
    }

    /// Get entries
    pub fn entries(&self) -> &[CatalogEntry] {
        &self.entries
    }

    /// Item count
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Outfit count
    pub fn outfit_count(&self) -> usize {
        self.outfits.len()
    }

    /// Effect count
    pub fn effect_count(&self) -> usize {
        self.effects.len()
    }

    /// Missile count
    pub fn missile_count(&self) -> usize {
        self.missiles.len()
    }
}

impl Default for AssetCatalog {
    fn default() -> Self {
        Self::new(".")
    }
}

/// Catalog builder for creating asset catalogs
pub struct CatalogBuilder {
    entries: Vec<CatalogEntry>,
    sprites_per_sheet: u32,
    current_sprite_id: u32,
}

impl CatalogBuilder {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            sprites_per_sheet: 4096,
            current_sprite_id: 1,
        }
    }

    pub fn sprites_per_sheet(mut self, count: u32) -> Self {
        self.sprites_per_sheet = count;
        self
    }

    pub fn add_sprite_sheet(&mut self, file: String, sprite_count: u32, sprite_type: SpriteType) {
        let first_id = self.current_sprite_id;
        let last_id = first_id + sprite_count - 1;

        self.entries.push(CatalogEntry {
            type_: CatalogType::Sprite,
            file,
            sprite_type: Some(sprite_type),
            first_sprite_id: first_id,
            last_sprite_id: last_id,
            area: None,
        });

        self.current_sprite_id = last_id + 1;
    }

    pub fn add_appearance_file(&mut self, file: String) {
        self.entries.push(CatalogEntry {
            type_: CatalogType::Appearance,
            file,
            sprite_type: None,
            first_sprite_id: 0,
            last_sprite_id: 0,
            area: None,
        });
    }

    pub fn build(self) -> Vec<CatalogEntry> {
        self.entries
    }
}

impl Default for CatalogBuilder {
    fn default() -> Self {
        Self::new()
    }
}
