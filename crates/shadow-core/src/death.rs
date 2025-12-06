//! Death and Blessing System
//!
//! Handles player death mechanics, experience/skill loss,
//! blessings, and resurrection. Similar to Tibia's death system
//! with enhancements.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// Types of blessings available
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BlessingType {
    /// Wisdom of Solitude - reduces experience loss
    WisdomOfSolitude,
    /// Spark of the Phoenix - reduces skill loss
    SparkOfThePhoenix,
    /// Fire of the Suns - prevents item loss
    FireOfTheSuns,
    /// Spiritual Shielding - protects equipment
    SpiritualShielding,
    /// Embrace of Tibia - protects backpack items
    EmbraceOfTibia,
    /// Heart of the Mountain - reduces gold loss
    HeartOfMountain,
    /// Blood of the Mountain - faster regeneration after death
    BloodOfMountain,
    /// Twist of Fate - chance to avoid death entirely
    TwistOfFate,
}

impl BlessingType {
    /// Get the base cost for this blessing
    pub fn base_cost(&self, level: u32) -> u64 {
        let base = match self {
            BlessingType::WisdomOfSolitude => 2000,
            BlessingType::SparkOfThePhoenix => 2000,
            BlessingType::FireOfTheSuns => 2000,
            BlessingType::SpiritualShielding => 2000,
            BlessingType::EmbraceOfTibia => 2000,
            BlessingType::HeartOfMountain => 5000,
            BlessingType::BloodOfMountain => 5000,
            BlessingType::TwistOfFate => 50000,
        };
        
        // Scale with level
        let level_multiplier = (level as f64 / 30.0).max(1.0);
        (base as f64 * level_multiplier) as u64
    }

    /// Get the display name
    pub fn display_name(&self) -> &'static str {
        match self {
            BlessingType::WisdomOfSolitude => "Wisdom of Solitude",
            BlessingType::SparkOfThePhoenix => "Spark of the Phoenix",
            BlessingType::FireOfTheSuns => "Fire of the Suns",
            BlessingType::SpiritualShielding => "Spiritual Shielding",
            BlessingType::EmbraceOfTibia => "Embrace of Tibia",
            BlessingType::HeartOfMountain => "Heart of the Mountain",
            BlessingType::BloodOfMountain => "Blood of the Mountain",
            BlessingType::TwistOfFate => "Twist of Fate",
        }
    }

    /// All standard blessings (first 5)
    pub fn standard_blessings() -> &'static [BlessingType] {
        &[
            BlessingType::WisdomOfSolitude,
            BlessingType::SparkOfThePhoenix,
            BlessingType::FireOfTheSuns,
            BlessingType::SpiritualShielding,
            BlessingType::EmbraceOfTibia,
        ]
    }

    /// All blessings
    pub fn all() -> &'static [BlessingType] {
        &[
            BlessingType::WisdomOfSolitude,
            BlessingType::SparkOfThePhoenix,
            BlessingType::FireOfTheSuns,
            BlessingType::SpiritualShielding,
            BlessingType::EmbraceOfTibia,
            BlessingType::HeartOfMountain,
            BlessingType::BloodOfMountain,
            BlessingType::TwistOfFate,
        ]
    }
}

/// Death type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeathType {
    /// Killed by a monster
    Monster,
    /// Killed by another player (PvP)
    Player,
    /// Killed by environment (drowning, fire, etc.)
    Environment,
    /// Killed by a trap
    Trap,
    /// Killed by poison/condition
    Condition,
    /// Suicide/self-damage
    Suicide,
    /// Unknown cause
    Unknown,
}

/// Player's blessing status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerBlessings {
    /// Character ID
    pub character_id: Uuid,
    /// Active blessings
    pub blessings: HashSet<BlessingType>,
    /// When each blessing was obtained
    pub obtained_at: HashMap<BlessingType, DateTime<Utc>>,
    /// Total blessings purchased ever
    pub total_purchased: u32,
    /// Total gold spent on blessings
    pub total_spent: u64,
}

impl PlayerBlessings {
    /// Create new blessing status for a character
    pub fn new(character_id: Uuid) -> Self {
        Self {
            character_id,
            blessings: HashSet::new(),
            obtained_at: HashMap::new(),
            total_purchased: 0,
            total_spent: 0,
        }
    }

    /// Check if player has a specific blessing
    pub fn has_blessing(&self, blessing: BlessingType) -> bool {
        self.blessings.contains(&blessing)
    }

    /// Count active blessings
    pub fn blessing_count(&self) -> usize {
        self.blessings.len()
    }

    /// Count standard blessings (first 5)
    pub fn standard_blessing_count(&self) -> usize {
        BlessingType::standard_blessings()
            .iter()
            .filter(|b| self.blessings.contains(b))
            .count()
    }

    /// Add a blessing
    pub fn add_blessing(&mut self, blessing: BlessingType, cost: u64) {
        if self.blessings.insert(blessing) {
            self.obtained_at.insert(blessing, Utc::now());
            self.total_purchased += 1;
            self.total_spent += cost;
        }
    }

    /// Remove a blessing (consumed on death)
    pub fn remove_blessing(&mut self, blessing: BlessingType) {
        self.blessings.remove(&blessing);
        self.obtained_at.remove(&blessing);
    }

    /// Clear all blessings (on death)
    pub fn clear_all(&mut self) {
        self.blessings.clear();
        self.obtained_at.clear();
    }

    /// Check if has all standard blessings
    pub fn has_all_standard(&self) -> bool {
        self.standard_blessing_count() >= 5
    }
}

/// Death record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeathRecord {
    /// Unique death ID
    pub id: Uuid,
    /// Character who died
    pub character_id: Uuid,
    /// Character name
    pub character_name: String,
    /// Level at death
    pub level: u32,
    /// Type of death
    pub death_type: DeathType,
    /// Killer name (monster name or player name)
    pub killer_name: String,
    /// Killer ID if player
    pub killer_id: Option<Uuid>,
    /// Participants (for PvP assist tracking)
    pub participants: Vec<(Uuid, String)>,
    /// Location of death
    pub location: (i32, i32, i32),
    /// When death occurred
    pub timestamp: DateTime<Utc>,
    /// Experience lost
    pub experience_lost: u64,
    /// Skill points lost (per skill)
    pub skill_loss: HashMap<String, u64>,
    /// Items dropped
    pub items_dropped: Vec<(u32, u32)>, // (item_id, count)
    /// Blessings active at time of death
    pub blessings_active: u32,
    /// Was this an unjustified kill (for skull system)
    pub unjustified: bool,
}

/// Death penalty calculator
pub struct DeathPenalty {
    /// Base experience loss percentage (0-100)
    pub exp_loss_percent: f64,
    /// Base skill loss percentage
    pub skill_loss_percent: f64,
    /// Item drop chance
    pub item_drop_chance: f64,
    /// Container item drop chance
    pub container_drop_chance: f64,
    /// Amulet of loss protection
    pub aol_protection: bool,
}

impl DeathPenalty {
    /// Calculate death penalty based on blessings and factors
    pub fn calculate(
        level: u32,
        blessings: &PlayerBlessings,
        death_type: DeathType,
        is_vip: bool,
        vip_reduction: f64,
    ) -> Self {
        let blessing_count = blessings.standard_blessing_count();
        
        // Base penalties
        let mut exp_loss = 10.0; // 10% base
        let mut skill_loss = 10.0;
        let mut item_drop = 10.0; // 10% chance per item slot
        let mut container_drop = 10.0;
        
        // Reduce based on blessings
        let blessing_reduction = blessing_count as f64 * 1.6; // -1.6% per blessing
        exp_loss -= blessing_reduction;
        skill_loss -= blessing_reduction;
        
        // Item protection
        if blessing_count >= 5 {
            item_drop = 0.0;
            container_drop = 0.0;
        } else {
            item_drop -= blessing_count as f64 * 2.0;
            container_drop -= blessing_count as f64 * 2.0;
        }
        
        // VIP reduction
        if is_vip {
            exp_loss *= 1.0 - vip_reduction;
            skill_loss *= 1.0 - vip_reduction;
        }
        
        // PvP deaths have reduced penalty
        if death_type == DeathType::Player {
            exp_loss *= 0.5;
            skill_loss *= 0.5;
        }
        
        // Low level protection
        if level <= 20 {
            exp_loss *= 0.5;
            skill_loss *= 0.5;
            item_drop = 0.0;
            container_drop = 0.0;
        }
        
        // Twist of Fate check
        let aol = blessings.has_blessing(BlessingType::TwistOfFate);
        
        Self {
            exp_loss_percent: exp_loss.max(0.0).min(10.0),
            skill_loss_percent: skill_loss.max(0.0).min(10.0),
            item_drop_chance: item_drop.max(0.0).min(10.0),
            container_drop_chance: container_drop.max(0.0).min(10.0),
            aol_protection: aol,
        }
    }

    /// Calculate actual experience loss
    pub fn calculate_exp_loss(&self, current_exp: u64, level: u32) -> u64 {
        // Calculate experience for current level
        let level_exp = Self::experience_for_level(level);
        let prev_level_exp = Self::experience_for_level(level.saturating_sub(1));
        let level_progress = current_exp.saturating_sub(prev_level_exp);
        
        // Lose percentage of progress in current level
        ((level_progress as f64 * self.exp_loss_percent / 100.0) as u64)
            .min(level_exp - prev_level_exp) // Can't lose more than one level
    }

    /// Calculate experience required for a level
    fn experience_for_level(level: u32) -> u64 {
        if level <= 1 {
            return 0;
        }
        // Tibia's formula: 50/3 * (L^3 - 6L^2 + 17L - 12)
        let l = level as i64;
        let exp = 50 * (l.pow(3) - 6 * l.pow(2) + 17 * l - 12) / 3;
        exp.max(0) as u64
    }

    /// Calculate skill loss for a skill
    pub fn calculate_skill_loss(&self, skill_tries: u64) -> u64 {
        (skill_tries as f64 * self.skill_loss_percent / 100.0) as u64
    }
}

/// Death manager handles all death-related operations
pub struct DeathManager {
    /// Death history by character
    death_history: HashMap<Uuid, Vec<DeathRecord>>,
    /// Player blessings
    blessings: HashMap<Uuid, PlayerBlessings>,
    /// Kill tracking for skull system (killer -> recent victims)
    recent_kills: HashMap<Uuid, Vec<(Uuid, DateTime<Utc>)>>,
    /// Max deaths to keep in history per player
    max_history: usize,
}

impl DeathManager {
    /// Create a new death manager
    pub fn new() -> Self {
        Self {
            death_history: HashMap::new(),
            blessings: HashMap::new(),
            recent_kills: HashMap::new(),
            max_history: 100,
        }
    }

    /// Get or create player blessings
    pub fn get_blessings(&mut self, character_id: Uuid) -> &PlayerBlessings {
        self.blessings.entry(character_id)
            .or_insert_with(|| PlayerBlessings::new(character_id))
    }

    /// Get mutable blessings
    pub fn get_blessings_mut(&mut self, character_id: Uuid) -> &mut PlayerBlessings {
        self.blessings.entry(character_id)
            .or_insert_with(|| PlayerBlessings::new(character_id))
    }

    /// Purchase a blessing
    pub fn purchase_blessing(
        &mut self,
        character_id: Uuid,
        blessing: BlessingType,
        level: u32,
    ) -> Result<u64, DeathError> {
        let cost = blessing.base_cost(level);
        let player = self.get_blessings_mut(character_id);
        
        if player.has_blessing(blessing) {
            return Err(DeathError::AlreadyHasBlessing);
        }
        
        player.add_blessing(blessing, cost);
        Ok(cost)
    }

    /// Process a death
    pub fn process_death(
        &mut self,
        character_id: Uuid,
        character_name: &str,
        level: u32,
        current_exp: u64,
        death_type: DeathType,
        killer_name: &str,
        killer_id: Option<Uuid>,
        location: (i32, i32, i32),
        temple_location: (i32, i32, i32),
        is_vip: bool,
        vip_reduction: f64,
    ) -> DeathResult {
        let blessings = self.get_blessings(character_id).clone();
        
        // Check for Twist of Fate (chance to avoid death)
        if blessings.has_blessing(BlessingType::TwistOfFate) {
            let roll: f64 = rand::random();
            if roll < 0.05 { // 5% chance to avoid death
                // Remove the blessing
                self.get_blessings_mut(character_id).remove_blessing(BlessingType::TwistOfFate);
                return DeathResult {
                    death_avoided: true,
                    experience_lost: 0,
                    skills_lost: HashMap::new(),
                    items_dropped: Vec::new(),
                    blessings_consumed: vec![BlessingType::TwistOfFate],
                    respawn_location: location,
                    penalty_applied: false,
                };
            }
        }
        
        // Calculate penalty
        let penalty = DeathPenalty::calculate(
            level,
            &blessings,
            death_type,
            is_vip,
            vip_reduction,
        );
        
        let experience_lost = penalty.calculate_exp_loss(current_exp, level);
        
        // Create death record
        let death_record = DeathRecord {
            id: Uuid::new_v4(),
            character_id,
            character_name: character_name.to_string(),
            level,
            death_type,
            killer_name: killer_name.to_string(),
            killer_id,
            participants: Vec::new(),
            location,
            timestamp: Utc::now(),
            experience_lost,
            skill_loss: HashMap::new(),
            items_dropped: Vec::new(),
            blessings_active: blessings.blessing_count() as u32,
            unjustified: false,
        };
        
        // Record death
        self.death_history.entry(character_id)
            .or_insert_with(Vec::new)
            .push(death_record);
        
        // Trim history
        if let Some(history) = self.death_history.get_mut(&character_id) {
            if history.len() > self.max_history {
                history.drain(0..(history.len() - self.max_history));
            }
        }
        
        // Track killer for skull system
        if let Some(killer) = killer_id {
            self.recent_kills.entry(killer)
                .or_insert_with(Vec::new)
                .push((character_id, Utc::now()));
        }
        
        // Consume blessings
        let consumed = blessings.blessings.iter().cloned().collect::<Vec<_>>();
        self.get_blessings_mut(character_id).clear_all();
        
        // Calculate respawn location based on blessings
        let respawn = self.get_respawn_location(character_id, location, temple_location, &blessings);
        
        DeathResult {
            death_avoided: false,
            experience_lost,
            skills_lost: HashMap::new(), // Would be populated with actual skill losses
            items_dropped: Vec::new(), // Would be populated with actual drops
            blessings_consumed: consumed,
            respawn_location: respawn,
            penalty_applied: true,
        }
    }

    /// Get respawn location for character
    fn get_respawn_location(
        &self,
        _character_id: Uuid,
        death_location: (i32, i32, i32),
        temple_location: (i32, i32, i32),
        blessings: &PlayerBlessings,
    ) -> (i32, i32, i32) {
        // Check if Blood of Mountain blessing allows closer respawn
        if blessings.has_blessing(BlessingType::BloodOfMountain) {
            // Blood of Mountain allows respawn closer to death location
            // Find the nearest temple/respawn point to death location
            // For now, return death location floor's temple equivalent
            (death_location.0, death_location.1, 7) // Surface level respawn
        } else {
            // Standard respawn at character's designated temple
            temple_location
        }
    }

    /// Get death history for a character
    pub fn get_death_history(&self, character_id: Uuid) -> &[DeathRecord] {
        self.death_history.get(&character_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Get recent kills by a character (for skull tracking)
    pub fn get_recent_kills(&self, character_id: Uuid, hours: i64) -> Vec<Uuid> {
        let cutoff = Utc::now() - chrono::Duration::hours(hours);
        
        self.recent_kills.get(&character_id)
            .map(|kills| {
                kills.iter()
                    .filter(|(_, time)| *time > cutoff)
                    .map(|(victim, _)| *victim)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Count unjustified kills for skull calculation
    pub fn count_unjustified_kills(&self, character_id: Uuid, hours: i64) -> usize {
        self.get_recent_kills(character_id, hours).len()
    }

    /// Check if character should have a skull
    pub fn get_skull_type(&self, character_id: Uuid) -> SkullType {
        let kills_24h = self.count_unjustified_kills(character_id, 24);
        let kills_week = self.count_unjustified_kills(character_id, 168);
        let kills_month = self.count_unjustified_kills(character_id, 720);
        
        if kills_month >= 20 {
            SkullType::Black
        } else if kills_week >= 10 {
            SkullType::Red
        } else if kills_24h >= 3 {
            SkullType::Red
        } else if kills_24h >= 1 {
            SkullType::White
        } else {
            SkullType::None
        }
    }
}

impl Default for DeathManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of processing a death
#[derive(Debug, Clone)]
pub struct DeathResult {
    /// Was death avoided (Twist of Fate)
    pub death_avoided: bool,
    /// Experience lost
    pub experience_lost: u64,
    /// Skills lost (skill_name -> tries_lost)
    pub skills_lost: HashMap<String, u64>,
    /// Items dropped (item_id, count)
    pub items_dropped: Vec<(u32, u32)>,
    /// Blessings that were consumed
    pub blessings_consumed: Vec<BlessingType>,
    /// Where to respawn
    pub respawn_location: (i32, i32, i32),
    /// Was penalty applied (false if protected)
    pub penalty_applied: bool,
}

/// Skull types for PvP tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkullType {
    /// No skull
    None,
    /// Yellow skull (attacked someone)
    Yellow,
    /// Green skull (party/guild)
    Green,
    /// White skull (1-2 unjustified kills in 24h)
    White,
    /// Red skull (3+ unjustified kills in 24h or 10+ in week)
    Red,
    /// Black skull (extreme killer - 20+ kills in month)
    Black,
    /// Orange skull (war enemy)
    Orange,
}

/// Death system errors
#[derive(Debug, Clone)]
pub enum DeathError {
    CharacterNotFound,
    AlreadyHasBlessing,
    BlessingNotAvailable,
    InsufficientFunds,
    DatabaseError(String),
}

impl std::fmt::Display for DeathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeathError::CharacterNotFound => write!(f, "Character not found"),
            DeathError::AlreadyHasBlessing => write!(f, "Already has this blessing"),
            DeathError::BlessingNotAvailable => write!(f, "Blessing not available"),
            DeathError::InsufficientFunds => write!(f, "Insufficient funds for blessing"),
            DeathError::DatabaseError(e) => write!(f, "Database error: {}", e),
        }
    }
}

impl std::error::Error for DeathError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blessing_cost() {
        let cost_30 = BlessingType::WisdomOfSolitude.base_cost(30);
        let cost_100 = BlessingType::WisdomOfSolitude.base_cost(100);
        assert!(cost_100 > cost_30);
    }

    #[test]
    fn test_death_penalty_with_blessings() {
        let mut blessings = PlayerBlessings::new(Uuid::new_v4());
        
        // Add all standard blessings
        for b in BlessingType::standard_blessings() {
            blessings.add_blessing(*b, 0);
        }
        
        let penalty = DeathPenalty::calculate(
            100,
            &blessings,
            DeathType::Monster,
            false,
            0.0,
        );
        
        // With 5 blessings, should have 0 item drop
        assert_eq!(penalty.item_drop_chance, 0.0);
        // Experience loss should be reduced
        assert!(penalty.exp_loss_percent < 10.0);
    }

    #[test]
    fn test_skull_calculation() {
        let mut manager = DeathManager::new();
        let killer = Uuid::new_v4();
        let victim = Uuid::new_v4();
        
        // No kills = no skull
        assert_eq!(manager.get_skull_type(killer), SkullType::None);
        
        // Add kills
        manager.recent_kills.entry(killer)
            .or_insert_with(Vec::new)
            .push((victim, Utc::now()));
        
        // 1 kill = white skull
        assert_eq!(manager.get_skull_type(killer), SkullType::White);
    }
}
