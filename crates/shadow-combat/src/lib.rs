//! Shadow OT Combat System
//!
//! Handles all combat mechanics including damage calculation, spells,
//! conditions, and combat interactions.

pub mod damage;
pub mod formula;
pub mod spell;
pub mod condition;
pub mod combat;
pub mod area;
pub mod loot;
pub mod prey;
pub mod bosstiary;

pub use damage::{DamageInfo, DamageType, DamageTypeExt, ConditionType, DamageOrigin, BlockType};
pub use formula::{CombatFormula, MeleeFormula, MagicFormula, DistanceFormula};
pub use spell::{Spell, SpellType, SpellLoader};
pub use condition::{CombatCondition, ConditionDamage};
pub use combat::{CombatSystem, CombatEvent, CombatResult};
pub use area::{AreaEffect, AreaType};
pub use loot::{LootGenerator, LootTable, LootEntry, LootConfig, LootResult, GeneratedLoot};
pub use prey::{PreyManager, PlayerPrey, PreySlot, PreyBonusType};
pub use bosstiary::{BosstiaryManager, PlayerBosstiary, BossEntry, BossDifficulty};

use thiserror::Error;

pub type Result<T> = std::result::Result<T, CombatError>;

#[derive(Error, Debug)]
pub enum CombatError {
    #[error("Target not found: {0}")]
    TargetNotFound(u32),

    #[error("Out of range")]
    OutOfRange,

    #[error("Cannot attack target")]
    CannotAttack,

    #[error("On cooldown: {0}ms remaining")]
    OnCooldown(u64),

    #[error("Not enough mana: need {0}, have {1}")]
    NotEnoughMana(i32, i32),

    #[error("Not enough soul: need {0}, have {1}")]
    NotEnoughSoul(u8, u8),

    #[error("Spell not found: {0}")]
    SpellNotFound(String),

    #[error("Cannot use spell")]
    CannotUseSpell,

    #[error("Invalid target")]
    InvalidTarget,
}
