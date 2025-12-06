//! Item Actions
//!
//! Handles item use, move, rotate, and other actions.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::item::{Item, ItemType, SlotType};
use crate::position::Position;
use crate::{Result, WorldError};

/// Result of an item action
#[derive(Debug, Clone)]
pub enum ItemActionResult {
    /// Action succeeded
    Success,
    /// Item consumed (count reduced)
    Consumed { remaining: u16 },
    /// Item transformed into another
    Transformed { new_item_id: u16 },
    /// Action failed with reason
    Failed(String),
    /// Action requires further input
    Pending { message: String },
}

/// Types of item actions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemAction {
    /// Use item (double-click)
    Use,
    /// Use item with another item
    UseWith,
    /// Use item on creature
    UseOnCreature,
    /// Move item
    Move,
    /// Rotate item
    Rotate,
    /// Wrap/unwrap item
    Wrap,
    /// Read item (books, scrolls)
    Read,
    /// Write on item
    Write,
    /// Browse container contents
    Browse,
}

/// Context for item action
#[derive(Debug, Clone)]
pub struct ItemActionContext {
    /// Player performing the action
    pub player_id: Uuid,
    /// Item being acted upon
    pub item_unique_id: u32,
    /// Item type ID
    pub item_type_id: u16,
    /// Source position
    pub from_position: Position,
    /// Target position (for move/use-with)
    pub to_position: Option<Position>,
    /// Target item (for use-with)
    pub target_item_id: Option<u32>,
    /// Target creature (for use-on-creature)
    pub target_creature_id: Option<u32>,
    /// Stack position
    pub stack_pos: u8,
    /// Count to move/use
    pub count: u16,
}

impl ItemActionContext {
    pub fn new(player_id: Uuid, item_unique_id: u32, item_type_id: u16, position: Position) -> Self {
        Self {
            player_id,
            item_unique_id,
            item_type_id,
            from_position: position,
            to_position: None,
            target_item_id: None,
            target_creature_id: None,
            stack_pos: 0,
            count: 1,
        }
    }

    pub fn with_target_position(mut self, pos: Position) -> Self {
        self.to_position = Some(pos);
        self
    }

    pub fn with_target_item(mut self, item_id: u32) -> Self {
        self.target_item_id = Some(item_id);
        self
    }

    pub fn with_count(mut self, count: u16) -> Self {
        self.count = count;
        self
    }
}

/// Handler for specific item type actions
pub trait ItemActionHandler: Send + Sync {
    /// Handle the item being used
    fn on_use(&self, ctx: &ItemActionContext) -> ItemActionResult;

    /// Handle the item being used with another item
    fn on_use_with(&self, ctx: &ItemActionContext, target: &Item) -> ItemActionResult;

    /// Handle the item being moved
    fn on_move(&self, ctx: &ItemActionContext) -> ItemActionResult;

    /// Get action ID this handler responds to
    fn action_id(&self) -> Option<u16> {
        None
    }

    /// Get unique ID this handler responds to
    fn unique_id(&self) -> Option<u16> {
        None
    }
}

/// Default handler for items with no special behavior
pub struct DefaultItemHandler;

impl ItemActionHandler for DefaultItemHandler {
    fn on_use(&self, ctx: &ItemActionContext) -> ItemActionResult {
        ItemActionResult::Success
    }

    fn on_use_with(&self, ctx: &ItemActionContext, _target: &Item) -> ItemActionResult {
        ItemActionResult::Failed("You cannot use these items together.".to_string())
    }

    fn on_move(&self, ctx: &ItemActionContext) -> ItemActionResult {
        ItemActionResult::Success
    }
}

/// Handler for ropes
pub struct RopeHandler;

impl ItemActionHandler for RopeHandler {
    fn on_use(&self, ctx: &ItemActionContext) -> ItemActionResult {
        // Check if using on rope spot
        ItemActionResult::Success
    }

    fn on_use_with(&self, ctx: &ItemActionContext, target: &Item) -> ItemActionResult {
        // Check if target is a rope spot or hole
        ItemActionResult::Success
    }

    fn on_move(&self, _ctx: &ItemActionContext) -> ItemActionResult {
        ItemActionResult::Success
    }
}

/// Handler for shovels
pub struct ShovelHandler;

impl ItemActionHandler for ShovelHandler {
    fn on_use(&self, ctx: &ItemActionContext) -> ItemActionResult {
        ItemActionResult::Success
    }

    fn on_use_with(&self, ctx: &ItemActionContext, target: &Item) -> ItemActionResult {
        // Check if target is diggable
        // 493 = loose stone pile -> 489 hole
        // 392 = desert sand -> 396 hole
        ItemActionResult::Success
    }

    fn on_move(&self, _ctx: &ItemActionContext) -> ItemActionResult {
        ItemActionResult::Success
    }
}

/// Handler for potions
pub struct PotionHandler {
    /// Health restored
    pub health_min: i32,
    pub health_max: i32,
    /// Mana restored
    pub mana_min: i32,
    pub mana_max: i32,
    /// Required level
    pub level: u16,
    /// Vocations that can use (bitmask)
    pub vocation_mask: u32,
    /// Flask item ID
    pub flask_id: u16,
}

impl PotionHandler {
    pub fn health_potion() -> Self {
        Self {
            health_min: 100,
            health_max: 200,
            mana_min: 0,
            mana_max: 0,
            level: 1,
            vocation_mask: 0xFFFF,
            flask_id: 2006,
        }
    }

    pub fn mana_potion() -> Self {
        Self {
            health_min: 0,
            health_max: 0,
            mana_min: 75,
            mana_max: 150,
            level: 1,
            vocation_mask: 0xFFFF,
            flask_id: 2006,
        }
    }

    pub fn great_health_potion() -> Self {
        Self {
            health_min: 400,
            health_max: 600,
            mana_min: 0,
            mana_max: 0,
            level: 50,
            vocation_mask: 0x3, // Knight, Paladin
            flask_id: 2006,
        }
    }

    pub fn ultimate_health_potion() -> Self {
        Self {
            health_min: 650,
            health_max: 850,
            mana_min: 0,
            mana_max: 0,
            level: 130,
            vocation_mask: 0x1, // Knight only
            flask_id: 2006,
        }
    }
}

impl ItemActionHandler for PotionHandler {
    fn on_use(&self, ctx: &ItemActionContext) -> ItemActionResult {
        // Would check player level and vocation
        // Then heal the player and replace potion with flask
        ItemActionResult::Transformed { new_item_id: self.flask_id }
    }

    fn on_use_with(&self, ctx: &ItemActionContext, _target: &Item) -> ItemActionResult {
        ItemActionResult::Failed("Use this potion on yourself.".to_string())
    }

    fn on_move(&self, _ctx: &ItemActionContext) -> ItemActionResult {
        ItemActionResult::Success
    }
}

/// Handler for runes
pub struct RuneHandler {
    /// Rune name
    pub name: String,
    /// Spell ID to cast
    pub spell_id: String,
    /// Required magic level
    pub magic_level: u16,
    /// Required level
    pub level: u16,
    /// Whether rune is aggressive (affects skulls)
    pub aggressive: bool,
    /// Needs target
    pub needs_target: bool,
    /// Vocations (bitmask)
    pub vocation_mask: u32,
}

impl RuneHandler {
    pub fn sudden_death() -> Self {
        Self {
            name: "sudden death rune".to_string(),
            spell_id: "exori mort".to_string(),
            magic_level: 15,
            level: 45,
            aggressive: true,
            needs_target: true,
            vocation_mask: 0xC, // Sorcerer, Druid
        }
    }

    pub fn ultimate_healing() -> Self {
        Self {
            name: "ultimate healing rune".to_string(),
            spell_id: "exura vita".to_string(),
            magic_level: 12,
            level: 24,
            aggressive: false,
            needs_target: true,
            vocation_mask: 0xFFFF,
        }
    }
}

impl ItemActionHandler for RuneHandler {
    fn on_use(&self, ctx: &ItemActionContext) -> ItemActionResult {
        if self.needs_target {
            ItemActionResult::Failed("You need to use this rune on a target.".to_string())
        } else {
            // Cast spell centered on player
            ItemActionResult::Consumed { remaining: 0 }
        }
    }

    fn on_use_with(&self, ctx: &ItemActionContext, _target: &Item) -> ItemActionResult {
        ItemActionResult::Failed("Use this rune on a creature.".to_string())
    }

    fn on_move(&self, _ctx: &ItemActionContext) -> ItemActionResult {
        ItemActionResult::Success
    }
}

/// Handler for food items
pub struct FoodHandler {
    /// Regeneration time in seconds
    pub regen_time: u32,
}

impl FoodHandler {
    pub fn meat() -> Self { Self { regen_time: 180 } }
    pub fn fish() -> Self { Self { regen_time: 144 } }
    pub fn ham() -> Self { Self { regen_time: 360 } }
    pub fn dragon_ham() -> Self { Self { regen_time: 720 } }
    pub fn brown_bread() -> Self { Self { regen_time: 168 } }
    pub fn apple() -> Self { Self { regen_time: 72 } }
}

impl ItemActionHandler for FoodHandler {
    fn on_use(&self, ctx: &ItemActionContext) -> ItemActionResult {
        // Check if player is not stuffed
        // Add regeneration time
        ItemActionResult::Consumed { remaining: 0 }
    }

    fn on_use_with(&self, _ctx: &ItemActionContext, _target: &Item) -> ItemActionResult {
        ItemActionResult::Failed("You cannot use this item that way.".to_string())
    }

    fn on_move(&self, _ctx: &ItemActionContext) -> ItemActionResult {
        ItemActionResult::Success
    }
}

/// Handler for doors
pub struct DoorHandler {
    /// Open door item ID
    pub open_id: u16,
    /// Closed door item ID
    pub closed_id: u16,
    /// Required level to open
    pub level: Option<u16>,
    /// Required quest storage key and value
    pub quest_key: Option<(u32, i32)>,
    /// Required key item ID
    pub key_id: Option<u16>,
}

impl ItemActionHandler for DoorHandler {
    fn on_use(&self, ctx: &ItemActionContext) -> ItemActionResult {
        // Check requirements and toggle door state
        ItemActionResult::Transformed { new_item_id: self.open_id }
    }

    fn on_use_with(&self, ctx: &ItemActionContext, target: &Item) -> ItemActionResult {
        // Check if target is the right key
        if let Some(key_id) = self.key_id {
            if target.item_type_id == key_id {
                return ItemActionResult::Transformed { new_item_id: self.open_id };
            }
        }
        ItemActionResult::Failed("This door is locked.".to_string())
    }

    fn on_move(&self, _ctx: &ItemActionContext) -> ItemActionResult {
        ItemActionResult::Failed("You cannot move a door.".to_string())
    }
}

/// Registry for item action handlers
pub struct ItemActionRegistry {
    /// Handlers by item type ID
    by_item_id: HashMap<u16, Arc<dyn ItemActionHandler>>,
    /// Handlers by action ID
    by_action_id: HashMap<u16, Arc<dyn ItemActionHandler>>,
    /// Handlers by unique ID
    by_unique_id: HashMap<u16, Arc<dyn ItemActionHandler>>,
    /// Default handler
    default: Arc<dyn ItemActionHandler>,
}

impl ItemActionRegistry {
    pub fn new() -> Self {
        Self {
            by_item_id: HashMap::new(),
            by_action_id: HashMap::new(),
            by_unique_id: HashMap::new(),
            default: Arc::new(DefaultItemHandler),
        }
    }

    /// Register handler for item type ID
    pub fn register_item(&mut self, item_id: u16, handler: Arc<dyn ItemActionHandler>) {
        self.by_item_id.insert(item_id, handler);
    }

    /// Register handler for action ID
    pub fn register_action(&mut self, action_id: u16, handler: Arc<dyn ItemActionHandler>) {
        self.by_action_id.insert(action_id, handler);
    }

    /// Register handler for unique ID
    pub fn register_unique(&mut self, unique_id: u16, handler: Arc<dyn ItemActionHandler>) {
        self.by_unique_id.insert(unique_id, handler);
    }

    /// Get handler for an item
    pub fn get_handler(&self, item: &Item) -> &dyn ItemActionHandler {
        // Check unique ID first (highest priority)
        if item.unique_action_id > 0 {
            if let Some(handler) = self.by_unique_id.get(&item.unique_action_id) {
                return handler.as_ref();
            }
        }

        // Check action ID
        if item.action_id > 0 {
            if let Some(handler) = self.by_action_id.get(&item.action_id) {
                return handler.as_ref();
            }
        }

        // Check item type ID
        if let Some(handler) = self.by_item_id.get(&item.item_type_id) {
            return handler.as_ref();
        }

        // Return default
        self.default.as_ref()
    }

    /// Register default handlers
    pub fn register_defaults(&mut self) {
        // Rope (2120)
        self.register_item(2120, Arc::new(RopeHandler));

        // Shovel (2554)
        self.register_item(2554, Arc::new(ShovelHandler));

        // Health potions
        self.register_item(7618, Arc::new(PotionHandler::health_potion()));
        self.register_item(7588, Arc::new(PotionHandler::great_health_potion()));
        self.register_item(7591, Arc::new(PotionHandler::great_health_potion()));
        self.register_item(8472, Arc::new(PotionHandler::ultimate_health_potion()));

        // Mana potions
        self.register_item(7620, Arc::new(PotionHandler::mana_potion()));
        self.register_item(7589, Arc::new(PotionHandler::mana_potion()));
        self.register_item(7590, Arc::new(PotionHandler::mana_potion()));
        self.register_item(8473, Arc::new(PotionHandler::mana_potion()));

        // Runes
        self.register_item(2268, Arc::new(RuneHandler::sudden_death()));
        self.register_item(2273, Arc::new(RuneHandler::ultimate_healing()));

        // Food
        self.register_item(2666, Arc::new(FoodHandler::meat()));
        self.register_item(2667, Arc::new(FoodHandler::fish()));
        self.register_item(2671, Arc::new(FoodHandler::ham()));
        self.register_item(2689, Arc::new(FoodHandler::brown_bread()));
        self.register_item(2695, Arc::new(FoodHandler::apple()));
        self.register_item(2672, Arc::new(FoodHandler::dragon_ham()));
    }
}

impl Default for ItemActionRegistry {
    fn default() -> Self {
        let mut registry = Self::new();
        registry.register_defaults();
        registry
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_action_context() {
        let ctx = ItemActionContext::new(
            Uuid::new_v4(),
            1,
            2120,
            Position::new(100, 100, 7),
        );
        assert_eq!(ctx.item_type_id, 2120);
    }

    #[test]
    fn test_registry() {
        let registry = ItemActionRegistry::default();
        let item = Item::new(2120); // Rope
        let handler = registry.get_handler(&item);
        // Handler should exist
    }

    #[test]
    fn test_potion_handler() {
        let handler = PotionHandler::health_potion();
        let ctx = ItemActionContext::new(Uuid::new_v4(), 1, 7618, Position::new(100, 100, 7));
        let result = handler.on_use(&ctx);
        assert!(matches!(result, ItemActionResult::Transformed { .. }));
    }
}
