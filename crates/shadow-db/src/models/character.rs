//! Character model - player characters in the game

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Player character
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Character {
    pub id: Uuid,
    pub account_id: Uuid,
    pub realm_id: Uuid,
    pub name: String,
    pub vocation: Vocation,
    pub promoted: bool,
    pub sex: Sex,
    pub level: i32,
    pub experience: i64,

    // Health & Mana
    pub health: i32,
    pub health_max: i32,
    pub mana: i32,
    pub mana_max: i32,

    // Capacity
    pub capacity: i32,
    pub cap_max: i32,

    // Soul & Stamina
    pub soul: i32,
    pub stamina: i32,

    // Magic Level
    pub magic_level: i32,
    pub magic_level_exp: i64,

    // Position
    pub pos_x: i32,
    pub pos_y: i32,
    pub pos_z: i32,
    pub town_id: i32,

    // Outfit
    pub look_type: i32,
    pub look_head: i32,
    pub look_body: i32,
    pub look_legs: i32,
    pub look_feet: i32,
    pub look_addons: i32,
    pub look_mount: i32,

    // Skull & PvP
    pub skull_type: SkullType,
    pub skull_until: Option<DateTime<Utc>>,
    pub frags: i32,
    pub frag_time: Option<DateTime<Utc>>,

    // Balance
    pub balance: i64,
    pub bank_balance: i64,

    // Guild
    pub guild_id: Option<Uuid>,
    pub guild_rank_id: Option<Uuid>,
    pub guild_nick: Option<String>,

    // House
    pub house_id: Option<i32>,

    // Blessing
    pub blessings: i32,

    // Online status
    pub online: bool,
    pub last_login: Option<DateTime<Utc>>,
    pub last_logout: Option<DateTime<Utc>>,

    // Deletion
    pub deletion_date: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,

    // Statistics
    pub total_playtime: i64,
    pub login_count: i32,
    pub deaths: i32,
    pub kills_players: i32,
    pub kills_monsters: i64,

    // Prey System
    pub prey_wildcard: i32,
    pub prey_bonus_rerolls: i32,

    // Bestiary
    pub charm_points: i32,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Character skills
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CharacterSkill {
    pub character_id: Uuid,
    pub skill_type: SkillType,
    pub level: i32,
    pub tries: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "skill_type", rename_all = "lowercase")]
pub enum SkillType {
    Fist,
    Club,
    Sword,
    Axe,
    Distance,
    Shielding,
    Fishing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "vocation", rename_all = "lowercase")]
pub enum Vocation {
    None,
    Sorcerer,
    Druid,
    Paladin,
    Knight,
    MasterSorcerer,
    ElderDruid,
    RoyalPaladin,
    EliteKnight,
}

impl Vocation {
    pub fn base_vocation(&self) -> Self {
        match self {
            Self::MasterSorcerer => Self::Sorcerer,
            Self::ElderDruid => Self::Druid,
            Self::RoyalPaladin => Self::Paladin,
            Self::EliteKnight => Self::Knight,
            _ => *self,
        }
    }

    pub fn is_promoted(&self) -> bool {
        matches!(
            self,
            Self::MasterSorcerer | Self::ElderDruid | Self::RoyalPaladin | Self::EliteKnight
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "sex", rename_all = "lowercase")]
pub enum Sex {
    Female,
    Male,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "skull_type", rename_all = "lowercase")]
pub enum SkullType {
    None,
    Yellow,
    Green,
    White,
    Red,
    Black,
    Orange,
}

/// Character storage (depot, inbox)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CharacterStorage {
    pub character_id: Uuid,
    pub key: String,
    pub value: i64,
}

/// Character spells
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CharacterSpell {
    pub character_id: Uuid,
    pub spell_id: i32,
    pub learned_at: DateTime<Utc>,
}

/// Character death record
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CharacterDeath {
    pub id: Uuid,
    pub character_id: Uuid,
    pub realm_id: Uuid,
    pub level: i32,
    pub killed_by: String,
    pub is_player: bool,
    pub mostdamage_by: String,
    pub mostdamage_is_player: bool,
    pub unjustified: bool,
    pub pos_x: i32,
    pub pos_y: i32,
    pub pos_z: i32,
    pub died_at: DateTime<Utc>,
}

/// Character outfit
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CharacterOutfit {
    pub character_id: Uuid,
    pub outfit_id: i32,
    pub addons: i32,
    pub unlocked_at: DateTime<Utc>,
}

/// Character mount
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CharacterMount {
    pub character_id: Uuid,
    pub mount_id: i32,
    pub unlocked_at: DateTime<Utc>,
}

/// Character achievement
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CharacterAchievement {
    pub character_id: Uuid,
    pub achievement_id: i32,
    pub completed_at: DateTime<Utc>,
}

/// Character bestiary progress
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CharacterBestiary {
    pub character_id: Uuid,
    pub creature_id: i32,
    pub kills: i32,
    pub stage: i32,
}

/// Character prey slot
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CharacterPrey {
    pub character_id: Uuid,
    pub slot: i32,
    pub creature_id: Option<i32>,
    pub bonus_type: Option<i32>,
    pub bonus_value: Option<i32>,
    pub bonus_grade: Option<i32>,
    pub time_left: Option<i32>,
    pub free_reroll_time: Option<DateTime<Utc>>,
}

/// Create character request
#[derive(Debug, Clone, Deserialize)]
pub struct CreateCharacterRequest {
    pub name: String,
    pub vocation: Vocation,
    pub sex: Sex,
    pub realm_id: Uuid,
    pub town_id: i32,
}
