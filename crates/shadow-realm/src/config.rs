//! Realm Configuration
//!
//! Configurable options for each realm.

use serde::{Deserialize, Serialize};

use crate::RealmType;

/// Complete realm configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealmConfig {
    /// Realm name
    pub name: String,
    /// Realm type
    pub realm_type: RealmType,
    /// Maximum players
    pub max_players: u32,
    /// Combat settings
    pub combat: CombatConfig,
    /// Economy settings
    pub economy: EconomyConfig,
    /// Experience settings
    pub experience: ExperienceConfig,
    /// PvP settings
    pub pvp: PvPConfig,
    /// Death settings
    pub death: DeathConfig,
    /// World settings
    pub world: WorldConfig,
    /// Custom features
    pub features: FeaturesConfig,
}

impl Default for RealmConfig {
    fn default() -> Self {
        Self {
            name: "Default Realm".to_string(),
            realm_type: RealmType::PvE,
            max_players: 500,
            combat: CombatConfig::default(),
            economy: EconomyConfig::default(),
            experience: ExperienceConfig::default(),
            pvp: PvPConfig::default(),
            death: DeathConfig::default(),
            world: WorldConfig::default(),
            features: FeaturesConfig::default(),
        }
    }
}

/// Combat configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatConfig {
    /// Base damage multiplier
    pub damage_multiplier: f64,
    /// Healing multiplier
    pub healing_multiplier: f64,
    /// Spell cooldown multiplier
    pub cooldown_multiplier: f64,
    /// Area spell damage multiplier
    pub aoe_damage_multiplier: f64,
    /// Critical hit chance bonus
    pub crit_chance_bonus: f64,
    /// Maximum simultaneous attackers
    pub max_attackers: u32,
    /// Combat log duration (seconds)
    pub combat_log_duration: u32,
}

impl Default for CombatConfig {
    fn default() -> Self {
        Self {
            damage_multiplier: 1.0,
            healing_multiplier: 1.0,
            cooldown_multiplier: 1.0,
            aoe_damage_multiplier: 1.0,
            crit_chance_bonus: 0.0,
            max_attackers: 10,
            combat_log_duration: 60,
        }
    }
}

/// Economy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomyConfig {
    /// Loot rate multiplier
    pub loot_rate: f64,
    /// Gold drop multiplier
    pub gold_rate: f64,
    /// NPC buy price multiplier
    pub buy_multiplier: f64,
    /// NPC sell price multiplier
    pub sell_multiplier: f64,
    /// Market tax percentage
    pub market_tax: f64,
    /// Maximum gold stack
    pub max_gold_stack: u64,
    /// Allow player trading
    pub allow_trading: bool,
    /// Minimum level to use market
    pub market_min_level: u32,
}

impl Default for EconomyConfig {
    fn default() -> Self {
        Self {
            loot_rate: 1.0,
            gold_rate: 1.0,
            buy_multiplier: 1.0,
            sell_multiplier: 1.0,
            market_tax: 2.0,
            max_gold_stack: 100_000_000_000, // 100 billion
            allow_trading: true,
            market_min_level: 20,
        }
    }
}

/// Experience configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceConfig {
    /// Experience rate multiplier
    pub exp_rate: f64,
    /// Skill training rate multiplier
    pub skill_rate: f64,
    /// Magic level rate multiplier
    pub magic_rate: f64,
    /// Level cap (0 = no cap)
    pub level_cap: u32,
    /// Skill cap
    pub skill_cap: u32,
    /// Party bonus per member
    pub party_bonus: f64,
    /// VIP experience bonus
    pub vip_bonus: f64,
    /// Stamina system enabled
    pub stamina_enabled: bool,
    /// Happy hour multiplier
    pub happy_hour_multiplier: f64,
}

impl Default for ExperienceConfig {
    fn default() -> Self {
        Self {
            exp_rate: 1.0,
            skill_rate: 1.0,
            magic_rate: 1.0,
            level_cap: 0,
            skill_cap: 125,
            party_bonus: 0.05,
            vip_bonus: 0.5,
            stamina_enabled: true,
            happy_hour_multiplier: 1.5,
        }
    }
}

/// PvP configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PvPConfig {
    /// PvP enabled
    pub enabled: bool,
    /// PvP type (open, optional, zones)
    pub pvp_type: String,
    /// Minimum level for PvP
    pub min_level: u32,
    /// Level difference for fair fight
    pub level_diff_limit: u32,
    /// Skull system enabled
    pub skulls_enabled: bool,
    /// Protection zones enabled
    pub protection_zones: bool,
    /// Frag time (hours)
    pub frag_time: u32,
    /// Frags for red skull
    pub red_skull_frags: u32,
    /// Frags for black skull
    pub black_skull_frags: u32,
    /// PvP damage reduction in safe areas
    pub safe_zone_reduction: f64,
}

impl Default for PvPConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            pvp_type: "optional".to_string(),
            min_level: 50,
            level_diff_limit: 30,
            skulls_enabled: true,
            protection_zones: true,
            frag_time: 24,
            red_skull_frags: 3,
            black_skull_frags: 10,
            safe_zone_reduction: 0.5,
        }
    }
}

/// Death configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeathConfig {
    /// Base experience loss percentage
    pub exp_loss: f64,
    /// Base skill loss percentage
    pub skill_loss: f64,
    /// Item drop enabled
    pub item_drop: bool,
    /// Container drop enabled
    pub container_drop: bool,
    /// Blessings system enabled
    pub blessings_enabled: bool,
    /// Amulet of loss enabled
    pub aol_enabled: bool,
    /// Minimum level for full penalties
    pub min_penalty_level: u32,
    /// Death penalty reduction per blessing
    pub blessing_reduction: f64,
    /// PvP death has reduced penalty
    pub pvp_reduced_penalty: bool,
}

impl Default for DeathConfig {
    fn default() -> Self {
        Self {
            exp_loss: 10.0,
            skill_loss: 10.0,
            item_drop: true,
            container_drop: true,
            blessings_enabled: true,
            aol_enabled: true,
            min_penalty_level: 20,
            blessing_reduction: 1.6,
            pvp_reduced_penalty: true,
        }
    }
}

/// World configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldConfig {
    /// Day/night cycle enabled
    pub day_night_cycle: bool,
    /// Day length in real minutes
    pub day_length_minutes: u32,
    /// Weather system enabled
    pub weather_enabled: bool,
    /// Monster respawn rate multiplier
    pub respawn_rate: f64,
    /// Boss spawn rate multiplier
    pub boss_spawn_rate: f64,
    /// Event spawn rate
    pub event_spawn_rate: f64,
    /// World save interval (minutes)
    pub save_interval: u32,
    /// Maximum NPCs per area
    pub max_npcs_per_area: u32,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            day_night_cycle: true,
            day_length_minutes: 60,
            weather_enabled: true,
            respawn_rate: 1.0,
            boss_spawn_rate: 1.0,
            event_spawn_rate: 1.0,
            save_interval: 5,
            max_npcs_per_area: 100,
        }
    }
}

/// Custom features configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturesConfig {
    /// Houses enabled
    pub houses: bool,
    /// Guilds enabled
    pub guilds: bool,
    /// Achievements enabled
    pub achievements: bool,
    /// Quest system enabled
    pub quests: bool,
    /// Market system enabled
    pub market: bool,
    /// Bank system enabled
    pub bank: bool,
    /// Arena/PvP zones enabled
    pub arena: bool,
    /// Matchmaking enabled
    pub matchmaking: bool,
    /// Mounts enabled
    pub mounts: bool,
    /// Addons/outfits system enabled
    pub outfits: bool,
    /// Daily rewards enabled
    pub daily_rewards: bool,
    /// Prey system enabled
    pub prey: bool,
    /// Forge system enabled
    pub forge: bool,
    /// Custom spells enabled
    pub custom_spells: bool,
    /// Lua scripting enabled
    pub lua_scripts: bool,
}

impl Default for FeaturesConfig {
    fn default() -> Self {
        Self {
            houses: true,
            guilds: true,
            achievements: true,
            quests: true,
            market: true,
            bank: true,
            arena: true,
            matchmaking: true,
            mounts: true,
            outfits: true,
            daily_rewards: true,
            prey: true,
            forge: true,
            custom_spells: true,
            lua_scripts: true,
        }
    }
}

/// Predefined realm configuration templates
impl RealmConfig {
    /// Standard PvE realm
    pub fn pve() -> Self {
        Self {
            name: "PvE Realm".to_string(),
            realm_type: RealmType::PvE,
            ..Default::default()
        }
    }

    /// Open PvP realm
    pub fn pvp() -> Self {
        Self {
            name: "PvP Realm".to_string(),
            realm_type: RealmType::PvP,
            pvp: PvPConfig {
                enabled: true,
                pvp_type: "open".to_string(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Hardcore realm
    pub fn hardcore() -> Self {
        Self {
            name: "Hardcore Realm".to_string(),
            realm_type: RealmType::Hardcore,
            death: DeathConfig {
                exp_loss: 50.0,
                skill_loss: 50.0,
                item_drop: true,
                container_drop: true,
                blessings_enabled: false,
                aol_enabled: false,
                ..Default::default()
            },
            experience: ExperienceConfig {
                exp_rate: 0.5,
                skill_rate: 0.5,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Retro realm (classic rules)
    pub fn retro() -> Self {
        Self {
            name: "Retro Realm".to_string(),
            realm_type: RealmType::Retro,
            experience: ExperienceConfig {
                exp_rate: 1.0,
                level_cap: 500,
                stamina_enabled: false,
                ..Default::default()
            },
            features: FeaturesConfig {
                prey: false,
                forge: false,
                daily_rewards: false,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// High-rate fun server
    pub fn fun_server() -> Self {
        Self {
            name: "Fun Server".to_string(),
            realm_type: RealmType::Custom,
            experience: ExperienceConfig {
                exp_rate: 100.0,
                skill_rate: 50.0,
                stamina_enabled: false,
                ..Default::default()
            },
            economy: EconomyConfig {
                loot_rate: 10.0,
                gold_rate: 10.0,
                ..Default::default()
            },
            death: DeathConfig {
                exp_loss: 0.0,
                skill_loss: 0.0,
                item_drop: false,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
