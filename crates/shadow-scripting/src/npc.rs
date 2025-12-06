//! NPC System
//!
//! Handles NPC definitions, behaviors, and interactions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::dialog::DialogHandler;
use crate::shop::Shop;
use crate::{Result, ScriptError};

/// NPC position in the world
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Position {
    pub x: u16,
    pub y: u16,
    pub z: u8,
}

impl Position {
    pub fn new(x: u16, y: u16, z: u8) -> Self {
        Self { x, y, z }
    }
}

/// Direction the NPC is facing
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Default for Direction {
    fn default() -> Self {
        Self::South
    }
}

/// NPC behavior type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NpcBehavior {
    /// Stands still
    Stationary,
    /// Walks around spawn point
    Wandering { radius: u8 },
    /// Follows a set path
    Patrol { path: Vec<Position> },
    /// Follows a schedule
    Scheduled { schedule: String },
}

impl Default for NpcBehavior {
    fn default() -> Self {
        Self::Stationary
    }
}

/// An NPC in the game world
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Npc {
    /// Unique identifier
    pub id: Uuid,
    /// NPC name
    pub name: String,
    /// Look type (outfit)
    pub look_type: u16,
    /// Head color
    pub look_head: u8,
    /// Body color
    pub look_body: u8,
    /// Legs color
    pub look_legs: u8,
    /// Feet color
    pub look_feet: u8,
    /// Addons
    pub look_addons: u8,
    /// Mount look type
    pub look_mount: u16,
    /// Position in world
    pub position: Position,
    /// Direction facing
    pub direction: Direction,
    /// Behavior type
    pub behavior: NpcBehavior,
    /// Health (for attackable NPCs)
    pub health: u32,
    /// Maximum health
    pub max_health: u32,
    /// Whether this NPC can be attacked
    pub attackable: bool,
    /// Whether this NPC can walk
    pub walkable: bool,
    /// Walk interval in ms
    pub walk_interval: u32,
    /// Script file to load
    pub script: Option<String>,
    /// Associated shop
    pub shop: Option<String>,
    /// Focus range (how far player can be to talk)
    pub focus_range: u8,
    /// Voice lines
    pub voices: Vec<NpcVoice>,
    /// Custom parameters
    #[serde(default)]
    pub parameters: HashMap<String, String>,
}

impl Npc {
    /// Create a new NPC with basic info
    pub fn new(name: impl Into<String>, position: Position) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            look_type: 128, // Default citizen
            look_head: 0,
            look_body: 0,
            look_legs: 0,
            look_feet: 0,
            look_addons: 0,
            look_mount: 0,
            position,
            direction: Direction::South,
            behavior: NpcBehavior::Stationary,
            health: 100,
            max_health: 100,
            attackable: false,
            walkable: true,
            walk_interval: 2000,
            script: None,
            shop: None,
            focus_range: 4,
            voices: Vec::new(),
            parameters: HashMap::new(),
        }
    }

    /// Set the NPC's outfit
    pub fn with_outfit(mut self, look_type: u16, head: u8, body: u8, legs: u8, feet: u8) -> Self {
        self.look_type = look_type;
        self.look_head = head;
        self.look_body = body;
        self.look_legs = legs;
        self.look_feet = feet;
        self
    }

    /// Set the NPC's script
    pub fn with_script(mut self, script: impl Into<String>) -> Self {
        self.script = Some(script.into());
        self
    }

    /// Set the NPC's shop
    pub fn with_shop(mut self, shop: impl Into<String>) -> Self {
        self.shop = Some(shop.into());
        self
    }

    /// Add a voice line
    pub fn add_voice(mut self, text: String, interval: u32, chance: u8) -> Self {
        self.voices.push(NpcVoice {
            text,
            interval,
            chance,
            yell: false,
        });
        self
    }
}

/// NPC voice line (periodic sayings)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcVoice {
    /// Text to say
    pub text: String,
    /// Interval between potential voice (ms)
    pub interval: u32,
    /// Chance to say (0-100)
    pub chance: u8,
    /// Whether to yell
    pub yell: bool,
}

/// Handles NPC interactions for a single NPC instance
pub struct NpcHandler {
    /// The NPC data
    pub npc: Npc,
    /// Current focus (player talking to NPC)
    pub focus: Option<Uuid>,
    /// Dialog handler
    pub dialog: DialogHandler,
    /// Last walk time
    pub last_walk: u64,
    /// Last voice time
    pub last_voice: u64,
    /// Current walk target (for wandering/patrol)
    pub walk_target: Option<Position>,
    /// Patrol index
    pub patrol_index: usize,
}

impl NpcHandler {
    /// Create a new NPC handler
    pub fn new(npc: Npc) -> Self {
        Self {
            npc,
            focus: None,
            dialog: DialogHandler::new(),
            last_walk: 0,
            last_voice: 0,
            walk_target: None,
            patrol_index: 0,
        }
    }

    /// Check if player is in focus range
    pub fn in_range(&self, player_pos: Position) -> bool {
        let dx = (self.npc.position.x as i32 - player_pos.x as i32).abs();
        let dy = (self.npc.position.y as i32 - player_pos.y as i32).abs();
        let dz = (self.npc.position.z as i8 - player_pos.z as i8).abs();

        dz == 0 && dx <= self.npc.focus_range as i32 && dy <= self.npc.focus_range as i32
    }

    /// Handle player saying something to NPC
    pub fn on_say(&mut self, player_id: Uuid, message: &str) -> Option<String> {
        // Check if player is focused
        if self.focus.is_none() {
            // Check for greeting
            if self.dialog.is_greeting(message) {
                self.focus = Some(player_id);
                return self.dialog.get_greeting(&self.npc.name);
            }
            return None;
        }

        // Must be same player
        if self.focus != Some(player_id) {
            return None;
        }

        // Check for farewell
        if self.dialog.is_farewell(message) {
            self.focus = None;
            return self.dialog.get_farewell(&self.npc.name);
        }

        // Process dialog
        self.dialog.process_message(message)
    }

    /// Handle player leaving range
    pub fn on_player_leave(&mut self, player_id: Uuid) {
        if self.focus == Some(player_id) {
            self.focus = None;
        }
    }

    /// Tick update for walking and voices
    pub fn tick(&mut self, current_time: u64) -> Vec<NpcAction> {
        let mut actions = Vec::new();

        // Handle walking
        if self.npc.walkable && self.focus.is_none() {
            if current_time >= self.last_walk + self.npc.walk_interval as u64 {
                if let Some(new_pos) = self.get_next_walk_position() {
                    actions.push(NpcAction::Walk {
                        npc_id: self.npc.id,
                        to: new_pos,
                    });
                    self.last_walk = current_time;
                }
            }
        }

        // Handle voices
        for voice in &self.npc.voices {
            if current_time >= self.last_voice + voice.interval as u64 {
                if rand::random::<u8>() % 100 < voice.chance {
                    actions.push(NpcAction::Say {
                        npc_id: self.npc.id,
                        text: voice.text.clone(),
                        yell: voice.yell,
                    });
                }
                self.last_voice = current_time;
            }
        }

        actions
    }

    /// Get next walk position based on behavior
    fn get_next_walk_position(&mut self) -> Option<Position> {
        match &self.npc.behavior {
            NpcBehavior::Stationary => None,
            NpcBehavior::Wandering { radius } => {
                // Random position within radius
                let offset_x = (rand::random::<i16>() % (*radius as i16 * 2 + 1)) - *radius as i16;
                let offset_y = (rand::random::<i16>() % (*radius as i16 * 2 + 1)) - *radius as i16;
                Some(Position::new(
                    (self.npc.position.x as i32 + offset_x as i32).max(0) as u16,
                    (self.npc.position.y as i32 + offset_y as i32).max(0) as u16,
                    self.npc.position.z,
                ))
            }
            NpcBehavior::Patrol { path } => {
                if path.is_empty() {
                    return None;
                }
                self.patrol_index = (self.patrol_index + 1) % path.len();
                Some(path[self.patrol_index])
            }
            NpcBehavior::Scheduled { .. } => {
                // Would check current schedule
                None
            }
        }
    }
}

/// Action produced by NPC tick
#[derive(Debug, Clone)]
pub enum NpcAction {
    Walk { npc_id: Uuid, to: Position },
    Say { npc_id: Uuid, text: String, yell: bool },
    Turn { npc_id: Uuid, direction: Direction },
}

/// Manages all NPCs in a realm
pub struct NpcManager {
    npcs: HashMap<Uuid, Arc<RwLock<NpcHandler>>>,
    shops: HashMap<String, Arc<Shop>>,
}

impl NpcManager {
    /// Create a new NPC manager
    pub fn new() -> Self {
        Self {
            npcs: HashMap::new(),
            shops: HashMap::new(),
        }
    }

    /// Add an NPC
    pub fn add_npc(&mut self, npc: Npc) -> Uuid {
        let id = npc.id;
        self.npcs.insert(id, Arc::new(RwLock::new(NpcHandler::new(npc))));
        id
    }

    /// Remove an NPC
    pub fn remove_npc(&mut self, id: Uuid) -> bool {
        self.npcs.remove(&id).is_some()
    }

    /// Get an NPC by ID
    pub fn get_npc(&self, id: Uuid) -> Option<Arc<RwLock<NpcHandler>>> {
        self.npcs.get(&id).cloned()
    }

    /// Get NPC by name
    pub async fn find_by_name(&self, name: &str) -> Option<Arc<RwLock<NpcHandler>>> {
        for npc in self.npcs.values() {
            let handler = npc.read().await;
            if handler.npc.name.eq_ignore_ascii_case(name) {
                return Some(npc.clone());
            }
        }
        None
    }

    /// Register a shop
    pub fn register_shop(&mut self, name: String, shop: Shop) {
        self.shops.insert(name, Arc::new(shop));
    }

    /// Get a shop
    pub fn get_shop(&self, name: &str) -> Option<Arc<Shop>> {
        self.shops.get(name).cloned()
    }

    /// Tick all NPCs
    pub async fn tick(&self, current_time: u64) -> Vec<NpcAction> {
        let mut actions = Vec::new();

        for npc in self.npcs.values() {
            let mut handler = npc.write().await;
            actions.extend(handler.tick(current_time));
        }

        actions
    }

    /// Load NPCs from JSON file
    pub fn load_npcs(&mut self, path: &str) -> Result<usize> {
        let content = std::fs::read_to_string(path)?;
        let npcs: Vec<Npc> = serde_json::from_str(&content)
            .map_err(|e| ScriptError::Invalid(e.to_string()))?;

        let count = npcs.len();
        for npc in npcs {
            self.add_npc(npc);
        }

        tracing::info!("Loaded {} NPCs from {}", count, path);
        Ok(count)
    }

    /// NPC count
    pub fn count(&self) -> usize {
        self.npcs.len()
    }
}

impl Default for NpcManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_npc_creation() {
        let npc = Npc::new("Test NPC", Position::new(100, 100, 7))
            .with_outfit(128, 78, 69, 58, 76)
            .with_script("test.lua");

        assert_eq!(npc.name, "Test NPC");
        assert_eq!(npc.look_type, 128);
        assert!(npc.script.is_some());
    }

    #[test]
    fn test_npc_in_range() {
        let npc = Npc::new("Test", Position::new(100, 100, 7));
        let handler = NpcHandler::new(npc);

        assert!(handler.in_range(Position::new(100, 100, 7)));
        assert!(handler.in_range(Position::new(104, 100, 7)));
        assert!(!handler.in_range(Position::new(105, 100, 7)));
        assert!(!handler.in_range(Position::new(100, 100, 6)));
    }

    #[tokio::test]
    async fn test_npc_manager() {
        let mut manager = NpcManager::new();
        
        let npc = Npc::new("Merchant", Position::new(100, 100, 7));
        let id = manager.add_npc(npc);

        assert_eq!(manager.count(), 1);
        
        let found = manager.find_by_name("merchant").await;
        assert!(found.is_some());

        manager.remove_npc(id);
        assert_eq!(manager.count(), 0);
    }
}
