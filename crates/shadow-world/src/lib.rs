//! Shadow OT World Management
//!
//! Handles all world-related systems: maps, tiles, creatures, items,
//! spawns, pathfinding, and spatial queries.

pub mod actions;
pub mod creature;
pub mod house;
pub mod item;
pub mod map;
pub mod npc;
pub mod otb;
pub mod otbm;
pub mod pathfinding;
pub mod position;
pub mod spawn;
pub mod tile;
pub mod town;

// Re-exports
pub use actions::{ItemActionRegistry, ItemActionHandler, ItemActionResult, ItemActionContext};
pub use creature::{Creature, CreatureType, Monster, MonsterLoader};
pub use house::{House, HouseManager};
pub use item::{Item, ItemLoader, ItemType};
pub use map::{Map, MapLayer};
pub use npc::{Npc, NpcLoader};
pub use otb::OtbLoader;
pub use otbm::OtbmLoader;
pub use pathfinding::{Pathfinder, PathResult};
pub use position::{Direction, Position};
pub use spawn::{SpawnManager, SpawnPoint};
pub use tile::{SharedTile, Tile, TileFlags};
pub use town::{Town, TownManager};

use thiserror::Error;

pub type Result<T> = std::result::Result<T, WorldError>;

#[derive(Error, Debug)]
pub enum WorldError {
    #[error("Map error: {0}")]
    Map(String),

    #[error("Failed to load OTBM: {0}")]
    OtbmLoad(String),

    #[error("Failed to load OTB: {0}")]
    OtbLoad(String),

    #[error("Position out of bounds: {0:?}")]
    OutOfBounds(Position),

    #[error("Tile not found at {0:?}")]
    TileNotFound(Position),

    #[error("Creature not found: {0}")]
    CreatureNotFound(u32),

    #[error("Item not found: {0}")]
    ItemNotFound(u32),

    #[error("Path not found from {0:?} to {1:?}")]
    PathNotFound(Position, Position),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("XML parse error: {0}")]
    XmlParse(String),
}

/// World dimensions
pub const MAP_MAX_X: u16 = 65535;
pub const MAP_MAX_Y: u16 = 65535;
pub const MAP_MAX_Z: u8 = 15;
pub const MAP_GROUND_FLOOR: u8 = 7;

/// Viewport dimensions (what the client sees)
pub const MAP_VIEW_WIDTH: u8 = 18;
pub const MAP_VIEW_HEIGHT: u8 = 14;
pub const MAP_MAX_VIEW_DISTANCE: u8 = 11;

/// Stack limits
pub const MAX_STACK_SIZE: u8 = 100;
pub const MAX_THINGS_PER_TILE: u8 = 10;
