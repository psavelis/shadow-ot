//! Combat conditions - damage over time, buffs, and debuffs

use crate::damage::{ConditionType, DamageType};
use serde::{Deserialize, Serialize};

/// Combat condition instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatCondition {
    pub condition_type: ConditionType,
    pub end_time: u64,
    pub interval: u32,
    pub last_tick: u64,
    pub source_id: Option<u32>,
    pub damage: Option<ConditionDamage>,
    pub speed_change: Option<i32>,
    pub regeneration: Option<Regeneration>,
}

impl CombatCondition {
    pub fn new(condition_type: ConditionType, duration_ms: u64, current_time: u64) -> Self {
        Self {
            condition_type,
            end_time: current_time + duration_ms,
            interval: 2000, // Default 2 second tick
            last_tick: current_time,
            source_id: None,
            damage: None,
            speed_change: None,
            regeneration: None,
        }
    }

    /// Create poison condition
    pub fn poison(start_damage: i32, total_rounds: u32, current_time: u64) -> Self {
        let duration = total_rounds as u64 * 2000;
        let mut condition = Self::new(ConditionType::Poison, duration, current_time);
        condition.damage = Some(ConditionDamage::decreasing(
            DamageType::Earth,
            start_damage,
            total_rounds,
        ));
        condition
    }

    /// Create fire condition (burning)
    pub fn burning(start_damage: i32, total_rounds: u32, current_time: u64) -> Self {
        let duration = total_rounds as u64 * 2000;
        let mut condition = Self::new(ConditionType::Fire, duration, current_time);
        condition.damage = Some(ConditionDamage::decreasing(
            DamageType::Fire,
            start_damage,
            total_rounds,
        ));
        condition
    }

    /// Create energy condition (electrified)
    pub fn electrified(start_damage: i32, total_rounds: u32, current_time: u64) -> Self {
        let duration = total_rounds as u64 * 2000;
        let mut condition = Self::new(ConditionType::Energy, duration, current_time);
        condition.damage = Some(ConditionDamage::decreasing(
            DamageType::Energy,
            start_damage,
            total_rounds,
        ));
        condition
    }

    /// Create bleeding condition
    pub fn bleeding(damage_per_tick: i32, duration_ms: u64, current_time: u64) -> Self {
        let mut condition = Self::new(ConditionType::Bleeding, duration_ms, current_time);
        condition.damage = Some(ConditionDamage::constant(
            DamageType::Physical,
            damage_per_tick,
        ));
        condition
    }

    /// Create cursed condition
    pub fn cursed(damage_per_tick: i32, duration_ms: u64, current_time: u64) -> Self {
        let mut condition = Self::new(ConditionType::Cursed, duration_ms, current_time);
        condition.damage = Some(ConditionDamage::constant(
            DamageType::Death,
            damage_per_tick,
        ));
        condition
    }

    /// Create drown condition
    pub fn drowning(damage_per_tick: i32, duration_ms: u64, current_time: u64) -> Self {
        let mut condition = Self::new(ConditionType::Drown, duration_ms, current_time);
        condition.damage = Some(ConditionDamage::constant(
            DamageType::Drown,
            damage_per_tick,
        ));
        condition.interval = 2000;
        condition
    }

    /// Create freezing condition
    pub fn freezing(damage_per_tick: i32, duration_ms: u64, current_time: u64) -> Self {
        let mut condition = Self::new(ConditionType::Freezing, duration_ms, current_time);
        condition.damage = Some(ConditionDamage::constant(
            DamageType::Ice,
            damage_per_tick,
        ));
        condition
    }

    /// Create paralyze condition
    pub fn paralyze(speed_reduction: i32, duration_ms: u64, current_time: u64) -> Self {
        let mut condition = Self::new(ConditionType::Paralyze, duration_ms, current_time);
        condition.speed_change = Some(-speed_reduction);
        condition
    }

    /// Check if condition is expired
    pub fn is_expired(&self, current_time: u64) -> bool {
        current_time >= self.end_time
    }

    /// Check if it's time for a tick
    pub fn needs_tick(&self, current_time: u64) -> bool {
        current_time >= self.last_tick + self.interval as u64
    }

    /// Process a tick and return damage if any
    pub fn tick(&mut self, current_time: u64) -> Option<i32> {
        if !self.needs_tick(current_time) {
            return None;
        }

        self.last_tick = current_time;

        if let Some(ref mut damage) = self.damage {
            return damage.tick();
        }

        None
    }

    /// Get remaining duration
    pub fn remaining(&self, current_time: u64) -> u64 {
        if current_time >= self.end_time {
            0
        } else {
            self.end_time - current_time
        }
    }

    /// Get damage type
    pub fn get_damage_type(&self) -> DamageType {
        self.damage
            .as_ref()
            .map(|d| d.damage_type)
            .unwrap_or(self.condition_type.get_damage_type())
    }
}

/// Condition damage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionDamage {
    pub damage_type: DamageType,
    pub start_damage: i32,
    pub current_damage: i32,
    pub total_rounds: u32,
    pub current_round: u32,
    pub decreasing: bool,
}

impl ConditionDamage {
    /// Create constant damage per tick
    pub fn constant(damage_type: DamageType, damage: i32) -> Self {
        Self {
            damage_type,
            start_damage: damage,
            current_damage: damage,
            total_rounds: u32::MAX,
            current_round: 0,
            decreasing: false,
        }
    }

    /// Create decreasing damage (like Tibia poison)
    pub fn decreasing(damage_type: DamageType, start_damage: i32, total_rounds: u32) -> Self {
        Self {
            damage_type,
            start_damage,
            current_damage: start_damage,
            total_rounds,
            current_round: 0,
            decreasing: true,
        }
    }

    /// Process a tick and return damage
    pub fn tick(&mut self) -> Option<i32> {
        if self.current_round >= self.total_rounds {
            return None;
        }

        self.current_round += 1;
        let damage = self.current_damage;

        if self.decreasing {
            // Tibia-style decreasing: damage decreases over time
            let remaining_ratio = 1.0 - (self.current_round as f32 / self.total_rounds as f32);
            self.current_damage = (self.start_damage as f32 * remaining_ratio) as i32;
            self.current_damage = self.current_damage.max(1); // Minimum 1 damage
        }

        Some(damage)
    }

    /// Get total damage that will be dealt
    pub fn total_damage(&self) -> i32 {
        if self.decreasing {
            // Sum of decreasing series
            let n = self.total_rounds as f32;
            (self.start_damage as f32 * (n + 1.0) / 2.0) as i32
        } else {
            self.start_damage * self.total_rounds as i32
        }
    }
}

/// Regeneration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Regeneration {
    pub health_gain: i32,
    pub health_ticks: u32,
    pub mana_gain: i32,
    pub mana_ticks: u32,
}

impl Default for Regeneration {
    fn default() -> Self {
        Self {
            health_gain: 1,
            health_ticks: 12000, // 12 seconds
            mana_gain: 1,
            mana_ticks: 6000, // 6 seconds
        }
    }
}

/// Condition icon for client display
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ConditionIcon {
    None = 0,
    Poison = 1,
    Fire = 2,
    Energy = 3,
    Drunk = 4,
    Mana = 5,
    Paralyze = 6,
    Haste = 7,
    Battle = 8,
    Drowning = 9,
    Freezing = 10,
    Dazzled = 11,
    Cursed = 12,
    Buff = 13,
    PzBlock = 14,
    Pz = 15,
    Bleeding = 16,
    Hungry = 17,
}

impl From<ConditionType> for ConditionIcon {
    fn from(condition: ConditionType) -> Self {
        match condition {
            ConditionType::Poison => ConditionIcon::Poison,
            ConditionType::Fire => ConditionIcon::Fire,
            ConditionType::Energy => ConditionIcon::Energy,
            ConditionType::Bleeding => ConditionIcon::Bleeding,
            ConditionType::Cursed => ConditionIcon::Cursed,
            ConditionType::Drown => ConditionIcon::Drowning,
            ConditionType::Freezing => ConditionIcon::Freezing,
            ConditionType::Dazzled => ConditionIcon::Dazzled,
            ConditionType::Paralyze => ConditionIcon::Paralyze,
            _ => ConditionIcon::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poison_condition() {
        let mut condition = CombatCondition::poison(100, 10, 0);

        assert!(!condition.is_expired(5000));
        assert_eq!(condition.tick(2000), Some(100));
        assert!(condition.damage.as_ref().unwrap().current_damage < 100);
    }

    #[test]
    fn test_decreasing_damage() {
        let mut damage = ConditionDamage::decreasing(DamageType::Earth, 100, 5);

        let d1 = damage.tick().unwrap();
        let d2 = damage.tick().unwrap();
        let d3 = damage.tick().unwrap();

        assert!(d1 >= d2);
        assert!(d2 >= d3);
    }

    #[test]
    fn test_constant_damage() {
        let mut damage = ConditionDamage::constant(DamageType::Physical, 50);

        assert_eq!(damage.tick(), Some(50));
        assert_eq!(damage.tick(), Some(50));
        assert_eq!(damage.tick(), Some(50));
    }
}
