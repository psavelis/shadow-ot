//! Imbuement System
//!
//! Tibia's imbuement system allows players to enhance equipment with temporary
//! magical effects. This module implements:
//! - Imbuement types and effects
//! - Imbuement tiers (Basic, Intricate, Powerful)
//! - Item slot management
//! - Duration tracking and decay

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Imbuement tiers - each tier is more powerful and expensive
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImbuementTier {
    /// Basic tier - lowest cost and effect
    Basic,
    /// Intricate tier - medium cost and effect
    Intricate,
    /// Powerful tier - highest cost and effect
    Powerful,
}

impl ImbuementTier {
    /// Get tier multiplier for effects
    pub fn effect_multiplier(&self) -> f32 {
        match self {
            ImbuementTier::Basic => 1.0,
            ImbuementTier::Intricate => 2.0,
            ImbuementTier::Powerful => 4.0,
        }
    }

    /// Get duration in hours
    pub fn duration_hours(&self) -> u32 {
        20 // All tiers have 20 hours of active time
    }

    /// Get gold cost
    pub fn gold_cost(&self) -> u64 {
        match self {
            ImbuementTier::Basic => 15000,
            ImbuementTier::Intricate => 55000,
            ImbuementTier::Powerful => 150000,
        }
    }

    /// Get success rate (100% base, can be modified)
    pub fn base_success_rate(&self) -> f32 {
        match self {
            ImbuementTier::Basic => 90.0,
            ImbuementTier::Intricate => 70.0,
            ImbuementTier::Powerful => 50.0,
        }
    }

    /// Get removal cost (percentage of imbue cost)
    pub fn removal_cost_percent(&self) -> u32 {
        25
    }
}

/// Imbuement category - determines which item slots can use it
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImbuementCategory {
    /// Elemental damage (weapons)
    ElementalDamage,
    /// Elemental protection (armor, helmets, shields)
    ElementalProtection,
    /// Life/Mana leech (weapons)
    Leech,
    /// Critical hit (weapons)
    Critical,
    /// Skill boost (various)
    SkillBoost,
    /// Speed boost (boots)
    Speed,
    /// Capacity boost (backpacks)
    Capacity,
    /// Paralysis removal (boots)
    Vibrancy,
}

/// Specific imbuement types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImbuementType {
    // Elemental Damage (Weapons)
    /// Fire damage - Scorch
    Scorch,
    /// Ice damage - Frost
    Frost,
    /// Energy damage - Electrify
    Electrify,
    /// Earth damage - Venom
    Venom,
    /// Death damage - Reap
    Reap,

    // Elemental Protection (Armor/Helmets/Shields)
    /// Fire protection - Lich Shroud
    LichShroud,
    /// Ice protection - Snake Skin
    SnakeSkin,
    /// Energy protection - Cloud Fabric
    CloudFabric,
    /// Earth protection - Quara Scale
    QuaraScale,
    /// Death protection - Dragon Hide
    DragonHide,
    /// Holy protection - Demon Presence
    DemonPresence,
    /// Physical protection - Swiftness
    Swiftness,

    // Leech (Weapons)
    /// Life leech - Vampirism
    Vampirism,
    /// Mana leech - Void
    Void,

    // Critical (Weapons)
    /// Critical hit chance - Strike
    Strike,

    // Skill Boost (Various)
    /// Sword skill - Slash
    Slash,
    /// Axe skill - Chop
    Chop,
    /// Club skill - Bash
    Bash,
    /// Distance skill - Precision
    Precision,
    /// Magic level - Epiphany
    Epiphany,
    /// Shielding - Blockade
    Blockade,
    /// Fist fighting - Featherweight
    Featherweight,

    // Special
    /// Speed - Swiftness
    SwiftnessBoots,
    /// Capacity - Featherweight
    FeatherweightBackpack,
    /// Paralyze removal - Vibrancy
    Vibrancy,
}

impl ImbuementType {
    /// Get the category of this imbuement
    pub fn category(&self) -> ImbuementCategory {
        match self {
            Self::Scorch | Self::Frost | Self::Electrify | Self::Venom | Self::Reap => {
                ImbuementCategory::ElementalDamage
            }
            Self::LichShroud | Self::SnakeSkin | Self::CloudFabric | Self::QuaraScale
            | Self::DragonHide | Self::DemonPresence | Self::Swiftness => {
                ImbuementCategory::ElementalProtection
            }
            Self::Vampirism | Self::Void => ImbuementCategory::Leech,
            Self::Strike => ImbuementCategory::Critical,
            Self::Slash | Self::Chop | Self::Bash | Self::Precision | Self::Epiphany
            | Self::Blockade | Self::Featherweight => ImbuementCategory::SkillBoost,
            Self::SwiftnessBoots => ImbuementCategory::Speed,
            Self::FeatherweightBackpack => ImbuementCategory::Capacity,
            Self::Vibrancy => ImbuementCategory::Vibrancy,
        }
    }

    /// Get the base effect value at Basic tier
    pub fn base_effect_value(&self) -> i32 {
        match self {
            // Elemental damage: +10/20/40
            Self::Scorch | Self::Frost | Self::Electrify | Self::Venom | Self::Reap => 10,
            // Elemental protection: +2/4/8%
            Self::LichShroud | Self::SnakeSkin | Self::CloudFabric | Self::QuaraScale
            | Self::DragonHide | Self::DemonPresence | Self::Swiftness => 2,
            // Life/Mana leech: +4/8/16%
            Self::Vampirism | Self::Void => 4,
            // Critical: +5/10/20% chance, +15/30/60% damage
            Self::Strike => 5,
            // Skill boost: +1/2/4
            Self::Slash | Self::Chop | Self::Bash | Self::Precision | Self::Blockade
            | Self::Featherweight => 1,
            // Magic level: +1/2/4
            Self::Epiphany => 1,
            // Speed: +10/20/40
            Self::SwiftnessBoots => 10,
            // Capacity: +250/500/1000
            Self::FeatherweightBackpack => 250,
            // Vibrancy: removes paralysis
            Self::Vibrancy => 1,
        }
    }

    /// Get the display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Scorch => "Scorch",
            Self::Frost => "Frost",
            Self::Electrify => "Electrify",
            Self::Venom => "Venom",
            Self::Reap => "Reap",
            Self::LichShroud => "Lich Shroud",
            Self::SnakeSkin => "Snake Skin",
            Self::CloudFabric => "Cloud Fabric",
            Self::QuaraScale => "Quara Scale",
            Self::DragonHide => "Dragon Hide",
            Self::DemonPresence => "Demon Presence",
            Self::Swiftness => "Swiftness",
            Self::Vampirism => "Vampirism",
            Self::Void => "Void",
            Self::Strike => "Strike",
            Self::Slash => "Slash",
            Self::Chop => "Chop",
            Self::Bash => "Bash",
            Self::Precision => "Precision",
            Self::Epiphany => "Epiphany",
            Self::Blockade => "Blockade",
            Self::Featherweight => "Featherweight",
            Self::SwiftnessBoots => "Swiftness",
            Self::FeatherweightBackpack => "Featherweight",
            Self::Vibrancy => "Vibrancy",
        }
    }

    /// Get required creature products for this imbuement
    pub fn required_products(&self, tier: ImbuementTier) -> Vec<ImbuementProduct> {
        let count = match tier {
            ImbuementTier::Basic => 20,
            ImbuementTier::Intricate => 25,
            ImbuementTier::Powerful => 25,
        };

        match self {
            Self::Vampirism => vec![
                ImbuementProduct { item_id: 10605, count }, // Vampire teeth
            ],
            Self::Void => vec![
                ImbuementProduct { item_id: 24663, count }, // Rope belt
            ],
            Self::Strike => vec![
                ImbuementProduct { item_id: 7439, count }, // Piece of scarab shell
            ],
            // Add more product requirements...
            _ => vec![],
        }
    }
}

/// Required product for an imbuement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImbuementProduct {
    pub item_id: u16,
    pub count: u32,
}

/// Active imbuement on an item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveImbuement {
    /// Unique ID
    pub id: u32,
    /// Imbuement type
    pub imbuement_type: ImbuementType,
    /// Tier level
    pub tier: ImbuementTier,
    /// Remaining duration in seconds
    pub remaining_seconds: u32,
    /// Applied timestamp
    pub applied_at: DateTime<Utc>,
    /// Slot index on the item
    pub slot_index: u8,
}

impl ActiveImbuement {
    /// Create a new active imbuement
    pub fn new(imbuement_type: ImbuementType, tier: ImbuementTier, slot_index: u8) -> Self {
        Self {
            id: rand::random(),
            imbuement_type,
            tier,
            remaining_seconds: tier.duration_hours() * 3600,
            applied_at: Utc::now(),
            slot_index,
        }
    }

    /// Get the current effect value
    pub fn effect_value(&self) -> i32 {
        (self.imbuement_type.base_effect_value() as f32 * self.tier.effect_multiplier()) as i32
    }

    /// Check if expired
    pub fn is_expired(&self) -> bool {
        self.remaining_seconds == 0
    }

    /// Consume time (called when item is equipped and in combat)
    pub fn consume_time(&mut self, seconds: u32) {
        self.remaining_seconds = self.remaining_seconds.saturating_sub(seconds);
    }

    /// Get remaining time as string
    pub fn remaining_time_string(&self) -> String {
        let hours = self.remaining_seconds / 3600;
        let minutes = (self.remaining_seconds % 3600) / 60;
        format!("{}h {:02}min", hours, minutes)
    }
}

/// Item slot types that can have imbuements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImbuementSlotType {
    Helmet,
    Armor,
    Weapon,
    Shield,
    Boots,
    Backpack,
}

impl ImbuementSlotType {
    /// Get allowed imbuement categories for this slot type
    pub fn allowed_categories(&self) -> Vec<ImbuementCategory> {
        match self {
            Self::Helmet => vec![
                ImbuementCategory::ElementalProtection,
                ImbuementCategory::SkillBoost,
            ],
            Self::Armor => vec![
                ImbuementCategory::ElementalProtection,
                ImbuementCategory::SkillBoost,
            ],
            Self::Weapon => vec![
                ImbuementCategory::ElementalDamage,
                ImbuementCategory::Leech,
                ImbuementCategory::Critical,
                ImbuementCategory::SkillBoost,
            ],
            Self::Shield => vec![
                ImbuementCategory::ElementalProtection,
                ImbuementCategory::SkillBoost,
            ],
            Self::Boots => vec![
                ImbuementCategory::Speed,
                ImbuementCategory::Vibrancy,
            ],
            Self::Backpack => vec![
                ImbuementCategory::Capacity,
            ],
        }
    }
}

/// Imbuement shrine location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImbuementShrine {
    pub position_x: u16,
    pub position_y: u16,
    pub position_z: u8,
    pub town_id: u16,
}

/// Result of imbuement attempt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImbuementResult {
    Success(ActiveImbuement),
    Failure { gold_lost: u64, items_lost: Vec<ImbuementProduct> },
    InvalidSlot,
    SlotOccupied,
    InsufficientGold,
    InsufficientItems,
    InvalidItem,
}

/// Imbuement manager
#[derive(Debug, Default)]
pub struct ImbuementManager {
    /// Active imbuements by item unique ID
    item_imbuements: HashMap<u32, Vec<ActiveImbuement>>,
    /// Shrine locations
    shrines: Vec<ImbuementShrine>,
}

impl ImbuementManager {
    /// Create new manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a shrine location
    pub fn register_shrine(&mut self, shrine: ImbuementShrine) {
        self.shrines.push(shrine);
    }

    /// Get imbuements for an item
    pub fn get_imbuements(&self, item_unique_id: u32) -> Option<&Vec<ActiveImbuement>> {
        self.item_imbuements.get(&item_unique_id)
    }

    /// Apply imbuement to an item
    pub fn apply_imbuement(
        &mut self,
        item_unique_id: u32,
        imbuement_type: ImbuementType,
        tier: ImbuementTier,
        slot_index: u8,
        max_slots: u8,
        player_gold: u64,
        player_items: &HashMap<u16, u32>,
        luck_bonus: f32,
    ) -> ImbuementResult {
        // Check gold
        let cost = tier.gold_cost();
        if player_gold < cost {
            return ImbuementResult::InsufficientGold;
        }

        // Check items
        let required = imbuement_type.required_products(tier);
        for product in &required {
            let have = player_items.get(&product.item_id).copied().unwrap_or(0);
            if have < product.count {
                return ImbuementResult::InsufficientItems;
            }
        }

        // Check slot validity
        if slot_index >= max_slots {
            return ImbuementResult::InvalidSlot;
        }

        // Check if slot is occupied
        if let Some(imbuements) = self.item_imbuements.get(&item_unique_id) {
            if imbuements.iter().any(|i| i.slot_index == slot_index) {
                return ImbuementResult::SlotOccupied;
            }
        }

        // Calculate success
        let success_rate = tier.base_success_rate() + luck_bonus;
        let roll: f32 = rand::random::<f32>() * 100.0;

        if roll <= success_rate {
            let imbuement = ActiveImbuement::new(imbuement_type, tier, slot_index);
            self.item_imbuements
                .entry(item_unique_id)
                .or_default()
                .push(imbuement.clone());
            ImbuementResult::Success(imbuement)
        } else {
            ImbuementResult::Failure {
                gold_lost: cost,
                items_lost: required,
            }
        }
    }

    /// Remove imbuement from an item
    pub fn remove_imbuement(&mut self, item_unique_id: u32, slot_index: u8) -> bool {
        if let Some(imbuements) = self.item_imbuements.get_mut(&item_unique_id) {
            let initial_len = imbuements.len();
            imbuements.retain(|i| i.slot_index != slot_index);
            return imbuements.len() < initial_len;
        }
        false
    }

    /// Update all imbuements (called periodically for equipped items)
    pub fn tick(&mut self, item_unique_id: u32, seconds: u32) {
        if let Some(imbuements) = self.item_imbuements.get_mut(&item_unique_id) {
            for imbuement in imbuements.iter_mut() {
                imbuement.consume_time(seconds);
            }
            // Remove expired
            imbuements.retain(|i| !i.is_expired());
        }
    }

    /// Clear imbuements for an item (on item destruction)
    pub fn clear_item(&mut self, item_unique_id: u32) {
        self.item_imbuements.remove(&item_unique_id);
    }

    /// Get total effect value for a type on an item
    pub fn get_total_effect(&self, item_unique_id: u32, imbuement_type: ImbuementType) -> i32 {
        self.item_imbuements
            .get(&item_unique_id)
            .map(|imbuements| {
                imbuements
                    .iter()
                    .filter(|i| i.imbuement_type == imbuement_type)
                    .map(|i| i.effect_value())
                    .sum()
            })
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier_multiplier() {
        assert_eq!(ImbuementTier::Basic.effect_multiplier(), 1.0);
        assert_eq!(ImbuementTier::Intricate.effect_multiplier(), 2.0);
        assert_eq!(ImbuementTier::Powerful.effect_multiplier(), 4.0);
    }

    #[test]
    fn test_imbuement_effect() {
        let imbuement = ActiveImbuement::new(
            ImbuementType::Vampirism,
            ImbuementTier::Powerful,
            0,
        );
        // Base 4 * 4.0 multiplier = 16
        assert_eq!(imbuement.effect_value(), 16);
    }

    #[test]
    fn test_manager() {
        let mut manager = ImbuementManager::new();
        let mut items = HashMap::new();
        items.insert(10605, 25); // Vampire teeth

        let result = manager.apply_imbuement(
            1,
            ImbuementType::Vampirism,
            ImbuementTier::Basic,
            0,
            3,
            100000,
            &items,
            100.0, // Guarantee success
        );

        assert!(matches!(result, ImbuementResult::Success(_)));
    }
}
