//! VIP (Premium) System
//!
//! Handles premium account features, benefits, and subscription management.
//! Similar to Tibia's premium account but with enhanced features.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// VIP tier levels with increasing benefits
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum VipTier {
    /// Free player (no VIP)
    None,
    /// Basic VIP - essential premium features
    Bronze,
    /// Enhanced VIP - additional conveniences
    Silver,
    /// Full VIP - all premium features
    Gold,
    /// Maximum VIP - exclusive benefits
    Platinum,
}

impl VipTier {
    /// Get experience bonus multiplier for this tier
    pub fn exp_bonus(&self) -> f64 {
        match self {
            VipTier::None => 1.0,
            VipTier::Bronze => 1.10,   // +10%
            VipTier::Silver => 1.20,   // +20%
            VipTier::Gold => 1.35,     // +35%
            VipTier::Platinum => 1.50, // +50%
        }
    }

    /// Get loot bonus multiplier for this tier
    pub fn loot_bonus(&self) -> f64 {
        match self {
            VipTier::None => 1.0,
            VipTier::Bronze => 1.05,   // +5%
            VipTier::Silver => 1.10,   // +10%
            VipTier::Gold => 1.20,     // +20%
            VipTier::Platinum => 1.30, // +30%
        }
    }

    /// Get skill training bonus for this tier
    pub fn skill_bonus(&self) -> f64 {
        match self {
            VipTier::None => 1.0,
            VipTier::Bronze => 1.05,
            VipTier::Silver => 1.10,
            VipTier::Gold => 1.20,
            VipTier::Platinum => 1.30,
        }
    }

    /// Number of depot pages available
    pub fn depot_pages(&self) -> u32 {
        match self {
            VipTier::None => 2,
            VipTier::Bronze => 4,
            VipTier::Silver => 6,
            VipTier::Gold => 10,
            VipTier::Platinum => 20,
        }
    }

    /// Maximum VIP list entries
    pub fn vip_list_size(&self) -> usize {
        match self {
            VipTier::None => 20,
            VipTier::Bronze => 50,
            VipTier::Silver => 100,
            VipTier::Gold => 200,
            VipTier::Platinum => 500,
        }
    }

    /// Can use premium-only transportation
    pub fn can_use_premium_transport(&self) -> bool {
        *self >= VipTier::Bronze
    }

    /// Can access premium-only areas
    pub fn can_access_premium_areas(&self) -> bool {
        *self >= VipTier::Bronze
    }

    /// Can rent houses
    pub fn can_rent_houses(&self) -> bool {
        *self >= VipTier::Bronze
    }

    /// Has offline training
    pub fn has_offline_training(&self) -> bool {
        *self >= VipTier::Bronze
    }

    /// Offline training duration limit (hours)
    pub fn offline_training_hours(&self) -> u32 {
        match self {
            VipTier::None => 0,
            VipTier::Bronze => 12,
            VipTier::Silver => 24,
            VipTier::Gold => 48,
            VipTier::Platinum => 168, // 1 week
        }
    }

    /// Has instant travel (no waiting)
    pub fn has_instant_travel(&self) -> bool {
        *self >= VipTier::Gold
    }

    /// Can create guilds
    pub fn can_create_guild(&self) -> bool {
        *self >= VipTier::Bronze
    }

    /// Max characters per account
    pub fn max_characters(&self) -> u32 {
        match self {
            VipTier::None => 3,
            VipTier::Bronze => 5,
            VipTier::Silver => 8,
            VipTier::Gold => 12,
            VipTier::Platinum => 20,
        }
    }

    /// Has priority login queue
    pub fn has_priority_login(&self) -> bool {
        *self >= VipTier::Silver
    }

    /// Can use prey system
    pub fn can_use_prey(&self) -> bool {
        *self >= VipTier::Bronze
    }

    /// Number of prey slots
    pub fn prey_slots(&self) -> u32 {
        match self {
            VipTier::None => 0,
            VipTier::Bronze => 1,
            VipTier::Silver => 2,
            VipTier::Gold => 3,
            VipTier::Platinum => 3,
        }
    }

    /// Death penalty reduction percentage
    pub fn death_penalty_reduction(&self) -> f64 {
        match self {
            VipTier::None => 0.0,
            VipTier::Bronze => 0.10,   // -10%
            VipTier::Silver => 0.20,   // -20%
            VipTier::Gold => 0.30,     // -30%
            VipTier::Platinum => 0.50, // -50%
        }
    }

    /// Can access exclusive mounts/outfits
    pub fn has_exclusive_cosmetics(&self) -> bool {
        *self >= VipTier::Gold
    }
}

/// VIP subscription status for an account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VipStatus {
    /// Account ID
    pub account_id: Uuid,
    /// Current VIP tier
    pub tier: VipTier,
    /// When VIP expires (None if never had VIP)
    pub expires_at: Option<DateTime<Utc>>,
    /// Total days of VIP ever purchased
    pub total_days_purchased: u32,
    /// VIP loyalty level (based on total days)
    pub loyalty_level: u32,
    /// Special perks unlocked
    pub perks: Vec<VipPerk>,
    /// When VIP was first activated
    pub first_activated: Option<DateTime<Utc>>,
    /// Last renewal date
    pub last_renewed: Option<DateTime<Utc>>,
}

impl VipStatus {
    /// Create a new non-VIP status
    pub fn new(account_id: Uuid) -> Self {
        Self {
            account_id,
            tier: VipTier::None,
            expires_at: None,
            total_days_purchased: 0,
            loyalty_level: 0,
            perks: Vec::new(),
            first_activated: None,
            last_renewed: None,
        }
    }

    /// Check if VIP is currently active
    pub fn is_active(&self) -> bool {
        if self.tier == VipTier::None {
            return false;
        }
        match self.expires_at {
            Some(expires) => Utc::now() < expires,
            None => false,
        }
    }

    /// Get remaining VIP days
    pub fn remaining_days(&self) -> i64 {
        match self.expires_at {
            Some(expires) => {
                let remaining = expires - Utc::now();
                remaining.num_days().max(0)
            }
            None => 0,
        }
    }

    /// Get effective tier (None if expired)
    pub fn effective_tier(&self) -> VipTier {
        if self.is_active() {
            self.tier
        } else {
            VipTier::None
        }
    }

    /// Add VIP time
    pub fn add_time(&mut self, days: u32, tier: VipTier) {
        let now = Utc::now();
        
        // First time activation
        if self.first_activated.is_none() {
            self.first_activated = Some(now);
        }
        
        self.last_renewed = Some(now);
        self.total_days_purchased += days;
        
        // Upgrade tier if new tier is higher
        if tier > self.tier {
            self.tier = tier;
        }
        
        // Extend expiration
        let new_expiry = match self.expires_at {
            Some(current) if current > now => {
                current + Duration::days(days as i64)
            }
            _ => now + Duration::days(days as i64),
        };
        self.expires_at = Some(new_expiry);
        
        // Update loyalty level
        self.update_loyalty();
    }

    /// Update loyalty level based on total days
    fn update_loyalty(&mut self) {
        self.loyalty_level = match self.total_days_purchased {
            0..=29 => 0,
            30..=89 => 1,
            90..=179 => 2,
            180..=364 => 3,
            365..=729 => 4,
            730..=1459 => 5,
            _ => 6,
        };
    }

    /// Get loyalty bonus (additional exp/loot based on loyalty)
    pub fn loyalty_bonus(&self) -> f64 {
        match self.loyalty_level {
            0 => 1.0,
            1 => 1.02,
            2 => 1.05,
            3 => 1.08,
            4 => 1.12,
            5 => 1.15,
            _ => 1.20,
        }
    }
}

/// Special VIP perks that can be unlocked
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VipPerk {
    /// Instant resurrection at temple
    InstantResurrection,
    /// No item drop on death
    NoItemDrop,
    /// Double stamina regeneration
    DoubleStamina,
    /// Free house teleportation
    FreeHouseTeleport,
    /// Access to exclusive boss fights
    ExclusiveBosses,
    /// Daily reward chest bonus
    DailyRewardBonus,
    /// Free depot transfer between cities
    FreeDepotTransfer,
    /// Unlimited market listings
    UnlimitedMarket,
    /// Priority customer support
    PrioritySupport,
    /// Beta access to new features
    BetaAccess,
    /// Custom title prefix
    CustomTitle,
    /// Special effects on spells
    SpellEffects,
    /// Mount speed bonus
    MountSpeedBonus,
    /// Extra outfit addons
    ExtraAddons,
}

/// VIP package that can be purchased
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VipPackage {
    /// Package ID
    pub id: String,
    /// Display name
    pub name: String,
    /// Description
    pub description: String,
    /// VIP tier granted
    pub tier: VipTier,
    /// Duration in days
    pub days: u32,
    /// Price in coins (premium currency)
    pub price_coins: u32,
    /// Bonus perks included
    pub bonus_perks: Vec<VipPerk>,
    /// Is currently available
    pub available: bool,
    /// Discount percentage (0-100)
    pub discount: u8,
}

impl VipPackage {
    pub fn bronze_30() -> Self {
        Self {
            id: "bronze_30".to_string(),
            name: "Bronze VIP (30 Days)".to_string(),
            description: "Essential premium features for 30 days".to_string(),
            tier: VipTier::Bronze,
            days: 30,
            price_coins: 500,
            bonus_perks: Vec::new(),
            available: true,
            discount: 0,
        }
    }

    pub fn silver_30() -> Self {
        Self {
            id: "silver_30".to_string(),
            name: "Silver VIP (30 Days)".to_string(),
            description: "Enhanced premium features for 30 days".to_string(),
            tier: VipTier::Silver,
            days: 30,
            price_coins: 800,
            bonus_perks: Vec::new(),
            available: true,
            discount: 0,
        }
    }

    pub fn gold_30() -> Self {
        Self {
            id: "gold_30".to_string(),
            name: "Gold VIP (30 Days)".to_string(),
            description: "Full premium features for 30 days".to_string(),
            tier: VipTier::Gold,
            days: 30,
            price_coins: 1200,
            bonus_perks: vec![VipPerk::DailyRewardBonus],
            available: true,
            discount: 0,
        }
    }

    pub fn platinum_30() -> Self {
        Self {
            id: "platinum_30".to_string(),
            name: "Platinum VIP (30 Days)".to_string(),
            description: "Maximum premium features for 30 days".to_string(),
            tier: VipTier::Platinum,
            days: 30,
            price_coins: 2000,
            bonus_perks: vec![VipPerk::DailyRewardBonus, VipPerk::ExclusiveBosses],
            available: true,
            discount: 0,
        }
    }

    /// Get final price after discount
    pub fn final_price(&self) -> u32 {
        let discount_amount = (self.price_coins as f64 * (self.discount as f64 / 100.0)) as u32;
        self.price_coins - discount_amount
    }
}

/// Daily login rewards for VIP players
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyReward {
    /// Day number in the cycle (1-28)
    pub day: u32,
    /// Reward items (item_id, count)
    pub items: Vec<(u32, u32)>,
    /// Bonus gold
    pub gold: u32,
    /// Bonus experience
    pub experience: u64,
    /// Special reward (tier required)
    pub special_tier: Option<VipTier>,
}

/// VIP Manager handles all VIP-related operations
pub struct VipManager {
    /// Account VIP statuses (account_id -> status)
    statuses: HashMap<Uuid, VipStatus>,
    /// Available VIP packages
    packages: Vec<VipPackage>,
    /// Daily rewards cycle
    daily_rewards: Vec<DailyReward>,
}

impl VipManager {
    /// Create a new VIP manager
    pub fn new() -> Self {
        Self {
            statuses: HashMap::new(),
            packages: vec![
                VipPackage::bronze_30(),
                VipPackage::silver_30(),
                VipPackage::gold_30(),
                VipPackage::platinum_30(),
            ],
            daily_rewards: Self::create_default_rewards(),
        }
    }

    /// Create default daily rewards cycle
    fn create_default_rewards() -> Vec<DailyReward> {
        let mut rewards = Vec::new();
        
        for day in 1..=28 {
            let reward = match day {
                7 | 14 | 21 | 28 => DailyReward {
                    day,
                    items: vec![(2160, 5)], // 5x crystal coins
                    gold: 10000,
                    experience: 50000,
                    special_tier: Some(VipTier::Gold),
                },
                _ => DailyReward {
                    day,
                    items: vec![(2152, 10)], // 10x platinum coins
                    gold: 1000,
                    experience: 5000,
                    special_tier: None,
                },
            };
            rewards.push(reward);
        }
        
        rewards
    }

    /// Get or create VIP status for an account
    pub fn get_status(&mut self, account_id: Uuid) -> &VipStatus {
        self.statuses.entry(account_id)
            .or_insert_with(|| VipStatus::new(account_id))
    }

    /// Get mutable VIP status
    pub fn get_status_mut(&mut self, account_id: Uuid) -> &mut VipStatus {
        self.statuses.entry(account_id)
            .or_insert_with(|| VipStatus::new(account_id))
    }

    /// Purchase a VIP package
    pub fn purchase_package(
        &mut self,
        account_id: Uuid,
        package_id: &str,
    ) -> Result<(), VipError> {
        let package = self.packages.iter()
            .find(|p| p.id == package_id)
            .ok_or(VipError::PackageNotFound)?;

        if !package.available {
            return Err(VipError::PackageUnavailable);
        }

        let package = package.clone();
        let status = self.get_status_mut(account_id);
        status.add_time(package.days, package.tier);

        // Add bonus perks
        for perk in &package.bonus_perks {
            if !status.perks.contains(perk) {
                status.perks.push(*perk);
            }
        }

        Ok(())
    }

    /// Check if player can access a feature
    pub fn can_access_feature(&self, account_id: Uuid, tier_required: VipTier) -> bool {
        match self.statuses.get(&account_id) {
            Some(status) => status.effective_tier() >= tier_required,
            None => tier_required == VipTier::None,
        }
    }

    /// Get total bonuses for a player
    pub fn get_bonuses(&self, account_id: Uuid) -> VipBonuses {
        match self.statuses.get(&account_id) {
            Some(status) if status.is_active() => {
                let tier = status.tier;
                let loyalty = status.loyalty_bonus();
                VipBonuses {
                    exp_multiplier: tier.exp_bonus() * loyalty,
                    loot_multiplier: tier.loot_bonus() * loyalty,
                    skill_multiplier: tier.skill_bonus(),
                    death_penalty_reduction: tier.death_penalty_reduction(),
                }
            }
            _ => VipBonuses::default(),
        }
    }

    /// Get daily reward for a specific day
    pub fn get_daily_reward(&self, day: u32, tier: VipTier) -> Option<&DailyReward> {
        let day_index = ((day - 1) % 28) as usize;
        self.daily_rewards.get(day_index).and_then(|reward| {
            match reward.special_tier {
                Some(required) if tier < required => None,
                _ => Some(reward),
            }
        })
    }

    /// Get available packages
    pub fn get_packages(&self) -> &[VipPackage] {
        &self.packages
    }
}

impl Default for VipManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Aggregated VIP bonuses
#[derive(Debug, Clone, Copy, Default)]
pub struct VipBonuses {
    pub exp_multiplier: f64,
    pub loot_multiplier: f64,
    pub skill_multiplier: f64,
    pub death_penalty_reduction: f64,
}

impl VipBonuses {
    pub fn none() -> Self {
        Self {
            exp_multiplier: 1.0,
            loot_multiplier: 1.0,
            skill_multiplier: 1.0,
            death_penalty_reduction: 0.0,
        }
    }
}

/// VIP system errors
#[derive(Debug, Clone)]
pub enum VipError {
    PackageNotFound,
    PackageUnavailable,
    InsufficientFunds,
    AlreadyMaxTier,
    DatabaseError(String),
}

impl std::fmt::Display for VipError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VipError::PackageNotFound => write!(f, "VIP package not found"),
            VipError::PackageUnavailable => write!(f, "VIP package is currently unavailable"),
            VipError::InsufficientFunds => write!(f, "Insufficient funds to purchase VIP"),
            VipError::AlreadyMaxTier => write!(f, "Already at maximum VIP tier"),
            VipError::DatabaseError(e) => write!(f, "Database error: {}", e),
        }
    }
}

impl std::error::Error for VipError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vip_tier_ordering() {
        assert!(VipTier::Platinum > VipTier::Gold);
        assert!(VipTier::Gold > VipTier::Silver);
        assert!(VipTier::Silver > VipTier::Bronze);
        assert!(VipTier::Bronze > VipTier::None);
    }

    #[test]
    fn test_vip_status_add_time() {
        let mut status = VipStatus::new(Uuid::new_v4());
        assert!(!status.is_active());
        
        status.add_time(30, VipTier::Bronze);
        assert!(status.is_active());
        assert_eq!(status.tier, VipTier::Bronze);
        assert!(status.remaining_days() >= 29);
    }

    #[test]
    fn test_vip_tier_upgrade() {
        let mut status = VipStatus::new(Uuid::new_v4());
        status.add_time(30, VipTier::Bronze);
        assert_eq!(status.tier, VipTier::Bronze);
        
        status.add_time(30, VipTier::Gold);
        assert_eq!(status.tier, VipTier::Gold);
        assert!(status.remaining_days() >= 59);
    }

    #[test]
    fn test_exp_bonus() {
        assert_eq!(VipTier::None.exp_bonus(), 1.0);
        assert_eq!(VipTier::Platinum.exp_bonus(), 1.50);
    }
}
