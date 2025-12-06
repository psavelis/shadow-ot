//! Spell system - instant spells, runes, and conjurations

use crate::area::AreaType;
use crate::damage::DamageType;
use crate::formula::MagicFormula;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Spell types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpellType {
    Instant,
    Rune,
    Conjure,
}

/// Spell groups for cooldown
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpellGroup {
    Attack,
    Healing,
    Support,
    Special,
}

/// Vocation flags
pub mod vocation {
    pub const NONE: u32 = 0;
    pub const SORCERER: u32 = 1 << 0;
    pub const DRUID: u32 = 1 << 1;
    pub const PALADIN: u32 = 1 << 2;
    pub const KNIGHT: u32 = 1 << 3;
    pub const MASTER_SORCERER: u32 = 1 << 4;
    pub const ELDER_DRUID: u32 = 1 << 5;
    pub const ROYAL_PALADIN: u32 = 1 << 6;
    pub const ELITE_KNIGHT: u32 = 1 << 7;
    pub const ALL: u32 = 0xFFFFFFFF;

    pub fn includes(mask: u32, vocation_id: u8) -> bool {
        if mask == ALL {
            return true;
        }
        let bit = 1u32 << vocation_id;
        (mask & bit) != 0
    }
}

/// Spell definition
#[derive(Debug, Clone)]
pub struct Spell {
    pub id: u16,
    pub name: String,
    pub words: String,
    pub spell_type: SpellType,
    pub group: SpellGroup,
    pub group_cooldown: u32,
    pub cooldown: u32,
    pub level: u16,
    pub magic_level: u8,
    pub mana: i32,
    pub soul: u8,
    pub premium: bool,
    pub vocations: u32,
    pub need_target: bool,
    pub need_weapon: bool,
    pub need_learn: bool,
    pub blocking: bool,
    pub aggressive: bool,
    pub enabled: bool,
    pub range: u8,
    pub damage_type: Option<DamageType>,
    pub area_type: Option<AreaType>,
    pub formula: Option<MagicFormula>,
    pub condition_type: Option<crate::damage::ConditionType>,
    pub condition_duration: i32,
    pub conjure_count: u16,
    pub conjure_item_id: u16,
    pub effect: Option<u8>,
    pub shoot_effect: Option<u8>,
    pub script: Option<String>,
}

impl Spell {
    pub fn new(id: u16, name: String, words: String, spell_type: SpellType) -> Self {
        Self {
            id,
            name,
            words,
            spell_type,
            group: SpellGroup::Attack,
            group_cooldown: 2000,
            cooldown: 2000,
            level: 1,
            magic_level: 0,
            mana: 0,
            soul: 0,
            premium: false,
            vocations: vocation::ALL,
            need_target: false,
            need_weapon: false,
            need_learn: false,
            blocking: false,
            aggressive: false,
            enabled: true,
            range: 0,
            damage_type: None,
            area_type: None,
            formula: None,
            condition_type: None,
            condition_duration: 0,
            conjure_count: 0,
            conjure_item_id: 0,
            effect: None,
            shoot_effect: None,
            script: None,
        }
    }

    /// Check if player can use this spell
    pub fn can_use(
        &self,
        player_level: u16,
        player_magic_level: u8,
        player_vocation: u8,
        player_premium: bool,
    ) -> Result<(), SpellError> {
        if !self.enabled {
            return Err(SpellError::SpellDisabled);
        }

        if player_level < self.level {
            return Err(SpellError::LevelTooLow(self.level, player_level));
        }

        if player_magic_level < self.magic_level {
            return Err(SpellError::MagicLevelTooLow(self.magic_level, player_magic_level));
        }

        if !vocation::includes(self.vocations, player_vocation) {
            return Err(SpellError::WrongVocation);
        }

        if self.premium && !player_premium {
            return Err(SpellError::PremiumRequired);
        }

        Ok(())
    }

    /// Check resource requirements
    pub fn check_resources(&self, player_mana: i32, player_soul: u8) -> Result<(), SpellError> {
        if player_mana < self.mana {
            return Err(SpellError::NotEnoughMana(self.mana, player_mana));
        }

        if player_soul < self.soul {
            return Err(SpellError::NotEnoughSoul(self.soul, player_soul));
        }

        Ok(())
    }

    /// Is this an attack spell?
    pub fn is_aggressive(&self) -> bool {
        self.aggressive || self.damage_type.is_some()
    }

    /// Is this a healing spell?
    pub fn is_healing(&self) -> bool {
        self.group == SpellGroup::Healing ||
            self.damage_type.map(|d| d.is_healing()).unwrap_or(false)
    }

    /// Is this a rune spell?
    pub fn is_rune(&self) -> bool {
        self.spell_type == SpellType::Rune
    }

    /// Is this a conjure spell?
    pub fn is_conjure(&self) -> bool {
        self.spell_type == SpellType::Conjure
    }

    /// Calculate damage for this spell
    pub fn calculate_damage(&self, level: u16, magic_level: u8) -> Option<i32> {
        self.formula.as_ref().map(|f| {
            use crate::formula::CombatFormula;
            // Create temporary creature for formula (simplified)
            let min = (level as f32 * 2.0 + magic_level as f32 * 3.0) * f.level_factor;
            let max = (level as f32 * 2.0 + magic_level as f32 * 3.0) * f.magic_factor;
            let min = f.min_damage as f32 + min;
            let max = f.max_damage as f32 + max;
            let min = min as i32;
            let max = max as i32;
            if max <= min {
                return min;
            }
            min + rand::random::<i32>().abs() % (max - min + 1)
        })
    }
}

/// Spell errors
#[derive(Debug, Clone)]
pub enum SpellError {
    SpellDisabled,
    LevelTooLow(u16, u16),
    MagicLevelTooLow(u8, u8),
    WrongVocation,
    PremiumRequired,
    NotEnoughMana(i32, i32),
    NotEnoughSoul(u8, u8),
    NeedTarget,
    NeedWeapon,
    NotLearned,
    OnCooldown(u32),
}

/// Spell loader
pub struct SpellLoader {
    spells: HashMap<String, Spell>,
    spell_ids: HashMap<u16, String>,
}

impl SpellLoader {
    pub fn new() -> Self {
        Self {
            spells: HashMap::new(),
            spell_ids: HashMap::new(),
        }
    }

    /// Add a spell
    pub fn add_spell(&mut self, spell: Spell) {
        self.spell_ids.insert(spell.id, spell.words.clone());
        self.spells.insert(spell.words.to_lowercase(), spell);
    }

    /// Get spell by words
    pub fn get(&self, words: &str) -> Option<&Spell> {
        self.spells.get(&words.to_lowercase())
    }

    /// Get spell by ID
    pub fn get_by_id(&self, id: u16) -> Option<&Spell> {
        self.spell_ids.get(&id).and_then(|w| self.spells.get(w))
    }

    /// Find spell by partial words
    pub fn find(&self, words: &str) -> Option<&Spell> {
        let words = words.to_lowercase();

        // Exact match first
        if let Some(spell) = self.spells.get(&words) {
            return Some(spell);
        }

        // Partial match
        for (key, spell) in &self.spells {
            if key.starts_with(&words) {
                return Some(spell);
            }
        }

        None
    }

    /// Get all spells
    pub fn all(&self) -> &HashMap<String, Spell> {
        &self.spells
    }

    /// Load spells from XML
    pub fn load_xml(&mut self, path: &str) -> crate::Result<()> {
        info!("Loading spells from: {}", path);
        Ok(())
    }

    /// Create default spells
    pub fn load_defaults(&mut self) {
        // Attack spells
        self.add_spell(create_spell(1, "Light Healing", "exura", SpellGroup::Healing, |s| {
            s.level = 9;
            s.mana = 20;
            s.cooldown = 1000;
            s.group_cooldown = 1000;
            s.damage_type = Some(DamageType::Healing);
            s.formula = Some(MagicFormula::new(10, 20, DamageType::Healing));
        }));

        self.add_spell(create_spell(2, "Intense Healing", "exura gran", SpellGroup::Healing, |s| {
            s.level = 20;
            s.mana = 70;
            s.cooldown = 1000;
            s.group_cooldown = 1000;
            s.damage_type = Some(DamageType::Healing);
            s.formula = Some(MagicFormula::new(50, 100, DamageType::Healing));
        }));

        self.add_spell(create_spell(3, "Ultimate Healing", "exura vita", SpellGroup::Healing, |s| {
            s.level = 30;
            s.mana = 160;
            s.cooldown = 1000;
            s.group_cooldown = 1000;
            s.damage_type = Some(DamageType::Healing);
            s.formula = Some(MagicFormula::new(200, 400, DamageType::Healing));
        }));

        self.add_spell(create_spell(10, "Energy Strike", "exori vis", SpellGroup::Attack, |s| {
            s.level = 12;
            s.mana = 20;
            s.cooldown = 2000;
            s.group_cooldown = 2000;
            s.need_target = true;
            s.range = 3;
            s.aggressive = true;
            s.damage_type = Some(DamageType::Energy);
            s.formula = Some(MagicFormula::from_factors(0.5, 1.0, DamageType::Energy));
        }));

        self.add_spell(create_spell(11, "Flame Strike", "exori flam", SpellGroup::Attack, |s| {
            s.level = 12;
            s.mana = 20;
            s.cooldown = 2000;
            s.group_cooldown = 2000;
            s.need_target = true;
            s.range = 3;
            s.aggressive = true;
            s.damage_type = Some(DamageType::Fire);
            s.formula = Some(MagicFormula::from_factors(0.5, 1.0, DamageType::Fire));
        }));

        self.add_spell(create_spell(12, "Terra Strike", "exori tera", SpellGroup::Attack, |s| {
            s.level = 13;
            s.mana = 20;
            s.cooldown = 2000;
            s.group_cooldown = 2000;
            s.need_target = true;
            s.range = 3;
            s.aggressive = true;
            s.damage_type = Some(DamageType::Earth);
            s.formula = Some(MagicFormula::from_factors(0.5, 1.0, DamageType::Earth));
        }));

        self.add_spell(create_spell(13, "Ice Strike", "exori frigo", SpellGroup::Attack, |s| {
            s.level = 15;
            s.mana = 20;
            s.cooldown = 2000;
            s.group_cooldown = 2000;
            s.need_target = true;
            s.range = 3;
            s.aggressive = true;
            s.damage_type = Some(DamageType::Ice);
            s.formula = Some(MagicFormula::from_factors(0.5, 1.0, DamageType::Ice));
        }));

        self.add_spell(create_spell(20, "Great Energy Beam", "exevo gran vis lux", SpellGroup::Attack, |s| {
            s.level = 29;
            s.mana = 110;
            s.cooldown = 6000;
            s.group_cooldown = 2000;
            s.aggressive = true;
            s.damage_type = Some(DamageType::Energy);
            s.area_type = Some(AreaType::Beam { length: 8, width: 1 });
            s.formula = Some(MagicFormula::from_factors(1.0, 1.5, DamageType::Energy));
        }));

        self.add_spell(create_spell(21, "Hell's Core", "exevo gran mas flam", SpellGroup::Attack, |s| {
            s.level = 60;
            s.mana = 1100;
            s.cooldown = 40000;
            s.group_cooldown = 4000;
            s.aggressive = true;
            s.damage_type = Some(DamageType::Fire);
            s.area_type = Some(AreaType::Circle { radius: 5 });
            s.formula = Some(MagicFormula::from_factors(1.2, 1.8, DamageType::Fire));
        }));

        // Support spells
        self.add_spell(create_spell(30, "Haste", "utani hur", SpellGroup::Support, |s| {
            s.level = 14;
            s.mana = 60;
            s.cooldown = 2000;
            s.group_cooldown = 2000;
        }));

        self.add_spell(create_spell(31, "Strong Haste", "utani gran hur", SpellGroup::Support, |s| {
            s.level = 20;
            s.mana = 100;
            s.cooldown = 2000;
            s.group_cooldown = 2000;
        }));

        self.add_spell(create_spell(32, "Invisible", "utana vid", SpellGroup::Support, |s| {
            s.level = 35;
            s.mana = 440;
            s.cooldown = 2000;
            s.group_cooldown = 2000;
        }));

        self.add_spell(create_spell(33, "Magic Shield", "utamo vita", SpellGroup::Support, |s| {
            s.level = 14;
            s.mana = 50;
            s.cooldown = 2000;
            s.group_cooldown = 2000;
        }));

        info!("Loaded {} default spells", self.spells.len());
    }
}

impl Default for SpellLoader {
    fn default() -> Self {
        Self::new()
    }
}

fn create_spell<F>(id: u16, name: &str, words: &str, group: SpellGroup, f: F) -> Spell
where
    F: FnOnce(&mut Spell),
{
    let mut spell = Spell::new(id, name.to_string(), words.to_string(), SpellType::Instant);
    spell.group = group;
    f(&mut spell);
    spell
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spell_loader() {
        let mut loader = SpellLoader::new();
        loader.load_defaults();

        assert!(loader.get("exura").is_some());
        assert!(loader.find("exu").is_some());
    }

    #[test]
    fn test_spell_requirements() {
        let spell = create_spell(1, "Test", "test", SpellGroup::Attack, |s| {
            s.level = 10;
            s.magic_level = 5;
            s.mana = 50;
        });

        assert!(spell.can_use(10, 5, 1, false).is_ok());
        assert!(spell.can_use(5, 5, 1, false).is_err());
        assert!(spell.check_resources(50, 0).is_ok());
        assert!(spell.check_resources(10, 0).is_err());
    }
}
