//! Combat system - main combat logic and event handling

use crate::area::{AreaEffect, AreaType};
use crate::condition::CombatCondition;
use crate::damage::{BlockType, ConditionType, DamageInfo, DamageOrigin, DamageType};
use crate::formula::{CombatFormula, MeleeFormula, DistanceFormula};
use crate::spell::{Spell, SpellLoader};
use crate::{CombatError, Result};
use shadow_world::creature::{AttackMode, Creature};
use shadow_world::item::SkillType;
use shadow_world::position::Position;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Combat event types
#[derive(Debug, Clone)]
pub enum CombatEvent {
    MeleeAttack {
        attacker_id: u32,
        target_id: u32,
        damage: DamageInfo,
    },
    RangedAttack {
        attacker_id: u32,
        target_id: u32,
        damage: DamageInfo,
        shoot_effect: Option<u8>,
    },
    SpellCast {
        caster_id: u32,
        spell_id: u16,
        target_id: Option<u32>,
        target_pos: Option<Position>,
    },
    SpellDamage {
        caster_id: u32,
        target_id: u32,
        damage: DamageInfo,
        effect: Option<u8>,
    },
    AreaDamage {
        caster_id: u32,
        center: Position,
        damages: Vec<(u32, DamageInfo)>, // (target_id, damage)
        effect: Option<u8>,
    },
    ConditionApplied {
        target_id: u32,
        condition: ConditionType,
        source_id: Option<u32>,
    },
    ConditionDamage {
        target_id: u32,
        damage: DamageInfo,
    },
    Heal {
        caster_id: u32,
        target_id: u32,
        amount: i32,
        effect: Option<u8>,
    },
    Death {
        creature_id: u32,
        killer_id: Option<u32>,
    },
    Block {
        defender_id: u32,
        attacker_id: u32,
        block_type: BlockType,
    },
}

/// Combat result
#[derive(Debug, Clone)]
pub struct CombatResult {
    pub success: bool,
    pub events: Vec<CombatEvent>,
    pub experience_gained: u64,
    pub skill_tries: HashMap<SkillType, u64>,
}

impl CombatResult {
    pub fn success(events: Vec<CombatEvent>) -> Self {
        Self {
            success: true,
            events,
            experience_gained: 0,
            skill_tries: HashMap::new(),
        }
    }

    pub fn failure() -> Self {
        Self {
            success: false,
            events: Vec::new(),
            experience_gained: 0,
            skill_tries: HashMap::new(),
        }
    }

    pub fn with_experience(mut self, exp: u64) -> Self {
        self.experience_gained = exp;
        self
    }

    pub fn with_skill_tries(mut self, skill: SkillType, tries: u64) -> Self {
        self.skill_tries.insert(skill, tries);
        self
    }
}

/// Combat system configuration
#[derive(Debug, Clone)]
pub struct CombatConfig {
    /// Enable PvP
    pub pvp_enabled: bool,
    /// Protection zone attack prevention
    pub pz_protection: bool,
    /// Level difference penalty/bonus
    pub level_difference_enabled: bool,
    /// Skill advancement rate
    pub skill_rate: f32,
    /// Magic level advancement rate
    pub magic_rate: f32,
    /// Experience rate
    pub exp_rate: f32,
    /// Loot rate
    pub loot_rate: f32,
    /// Critical hit chance bonus
    pub critical_chance_bonus: f32,
    /// Life leech chance bonus
    pub life_leech_bonus: f32,
    /// Mana leech chance bonus
    pub mana_leech_bonus: f32,
}

impl Default for CombatConfig {
    fn default() -> Self {
        Self {
            pvp_enabled: true,
            pz_protection: true,
            level_difference_enabled: true,
            skill_rate: 1.0,
            magic_rate: 1.0,
            exp_rate: 1.0,
            loot_rate: 1.0,
            critical_chance_bonus: 0.0,
            life_leech_bonus: 0.0,
            mana_leech_bonus: 0.0,
        }
    }
}

/// Combat system
pub struct CombatSystem {
    config: CombatConfig,
    spell_loader: Arc<RwLock<SpellLoader>>,
    cooldowns: HashMap<u32, HashMap<u16, u64>>, // creature_id -> spell_id -> end_time
    group_cooldowns: HashMap<u32, HashMap<u8, u64>>, // creature_id -> group -> end_time
}

impl CombatSystem {
    pub fn new(config: CombatConfig, spell_loader: Arc<RwLock<SpellLoader>>) -> Self {
        Self {
            config,
            spell_loader,
            cooldowns: HashMap::new(),
            group_cooldowns: HashMap::new(),
        }
    }

    /// Process melee attack
    pub async fn melee_attack(
        &mut self,
        attacker: &mut Creature,
        target: &mut Creature,
        current_time: u64,
    ) -> Result<CombatResult> {
        // Check if can attack
        self.validate_attack(attacker, target)?;

        // Get weapon stats (simplified - would come from equipment)
        let attack = 50; // Default fist attack
        let skill = attacker.get_skill(SkillType::Fist);

        // Create formula based on attack mode
        let attack_factor = match attacker.combat.attack_mode {
            AttackMode::Offensive => 1.2,
            AttackMode::Balanced => 1.0,
            AttackMode::Defensive => 0.8,
        };

        let formula = MeleeFormula::new(attack).with_attack_factor(attack_factor);

        // Calculate damage
        let base_damage = formula.calculate_damage(attacker, attacker.stats.level, skill);
        let mut damage = DamageInfo::melee(base_damage).with_attacker(attacker.id);

        // Apply special abilities
        self.apply_combat_abilities(&mut damage, attacker);

        // Apply target defense
        let defense = target.get_skill(SkillType::Shielding) as i32;
        let armor = 0; // Would come from equipment
        damage.apply_defense(defense, armor);

        // Apply resistance
        if let Some(&resistance) = target.resistances.get(&DamageType::Physical) {
            damage.apply_resistance(resistance);
        }

        // Apply damage
        let mut events = Vec::new();

        if damage.is_blocked() {
            events.push(CombatEvent::Block {
                defender_id: target.id,
                attacker_id: attacker.id,
                block_type: damage.blocked,
            });
        } else {
            let actual_damage = target.apply_damage(damage.value, damage.damage_type);

            events.push(CombatEvent::MeleeAttack {
                attacker_id: attacker.id,
                target_id: target.id,
                damage: damage.clone(),
            });

            // Apply life leech
            if damage.life_leech > 0 {
                attacker.heal(damage.life_leech);
            }

            // Apply mana leech
            if damage.mana_leech > 0 {
                attacker.restore_mana(damage.mana_leech);
            }

            // Check for death
            if !target.is_alive() {
                events.push(CombatEvent::Death {
                    creature_id: target.id,
                    killer_id: Some(attacker.id),
                });
            }
        }

        // Create result with skill advancement
        let mut result = CombatResult::success(events);
        result = result.with_skill_tries(SkillType::Fist, 1);

        // Add shielding skill tries for defender
        // (would be handled separately)

        Ok(result)
    }

    /// Process ranged attack
    pub async fn ranged_attack(
        &mut self,
        attacker: &mut Creature,
        target: &mut Creature,
        weapon_attack: i32,
        ammo_attack: i32,
        hit_chance: i32,
        current_time: u64,
    ) -> Result<CombatResult> {
        // Check if can attack
        self.validate_attack(attacker, target)?;

        // Check range
        let distance = attacker.position.distance_to(&target.position);
        if distance > 7 {
            return Err(CombatError::OutOfRange);
        }

        // Calculate hit chance
        let skill = attacker.get_skill(SkillType::Distance);
        let actual_hit_chance = crate::formula::calculate_hit_chance(skill, hit_chance, distance);

        // Check if hit
        if rand::random::<f32>() > actual_hit_chance {
            // Miss
            return Ok(CombatResult::success(vec![]));
        }

        // Calculate damage
        let formula = DistanceFormula::new(weapon_attack, ammo_attack, hit_chance);
        let base_damage = formula.calculate_damage(attacker, attacker.stats.level, skill);
        let mut damage = DamageInfo::new(DamageType::Physical, base_damage)
            .with_attacker(attacker.id);
        damage.origin = DamageOrigin::Ranged;

        // Apply abilities
        self.apply_combat_abilities(&mut damage, attacker);

        // Apply defense
        let defense = target.get_skill(SkillType::Shielding) as i32;
        damage.apply_defense(defense, 0);

        // Apply resistance
        if let Some(&resistance) = target.resistances.get(&DamageType::Physical) {
            damage.apply_resistance(resistance);
        }

        // Apply damage
        let mut events = Vec::new();
        target.apply_damage(damage.value, damage.damage_type);

        events.push(CombatEvent::RangedAttack {
            attacker_id: attacker.id,
            target_id: target.id,
            damage: damage.clone(),
            shoot_effect: Some(1), // Arrow
        });

        // Check for death
        if !target.is_alive() {
            events.push(CombatEvent::Death {
                creature_id: target.id,
                killer_id: Some(attacker.id),
            });
        }

        let mut result = CombatResult::success(events);
        result = result.with_skill_tries(SkillType::Distance, 1);

        Ok(result)
    }

    /// Cast a spell
    pub async fn cast_spell(
        &mut self,
        caster: &mut Creature,
        spell_words: &str,
        target: Option<&mut Creature>,
        target_pos: Option<Position>,
        current_time: u64,
    ) -> Result<CombatResult> {
        let spell_loader = self.spell_loader.read().await;
        let spell = spell_loader
            .find(spell_words)
            .ok_or_else(|| CombatError::SpellNotFound(spell_words.to_string()))?
            .clone();
        drop(spell_loader);

        // Check requirements
        let vocation = 1; // Would come from player data
        let premium = true;
        spell.can_use(caster.stats.level, caster.stats.magic_level, vocation, premium)
            .map_err(|_| CombatError::CannotUseSpell)?;

        // Check resources
        spell.check_resources(caster.stats.mana, caster.stats.soul)
            .map_err(|_| CombatError::NotEnoughMana(spell.mana, caster.stats.mana))?;

        // Check cooldown
        if let Some(cooldowns) = self.cooldowns.get(&caster.id) {
            if let Some(&end_time) = cooldowns.get(&spell.id) {
                if current_time < end_time {
                    return Err(CombatError::OnCooldown(end_time - current_time));
                }
            }
        }

        // Check target requirement
        if spell.need_target && target.is_none() {
            return Err(CombatError::InvalidTarget);
        }

        // Consume resources
        caster.stats.mana -= spell.mana;
        if spell.soul > 0 {
            caster.stats.soul -= spell.soul;
        }

        // Set cooldown
        self.cooldowns
            .entry(caster.id)
            .or_insert_with(HashMap::new)
            .insert(spell.id, current_time + spell.cooldown as u64);

        // Process spell effect
        let mut events = Vec::new();

        events.push(CombatEvent::SpellCast {
            caster_id: caster.id,
            spell_id: spell.id,
            target_id: target.as_ref().map(|t| t.id),
            target_pos,
        });

        // Handle different spell types
        if spell.is_healing() {
            // Healing spell
            let heal_target = target.unwrap_or(caster);
            if let Some(heal_amount) = spell.calculate_damage(caster.stats.level, caster.stats.magic_level) {
                let actual_heal = heal_target.heal(heal_amount.abs());
                events.push(CombatEvent::Heal {
                    caster_id: caster.id,
                    target_id: heal_target.id,
                    amount: actual_heal,
                    effect: spell.effect,
                });
            }
        } else if spell.is_aggressive() {
            // Damage spell
            if let Some(target) = target {
                if let Some(damage_type) = spell.damage_type {
                    if let Some(damage_value) = spell.calculate_damage(caster.stats.level, caster.stats.magic_level) {
                        let mut damage = DamageInfo::spell(damage_type, damage_value.abs())
                            .with_attacker(caster.id);

                        // Apply resistance
                        if let Some(&resistance) = target.resistances.get(&damage_type) {
                            damage.apply_resistance(resistance);
                        }

                        target.apply_damage(damage.value, damage.damage_type);

                        events.push(CombatEvent::SpellDamage {
                            caster_id: caster.id,
                            target_id: target.id,
                            damage,
                            effect: spell.effect,
                        });

                        // Apply condition if any
                        if let Some(condition_type) = spell.condition_type {
                            events.push(CombatEvent::ConditionApplied {
                                target_id: target.id,
                                condition: condition_type,
                                source_id: Some(caster.id),
                            });
                        }

                        // Check for death
                        if !target.is_alive() {
                            events.push(CombatEvent::Death {
                                creature_id: target.id,
                                killer_id: Some(caster.id),
                            });
                        }
                    }
                }
            }
        }

        Ok(CombatResult::success(events))
    }

    /// Apply area damage
    pub async fn apply_area_damage(
        &mut self,
        caster: &mut Creature,
        area: AreaEffect,
        damage_type: DamageType,
        base_damage: i32,
        targets: &mut [&mut Creature],
    ) -> Result<CombatResult> {
        let mut events = Vec::new();
        let mut area_damages = Vec::new();

        for target in targets {
            if !area.contains(&target.position) {
                continue;
            }

            // Get damage percentage for this position
            let damage_pct = area.get_damage_percent(&target.position);
            let actual_damage = (base_damage as f32 * damage_pct as f32 / 100.0) as i32;

            let mut damage = DamageInfo::spell(damage_type, actual_damage)
                .with_attacker(caster.id);

            // Apply resistance
            if let Some(&resistance) = target.resistances.get(&damage_type) {
                damage.apply_resistance(resistance);
            }

            target.apply_damage(damage.value, damage.damage_type);
            area_damages.push((target.id, damage));

            // Check for death
            if !target.is_alive() {
                events.push(CombatEvent::Death {
                    creature_id: target.id,
                    killer_id: Some(caster.id),
                });
            }
        }

        if !area_damages.is_empty() {
            events.insert(
                0,
                CombatEvent::AreaDamage {
                    caster_id: caster.id,
                    center: area.center,
                    damages: area_damages,
                    effect: Some(damage_type.get_magic_effect()),
                },
            );
        }

        Ok(CombatResult::success(events))
    }

    /// Apply combat abilities (critical, life leech, mana leech)
    fn apply_combat_abilities(&self, damage: &mut DamageInfo, attacker: &Creature) {
        // Critical hit (example: 10% chance, 50% bonus)
        let crit_chance = 0.10 + self.config.critical_chance_bonus;
        damage.apply_critical(crit_chance, 50);

        // Life leech (example: 10% chance, 10% amount)
        let life_leech_chance = 0.10 + self.config.life_leech_bonus;
        damage.apply_life_leech(life_leech_chance, 10);

        // Mana leech (example: 10% chance, 10% amount)
        let mana_leech_chance = 0.10 + self.config.mana_leech_bonus;
        damage.apply_mana_leech(mana_leech_chance, 10);
    }

    /// Validate if attacker can attack target
    fn validate_attack(&self, attacker: &Creature, target: &Creature) -> Result<()> {
        // Check if target is alive
        if !target.is_alive() {
            return Err(CombatError::TargetNotFound(target.id));
        }

        // Check if in range (melee = adjacent)
        if !attacker.position.is_adjacent(&target.position) {
            return Err(CombatError::OutOfRange);
        }

        // Check PvP rules
        if attacker.is_player() && target.is_player() && !self.config.pvp_enabled {
            return Err(CombatError::CannotAttack);
        }

        Ok(())
    }

    /// Process condition tick damage
    pub fn process_condition_damage(
        target: &mut Creature,
        condition: &mut CombatCondition,
        current_time: u64,
    ) -> Option<CombatEvent> {
        if let Some(damage_value) = condition.tick(current_time) {
            let damage = DamageInfo::new(condition.get_damage_type(), damage_value);
            target.apply_damage(damage.value, damage.damage_type);

            return Some(CombatEvent::ConditionDamage {
                target_id: target.id,
                damage,
            });
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shadow_world::creature::CreatureType;

    fn create_test_creature(name: &str) -> Creature {
        let mut creature = Creature::new(
            name.to_string(),
            CreatureType::Player,
            Position::new(100, 100, 7),
        );
        creature.stats.health = 100;
        creature.stats.max_health = 100;
        creature.stats.mana = 100;
        creature.stats.max_mana = 100;
        creature.stats.level = 100;
        creature.stats.magic_level = 50;
        creature
    }

    #[tokio::test]
    async fn test_melee_attack() {
        let mut spell_loader = SpellLoader::new();
        spell_loader.load_defaults();

        let config = CombatConfig::default();
        let mut combat = CombatSystem::new(config, Arc::new(RwLock::new(spell_loader)));

        let mut attacker = create_test_creature("Attacker");
        let mut target = create_test_creature("Target");
        target.position = Position::new(101, 100, 7); // Adjacent

        let result = combat.melee_attack(&mut attacker, &mut target, 0).await;
        assert!(result.is_ok());
    }
}
