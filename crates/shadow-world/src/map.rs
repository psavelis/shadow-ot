//! Map management - handles the game world grid and sectors

use crate::item::Item;
use crate::position::{Direction, Position};
use crate::tile::{SharedTile, Tile, TileFlags};
use crate::{Result, WorldError};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Sector size in tiles (16x16 is standard for Tibia)
pub const SECTOR_SIZE: u16 = 16;

/// A map sector containing tiles
#[derive(Debug)]
pub struct MapSector {
    /// Sector position (in sector coordinates)
    pub sector_x: u16,
    pub sector_y: u16,
    pub floor: u8,
    /// Tiles in this sector (indexed by local x,y)
    tiles: HashMap<(u8, u8), SharedTile>,
}

impl MapSector {
    pub fn new(sector_x: u16, sector_y: u16, floor: u8) -> Self {
        Self {
            sector_x,
            sector_y,
            floor,
            tiles: HashMap::new(),
        }
    }

    /// Get tile at local position
    pub fn get_tile(&self, local_x: u8, local_y: u8) -> Option<SharedTile> {
        self.tiles.get(&(local_x, local_y)).cloned()
    }

    /// Set tile at local position
    pub fn set_tile(&mut self, local_x: u8, local_y: u8, tile: SharedTile) {
        self.tiles.insert((local_x, local_y), tile);
    }

    /// Remove tile at local position
    pub fn remove_tile(&mut self, local_x: u8, local_y: u8) -> Option<SharedTile> {
        self.tiles.remove(&(local_x, local_y))
    }

    /// Get number of tiles in this sector
    pub fn tile_count(&self) -> usize {
        self.tiles.len()
    }

    /// Check if sector is empty
    pub fn is_empty(&self) -> bool {
        self.tiles.is_empty()
    }

    /// Iterate over all tiles
    pub fn tiles(&self) -> impl Iterator<Item = (&(u8, u8), &SharedTile)> {
        self.tiles.iter()
    }

    /// Convert world position to local position
    pub fn to_local(world_x: u16, world_y: u16) -> (u8, u8) {
        (
            (world_x % SECTOR_SIZE) as u8,
            (world_y % SECTOR_SIZE) as u8,
        )
    }

    /// Convert local position to world position
    pub fn to_world(&self, local_x: u8, local_y: u8) -> Position {
        Position::new(
            self.sector_x * SECTOR_SIZE + local_x as u16,
            self.sector_y * SECTOR_SIZE + local_y as u16,
            self.floor,
        )
    }
}

/// Map layer containing all sectors for a single floor
#[derive(Debug)]
pub struct MapLayer {
    pub floor: u8,
    sectors: HashMap<(u16, u16), Arc<RwLock<MapSector>>>,
}

impl MapLayer {
    pub fn new(floor: u8) -> Self {
        Self {
            floor,
            sectors: HashMap::new(),
        }
    }

    /// Get sector at sector coordinates
    pub fn get_sector(&self, sector_x: u16, sector_y: u16) -> Option<Arc<RwLock<MapSector>>> {
        self.sectors.get(&(sector_x, sector_y)).cloned()
    }

    /// Get or create sector
    pub fn get_or_create_sector(&mut self, sector_x: u16, sector_y: u16) -> Arc<RwLock<MapSector>> {
        self.sectors
            .entry((sector_x, sector_y))
            .or_insert_with(|| Arc::new(RwLock::new(MapSector::new(sector_x, sector_y, self.floor))))
            .clone()
    }

    /// Get tile at world position
    pub async fn get_tile(&self, x: u16, y: u16) -> Option<SharedTile> {
        let (sector_x, sector_y) = (x / SECTOR_SIZE, y / SECTOR_SIZE);
        let (local_x, local_y) = MapSector::to_local(x, y);

        if let Some(sector) = self.get_sector(sector_x, sector_y) {
            let sector = sector.read().await;
            sector.get_tile(local_x, local_y)
        } else {
            None
        }
    }

    /// Set tile at world position
    pub async fn set_tile(&mut self, x: u16, y: u16, tile: SharedTile) {
        let (sector_x, sector_y) = (x / SECTOR_SIZE, y / SECTOR_SIZE);
        let (local_x, local_y) = MapSector::to_local(x, y);

        let sector = self.get_or_create_sector(sector_x, sector_y);
        let mut sector = sector.write().await;
        sector.set_tile(local_x, local_y, tile);
    }

    /// Get number of sectors
    pub fn sector_count(&self) -> usize {
        self.sectors.len()
    }

    /// Iterate over all sectors
    pub fn sectors(&self) -> impl Iterator<Item = (&(u16, u16), &Arc<RwLock<MapSector>>)> {
        self.sectors.iter()
    }
}

/// The complete game map
#[derive(Debug)]
pub struct Map {
    /// Map name
    pub name: String,
    /// Map description
    pub description: String,
    /// Map author
    pub author: String,
    /// Map width
    pub width: u16,
    /// Map height
    pub height: u16,
    /// Spawn position
    pub spawn_position: Position,
    /// Temple position (respawn point)
    pub temple_position: Position,
    /// Map layers (one per floor 0-15)
    layers: Vec<MapLayer>,
    /// Waypoints for pathfinding
    waypoints: HashMap<String, Position>,
    /// House tiles (position -> house_id)
    house_tiles: HashMap<Position, u32>,
}

impl Map {
    pub fn new(name: String) -> Self {
        let mut layers = Vec::with_capacity(16);
        for floor in 0..16 {
            layers.push(MapLayer::new(floor));
        }

        Self {
            name,
            description: String::new(),
            author: String::new(),
            width: 0,
            height: 0,
            spawn_position: Position::new(100, 100, 7),
            temple_position: Position::new(100, 100, 7),
            layers,
            waypoints: HashMap::new(),
            house_tiles: HashMap::new(),
        }
    }

    /// Get a tile at position
    pub async fn get_tile(&self, pos: &Position) -> Option<SharedTile> {
        if pos.z as usize >= self.layers.len() {
            return None;
        }
        self.layers[pos.z as usize].get_tile(pos.x, pos.y).await
    }

    /// Set a tile at position
    pub async fn set_tile(&mut self, pos: Position, tile: Tile) {
        if pos.z as usize >= self.layers.len() {
            return;
        }
        let shared_tile = Arc::new(RwLock::new(tile));
        self.layers[pos.z as usize]
            .set_tile(pos.x, pos.y, shared_tile)
            .await;
    }

    /// Create a new tile at position with ground item
    pub async fn create_tile(&mut self, pos: Position, ground_id: u16) -> SharedTile {
        let mut tile = Tile::new(pos);
        tile.set_ground(Item::new(ground_id));
        let shared_tile = Arc::new(RwLock::new(tile));

        if (pos.z as usize) < self.layers.len() {
            self.layers[pos.z as usize]
                .set_tile(pos.x, pos.y, shared_tile.clone())
                .await;
        }

        shared_tile
    }

    /// Get or create tile at position
    pub async fn get_or_create_tile(&mut self, pos: Position) -> SharedTile {
        if let Some(tile) = self.get_tile(&pos).await {
            return tile;
        }

        let tile = Tile::new(pos);
        let shared_tile = Arc::new(RwLock::new(tile));

        if (pos.z as usize) < self.layers.len() {
            self.layers[pos.z as usize]
                .set_tile(pos.x, pos.y, shared_tile.clone())
                .await;
        }

        shared_tile
    }

    /// Check if tile exists
    pub async fn has_tile(&self, pos: &Position) -> bool {
        self.get_tile(pos).await.is_some()
    }

    /// Check if position is walkable
    pub async fn is_walkable(&self, pos: &Position) -> bool {
        if let Some(tile) = self.get_tile(pos).await {
            let tile = tile.read().await;
            tile.is_walkable()
        } else {
            false
        }
    }

    /// Check if position blocks projectiles
    pub async fn blocks_projectile(&self, pos: &Position) -> bool {
        if let Some(tile) = self.get_tile(pos).await {
            let tile = tile.read().await;
            !tile.is_projectile_passable()
        } else {
            true
        }
    }

    /// Get tiles in viewport around position
    pub async fn get_viewport_tiles(
        &self,
        center: &Position,
        width: u8,
        height: u8,
    ) -> Vec<(Position, SharedTile)> {
        let mut tiles = Vec::new();
        let half_width = width as i32 / 2;
        let half_height = height as i32 / 2;

        for dy in -half_height..=half_height {
            for dx in -half_width..=half_width {
                let x = (center.x as i32 + dx).max(0) as u16;
                let y = (center.y as i32 + dy).max(0) as u16;
                let pos = Position::new(x, y, center.z);

                if let Some(tile) = self.get_tile(&pos).await {
                    tiles.push((pos, tile));
                }
            }
        }

        tiles
    }

    /// Get tiles in a range of floors for viewport
    pub async fn get_viewport_tiles_3d(
        &self,
        center: &Position,
        width: u8,
        height: u8,
    ) -> Vec<(Position, SharedTile)> {
        let mut tiles = Vec::new();

        // Determine floor range based on center position
        let (start_z, end_z) = if center.z > 7 {
            // Underground: show current floor and below
            (center.z.saturating_sub(2), center.z)
        } else {
            // Above ground: show floors above
            (center.z, (center.z + 2).min(7))
        };

        for z in start_z..=end_z {
            let view_center = Position::new(center.x, center.y, z);
            let floor_tiles = self.get_viewport_tiles(&view_center, width, height).await;
            tiles.extend(floor_tiles);
        }

        tiles
    }

    /// Add a creature to a tile
    pub async fn add_creature(&self, pos: &Position, creature_id: u32) -> Result<()> {
        if let Some(tile) = self.get_tile(pos).await {
            let mut tile = tile.write().await;
            tile.add_creature(creature_id);
            Ok(())
        } else {
            Err(WorldError::TileNotFound(*pos))
        }
    }

    /// Remove a creature from a tile
    pub async fn remove_creature(&self, pos: &Position, creature_id: u32) -> Result<bool> {
        if let Some(tile) = self.get_tile(pos).await {
            let mut tile = tile.write().await;
            Ok(tile.remove_creature(creature_id))
        } else {
            Err(WorldError::TileNotFound(*pos))
        }
    }

    /// Move a creature from one tile to another
    pub async fn move_creature(
        &self,
        from: &Position,
        to: &Position,
        creature_id: u32,
    ) -> Result<()> {
        // Verify destination is walkable
        if !self.is_walkable(to).await {
            return Err(WorldError::Map("Destination not walkable".to_string()));
        }

        // Remove from source
        self.remove_creature(from, creature_id).await?;

        // Add to destination
        self.add_creature(to, creature_id).await?;

        Ok(())
    }

    /// Add an item to a tile
    pub async fn add_item(&self, pos: &Position, item: Item) -> Result<()> {
        if let Some(tile) = self.get_tile(pos).await {
            let mut tile = tile.write().await;
            if tile.can_add_item() {
                tile.add_item(item);
                Ok(())
            } else {
                Err(WorldError::Map("Tile is full".to_string()))
            }
        } else {
            Err(WorldError::TileNotFound(*pos))
        }
    }

    /// Remove an item from a tile
    pub async fn remove_item(&self, pos: &Position, index: usize) -> Result<Item> {
        if let Some(tile) = self.get_tile(pos).await {
            let mut tile = tile.write().await;
            tile.remove_item(index)
                .ok_or_else(|| WorldError::ItemNotFound(index as u32))
        } else {
            Err(WorldError::TileNotFound(*pos))
        }
    }

    /// Add a waypoint
    pub fn add_waypoint(&mut self, name: String, position: Position) {
        self.waypoints.insert(name, position);
    }

    /// Get a waypoint
    pub fn get_waypoint(&self, name: &str) -> Option<&Position> {
        self.waypoints.get(name)
    }

    /// Get all waypoints
    pub fn waypoints(&self) -> &HashMap<String, Position> {
        &self.waypoints
    }

    /// Set house tile
    pub fn set_house_tile(&mut self, pos: Position, house_id: u32) {
        self.house_tiles.insert(pos, house_id);
    }

    /// Get house ID for a tile
    pub fn get_house_id(&self, pos: &Position) -> Option<u32> {
        self.house_tiles.get(pos).copied()
    }

    /// Check if position is in a house
    pub fn is_house(&self, pos: &Position) -> bool {
        self.house_tiles.contains_key(pos)
    }

    /// Get all tiles for a house
    pub fn get_house_tiles(&self, house_id: u32) -> Vec<Position> {
        self.house_tiles
            .iter()
            .filter(|(_, &id)| id == house_id)
            .map(|(pos, _)| *pos)
            .collect()
    }

    /// Get layer for a floor
    pub fn get_layer(&self, floor: u8) -> Option<&MapLayer> {
        self.layers.get(floor as usize)
    }

    /// Get mutable layer for a floor
    pub fn get_layer_mut(&mut self, floor: u8) -> Option<&mut MapLayer> {
        self.layers.get_mut(floor as usize)
    }

    /// Get total tile count across all layers
    pub async fn total_tile_count(&self) -> usize {
        let mut count = 0;
        for layer in &self.layers {
            for (_, sector) in layer.sectors() {
                let sector = sector.read().await;
                count += sector.tile_count();
            }
        }
        count
    }

    /// Get total sector count
    pub fn total_sector_count(&self) -> usize {
        self.layers.iter().map(|l| l.sector_count()).sum()
    }

    /// Get spectators in range of a position
    pub async fn get_spectators(
        &self,
        center: &Position,
        range_x: u8,
        range_y: u8,
        multi_floor: bool,
    ) -> Vec<u32> {
        let mut spectators = Vec::new();

        let floors = if multi_floor {
            if center.z > 7 {
                vec![center.z.saturating_sub(2), center.z.saturating_sub(1), center.z]
            } else {
                vec![center.z, (center.z + 1).min(7), (center.z + 2).min(7)]
            }
        } else {
            vec![center.z]
        };

        for z in floors {
            let min_x = center.x.saturating_sub(range_x as u16);
            let max_x = center.x.saturating_add(range_x as u16);
            let min_y = center.y.saturating_sub(range_y as u16);
            let max_y = center.y.saturating_add(range_y as u16);

            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    let pos = Position::new(x, y, z);
                    if let Some(tile) = self.get_tile(&pos).await {
                        let tile = tile.read().await;
                        spectators.extend(tile.creatures.iter().copied());
                    }
                }
            }
        }

        spectators
    }

    /// Clear all tiles (for map reload)
    pub fn clear(&mut self) {
        for layer in &mut self.layers {
            layer.sectors.clear();
        }
        self.waypoints.clear();
        self.house_tiles.clear();
    }

    /// Log map statistics
    pub async fn log_stats(&self) {
        let tile_count = self.total_tile_count().await;
        let sector_count = self.total_sector_count();
        let waypoint_count = self.waypoints.len();
        let house_tile_count = self.house_tiles.len();

        info!(
            "Map '{}' loaded: {} tiles in {} sectors, {} waypoints, {} house tiles",
            self.name, tile_count, sector_count, waypoint_count, house_tile_count
        );
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::new("Untitled".to_string())
    }
}

/// Thread-safe map wrapper
pub type SharedMap = Arc<RwLock<Map>>;

/// Create a shared map
pub fn create_shared_map(name: String) -> SharedMap {
    Arc::new(RwLock::new(Map::new(name)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_tile() {
        let mut map = Map::new("Test".to_string());
        let pos = Position::new(100, 100, 7);
        let tile = map.create_tile(pos, 100).await;

        let tile_read = tile.read().await;
        assert_eq!(tile_read.position, pos);
        assert!(tile_read.get_ground().is_some());
    }

    #[tokio::test]
    async fn test_sector_coordinates() {
        let (local_x, local_y) = MapSector::to_local(35, 47);
        assert_eq!(local_x, 3);  // 35 % 16 = 3
        assert_eq!(local_y, 15); // 47 % 16 = 15
    }

    #[tokio::test]
    async fn test_get_tile() {
        let mut map = Map::new("Test".to_string());
        let pos = Position::new(100, 100, 7);
        map.create_tile(pos, 100).await;

        assert!(map.has_tile(&pos).await);
        assert!(!map.has_tile(&Position::new(200, 200, 7)).await);
    }
}
