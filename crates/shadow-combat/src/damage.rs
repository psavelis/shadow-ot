//! Damage types and damage info structures

use serde::{Deserialize, Serialize};

// Re-export DamageType from shadow-world to ensure type compatibility
pub use shadow_world::item::DamageType;

/// Extension trait for DamageType with combat-specific methods
pub trait DamageTypeExt {
    /// Get the magic effect for this damage type
    fn get_magic_effect(&self) -> u8;
    /// Get text color for damage display
    fn get_text_color(&self) -> u8;
    /// Check if this is a healing type
    fn is_healing(&self) -> bool;
    /// Check if this is a drain type
    fn is_drain(&self) -> bool;
}

impl DamageTypeExt for DamageType {
    fn get_magic_effect(&self) -> u8 {
        match self {
            DamageType::Physical => 1,  // CONST_ME_DRAWBLOOD
            DamageType::Energy => 11,   // CONST_ME_ENERGYDAMAGE
            DamageType::Earth => 6,     // CONST_ME_HITBYPOISON
            DamageType::Fire => 5,      // CONST_ME_HITBYFIRE
            DamageType::Ice => 43,      // CONST_ME_ICEATTACK
            DamageType::Holy => 39,     // CONST_ME_HOLYDAMAGE
            DamageType::Death => 18,    // CONST_ME_MORTAREA
            DamageType::Drown => 30,    // CONST_ME_WATERSPLASH
            DamageType::LifeDrain => 16, // CONST_ME_MAGIC_RED
            DamageType::ManaDrain => 14, // CONST_ME_MAGIC_BLUE
            DamageType::Healing => 15,  // CONST_ME_MAGIC_GREEN
            DamageType::ManaRestore => 14,
        }
    }

    fn get_text_color(&self) -> u8 {
        match self {
            DamageType::Physical => 180, // Red
            DamageType::Energy => 35,    // Purple
            DamageType::Earth => 30,     // Green
            DamageType::Fire => 198,     // Orange
            DamageType::Ice => 35,       // Light blue
            DamageType::Holy => 210,     // Yellow
            DamageType::Death => 108,    // Dark red
            DamageType::Drown => 35,     // Blue
            DamageType::LifeDrain => 108,
            DamageType::ManaDrain => 35,
            DamageType::Healing => 65,   // Green
            DamageType::ManaRestore => 35,
        }
    }

    fn is_healing(&self) -> bool {
        matches!(self, DamageType::Healing | DamageType::ManaRestore)
    }

    fn is_drain(&self) -> bool {
        matches!(self, DamageType::LifeDrain | DamageType::ManaDrain)
    }
}

/// Origin of damage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamageOrigin {
    None,
    Condition,
    Spell,
    Melee,
    Ranged,
    Reflection,
    Field,
    Script,
}

/// Block type when damage is blocked
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockType {
    None,
    Defense,
    Armor,
    Immunity,
}

/// Complete damage information
#[derive(Debug, Clone)]
pub struct DamageInfo {
    /// Primary damage type
    pub damage_type: DamageType,
    /// Amount of damage (negative for damage, positive for healing)
    pub value: i32,
    /// Original value before reductions
    pub original_value: i32,
    /// Source of the damage
    pub origin: DamageOrigin,
    /// Attacker creature ID
    pub attacker_id: Option<u32>,
    /// Whether this is a critical hit
    pub critical: bool,
    /// Critical bonus percentage
    pub critical_bonus: u8,
    /// Life leech amount
    pub life_leech: i32,
    /// Mana leech amount
    pub mana_leech: i32,
    /// Block type if blocked
    pub blocked: BlockType,
    /// Amount blocked
    pub blocked_amount: i32,
    /// Condition to apply
    pub condition_type: Option<ConditionType>,
    /// Condition duration
    pub condition_duration: i32,
}

impl DamageInfo {
    pub fn new(damage_type: DamageType, value: i32) -> Self {
        Self {
            damage_type,
            value,
            original_value: value,
            origin: DamageOrigin::None,
            attacker_id: None,
            critical: false,
            critical_bonus: 0,
            life_leech: 0,
            mana_leech: 0,
            blocked: BlockType::None,
            blocked_amount: 0,
            condition_type: None,
            condition_duration: 0,
        }
    }

    pub fn melee(value: i32) -> Self {
        let mut info = Self::new(DamageType::Physical, value);
        info.origin = DamageOrigin::Melee;
        info
    }

    pub fn spell(damage_type: DamageType, value: i32) -> Self {
        let mut info = Self::new(damage_type, value);
        info.origin = DamageOrigin::Spell;
        info
    }

    pub fn healing(value: i32) -> Self {
        Self::new(DamageType::Healing, value)
    }

    pub fn with_attacker(mut self, attacker_id: u32) -> Self {
        self.attacker_id = Some(attacker_id);
        self
    }

    pub fn with_condition(mut self, condition_type: ConditionType, duration: i32) -> Self {
        self.condition_type = Some(condition_type);
        self.condition_duration = duration;
        self
    }

    /// Apply critical hit
    pub fn apply_critical(&mut self, chance: f32, bonus: u8) {
        if rand::random::<f32>() < chance {
            self.critical = true;
            self.critical_bonus = bonus;
            let critical_damage = (self.value as f32 * (1.0 + bonus as f32 / 100.0)) as i32;
            self.value = critical_damage;
        }
    }

    /// Apply life leech
    pub fn apply_life_leech(&mut self, chance: f32, amount: u8) {
        if rand::random::<f32>() < chance {
            self.life_leech = (self.value.abs() as f32 * amount as f32 / 100.0) as i32;
        }
    }

    /// Apply mana leech
    pub fn apply_mana_leech(&mut self, chance: f32, amount: u8) {
        if rand::random::<f32>() < chance {
            self.mana_leech = (self.value.abs() as f32 * amount as f32 / 100.0) as i32;
        }
    }

    /// Apply resistance
    pub fn apply_resistance(&mut self, resistance: i32) {
        if resistance != 0 {
            let reduction = self.value as f32 * (resistance as f32 / 100.0);
            self.value = (self.value as f32 - reduction) as i32;
        }
    }

    /// Apply defense blocking
    pub fn apply_defense(&mut self, defense: i32, armor: i32) {
        if defense > 0 {
            let block = rand::random::<i32>() % (defense + 1);
            if block > 0 {
                self.blocked = BlockType::Defense;
                self.blocked_amount += block;
                self.value = (self.value - block).max(0);
            }
        }

        if armor > 0 && self.value > 0 {
            let armor_reduction = armor / 2 + rand::random::<i32>() % (armor / 2 + 1);
            if armor_reduction > 0 {
                if self.blocked == BlockType::None {
                    self.blocked = BlockType::Armor;
                }
                self.blocked_amount += armor_reduction;
                self.value = (self.value - armor_reduction).max(0);
            }
        }
    }

    /// Get the actual damage dealt (absolute value for damage)
    pub fn get_effective_damage(&self) -> i32 {
        if self.damage_type.is_healing() {
            self.value
        } else {
            self.value.abs()
        }
    }

    /// Check if damage was fully blocked
    pub fn is_blocked(&self) -> bool {
        self.value == 0 && self.blocked != BlockType::None
    }
}

/// Condition types that can be applied with damage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConditionType {
    None,
    Poison,
    Fire,
    Energy,
    Bleeding,
    Cursed,
    Drown,
    Freezing,
    Dazzled,
    Paralyze,
}

impl ConditionType {
    pub fn get_damage_type(&self) -> DamageType {
        match self {
            ConditionType::Poison => DamageType::Earth,
            ConditionType::Fire => DamageType::Fire,
            ConditionType::Energy => DamageType::Energy,
            ConditionType::Bleeding => DamageType::Physical,
            ConditionType::Cursed => DamageType::Death,
            ConditionType::Drown => DamageType::Drown,
            ConditionType::Freezing => DamageType::Ice,
            ConditionType::Dazzled => DamageType::Holy,
            _ => DamageType::Physical,
        }
    }

    pub fn get_magic_effect(&self) -> u8 {
        match self {
            ConditionType::Poison => 6,
            ConditionType::Fire => 5,
            ConditionType::Energy => 11,
            ConditionType::Bleeding => 1,
            ConditionType::Cursed => 18,
            ConditionType::Drown => 30,
            ConditionType::Freezing => 43,
            ConditionType::Dazzled => 39,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_damage_info() {
        let damage = DamageInfo::melee(100);
        assert_eq!(damage.value, 100);
        assert_eq!(damage.origin, DamageOrigin::Melee);
    }

    #[test]
    fn test_resistance() {
        let mut damage = DamageInfo::new(DamageType::Fire, 100);
        damage.apply_resistance(50);
        assert_eq!(damage.value, 50);
    }
}
