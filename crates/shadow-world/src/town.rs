//! Town system - town definitions and management

use crate::position::Position;
use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Town definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Town {
    pub id: u32,
    pub name: String,
    pub temple_position: Position,
}

impl Town {
    pub fn new(id: u32, name: String, temple_position: Position) -> Self {
        Self {
            id,
            name,
            temple_position,
        }
    }
}

/// Town manager
pub struct TownManager {
    towns: HashMap<u32, Town>,
    name_map: HashMap<String, u32>,
}

impl TownManager {
    pub fn new() -> Self {
        Self {
            towns: HashMap::new(),
            name_map: HashMap::new(),
        }
    }

    /// Add a town
    pub fn add_town(&mut self, town: Town) {
        self.name_map.insert(town.name.to_lowercase(), town.id);
        self.towns.insert(town.id, town);
    }

    /// Get town by ID
    pub fn get(&self, id: u32) -> Option<&Town> {
        self.towns.get(&id)
    }

    /// Get town by name
    pub fn get_by_name(&self, name: &str) -> Option<&Town> {
        self.name_map
            .get(&name.to_lowercase())
            .and_then(|id| self.towns.get(id))
    }

    /// Get all towns
    pub fn all(&self) -> &HashMap<u32, Town> {
        &self.towns
    }

    /// Get town count
    pub fn count(&self) -> usize {
        self.towns.len()
    }

    /// Load towns from XML
    pub fn load_xml(&mut self, path: &str) -> Result<()> {
        info!("Loading towns from: {}", path);
        Ok(())
    }

    /// Get default spawn town (usually ID 1)
    pub fn get_default(&self) -> Option<&Town> {
        self.towns.get(&1)
    }
}

impl Default for TownManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_town_creation() {
        let town = Town::new(1, "Test Town".to_string(), Position::new(100, 100, 7));
        assert_eq!(town.id, 1);
        assert_eq!(town.name, "Test Town");
    }

    #[test]
    fn test_town_manager() {
        let mut manager = TownManager::new();
        let town = Town::new(1, "Thais".to_string(), Position::new(100, 100, 7));
        manager.add_town(town);

        assert_eq!(manager.count(), 1);
        assert!(manager.get(1).is_some());
        assert!(manager.get_by_name("thais").is_some());
    }
}
