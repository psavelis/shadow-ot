//! Spawn system - manages creature spawning

use crate::creature::{Creature, CreatureType, Monster, MonsterLoader};
use crate::position::Position;
use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Spawn point configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnPoint {
    /// Center position
    pub position: Position,
    /// Spawn radius
    pub radius: u8,
    /// Spawn interval in seconds
    pub interval: u32,
    /// Monsters that can spawn here
    pub monsters: Vec<SpawnMonster>,
    /// Whether this spawn is active
    pub active: bool,
    /// Last spawn time (milliseconds)
    pub last_spawn: u64,
    /// Currently spawned creatures
    pub spawned_creatures: Vec<u32>,
}

impl SpawnPoint {
    pub fn new(position: Position, radius: u8, interval: u32) -> Self {
        Self {
            position,
            radius,
            interval,
            monsters: Vec::new(),
            active: true,
            last_spawn: 0,
            spawned_creatures: Vec::new(),
        }
    }

    /// Add a monster type to this spawn
    pub fn add_monster(&mut self, name: String, count: u8) {
        self.monsters.push(SpawnMonster {
            name,
            count,
            spawned: 0,
        });
    }

    /// Check if spawn needs to create new creatures
    pub fn needs_spawn(&self, current_time: u64) -> bool {
        if !self.active {
            return false;
        }

        // Check interval
        if current_time < self.last_spawn + (self.interval as u64 * 1000) {
            return false;
        }

        // Check if any monster type needs spawning
        self.monsters.iter().any(|m| m.spawned < m.count)
    }

    /// Get spawn positions within radius
    pub fn get_spawn_positions(&self) -> Vec<Position> {
        let mut positions = Vec::new();
        let r = self.radius as i32;

        for dx in -r..=r {
            for dy in -r..=r {
                if dx * dx + dy * dy <= r * r {
                    let x = (self.position.x as i32 + dx).max(0) as u16;
                    let y = (self.position.y as i32 + dy).max(0) as u16;
                    positions.push(Position::new(x, y, self.position.z));
                }
            }
        }

        positions
    }

    /// Record a creature as spawned
    pub fn record_spawn(&mut self, monster_name: &str, creature_id: u32) {
        self.spawned_creatures.push(creature_id);
        if let Some(monster) = self.monsters.iter_mut().find(|m| m.name.eq_ignore_ascii_case(monster_name)) {
            monster.spawned += 1;
        }
    }

    /// Record a creature as dead
    pub fn record_death(&mut self, monster_name: &str, creature_id: u32) {
        self.spawned_creatures.retain(|&id| id != creature_id);
        if let Some(monster) = self.monsters.iter_mut().find(|m| m.name.eq_ignore_ascii_case(monster_name)) {
            monster.spawned = monster.spawned.saturating_sub(1);
        }
    }
}

/// Monster spawn configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnMonster {
    /// Monster name
    pub name: String,
    /// Maximum count
    pub count: u8,
    /// Currently spawned
    pub spawned: u8,
}

/// Spawn manager handles all spawn points
pub struct SpawnManager {
    /// All spawn points
    spawns: Vec<SpawnPoint>,
    /// Monster loader reference
    monster_loader: Arc<RwLock<MonsterLoader>>,
    /// Spawn interval check (milliseconds)
    check_interval: u64,
    /// Last check time
    last_check: u64,
}

impl SpawnManager {
    pub fn new(monster_loader: Arc<RwLock<MonsterLoader>>) -> Self {
        Self {
            spawns: Vec::new(),
            monster_loader,
            check_interval: 1000, // Check every second
            last_check: 0,
        }
    }

    /// Add a spawn point
    pub fn add_spawn(&mut self, spawn: SpawnPoint) {
        self.spawns.push(spawn);
    }

    /// Remove spawn at position
    pub fn remove_spawn(&mut self, position: &Position) -> bool {
        let initial_len = self.spawns.len();
        self.spawns.retain(|s| s.position != *position);
        self.spawns.len() != initial_len
    }

    /// Get spawn at position
    pub fn get_spawn(&self, position: &Position) -> Option<&SpawnPoint> {
        self.spawns.iter().find(|s| s.position == *position)
    }

    /// Get mutable spawn at position
    pub fn get_spawn_mut(&mut self, position: &Position) -> Option<&mut SpawnPoint> {
        self.spawns.iter_mut().find(|s| s.position == *position)
    }

    /// Get all spawns
    pub fn spawns(&self) -> &[SpawnPoint] {
        &self.spawns
    }

    /// Process spawns and return creatures to spawn
    pub async fn tick(&mut self, current_time: u64) -> Vec<SpawnRequest> {
        // Check if enough time has passed
        if current_time < self.last_check + self.check_interval {
            return Vec::new();
        }
        self.last_check = current_time;

        let mut requests = Vec::new();
        let monster_loader = self.monster_loader.read().await;

        for spawn in &mut self.spawns {
            if !spawn.needs_spawn(current_time) {
                continue;
            }

            // Get spawn positions
            let positions = spawn.get_spawn_positions();
            if positions.is_empty() {
                continue;
            }

            // Try to spawn each monster type
            for monster_config in &spawn.monsters {
                if monster_config.spawned >= monster_config.count {
                    continue;
                }

                // Check if monster type exists
                if monster_loader.get(&monster_config.name).is_none() {
                    warn!("Unknown monster type: {}", monster_config.name);
                    continue;
                }

                // Create spawn request
                let spawn_count = monster_config.count - monster_config.spawned;
                for _ in 0..spawn_count {
                    // Pick a random position
                    let pos_index = rand::random::<usize>() % positions.len();
                    let position = positions[pos_index];

                    requests.push(SpawnRequest {
                        monster_name: monster_config.name.clone(),
                        position,
                        spawn_position: spawn.position,
                    });
                }
            }

            spawn.last_spawn = current_time;
        }

        requests
    }

    /// Record a successful spawn
    pub fn record_spawn(&mut self, spawn_position: &Position, monster_name: &str, creature_id: u32) {
        if let Some(spawn) = self.get_spawn_mut(spawn_position) {
            spawn.record_spawn(monster_name, creature_id);
        }
    }

    /// Record a creature death
    pub fn record_death(&mut self, spawn_position: &Position, monster_name: &str, creature_id: u32) {
        if let Some(spawn) = self.get_spawn_mut(spawn_position) {
            spawn.record_death(monster_name, creature_id);
        }
    }

    /// Load spawns from XML file
    pub fn load_xml(&mut self, path: &str) -> Result<()> {
        info!("Loading spawns from: {}", path);
        // XML parsing implementation
        Ok(())
    }

    /// Get total spawn count
    pub fn spawn_count(&self) -> usize {
        self.spawns.len()
    }

    /// Get total monster count configured
    pub fn total_monster_count(&self) -> usize {
        self.spawns.iter()
            .flat_map(|s| &s.monsters)
            .map(|m| m.count as usize)
            .sum()
    }

    /// Get currently spawned creature count
    pub fn spawned_creature_count(&self) -> usize {
        self.spawns.iter()
            .map(|s| s.spawned_creatures.len())
            .sum()
    }

    /// Activate all spawns
    pub fn activate_all(&mut self) {
        for spawn in &mut self.spawns {
            spawn.active = true;
        }
    }

    /// Deactivate all spawns
    pub fn deactivate_all(&mut self) {
        for spawn in &mut self.spawns {
            spawn.active = false;
        }
    }

    /// Clear all spawns
    pub fn clear(&mut self) {
        self.spawns.clear();
    }
}

/// Spawn request to be processed by the game engine
#[derive(Debug, Clone)]
pub struct SpawnRequest {
    /// Monster type name
    pub monster_name: String,
    /// Position to spawn at
    pub position: Position,
    /// Spawn point position (for tracking)
    pub spawn_position: Position,
}

/// NPC spawn point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcSpawn {
    pub name: String,
    pub position: Position,
    pub direction: u8,
}

/// NPC spawn manager
pub struct NpcSpawnManager {
    spawns: Vec<NpcSpawn>,
}

impl NpcSpawnManager {
    pub fn new() -> Self {
        Self {
            spawns: Vec::new(),
        }
    }

    pub fn add_spawn(&mut self, spawn: NpcSpawn) {
        self.spawns.push(spawn);
    }

    pub fn spawns(&self) -> &[NpcSpawn] {
        &self.spawns
    }

    pub fn load_xml(&mut self, path: &str) -> Result<()> {
        info!("Loading NPC spawns from: {}", path);
        Ok(())
    }
}

impl Default for NpcSpawnManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_point() {
        let mut spawn = SpawnPoint::new(Position::new(100, 100, 7), 5, 60);
        spawn.add_monster("Rat".to_string(), 3);

        assert_eq!(spawn.monsters.len(), 1);
        assert!(spawn.needs_spawn(0));
    }

    #[test]
    fn test_spawn_positions() {
        let spawn = SpawnPoint::new(Position::new(100, 100, 7), 2, 60);
        let positions = spawn.get_spawn_positions();

        assert!(!positions.is_empty());
        assert!(positions.contains(&Position::new(100, 100, 7)));
    }
}
