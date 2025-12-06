//! Prey System Implementation
//!
//! The Prey system allows players to select creatures for bonus effects.
//! Each prey slot provides configurable bonuses like damage boost, XP boost,
//! loot improvement, or bestiary progress.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Maximum number of prey slots (3 default, can unlock more)
pub const MAX_PREY_SLOTS: usize = 3;
pub const PREY_REROLL_WILDCARDS_COST: u32 = 5;
pub const PREY_DURATION_HOURS: u32 = 2;

/// Types of bonuses that prey can provide
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PreyBonusType {
    /// Increased damage against the prey creature
    DamageBoost,
    /// Reduced damage from the prey creature
    DefenseBoost,
    /// Increased experience from killing the prey creature
    ExperienceBoost,
    /// Improved loot drops from the prey creature
    LootBoost,
}

impl PreyBonusType {
    /// Get the maximum bonus percentage for this type
    pub fn max_bonus(&self) -> u8 {
        match self {
            PreyBonusType::DamageBoost => 25,
            PreyBonusType::DefenseBoost => 30,
            PreyBonusType::ExperienceBoost => 40,
            PreyBonusType::LootBoost => 25,
        }
    }

    /// Get bonus value for a given star level (1-10)
    pub fn bonus_for_stars(&self, stars: u8) -> f32 {
        let max = self.max_bonus() as f32;
        let stars_clamped = stars.min(10).max(1);
        (max / 10.0) * stars_clamped as f32
    }
}

/// A single prey slot configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreySlot {
    /// Slot index (0-2 for standard slots)
    pub index: u8,
    /// Whether this slot is unlocked
    pub unlocked: bool,
    /// Currently selected creature race ID
    pub creature_id: Option<u32>,
    /// Currently selected creature name
    pub creature_name: Option<String>,
    /// Selected bonus type
    pub bonus_type: Option<PreyBonusType>,
    /// Bonus star level (1-10)
    pub bonus_stars: u8,
    /// Available creatures to select from
    pub available_creatures: Vec<PreyCreatureOption>,
    /// Time remaining in seconds
    pub time_remaining: u32,
    /// Whether the slot is locked (can't change creature)
    pub is_locked: bool,
    /// Number of free rerolls remaining
    pub free_rerolls: u8,
}

impl Default for PreySlot {
    fn default() -> Self {
        Self {
            index: 0,
            unlocked: true,
            creature_id: None,
            creature_name: None,
            bonus_type: None,
            bonus_stars: 1,
            available_creatures: Vec::new(),
            time_remaining: 0,
            is_locked: false,
            free_rerolls: 1,
        }
    }
}

impl PreySlot {
    pub fn new(index: u8) -> Self {
        Self {
            index,
            unlocked: index == 0, // First slot is free
            ..Default::default()
        }
    }

    /// Check if prey is active
    pub fn is_active(&self) -> bool {
        self.creature_id.is_some() && self.time_remaining > 0
    }

    /// Get current bonus value
    pub fn bonus_value(&self) -> f32 {
        self.bonus_type
            .map(|bt| bt.bonus_for_stars(self.bonus_stars))
            .unwrap_or(0.0)
    }

    /// Select a creature for this slot
    pub fn select_creature(&mut self, creature_id: u32, creature_name: String) -> bool {
        if self.is_locked {
            return false;
        }

        // Check if creature is in available list
        if !self.available_creatures.iter().any(|c| c.creature_id == creature_id) {
            return false;
        }

        self.creature_id = Some(creature_id);
        self.creature_name = Some(creature_name);
        self.time_remaining = PREY_DURATION_HOURS * 3600;
        true
    }

    /// Select a bonus type for this slot
    pub fn select_bonus(&mut self, bonus_type: PreyBonusType) {
        self.bonus_type = Some(bonus_type);
    }

    /// Reroll available creatures
    pub fn reroll(&mut self, new_creatures: Vec<PreyCreatureOption>) -> bool {
        if self.is_locked {
            return false;
        }

        self.available_creatures = new_creatures;
        self.creature_id = None;
        self.creature_name = None;
        true
    }

    /// Update time (call every second)
    pub fn tick(&mut self) {
        if self.time_remaining > 0 {
            self.time_remaining -= 1;
            if self.time_remaining == 0 {
                self.creature_id = None;
                self.creature_name = None;
                self.bonus_type = None;
            }
        }
    }
}

/// A creature option for prey selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreyCreatureOption {
    pub creature_id: u32,
    pub name: String,
    pub difficulty: u8,
}

/// Player's prey state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerPrey {
    pub player_id: Uuid,
    pub slots: Vec<PreySlot>,
    pub wildcard_count: u32,
    /// Total prey kills for statistics
    pub total_prey_kills: u64,
    /// Bonus points earned from prey system
    pub prey_points: u64,
}

impl PlayerPrey {
    pub fn new(player_id: Uuid) -> Self {
        Self {
            player_id,
            slots: vec![
                PreySlot::new(0),
                PreySlot::new(1),
                PreySlot::new(2),
            ],
            wildcard_count: 0,
            total_prey_kills: 0,
            prey_points: 0,
        }
    }

    /// Check if a creature is active prey in any slot
    pub fn is_prey(&self, creature_id: u32) -> Option<&PreySlot> {
        self.slots.iter().find(|s| s.creature_id == Some(creature_id) && s.is_active())
    }

    /// Get bonus for killing a creature
    pub fn get_kill_bonus(&self, creature_id: u32) -> Option<(PreyBonusType, f32)> {
        self.is_prey(creature_id)
            .and_then(|slot| slot.bonus_type.map(|bt| (bt, slot.bonus_value())))
    }

    /// Record a prey kill
    pub fn record_kill(&mut self, creature_id: u32) {
        if self.is_prey(creature_id).is_some() {
            self.total_prey_kills += 1;
            self.prey_points += 1;
        }
    }

    /// Unlock a prey slot with Tibia Coins
    pub fn unlock_slot(&mut self, slot_index: usize) -> bool {
        if slot_index < self.slots.len() && !self.slots[slot_index].unlocked {
            self.slots[slot_index].unlocked = true;
            true
        } else {
            false
        }
    }

    /// Use wildcards to reroll
    pub fn use_wildcards_for_reroll(&mut self, slot_index: usize) -> bool {
        if slot_index >= self.slots.len() {
            return false;
        }
        if self.wildcard_count < PREY_REROLL_WILDCARDS_COST {
            return false;
        }
        self.wildcard_count -= PREY_REROLL_WILDCARDS_COST;
        true
    }

    /// Update all slots (call every second)
    pub fn tick(&mut self) {
        for slot in &mut self.slots {
            slot.tick();
        }
    }
}

/// Prey system manager
pub struct PreyManager {
    /// Player prey data
    player_prey: HashMap<Uuid, PlayerPrey>,
    /// Creature difficulty ratings for prey selection
    creature_difficulties: HashMap<u32, u8>,
}

impl PreyManager {
    pub fn new() -> Self {
        Self {
            player_prey: HashMap::new(),
            creature_difficulties: HashMap::new(),
        }
    }

    /// Get or create player prey data
    pub fn get_or_create(&mut self, player_id: Uuid) -> &mut PlayerPrey {
        self.player_prey
            .entry(player_id)
            .or_insert_with(|| PlayerPrey::new(player_id))
    }

    /// Generate random creatures for prey selection
    pub fn generate_prey_options(&self, player_level: u16, count: usize) -> Vec<PreyCreatureOption> {
        // In a real implementation, this would query available creatures
        // based on player level and return random selections
        Vec::new()
    }

    /// Set creature difficulty rating
    pub fn set_creature_difficulty(&mut self, creature_id: u32, difficulty: u8) {
        self.creature_difficulties.insert(creature_id, difficulty);
    }

    /// Process a creature kill for prey bonuses
    pub fn process_kill(&mut self, player_id: Uuid, creature_id: u32) -> Option<(PreyBonusType, f32)> {
        if let Some(prey) = self.player_prey.get_mut(&player_id) {
            prey.record_kill(creature_id);
            prey.get_kill_bonus(creature_id)
        } else {
            None
        }
    }

    /// Update all prey timers
    pub fn tick(&mut self) {
        for prey in self.player_prey.values_mut() {
            prey.tick();
        }
    }
}

impl Default for PreyManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prey_bonus_calculation() {
        assert_eq!(PreyBonusType::DamageBoost.bonus_for_stars(10), 25.0);
        assert_eq!(PreyBonusType::ExperienceBoost.bonus_for_stars(5), 20.0);
    }

    #[test]
    fn test_prey_slot_selection() {
        let mut slot = PreySlot::new(0);
        slot.available_creatures.push(PreyCreatureOption {
            creature_id: 100,
            name: "Dragon".to_string(),
            difficulty: 5,
        });

        assert!(slot.select_creature(100, "Dragon".to_string()));
        assert!(slot.is_active());
    }
}
