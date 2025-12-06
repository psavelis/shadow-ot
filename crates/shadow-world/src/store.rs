//! In-Game Store System
//!
//! Tibia's premium store allows players to purchase cosmetics,
//! services, and premium time using Tibia Coins.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Store currency
pub type TibiaCoins = u32;

/// Store category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StoreCategory {
    /// Premium time
    Premium,
    /// Useful items (prey wildcards, etc.)
    Useful,
    /// Boosts (XP, loot, etc.)
    Boosts,
    /// Cosmetics - Outfits
    Outfits,
    /// Cosmetics - Mounts
    Mounts,
    /// Cosmetics - Hireling
    Hirelings,
    /// Cosmetics - House decorations
    HouseDecorations,
    /// Extra services
    ExtraServices,
    /// Tournament coins
    Tournament,
    /// Tibia coin packages (real money)
    CoinPackages,
}

impl StoreCategory {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Premium => "Premium Time",
            Self::Useful => "Useful Things",
            Self::Boosts => "Boosts",
            Self::Outfits => "Outfits",
            Self::Mounts => "Mounts",
            Self::Hirelings => "Hirelings",
            Self::HouseDecorations => "House Decorations",
            Self::ExtraServices => "Extra Services",
            Self::Tournament => "Tournament",
            Self::CoinPackages => "Tibia Coins",
        }
    }

    /// Get icon ID
    pub fn icon_id(&self) -> u16 {
        match self {
            Self::Premium => 1,
            Self::Useful => 2,
            Self::Boosts => 3,
            Self::Outfits => 4,
            Self::Mounts => 5,
            Self::Hirelings => 6,
            Self::HouseDecorations => 7,
            Self::ExtraServices => 8,
            Self::Tournament => 9,
            Self::CoinPackages => 10,
        }
    }
}

/// Store offer type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OfferType {
    /// One-time purchase
    OneTime,
    /// Subscription
    Subscription,
    /// Consumable (can buy multiple)
    Consumable,
    /// Limited time offer
    LimitedTime,
    /// Bundle
    Bundle,
}

/// Store offer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreOffer {
    /// Unique offer ID
    pub id: u32,
    /// Category
    pub category: StoreCategory,
    /// Offer type
    pub offer_type: OfferType,
    /// Display name
    pub name: String,
    /// Description
    pub description: String,
    /// Price in Tibia Coins
    pub price: TibiaCoins,
    /// Original price (for sales)
    pub original_price: Option<TibiaCoins>,
    /// Icon/sprite ID
    pub icon_id: u16,
    /// Whether this is highlighted/featured
    pub featured: bool,
    /// Whether this is on sale
    pub on_sale: bool,
    /// Sale end date
    pub sale_end: Option<DateTime<Utc>>,
    /// Items/rewards given
    pub rewards: Vec<StoreReward>,
    /// Requirements
    pub requirements: Vec<StoreRequirement>,
    /// Maximum purchases (0 = unlimited)
    pub max_purchases: u32,
    /// If premium is required
    pub requires_premium: bool,
    /// Minimum level required
    pub min_level: u16,
    /// Available from
    pub available_from: Option<DateTime<Utc>>,
    /// Available until
    pub available_until: Option<DateTime<Utc>>,
}

impl StoreOffer {
    /// Check if offer is available
    pub fn is_available(&self, now: DateTime<Utc>) -> bool {
        if let Some(from) = self.available_from {
            if now < from {
                return false;
            }
        }
        if let Some(until) = self.available_until {
            if now > until {
                return false;
            }
        }
        true
    }

    /// Get effective price (considers sale)
    pub fn effective_price(&self) -> TibiaCoins {
        self.price
    }

    /// Get discount percentage
    pub fn discount_percent(&self) -> Option<u8> {
        self.original_price.map(|original| {
            ((1.0 - self.price as f32 / original as f32) * 100.0) as u8
        })
    }
}

/// Store reward - what the player gets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StoreReward {
    /// Premium days
    PremiumDays(u32),
    /// Outfit (look_type, addons_mask)
    Outfit { look_type: u16, addons: u8 },
    /// Mount (mount_id)
    Mount(u16),
    /// Item (item_id, count)
    Item { item_id: u16, count: u32 },
    /// Hireling
    Hireling { name: String, outfit: u16 },
    /// House decoration
    HouseDecoration { item_id: u16 },
    /// XP Boost (percentage, hours)
    XpBoost { percentage: u8, hours: u32 },
    /// Prey wildcard count
    PreyWildcard(u32),
    /// Character rename
    CharacterRename,
    /// Sex change
    SexChange,
    /// World transfer
    WorldTransfer,
    /// Name change
    NameChange,
    /// Charm expansion
    CharmExpansion,
    /// Huntsman outfit
    HuntsmanOutfit,
    /// Tournament coins
    TournamentCoins(u32),
    /// Blessing
    Blessing { blessing_id: u8 },
    /// Instant reward access
    InstantRewardAccess,
    /// Gold pouch (capacity increase)
    GoldPouch { capacity: u32 },
}

/// Store requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StoreRequirement {
    /// Premium required
    Premium,
    /// Minimum level
    MinLevel(u16),
    /// Maximum level
    MaxLevel(u16),
    /// Specific vocation
    Vocation(u8),
    /// Not already owned (for one-time purchases)
    NotOwned,
    /// Has specific outfit
    HasOutfit(u16),
    /// Has specific mount
    HasMount(u16),
}

/// Store transaction history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreTransaction {
    /// Transaction ID
    pub id: Uuid,
    /// Player ID
    pub player_id: u32,
    /// Offer ID
    pub offer_id: u32,
    /// Amount paid
    pub amount: TibiaCoins,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Status
    pub status: TransactionStatus,
    /// Recipient (for gifts)
    pub recipient_id: Option<u32>,
}

/// Transaction status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Refunded,
}

/// Player's Tibia Coin balance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinBalance {
    /// Transferable coins (can be traded/gifted)
    pub transferable: TibiaCoins,
    /// Non-transferable coins (bound to account)
    pub non_transferable: TibiaCoins,
    /// Tournament coins
    pub tournament: TibiaCoins,
}

impl CoinBalance {
    /// Get total coins
    pub fn total(&self) -> TibiaCoins {
        self.transferable + self.non_transferable
    }

    /// Check if can afford
    pub fn can_afford(&self, amount: TibiaCoins) -> bool {
        self.total() >= amount
    }

    /// Spend coins (uses non-transferable first)
    pub fn spend(&mut self, amount: TibiaCoins) -> bool {
        if !self.can_afford(amount) {
            return false;
        }

        let mut remaining = amount;

        // Use non-transferable first
        if self.non_transferable >= remaining {
            self.non_transferable -= remaining;
            return true;
        }

        remaining -= self.non_transferable;
        self.non_transferable = 0;
        self.transferable -= remaining;
        true
    }

    /// Add coins
    pub fn add(&mut self, transferable: TibiaCoins, non_transferable: TibiaCoins) {
        self.transferable += transferable;
        self.non_transferable += non_transferable;
    }
}

impl Default for CoinBalance {
    fn default() -> Self {
        Self {
            transferable: 0,
            non_transferable: 0,
            tournament: 0,
        }
    }
}

/// Purchase result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PurchaseResult {
    Success { transaction_id: Uuid, rewards: Vec<StoreReward> },
    InsufficientCoins { needed: TibiaCoins, have: TibiaCoins },
    OfferNotFound,
    OfferUnavailable,
    RequirementsNotMet { failed: Vec<String> },
    AlreadyOwned,
    MaxPurchasesReached,
    InvalidRecipient,
}

/// Store manager
#[derive(Debug)]
pub struct StoreManager {
    /// Available offers by ID
    offers: HashMap<u32, StoreOffer>,
    /// Offers by category
    offers_by_category: HashMap<StoreCategory, Vec<u32>>,
    /// Player balances
    balances: HashMap<u32, CoinBalance>,
    /// Transaction history
    transactions: Vec<StoreTransaction>,
    /// Player purchase counts (player_id -> offer_id -> count)
    purchase_counts: HashMap<u32, HashMap<u32, u32>>,
    /// Player owned items (outfits, mounts)
    player_owned: HashMap<u32, PlayerOwnedStore>,
}

/// Player's owned store items
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlayerOwnedStore {
    pub outfits: HashMap<u16, u8>, // look_type -> addons_mask
    pub mounts: Vec<u16>,
    pub hirelings: Vec<String>,
    pub house_decorations: Vec<u16>,
    pub premium_days_remaining: u32,
    pub xp_boost_hours: u32,
    pub xp_boost_percentage: u8,
    pub prey_wildcards: u32,
    pub charm_expansion: bool,
    pub gold_pouch_capacity: u32,
}

impl StoreManager {
    /// Create new store manager
    pub fn new() -> Self {
        let mut manager = Self {
            offers: HashMap::new(),
            offers_by_category: HashMap::new(),
            balances: HashMap::new(),
            transactions: Vec::new(),
            purchase_counts: HashMap::new(),
            player_owned: HashMap::new(),
        };

        // Initialize default offers
        manager.init_default_offers();
        manager
    }

    /// Initialize default store offers
    fn init_default_offers(&mut self) {
        // Premium Time
        self.add_offer(StoreOffer {
            id: 1,
            category: StoreCategory::Premium,
            offer_type: OfferType::Consumable,
            name: "30 Days Premium".into(),
            description: "Add 30 days of premium time to your account.".into(),
            price: 250,
            original_price: None,
            icon_id: 100,
            featured: true,
            on_sale: false,
            sale_end: None,
            rewards: vec![StoreReward::PremiumDays(30)],
            requirements: vec![],
            max_purchases: 0,
            requires_premium: false,
            min_level: 0,
            available_from: None,
            available_until: None,
        });

        // XP Boost
        self.add_offer(StoreOffer {
            id: 2,
            category: StoreCategory::Boosts,
            offer_type: OfferType::Consumable,
            name: "XP Boost".into(),
            description: "50% bonus experience for 1 hour.".into(),
            price: 30,
            original_price: None,
            icon_id: 101,
            featured: false,
            on_sale: false,
            sale_end: None,
            rewards: vec![StoreReward::XpBoost { percentage: 50, hours: 1 }],
            requirements: vec![StoreRequirement::Premium],
            max_purchases: 0,
            requires_premium: true,
            min_level: 0,
            available_from: None,
            available_until: None,
        });

        // Prey Wildcards
        self.add_offer(StoreOffer {
            id: 3,
            category: StoreCategory::Useful,
            offer_type: OfferType::Consumable,
            name: "5 Prey Wildcards".into(),
            description: "Use to select any creature for your prey.".into(),
            price: 50,
            original_price: None,
            icon_id: 102,
            featured: false,
            on_sale: false,
            sale_end: None,
            rewards: vec![StoreReward::PreyWildcard(5)],
            requirements: vec![StoreRequirement::Premium],
            max_purchases: 0,
            requires_premium: true,
            min_level: 0,
            available_from: None,
            available_until: None,
        });

        // Character rename
        self.add_offer(StoreOffer {
            id: 4,
            category: StoreCategory::ExtraServices,
            offer_type: OfferType::Consumable,
            name: "Name Change".into(),
            description: "Change your character's name.".into(),
            price: 250,
            original_price: None,
            icon_id: 103,
            featured: false,
            on_sale: false,
            sale_end: None,
            rewards: vec![StoreReward::NameChange],
            requirements: vec![],
            max_purchases: 0,
            requires_premium: false,
            min_level: 0,
            available_from: None,
            available_until: None,
        });

        // Blessings
        self.add_offer(StoreOffer {
            id: 5,
            category: StoreCategory::Useful,
            offer_type: OfferType::Consumable,
            name: "All Blessings".into(),
            description: "Receive all 5 regular blessings.".into(),
            price: 130,
            original_price: None,
            icon_id: 104,
            featured: false,
            on_sale: false,
            sale_end: None,
            rewards: vec![
                StoreReward::Blessing { blessing_id: 1 },
                StoreReward::Blessing { blessing_id: 2 },
                StoreReward::Blessing { blessing_id: 3 },
                StoreReward::Blessing { blessing_id: 4 },
                StoreReward::Blessing { blessing_id: 5 },
            ],
            requirements: vec![],
            max_purchases: 0,
            requires_premium: false,
            min_level: 0,
            available_from: None,
            available_until: None,
        });
    }

    /// Add an offer
    pub fn add_offer(&mut self, offer: StoreOffer) {
        let id = offer.id;
        let category = offer.category;
        self.offers.insert(id, offer);
        self.offers_by_category.entry(category).or_default().push(id);
    }

    /// Get offer by ID
    pub fn get_offer(&self, offer_id: u32) -> Option<&StoreOffer> {
        self.offers.get(&offer_id)
    }

    /// Get offers by category
    pub fn get_category_offers(&self, category: StoreCategory) -> Vec<&StoreOffer> {
        self.offers_by_category
            .get(&category)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.offers.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get featured offers
    pub fn get_featured_offers(&self) -> Vec<&StoreOffer> {
        self.offers.values().filter(|o| o.featured).collect()
    }

    /// Get player balance
    pub fn get_balance(&self, player_id: u32) -> CoinBalance {
        self.balances.get(&player_id).cloned().unwrap_or_default()
    }

    /// Add coins to player
    pub fn add_coins(&mut self, player_id: u32, transferable: TibiaCoins, non_transferable: TibiaCoins) {
        self.balances
            .entry(player_id)
            .or_default()
            .add(transferable, non_transferable);
    }

    /// Purchase offer
    pub fn purchase(
        &mut self,
        player_id: u32,
        offer_id: u32,
        recipient_id: Option<u32>,
    ) -> PurchaseResult {
        let now = Utc::now();

        // Get offer
        let offer = match self.offers.get(&offer_id) {
            Some(o) => o.clone(),
            None => return PurchaseResult::OfferNotFound,
        };

        // Check availability
        if !offer.is_available(now) {
            return PurchaseResult::OfferUnavailable;
        }

        // Check max purchases
        if offer.max_purchases > 0 {
            let count = self.purchase_counts
                .get(&player_id)
                .and_then(|m| m.get(&offer_id))
                .copied()
                .unwrap_or(0);
            if count >= offer.max_purchases {
                return PurchaseResult::MaxPurchasesReached;
            }
        }

        // Check balance
        let balance = self.balances.entry(player_id).or_default();
        if !balance.can_afford(offer.price) {
            return PurchaseResult::InsufficientCoins {
                needed: offer.price,
                have: balance.total(),
            };
        }

        // Process payment
        balance.spend(offer.price);

        // Record transaction
        let transaction_id = Uuid::new_v4();
        self.transactions.push(StoreTransaction {
            id: transaction_id,
            player_id,
            offer_id,
            amount: offer.price,
            timestamp: now,
            status: TransactionStatus::Completed,
            recipient_id,
        });

        // Increment purchase count
        *self.purchase_counts
            .entry(player_id)
            .or_default()
            .entry(offer_id)
            .or_insert(0) += 1;

        // Apply rewards to target
        let target_id = recipient_id.unwrap_or(player_id);
        self.apply_rewards(target_id, &offer.rewards);

        PurchaseResult::Success {
            transaction_id,
            rewards: offer.rewards.clone(),
        }
    }

    /// Apply rewards to player
    fn apply_rewards(&mut self, player_id: u32, rewards: &[StoreReward]) {
        let owned = self.player_owned.entry(player_id).or_default();

        for reward in rewards {
            match reward {
                StoreReward::PremiumDays(days) => {
                    owned.premium_days_remaining += days;
                }
                StoreReward::Outfit { look_type, addons } => {
                    let current = owned.outfits.entry(*look_type).or_insert(0);
                    *current |= addons;
                }
                StoreReward::Mount(mount_id) => {
                    if !owned.mounts.contains(mount_id) {
                        owned.mounts.push(*mount_id);
                    }
                }
                StoreReward::PreyWildcard(count) => {
                    owned.prey_wildcards += count;
                }
                StoreReward::XpBoost { percentage, hours } => {
                    owned.xp_boost_hours += hours;
                    owned.xp_boost_percentage = owned.xp_boost_percentage.max(*percentage);
                }
                StoreReward::CharmExpansion => {
                    owned.charm_expansion = true;
                }
                StoreReward::GoldPouch { capacity } => {
                    owned.gold_pouch_capacity = owned.gold_pouch_capacity.max(*capacity);
                }
                StoreReward::Hireling { name, .. } => {
                    owned.hirelings.push(name.clone());
                }
                StoreReward::HouseDecoration { item_id } => {
                    owned.house_decorations.push(*item_id);
                }
                _ => {
                    // Other rewards handled elsewhere (blessings, name change, etc.)
                }
            }
        }
    }

    /// Get player owned items
    pub fn get_player_owned(&self, player_id: u32) -> PlayerOwnedStore {
        self.player_owned.get(&player_id).cloned().unwrap_or_default()
    }

    /// Get transaction history
    pub fn get_transactions(&self, player_id: u32, limit: usize) -> Vec<&StoreTransaction> {
        self.transactions
            .iter()
            .filter(|t| t.player_id == player_id || t.recipient_id == Some(player_id))
            .rev()
            .take(limit)
            .collect()
    }
}

impl Default for StoreManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coin_balance() {
        let mut balance = CoinBalance {
            transferable: 100,
            non_transferable: 50,
            tournament: 0,
        };

        assert!(balance.can_afford(150));
        assert!(!balance.can_afford(200));

        // Spend 70 - should use 50 non-transferable + 20 transferable
        balance.spend(70);
        assert_eq!(balance.non_transferable, 0);
        assert_eq!(balance.transferable, 80);
    }

    #[test]
    fn test_store_purchase() {
        let mut store = StoreManager::new();
        
        // Add coins to player
        store.add_coins(1, 500, 0);
        
        // Purchase premium
        let result = store.purchase(1, 1, None);
        assert!(matches!(result, PurchaseResult::Success { .. }));

        // Check balance reduced
        let balance = store.get_balance(1);
        assert_eq!(balance.total(), 250);
    }

    #[test]
    fn test_insufficient_coins() {
        let mut store = StoreManager::new();
        
        // No coins
        let result = store.purchase(1, 1, None);
        assert!(matches!(result, PurchaseResult::InsufficientCoins { .. }));
    }
}
