//! Creature system - players, monsters, and NPCs

use crate::item::{DamageType, SkillType};
use crate::position::{Direction, Position};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;

static CREATURE_ID_COUNTER: AtomicU32 = AtomicU32::new(1);

fn next_creature_id() -> u32 {
    CREATURE_ID_COUNTER.fetch_add(1, Ordering::SeqCst)
}

/// Creature types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreatureType {
    Player,
    Monster,
    Npc,
    Summon,
}

/// Creature skulls (PvP indicators)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[repr(u8)]
pub enum Skull {
    #[default]
    None = 0,
    Yellow = 1,
    Green = 2,
    White = 3,
    Red = 4,
    Black = 5,
    Orange = 6,
}

/// Party/guild shields
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[repr(u8)]
pub enum Shield {
    #[default]
    None = 0,
    Whiteyellow = 1,
    Whiteblue = 2,
    Blue = 3,
    Yellow = 4,
    BlueSharedexp = 5,
    YellowSharedexp = 6,
    BluenosharedexpBlink = 7,
    YellownosharedexpBlink = 8,
    BluenosharedExp = 9,
    YellownosharedExp = 10,
    Gray = 11,
}

/// War emblems
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[repr(u8)]
pub enum Emblem {
    #[default]
    None = 0,
    Green = 1,
    Red = 2,
    Blue = 3,
    Member = 4,
    Other = 5,
}

/// Outfit definition - matches client Outfit struct
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Outfit {
    /// Main outfit type ID
    pub look_type: u16,
    /// Item ID when appearing as an item (lookType = 0)
    pub look_type_ex: u16,
    /// Head color (0-132)
    pub look_head: u8,
    /// Body color (0-132)
    pub look_body: u8,
    /// Legs color (0-132)
    pub look_legs: u8,
    /// Feet color (0-132)
    pub look_feet: u8,
    /// Addon flags (0=none, 1=first, 2=second, 3=both)
    pub look_addons: u8,
    /// Mount type ID
    pub look_mount: u16,
    /// Mount head color
    pub look_mount_head: u8,
    /// Mount body color
    pub look_mount_body: u8,
    /// Mount legs color
    pub look_mount_legs: u8,
    /// Mount feet color
    pub look_mount_feet: u8,
    /// Familiar type ID (summoned companion)
    pub familiar: u16,
    /// Whether this is a podium display outfit
    pub podium: bool,
}

impl Outfit {
    pub fn new(look_type: u16) -> Self {
        Self {
            look_type,
            ..Default::default()
        }
    }

    pub fn with_colors(look_type: u16, head: u8, body: u8, legs: u8, feet: u8) -> Self {
        Self {
            look_type,
            look_head: head,
            look_body: body,
            look_legs: legs,
            look_feet: feet,
            ..Default::default()
        }
    }

    pub fn with_mount(mut self, mount: u16) -> Self {
        self.look_mount = mount;
        self
    }

    pub fn with_mount_colors(mut self, head: u8, body: u8, legs: u8, feet: u8) -> Self {
        self.look_mount_head = head;
        self.look_mount_body = body;
        self.look_mount_legs = legs;
        self.look_mount_feet = feet;
        self
    }

    pub fn with_familiar(mut self, familiar: u16) -> Self {
        self.familiar = familiar;
        self
    }

    pub fn with_addons(mut self, addons: u8) -> Self {
        self.look_addons = addons;
        self
    }

    /// Returns true if this outfit has a mount
    pub fn has_mount(&self) -> bool {
        self.look_mount != 0
    }

    /// Returns true if this outfit has a familiar
    pub fn has_familiar(&self) -> bool {
        self.familiar != 0
    }

    /// Returns true if this represents an item (not a creature outfit)
    pub fn is_item(&self) -> bool {
        self.look_type == 0 && self.look_type_ex != 0
    }

    /// Returns true if this outfit is invisible (no type and no item)
    pub fn is_invisible(&self) -> bool {
        self.look_type == 0 && self.look_type_ex == 0
    }
}

/// Light info
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Light {
    pub level: u8,
    pub color: u8,
}

/// Base creature stats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreatureStats {
    pub health: i32,
    pub max_health: i32,
    pub mana: i32,
    pub max_mana: i32,
    pub soul: u8,
    pub stamina: u16,
    pub capacity: u32,
    pub max_capacity: u32,
    pub base_speed: u16,
    pub level: u16,
    pub experience: u64,
    pub magic_level: u8,
    pub magic_level_percent: u8,
}

/// Combat stats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CombatStats {
    pub attack_target: Option<u32>,
    pub follow_target: Option<u32>,
    pub attack_mode: AttackMode,
    pub chase_mode: ChaseMode,
    pub secure_mode: SecureMode,
    pub pvp_mode: PvpMode,
    pub last_attack_time: u64,
    pub last_hit_time: u64,
    pub attack_ticks: u32,
}

/// Attack modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[repr(u8)]
pub enum AttackMode {
    Balanced = 0,
    #[default]
    Offensive = 1,
    Defensive = 2,
}

/// Chase modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[repr(u8)]
pub enum ChaseMode {
    #[default]
    Stand = 0,
    Chase = 1,
}

/// Secure mode (attack unmarked players)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[repr(u8)]
pub enum SecureMode {
    #[default]
    On = 0,
    Off = 1,
}

/// PvP modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[repr(u8)]
pub enum PvpMode {
    #[default]
    Dove = 0,
    WhiteHand = 1,
    YellowHand = 2,
    RedFist = 3,
}

/// Condition types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConditionType {
    None,
    Poison,
    Fire,
    Energy,
    Bleeding,
    Haste,
    Paralyze,
    Outfit,
    Invisible,
    Light,
    ManaShield,
    InFight,
    Drunk,
    ExhaustWeapon,
    Regeneration,
    Soul,
    Drown,
    Muted,
    ChannelMuted,
    YellTicks,
    Attributes,
    Freezing,
    Dazzled,
    Cursed,
    ExhaustCombat,
    ExhaustHeal,
    Pacified,
    SpellCooldown,
    SpellGroupCooldown,
    RootedCreature,
    Feared,
}

/// Condition instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub condition_type: ConditionType,
    pub ticks: i32,
    pub interval: i32,
    pub total_damage: i32,
    pub start_damage: i32,
    pub persistent: bool,
    pub subtype: i32,
}

impl Condition {
    pub fn new(condition_type: ConditionType, ticks: i32) -> Self {
        Self {
            condition_type,
            ticks,
            interval: 1000,
            total_damage: 0,
            start_damage: 0,
            persistent: false,
            subtype: 0,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.ticks <= 0
    }

    pub fn tick(&mut self, elapsed_ms: i32) -> bool {
        self.ticks -= elapsed_ms;
        self.ticks <= 0
    }
}

/// Base creature structure
#[derive(Debug)]
pub struct Creature {
    pub id: u32,
    pub name: String,
    pub creature_type: CreatureType,
    pub position: Position,
    pub direction: Direction,
    pub outfit: Outfit,
    pub light: Light,
    pub stats: CreatureStats,
    pub combat: CombatStats,
    pub skull: Skull,
    pub shield: Shield,
    pub emblem: Emblem,
    pub conditions: Vec<Condition>,
    pub visible: bool,
    pub removed: bool,
    pub path: Vec<Direction>,
    pub walk_ticks: u32,
    pub last_step_time: u64,
    pub skills: HashMap<SkillType, (u8, u8)>, // (level, percent)
    pub resistances: HashMap<DamageType, i32>,
    pub summon_master_id: Option<u32>,
    pub summons: Vec<u32>,
}

impl Creature {
    pub fn new(name: String, creature_type: CreatureType, position: Position) -> Self {
        Self {
            id: next_creature_id(),
            name,
            creature_type,
            position,
            direction: Direction::South,
            outfit: Outfit::default(),
            light: Light::default(),
            stats: CreatureStats::default(),
            combat: CombatStats::default(),
            skull: Skull::None,
            shield: Shield::None,
            emblem: Emblem::None,
            conditions: Vec::new(),
            visible: true,
            removed: false,
            path: Vec::new(),
            walk_ticks: 0,
            last_step_time: 0,
            skills: HashMap::new(),
            resistances: HashMap::new(),
            summon_master_id: None,
            summons: Vec::new(),
        }
    }

    /// Check if creature is alive
    pub fn is_alive(&self) -> bool {
        self.stats.health > 0 && !self.removed
    }

    /// Check if creature is a player
    pub fn is_player(&self) -> bool {
        self.creature_type == CreatureType::Player
    }

    /// Check if creature is a monster
    pub fn is_monster(&self) -> bool {
        self.creature_type == CreatureType::Monster
    }

    /// Check if creature is an NPC
    pub fn is_npc(&self) -> bool {
        self.creature_type == CreatureType::Npc
    }

    /// Check if creature is a summon
    pub fn is_summon(&self) -> bool {
        self.creature_type == CreatureType::Summon || self.summon_master_id.is_some()
    }

    /// Get health percentage
    pub fn health_percent(&self) -> u8 {
        if self.stats.max_health == 0 {
            return 0;
        }
        ((self.stats.health as f64 / self.stats.max_health as f64) * 100.0).round() as u8
    }

    /// Get current speed
    pub fn get_speed(&self) -> u16 {
        let mut speed = self.stats.base_speed;

        // Apply haste/paralyze
        for condition in &self.conditions {
            match condition.condition_type {
                ConditionType::Haste => {
                    speed = (speed as f64 * 1.3) as u16;
                }
                ConditionType::Paralyze => {
                    speed = (speed as f64 * 0.5) as u16;
                }
                _ => {}
            }
        }

        speed.max(1)
    }

    /// Get step duration based on speed
    pub fn get_step_duration(&self, ground_speed: u16) -> u32 {
        let speed = self.get_speed() as u32;
        if speed == 0 {
            return 1000;
        }

        let ground_speed = ground_speed.max(150) as u32;
        let duration = (1000 * ground_speed) / speed;
        duration.max(50)
    }

    /// Add a condition
    pub fn add_condition(&mut self, condition: Condition) {
        // Remove existing condition of same type
        self.conditions.retain(|c| c.condition_type != condition.condition_type);
        self.conditions.push(condition);
    }

    /// Remove a condition
    pub fn remove_condition(&mut self, condition_type: ConditionType) -> bool {
        let initial_len = self.conditions.len();
        self.conditions.retain(|c| c.condition_type != condition_type);
        self.conditions.len() != initial_len
    }

    /// Check if creature has a condition
    pub fn has_condition(&self, condition_type: ConditionType) -> bool {
        self.conditions.iter().any(|c| c.condition_type == condition_type)
    }

    /// Get condition
    pub fn get_condition(&self, condition_type: ConditionType) -> Option<&Condition> {
        self.conditions.iter().find(|c| c.condition_type == condition_type)
    }

    /// Apply damage
    pub fn apply_damage(&mut self, damage: i32, damage_type: DamageType) -> i32 {
        let resistance = self.resistances.get(&damage_type).copied().unwrap_or(0);
        let actual_damage = (damage as f64 * (100 - resistance) as f64 / 100.0).round() as i32;

        if self.has_condition(ConditionType::ManaShield) {
            let mana_damage = actual_damage.min(self.stats.mana);
            self.stats.mana -= mana_damage;
            let remaining = actual_damage - mana_damage;
            self.stats.health -= remaining;
        } else {
            self.stats.health -= actual_damage;
        }

        self.stats.health = self.stats.health.max(0);
        actual_damage
    }

    /// Heal
    pub fn heal(&mut self, amount: i32) -> i32 {
        let actual_heal = amount.min(self.stats.max_health - self.stats.health);
        self.stats.health += actual_heal;
        actual_heal
    }

    /// Restore mana
    pub fn restore_mana(&mut self, amount: i32) -> i32 {
        let actual_restore = amount.min(self.stats.max_mana - self.stats.mana);
        self.stats.mana += actual_restore;
        actual_restore
    }

    /// Set attack target
    pub fn set_attack_target(&mut self, target_id: Option<u32>) {
        self.combat.attack_target = target_id;
    }

    /// Set follow target
    pub fn set_follow_target(&mut self, target_id: Option<u32>) {
        self.combat.follow_target = target_id;
    }

    /// Check if in combat
    pub fn is_in_combat(&self) -> bool {
        self.has_condition(ConditionType::InFight)
    }

    /// Turn to direction
    pub fn turn(&mut self, direction: Direction) {
        self.direction = direction;
    }

    /// Turn towards a position
    pub fn turn_to(&mut self, target: &Position) {
        self.direction = self.position.direction_to(target);
    }

    /// Can see position (line of sight check simplified)
    pub fn can_see(&self, target: &Position) -> bool {
        if self.position.z != target.z {
            return false;
        }
        self.position.distance_to(target) <= 9
    }

    /// Can see creature (checks invisibility)
    pub fn can_see_creature(&self, other: &Creature) -> bool {
        if !other.visible {
            return false;
        }
        if other.has_condition(ConditionType::Invisible) {
            // Players can see invisible if they have specific conditions/items
            return false;
        }
        self.can_see(&other.position)
    }

    /// Get skill level
    pub fn get_skill(&self, skill: SkillType) -> u8 {
        self.skills.get(&skill).map(|(level, _)| *level).unwrap_or(10)
    }

    /// Get skill percent
    pub fn get_skill_percent(&self, skill: SkillType) -> u8 {
        self.skills.get(&skill).map(|(_, pct)| *pct).unwrap_or(0)
    }

    /// Set skill level and percent
    pub fn set_skill(&mut self, skill: SkillType, level: u8, percent: u8) {
        self.skills.insert(skill, (level, percent));
    }

    /// Process conditions tick
    pub fn tick_conditions(&mut self, elapsed_ms: i32) -> Vec<(ConditionType, i32)> {
        let mut damages = Vec::new();

        self.conditions.retain_mut(|condition| {
            let expired = condition.tick(elapsed_ms);
            if !expired && condition.total_damage > 0 {
                // Calculate periodic damage
                let damage = condition.start_damage.max(1);
                damages.push((condition.condition_type, damage));
            }
            !expired
        });

        damages
    }
}

impl Clone for Creature {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            name: self.name.clone(),
            creature_type: self.creature_type,
            position: self.position,
            direction: self.direction,
            outfit: self.outfit,
            light: self.light,
            stats: self.stats.clone(),
            combat: self.combat.clone(),
            skull: self.skull,
            shield: self.shield,
            emblem: self.emblem,
            conditions: self.conditions.clone(),
            visible: self.visible,
            removed: self.removed,
            path: self.path.clone(),
            walk_ticks: self.walk_ticks,
            last_step_time: self.last_step_time,
            skills: self.skills.clone(),
            resistances: self.resistances.clone(),
            summon_master_id: self.summon_master_id,
            summons: self.summons.clone(),
        }
    }
}

/// Thread-safe creature wrapper
pub type SharedCreature = Arc<RwLock<Creature>>;

/// Monster type definition (loaded from monsters.xml)
#[derive(Debug, Clone)]
pub struct Monster {
    pub name: String,
    pub name_description: String,
    pub race: MonsterRace,
    pub experience: u64,
    pub health: i32,
    pub max_health: i32,
    pub speed: u16,
    pub outfit: Outfit,
    pub corpse_id: u16,
    pub light: Light,
    pub target_distance: u8,
    pub run_health: i32,
    pub pushable: bool,
    pub can_push_items: bool,
    pub can_push_creatures: bool,
    pub hostile: bool,
    pub static_attack_chance: u8,
    pub flee_health: i32,
    pub attacks: Vec<MonsterAttack>,
    pub defenses: Vec<MonsterDefense>,
    pub elements: HashMap<DamageType, i32>,
    pub immunities: Vec<ConditionType>,
    pub voices: Vec<MonsterVoice>,
    pub summons: Vec<MonsterSummon>,
    pub loot: Vec<LootItem>,
    pub script: Option<String>,
}

impl Monster {
    pub fn new(name: String) -> Self {
        Self {
            name,
            name_description: String::new(),
            race: MonsterRace::Blood,
            experience: 0,
            health: 100,
            max_health: 100,
            speed: 200,
            outfit: Outfit::default(),
            corpse_id: 0,
            light: Light::default(),
            target_distance: 1,
            run_health: 0,
            pushable: true,
            can_push_items: false,
            can_push_creatures: false,
            hostile: true,
            static_attack_chance: 95,
            flee_health: 0,
            attacks: Vec::new(),
            defenses: Vec::new(),
            elements: HashMap::new(),
            immunities: Vec::new(),
            voices: Vec::new(),
            summons: Vec::new(),
            loot: Vec::new(),
            script: None,
        }
    }

    /// Create a creature instance from this monster type
    pub fn spawn(&self, position: Position) -> Creature {
        let mut creature = Creature::new(self.name.clone(), CreatureType::Monster, position);
        creature.outfit = self.outfit;
        creature.light = self.light;
        creature.stats.health = self.health;
        creature.stats.max_health = self.max_health;
        creature.stats.base_speed = self.speed;
        creature.stats.experience = self.experience;
        creature.resistances = self.elements.clone();
        creature
    }
}

/// Monster races
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MonsterRace {
    None,
    Venom,
    Blood,
    Undead,
    Fire,
    Energy,
}

/// Monster attack
#[derive(Debug, Clone)]
pub struct MonsterAttack {
    pub name: String,
    pub interval: u32,
    pub chance: u8,
    pub range: u8,
    pub min_damage: i32,
    pub max_damage: i32,
    pub damage_type: DamageType,
    pub condition: Option<ConditionType>,
    pub condition_duration: i32,
    pub shoot_effect: Option<u8>,
    pub area_effect: Option<u8>,
}

/// Monster defense
#[derive(Debug, Clone)]
pub struct MonsterDefense {
    pub name: String,
    pub interval: u32,
    pub chance: u8,
    pub min: i32,
    pub max: i32,
}

/// Monster voice
#[derive(Debug, Clone)]
pub struct MonsterVoice {
    pub sentence: String,
    pub yell: bool,
    pub interval: u32,
    pub chance: u8,
}

/// Monster summon
#[derive(Debug, Clone)]
pub struct MonsterSummon {
    pub name: String,
    pub interval: u32,
    pub chance: u8,
    pub max: u8,
}

/// Loot item
#[derive(Debug, Clone)]
pub struct LootItem {
    pub item_id: u16,
    pub chance: f32,
    pub count_min: u8,
    pub count_max: u8,
    pub sub_loot: Vec<LootItem>,
}

/// Monster loader
pub struct MonsterLoader {
    monsters: HashMap<String, Monster>,
}

impl MonsterLoader {
    pub fn new() -> Self {
        Self {
            monsters: HashMap::new(),
        }
    }

    /// Load monsters from XML files
    pub fn load_directory(&mut self, path: &str) -> crate::Result<()> {
        tracing::info!("Loading monsters from: {}", path);
        // Implementation will parse monster XML files
        Ok(())
    }

    /// Load a single monster
    pub fn load_monster(&mut self, path: &str) -> crate::Result<()> {
        tracing::debug!("Loading monster: {}", path);
        // Parse individual monster XML
        Ok(())
    }

    /// Get monster by name
    pub fn get(&self, name: &str) -> Option<&Monster> {
        self.monsters.get(&name.to_lowercase())
    }

    /// Get all monsters
    pub fn all(&self) -> &HashMap<String, Monster> {
        &self.monsters
    }

    /// Add a monster
    pub fn add(&mut self, monster: Monster) {
        self.monsters.insert(monster.name.to_lowercase(), monster);
    }
}

impl Default for MonsterLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creature_creation() {
        let creature = Creature::new(
            "Test".to_string(),
            CreatureType::Monster,
            Position::new(100, 100, 7),
        );
        assert!(creature.is_monster());
        assert!(creature.is_alive());
    }

    #[test]
    fn test_damage_application() {
        let mut creature = Creature::new(
            "Test".to_string(),
            CreatureType::Monster,
            Position::new(100, 100, 7),
        );
        creature.stats.health = 100;
        creature.stats.max_health = 100;

        let damage = creature.apply_damage(30, DamageType::Physical);
        assert_eq!(damage, 30);
        assert_eq!(creature.stats.health, 70);
    }

    #[test]
    fn test_conditions() {
        let mut creature = Creature::new(
            "Test".to_string(),
            CreatureType::Monster,
            Position::new(100, 100, 7),
        );

        creature.add_condition(Condition::new(ConditionType::Poison, 5000));
        assert!(creature.has_condition(ConditionType::Poison));

        creature.remove_condition(ConditionType::Poison);
        assert!(!creature.has_condition(ConditionType::Poison));
    }
}
