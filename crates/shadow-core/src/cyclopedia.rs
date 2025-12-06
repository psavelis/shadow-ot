//! Cyclopedia System
//!
//! Implements the in-game knowledge database including:
//! - Character information
//! - Item wiki
//! - Monster bestiary
//! - Map exploration
//! - Achievement tracking
//! - World statistics

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Utc};

/// Cyclopedia system for tracking player knowledge and world information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cyclopedia {
    /// Player ID this cyclopedia belongs to
    pub player_id: u32,
    /// Character stats section
    pub character_stats: CharacterStats,
    /// Items discovered
    pub items: ItemCyclopedia,
    /// Monsters tracked (similar to bestiary but extended)
    pub monsters: MonsterCyclopedia,
    /// Map exploration data
    pub map_exploration: MapExploration,
    /// House information
    pub houses: HouseCyclopedia,
    /// World statistics
    pub world_stats: WorldStats,
    /// Badge collection
    pub badges: BadgeCollection,
    /// Inspected players (recent)
    pub inspected_players: Vec<InspectedPlayer>,
    /// Last update timestamp
    pub last_updated: DateTime<Utc>,
}

impl Cyclopedia {
    /// Create a new cyclopedia for a player
    pub fn new(player_id: u32) -> Self {
        Self {
            player_id,
            character_stats: CharacterStats::default(),
            items: ItemCyclopedia::default(),
            monsters: MonsterCyclopedia::default(),
            map_exploration: MapExploration::default(),
            houses: HouseCyclopedia::default(),
            world_stats: WorldStats::default(),
            badges: BadgeCollection::default(),
            inspected_players: Vec::new(),
            last_updated: Utc::now(),
        }
    }

    /// Update character stats
    pub fn update_character_stats(&mut self, stats: CharacterStats) {
        self.character_stats = stats;
        self.last_updated = Utc::now();
    }

    /// Discover an item
    pub fn discover_item(&mut self, item_id: u16) {
        self.items.discover(item_id);
        self.last_updated = Utc::now();
    }

    /// Track monster kill
    pub fn track_monster_kill(&mut self, race_id: u16) {
        self.monsters.add_kill(race_id);
        self.last_updated = Utc::now();
    }

    /// Explore a map area
    pub fn explore_area(&mut self, area_id: u32) {
        self.map_exploration.discover_area(area_id);
        self.last_updated = Utc::now();
    }

    /// Add inspected player
    pub fn add_inspected_player(&mut self, player: InspectedPlayer) {
        // Keep only last 20 inspected players
        if self.inspected_players.len() >= 20 {
            self.inspected_players.remove(0);
        }
        self.inspected_players.push(player);
        self.last_updated = Utc::now();
    }

    /// Earn a badge
    pub fn earn_badge(&mut self, badge_id: u32) {
        self.badges.earn(badge_id);
        self.last_updated = Utc::now();
    }

    /// Get exploration percentage
    pub fn exploration_percentage(&self) -> f32 {
        self.map_exploration.completion_percentage()
    }

    /// Get item discovery percentage
    pub fn item_discovery_percentage(&self, total_items: usize) -> f32 {
        if total_items == 0 {
            return 100.0;
        }
        (self.items.discovered.len() as f32 / total_items as f32) * 100.0
    }

    /// Get total play time in seconds
    pub fn total_playtime_seconds(&self) -> u64 {
        self.character_stats.total_playtime_seconds
    }
}

/// Character statistics section
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CharacterStats {
    /// Account creation date
    pub account_created: Option<DateTime<Utc>>,
    /// Character creation date
    pub character_created: Option<DateTime<Utc>>,
    /// Total online time in seconds
    pub total_playtime_seconds: u64,
    /// Total experience earned (lifetime)
    pub total_exp_earned: u64,
    /// Total gold earned (lifetime)
    pub total_gold_earned: u64,
    /// Total monsters killed
    pub total_monsters_killed: u64,
    /// Total players killed
    pub total_players_killed: u32,
    /// Total deaths
    pub total_deaths: u32,
    /// Highest level reached
    pub highest_level: u16,
    /// Highest skill level reached (per skill)
    pub highest_skills: HashMap<u8, u8>,
    /// Total damage dealt
    pub total_damage_dealt: u64,
    /// Total damage received
    pub total_damage_received: u64,
    /// Total healing done
    pub total_healing_done: u64,
    /// Total items looted
    pub total_items_looted: u64,
    /// Total quests completed
    pub total_quests_completed: u32,
    /// Total bosses killed
    pub total_bosses_killed: u32,
    /// Sessions count
    pub total_sessions: u32,
    /// Average session duration
    pub avg_session_minutes: f32,
    /// Longest session
    pub longest_session_minutes: u32,
    /// First kill (monster race ID)
    pub first_kill: Option<u16>,
    /// Last login time
    pub last_login: Option<DateTime<Utc>>,
}

impl CharacterStats {
    /// Update on monster kill
    pub fn on_monster_kill(&mut self, race_id: u16, exp_gained: u64) {
        self.total_monsters_killed += 1;
        self.total_exp_earned += exp_gained;
        if self.first_kill.is_none() {
            self.first_kill = Some(race_id);
        }
    }

    /// Update on player kill
    pub fn on_player_kill(&mut self) {
        self.total_players_killed += 1;
    }

    /// Update on death
    pub fn on_death(&mut self) {
        self.total_deaths += 1;
    }

    /// Update on loot
    pub fn on_loot(&mut self, item_count: u32, gold_value: u64) {
        self.total_items_looted += item_count as u64;
        self.total_gold_earned += gold_value;
    }

    /// Update level
    pub fn update_level(&mut self, level: u16) {
        if level > self.highest_level {
            self.highest_level = level;
        }
    }

    /// Update skill
    pub fn update_skill(&mut self, skill_id: u8, level: u8) {
        let current = self.highest_skills.get(&skill_id).copied().unwrap_or(0);
        if level > current {
            self.highest_skills.insert(skill_id, level);
        }
    }

    /// Record session
    pub fn record_session(&mut self, duration_minutes: u32) {
        self.total_sessions += 1;
        self.total_playtime_seconds += (duration_minutes as u64) * 60;
        
        if duration_minutes > self.longest_session_minutes {
            self.longest_session_minutes = duration_minutes;
        }
        
        // Recalculate average
        if self.total_sessions > 0 {
            self.avg_session_minutes = 
                (self.total_playtime_seconds as f32 / 60.0) / self.total_sessions as f32;
        }
    }
}

/// Item cyclopedia data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ItemCyclopedia {
    /// Set of discovered item IDs
    pub discovered: HashSet<u16>,
    /// Items with usage count
    pub usage_count: HashMap<u16, u32>,
    /// Favorite items (marked by player)
    pub favorites: HashSet<u16>,
    /// Recently discovered items
    pub recent_discoveries: Vec<ItemDiscovery>,
}

impl ItemCyclopedia {
    /// Discover an item
    pub fn discover(&mut self, item_id: u16) {
        if self.discovered.insert(item_id) {
            // New discovery
            let discovery = ItemDiscovery {
                item_id,
                discovered_at: Utc::now(),
            };
            
            // Keep last 50 discoveries
            if self.recent_discoveries.len() >= 50 {
                self.recent_discoveries.remove(0);
            }
            self.recent_discoveries.push(discovery);
        }
    }

    /// Record item usage
    pub fn record_usage(&mut self, item_id: u16) {
        *self.usage_count.entry(item_id).or_insert(0) += 1;
        self.discover(item_id);
    }

    /// Toggle favorite
    pub fn toggle_favorite(&mut self, item_id: u16) -> bool {
        if self.favorites.contains(&item_id) {
            self.favorites.remove(&item_id);
            false
        } else {
            self.favorites.insert(item_id);
            true
        }
    }

    /// Check if item is discovered
    pub fn is_discovered(&self, item_id: u16) -> bool {
        self.discovered.contains(&item_id)
    }

    /// Get most used items
    pub fn most_used(&self, count: usize) -> Vec<(u16, u32)> {
        let mut items: Vec<_> = self.usage_count.iter()
            .map(|(&id, &count)| (id, count))
            .collect();
        items.sort_by(|a, b| b.1.cmp(&a.1));
        items.truncate(count);
        items
    }
}

/// Item discovery record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDiscovery {
    pub item_id: u16,
    pub discovered_at: DateTime<Utc>,
}

/// Monster cyclopedia (extended bestiary)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MonsterCyclopedia {
    /// Kill counts per race
    pub kills: HashMap<u16, MonsterEntry>,
    /// Charm points earned
    pub charm_points: u32,
    /// Active charms
    pub active_charms: HashMap<u16, CharmAssignment>,
    /// Unlocked charms
    pub unlocked_charms: HashSet<u32>,
}

impl MonsterCyclopedia {
    /// Add a kill
    pub fn add_kill(&mut self, race_id: u16) {
        let entry = self.kills.entry(race_id).or_insert_with(|| MonsterEntry {
            race_id,
            kills: 0,
            first_kill: Utc::now(),
            last_kill: Utc::now(),
            charm_unlocked: false,
        });
        
        entry.kills += 1;
        entry.last_kill = Utc::now();
    }

    /// Get kills for race
    pub fn get_kills(&self, race_id: u16) -> u32 {
        self.kills.get(&race_id).map(|e| e.kills).unwrap_or(0)
    }

    /// Check if charm can be unlocked (threshold varies by difficulty)
    pub fn can_unlock_charm(&self, race_id: u16, required_kills: u32) -> bool {
        self.get_kills(race_id) >= required_kills
    }

    /// Unlock charm for a monster
    pub fn unlock_charm(&mut self, race_id: u16, charm_points: u32) -> bool {
        if let Some(entry) = self.kills.get_mut(&race_id) {
            if !entry.charm_unlocked {
                entry.charm_unlocked = true;
                self.charm_points += charm_points;
                return true;
            }
        }
        false
    }

    /// Assign charm to monster
    pub fn assign_charm(&mut self, race_id: u16, charm_id: u32) -> bool {
        if self.unlocked_charms.contains(&charm_id) {
            self.active_charms.insert(race_id, CharmAssignment {
                charm_id,
                assigned_at: Utc::now(),
            });
            true
        } else {
            false
        }
    }

    /// Get total unique monsters killed
    pub fn unique_monsters_killed(&self) -> usize {
        self.kills.len()
    }

    /// Get total kills
    pub fn total_kills(&self) -> u64 {
        self.kills.values().map(|e| e.kills as u64).sum()
    }
}

/// Monster entry in cyclopedia
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterEntry {
    pub race_id: u16,
    pub kills: u32,
    pub first_kill: DateTime<Utc>,
    pub last_kill: DateTime<Utc>,
    pub charm_unlocked: bool,
}

/// Charm assignment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharmAssignment {
    pub charm_id: u32,
    pub assigned_at: DateTime<Utc>,
}

/// Map exploration tracking
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MapExploration {
    /// Discovered areas (area ID -> discovery info)
    pub discovered_areas: HashMap<u32, AreaDiscovery>,
    /// Discovered floor tiles count per floor
    pub tiles_per_floor: HashMap<i8, u32>,
    /// Total map tiles discovered
    pub total_tiles_discovered: u32,
    /// Total map areas
    pub total_areas: u32,
}

impl MapExploration {
    /// Discover an area
    pub fn discover_area(&mut self, area_id: u32) {
        self.discovered_areas.entry(area_id).or_insert_with(|| AreaDiscovery {
            area_id,
            discovered_at: Utc::now(),
            fully_explored: false,
        });
    }

    /// Mark area as fully explored
    pub fn mark_fully_explored(&mut self, area_id: u32) {
        if let Some(area) = self.discovered_areas.get_mut(&area_id) {
            area.fully_explored = true;
        }
    }

    /// Record tiles discovered on a floor
    pub fn record_tiles(&mut self, floor: i8, count: u32) {
        *self.tiles_per_floor.entry(floor).or_insert(0) += count;
        self.total_tiles_discovered += count;
    }

    /// Get completion percentage
    pub fn completion_percentage(&self) -> f32 {
        if self.total_areas == 0 {
            return 0.0;
        }
        (self.discovered_areas.len() as f32 / self.total_areas as f32) * 100.0
    }

    /// Get fully explored areas count
    pub fn fully_explored_count(&self) -> usize {
        self.discovered_areas.values().filter(|a| a.fully_explored).count()
    }
}

/// Area discovery record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AreaDiscovery {
    pub area_id: u32,
    pub discovered_at: DateTime<Utc>,
    pub fully_explored: bool,
}

/// House cyclopedia
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HouseCyclopedia {
    /// Owned houses
    pub owned_houses: Vec<OwnedHouse>,
    /// House history
    pub house_history: Vec<HouseHistoryEntry>,
    /// Total rent paid
    pub total_rent_paid: u64,
}

impl HouseCyclopedia {
    /// Add owned house
    pub fn add_house(&mut self, house: OwnedHouse) {
        self.owned_houses.push(house);
    }

    /// Record rent payment
    pub fn record_rent(&mut self, house_id: u32, amount: u64) {
        self.total_rent_paid += amount;
        
        // Add to history
        self.house_history.push(HouseHistoryEntry {
            house_id,
            action: HouseAction::RentPaid,
            timestamp: Utc::now(),
            amount: Some(amount),
        });
    }

    /// Record house purchase
    pub fn record_purchase(&mut self, house_id: u32, price: u64) {
        self.house_history.push(HouseHistoryEntry {
            house_id,
            action: HouseAction::Purchased,
            timestamp: Utc::now(),
            amount: Some(price),
        });
    }

    /// Record house sold/lost
    pub fn record_loss(&mut self, house_id: u32) {
        self.owned_houses.retain(|h| h.house_id != house_id);
        
        self.house_history.push(HouseHistoryEntry {
            house_id,
            action: HouseAction::Lost,
            timestamp: Utc::now(),
            amount: None,
        });
    }
}

/// Owned house entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnedHouse {
    pub house_id: u32,
    pub name: String,
    pub town_id: u16,
    pub size: u32,
    pub rent: u64,
    pub purchased_at: DateTime<Utc>,
}

/// House history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HouseHistoryEntry {
    pub house_id: u32,
    pub action: HouseAction,
    pub timestamp: DateTime<Utc>,
    pub amount: Option<u64>,
}

/// House action type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HouseAction {
    Purchased,
    RentPaid,
    Lost,
    Upgraded,
}

/// World statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorldStats {
    /// Online record
    pub online_record: u32,
    /// Online record date
    pub online_record_date: Option<DateTime<Utc>>,
    /// Total registered accounts
    pub total_accounts: u32,
    /// Total characters
    pub total_characters: u32,
    /// Total guilds
    pub total_guilds: u32,
    /// Top players by level
    pub highscores: Highscores,
    /// World age in days
    pub world_age_days: u32,
    /// Total gold in economy
    pub total_gold: u64,
    /// Total items in world
    pub total_items: u64,
}

/// Highscore data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Highscores {
    /// Top by level
    pub by_level: Vec<HighscoreEntry>,
    /// Top by skill (skill_id -> entries)
    pub by_skill: HashMap<u8, Vec<HighscoreEntry>>,
    /// Top by achievements
    pub by_achievements: Vec<HighscoreEntry>,
    /// Top by charm points
    pub by_charm_points: Vec<HighscoreEntry>,
}

/// Highscore entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighscoreEntry {
    pub rank: u32,
    pub player_id: u32,
    pub player_name: String,
    pub value: u64,
    pub vocation: u8,
}

/// Badge collection
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BadgeCollection {
    /// Earned badge IDs with earn date
    pub earned: HashMap<u32, DateTime<Utc>>,
    /// Selected display badges
    pub display_badges: Vec<u32>,
}

impl BadgeCollection {
    /// Earn a badge
    pub fn earn(&mut self, badge_id: u32) {
        self.earned.entry(badge_id).or_insert_with(Utc::now);
    }

    /// Check if badge is earned
    pub fn has_badge(&self, badge_id: u32) -> bool {
        self.earned.contains_key(&badge_id)
    }

    /// Set display badges (max 5)
    pub fn set_display(&mut self, badges: Vec<u32>) {
        self.display_badges = badges.into_iter()
            .filter(|b| self.earned.contains_key(b))
            .take(5)
            .collect();
    }

    /// Get total badges earned
    pub fn total(&self) -> usize {
        self.earned.len()
    }
}

/// Inspected player record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectedPlayer {
    pub player_id: u32,
    pub name: String,
    pub level: u16,
    pub vocation: u8,
    pub inspected_at: DateTime<Utc>,
}

/// Cyclopedia category for filtering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CyclopediaCategory {
    CharacterInfo,
    Items,
    Monsters,
    Map,
    Houses,
    WorldStats,
    Badges,
    Highscores,
}

/// Cyclopedia manager for server-wide data
#[derive(Debug, Default)]
pub struct CyclopediaManager {
    /// Player cyclopedias
    cyclopedias: HashMap<u32, Cyclopedia>,
    /// World statistics (shared)
    world_stats: WorldStats,
    /// Badge definitions
    badge_definitions: HashMap<u32, BadgeDefinition>,
    /// Total discoverable items
    total_items: usize,
    /// Total map areas
    total_areas: u32,
}

impl CyclopediaManager {
    /// Create new manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Get or create player cyclopedia
    pub fn get_or_create(&mut self, player_id: u32) -> &mut Cyclopedia {
        self.cyclopedias.entry(player_id).or_insert_with(|| {
            let mut cyclo = Cyclopedia::new(player_id);
            cyclo.world_stats = self.world_stats.clone();
            cyclo
        })
    }

    /// Get player cyclopedia (read-only)
    pub fn get(&self, player_id: u32) -> Option<&Cyclopedia> {
        self.cyclopedias.get(&player_id)
    }

    /// Update world stats
    pub fn update_world_stats(&mut self, stats: WorldStats) {
        self.world_stats = stats;
    }

    /// Register badge definition
    pub fn register_badge(&mut self, badge: BadgeDefinition) {
        self.badge_definitions.insert(badge.id, badge);
    }

    /// Get badge definition
    pub fn get_badge(&self, badge_id: u32) -> Option<&BadgeDefinition> {
        self.badge_definitions.get(&badge_id)
    }

    /// Set total discoverable items
    pub fn set_total_items(&mut self, count: usize) {
        self.total_items = count;
    }

    /// Set total map areas
    pub fn set_total_areas(&mut self, count: u32) {
        self.total_areas = count;
    }

    /// Get global highscores
    pub fn get_highscores(&self) -> &Highscores {
        &self.world_stats.highscores
    }

    /// Record player discovery
    pub fn record_item_discovery(&mut self, player_id: u32, item_id: u16) {
        if let Some(cyclo) = self.cyclopedias.get_mut(&player_id) {
            cyclo.discover_item(item_id);
        }
    }

    /// Record monster kill
    pub fn record_monster_kill(&mut self, player_id: u32, race_id: u16, exp: u64) {
        if let Some(cyclo) = self.cyclopedias.get_mut(&player_id) {
            cyclo.track_monster_kill(race_id);
            cyclo.character_stats.on_monster_kill(race_id, exp);
        }
    }

    /// Remove player cyclopedia (on logout, optionally save first)
    pub fn remove(&mut self, player_id: u32) -> Option<Cyclopedia> {
        self.cyclopedias.remove(&player_id)
    }
}

/// Badge definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadgeDefinition {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub icon_id: u16,
    pub rarity: BadgeRarity,
}

/// Badge rarity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BadgeRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cyclopedia_creation() {
        let cyclo = Cyclopedia::new(1);
        assert_eq!(cyclo.player_id, 1);
        assert!(cyclo.items.discovered.is_empty());
    }

    #[test]
    fn test_item_discovery() {
        let mut cyclo = Cyclopedia::new(1);
        cyclo.discover_item(100);
        cyclo.discover_item(200);
        cyclo.discover_item(100); // duplicate
        
        assert_eq!(cyclo.items.discovered.len(), 2);
        assert!(cyclo.items.is_discovered(100));
    }

    #[test]
    fn test_monster_tracking() {
        let mut cyclo = Cyclopedia::new(1);
        cyclo.track_monster_kill(50);
        cyclo.track_monster_kill(50);
        cyclo.track_monster_kill(75);
        
        assert_eq!(cyclo.monsters.get_kills(50), 2);
        assert_eq!(cyclo.monsters.get_kills(75), 1);
        assert_eq!(cyclo.monsters.unique_monsters_killed(), 2);
    }

    #[test]
    fn test_badge_collection() {
        let mut badges = BadgeCollection::default();
        badges.earn(1);
        badges.earn(2);
        badges.earn(1); // duplicate
        
        assert_eq!(badges.total(), 2);
        assert!(badges.has_badge(1));
        assert!(!badges.has_badge(3));
    }

    #[test]
    fn test_character_stats() {
        let mut stats = CharacterStats::default();
        stats.on_monster_kill(50, 100);
        stats.on_monster_kill(75, 200);
        stats.on_loot(5, 500);
        
        assert_eq!(stats.total_monsters_killed, 2);
        assert_eq!(stats.total_exp_earned, 300);
        assert_eq!(stats.total_items_looted, 5);
        assert_eq!(stats.total_gold_earned, 500);
    }

    #[test]
    fn test_cyclopedia_manager() {
        let mut manager = CyclopediaManager::new();
        
        let cyclo = manager.get_or_create(1);
        cyclo.discover_item(100);
        
        assert!(manager.get(1).is_some());
        assert_eq!(manager.get(1).unwrap().items.discovered.len(), 1);
    }
}
