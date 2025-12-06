//! NPC system - non-player characters with dialogue and trading

use crate::creature::{Creature, CreatureType, Outfit};
use crate::item::Item;
use crate::position::{Direction, Position};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// NPC type definition
#[derive(Debug, Clone)]
pub struct Npc {
    pub name: String,
    pub outfit: Outfit,
    pub health: i32,
    pub max_health: i32,
    pub speed: u16,
    pub walk_radius: u8,
    pub floorchange: bool,
    pub attackable: bool,
    pub pushable: bool,
    pub walkable: bool,
    pub script: Option<String>,
    pub shop_items: Vec<ShopItem>,
    pub dialogue: NpcDialogue,
    pub focus_distance: u8,
    pub idle_interval: u32,
    pub walk_interval: u32,
}

impl Npc {
    pub fn new(name: String) -> Self {
        Self {
            name,
            outfit: Outfit::default(),
            health: 100,
            max_health: 100,
            speed: 200,
            walk_radius: 0,
            floorchange: false,
            attackable: false,
            pushable: false,
            walkable: false,
            script: None,
            shop_items: Vec::new(),
            dialogue: NpcDialogue::default(),
            focus_distance: 4,
            idle_interval: 5000,
            walk_interval: 2000,
        }
    }

    /// Create a creature instance from this NPC type
    pub fn spawn(&self, position: Position) -> Creature {
        let mut creature = Creature::new(self.name.clone(), CreatureType::Npc, position);
        creature.outfit = self.outfit;
        creature.stats.health = self.health;
        creature.stats.max_health = self.max_health;
        creature.stats.base_speed = self.speed;
        creature
    }

    /// Check if NPC is a shop
    pub fn is_shop(&self) -> bool {
        !self.shop_items.is_empty()
    }

    /// Get buy price for item
    pub fn get_buy_price(&self, item_id: u16) -> Option<u64> {
        self.shop_items.iter()
            .find(|i| i.item_id == item_id && i.buy_price > 0)
            .map(|i| i.buy_price)
    }

    /// Get sell price for item
    pub fn get_sell_price(&self, item_id: u16) -> Option<u64> {
        self.shop_items.iter()
            .find(|i| i.item_id == item_id && i.sell_price > 0)
            .map(|i| i.sell_price)
    }
}

/// Shop item configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopItem {
    pub item_id: u16,
    pub name: String,
    pub buy_price: u64,
    pub sell_price: u64,
    pub count: u16,
    pub subtype: u8,
}

impl ShopItem {
    pub fn new(item_id: u16, name: String) -> Self {
        Self {
            item_id,
            name,
            buy_price: 0,
            sell_price: 0,
            count: 1,
            subtype: 0,
        }
    }

    pub fn with_prices(item_id: u16, name: String, buy: u64, sell: u64) -> Self {
        Self {
            item_id,
            name,
            buy_price: buy,
            sell_price: sell,
            count: 1,
            subtype: 0,
        }
    }
}

/// NPC dialogue system
#[derive(Debug, Clone, Default)]
pub struct NpcDialogue {
    /// Greeting message
    pub greet: String,
    /// Farewell message
    pub farewell: String,
    /// Busy message (when talking to someone else)
    pub busy: String,
    /// Walk away message
    pub walk_away: String,
    /// Keyword responses
    pub responses: HashMap<String, NpcResponse>,
    /// Keywords for trade
    pub trade_keywords: Vec<String>,
    /// Default response for unknown keywords
    pub default_response: String,
}

/// NPC response
#[derive(Debug, Clone)]
pub struct NpcResponse {
    pub text: String,
    pub topic: Option<String>,
    pub action: Option<NpcAction>,
}

impl NpcResponse {
    pub fn new(text: String) -> Self {
        Self {
            text,
            topic: None,
            action: None,
        }
    }

    pub fn with_action(text: String, action: NpcAction) -> Self {
        Self {
            text,
            topic: None,
            action: Some(action),
        }
    }
}

/// NPC actions
#[derive(Debug, Clone)]
pub enum NpcAction {
    OpenShop,
    GiveItem { item_id: u16, count: u16 },
    TakeItem { item_id: u16, count: u16 },
    Teleport { position: Position },
    StartQuest { quest_id: u32 },
    CompleteQuest { quest_id: u32 },
    Heal,
    TeachSpell { spell_id: u16 },
    Custom { name: String, params: HashMap<String, String> },
}

/// NPC state during conversation
#[derive(Debug, Clone)]
pub struct NpcConversation {
    pub npc_id: u32,
    pub player_id: u32,
    pub topic: Option<String>,
    pub started_at: u64,
    pub last_message: u64,
}

impl NpcConversation {
    pub fn new(npc_id: u32, player_id: u32, time: u64) -> Self {
        Self {
            npc_id,
            player_id,
            topic: None,
            started_at: time,
            last_message: time,
        }
    }

    pub fn is_expired(&self, current_time: u64, timeout_ms: u64) -> bool {
        current_time > self.last_message + timeout_ms
    }
}

/// NPC instance state (extends Creature)
#[derive(Debug)]
pub struct NpcInstance {
    pub creature_id: u32,
    pub npc_type: String,
    pub home_position: Position,
    pub current_conversation: Option<NpcConversation>,
    pub last_walk_time: u64,
    pub last_idle_time: u64,
}

impl NpcInstance {
    pub fn new(creature_id: u32, npc_type: String, home_position: Position) -> Self {
        Self {
            creature_id,
            npc_type,
            home_position,
            current_conversation: None,
            last_walk_time: 0,
            last_idle_time: 0,
        }
    }

    pub fn is_in_conversation(&self) -> bool {
        self.current_conversation.is_some()
    }

    pub fn is_talking_to(&self, player_id: u32) -> bool {
        self.current_conversation
            .as_ref()
            .map(|c| c.player_id == player_id)
            .unwrap_or(false)
    }

    pub fn start_conversation(&mut self, player_id: u32, time: u64) {
        self.current_conversation = Some(NpcConversation::new(self.creature_id, player_id, time));
    }

    pub fn end_conversation(&mut self) {
        self.current_conversation = None;
    }

    pub fn update_conversation(&mut self, time: u64) {
        if let Some(ref mut conv) = self.current_conversation {
            conv.last_message = time;
        }
    }
}

/// NPC loader
pub struct NpcLoader {
    npcs: HashMap<String, Npc>,
}

impl NpcLoader {
    pub fn new() -> Self {
        Self {
            npcs: HashMap::new(),
        }
    }

    /// Load NPCs from directory
    pub fn load_directory(&mut self, path: &str) -> crate::Result<()> {
        info!("Loading NPCs from: {}", path);
        Ok(())
    }

    /// Load a single NPC
    pub fn load_npc(&mut self, path: &str) -> crate::Result<()> {
        info!("Loading NPC: {}", path);
        Ok(())
    }

    /// Get NPC by name
    pub fn get(&self, name: &str) -> Option<&Npc> {
        self.npcs.get(&name.to_lowercase())
    }

    /// Get all NPCs
    pub fn all(&self) -> &HashMap<String, Npc> {
        &self.npcs
    }

    /// Add an NPC
    pub fn add(&mut self, npc: Npc) {
        self.npcs.insert(npc.name.to_lowercase(), npc);
    }
}

impl Default for NpcLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// NPC manager for runtime NPC handling
pub struct NpcManager {
    /// NPC type definitions
    loader: Arc<RwLock<NpcLoader>>,
    /// Active NPC instances
    instances: HashMap<u32, NpcInstance>,
    /// Conversation timeout (milliseconds)
    conversation_timeout: u64,
}

impl NpcManager {
    pub fn new(loader: Arc<RwLock<NpcLoader>>) -> Self {
        Self {
            loader,
            instances: HashMap::new(),
            conversation_timeout: 60000, // 1 minute
        }
    }

    /// Register an NPC instance
    pub fn register(&mut self, creature_id: u32, npc_type: String, home_position: Position) {
        self.instances.insert(
            creature_id,
            NpcInstance::new(creature_id, npc_type, home_position),
        );
    }

    /// Unregister an NPC instance
    pub fn unregister(&mut self, creature_id: u32) {
        self.instances.remove(&creature_id);
    }

    /// Get NPC instance
    pub fn get(&self, creature_id: u32) -> Option<&NpcInstance> {
        self.instances.get(&creature_id)
    }

    /// Get mutable NPC instance
    pub fn get_mut(&mut self, creature_id: u32) -> Option<&mut NpcInstance> {
        self.instances.get_mut(&creature_id)
    }

    /// Process player message to NPC
    pub async fn process_message(
        &mut self,
        creature_id: u32,
        player_id: u32,
        message: &str,
        current_time: u64,
    ) -> Option<String> {
        let instance = self.instances.get_mut(&creature_id)?;
        let loader = self.loader.read().await;
        let npc = loader.get(&instance.npc_type)?;

        // Check if NPC is talking to someone else
        if let Some(ref conv) = instance.current_conversation {
            if conv.player_id != player_id {
                return Some(npc.dialogue.busy.clone());
            }
        }

        let message_lower = message.to_lowercase();

        // Handle greetings
        if message_lower.contains("hi") || message_lower.contains("hello") {
            if instance.current_conversation.is_none() {
                instance.start_conversation(player_id, current_time);
                return Some(npc.dialogue.greet.clone());
            }
        }

        // Handle farewell
        if message_lower.contains("bye") || message_lower.contains("farewell") {
            instance.end_conversation();
            return Some(npc.dialogue.farewell.clone());
        }

        // Handle trade
        if npc.dialogue.trade_keywords.iter().any(|k| message_lower.contains(k)) {
            return Some("I can show you my wares.".to_string());
        }

        // Check keyword responses
        for (keyword, response) in &npc.dialogue.responses {
            if message_lower.contains(keyword) {
                instance.update_conversation(current_time);
                return Some(response.text.clone());
            }
        }

        // Default response
        if !npc.dialogue.default_response.is_empty() {
            return Some(npc.dialogue.default_response.clone());
        }

        None
    }

    /// Tick - process NPC AI
    pub fn tick(&mut self, current_time: u64) {
        // Check conversation timeouts
        for instance in self.instances.values_mut() {
            if let Some(ref conv) = instance.current_conversation {
                if conv.is_expired(current_time, self.conversation_timeout) {
                    instance.end_conversation();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_npc_creation() {
        let npc = Npc::new("Test NPC".to_string());
        assert_eq!(npc.name, "Test NPC");
        assert!(!npc.is_shop());
    }

    #[test]
    fn test_shop_item() {
        let mut npc = Npc::new("Shop NPC".to_string());
        npc.shop_items.push(ShopItem::with_prices(100, "Sword".to_string(), 100, 50));

        assert!(npc.is_shop());
        assert_eq!(npc.get_buy_price(100), Some(100));
        assert_eq!(npc.get_sell_price(100), Some(50));
    }
}
