//! Loot system - handles monster drops and corpse loot generation
//!
//! This module provides a complete loot system for Shadow OT including:
//! - Loot table definitions with weighted random drops
//! - Nested loot (bags containing items)
//! - Server-side loot rate multipliers
//! - Party loot distribution modes
//! - Boss-specific loot mechanics
//! - Rare item announcements

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single loot entry representing an item that can drop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootEntry {
    /// Item type ID (references items.json / OTB)
    pub item_id: u16,
    /// Base drop chance (0.0 - 100.0 percent)
    pub chance: f32,
    /// Minimum count if dropped
    pub count_min: u16,
    /// Maximum count if dropped  
    pub count_max: u16,
    /// Action ID for special items (quest items, keys, etc.)
    pub action_id: Option<u16>,
    /// Unique ID for unique items
    pub unique_id: Option<u16>,
    /// Nested loot for containers (bags, backpacks)
    #[serde(default)]
    pub contents: Vec<LootEntry>,
    /// Optional item name for logging
    #[serde(default)]
    pub name: Option<String>,
}

impl LootEntry {
    /// Create a simple loot entry for a single item
    pub fn new(item_id: u16, chance: f32) -> Self {
        Self {
            item_id,
            chance,
            count_min: 1,
            count_max: 1,
            action_id: None,
            unique_id: None,
            contents: Vec::new(),
            name: None,
        }
    }

    /// Create a stackable item loot entry
    pub fn stackable(item_id: u16, chance: f32, min: u16, max: u16) -> Self {
        Self {
            item_id,
            chance,
            count_min: min,
            count_max: max,
            action_id: None,
            unique_id: None,
            contents: Vec::new(),
            name: None,
        }
    }

    /// Create a container with nested loot
    pub fn container(item_id: u16, chance: f32, contents: Vec<LootEntry>) -> Self {
        Self {
            item_id,
            chance,
            count_min: 1,
            count_max: 1,
            action_id: None,
            unique_id: None,
            contents,
            name: None,
        }
    }

    /// Add action ID to this entry
    pub fn with_action_id(mut self, action_id: u16) -> Self {
        self.action_id = Some(action_id);
        self
    }

    /// Add name for logging
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

/// Complete loot table for a creature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootTable {
    /// Creature name this table belongs to
    pub creature_name: String,
    /// Gold drop configuration
    pub gold: Option<GoldDrop>,
    /// List of all possible drops
    pub entries: Vec<LootEntry>,
    /// Maximum items that can drop (0 = unlimited)
    pub max_items: u8,
    /// Whether this creature always drops at least one item
    pub guaranteed_drop: bool,
}

impl LootTable {
    /// Create a new empty loot table
    pub fn new(creature_name: impl Into<String>) -> Self {
        Self {
            creature_name: creature_name.into(),
            gold: None,
            entries: Vec::new(),
            max_items: 0,
            guaranteed_drop: false,
        }
    }

    /// Add gold drop configuration
    pub fn with_gold(mut self, min: u32, max: u32, chance: f32) -> Self {
        self.gold = Some(GoldDrop {
            min_amount: min,
            max_amount: max,
            chance,
        });
        self
    }

    /// Add a loot entry
    pub fn add_entry(mut self, entry: LootEntry) -> Self {
        self.entries.push(entry);
        self
    }

    /// Set max items
    pub fn with_max_items(mut self, max: u8) -> Self {
        self.max_items = max;
        self
    }
}

/// Gold drop configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldDrop {
    pub min_amount: u32,
    pub max_amount: u32,
    pub chance: f32,
}

/// Generated loot item ready to be added to corpse
#[derive(Debug, Clone)]
pub struct GeneratedLoot {
    pub item_id: u16,
    pub count: u16,
    pub action_id: Option<u16>,
    pub unique_id: Option<u16>,
    pub contents: Vec<GeneratedLoot>,
}

impl GeneratedLoot {
    /// Create a simple item
    pub fn item(item_id: u16, count: u16) -> Self {
        Self {
            item_id,
            count,
            action_id: None,
            unique_id: None,
            contents: Vec::new(),
        }
    }

    /// Total item count including contents
    pub fn total_items(&self) -> usize {
        1 + self.contents.iter().map(|c| c.total_items()).sum::<usize>()
    }
}

/// Loot distribution mode for parties
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LootDistribution {
    /// Leader gets all loot
    Leader,
    /// Random party member for each drop
    Random,
    /// Round robin distribution
    RoundRobin,
    /// Free for all - first come first served
    FreeForAll,
}

impl Default for LootDistribution {
    fn default() -> Self {
        Self::Leader
    }
}

/// Loot generation configuration
#[derive(Debug, Clone)]
pub struct LootConfig {
    /// Server loot rate multiplier (1.0 = normal)
    pub loot_rate: f32,
    /// Bonus loot rate from premium status
    pub premium_bonus: f32,
    /// Enable rare item announcements
    pub announce_rare: bool,
    /// Rare item threshold (items with chance below this are "rare")
    pub rare_threshold: f32,
    /// Maximum nested container depth
    pub max_container_depth: u8,
}

impl Default for LootConfig {
    fn default() -> Self {
        Self {
            loot_rate: 1.0,
            premium_bonus: 0.0,
            announce_rare: true,
            rare_threshold: 0.5,
            max_container_depth: 2,
        }
    }
}

/// The loot generator handles creating loot from tables
pub struct LootGenerator {
    config: LootConfig,
    loot_tables: HashMap<String, LootTable>,
    rng: rand::rngs::ThreadRng,
}

impl LootGenerator {
    /// Create a new loot generator
    pub fn new(config: LootConfig) -> Self {
        Self {
            config,
            loot_tables: HashMap::new(),
            rng: rand::thread_rng(),
        }
    }

    /// Register a loot table for a creature
    pub fn register_table(&mut self, table: LootTable) {
        let name = table.creature_name.to_lowercase();
        self.loot_tables.insert(name, table);
    }

    /// Load loot tables from JSON file
    pub fn load_from_file(&mut self, path: &str) -> Result<(), LootError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| LootError::IoError(e.to_string()))?;
        
        let tables: Vec<LootTable> = serde_json::from_str(&content)
            .map_err(|e| LootError::ParseError(e.to_string()))?;

        for table in tables {
            self.register_table(table);
        }

        Ok(())
    }

    /// Generate loot for a killed creature
    pub fn generate(
        &mut self,
        creature_name: &str,
        killer_premium: bool,
    ) -> Result<LootResult, LootError> {
        let table = self.loot_tables
            .get(&creature_name.to_lowercase())
            .ok_or_else(|| LootError::TableNotFound(creature_name.to_string()))?
            .clone();

        let mut result = LootResult {
            creature_name: creature_name.to_string(),
            items: Vec::new(),
            gold: 0,
            rare_items: Vec::new(),
        };

        // Calculate effective loot rate
        let loot_rate = self.config.loot_rate
            + if killer_premium { self.config.premium_bonus } else { 0.0 };

        // Generate gold
        if let Some(ref gold) = table.gold {
            let adjusted_chance = (gold.chance * loot_rate).min(100.0);
            if self.roll_chance(adjusted_chance) {
                let amount = self.rng.gen_range(gold.min_amount..=gold.max_amount);
                result.gold = (amount as f32 * loot_rate) as u32;
            }
        }

        // Generate items
        let mut item_count = 0;
        for entry in &table.entries {
            if table.max_items > 0 && item_count >= table.max_items as usize {
                break;
            }

            if let Some(generated) = self.generate_entry(entry, loot_rate, 0) {
                // Check if rare
                if entry.chance < self.config.rare_threshold {
                    result.rare_items.push(RareItemDrop {
                        item_id: entry.item_id,
                        item_name: entry.name.clone(),
                        chance: entry.chance,
                    });
                }

                item_count += generated.total_items();
                result.items.push(generated);
            }
        }

        // Guaranteed drop - if nothing dropped, force one item
        if table.guaranteed_drop && result.items.is_empty() && !table.entries.is_empty() {
            // Pick a random entry and force it to drop
            let idx = self.rng.gen_range(0..table.entries.len());
            if let Some(entry) = table.entries.get(idx) {
                let count = self.rng.gen_range(entry.count_min..=entry.count_max);
                result.items.push(GeneratedLoot {
                    item_id: entry.item_id,
                    count,
                    action_id: entry.action_id,
                    unique_id: entry.unique_id,
                    contents: Vec::new(),
                });
            }
        }

        Ok(result)
    }

    /// Generate a single entry recursively (for containers)
    fn generate_entry(
        &mut self,
        entry: &LootEntry,
        loot_rate: f32,
        depth: u8,
    ) -> Option<GeneratedLoot> {
        // Check max depth for containers
        if depth > self.config.max_container_depth {
            return None;
        }

        // Roll for this item
        let adjusted_chance = (entry.chance * loot_rate).min(100.0);
        if !self.roll_chance(adjusted_chance) {
            return None;
        }

        // Determine count
        let count = self.rng.gen_range(entry.count_min..=entry.count_max);

        // Generate contents if this is a container
        let contents = entry.contents
            .iter()
            .filter_map(|e| self.generate_entry(e, loot_rate, depth + 1))
            .collect();

        Some(GeneratedLoot {
            item_id: entry.item_id,
            count,
            action_id: entry.action_id,
            unique_id: entry.unique_id,
            contents,
        })
    }

    /// Roll a chance check (chance is 0.0-100.0)
    fn roll_chance(&mut self, chance: f32) -> bool {
        if chance >= 100.0 {
            return true;
        }
        if chance <= 0.0 {
            return false;
        }
        self.rng.gen_range(0.0..100.0) < chance
    }

    /// Set loot rate
    pub fn set_loot_rate(&mut self, rate: f32) {
        self.config.loot_rate = rate;
    }

    /// Get a loot table
    pub fn get_table(&self, creature_name: &str) -> Option<&LootTable> {
        self.loot_tables.get(&creature_name.to_lowercase())
    }
}

/// Result of loot generation
#[derive(Debug, Clone)]
pub struct LootResult {
    pub creature_name: String,
    pub items: Vec<GeneratedLoot>,
    pub gold: u32,
    pub rare_items: Vec<RareItemDrop>,
}

impl LootResult {
    /// Total number of items including nested
    pub fn total_items(&self) -> usize {
        self.items.iter().map(|i| i.total_items()).sum()
    }

    /// Whether this drop contained rare items
    pub fn has_rare(&self) -> bool {
        !self.rare_items.is_empty()
    }
}

/// Information about a rare item that dropped
#[derive(Debug, Clone)]
pub struct RareItemDrop {
    pub item_id: u16,
    pub item_name: Option<String>,
    pub chance: f32,
}

/// Loot system errors
#[derive(Debug, Clone)]
pub enum LootError {
    TableNotFound(String),
    IoError(String),
    ParseError(String),
}

impl std::fmt::Display for LootError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LootError::TableNotFound(name) => write!(f, "Loot table not found for: {}", name),
            LootError::IoError(msg) => write!(f, "IO error: {}", msg),
            LootError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for LootError {}

/// Party loot handler for distributing loot among party members
pub struct PartyLootHandler {
    distribution: LootDistribution,
    round_robin_index: usize,
}

impl PartyLootHandler {
    pub fn new(distribution: LootDistribution) -> Self {
        Self {
            distribution,
            round_robin_index: 0,
        }
    }

    /// Determine who gets the next piece of loot
    pub fn get_loot_recipient(&mut self, party_members: &[uuid::Uuid]) -> Option<uuid::Uuid> {
        if party_members.is_empty() {
            return None;
        }

        match self.distribution {
            LootDistribution::Leader => Some(party_members[0]),
            LootDistribution::Random => {
                let idx = rand::thread_rng().gen_range(0..party_members.len());
                Some(party_members[idx])
            }
            LootDistribution::RoundRobin => {
                let recipient = party_members[self.round_robin_index % party_members.len()];
                self.round_robin_index += 1;
                Some(recipient)
            }
            LootDistribution::FreeForAll => None, // Anyone can loot
        }
    }

    /// Set distribution mode
    pub fn set_distribution(&mut self, distribution: LootDistribution) {
        self.distribution = distribution;
        if matches!(distribution, LootDistribution::RoundRobin) {
            self.round_robin_index = 0;
        }
    }
}

/// Boss loot mechanics
#[derive(Debug, Clone)]
pub struct BossLootConfig {
    /// Minimum damage to qualify for loot
    pub min_damage_threshold: u32,
    /// Whether to split loot among all contributors
    pub split_loot: bool,
    /// Bonus chance for top damage dealer
    pub top_damage_bonus: f32,
}

impl Default for BossLootConfig {
    fn default() -> Self {
        Self {
            min_damage_threshold: 100,
            split_loot: true,
            top_damage_bonus: 10.0,
        }
    }
}

/// Tracks damage contributions for boss loot distribution
#[derive(Debug, Clone)]
pub struct DamageTracker {
    contributions: HashMap<uuid::Uuid, u32>,
}

impl DamageTracker {
    pub fn new() -> Self {
        Self {
            contributions: HashMap::new(),
        }
    }

    /// Record damage dealt by a player
    pub fn record_damage(&mut self, player_id: uuid::Uuid, damage: u32) {
        *self.contributions.entry(player_id).or_insert(0) += damage;
    }

    /// Get all players who qualify for loot
    pub fn get_qualifiers(&self, min_threshold: u32) -> Vec<uuid::Uuid> {
        self.contributions
            .iter()
            .filter(|(_, &damage)| damage >= min_threshold)
            .map(|(&id, _)| id)
            .collect()
    }

    /// Get the top damage dealer
    pub fn top_dealer(&self) -> Option<uuid::Uuid> {
        self.contributions
            .iter()
            .max_by_key(|(_, &damage)| damage)
            .map(|(&id, _)| id)
    }

    /// Get damage dealt by a specific player
    pub fn get_damage(&self, player_id: uuid::Uuid) -> u32 {
        self.contributions.get(&player_id).copied().unwrap_or(0)
    }

    /// Clear all tracked damage
    pub fn clear(&mut self) {
        self.contributions.clear();
    }
}

impl Default for DamageTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loot_entry_creation() {
        let entry = LootEntry::new(100, 50.0);
        assert_eq!(entry.item_id, 100);
        assert_eq!(entry.chance, 50.0);
        assert_eq!(entry.count_min, 1);
        assert_eq!(entry.count_max, 1);
    }

    #[test]
    fn test_stackable_entry() {
        let entry = LootEntry::stackable(2148, 100.0, 1, 50); // Gold coin
        assert_eq!(entry.count_min, 1);
        assert_eq!(entry.count_max, 50);
    }

    #[test]
    fn test_loot_table() {
        let table = LootTable::new("Rat")
            .with_gold(0, 4, 50.0)
            .add_entry(LootEntry::new(2666, 25.0)) // Meat
            .add_entry(LootEntry::new(5882, 5.0));  // Rat's tail

        assert_eq!(table.creature_name, "Rat");
        assert!(table.gold.is_some());
        assert_eq!(table.entries.len(), 2);
    }

    #[test]
    fn test_loot_generator() {
        let mut generator = LootGenerator::new(LootConfig {
            loot_rate: 100.0, // 100x rate for testing
            ..Default::default()
        });

        let table = LootTable::new("Test")
            .add_entry(LootEntry::new(100, 100.0)); // 100% chance

        generator.register_table(table);

        let result = generator.generate("Test", false).unwrap();
        assert!(!result.items.is_empty());
    }

    #[test]
    fn test_damage_tracker() {
        let mut tracker = DamageTracker::new();
        let player1 = uuid::Uuid::new_v4();
        let player2 = uuid::Uuid::new_v4();

        tracker.record_damage(player1, 100);
        tracker.record_damage(player2, 50);
        tracker.record_damage(player1, 100);

        assert_eq!(tracker.get_damage(player1), 200);
        assert_eq!(tracker.get_damage(player2), 50);
        assert_eq!(tracker.top_dealer(), Some(player1));
    }

    #[test]
    fn test_party_loot_distribution() {
        let mut handler = PartyLootHandler::new(LootDistribution::Leader);
        let members = vec![uuid::Uuid::new_v4(), uuid::Uuid::new_v4()];

        assert_eq!(handler.get_loot_recipient(&members), Some(members[0]));

        handler.set_distribution(LootDistribution::RoundRobin);
        assert_eq!(handler.get_loot_recipient(&members), Some(members[0]));
        assert_eq!(handler.get_loot_recipient(&members), Some(members[1]));
        assert_eq!(handler.get_loot_recipient(&members), Some(members[0]));
    }
}
