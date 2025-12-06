//! Appearance types for items, creatures, effects

use serde::{Deserialize, Serialize};

/// Appearance definition (modern format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Appearance {
    pub id: u32,
    pub category: AppearanceCategory,
    pub name: Option<String>,
    pub description: Option<String>,
    pub flags: AppearanceFlags,
    pub frame_groups: Vec<AppearanceFrameGroup>,
}

/// Appearance category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppearanceCategory {
    Object = 0,
    Outfit = 1,
    Effect = 2,
    Missile = 3,
}

/// Appearance frame group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceFrameGroup {
    pub fixed_frame_group: FrameGroupType,
    pub id: u8,
    pub width: u8,
    pub height: u8,
    pub real_size: u8,
    pub layers: u8,
    pub pattern_width: u8,
    pub pattern_height: u8,
    pub pattern_depth: u8,
    pub sprite_info: SpriteInfo,
}

/// Frame group type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrameGroupType {
    Idle = 0,
    Moving = 1,
}

/// Sprite information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteInfo {
    pub pattern_size: u32,
    pub animation: Option<SpriteAnimation>,
    pub sprite_ids: Vec<u32>,
    pub bounding_square: u32,
    pub bounding_add_top: u32,
    pub bounding_add_right: u32,
    pub bounding_add_bottom: u32,
    pub bounding_add_left: u32,
}

/// Sprite animation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteAnimation {
    pub default_start_phase: u32,
    pub synchronized: bool,
    pub random_start_phase: bool,
    pub loop_type: LoopType,
    pub loop_count: u32,
    pub phases: Vec<SpritePhase>,
}

/// Animation loop type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoopType {
    Pingpong = -1,
    Infinite = 0,
    Counted = 1,
}

/// Animation phase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpritePhase {
    pub duration_min: u32,
    pub duration_max: u32,
}

/// Appearance flags
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppearanceFlags {
    // Ground
    pub ground: Option<Ground>,
    // Clip/border
    pub clip: bool,
    // Bottom layer
    pub bottom: bool,
    // Top layer
    pub top: bool,
    // Container
    pub container: bool,
    // Stackable
    pub cumulative: bool,
    // Multi-use
    pub use_target: bool,
    // Force use
    pub force_use: bool,
    // Writable
    pub write: Option<Write>,
    // Writable once
    pub write_once: Option<WriteOnce>,
    // Fluid container
    pub liquid_pool: bool,
    // Splash
    pub liquid_container: bool,
    // Not walkable
    pub unpass: bool,
    // Not moveable
    pub unmove: bool,
    // Not throwable
    pub unsight: bool,
    // Block pathfinder
    pub avoid: bool,
    // No move animation
    pub no_movement_animation: bool,
    // Pickupable
    pub take: bool,
    // Hangable
    pub hang: bool,
    // Hook south
    pub hook_south: bool,
    // Hook east
    pub hook_east: bool,
    // Rotatable
    pub rotate: bool,
    // Light
    pub light: Option<Light>,
    // Don't hide
    pub dont_hide: bool,
    // Translucent
    pub translucent: bool,
    // Displacement
    pub shift: Option<Shift>,
    // Height
    pub height: Option<Height>,
    // Lying object
    pub lying_object: bool,
    // Always animate
    pub animate_always: bool,
    // Automap
    pub automap: Option<Automap>,
    // Lens help
    pub lens_help: Option<LensHelp>,
    // Full tile
    pub full_bank: bool,
    // Ignore look
    pub ignore_look: bool,
    // Clothes
    pub clothes: Option<Clothes>,
    // Market
    pub market: Option<Market>,
    // Default action
    pub default_action: Option<DefaultAction>,
    // Wrap
    pub wrap: bool,
    // Unwrap
    pub unwrap: bool,
    // Top effect
    pub top_effect: bool,
    // NPC sale data
    pub npc_sale_data: Vec<NpcSaleData>,
    // Changed to expire
    pub changed_to_expire: Option<ChangedToExpire>,
    // Corpse
    pub corpse: bool,
    // Player corpse
    pub player_corpse: bool,
    // Cyclopedia item
    pub cyclopedia_item: Option<CyclopediaItem>,
    // Ammo type
    pub ammo_type: u32,
    // Show off socket
    pub show_off_socket: bool,
    // Reportable
    pub reportable: bool,
    // Upgradable
    pub upgradable: Option<Upgradable>,
    // Reverse addons east
    pub reverse_addons_east: bool,
    // Reverse addons west
    pub reverse_addons_west: bool,
    // Reverse addons south
    pub reverse_addons_south: bool,
    // Reverse addons north
    pub reverse_addons_north: bool,
    // Wearout
    pub wearout: bool,
    // Clockwise
    pub clockwise: bool,
    // Counter clockwise
    pub counter_clockwise: bool,
    // Expirestop
    pub expirestop: bool,
    // Slot
    pub slot: u32,
}

/// Ground properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ground {
    pub speed: u16,
}

/// Write properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Write {
    pub max_text_length: u16,
}

/// Write once properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteOnce {
    pub max_text_length_once: u16,
}

/// Light properties
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Light {
    pub brightness: u16,
    pub color: u16,
}

/// Shift/displacement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shift {
    pub x: u16,
    pub y: u16,
}

/// Height/elevation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Height {
    pub elevation: u16,
}

/// Automap/minimap
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Automap {
    pub color: u16,
}

/// Lens help
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LensHelp {
    pub id: u16,
}

/// Clothes slot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clothes {
    pub slot: u16,
}

/// Market data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Market {
    pub category: u16,
    pub trade_as_object_id: u16,
    pub show_as_object_id: u16,
    pub name: String,
    pub restrict_to_profession: u16,
    pub minimum_level: u16,
}

/// Default action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultAction {
    pub action: DefaultActionType,
}

/// Default action types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefaultActionType {
    None = 0,
    Look = 1,
    Use = 2,
    Open = 3,
    AutoWalkHighlight = 4,
}

/// NPC sale data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcSaleData {
    pub name: String,
    pub location: String,
    pub sale_price: u32,
    pub buy_price: u32,
    pub currency_object_type_id: u32,
    pub currency_quest_flag_display_name: String,
}

/// Changed to expire
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangedToExpire {
    pub former_object_type_id: u32,
}

/// Cyclopedia item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CyclopediaItem {
    pub cyclopedia_type: u32,
}

/// Upgradable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Upgradable {
    pub classification: u32,
}

impl Appearance {
    pub fn new(id: u32, category: AppearanceCategory) -> Self {
        Self {
            id,
            category,
            name: None,
            description: None,
            flags: AppearanceFlags::default(),
            frame_groups: Vec::new(),
        }
    }

    /// Check if this is a ground tile
    pub fn is_ground(&self) -> bool {
        self.flags.ground.is_some()
    }

    /// Check if walkable
    pub fn is_walkable(&self) -> bool {
        !self.flags.unpass
    }

    /// Check if blocks projectiles
    pub fn blocks_projectile(&self) -> bool {
        self.flags.unsight
    }

    /// Check if blocks pathfinding
    pub fn blocks_pathfind(&self) -> bool {
        self.flags.avoid
    }

    /// Get ground speed
    pub fn ground_speed(&self) -> u16 {
        self.flags.ground.as_ref().map(|g| g.speed).unwrap_or(100)
    }

    /// Get light info
    pub fn light(&self) -> Option<&Light> {
        self.flags.light.as_ref()
    }

    /// Get market info
    pub fn market(&self) -> Option<&Market> {
        self.flags.market.as_ref()
    }

    /// Check if stackable
    pub fn is_stackable(&self) -> bool {
        self.flags.cumulative
    }

    /// Check if container
    pub fn is_container(&self) -> bool {
        self.flags.container
    }

    /// Check if pickupable
    pub fn is_pickupable(&self) -> bool {
        self.flags.take
    }

    /// Get all sprite IDs
    pub fn sprite_ids(&self) -> Vec<u32> {
        self.frame_groups
            .iter()
            .flat_map(|fg| fg.sprite_info.sprite_ids.iter().copied())
            .collect()
    }
}
