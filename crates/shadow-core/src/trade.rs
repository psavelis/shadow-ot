//! Trade System
//!
//! Handles player-to-player trading and market operations.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};

/// Trade item entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeItem {
    /// Item unique ID
    pub unique_id: u32,
    /// Item type ID
    pub item_type_id: u16,
    /// Stack count
    pub count: u16,
    /// Container slot (if applicable)
    pub container_slot: Option<u8>,
}

impl TradeItem {
    pub fn new(unique_id: u32, item_type_id: u16, count: u16) -> Self {
        Self {
            unique_id,
            item_type_id,
            count,
            container_slot: None,
        }
    }
}

/// Trade offer state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeState {
    /// Trade request sent
    Pending,
    /// Both parties adding items
    Active,
    /// Both parties accepted
    Accepted,
    /// Trade completed
    Completed,
    /// Trade cancelled
    Cancelled,
    /// Trade expired
    Expired,
}

/// A trade session between two players
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    /// Trade ID
    pub id: Uuid,
    /// First player (initiator)
    pub player1_id: Uuid,
    /// Second player
    pub player2_id: Uuid,
    /// Player 1's offered items
    pub player1_items: Vec<TradeItem>,
    /// Player 2's offered items
    pub player2_items: Vec<TradeItem>,
    /// Player 1's offered gold
    pub player1_gold: u32,
    /// Player 2's offered gold
    pub player2_gold: u32,
    /// Player 1 accepted
    pub player1_accepted: bool,
    /// Player 2 accepted
    pub player2_accepted: bool,
    /// Trade state
    pub state: TradeState,
    /// When trade was initiated
    pub created_at: DateTime<Utc>,
    /// Trade timeout
    pub expires_at: DateTime<Utc>,
}

impl Trade {
    /// Create a new trade session
    pub fn new(player1_id: Uuid, player2_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            player1_id,
            player2_id,
            player1_items: Vec::new(),
            player2_items: Vec::new(),
            player1_gold: 0,
            player2_gold: 0,
            player1_accepted: false,
            player2_accepted: false,
            state: TradeState::Pending,
            created_at: now,
            expires_at: now + Duration::minutes(5),
        }
    }

    /// Check if player is participant
    pub fn is_participant(&self, player_id: Uuid) -> bool {
        self.player1_id == player_id || self.player2_id == player_id
    }

    /// Get the other player
    pub fn other_player(&self, player_id: Uuid) -> Option<Uuid> {
        if self.player1_id == player_id {
            Some(self.player2_id)
        } else if self.player2_id == player_id {
            Some(self.player1_id)
        } else {
            None
        }
    }

    /// Add item to trade
    pub fn add_item(&mut self, player_id: Uuid, item: TradeItem) -> Result<(), TradeError> {
        if self.state != TradeState::Active {
            return Err(TradeError::InvalidState);
        }

        // Reset acceptance when items change
        self.player1_accepted = false;
        self.player2_accepted = false;

        if self.player1_id == player_id {
            self.player1_items.push(item);
        } else if self.player2_id == player_id {
            self.player2_items.push(item);
        } else {
            return Err(TradeError::NotParticipant);
        }

        Ok(())
    }

    /// Remove item from trade
    pub fn remove_item(&mut self, player_id: Uuid, item_unique_id: u32) -> Result<(), TradeError> {
        if self.state != TradeState::Active {
            return Err(TradeError::InvalidState);
        }

        // Reset acceptance when items change
        self.player1_accepted = false;
        self.player2_accepted = false;

        if self.player1_id == player_id {
            self.player1_items.retain(|i| i.unique_id != item_unique_id);
        } else if self.player2_id == player_id {
            self.player2_items.retain(|i| i.unique_id != item_unique_id);
        } else {
            return Err(TradeError::NotParticipant);
        }

        Ok(())
    }

    /// Set gold offer
    pub fn set_gold(&mut self, player_id: Uuid, amount: u32) -> Result<(), TradeError> {
        if self.state != TradeState::Active {
            return Err(TradeError::InvalidState);
        }

        // Reset acceptance when gold changes
        self.player1_accepted = false;
        self.player2_accepted = false;

        if self.player1_id == player_id {
            self.player1_gold = amount;
        } else if self.player2_id == player_id {
            self.player2_gold = amount;
        } else {
            return Err(TradeError::NotParticipant);
        }

        Ok(())
    }

    /// Accept trade offer
    pub fn accept(&mut self, player_id: Uuid) -> Result<bool, TradeError> {
        if self.state == TradeState::Pending {
            // Accept trade request
            self.state = TradeState::Active;
            return Ok(false);
        }

        if self.state != TradeState::Active {
            return Err(TradeError::InvalidState);
        }

        if self.player1_id == player_id {
            self.player1_accepted = true;
        } else if self.player2_id == player_id {
            self.player2_accepted = true;
        } else {
            return Err(TradeError::NotParticipant);
        }

        // Check if both accepted
        if self.player1_accepted && self.player2_accepted {
            self.state = TradeState::Accepted;
            return Ok(true);
        }

        Ok(false)
    }

    /// Cancel trade
    pub fn cancel(&mut self) {
        self.state = TradeState::Cancelled;
    }

    /// Complete trade
    pub fn complete(&mut self) {
        self.state = TradeState::Completed;
    }

    /// Check if trade expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}

/// Trade manager
pub struct TradeManager {
    /// Active trades
    trades: HashMap<Uuid, Arc<RwLock<Trade>>>,
    /// Player -> Trade mapping
    player_trades: HashMap<Uuid, Uuid>,
}

impl TradeManager {
    pub fn new() -> Self {
        Self {
            trades: HashMap::new(),
            player_trades: HashMap::new(),
        }
    }

    /// Request a trade with another player
    pub fn request_trade(&mut self, requester_id: Uuid, target_id: Uuid) -> Result<Uuid, TradeError> {
        // Check if either player is already in a trade
        if self.player_trades.contains_key(&requester_id) {
            return Err(TradeError::AlreadyTrading);
        }
        if self.player_trades.contains_key(&target_id) {
            return Err(TradeError::TargetBusy);
        }

        let trade = Trade::new(requester_id, target_id);
        let id = trade.id;

        self.player_trades.insert(requester_id, id);
        self.player_trades.insert(target_id, id);
        self.trades.insert(id, Arc::new(RwLock::new(trade)));

        Ok(id)
    }

    /// Get trade by ID
    pub fn get(&self, id: Uuid) -> Option<Arc<RwLock<Trade>>> {
        self.trades.get(&id).cloned()
    }

    /// Get trade by player ID
    pub fn get_by_player(&self, player_id: Uuid) -> Option<Arc<RwLock<Trade>>> {
        self.player_trades.get(&player_id)
            .and_then(|id| self.trades.get(id).cloned())
    }

    /// Check if player is trading
    pub fn is_trading(&self, player_id: Uuid) -> bool {
        self.player_trades.contains_key(&player_id)
    }

    /// End a trade (complete or cancel)
    pub async fn end_trade(&mut self, trade_id: Uuid) {
        if let Some(trade) = self.trades.remove(&trade_id) {
            let t = trade.read().await;
            self.player_trades.remove(&t.player1_id);
            self.player_trades.remove(&t.player2_id);
        }
    }

    /// Cleanup expired trades
    pub async fn cleanup_expired(&mut self) {
        let expired: Vec<Uuid> = {
            let mut expired = Vec::new();
            for (id, trade) in &self.trades {
                let t = trade.read().await;
                if t.is_expired() {
                    expired.push(*id);
                }
            }
            expired
        };

        for id in expired {
            self.end_trade(id).await;
        }
    }
}

impl Default for TradeManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============== Market System ==============

/// Market offer type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarketOfferType {
    Buy,
    Sell,
}

/// Market offer state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarketOfferState {
    Active,
    Completed,
    Cancelled,
    Expired,
}

/// A market offer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketOffer {
    /// Offer ID
    pub id: Uuid,
    /// Player who created offer
    pub player_id: Uuid,
    /// Player name
    pub player_name: String,
    /// Offer type (buy/sell)
    pub offer_type: MarketOfferType,
    /// Item type ID
    pub item_type_id: u16,
    /// Amount
    pub amount: u32,
    /// Price per item
    pub price: u32,
    /// Remaining amount
    pub remaining: u32,
    /// State
    pub state: MarketOfferState,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Expires timestamp
    pub expires_at: DateTime<Utc>,
}

impl MarketOffer {
    /// Create a new buy offer
    pub fn buy(player_id: Uuid, player_name: impl Into<String>, item_type_id: u16, amount: u32, price: u32) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            player_id,
            player_name: player_name.into(),
            offer_type: MarketOfferType::Buy,
            item_type_id,
            amount,
            price,
            remaining: amount,
            state: MarketOfferState::Active,
            created_at: now,
            expires_at: now + Duration::days(30),
        }
    }

    /// Create a new sell offer
    pub fn sell(player_id: Uuid, player_name: impl Into<String>, item_type_id: u16, amount: u32, price: u32) -> Self {
        let mut offer = Self::buy(player_id, player_name, item_type_id, amount, price);
        offer.offer_type = MarketOfferType::Sell;
        offer
    }

    /// Check if offer is active
    pub fn is_active(&self) -> bool {
        self.state == MarketOfferState::Active && !self.is_expired()
    }

    /// Check if expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    /// Total value
    pub fn total_value(&self) -> u64 {
        self.amount as u64 * self.price as u64
    }

    /// Remaining value
    pub fn remaining_value(&self) -> u64 {
        self.remaining as u64 * self.price as u64
    }
}

/// Market history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketHistory {
    pub item_type_id: u16,
    pub amount: u32,
    pub price: u32,
    pub buyer_id: Uuid,
    pub seller_id: Uuid,
    pub timestamp: DateTime<Utc>,
}

/// Market manager
pub struct MarketManager {
    /// All offers indexed by ID
    offers: HashMap<Uuid, MarketOffer>,
    /// Offers indexed by item type
    by_item: HashMap<u16, Vec<Uuid>>,
    /// Offers indexed by player
    by_player: HashMap<Uuid, Vec<Uuid>>,
    /// Transaction history
    history: Vec<MarketHistory>,
    /// Market fee percentage (0-100)
    fee_percent: u8,
}

impl MarketManager {
    pub fn new() -> Self {
        Self {
            offers: HashMap::new(),
            by_item: HashMap::new(),
            by_player: HashMap::new(),
            history: Vec::new(),
            fee_percent: 2, // 2% default fee
        }
    }

    /// Create a new offer
    pub fn create_offer(&mut self, offer: MarketOffer) -> Uuid {
        let id = offer.id;
        let item_id = offer.item_type_id;
        let player_id = offer.player_id;

        self.by_item.entry(item_id).or_default().push(id);
        self.by_player.entry(player_id).or_default().push(id);
        self.offers.insert(id, offer);

        id
    }

    /// Get offer by ID
    pub fn get_offer(&self, id: Uuid) -> Option<&MarketOffer> {
        self.offers.get(&id)
    }

    /// Get mutable offer
    pub fn get_offer_mut(&mut self, id: Uuid) -> Option<&mut MarketOffer> {
        self.offers.get_mut(&id)
    }

    /// Get offers for item type
    pub fn get_offers_for_item(&self, item_type_id: u16) -> Vec<&MarketOffer> {
        self.by_item.get(&item_type_id)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.offers.get(id))
                    .filter(|o| o.is_active())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get buy offers for item (sorted by price descending)
    pub fn get_buy_offers(&self, item_type_id: u16) -> Vec<&MarketOffer> {
        let mut offers: Vec<_> = self.get_offers_for_item(item_type_id)
            .into_iter()
            .filter(|o| o.offer_type == MarketOfferType::Buy)
            .collect();
        offers.sort_by(|a, b| b.price.cmp(&a.price)); // Best price first
        offers
    }

    /// Get sell offers for item (sorted by price ascending)
    pub fn get_sell_offers(&self, item_type_id: u16) -> Vec<&MarketOffer> {
        let mut offers: Vec<_> = self.get_offers_for_item(item_type_id)
            .into_iter()
            .filter(|o| o.offer_type == MarketOfferType::Sell)
            .collect();
        offers.sort_by(|a, b| a.price.cmp(&b.price)); // Best price first
        offers
    }

    /// Get player's offers
    pub fn get_player_offers(&self, player_id: Uuid) -> Vec<&MarketOffer> {
        self.by_player.get(&player_id)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.offers.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Cancel an offer
    pub fn cancel_offer(&mut self, offer_id: Uuid, player_id: Uuid) -> Result<(), TradeError> {
        let offer = self.offers.get_mut(&offer_id).ok_or(TradeError::OfferNotFound)?;
        
        if offer.player_id != player_id {
            return Err(TradeError::NotOwner);
        }

        offer.state = MarketOfferState::Cancelled;
        Ok(())
    }

    /// Execute a trade between offers
    pub fn execute_trade(&mut self, buy_offer_id: Uuid, sell_offer_id: Uuid, amount: u32) -> Result<MarketHistory, TradeError> {
        let buy_offer = self.offers.get(&buy_offer_id).ok_or(TradeError::OfferNotFound)?.clone();
        let sell_offer = self.offers.get(&sell_offer_id).ok_or(TradeError::OfferNotFound)?.clone();

        if buy_offer.item_type_id != sell_offer.item_type_id {
            return Err(TradeError::ItemMismatch);
        }

        if buy_offer.price < sell_offer.price {
            return Err(TradeError::PriceMismatch);
        }

        let trade_amount = amount.min(buy_offer.remaining).min(sell_offer.remaining);
        if trade_amount == 0 {
            return Err(TradeError::InsufficientAmount);
        }

        // Update offers
        {
            let buy = self.offers.get_mut(&buy_offer_id).unwrap();
            buy.remaining -= trade_amount;
            if buy.remaining == 0 {
                buy.state = MarketOfferState::Completed;
            }
        }
        {
            let sell = self.offers.get_mut(&sell_offer_id).unwrap();
            sell.remaining -= trade_amount;
            if sell.remaining == 0 {
                sell.state = MarketOfferState::Completed;
            }
        }

        // Create history entry
        let history = MarketHistory {
            item_type_id: buy_offer.item_type_id,
            amount: trade_amount,
            price: sell_offer.price, // Trade at sell price
            buyer_id: buy_offer.player_id,
            seller_id: sell_offer.player_id,
            timestamp: Utc::now(),
        };

        self.history.push(history.clone());
        Ok(history)
    }

    /// Get market statistics for an item
    pub fn get_statistics(&self, item_type_id: u16) -> MarketStatistics {
        let item_history: Vec<_> = self.history.iter()
            .filter(|h| h.item_type_id == item_type_id)
            .collect();

        let total_volume: u64 = item_history.iter().map(|h| h.amount as u64).sum();
        let avg_price = if !item_history.is_empty() {
            item_history.iter().map(|h| h.price as u64).sum::<u64>() / item_history.len() as u64
        } else {
            0
        };

        let buy_offers = self.get_buy_offers(item_type_id);
        let sell_offers = self.get_sell_offers(item_type_id);

        MarketStatistics {
            item_type_id,
            total_volume: total_volume as u32,
            average_price: avg_price as u32,
            highest_buy: buy_offers.first().map(|o| o.price).unwrap_or(0),
            lowest_sell: sell_offers.first().map(|o| o.price).unwrap_or(0),
            active_buy_offers: buy_offers.len(),
            active_sell_offers: sell_offers.len(),
        }
    }

    /// Calculate fee for amount
    pub fn calculate_fee(&self, amount: u64) -> u64 {
        amount * self.fee_percent as u64 / 100
    }

    /// Cleanup expired offers
    pub fn cleanup_expired(&mut self) {
        for offer in self.offers.values_mut() {
            if offer.is_expired() && offer.state == MarketOfferState::Active {
                offer.state = MarketOfferState::Expired;
            }
        }
    }
}

impl Default for MarketManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Market statistics
#[derive(Debug, Clone)]
pub struct MarketStatistics {
    pub item_type_id: u16,
    pub total_volume: u32,
    pub average_price: u32,
    pub highest_buy: u32,
    pub lowest_sell: u32,
    pub active_buy_offers: usize,
    pub active_sell_offers: usize,
}

/// Trade errors
#[derive(Debug, Clone)]
pub enum TradeError {
    InvalidState,
    NotParticipant,
    AlreadyTrading,
    TargetBusy,
    OfferNotFound,
    NotOwner,
    ItemMismatch,
    PriceMismatch,
    InsufficientAmount,
    InsufficientFunds,
    InventoryFull,
}

impl std::fmt::Display for TradeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TradeError::InvalidState => write!(f, "Trade is not in valid state"),
            TradeError::NotParticipant => write!(f, "You are not a participant in this trade"),
            TradeError::AlreadyTrading => write!(f, "You are already in a trade"),
            TradeError::TargetBusy => write!(f, "Target player is already trading"),
            TradeError::OfferNotFound => write!(f, "Offer not found"),
            TradeError::NotOwner => write!(f, "You do not own this offer"),
            TradeError::ItemMismatch => write!(f, "Items do not match"),
            TradeError::PriceMismatch => write!(f, "Price mismatch"),
            TradeError::InsufficientAmount => write!(f, "Insufficient amount"),
            TradeError::InsufficientFunds => write!(f, "Insufficient funds"),
            TradeError::InventoryFull => write!(f, "Inventory is full"),
        }
    }
}

impl std::error::Error for TradeError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trade_creation() {
        let p1 = Uuid::new_v4();
        let p2 = Uuid::new_v4();
        let trade = Trade::new(p1, p2);

        assert!(trade.is_participant(p1));
        assert!(trade.is_participant(p2));
        assert_eq!(trade.other_player(p1), Some(p2));
    }

    #[test]
    fn test_trade_items() {
        let p1 = Uuid::new_v4();
        let p2 = Uuid::new_v4();
        let mut trade = Trade::new(p1, p2);
        trade.state = TradeState::Active;

        trade.add_item(p1, TradeItem::new(1, 100, 1)).unwrap();
        assert_eq!(trade.player1_items.len(), 1);

        trade.remove_item(p1, 1).unwrap();
        assert!(trade.player1_items.is_empty());
    }

    #[test]
    fn test_trade_acceptance() {
        let p1 = Uuid::new_v4();
        let p2 = Uuid::new_v4();
        let mut trade = Trade::new(p1, p2);
        
        // Accept request
        trade.accept(p2).unwrap();
        assert_eq!(trade.state, TradeState::Active);

        // Accept trade
        trade.accept(p1).unwrap();
        assert!(!trade.player2_accepted); // Still false
        trade.accept(p2).unwrap();
        
        assert_eq!(trade.state, TradeState::Accepted);
    }

    #[test]
    fn test_market_offers() {
        let mut market = MarketManager::new();
        let player = Uuid::new_v4();

        let sell = MarketOffer::sell(player, "Seller", 2160, 100, 10000);
        market.create_offer(sell);

        let offers = market.get_sell_offers(2160);
        assert_eq!(offers.len(), 1);
        assert_eq!(offers[0].price, 10000);
    }

    #[test]
    fn test_market_trade() {
        let mut market = MarketManager::new();
        let buyer = Uuid::new_v4();
        let seller = Uuid::new_v4();

        let buy_offer = MarketOffer::buy(buyer, "Buyer", 2160, 10, 10000);
        let sell_offer = MarketOffer::sell(seller, "Seller", 2160, 10, 9500);

        let buy_id = market.create_offer(buy_offer);
        let sell_id = market.create_offer(sell_offer);

        let history = market.execute_trade(buy_id, sell_id, 5).unwrap();
        assert_eq!(history.amount, 5);
        assert_eq!(history.price, 9500);

        let buy = market.get_offer(buy_id).unwrap();
        assert_eq!(buy.remaining, 5);
    }
}
