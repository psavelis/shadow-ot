//! Item models - inventory, depot, containers

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Item instance in the game
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Item {
    pub id: Uuid,
    pub item_type_id: i32,
    pub count: i32,
    pub owner_id: Option<Uuid>,
    pub location_type: ItemLocationType,
    pub container_id: Option<Uuid>,
    pub slot: Option<i32>,
    pub attributes: serde_json::Value, // Custom attributes JSON
    pub is_nft: bool,
    pub nft_token_id: Option<String>,
    pub nft_chain: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "item_location_type", rename_all = "snake_case")]
pub enum ItemLocationType {
    Inventory,
    Depot,
    Inbox,
    Container,
    Ground,
    Trade,
    Market,
    Stash,
    House,
}

/// Item type definition (from data files)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ItemType {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub group: ItemGroup,
    pub flags: i64, // Bitmask of item flags
    pub weight: i32, // In oz
    pub armor: Option<i32>,
    pub defense: Option<i32>,
    pub attack: Option<i32>,
    pub attack_speed: Option<i32>,
    pub range: Option<i32>,
    pub container_size: Option<i32>,
    pub slot_type: Option<ItemSlot>,
    pub weapon_type: Option<WeaponType>,
    pub ammo_type: Option<AmmoType>,
    pub shoot_type: Option<ShootType>,
    pub effect: Option<MagicEffect>,
    pub level_requirement: Option<i32>,
    pub vocation_requirement: Option<String>,
    pub decay_to: Option<i32>,
    pub decay_time: Option<i32>,
    pub write_once_id: Option<i32>,
    pub max_text_len: Option<i32>,
    pub speed_boost: Option<i32>,
    pub article: Option<String>,
    pub plural: Option<String>,
    pub sprite_id: i32,
    pub classification: i32,
    pub tier: Option<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "item_group", rename_all = "lowercase")]
pub enum ItemGroup {
    None,
    Ground,
    Container,
    Weapon,
    Ammunition,
    Armor,
    Amulet,
    Ring,
    Legs,
    Boots,
    Shield,
    Splash,
    Fluid,
    Door,
    Bed,
    Rune,
    Key,
    Food,
    Money,
    Teleport,
    MagicField,
    Writable,
    WritableOnce,
    ShowOffSocket,
    Podium,
    Light,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "item_slot", rename_all = "lowercase")]
pub enum ItemSlot {
    Head,
    Necklace,
    Backpack,
    Armor,
    RightHand,
    LeftHand,
    Legs,
    Feet,
    Ring,
    Ammo,
    Store,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "weapon_type", rename_all = "lowercase")]
pub enum WeaponType {
    Sword,
    Club,
    Axe,
    Distance,
    Wand,
    Rod,
    Fist,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ammo_type", rename_all = "lowercase")]
pub enum AmmoType {
    Arrow,
    Bolt,
    Spear,
    ThrowingStar,
    ThrowingKnife,
    Stone,
    Snowball,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "shoot_type", rename_all = "lowercase")]
pub enum ShootType {
    Arrow,
    Bolt,
    Fire,
    Ice,
    Earth,
    Energy,
    Death,
    Holy,
    Physical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "magic_effect", rename_all = "lowercase")]
pub enum MagicEffect {
    None,
    DrawBlood,
    LoseEnergy,
    Poof,
    BlockHit,
    ExplosionArea,
    ExplosionDamage,
    FireArea,
    YellowRings,
    GreenRings,
    HitArea,
    Teleport,
    EnergyDamage,
    MagicGreen,
    MagicRed,
    MagicBlue,
    MagicPurple,
    MagicYellow,
    BubbleBlue,
    BubblePurple,
    Assassin,
    // ... many more effects
}

/// Character inventory slot
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CharacterInventory {
    pub character_id: Uuid,
    pub slot: i32,
    pub item_id: Uuid,
}

/// Character depot
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CharacterDepot {
    pub character_id: Uuid,
    pub town_id: i32,
    pub item_id: Uuid,
}

/// Container contents
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ContainerItem {
    pub container_id: Uuid,
    pub slot: i32,
    pub item_id: Uuid,
}

/// Item flags (bitmask values)
pub mod item_flags {
    pub const STACKABLE: i64 = 1 << 0;
    pub const PICKUP: i64 = 1 << 1;
    pub const USABLE: i64 = 1 << 2;
    pub const MOVABLE: i64 = 1 << 3;
    pub const BLOCKSOLID: i64 = 1 << 4;
    pub const BLOCKPROJECTILE: i64 = 1 << 5;
    pub const BLOCKPATHFIND: i64 = 1 << 6;
    pub const HASHEIGHT: i64 = 1 << 7;
    pub const LOOKTHROUGH: i64 = 1 << 8;
    pub const HANGABLE: i64 = 1 << 9;
    pub const HORIZONTAL: i64 = 1 << 10;
    pub const VERTICAL: i64 = 1 << 11;
    pub const ROTATABLE: i64 = 1 << 12;
    pub const READABLE: i64 = 1 << 13;
    pub const WRITEABLE: i64 = 1 << 14;
    pub const CORPSE: i64 = 1 << 15;
    pub const SPLASH: i64 = 1 << 16;
    pub const FLUID_CONTAINER: i64 = 1 << 17;
    pub const ALLOWDISTREAD: i64 = 1 << 18;
    pub const ALWAYS_ON_TOP: i64 = 1 << 19;
    pub const SHOWOFFSET: i64 = 1 << 20;
    pub const TOPEFFECT: i64 = 1 << 21;
    pub const ANIMATION: i64 = 1 << 22;
    pub const WEAROUT: i64 = 1 << 23;
    pub const CLOCKEXPIRE: i64 = 1 << 24;
    pub const EXPIRE: i64 = 1 << 25;
    pub const EXPIRESTOP: i64 = 1 << 26;
    pub const MULTIUSE: i64 = 1 << 27;
    pub const CHARGES: i64 = 1 << 28;
    pub const SHOWCHARGES: i64 = 1 << 29;
    pub const SHOWATTRIBUTES: i64 = 1 << 30;
    pub const DISGUISE: i64 = 1 << 31;
}
