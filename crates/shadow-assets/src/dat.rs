//! DAT file parser
//!
//! DAT files contain item, creature, effect, and missile definitions.
//! Each "thing" has properties like sprites, flags, light, market data, etc.

use crate::{AssetError, AssetResult, ClientVersion};
use byteorder::{LittleEndian, ReadBytesExt};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use tracing::{debug, trace};

/// Thing categories in DAT file
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThingCategory {
    Item = 0,
    Creature = 1,
    Effect = 2,
    Missile = 3,
}

impl ThingCategory {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(ThingCategory::Item),
            1 => Some(ThingCategory::Creature),
            2 => Some(ThingCategory::Effect),
            3 => Some(ThingCategory::Missile),
            _ => None,
        }
    }
}

/// Thing type definition
#[derive(Debug, Clone)]
pub struct ThingType {
    pub id: u16,
    pub category: ThingCategory,
    pub flags: ThingFlags,
    pub frame_groups: Vec<FrameGroupDef>,
}

/// Frame group definition (for animations)
#[derive(Debug, Clone)]
pub struct FrameGroupDef {
    pub width: u8,
    pub height: u8,
    pub exact_size: u8,
    pub layers: u8,
    pub pattern_x: u8,
    pub pattern_y: u8,
    pub pattern_z: u8,
    pub frames: u8,
    pub animation: Option<AnimationDef>,
    pub sprite_ids: Vec<u32>,
}

/// Animation definition
#[derive(Debug, Clone)]
pub struct AnimationDef {
    pub async_animation: bool,
    pub loop_count: i32,
    pub start_phase: u8,
    pub phases: Vec<AnimationPhase>,
}

/// Animation phase timing
#[derive(Debug, Clone)]
pub struct AnimationPhase {
    pub min_duration: u32,
    pub max_duration: u32,
}

/// Thing flags
#[derive(Debug, Clone, Default)]
pub struct ThingFlags {
    pub is_ground: bool,
    pub ground_speed: u16,
    pub is_ground_border: bool,
    pub is_on_bottom: bool,
    pub is_on_top: bool,
    pub is_container: bool,
    pub is_stackable: bool,
    pub is_force_use: bool,
    pub is_multi_use: bool,
    pub is_writable: bool,
    pub writable_length: u16,
    pub is_writable_once: bool,
    pub is_fluid_container: bool,
    pub is_splash: bool,
    pub is_not_walkable: bool,
    pub is_not_moveable: bool,
    pub is_block_projectile: bool,
    pub is_not_pathable: bool,
    pub is_no_move_animation: bool,
    pub is_pickupable: bool,
    pub is_hangable: bool,
    pub is_hook_south: bool,
    pub is_hook_east: bool,
    pub is_rotatable: bool,
    pub has_light: bool,
    pub light_level: u16,
    pub light_color: u16,
    pub is_dont_hide: bool,
    pub is_translucent: bool,
    pub has_displacement: bool,
    pub displacement_x: u16,
    pub displacement_y: u16,
    pub has_elevation: bool,
    pub elevation: u16,
    pub is_lying_corpse: bool,
    pub is_animate_always: bool,
    pub is_minimap: bool,
    pub minimap_color: u16,
    pub is_lens_help: bool,
    pub lens_help: u16,
    pub is_full_ground: bool,
    pub is_look_through: bool,
    pub is_cloth: bool,
    pub cloth_slot: u16,
    pub is_market: bool,
    pub market_category: u16,
    pub market_trade_as: u16,
    pub market_show_as: u16,
    pub market_name: String,
    pub market_restrict_profession: u16,
    pub market_restrict_level: u16,
    pub has_default_action: bool,
    pub default_action: u16,
    pub is_usable: bool,
    pub has_charges: bool,
    pub floor_change: bool,
    pub is_top_effect: bool,
    pub has_wrap: bool,
    pub is_unwrap: bool,
}

/// DAT file flags (attribute IDs)
#[repr(u8)]
enum DatAttr {
    Ground = 0,
    GroundBorder = 1,
    OnBottom = 2,
    OnTop = 3,
    Container = 4,
    Stackable = 5,
    ForceUse = 6,
    MultiUse = 7,
    Writable = 8,
    WritableOnce = 9,
    FluidContainer = 10,
    Splash = 11,
    NotWalkable = 12,
    NotMoveable = 13,
    BlockProjectile = 14,
    NotPathable = 15,
    NoMoveAnimation = 16,
    Pickupable = 17,
    Hangable = 18,
    HookSouth = 19,
    HookEast = 20,
    Rotatable = 21,
    Light = 22,
    DontHide = 23,
    Translucent = 24,
    Displacement = 25,
    Elevation = 26,
    LyingCorpse = 27,
    AnimateAlways = 28,
    Minimap = 29,
    LensHelp = 30,
    FullGround = 31,
    LookThrough = 32,
    Cloth = 33,
    Market = 34,
    DefaultAction = 35,
    Wrap = 36,
    Unwrap = 37,
    TopEffect = 38,
    Usable = 254,
    End = 255,
}

/// DAT file reader
pub struct DatFile {
    version: ClientVersion,
    signature: u32,
    items: HashMap<u16, ThingType>,
    creatures: HashMap<u16, ThingType>,
    effects: HashMap<u16, ThingType>,
    missiles: HashMap<u16, ThingType>,
    item_count: u16,
    creature_count: u16,
    effect_count: u16,
    missile_count: u16,
}

impl DatFile {
    /// Load DAT file
    pub fn load<P: AsRef<Path>>(path: P, version: ClientVersion) -> AssetResult<Self> {
        let file = File::open(path.as_ref())?;
        let mut reader = BufReader::new(file);

        // Read signature
        let signature = reader.read_u32::<LittleEndian>()?;
        debug!("DAT signature: 0x{:08X}", signature);

        // Read counts
        let item_count = reader.read_u16::<LittleEndian>()?;
        let creature_count = reader.read_u16::<LittleEndian>()?;
        let effect_count = reader.read_u16::<LittleEndian>()?;
        let missile_count = reader.read_u16::<LittleEndian>()?;

        debug!(
            "DAT counts - items: {}, creatures: {}, effects: {}, missiles: {}",
            item_count, creature_count, effect_count, missile_count
        );

        let mut dat = Self {
            version,
            signature,
            items: HashMap::new(),
            creatures: HashMap::new(),
            effects: HashMap::new(),
            missiles: HashMap::new(),
            item_count,
            creature_count,
            effect_count,
            missile_count,
        };

        // Load items (start at 100)
        for id in 100..=item_count {
            let thing = dat.read_thing(&mut reader, id, ThingCategory::Item)?;
            dat.items.insert(id, thing);
        }

        // Load creatures (start at 1)
        for id in 1..=creature_count {
            let thing = dat.read_thing(&mut reader, id, ThingCategory::Creature)?;
            dat.creatures.insert(id, thing);
        }

        // Load effects (start at 1)
        for id in 1..=effect_count {
            let thing = dat.read_thing(&mut reader, id, ThingCategory::Effect)?;
            dat.effects.insert(id, thing);
        }

        // Load missiles (start at 1)
        for id in 1..=missile_count {
            let thing = dat.read_thing(&mut reader, id, ThingCategory::Missile)?;
            dat.missiles.insert(id, thing);
        }

        Ok(dat)
    }

    /// Read a thing definition
    fn read_thing(
        &self,
        reader: &mut BufReader<File>,
        id: u16,
        category: ThingCategory,
    ) -> AssetResult<ThingType> {
        let flags = self.read_flags(reader)?;
        let frame_groups = self.read_frame_groups(reader, category)?;

        Ok(ThingType {
            id,
            category,
            flags,
            frame_groups,
        })
    }

    /// Read thing flags
    fn read_flags(&self, reader: &mut BufReader<File>) -> AssetResult<ThingFlags> {
        let mut flags = ThingFlags::default();

        loop {
            let attr = reader.read_u8()?;

            if attr == DatAttr::End as u8 {
                break;
            }

            match attr {
                x if x == DatAttr::Ground as u8 => {
                    flags.is_ground = true;
                    flags.ground_speed = reader.read_u16::<LittleEndian>()?;
                }
                x if x == DatAttr::GroundBorder as u8 => {
                    flags.is_ground_border = true;
                }
                x if x == DatAttr::OnBottom as u8 => {
                    flags.is_on_bottom = true;
                }
                x if x == DatAttr::OnTop as u8 => {
                    flags.is_on_top = true;
                }
                x if x == DatAttr::Container as u8 => {
                    flags.is_container = true;
                }
                x if x == DatAttr::Stackable as u8 => {
                    flags.is_stackable = true;
                }
                x if x == DatAttr::ForceUse as u8 => {
                    flags.is_force_use = true;
                }
                x if x == DatAttr::MultiUse as u8 => {
                    flags.is_multi_use = true;
                }
                x if x == DatAttr::Writable as u8 => {
                    flags.is_writable = true;
                    flags.writable_length = reader.read_u16::<LittleEndian>()?;
                }
                x if x == DatAttr::WritableOnce as u8 => {
                    flags.is_writable_once = true;
                    flags.writable_length = reader.read_u16::<LittleEndian>()?;
                }
                x if x == DatAttr::FluidContainer as u8 => {
                    flags.is_fluid_container = true;
                }
                x if x == DatAttr::Splash as u8 => {
                    flags.is_splash = true;
                }
                x if x == DatAttr::NotWalkable as u8 => {
                    flags.is_not_walkable = true;
                }
                x if x == DatAttr::NotMoveable as u8 => {
                    flags.is_not_moveable = true;
                }
                x if x == DatAttr::BlockProjectile as u8 => {
                    flags.is_block_projectile = true;
                }
                x if x == DatAttr::NotPathable as u8 => {
                    flags.is_not_pathable = true;
                }
                x if x == DatAttr::NoMoveAnimation as u8 => {
                    flags.is_no_move_animation = true;
                }
                x if x == DatAttr::Pickupable as u8 => {
                    flags.is_pickupable = true;
                }
                x if x == DatAttr::Hangable as u8 => {
                    flags.is_hangable = true;
                }
                x if x == DatAttr::HookSouth as u8 => {
                    flags.is_hook_south = true;
                }
                x if x == DatAttr::HookEast as u8 => {
                    flags.is_hook_east = true;
                }
                x if x == DatAttr::Rotatable as u8 => {
                    flags.is_rotatable = true;
                }
                x if x == DatAttr::Light as u8 => {
                    flags.has_light = true;
                    flags.light_level = reader.read_u16::<LittleEndian>()?;
                    flags.light_color = reader.read_u16::<LittleEndian>()?;
                }
                x if x == DatAttr::DontHide as u8 => {
                    flags.is_dont_hide = true;
                }
                x if x == DatAttr::Translucent as u8 => {
                    flags.is_translucent = true;
                }
                x if x == DatAttr::Displacement as u8 => {
                    flags.has_displacement = true;
                    flags.displacement_x = reader.read_u16::<LittleEndian>()?;
                    flags.displacement_y = reader.read_u16::<LittleEndian>()?;
                }
                x if x == DatAttr::Elevation as u8 => {
                    flags.has_elevation = true;
                    flags.elevation = reader.read_u16::<LittleEndian>()?;
                }
                x if x == DatAttr::LyingCorpse as u8 => {
                    flags.is_lying_corpse = true;
                }
                x if x == DatAttr::AnimateAlways as u8 => {
                    flags.is_animate_always = true;
                }
                x if x == DatAttr::Minimap as u8 => {
                    flags.is_minimap = true;
                    flags.minimap_color = reader.read_u16::<LittleEndian>()?;
                }
                x if x == DatAttr::LensHelp as u8 => {
                    flags.is_lens_help = true;
                    flags.lens_help = reader.read_u16::<LittleEndian>()?;
                }
                x if x == DatAttr::FullGround as u8 => {
                    flags.is_full_ground = true;
                }
                x if x == DatAttr::LookThrough as u8 => {
                    flags.is_look_through = true;
                }
                x if x == DatAttr::Cloth as u8 => {
                    flags.is_cloth = true;
                    flags.cloth_slot = reader.read_u16::<LittleEndian>()?;
                }
                x if x == DatAttr::Market as u8 => {
                    flags.is_market = true;
                    flags.market_category = reader.read_u16::<LittleEndian>()?;
                    flags.market_trade_as = reader.read_u16::<LittleEndian>()?;
                    flags.market_show_as = reader.read_u16::<LittleEndian>()?;

                    // Read market name
                    let name_len = reader.read_u16::<LittleEndian>()? as usize;
                    let mut name_bytes = vec![0u8; name_len];
                    reader.read_exact(&mut name_bytes)?;
                    flags.market_name = String::from_utf8_lossy(&name_bytes).to_string();

                    flags.market_restrict_profession = reader.read_u16::<LittleEndian>()?;
                    flags.market_restrict_level = reader.read_u16::<LittleEndian>()?;
                }
                x if x == DatAttr::DefaultAction as u8 => {
                    flags.has_default_action = true;
                    flags.default_action = reader.read_u16::<LittleEndian>()?;
                }
                x if x == DatAttr::Wrap as u8 => {
                    flags.has_wrap = true;
                }
                x if x == DatAttr::Unwrap as u8 => {
                    flags.is_unwrap = true;
                }
                x if x == DatAttr::TopEffect as u8 => {
                    flags.is_top_effect = true;
                }
                x if x == DatAttr::Usable as u8 => {
                    flags.is_usable = true;
                }
                _ => {
                    // Unknown attribute, try to skip
                    trace!("Unknown DAT attribute: {}", attr);
                }
            }
        }

        Ok(flags)
    }

    /// Read frame groups
    fn read_frame_groups(
        &self,
        reader: &mut BufReader<File>,
        category: ThingCategory,
    ) -> AssetResult<Vec<FrameGroupDef>> {
        let group_count = if self.version.supports_extended_sprites()
            && (category == ThingCategory::Item || category == ThingCategory::Creature)
        {
            reader.read_u8()? as usize
        } else {
            1
        };

        let mut groups = Vec::with_capacity(group_count);

        for _ in 0..group_count {
            if self.version.supports_extended_sprites()
                && (category == ThingCategory::Item || category == ThingCategory::Creature)
            {
                let _frame_group_type = reader.read_u8()?;
            }

            let width = reader.read_u8()?;
            let height = reader.read_u8()?;

            let exact_size = if width > 1 || height > 1 {
                reader.read_u8()?
            } else {
                32
            };

            let layers = reader.read_u8()?;
            let pattern_x = reader.read_u8()?;
            let pattern_y = reader.read_u8()?;
            let pattern_z = reader.read_u8()?;
            let frames = reader.read_u8()?;

            // Read animation if frames > 1
            let animation = if frames > 1 {
                let async_animation = if self.version.supports_extended_sprites() {
                    reader.read_u8()? != 0
                } else {
                    false
                };

                let loop_count = if self.version.supports_extended_sprites() {
                    reader.read_i32::<LittleEndian>()?
                } else {
                    0
                };

                let start_phase = if self.version.supports_extended_sprites() {
                    reader.read_u8()?
                } else {
                    0
                };

                let phases = if self.version.supports_extended_sprites() {
                    (0..frames)
                        .map(|_| {
                            let min = reader.read_u32::<LittleEndian>().unwrap_or(100);
                            let max = reader.read_u32::<LittleEndian>().unwrap_or(100);
                            AnimationPhase {
                                min_duration: min,
                                max_duration: max,
                            }
                        })
                        .collect()
                } else {
                    vec![]
                };

                Some(AnimationDef {
                    async_animation,
                    loop_count,
                    start_phase,
                    phases,
                })
            } else {
                None
            };

            // Calculate sprite count
            let sprite_count = (width as usize)
                * (height as usize)
                * (layers as usize)
                * (pattern_x as usize)
                * (pattern_y as usize)
                * (pattern_z as usize)
                * (frames as usize);

            // Read sprite IDs
            let sprite_ids: Vec<u32> = if self.version.supports_extended_sprites() {
                (0..sprite_count)
                    .map(|_| reader.read_u32::<LittleEndian>().unwrap_or(0))
                    .collect()
            } else {
                (0..sprite_count)
                    .map(|_| reader.read_u16::<LittleEndian>().unwrap_or(0) as u32)
                    .collect()
            };

            groups.push(FrameGroupDef {
                width,
                height,
                exact_size,
                layers,
                pattern_x,
                pattern_y,
                pattern_z,
                frames,
                animation,
                sprite_ids,
            });
        }

        Ok(groups)
    }

    /// Get signature
    pub fn signature(&self) -> u32 {
        self.signature
    }

    /// Get item by ID
    pub fn get_item(&self, id: u16) -> Option<&ThingType> {
        self.items.get(&id)
    }

    /// Get creature by ID
    pub fn get_creature(&self, id: u16) -> Option<&ThingType> {
        self.creatures.get(&id)
    }

    /// Get effect by ID
    pub fn get_effect(&self, id: u16) -> Option<&ThingType> {
        self.effects.get(&id)
    }

    /// Get missile by ID
    pub fn get_missile(&self, id: u16) -> Option<&ThingType> {
        self.missiles.get(&id)
    }

    /// Get all items
    pub fn items(&self) -> impl Iterator<Item = &ThingType> {
        self.items.values()
    }

    /// Get all creatures
    pub fn creatures(&self) -> impl Iterator<Item = &ThingType> {
        self.creatures.values()
    }

    /// Get all effects
    pub fn effects(&self) -> impl Iterator<Item = &ThingType> {
        self.effects.values()
    }

    /// Get all missiles
    pub fn missiles(&self) -> impl Iterator<Item = &ThingType> {
        self.missiles.values()
    }

    /// Get counts
    pub fn item_count(&self) -> u16 {
        self.item_count
    }

    pub fn creature_count(&self) -> u16 {
        self.creature_count
    }

    pub fn effect_count(&self) -> u16 {
        self.effect_count
    }

    pub fn missile_count(&self) -> u16 {
        self.missile_count
    }

    /// Check if item is ground
    pub fn is_ground(&self, id: u16) -> bool {
        self.items
            .get(&id)
            .map(|t| t.flags.is_ground)
            .unwrap_or(false)
    }

    /// Check if item is walkable
    pub fn is_walkable(&self, id: u16) -> bool {
        self.items
            .get(&id)
            .map(|t| !t.flags.is_not_walkable)
            .unwrap_or(true)
    }

    /// Check if item blocks projectiles
    pub fn blocks_projectile(&self, id: u16) -> bool {
        self.items
            .get(&id)
            .map(|t| t.flags.is_block_projectile)
            .unwrap_or(false)
    }

    /// Get ground speed
    pub fn ground_speed(&self, id: u16) -> u16 {
        self.items
            .get(&id)
            .map(|t| t.flags.ground_speed)
            .unwrap_or(100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thing_category() {
        assert_eq!(ThingCategory::from_u8(0), Some(ThingCategory::Item));
        assert_eq!(ThingCategory::from_u8(1), Some(ThingCategory::Creature));
        assert_eq!(ThingCategory::from_u8(2), Some(ThingCategory::Effect));
        assert_eq!(ThingCategory::from_u8(3), Some(ThingCategory::Missile));
        assert_eq!(ThingCategory::from_u8(4), None);
    }
}
