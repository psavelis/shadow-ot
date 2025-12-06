//! Forge System
//!
//! Tibia's Exaltation Forge allows players to upgrade equipment
//! using special dust and resources. This module implements:
//! - Dust conversion from items
//! - Equipment tier upgrades
//! - Fusion attempts with success rates
//! - Transfer between items

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Forge dust types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DustType {
    /// Regular dust from boss drops
    Regular,
    /// Exalted dust from rare drops
    Exalted,
}

/// Item classification for forge purposes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ForgeClassification {
    /// Class 1 items (most common)
    Class1,
    /// Class 2 items
    Class2,
    /// Class 3 items
    Class3,
    /// Class 4 items (rarest)
    Class4,
}

impl ForgeClassification {
    /// Get the maximum tier for this classification
    pub fn max_tier(&self) -> u8 {
        match self {
            Self::Class1 => 4,
            Self::Class2 => 4,
            Self::Class3 => 4,
            Self::Class4 => 4,
        }
    }

    /// Get base success rate for upgrading from given tier
    pub fn base_success_rate(&self, current_tier: u8) -> f32 {
        let class_modifier = match self {
            Self::Class1 => 1.0,
            Self::Class2 => 0.9,
            Self::Class3 => 0.8,
            Self::Class4 => 0.7,
        };

        let tier_rate = match current_tier {
            0 => 50.0,
            1 => 35.0,
            2 => 25.0,
            3 => 15.0,
            _ => 0.0,
        };

        tier_rate * class_modifier
    }

    /// Get dust cost for upgrading from given tier
    pub fn dust_cost(&self, current_tier: u8) -> u32 {
        let base_cost = match current_tier {
            0 => 100,
            1 => 200,
            2 => 400,
            3 => 800,
            _ => 0,
        };

        let class_mult = match self {
            Self::Class1 => 1,
            Self::Class2 => 2,
            Self::Class3 => 3,
            Self::Class4 => 4,
        };

        base_cost * class_mult
    }

    /// Get exalted cores required for upgrading from given tier
    pub fn cores_required(&self, current_tier: u8) -> u32 {
        match current_tier {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 4,
            _ => 0,
        }
    }

    /// Get gold cost for upgrading from given tier
    pub fn gold_cost(&self, current_tier: u8) -> u64 {
        let base = match current_tier {
            0 => 100_000,
            1 => 500_000,
            2 => 2_000_000,
            3 => 10_000_000,
            _ => 0,
        };

        let mult = match self {
            Self::Class1 => 1,
            Self::Class2 => 2,
            Self::Class3 => 4,
            Self::Class4 => 8,
        };

        base * mult
    }
}

/// Tier bonus effects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierBonuses {
    /// Attack bonus
    pub attack: i32,
    /// Defense bonus
    pub defense: i32,
    /// Armor bonus
    pub armor: i32,
    /// Special ability bonus (depends on item)
    pub special: i32,
}

impl TierBonuses {
    /// Get bonuses for a tier
    pub fn for_tier(tier: u8) -> Self {
        match tier {
            1 => Self { attack: 1, defense: 1, armor: 1, special: 5 },
            2 => Self { attack: 2, defense: 2, armor: 2, special: 10 },
            3 => Self { attack: 4, defense: 4, armor: 4, special: 20 },
            4 => Self { attack: 8, defense: 8, armor: 8, special: 40 },
            _ => Self { attack: 0, defense: 0, armor: 0, special: 0 },
        }
    }

    /// Get percentage bonus (for skills, etc.)
    pub fn percentage(&self) -> f32 {
        self.special as f32 / 100.0
    }
}

/// Forge item state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeableItem {
    /// Item unique ID
    pub item_unique_id: u32,
    /// Item type ID
    pub item_type_id: u16,
    /// Classification
    pub classification: ForgeClassification,
    /// Current tier
    pub tier: u8,
    /// Fusion attempts count
    pub fusion_attempts: u32,
    /// Last upgrade attempt
    pub last_attempt: Option<DateTime<Utc>>,
}

impl ForgeableItem {
    /// Create new forgeable item
    pub fn new(item_unique_id: u32, item_type_id: u16, classification: ForgeClassification) -> Self {
        Self {
            item_unique_id,
            item_type_id,
            classification,
            tier: 0,
            fusion_attempts: 0,
            last_attempt: None,
        }
    }

    /// Get current tier bonuses
    pub fn bonuses(&self) -> TierBonuses {
        TierBonuses::for_tier(self.tier)
    }

    /// Check if can be upgraded
    pub fn can_upgrade(&self) -> bool {
        self.tier < self.classification.max_tier()
    }

    /// Get upgrade requirements
    pub fn upgrade_requirements(&self) -> UpgradeRequirements {
        UpgradeRequirements {
            dust: self.classification.dust_cost(self.tier),
            cores: self.classification.cores_required(self.tier),
            gold: self.classification.gold_cost(self.tier),
            success_rate: self.classification.base_success_rate(self.tier),
        }
    }
}

/// Requirements for an upgrade
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeRequirements {
    pub dust: u32,
    pub cores: u32,
    pub gold: u64,
    pub success_rate: f32,
}

/// Player's forge resources
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ForgeResources {
    /// Regular dust
    pub dust: u32,
    /// Exalted cores
    pub cores: u32,
    /// Slivers (100 = 1 dust)
    pub slivers: u32,
}

impl ForgeResources {
    /// Convert slivers to dust
    pub fn convert_slivers(&mut self) {
        let new_dust = self.slivers / 100;
        self.dust += new_dust;
        self.slivers %= 100;
    }

    /// Add resources from item decomposition
    pub fn add_from_item(&mut self, item_type_id: u16, _classification: ForgeClassification) {
        // Simplified: each item gives 1-10 slivers based on type
        let slivers = (item_type_id % 10 + 1) as u32;
        self.slivers += slivers;
        self.convert_slivers();
    }

    /// Check if has enough for upgrade
    pub fn can_afford(&self, req: &UpgradeRequirements) -> bool {
        self.dust >= req.dust && self.cores >= req.cores
    }

    /// Spend resources for upgrade
    pub fn spend(&mut self, req: &UpgradeRequirements) -> bool {
        if !self.can_afford(req) {
            return false;
        }
        self.dust -= req.dust;
        self.cores -= req.cores;
        true
    }
}

/// Forge action result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ForgeResult {
    /// Successful upgrade
    Success {
        new_tier: u8,
        bonuses: TierBonuses,
    },
    /// Failed upgrade (item stays same tier)
    Failure {
        current_tier: u8,
        dust_lost: u32,
    },
    /// Item broke (tier reduced by 1)
    Broken {
        new_tier: u8,
    },
    /// Not enough resources
    InsufficientResources {
        needed: UpgradeRequirements,
        have_dust: u32,
        have_cores: u32,
    },
    /// Item cannot be upgraded further
    MaxTierReached,
    /// Item not forgeable
    NotForgeable,
    /// Invalid fusion pair
    InvalidFusion,
    /// Transfer successful
    TransferSuccess {
        tier: u8,
    },
    /// Transfer failed
    TransferFailure,
}

/// Forge history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeHistoryEntry {
    pub player_id: u32,
    pub item_type_id: u16,
    pub action: ForgeAction,
    pub result: ForgeResultType,
    pub timestamp: DateTime<Utc>,
}

/// Type of forge action
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ForgeAction {
    Upgrade,
    Fusion,
    Transfer,
    Decompose,
}

/// Simplified result type for history
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ForgeResultType {
    Success,
    Failure,
    Broken,
}

/// Forge manager
#[derive(Debug, Default)]
pub struct ForgeManager {
    /// Forgeable items by unique ID
    items: HashMap<u32, ForgeableItem>,
    /// Player resources
    resources: HashMap<u32, ForgeResources>,
    /// Item classifications (item_type_id -> classification)
    classifications: HashMap<u16, ForgeClassification>,
    /// History
    history: Vec<ForgeHistoryEntry>,
}

impl ForgeManager {
    /// Create new forge manager
    pub fn new() -> Self {
        let mut manager = Self::default();
        manager.init_classifications();
        manager
    }

    /// Initialize item classifications
    fn init_classifications(&mut self) {
        // Example classifications - in production, load from data files
        // Weapons
        for id in 2400..2450 {
            self.classifications.insert(id, ForgeClassification::Class2);
        }
        // Armor
        for id in 2450..2500 {
            self.classifications.insert(id, ForgeClassification::Class2);
        }
        // Rare weapons
        for id in 8920..8930 {
            self.classifications.insert(id, ForgeClassification::Class4);
        }
    }

    /// Register item as forgeable
    pub fn register_item(&mut self, item_unique_id: u32, item_type_id: u16) -> bool {
        if let Some(&classification) = self.classifications.get(&item_type_id) {
            let item = ForgeableItem::new(item_unique_id, item_type_id, classification);
            self.items.insert(item_unique_id, item);
            true
        } else {
            false
        }
    }

    /// Get forgeable item
    pub fn get_item(&self, item_unique_id: u32) -> Option<&ForgeableItem> {
        self.items.get(&item_unique_id)
    }

    /// Get mutable forgeable item
    pub fn get_item_mut(&mut self, item_unique_id: u32) -> Option<&mut ForgeableItem> {
        self.items.get_mut(&item_unique_id)
    }

    /// Get player resources
    pub fn get_resources(&self, player_id: u32) -> ForgeResources {
        self.resources.get(&player_id).cloned().unwrap_or_default()
    }

    /// Get mutable player resources
    pub fn get_resources_mut(&mut self, player_id: u32) -> &mut ForgeResources {
        self.resources.entry(player_id).or_default()
    }

    /// Attempt to upgrade an item
    pub fn upgrade(
        &mut self,
        player_id: u32,
        item_unique_id: u32,
        player_gold: u64,
        luck_bonus: f32,
    ) -> ForgeResult {
        let now = Utc::now();

        // Get item
        let item = match self.items.get(&item_unique_id) {
            Some(i) => i.clone(),
            None => return ForgeResult::NotForgeable,
        };

        // Check if can upgrade
        if !item.can_upgrade() {
            return ForgeResult::MaxTierReached;
        }

        // Get requirements
        let req = item.upgrade_requirements();

        // Check gold
        if player_gold < req.gold {
            return ForgeResult::InsufficientResources {
                needed: req,
                have_dust: 0,
                have_cores: 0,
            };
        }

        // Check resources
        let resources = self.get_resources(player_id);
        if !resources.can_afford(&req) {
            return ForgeResult::InsufficientResources {
                needed: req.clone(),
                have_dust: resources.dust,
                have_cores: resources.cores,
            };
        }

        // Spend resources
        self.get_resources_mut(player_id).spend(&req);

        // Calculate success
        let success_rate = req.success_rate + luck_bonus;
        let roll: f32 = rand::random::<f32>() * 100.0;

        let item_mut = self.items.get_mut(&item_unique_id).unwrap();
        item_mut.fusion_attempts += 1;
        item_mut.last_attempt = Some(now);

        if roll <= success_rate {
            // Success
            item_mut.tier += 1;
            let new_tier = item_mut.tier;
            self.record_history(player_id, item.item_type_id, ForgeAction::Upgrade, ForgeResultType::Success);

            ForgeResult::Success {
                new_tier,
                bonuses: TierBonuses::for_tier(new_tier),
            }
        } else if roll > 95.0 && item_mut.tier > 0 {
            // Critical failure - tier reduced
            item_mut.tier -= 1;
            let new_tier = item_mut.tier;
            self.record_history(player_id, item.item_type_id, ForgeAction::Upgrade, ForgeResultType::Broken);

            ForgeResult::Broken { new_tier }
        } else {
            // Normal failure
            self.record_history(player_id, item.item_type_id, ForgeAction::Upgrade, ForgeResultType::Failure);

            ForgeResult::Failure {
                current_tier: item_mut.tier,
                dust_lost: req.dust,
            }
        }
    }

    /// Fuse two items of same tier to guarantee upgrade
    pub fn fuse(
        &mut self,
        player_id: u32,
        item1_id: u32,
        item2_id: u32,
        player_gold: u64,
    ) -> ForgeResult {
        // Get both items
        let item1 = match self.items.get(&item1_id) {
            Some(i) => i.clone(),
            None => return ForgeResult::NotForgeable,
        };

        let item2 = match self.items.get(&item2_id) {
            Some(i) => i.clone(),
            None => return ForgeResult::NotForgeable,
        };

        // Must be same type and tier
        if item1.item_type_id != item2.item_type_id || item1.tier != item2.tier {
            return ForgeResult::InvalidFusion;
        }

        // Must be able to upgrade
        if !item1.can_upgrade() {
            return ForgeResult::MaxTierReached;
        }

        // Calculate cost (double normal)
        let req = item1.upgrade_requirements();
        if player_gold < req.gold * 2 {
            return ForgeResult::InsufficientResources {
                needed: UpgradeRequirements {
                    dust: req.dust * 2,
                    cores: req.cores * 2,
                    gold: req.gold * 2,
                    success_rate: 100.0,
                },
                have_dust: 0,
                have_cores: 0,
            };
        }

        // Fusion always succeeds - item2 is consumed
        let item1_mut = self.items.get_mut(&item1_id).unwrap();
        item1_mut.tier += 1;
        let new_tier = item1_mut.tier;

        // Remove second item
        self.items.remove(&item2_id);

        self.record_history(player_id, item1.item_type_id, ForgeAction::Fusion, ForgeResultType::Success);

        ForgeResult::Success {
            new_tier,
            bonuses: TierBonuses::for_tier(new_tier),
        }
    }

    /// Transfer tier from one item to another (consumes source)
    pub fn transfer(
        &mut self,
        player_id: u32,
        source_id: u32,
        target_id: u32,
        player_gold: u64,
    ) -> ForgeResult {
        let source = match self.items.get(&source_id) {
            Some(i) => i.clone(),
            None => return ForgeResult::NotForgeable,
        };

        let target = match self.items.get(&target_id) {
            Some(i) => i.clone(),
            None => return ForgeResult::NotForgeable,
        };

        // Source must have tier > 0
        if source.tier == 0 {
            return ForgeResult::InvalidFusion;
        }

        // Target must be tier 0
        if target.tier > 0 {
            return ForgeResult::InvalidFusion;
        }

        // Must be same classification
        if source.classification != target.classification {
            return ForgeResult::InvalidFusion;
        }

        // Cost based on source tier
        let gold_cost = 1_000_000u64 * source.tier as u64;
        if player_gold < gold_cost {
            return ForgeResult::InsufficientResources {
                needed: UpgradeRequirements {
                    dust: 0,
                    cores: 0,
                    gold: gold_cost,
                    success_rate: 0.0,
                },
                have_dust: 0,
                have_cores: 0,
            };
        }

        // 50% chance to transfer successfully
        let roll: f32 = rand::random::<f32>() * 100.0;

        if roll <= 50.0 {
            // Transfer tier to target
            let tier = source.tier;
            let target_mut = self.items.get_mut(&target_id).unwrap();
            target_mut.tier = tier;

            // Remove source
            self.items.remove(&source_id);

            self.record_history(player_id, target.item_type_id, ForgeAction::Transfer, ForgeResultType::Success);

            ForgeResult::TransferSuccess { tier }
        } else {
            // Transfer failed - both items lose tier
            let source_mut = self.items.get_mut(&source_id).unwrap();
            source_mut.tier = 0;

            self.record_history(player_id, target.item_type_id, ForgeAction::Transfer, ForgeResultType::Failure);

            ForgeResult::TransferFailure
        }
    }

    /// Decompose item for resources
    pub fn decompose(&mut self, player_id: u32, item_unique_id: u32) {
        if let Some(item) = self.items.remove(&item_unique_id) {
            // Base resources from item
            let base_slivers = match item.classification {
                ForgeClassification::Class1 => 10,
                ForgeClassification::Class2 => 25,
                ForgeClassification::Class3 => 50,
                ForgeClassification::Class4 => 100,
            };

            // Bonus from tier
            let tier_bonus = item.tier as u32 * 25;

            let resources = self.get_resources_mut(player_id);
            resources.slivers += base_slivers + tier_bonus;
            resources.convert_slivers();

            self.record_history(player_id, item.item_type_id, ForgeAction::Decompose, ForgeResultType::Success);
        }
    }

    /// Record history entry
    fn record_history(&mut self, player_id: u32, item_type_id: u16, action: ForgeAction, result: ForgeResultType) {
        self.history.push(ForgeHistoryEntry {
            player_id,
            item_type_id,
            action,
            result,
            timestamp: Utc::now(),
        });
    }

    /// Get player history
    pub fn get_history(&self, player_id: u32, limit: usize) -> Vec<&ForgeHistoryEntry> {
        self.history
            .iter()
            .filter(|h| h.player_id == player_id)
            .rev()
            .take(limit)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier_bonuses() {
        let t1 = TierBonuses::for_tier(1);
        let t4 = TierBonuses::for_tier(4);

        assert_eq!(t1.attack, 1);
        assert_eq!(t4.attack, 8);
    }

    #[test]
    fn test_upgrade_requirements() {
        let item = ForgeableItem::new(1, 2400, ForgeClassification::Class2);
        let req = item.upgrade_requirements();

        assert!(req.dust > 0);
        assert!(req.gold > 0);
    }

    #[test]
    fn test_resources() {
        let mut resources = ForgeResources::default();
        resources.slivers = 250;
        resources.convert_slivers();

        assert_eq!(resources.dust, 2);
        assert_eq!(resources.slivers, 50);
    }

    #[test]
    fn test_forge_manager() {
        let mut manager = ForgeManager::new();

        // Register an item
        assert!(manager.register_item(1, 2400));

        // Get item
        let item = manager.get_item(1).unwrap();
        assert_eq!(item.tier, 0);
        assert!(item.can_upgrade());
    }
}
