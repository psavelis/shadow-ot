//! Tile management - represents a single cell in the game world

use crate::item::Item;
use crate::position::Position;
use crate::creature::Creature;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Tile flags representing tile properties
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TileFlags(u32);

impl TileFlags {
    pub const NONE: u32 = 0;
    pub const PROTECTION_ZONE: u32 = 1 << 0;
    pub const NO_PVP_ZONE: u32 = 1 << 1;
    pub const NO_LOGOUT: u32 = 1 << 2;
    pub const PVP_ZONE: u32 = 1 << 3;
    pub const REFRESH: u32 = 1 << 4;
    pub const HOUSE: u32 = 1 << 5;
    pub const NO_SUMMON: u32 = 1 << 6;
    pub const NO_MONSTER: u32 = 1 << 7;
    pub const BLOCK_SOLID: u32 = 1 << 8;
    pub const BLOCK_PROJECTILE: u32 = 1 << 9;
    pub const BLOCK_PATHFIND: u32 = 1 << 10;
    pub const IMMOVABLE_BLOCK_SOLID: u32 = 1 << 11;
    pub const IMMOVABLE_BLOCK_PROJECTILE: u32 = 1 << 12;
    pub const IMMOVABLE_NO_FIELD_BLOCK_PATHFIND: u32 = 1 << 13;
    pub const NO_FIELD_BLOCK_PATHFIND: u32 = 1 << 14;
    pub const SUPPORTS_HANGABLE: u32 = 1 << 15;

    pub fn new() -> Self {
        Self(0)
    }

    pub fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    pub fn bits(&self) -> u32 {
        self.0
    }

    pub fn set(&mut self, flag: u32) {
        self.0 |= flag;
    }

    pub fn unset(&mut self, flag: u32) {
        self.0 &= !flag;
    }

    pub fn has(&self, flag: u32) -> bool {
        (self.0 & flag) != 0
    }

    pub fn is_protection_zone(&self) -> bool {
        self.has(Self::PROTECTION_ZONE)
    }

    pub fn is_pvp_zone(&self) -> bool {
        self.has(Self::PVP_ZONE)
    }

    pub fn is_no_pvp_zone(&self) -> bool {
        self.has(Self::NO_PVP_ZONE)
    }

    pub fn is_house(&self) -> bool {
        self.has(Self::HOUSE)
    }

    pub fn blocks_solid(&self) -> bool {
        self.has(Self::BLOCK_SOLID)
    }

    pub fn blocks_projectile(&self) -> bool {
        self.has(Self::BLOCK_PROJECTILE)
    }

    pub fn blocks_pathfind(&self) -> bool {
        self.has(Self::BLOCK_PATHFIND)
    }
}

/// Represents a single tile in the game world
#[derive(Debug)]
pub struct Tile {
    pub position: Position,
    pub flags: TileFlags,
    pub ground: Option<Item>,
    pub items: Vec<Item>,
    pub creatures: Vec<u32>, // Creature IDs
    pub house_id: Option<u32>,
}

impl Tile {
    pub fn new(position: Position) -> Self {
        Self {
            position,
            flags: TileFlags::new(),
            ground: None,
            items: Vec::new(),
            creatures: Vec::new(),
            house_id: None,
        }
    }

    pub fn with_ground(position: Position, ground: Item) -> Self {
        Self {
            position,
            flags: TileFlags::new(),
            ground: Some(ground),
            items: Vec::new(),
            creatures: Vec::new(),
            house_id: None,
        }
    }

    /// Get the ground item
    pub fn get_ground(&self) -> Option<&Item> {
        self.ground.as_ref()
    }

    /// Set the ground item
    pub fn set_ground(&mut self, item: Item) {
        self.ground = Some(item);
    }

    /// Add an item to the tile
    pub fn add_item(&mut self, item: Item) {
        // Items with always-on-top go first
        if item.is_always_on_top() {
            // Find the correct position for always-on-top items
            let pos = self.items.iter().position(|i| !i.is_always_on_top()).unwrap_or(self.items.len());
            self.items.insert(pos, item);
        } else {
            self.items.push(item);
        }
        self.update_flags();
    }

    /// Remove an item from the tile
    pub fn remove_item(&mut self, index: usize) -> Option<Item> {
        if index < self.items.len() {
            let item = self.items.remove(index);
            self.update_flags();
            Some(item)
        } else {
            None
        }
    }

    /// Get item at stack position
    pub fn get_item_at(&self, stack_pos: u8) -> Option<&Item> {
        let pos = stack_pos as usize;
        if pos == 0 {
            self.ground.as_ref()
        } else {
            self.items.get(pos - 1)
        }
    }

    /// Get mutable item at stack position
    pub fn get_item_at_mut(&mut self, stack_pos: u8) -> Option<&mut Item> {
        let pos = stack_pos as usize;
        if pos == 0 {
            self.ground.as_mut()
        } else {
            self.items.get_mut(pos - 1)
        }
    }

    /// Find item by item type ID
    pub fn find_item(&self, item_id: u16) -> Option<(usize, &Item)> {
        for (i, item) in self.items.iter().enumerate() {
            if item.item_type_id == item_id {
                return Some((i, item));
            }
        }
        None
    }

    /// Add a creature to this tile
    pub fn add_creature(&mut self, creature_id: u32) {
        if !self.creatures.contains(&creature_id) {
            self.creatures.push(creature_id);
        }
    }

    /// Remove a creature from this tile
    pub fn remove_creature(&mut self, creature_id: u32) -> bool {
        if let Some(pos) = self.creatures.iter().position(|&id| id == creature_id) {
            self.creatures.remove(pos);
            true
        } else {
            false
        }
    }

    /// Check if tile has any creatures
    pub fn has_creatures(&self) -> bool {
        !self.creatures.is_empty()
    }

    /// Get creature count
    pub fn creature_count(&self) -> usize {
        self.creatures.len()
    }

    /// Check if a creature can walk on this tile
    pub fn is_walkable(&self) -> bool {
        !self.flags.blocks_solid()
    }

    /// Check if projectiles can pass through
    pub fn is_projectile_passable(&self) -> bool {
        !self.flags.blocks_projectile()
    }

    /// Check if pathfinding should avoid this tile
    pub fn blocks_pathfind(&self) -> bool {
        self.flags.blocks_pathfind()
    }

    /// Update tile flags based on items
    fn update_flags(&mut self) {
        // Reset dynamic flags
        self.flags.unset(TileFlags::BLOCK_SOLID);
        self.flags.unset(TileFlags::BLOCK_PROJECTILE);
        self.flags.unset(TileFlags::BLOCK_PATHFIND);

        // Check ground
        if let Some(ref ground) = self.ground {
            if ground.blocks_solid() {
                self.flags.set(TileFlags::BLOCK_SOLID);
            }
            if ground.blocks_projectile() {
                self.flags.set(TileFlags::BLOCK_PROJECTILE);
            }
            if ground.blocks_pathfind() {
                self.flags.set(TileFlags::BLOCK_PATHFIND);
            }
        }

        // Check items
        for item in &self.items {
            if item.blocks_solid() {
                self.flags.set(TileFlags::BLOCK_SOLID);
            }
            if item.blocks_projectile() {
                self.flags.set(TileFlags::BLOCK_PROJECTILE);
            }
            if item.blocks_pathfind() {
                self.flags.set(TileFlags::BLOCK_PATHFIND);
            }
        }

        // Creatures block movement
        if !self.creatures.is_empty() {
            self.flags.set(TileFlags::BLOCK_SOLID);
        }
    }

    /// Get the total item count (including ground)
    pub fn get_thing_count(&self) -> u8 {
        let ground_count = if self.ground.is_some() { 1 } else { 0 };
        let creature_count = self.creatures.len() as u8;
        let item_count = self.items.len() as u8;
        ground_count + creature_count + item_count
    }

    /// Get the stack position of a thing
    pub fn get_stack_position(&self, thing_id: u32) -> Option<u8> {
        // Ground is always 0
        let mut pos: u8 = if self.ground.is_some() { 1 } else { 0 };

        // Check always-on-top items
        for item in &self.items {
            if item.is_always_on_top() {
                if item.unique_id == thing_id {
                    return Some(pos);
                }
                pos += 1;
            } else {
                break;
            }
        }

        // Check creatures
        for &creature_id in &self.creatures {
            if creature_id == thing_id {
                return Some(pos);
            }
            pos += 1;
        }

        // Check remaining items
        for item in &self.items {
            if !item.is_always_on_top() {
                if item.unique_id == thing_id {
                    return Some(pos);
                }
                pos += 1;
            }
        }

        None
    }

    /// Get top creature on tile
    pub fn get_top_creature(&self) -> Option<u32> {
        self.creatures.first().copied()
    }

    /// Get top item on tile (excluding ground)
    pub fn get_top_item(&self) -> Option<&Item> {
        self.items.last()
    }

    /// Get top visible creature (for targeting)
    pub fn get_top_visible_creature(&self) -> Option<u32> {
        // In real implementation, would filter by visibility
        self.creatures.first().copied()
    }

    /// Check if tile has specific item
    pub fn has_item(&self, item_id: u16) -> bool {
        self.items.iter().any(|item| item.item_type_id == item_id)
    }

    /// Get all items on tile
    pub fn get_items(&self) -> &[Item] {
        &self.items
    }

    /// Get field item (magic field like fire, poison, etc.)
    pub fn get_field_item(&self) -> Option<&Item> {
        self.items.iter().find(|item| item.is_magic_field())
    }

    /// Remove field item
    pub fn remove_field_item(&mut self) -> Option<Item> {
        if let Some(pos) = self.items.iter().position(|item| item.is_magic_field()) {
            Some(self.items.remove(pos))
        } else {
            None
        }
    }

    /// Check if tile can have more items
    pub fn can_add_item(&self) -> bool {
        self.items.len() < crate::MAX_THINGS_PER_TILE as usize
    }
}

/// Thread-safe tile wrapper
pub type SharedTile = Arc<RwLock<Tile>>;

impl Clone for Tile {
    fn clone(&self) -> Self {
        Self {
            position: self.position,
            flags: self.flags,
            ground: self.ground.clone(),
            items: self.items.clone(),
            creatures: self.creatures.clone(),
            house_id: self.house_id,
        }
    }
}
