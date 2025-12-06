//! Combat damage formulas
//!
//! Implements the Tibia damage calculation formulas for melee, magic, and distance.

use crate::damage::{DamageInfo, DamageType};
use shadow_world::creature::Creature;

/// Base combat formula trait
pub trait CombatFormula {
    /// Calculate minimum damage
    fn get_min_damage(&self, attacker: &Creature, level: u16, skill: u8) -> i32;

    /// Calculate maximum damage
    fn get_max_damage(&self, attacker: &Creature, level: u16, skill: u8) -> i32;

    /// Calculate actual damage (random between min and max)
    fn calculate_damage(&self, attacker: &Creature, level: u16, skill: u8) -> i32 {
        let min = self.get_min_damage(attacker, level, skill);
        let max = self.get_max_damage(attacker, level, skill);
        if max <= min {
            return min;
        }
        min + rand::random::<i32>().abs() % (max - min + 1)
    }
}

/// Melee damage formula (for physical attacks)
#[derive(Debug, Clone)]
pub struct MeleeFormula {
    /// Attack value from weapon
    pub attack: i32,
    /// Attack mode factor (1.0 = balanced, 0.8 = defensive, 1.2 = offensive)
    pub attack_factor: f32,
    /// Element type (for enchanted weapons)
    pub element_type: Option<DamageType>,
    /// Element damage
    pub element_damage: i32,
}

impl MeleeFormula {
    pub fn new(attack: i32) -> Self {
        Self {
            attack,
            attack_factor: 1.0,
            element_type: None,
            element_damage: 0,
        }
    }

    pub fn with_attack_factor(mut self, factor: f32) -> Self {
        self.attack_factor = factor;
        self
    }

    pub fn with_element(mut self, damage_type: DamageType, damage: i32) -> Self {
        self.element_type = Some(damage_type);
        self.element_damage = damage;
        self
    }
}

impl CombatFormula for MeleeFormula {
    fn get_min_damage(&self, attacker: &Creature, level: u16, skill: u8) -> i32 {
        // Tibia formula: 0.5 * attack * skill / 20 + level / 5
        let base = (0.5 * self.attack as f32 * skill as f32 / 20.0 + level as f32 / 5.0) * self.attack_factor;
        (base * 0.5) as i32 // Min is 50% of calculated
    }

    fn get_max_damage(&self, attacker: &Creature, level: u16, skill: u8) -> i32 {
        // Tibia formula: attack * skill / 20 + level / 5
        let base = (self.attack as f32 * skill as f32 / 20.0 + level as f32 / 5.0) * self.attack_factor;
        base as i32
    }
}

/// Magic damage formula (for spells and runes)
#[derive(Debug, Clone)]
pub struct MagicFormula {
    /// Base minimum damage
    pub min_damage: i32,
    /// Base maximum damage
    pub max_damage: i32,
    /// Level factor
    pub level_factor: f32,
    /// Magic level factor
    pub magic_factor: f32,
    /// Damage type
    pub damage_type: DamageType,
}

impl MagicFormula {
    pub fn new(min_damage: i32, max_damage: i32, damage_type: DamageType) -> Self {
        Self {
            min_damage,
            max_damage,
            level_factor: 0.2,
            magic_factor: 1.0,
            damage_type,
        }
    }

    /// Create formula from spell parameters (like TFS)
    /// Formula: min = (level * 2 + magic_level * 3) * min_factor
    /// Formula: max = (level * 2 + magic_level * 3) * max_factor
    pub fn from_factors(min_factor: f32, max_factor: f32, damage_type: DamageType) -> Self {
        Self {
            min_damage: 0,
            max_damage: 0,
            level_factor: min_factor,
            magic_factor: max_factor,
            damage_type,
        }
    }
}

impl CombatFormula for MagicFormula {
    fn get_min_damage(&self, attacker: &Creature, level: u16, _skill: u8) -> i32 {
        let magic_level = attacker.stats.magic_level as f32;

        if self.min_damage != 0 {
            // Fixed damage + scaling
            (self.min_damage as f32 + level as f32 * self.level_factor + magic_level * self.magic_factor) as i32
        } else {
            // Factor-based formula
            ((level as f32 * 2.0 + magic_level * 3.0) * self.level_factor) as i32
        }
    }

    fn get_max_damage(&self, attacker: &Creature, level: u16, _skill: u8) -> i32 {
        let magic_level = attacker.stats.magic_level as f32;

        if self.max_damage != 0 {
            // Fixed damage + scaling
            (self.max_damage as f32 + level as f32 * self.level_factor + magic_level * self.magic_factor) as i32
        } else {
            // Factor-based formula
            ((level as f32 * 2.0 + magic_level * 3.0) * self.magic_factor) as i32
        }
    }
}

/// Distance damage formula (for bows, crossbows, throwing weapons)
#[derive(Debug, Clone)]
pub struct DistanceFormula {
    /// Attack value from weapon
    pub attack: i32,
    /// Attack from ammunition
    pub ammo_attack: i32,
    /// Hit chance modifier
    pub hit_chance: i32,
    /// Attack factor
    pub attack_factor: f32,
}

impl DistanceFormula {
    pub fn new(attack: i32, ammo_attack: i32, hit_chance: i32) -> Self {
        Self {
            attack,
            ammo_attack,
            hit_chance,
            attack_factor: 1.0,
        }
    }
}

impl CombatFormula for DistanceFormula {
    fn get_min_damage(&self, _attacker: &Creature, level: u16, skill: u8) -> i32 {
        let total_attack = self.attack + self.ammo_attack;
        let base = (0.5 * total_attack as f32 * skill as f32 / 20.0 + level as f32 / 5.0) * self.attack_factor;
        (base * 0.5) as i32
    }

    fn get_max_damage(&self, _attacker: &Creature, level: u16, skill: u8) -> i32 {
        let total_attack = self.attack + self.ammo_attack;
        let base = (total_attack as f32 * skill as f32 / 20.0 + level as f32 / 5.0) * self.attack_factor;
        base as i32
    }
}

/// Healing formula
#[derive(Debug, Clone)]
pub struct HealingFormula {
    pub min_heal: i32,
    pub max_heal: i32,
    pub level_factor: f32,
    pub magic_factor: f32,
}

impl HealingFormula {
    pub fn new(min_heal: i32, max_heal: i32) -> Self {
        Self {
            min_heal,
            max_heal,
            level_factor: 0.2,
            magic_factor: 1.0,
        }
    }

    pub fn calculate(&self, level: u16, magic_level: u8) -> i32 {
        let min = self.min_heal as f32 + level as f32 * self.level_factor + magic_level as f32 * self.magic_factor;
        let max = self.max_heal as f32 + level as f32 * self.level_factor + magic_level as f32 * self.magic_factor;

        let min = min as i32;
        let max = max as i32;

        if max <= min {
            return min;
        }

        min + rand::random::<i32>().abs() % (max - min + 1)
    }
}

/// Calculate defense value
pub fn calculate_defense(base_defense: i32, extra_defense: i32, shielding_skill: u8) -> i32 {
    let defense = base_defense + extra_defense;
    (defense as f32 * shielding_skill as f32 / 100.0) as i32
}

/// Calculate armor reduction
pub fn calculate_armor_reduction(armor: i32) -> i32 {
    if armor <= 0 {
        return 0;
    }
    armor / 2 + rand::random::<i32>().abs() % (armor / 2 + 1)
}

/// Calculate hit chance for distance attacks
pub fn calculate_hit_chance(distance_skill: u8, hit_chance: i32, distance: u32) -> f32 {
    // Base formula: skill + hit_chance - distance_penalty
    let distance_penalty = if distance > 1 {
        (distance - 1) as f32 * 2.0
    } else {
        0.0
    };

    let chance = (distance_skill as f32 + hit_chance as f32 - distance_penalty) / 100.0;
    chance.clamp(0.1, 0.95) // 10% minimum, 95% maximum
}

/// Experience gained from combat
pub fn calculate_experience(
    monster_experience: u64,
    player_level: u16,
    monster_level: u16,
    stamina_minutes: u16,
) -> u64 {
    let mut exp = monster_experience;

    // Stamina bonus
    if stamina_minutes > 42 * 60 {
        // Happy hour bonus (150%)
        exp = (exp as f64 * 1.5) as u64;
    } else if stamina_minutes < 14 * 60 {
        // Low stamina penalty (50%)
        exp = (exp as f64 * 0.5) as u64;
    }

    // Level difference penalty (optional, for more balanced servers)
    let level_diff = player_level as i32 - monster_level as i32;
    if level_diff > 50 {
        // Reduce exp for killing much weaker monsters
        let penalty = 1.0 - ((level_diff - 50) as f64 * 0.01).min(0.9);
        exp = (exp as f64 * penalty) as u64;
    }

    exp
}

/// Skill advancement calculation
pub fn calculate_skill_tries(skill_level: u8, vocation_factor: f32) -> u64 {
    // Formula: base_tries * skill_level^2 * vocation_factor
    let base_tries = 50.0;
    (base_tries * (skill_level as f64).powi(2) * vocation_factor as f64) as u64
}

/// Magic level advancement calculation
pub fn calculate_mana_spent_for_level(magic_level: u8, vocation_factor: f32) -> u64 {
    // Formula: base * magic_level^1.5 * vocation_factor
    let base = 1600.0;
    (base * (magic_level as f64).powf(1.5) * vocation_factor as f64) as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use shadow_world::position::Position;

    fn create_test_creature() -> Creature {
        let mut creature = Creature::new(
            "Test".to_string(),
            shadow_world::creature::CreatureType::Player,
            Position::new(100, 100, 7),
        );
        creature.stats.level = 100;
        creature.stats.magic_level = 50;
        creature
    }

    #[test]
    fn test_melee_formula() {
        let formula = MeleeFormula::new(50);
        let creature = create_test_creature();

        let max = formula.get_max_damage(&creature, 100, 100);
        let min = formula.get_min_damage(&creature, 100, 100);

        assert!(max > 0);
        assert!(min > 0);
        assert!(max >= min);
    }

    #[test]
    fn test_magic_formula() {
        let formula = MagicFormula::new(50, 100, DamageType::Fire);
        let creature = create_test_creature();

        let max = formula.get_max_damage(&creature, 100, 0);
        let min = formula.get_min_damage(&creature, 100, 0);

        assert!(max > 0);
        assert!(min > 0);
        assert!(max >= min);
    }

    #[test]
    fn test_hit_chance() {
        let chance = calculate_hit_chance(100, 90, 1);
        assert!(chance > 0.5);

        let far_chance = calculate_hit_chance(100, 90, 10);
        assert!(far_chance < chance);
    }
}
