//! Item system - game items and their properties

use crate::position::Position;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

static ITEM_UNIQUE_ID: AtomicU32 = AtomicU32::new(1);

fn next_unique_id() -> u32 {
    ITEM_UNIQUE_ID.fetch_add(1, Ordering::SeqCst)
}

/// Item instance in the game world
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub unique_id: u32,
    pub item_type_id: u16,
    pub count: u16,
    pub action_id: u16,
    pub unique_action_id: u16,
    pub text: Option<String>,
    pub written_by: Option<String>,
    pub written_date: Option<i64>,
    pub charges: Option<u16>,
    pub duration: Option<u32>,
    pub decay_state: DecayState,
    pub attributes: HashMap<String, ItemAttribute>,
}

impl Item {
    pub fn new(item_type_id: u16) -> Self {
        Self {
            unique_id: next_unique_id(),
            item_type_id,
            count: 1,
            action_id: 0,
            unique_action_id: 0,
            text: None,
            written_by: None,
            written_date: None,
            charges: None,
            duration: None,
            decay_state: DecayState::None,
            attributes: HashMap::new(),
        }
    }

    pub fn with_count(item_type_id: u16, count: u16) -> Self {
        let mut item = Self::new(item_type_id);
        item.count = count;
        item
    }

    /// Get the item type from the loader
    pub fn get_type(&self) -> Option<&'static ItemType> {
        ITEM_TYPES.get(&self.item_type_id)
    }

    /// Check if item blocks solid
    pub fn blocks_solid(&self) -> bool {
        self.get_type().map(|t| t.flags.blocks_solid()).unwrap_or(false)
    }

    /// Check if item blocks projectile
    pub fn blocks_projectile(&self) -> bool {
        self.get_type().map(|t| t.flags.blocks_projectile()).unwrap_or(false)
    }

    /// Check if item blocks pathfind
    pub fn blocks_pathfind(&self) -> bool {
        self.get_type().map(|t| t.flags.blocks_pathfind()).unwrap_or(false)
    }

    /// Check if item is always on top
    pub fn is_always_on_top(&self) -> bool {
        self.get_type().map(|t| t.flags.always_on_top()).unwrap_or(false)
    }

    /// Check if item is stackable
    pub fn is_stackable(&self) -> bool {
        self.get_type().map(|t| t.flags.stackable()).unwrap_or(false)
    }

    /// Check if item is a container
    pub fn is_container(&self) -> bool {
        self.get_type().map(|t| t.container_size.is_some()).unwrap_or(false)
    }

    /// Check if item is a magic field
    pub fn is_magic_field(&self) -> bool {
        self.get_type().map(|t| t.flags.is_magic_field()).unwrap_or(false)
    }

    /// Check if item is movable
    pub fn is_movable(&self) -> bool {
        self.get_type().map(|t| t.flags.movable()).unwrap_or(true)
    }

    /// Check if item is pickupable
    pub fn is_pickupable(&self) -> bool {
        self.get_type().map(|t| t.flags.pickupable()).unwrap_or(false)
    }

    /// Check if item is usable
    pub fn is_usable(&self) -> bool {
        self.get_type().map(|t| t.flags.usable()).unwrap_or(false)
    }

    /// Get item weight
    pub fn get_weight(&self) -> u32 {
        self.get_type().map(|t| t.weight * self.count as u32).unwrap_or(0)
    }

    /// Get item name
    pub fn get_name(&self) -> &str {
        self.get_type().map(|t| t.name.as_str()).unwrap_or("unknown item")
    }

    /// Get item description
    pub fn get_description(&self) -> String {
        if let Some(item_type) = self.get_type() {
            let mut desc = format!("You see {}.", item_type.name);
            if let Some(ref article) = item_type.article {
                desc = format!("You see {} {}.", article, item_type.name);
            }
            if self.count > 1 && item_type.flags.stackable() {
                desc = format!("You see {} {}.", self.count, item_type.plural.as_ref().unwrap_or(&item_type.name));
            }
            if let Some(ref item_desc) = item_type.description {
                desc.push_str(&format!("\n{}", item_desc));
            }
            if let Some(weight) = Some(self.get_weight()) {
                desc.push_str(&format!("\nIt weighs {:.2} oz.", weight as f32 / 100.0));
            }
            desc
        } else {
            "You see an item of unknown type.".to_string()
        }
    }

    /// Set attribute
    pub fn set_attribute(&mut self, key: &str, value: ItemAttribute) {
        self.attributes.insert(key.to_string(), value);
    }

    /// Get attribute
    pub fn get_attribute(&self, key: &str) -> Option<&ItemAttribute> {
        self.attributes.get(key)
    }

    /// Check if item can be stacked with another
    pub fn can_stack_with(&self, other: &Item) -> bool {
        self.item_type_id == other.item_type_id
            && self.is_stackable()
            && self.count + other.count <= 100
    }
}

/// Item attribute value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemAttribute {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

/// Item decay state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecayState {
    None,
    Started,
    Paused,
}

/// Item type definition (loaded from items.otb/items.xml)
#[derive(Debug, Clone)]
pub struct ItemType {
    pub id: u16,
    pub client_id: u16,
    pub name: String,
    pub article: Option<String>,
    pub plural: Option<String>,
    pub description: Option<String>,
    pub group: ItemGroup,
    pub flags: ItemFlags,
    pub weight: u32,
    pub slot_type: Option<SlotType>,
    pub weapon_type: Option<WeaponType>,
    pub ammo_type: Option<AmmoType>,
    pub attack: Option<i32>,
    pub defense: Option<i32>,
    pub extra_defense: Option<i32>,
    pub armor: Option<i32>,
    pub attack_speed: Option<u32>,
    pub hit_chance: Option<i32>,
    pub shoot_range: Option<u8>,
    pub shoot_type: Option<ShootType>,
    pub magic_effect: Option<MagicEffect>,
    pub container_size: Option<u8>,
    pub max_text_length: Option<u16>,
    pub max_write_once_length: Option<u16>,
    pub light_level: Option<u8>,
    pub light_color: Option<u8>,
    pub level_requirement: Option<u16>,
    pub vocation_mask: Option<u32>,
    pub speed_boost: Option<i32>,
    pub decay_to: Option<u16>,
    pub decay_time: Option<u32>,
    pub transform_equip: Option<u16>,
    pub transform_deequip: Option<u16>,
    pub charges: Option<u16>,
    pub show_charges: bool,
    pub show_duration: bool,
    pub break_chance: Option<u8>,
    pub worth: Option<u64>,
    pub supply: bool,
    pub absorb: HashMap<DamageType, i32>,
    pub field_absorb: HashMap<DamageType, i32>,
    pub skill_boost: HashMap<SkillType, i32>,
    pub special_skill: HashMap<SpecialSkill, i32>,
    pub abilities: ItemAbilities,
}

impl ItemType {
    pub fn new(id: u16) -> Self {
        Self {
            id,
            client_id: id,
            name: String::new(),
            article: None,
            plural: None,
            description: None,
            group: ItemGroup::None,
            flags: ItemFlags::default(),
            weight: 0,
            slot_type: None,
            weapon_type: None,
            ammo_type: None,
            attack: None,
            defense: None,
            extra_defense: None,
            armor: None,
            attack_speed: None,
            hit_chance: None,
            shoot_range: None,
            shoot_type: None,
            magic_effect: None,
            container_size: None,
            max_text_length: None,
            max_write_once_length: None,
            light_level: None,
            light_color: None,
            level_requirement: None,
            vocation_mask: None,
            speed_boost: None,
            decay_to: None,
            decay_time: None,
            transform_equip: None,
            transform_deequip: None,
            charges: None,
            show_charges: false,
            show_duration: false,
            break_chance: None,
            worth: None,
            supply: false,
            absorb: HashMap::new(),
            field_absorb: HashMap::new(),
            skill_boost: HashMap::new(),
            special_skill: HashMap::new(),
            abilities: ItemAbilities::default(),
        }
    }
}

/// Item flags
#[derive(Debug, Clone, Copy, Default)]
pub struct ItemFlags(u64);

impl ItemFlags {
    const BLOCK_SOLID: u64 = 1 << 0;
    const BLOCK_PROJECTILE: u64 = 1 << 1;
    const BLOCK_PATHFIND: u64 = 1 << 2;
    const HAS_HEIGHT: u64 = 1 << 3;
    const USABLE: u64 = 1 << 4;
    const PICKUPABLE: u64 = 1 << 5;
    const MOVABLE: u64 = 1 << 6;
    const STACKABLE: u64 = 1 << 7;
    const ALWAYS_ON_TOP: u64 = 1 << 8;
    const READABLE: u64 = 1 << 9;
    const ROTATABLE: u64 = 1 << 10;
    const HANGABLE: u64 = 1 << 11;
    const VERTICAL: u64 = 1 << 12;
    const HORIZONTAL: u64 = 1 << 13;
    const ALLOW_DIST_READ: u64 = 1 << 14;
    const CORPSE: u64 = 1 << 15;
    const MAGIC_FIELD: u64 = 1 << 16;
    const SPLASH: u64 = 1 << 17;
    const FLUID_CONTAINER: u64 = 1 << 18;
    const CONTAINER: u64 = 1 << 19;

    pub fn new() -> Self { Self(0) }
    pub fn set(&mut self, flag: u64) { self.0 |= flag; }
    pub fn has(&self, flag: u64) -> bool { (self.0 & flag) != 0 }

    pub fn blocks_solid(&self) -> bool { self.has(Self::BLOCK_SOLID) }
    pub fn blocks_projectile(&self) -> bool { self.has(Self::BLOCK_PROJECTILE) }
    pub fn blocks_pathfind(&self) -> bool { self.has(Self::BLOCK_PATHFIND) }
    pub fn usable(&self) -> bool { self.has(Self::USABLE) }
    pub fn pickupable(&self) -> bool { self.has(Self::PICKUPABLE) }
    pub fn movable(&self) -> bool { self.has(Self::MOVABLE) }
    pub fn stackable(&self) -> bool { self.has(Self::STACKABLE) }
    pub fn always_on_top(&self) -> bool { self.has(Self::ALWAYS_ON_TOP) }
    pub fn is_magic_field(&self) -> bool { self.has(Self::MAGIC_FIELD) }
    pub fn is_container(&self) -> bool { self.has(Self::CONTAINER) }
}

/// Item groups
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemGroup {
    None,
    Ground,
    Container,
    Weapon,
    Ammunition,
    Armor,
    Charges,
    Teleport,
    MagicField,
    Writeable,
    Key,
    Splash,
    Fluid,
    Door,
    Deprecated,
}

/// Equipment slot types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlotType {
    Head,
    Necklace,
    Backpack,
    Armor,
    Right,
    Left,
    Legs,
    Feet,
    Ring,
    Ammo,
    TwoHanded,
}

/// Weapon types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeaponType {
    Sword,
    Club,
    Axe,
    Distance,
    Wand,
    Shield,
    Ammunition,
}

/// Ammunition types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AmmoType {
    Arrow,
    Bolt,
    Spear,
    ThrowingStar,
    ThrowingKnife,
    Stone,
    Snowball,
}

/// Shoot types (projectile visuals)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShootType {
    Spear,
    Bolt,
    Arrow,
    Fire,
    Energy,
    PoisonArrow,
    BurstArrow,
    ThrowingStar,
    ThrowingKnife,
    SmallStone,
    Death,
    LargeRock,
    Snowball,
    PowerBolt,
    PoisonField,
    InfernalBolt,
    HuntingSpear,
    EnchantedSpear,
    RedStar,
    GreenStar,
    RoyalSpear,
    SniperArrow,
    OnyxArrow,
    PiercingBolt,
    WhirlwindSword,
    WhirlwindAxe,
    WhirlwindClub,
    EtherealSpear,
    Ice,
    Earth,
    Holy,
    SuddenDeath,
    FlashArrow,
    FlamingArrow,
    ShiverArrow,
    EnergyBall,
    SmallIce,
    SmallHoly,
    SmallEarth,
    EarthArrow,
    Explosion,
    Cake,
    TarsalArrow,
    VortexBolt,
    PrismaticBolt,
    CrystallineArrow,
    DrillBolt,
    EnvenomedSpear,
}

/// Magic effects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MagicEffect {
    DrawBlood,
    LoseEnergy,
    Poff,
    BlockHit,
    ExplosionArea,
    ExplosionDamage,
    FireArea,
    YellowRings,
    GreenRings,
    HitArea,
    Teleport,
    EnergyDamage,
    MagicBlue,
    MagicRed,
    MagicGreen,
    MagicYellow,
    HitByFire,
    HitByPoison,
    MortArea,
    SoundGreen,
    SoundRed,
    PoisonArea,
    SoundYellow,
    SoundPurple,
    SoundBlue,
    SoundWhite,
    // ... many more
}

/// Damage types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamageType {
    Physical,
    Energy,
    Earth,
    Fire,
    Ice,
    Holy,
    Death,
    Drown,
    LifeDrain,
    ManaDrain,
    Healing,
    ManaRestore,
}

/// Skill types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SkillType {
    Fist,
    Club,
    Sword,
    Axe,
    Distance,
    Shielding,
    Fishing,
    MagicLevel,
}

/// Special skills
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpecialSkill {
    CriticalHitChance,
    CriticalHitDamage,
    LifeLeechChance,
    LifeLeechAmount,
    ManaLeechChance,
    ManaLeechAmount,
}

/// Item abilities from imbuements and enchantments
#[derive(Debug, Clone, Default)]
pub struct ItemAbilities {
    pub health_gain: i32,
    pub health_ticks: u32,
    pub mana_gain: i32,
    pub mana_ticks: u32,
    pub regeneration: bool,
    pub invisible: bool,
    pub mana_shield: bool,
    pub speed: i32,
    pub skills: HashMap<SkillType, i32>,
    pub stats: HashMap<String, i32>,
}

/// Item loader from OTB/XML files
pub struct ItemLoader {
    items: HashMap<u16, ItemType>,
}

impl ItemLoader {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    /// Load items from OTB file
    pub fn load_otb(&mut self, path: &str) -> crate::Result<()> {
        // OTB parsing implementation
        tracing::info!("Loading items from OTB: {}", path);
        Ok(())
    }

    /// Load item attributes from XML
    pub fn load_xml(&mut self, path: &str) -> crate::Result<()> {
        // XML parsing implementation
        tracing::info!("Loading item attributes from XML: {}", path);
        Ok(())
    }

    /// Get item type by ID
    pub fn get(&self, id: u16) -> Option<&ItemType> {
        self.items.get(&id)
    }

    /// Get all item types
    pub fn all(&self) -> &HashMap<u16, ItemType> {
        &self.items
    }
}

// Global item types registry (populated at server start)
lazy_static::lazy_static! {
    pub static ref ITEM_TYPES: HashMap<u16, ItemType> = HashMap::new();
}
