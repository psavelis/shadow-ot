//! House system - player housing management

use crate::position::Position;
use crate::{Result, WorldError};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// House access levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HouseAccessLevel {
    None = 0,
    Guest = 1,
    SubOwner = 2,
    Owner = 3,
}

/// House definition
#[derive(Debug, Clone)]
pub struct House {
    pub id: u32,
    pub name: String,
    pub owner_id: Option<u32>,
    pub paid_until: Option<i64>,
    pub rent: u64,
    pub size: u32,
    pub beds: u8,
    pub town_id: u32,
    pub entry: Position,
    pub tiles: Vec<Position>,
    pub doors: Vec<HouseDoor>,
    pub access_list: HashMap<u32, HouseAccessLevel>,
    pub guest_list: String,
    pub sub_owner_list: String,
    pub guild_id: Option<u32>,
    pub guild_rank: Option<u32>,
    pub transfer_to: Option<u32>,
    pub transfer_price: Option<u64>,
    pub bid_end: Option<i64>,
    pub highest_bidder: Option<u32>,
    pub highest_bid: Option<u64>,
}

impl House {
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            owner_id: None,
            paid_until: None,
            rent: 0,
            size: 0,
            beds: 0,
            town_id: 0,
            entry: Position::default(),
            tiles: Vec::new(),
            doors: Vec::new(),
            access_list: HashMap::new(),
            guest_list: String::new(),
            sub_owner_list: String::new(),
            guild_id: None,
            guild_rank: None,
            transfer_to: None,
            transfer_price: None,
            bid_end: None,
            highest_bidder: None,
            highest_bid: None,
        }
    }

    /// Check if house has an owner
    pub fn has_owner(&self) -> bool {
        self.owner_id.is_some()
    }

    /// Check if player owns the house
    pub fn is_owner(&self, player_id: u32) -> bool {
        self.owner_id == Some(player_id)
    }

    /// Check if player has access
    pub fn has_access(&self, player_id: u32) -> bool {
        if self.is_owner(player_id) {
            return true;
        }

        self.access_list
            .get(&player_id)
            .map(|&level| level != HouseAccessLevel::None)
            .unwrap_or(false)
    }

    /// Get access level for player
    pub fn get_access_level(&self, player_id: u32) -> HouseAccessLevel {
        if self.is_owner(player_id) {
            return HouseAccessLevel::Owner;
        }

        self.access_list
            .get(&player_id)
            .copied()
            .unwrap_or(HouseAccessLevel::None)
    }

    /// Set access level for player
    pub fn set_access(&mut self, player_id: u32, level: HouseAccessLevel) {
        if level == HouseAccessLevel::None {
            self.access_list.remove(&player_id);
        } else {
            self.access_list.insert(player_id, level);
        }
    }

    /// Check if position is inside house
    pub fn contains_position(&self, pos: &Position) -> bool {
        self.tiles.contains(pos)
    }

    /// Add a tile to the house
    pub fn add_tile(&mut self, pos: Position) {
        if !self.tiles.contains(&pos) {
            self.tiles.push(pos);
            self.size = self.tiles.len() as u32;
        }
    }

    /// Add a door
    pub fn add_door(&mut self, door: HouseDoor) {
        self.doors.push(door);
    }

    /// Get door at position
    pub fn get_door(&self, pos: &Position) -> Option<&HouseDoor> {
        self.doors.iter().find(|d| d.position == *pos)
    }

    /// Get door at position (mutable)
    pub fn get_door_mut(&mut self, pos: &Position) -> Option<&mut HouseDoor> {
        self.doors.iter_mut().find(|d| d.position == *pos)
    }

    /// Check if house is up for auction
    pub fn is_auction(&self) -> bool {
        self.bid_end.is_some()
    }

    /// Check if house is pending transfer
    pub fn is_transferring(&self) -> bool {
        self.transfer_to.is_some()
    }

    /// Calculate rent based on size
    pub fn calculate_rent(size: u32, beds: u8) -> u64 {
        // Base rent + per sqm + per bed
        let base = 500;
        let per_sqm = 10;
        let per_bed = 100;
        base + (size as u64 * per_sqm) + (beds as u64 * per_bed)
    }
}

/// House door definition
#[derive(Debug, Clone)]
pub struct HouseDoor {
    pub position: Position,
    pub door_id: u8,
    pub access_list: HashSet<u32>,
    pub locked: bool,
}

impl HouseDoor {
    pub fn new(position: Position, door_id: u8) -> Self {
        Self {
            position,
            door_id,
            access_list: HashSet::new(),
            locked: true,
        }
    }

    pub fn has_access(&self, player_id: u32) -> bool {
        self.access_list.contains(&player_id)
    }

    pub fn grant_access(&mut self, player_id: u32) {
        self.access_list.insert(player_id);
    }

    pub fn revoke_access(&mut self, player_id: u32) {
        self.access_list.remove(&player_id);
    }
}

/// House manager
pub struct HouseManager {
    houses: HashMap<u32, House>,
    position_map: HashMap<Position, u32>,
}

impl HouseManager {
    pub fn new() -> Self {
        Self {
            houses: HashMap::new(),
            position_map: HashMap::new(),
        }
    }

    /// Add a house
    pub fn add_house(&mut self, house: House) {
        let id = house.id;
        for pos in &house.tiles {
            self.position_map.insert(*pos, id);
        }
        self.houses.insert(id, house);
    }

    /// Get house by ID
    pub fn get(&self, id: u32) -> Option<&House> {
        self.houses.get(&id)
    }

    /// Get house by ID (mutable)
    pub fn get_mut(&mut self, id: u32) -> Option<&mut House> {
        self.houses.get_mut(&id)
    }

    /// Get house at position
    pub fn get_at_position(&self, pos: &Position) -> Option<&House> {
        self.position_map
            .get(pos)
            .and_then(|id| self.houses.get(id))
    }

    /// Get house at position (mutable)
    pub fn get_at_position_mut(&mut self, pos: &Position) -> Option<&mut House> {
        let id = self.position_map.get(pos).copied();
        id.and_then(move |id| self.houses.get_mut(&id))
    }

    /// Get houses owned by player
    pub fn get_player_houses(&self, player_id: u32) -> Vec<&House> {
        self.houses
            .values()
            .filter(|h| h.owner_id == Some(player_id))
            .collect()
    }

    /// Get houses in town
    pub fn get_town_houses(&self, town_id: u32) -> Vec<&House> {
        self.houses
            .values()
            .filter(|h| h.town_id == town_id)
            .collect()
    }

    /// Get unowned houses
    pub fn get_available_houses(&self) -> Vec<&House> {
        self.houses
            .values()
            .filter(|h| h.owner_id.is_none())
            .collect()
    }

    /// Get houses up for auction
    pub fn get_auction_houses(&self) -> Vec<&House> {
        self.houses
            .values()
            .filter(|h| h.is_auction())
            .collect()
    }

    /// Transfer house ownership
    pub fn transfer_ownership(&mut self, house_id: u32, new_owner_id: u32) -> bool {
        if let Some(house) = self.houses.get_mut(&house_id) {
            house.owner_id = Some(new_owner_id);
            house.transfer_to = None;
            house.transfer_price = None;
            house.access_list.clear();
            house.guest_list.clear();
            house.sub_owner_list.clear();
            true
        } else {
            false
        }
    }

    /// Remove house ownership
    pub fn remove_ownership(&mut self, house_id: u32) -> bool {
        if let Some(house) = self.houses.get_mut(&house_id) {
            house.owner_id = None;
            house.paid_until = None;
            house.access_list.clear();
            house.guest_list.clear();
            house.sub_owner_list.clear();
            house.guild_id = None;
            house.guild_rank = None;
            true
        } else {
            false
        }
    }

    /// Load houses from XML
    pub fn load_xml(&mut self, path: &str) -> Result<()> {
        info!("Loading houses from: {}", path);
        Ok(())
    }

    /// Get total house count
    pub fn house_count(&self) -> usize {
        self.houses.len()
    }

    /// Get owned house count
    pub fn owned_count(&self) -> usize {
        self.houses.values().filter(|h| h.has_owner()).count()
    }

    /// Get all houses
    pub fn all(&self) -> &HashMap<u32, House> {
        &self.houses
    }
}

impl Default for HouseManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_house_creation() {
        let house = House::new(1, "Test House".to_string());
        assert_eq!(house.id, 1);
        assert!(!house.has_owner());
    }

    #[test]
    fn test_house_access() {
        let mut house = House::new(1, "Test House".to_string());
        house.owner_id = Some(100);

        assert!(house.is_owner(100));
        assert!(house.has_access(100));
        assert!(!house.has_access(200));

        house.set_access(200, HouseAccessLevel::Guest);
        assert!(house.has_access(200));
        assert_eq!(house.get_access_level(200), HouseAccessLevel::Guest);
    }

    #[test]
    fn test_house_tiles() {
        let mut house = House::new(1, "Test House".to_string());
        let pos = Position::new(100, 100, 7);

        house.add_tile(pos);
        assert!(house.contains_position(&pos));
        assert_eq!(house.size, 1);
    }
}
