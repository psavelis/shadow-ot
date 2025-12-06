//! Shop System
//!
//! Handles NPC shops, buying, selling, and trade offers.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A shop item available for purchase or sale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopItem {
    /// Item type ID
    pub item_id: u16,
    /// Item name (for display)
    pub name: String,
    /// Subtype (for stackables, fluids)
    pub subtype: u8,
    /// Buy price (player pays, 0 = not for sale)
    pub buy_price: u32,
    /// Sell price (player receives, 0 = won't buy)
    pub sell_price: u32,
    /// Stock limit (0 = unlimited)
    pub stock: u32,
}

impl ShopItem {
    /// Create a new shop item
    pub fn new(item_id: u16, name: impl Into<String>) -> Self {
        Self {
            item_id,
            name: name.into(),
            subtype: 0,
            buy_price: 0,
            sell_price: 0,
            stock: 0,
        }
    }

    /// Set buy price
    pub fn buy(mut self, price: u32) -> Self {
        self.buy_price = price;
        self
    }

    /// Set sell price
    pub fn sell(mut self, price: u32) -> Self {
        self.sell_price = price;
        self
    }

    /// Set subtype
    pub fn with_subtype(mut self, subtype: u8) -> Self {
        self.subtype = subtype;
        self
    }

    /// Set stock
    pub fn with_stock(mut self, stock: u32) -> Self {
        self.stock = stock;
        self
    }

    /// Whether player can buy this item
    pub fn can_buy(&self) -> bool {
        self.buy_price > 0 && (self.stock == 0 || self.stock > 0)
    }

    /// Whether player can sell this item
    pub fn can_sell(&self) -> bool {
        self.sell_price > 0
    }
}

/// Currency types for shops
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Currency {
    Gold,
    Platinum,
    Crystal,
    BankBalance,
    Custom(u16), // Item ID as currency
}

impl Default for Currency {
    fn default() -> Self {
        Self::Gold
    }
}

/// A complete shop definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shop {
    /// Shop identifier
    pub id: String,
    /// Shop name
    pub name: String,
    /// NPC name that owns this shop
    pub npc_name: String,
    /// Items available
    pub items: Vec<ShopItem>,
    /// Currency used
    pub currency: Currency,
    /// Discount percentage (0-100)
    pub discount: u8,
    /// Premium discount (additional)
    pub premium_discount: u8,
}

impl Shop {
    /// Create a new shop
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            npc_name: String::new(),
            items: Vec::new(),
            currency: Currency::Gold,
            discount: 0,
            premium_discount: 0,
        }
    }

    /// Add an item to the shop
    pub fn add_item(mut self, item: ShopItem) -> Self {
        self.items.push(item);
        self
    }

    /// Set NPC name
    pub fn for_npc(mut self, npc: impl Into<String>) -> Self {
        self.npc_name = npc.into();
        self
    }

    /// Set discount
    pub fn with_discount(mut self, discount: u8) -> Self {
        self.discount = discount.min(100);
        self
    }

    /// Get item by ID
    pub fn get_item(&self, item_id: u16) -> Option<&ShopItem> {
        self.items.iter().find(|i| i.item_id == item_id)
    }

    /// Get items player can buy
    pub fn buyable_items(&self) -> impl Iterator<Item = &ShopItem> {
        self.items.iter().filter(|i| i.can_buy())
    }

    /// Get items player can sell
    pub fn sellable_items(&self) -> impl Iterator<Item = &ShopItem> {
        self.items.iter().filter(|i| i.can_sell())
    }

    /// Calculate final buy price with discounts
    pub fn final_buy_price(&self, item_id: u16, is_premium: bool) -> Option<u32> {
        self.get_item(item_id).map(|item| {
            let mut price = item.buy_price;
            if self.discount > 0 {
                price = price.saturating_sub(price * self.discount as u32 / 100);
            }
            if is_premium && self.premium_discount > 0 {
                price = price.saturating_sub(price * self.premium_discount as u32 / 100);
            }
            price
        })
    }

    /// Calculate final sell price
    pub fn final_sell_price(&self, item_id: u16, is_premium: bool) -> Option<u32> {
        self.get_item(item_id).map(|item| {
            let mut price = item.sell_price;
            // Premium might get better sell prices
            if is_premium {
                price = price + price / 10; // 10% bonus
            }
            price
        })
    }
}

/// Handler for shop transactions
pub struct ShopHandler {
    shops: HashMap<String, Shop>,
}

impl ShopHandler {
    /// Create a new shop handler
    pub fn new() -> Self {
        Self {
            shops: HashMap::new(),
        }
    }

    /// Register a shop
    pub fn register(&mut self, shop: Shop) {
        self.shops.insert(shop.id.clone(), shop);
    }

    /// Get a shop by ID
    pub fn get(&self, id: &str) -> Option<&Shop> {
        self.shops.get(id)
    }

    /// Get mutable shop
    pub fn get_mut(&mut self, id: &str) -> Option<&mut Shop> {
        self.shops.get_mut(id)
    }

    /// Load shops from JSON
    pub fn load_from_json(&mut self, json: &str) -> Result<usize, serde_json::Error> {
        let shops: Vec<Shop> = serde_json::from_str(json)?;
        let count = shops.len();
        for shop in shops {
            self.register(shop);
        }
        Ok(count)
    }

    /// Create pre-built shops for common NPCs
    pub fn create_default_shops(&mut self) {
        // General merchant
        self.register(
            Shop::new("general_merchant", "General Store")
                .for_npc("Merchant")
                .add_item(ShopItem::new(2160, "Crystal Coin").buy(10000).sell(10000))
                .add_item(ShopItem::new(2152, "Platinum Coin").buy(100).sell(100))
                .add_item(ShopItem::new(2148, "Gold Coin").buy(0).sell(1))
                .add_item(ShopItem::new(2120, "Rope").buy(50).sell(15))
                .add_item(ShopItem::new(2554, "Shovel").buy(50).sell(8))
                .add_item(ShopItem::new(2580, "Torch").buy(2))
                .add_item(ShopItem::new(2006, "Vial").buy(5).sell(5))
        );

        // Equipment shop
        self.register(
            Shop::new("armor_shop", "Armor Shop")
                .for_npc("Armorsmith")
                .add_item(ShopItem::new(2463, "Plate Armor").buy(400).sell(100))
                .add_item(ShopItem::new(2457, "Steel Helmet").buy(290).sell(72))
                .add_item(ShopItem::new(2647, "Plate Legs").buy(115).sell(28))
                .add_item(ShopItem::new(2643, "Leather Boots").buy(10).sell(2))
                .add_item(ShopItem::new(2525, "Dwarven Shield").buy(100).sell(25))
        );

        // Weapon shop
        self.register(
            Shop::new("weapon_shop", "Weapon Shop")
                .for_npc("Weaponsmith")
                .add_item(ShopItem::new(2383, "Spike Sword").buy(240).sell(60))
                .add_item(ShopItem::new(2377, "Two Handed Sword").buy(450).sell(112))
                .add_item(ShopItem::new(2428, "Crossbow").buy(160).sell(40))
                .add_item(ShopItem::new(2544, "Bolt").buy(3).sell(1).with_subtype(100))
                .add_item(ShopItem::new(2456, "Bow").buy(130).sell(32))
                .add_item(ShopItem::new(2544, "Arrow").buy(2).sell(1).with_subtype(100))
        );

        // Magic shop
        self.register(
            Shop::new("magic_shop", "Magic Shop")
                .for_npc("Mage")
                .add_item(ShopItem::new(2190, "Wand of Dragonbreath").buy(1000).sell(250))
                .add_item(ShopItem::new(2182, "Snakebite Rod").buy(500).sell(125))
                .add_item(ShopItem::new(2260, "Blank Rune").buy(10).sell(5))
                .add_item(ShopItem::new(2268, "Sudden Death Rune").buy(135).sell(50))
                .add_item(ShopItem::new(2273, "Ultimate Healing Rune").buy(175).sell(50))
        );

        // Food shop
        self.register(
            Shop::new("food_shop", "Food Shop")
                .for_npc("Cook")
                .add_item(ShopItem::new(2666, "Meat").buy(5).sell(2))
                .add_item(ShopItem::new(2667, "Fish").buy(4).sell(1))
                .add_item(ShopItem::new(2681, "Ham").buy(8).sell(3))
                .add_item(ShopItem::new(2689, "Bread").buy(4).sell(1))
                .add_item(ShopItem::new(2695, "Apple").buy(3).sell(1))
                .add_item(ShopItem::new(2006, "Vial of Water").buy(5).with_subtype(1))
        );

        // Potion shop
        self.register(
            Shop::new("potion_shop", "Potion Shop")
                .for_npc("Alchemist")
                .add_item(ShopItem::new(7618, "Health Potion").buy(45).sell(15))
                .add_item(ShopItem::new(7620, "Mana Potion").buy(50).sell(18))
                .add_item(ShopItem::new(7588, "Strong Health Potion").buy(100).sell(35))
                .add_item(ShopItem::new(7589, "Strong Mana Potion").buy(80).sell(28))
                .add_item(ShopItem::new(7591, "Great Health Potion").buy(190).sell(70))
                .add_item(ShopItem::new(7590, "Great Mana Potion").buy(120).sell(45))
                .add_item(ShopItem::new(8472, "Ultimate Health Potion").buy(310).sell(115))
                .add_item(ShopItem::new(8473, "Ultimate Mana Potion").buy(350).sell(130))
        );
    }
}

impl Default for ShopHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Transaction result
#[derive(Debug, Clone)]
pub enum TransactionResult {
    Success {
        item_id: u16,
        count: u16,
        total_price: u32,
    },
    InsufficientFunds {
        required: u32,
        available: u32,
    },
    InsufficientItems {
        required: u16,
        available: u16,
    },
    InsufficientCapacity {
        required: u32,
        available: u32,
    },
    ItemNotFound,
    ShopNotFound,
    NotForSale,
    NotBuying,
    OutOfStock,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shop_item() {
        let item = ShopItem::new(2160, "Crystal Coin")
            .buy(10000)
            .sell(10000);

        assert!(item.can_buy());
        assert!(item.can_sell());
    }

    #[test]
    fn test_shop_creation() {
        let shop = Shop::new("test", "Test Shop")
            .for_npc("Test NPC")
            .add_item(ShopItem::new(100, "Test Item").buy(100).sell(50));

        assert_eq!(shop.items.len(), 1);
        assert!(shop.get_item(100).is_some());
    }

    #[test]
    fn test_discount() {
        let shop = Shop::new("test", "Test")
            .with_discount(10)
            .add_item(ShopItem::new(100, "Test").buy(100));

        let price = shop.final_buy_price(100, false);
        assert_eq!(price, Some(90));
    }

    #[test]
    fn test_shop_handler() {
        let mut handler = ShopHandler::new();
        handler.create_default_shops();

        assert!(handler.get("general_merchant").is_some());
        assert!(handler.get("potion_shop").is_some());
    }
}
