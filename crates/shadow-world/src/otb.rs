//! OTB (Open Tibia Binary) item parser
//!
//! Handles loading of item type definitions from OTB files.

use crate::item::{
    AmmoType, ItemFlags, ItemGroup, ItemType, ShootType, SlotType, WeaponType,
};
use crate::{Result, WorldError};
use byteorder::{LittleEndian, ReadBytesExt};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;
use tracing::{debug, info, warn};

/// OTB file signature
const OTB_SIGNATURE: u32 = 0;

/// OTB item group constants
const ITEM_GROUP_NONE: u8 = 0;
const ITEM_GROUP_GROUND: u8 = 1;
const ITEM_GROUP_CONTAINER: u8 = 2;
const ITEM_GROUP_WEAPON: u8 = 3;
const ITEM_GROUP_AMMUNITION: u8 = 4;
const ITEM_GROUP_ARMOR: u8 = 5;
const ITEM_GROUP_CHARGES: u8 = 6;
const ITEM_GROUP_TELEPORT: u8 = 7;
const ITEM_GROUP_MAGICFIELD: u8 = 8;
const ITEM_GROUP_WRITEABLE: u8 = 9;
const ITEM_GROUP_KEY: u8 = 10;
const ITEM_GROUP_SPLASH: u8 = 11;
const ITEM_GROUP_FLUID: u8 = 12;
const ITEM_GROUP_DOOR: u8 = 13;
const ITEM_GROUP_DEPRECATED: u8 = 14;

/// OTB item flags
const FLAG_BLOCK_SOLID: u32 = 1 << 0;
const FLAG_BLOCK_PROJECTILE: u32 = 1 << 1;
const FLAG_BLOCK_PATHFIND: u32 = 1 << 2;
const FLAG_HAS_HEIGHT: u32 = 1 << 3;
const FLAG_USEABLE: u32 = 1 << 4;
const FLAG_PICKUPABLE: u32 = 1 << 5;
const FLAG_MOVEABLE: u32 = 1 << 6;
const FLAG_STACKABLE: u32 = 1 << 7;
const FLAG_FLOORCHANGEDOWN: u32 = 1 << 8;
const FLAG_FLOORCHANGENORTH: u32 = 1 << 9;
const FLAG_FLOORCHANGEEAST: u32 = 1 << 10;
const FLAG_FLOORCHANGESOUTH: u32 = 1 << 11;
const FLAG_FLOORCHANGEWEST: u32 = 1 << 12;
const FLAG_ALWAYSONTOP: u32 = 1 << 13;
const FLAG_READABLE: u32 = 1 << 14;
const FLAG_ROTATABLE: u32 = 1 << 15;
const FLAG_HANGABLE: u32 = 1 << 16;
const FLAG_VERTICAL: u32 = 1 << 17;
const FLAG_HORIZONTAL: u32 = 1 << 18;
const FLAG_CANNOTDECAY: u32 = 1 << 19;
const FLAG_ALLOWDISTREAD: u32 = 1 << 20;
const FLAG_UNUSED: u32 = 1 << 21;
const FLAG_CLIENTCHARGES: u32 = 1 << 22;
const FLAG_LOOKTHROUGH: u32 = 1 << 23;
const FLAG_ANIMATION: u32 = 1 << 24;
const FLAG_FULLTILE: u32 = 1 << 25;
const FLAG_FORCEUSE: u32 = 1 << 26;

/// OTB attribute types
const ATTR_SERVERID: u8 = 0x10;
const ATTR_CLIENTID: u8 = 0x11;
const ATTR_NAME: u8 = 0x12;
const ATTR_DESCR: u8 = 0x13;
const ATTR_SPEED: u8 = 0x14;
const ATTR_SLOT: u8 = 0x15;
const ATTR_MAXITEMS: u8 = 0x16;
const ATTR_WEIGHT: u8 = 0x17;
const ATTR_WEAPON: u8 = 0x18;
const ATTR_AMU: u8 = 0x19;
const ATTR_ARMOR: u8 = 0x1A;
const ATTR_MAGLEVEL: u8 = 0x1B;
const ATTR_MAGFIELDTYPE: u8 = 0x1C;
const ATTR_WRITEABLE: u8 = 0x1D;
const ATTR_ROTATETO: u8 = 0x1E;
const ATTR_DECAY: u8 = 0x1F;
const ATTR_SPRITEHASH: u8 = 0x20;
const ATTR_MINIMAPCOLOR: u8 = 0x21;
const ATTR_07: u8 = 0x22;
const ATTR_08: u8 = 0x23;
const ATTR_LIGHT: u8 = 0x24;
const ATTR_DECAY2: u8 = 0x25;
const ATTR_WEAPON2: u8 = 0x26;
const ATTR_AMU2: u8 = 0x27;
const ATTR_ARMOR2: u8 = 0x28;
const ATTR_WRITEABLE2: u8 = 0x29;
const ATTR_LIGHT2: u8 = 0x2A;
const ATTR_TOPORDER: u8 = 0x2B;
const ATTR_WRITEABLE3: u8 = 0x2C;
const ATTR_WAREID: u8 = 0x2D;

/// Node markers
const NODE_START: u8 = 0xFE;
const NODE_END: u8 = 0xFF;
const ESCAPE_CHAR: u8 = 0xFD;

/// OTB parser
pub struct OtbLoader {
    /// Buffer for reading
    buffer: Vec<u8>,
    /// Current position
    position: usize,
    /// Major version
    major_version: u32,
    /// Minor version
    minor_version: u32,
    /// Build number
    build_number: u32,
}

impl OtbLoader {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            position: 0,
            major_version: 0,
            minor_version: 0,
            build_number: 0,
        }
    }

    /// Load items from OTB file
    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<HashMap<u16, ItemType>> {
        info!("Loading OTB items from: {}", path.as_ref().display());

        // Read file
        let file = File::open(&path)
            .map_err(|e| WorldError::OtbLoad(format!("Failed to open file: {}", e)))?;

        let mut reader = BufReader::new(file);
        self.buffer.clear();
        reader.read_to_end(&mut self.buffer)
            .map_err(|e| WorldError::OtbLoad(format!("Failed to read file: {}", e)))?;

        self.position = 0;

        // Read signature
        let signature = self.read_u32()?;
        if signature != OTB_SIGNATURE {
            return Err(WorldError::OtbLoad(format!(
                "Invalid OTB signature: expected {}, got {}",
                OTB_SIGNATURE, signature
            )));
        }

        // Parse root node
        self.parse_root()
    }

    /// Parse root node
    fn parse_root(&mut self) -> Result<HashMap<u16, ItemType>> {
        // Read node start
        let marker = self.read_u8()?;
        if marker != NODE_START {
            return Err(WorldError::OtbLoad("Expected node start".to_string()));
        }

        // Skip node type (should be 0)
        let _node_type = self.read_u8()?;

        // Read flags (unused)
        let _flags = self.read_u32()?;

        // Read version info
        let attr = self.read_u8()?;
        if attr == 0x01 {
            let data_len = self.read_u16()?;
            if data_len >= 12 {
                // Skip first 4 bytes
                self.position += 4;
                self.major_version = self.read_u32()?;
                self.minor_version = self.read_u32()?;
                self.build_number = self.read_u32()?;

                // Skip remaining bytes
                if data_len > 16 {
                    self.position += (data_len - 16) as usize;
                }
            } else {
                self.position += data_len as usize;
            }
        }

        info!(
            "OTB version: {}.{}.{}",
            self.major_version, self.minor_version, self.build_number
        );

        let mut items = HashMap::new();

        // Parse item nodes
        loop {
            if self.position >= self.buffer.len() {
                break;
            }

            let marker = self.peek_u8()?;
            if marker == NODE_END {
                self.position += 1;
                break;
            }

            if marker == NODE_START {
                self.position += 1;
                if let Some(item) = self.parse_item_node()? {
                    items.insert(item.id, item);
                }
            } else {
                self.position += 1;
            }
        }

        info!("Loaded {} item types from OTB", items.len());
        Ok(items)
    }

    /// Parse item node
    fn parse_item_node(&mut self) -> Result<Option<ItemType>> {
        // Read group
        let group = self.read_u8()?;

        // Read flags
        let flags = self.read_u32()?;

        let mut item = ItemType::new(0);
        item.group = Self::convert_group(group);
        Self::apply_flags(&mut item, flags);

        // Read attributes
        while self.position < self.buffer.len() {
            let byte = self.peek_u8()?;
            if byte == NODE_START || byte == NODE_END {
                break;
            }

            let attr = self.read_u8()?;
            let data_len = self.read_u16()? as usize;

            match attr {
                ATTR_SERVERID => {
                    item.id = self.read_u16()?;
                    if data_len > 2 {
                        self.position += data_len - 2;
                    }
                }
                ATTR_CLIENTID => {
                    item.client_id = self.read_u16()?;
                    if data_len > 2 {
                        self.position += data_len - 2;
                    }
                }
                ATTR_NAME => {
                    item.name = self.read_string(data_len)?;
                }
                ATTR_DESCR => {
                    item.description = Some(self.read_string(data_len)?);
                }
                ATTR_SPEED => {
                    if data_len >= 2 {
                        item.speed_boost = Some(self.read_u16()? as i32);
                        if data_len > 2 {
                            self.position += data_len - 2;
                        }
                    } else {
                        self.position += data_len;
                    }
                }
                ATTR_SLOT => {
                    if data_len >= 2 {
                        let slot = self.read_u16()?;
                        item.slot_type = Self::convert_slot(slot);
                        if data_len > 2 {
                            self.position += data_len - 2;
                        }
                    } else {
                        self.position += data_len;
                    }
                }
                ATTR_MAXITEMS => {
                    if data_len >= 2 {
                        item.container_size = Some(self.read_u16()? as u8);
                        if data_len > 2 {
                            self.position += data_len - 2;
                        }
                    } else {
                        self.position += data_len;
                    }
                }
                ATTR_WEIGHT => {
                    if data_len >= 4 {
                        item.weight = self.read_u32()?;
                        if data_len > 4 {
                            self.position += data_len - 4;
                        }
                    } else {
                        self.position += data_len;
                    }
                }
                ATTR_ARMOR | ATTR_ARMOR2 => {
                    if data_len >= 2 {
                        item.armor = Some(self.read_u16()? as i32);
                        if data_len > 2 {
                            self.position += data_len - 2;
                        }
                    } else {
                        self.position += data_len;
                    }
                }
                ATTR_LIGHT | ATTR_LIGHT2 => {
                    if data_len >= 4 {
                        item.light_level = Some(self.read_u16()? as u8);
                        item.light_color = Some(self.read_u16()? as u8);
                        if data_len > 4 {
                            self.position += data_len - 4;
                        }
                    } else {
                        self.position += data_len;
                    }
                }
                ATTR_WRITEABLE | ATTR_WRITEABLE2 | ATTR_WRITEABLE3 => {
                    if data_len >= 2 {
                        item.max_text_length = Some(self.read_u16()?);
                        if data_len > 2 {
                            self.position += data_len - 2;
                        }
                    } else {
                        self.position += data_len;
                    }
                }
                ATTR_TOPORDER => {
                    if data_len >= 1 {
                        let _top_order = self.read_u8()?;
                        if data_len > 1 {
                            self.position += data_len - 1;
                        }
                    } else {
                        self.position += data_len;
                    }
                }
                _ => {
                    // Skip unknown attributes
                    self.position += data_len;
                }
            }
        }

        // Skip to node end
        while self.position < self.buffer.len() {
            let byte = self.read_u8()?;
            if byte == NODE_END {
                break;
            }
        }

        if item.id != 0 {
            Ok(Some(item))
        } else {
            Ok(None)
        }
    }

    /// Convert OTB group to ItemGroup
    fn convert_group(group: u8) -> ItemGroup {
        match group {
            ITEM_GROUP_GROUND => ItemGroup::Ground,
            ITEM_GROUP_CONTAINER => ItemGroup::Container,
            ITEM_GROUP_WEAPON => ItemGroup::Weapon,
            ITEM_GROUP_AMMUNITION => ItemGroup::Ammunition,
            ITEM_GROUP_ARMOR => ItemGroup::Armor,
            ITEM_GROUP_CHARGES => ItemGroup::Charges,
            ITEM_GROUP_TELEPORT => ItemGroup::Teleport,
            ITEM_GROUP_MAGICFIELD => ItemGroup::MagicField,
            ITEM_GROUP_WRITEABLE => ItemGroup::Writeable,
            ITEM_GROUP_KEY => ItemGroup::Key,
            ITEM_GROUP_SPLASH => ItemGroup::Splash,
            ITEM_GROUP_FLUID => ItemGroup::Fluid,
            ITEM_GROUP_DOOR => ItemGroup::Door,
            ITEM_GROUP_DEPRECATED => ItemGroup::Deprecated,
            _ => ItemGroup::None,
        }
    }

    /// Convert slot number to SlotType
    fn convert_slot(slot: u16) -> Option<SlotType> {
        match slot {
            1 => Some(SlotType::Head),
            2 => Some(SlotType::Necklace),
            3 => Some(SlotType::Backpack),
            4 => Some(SlotType::Armor),
            5 => Some(SlotType::Right),
            6 => Some(SlotType::Left),
            7 => Some(SlotType::Legs),
            8 => Some(SlotType::Feet),
            9 => Some(SlotType::Ring),
            10 => Some(SlotType::Ammo),
            _ => None,
        }
    }

    /// Apply OTB flags to ItemType
    fn apply_flags(item: &mut ItemType, flags: u32) {
        if flags & FLAG_BLOCK_SOLID != 0 {
            item.flags.set(1 << 0);
        }
        if flags & FLAG_BLOCK_PROJECTILE != 0 {
            item.flags.set(1 << 1);
        }
        if flags & FLAG_BLOCK_PATHFIND != 0 {
            item.flags.set(1 << 2);
        }
        if flags & FLAG_HAS_HEIGHT != 0 {
            item.flags.set(1 << 3);
        }
        if flags & FLAG_USEABLE != 0 {
            item.flags.set(1 << 4);
        }
        if flags & FLAG_PICKUPABLE != 0 {
            item.flags.set(1 << 5);
        }
        if flags & FLAG_MOVEABLE != 0 {
            item.flags.set(1 << 6);
        }
        if flags & FLAG_STACKABLE != 0 {
            item.flags.set(1 << 7);
        }
        if flags & FLAG_ALWAYSONTOP != 0 {
            item.flags.set(1 << 8);
        }
        if flags & FLAG_READABLE != 0 {
            item.flags.set(1 << 9);
        }
        if flags & FLAG_ROTATABLE != 0 {
            item.flags.set(1 << 10);
        }
        if flags & FLAG_HANGABLE != 0 {
            item.flags.set(1 << 11);
        }
    }

    // Helper methods

    fn read_u8(&mut self) -> Result<u8> {
        if self.position >= self.buffer.len() {
            return Err(WorldError::OtbLoad("Unexpected end of file".to_string()));
        }
        let byte = self.buffer[self.position];
        self.position += 1;

        // Handle escape character
        if byte == ESCAPE_CHAR && self.position < self.buffer.len() {
            let next = self.buffer[self.position];
            self.position += 1;
            return Ok(next);
        }

        Ok(byte)
    }

    fn peek_u8(&self) -> Result<u8> {
        if self.position >= self.buffer.len() {
            return Err(WorldError::OtbLoad("Unexpected end of file".to_string()));
        }
        Ok(self.buffer[self.position])
    }

    fn read_u16(&mut self) -> Result<u16> {
        let lo = self.read_u8()? as u16;
        let hi = self.read_u8()? as u16;
        Ok(lo | (hi << 8))
    }

    fn read_u32(&mut self) -> Result<u32> {
        let b0 = self.read_u8()? as u32;
        let b1 = self.read_u8()? as u32;
        let b2 = self.read_u8()? as u32;
        let b3 = self.read_u8()? as u32;
        Ok(b0 | (b1 << 8) | (b2 << 16) | (b3 << 24))
    }

    fn read_string(&mut self, len: usize) -> Result<String> {
        let mut bytes = Vec::with_capacity(len);
        for _ in 0..len {
            bytes.push(self.read_u8()?);
        }
        // Remove null terminator if present
        if bytes.last() == Some(&0) {
            bytes.pop();
        }
        String::from_utf8(bytes)
            .map_err(|e| WorldError::OtbLoad(format!("Invalid string: {}", e)))
    }
}

impl Default for OtbLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_conversion() {
        assert_eq!(OtbLoader::convert_group(ITEM_GROUP_GROUND), ItemGroup::Ground);
        assert_eq!(OtbLoader::convert_group(ITEM_GROUP_CONTAINER), ItemGroup::Container);
        assert_eq!(OtbLoader::convert_group(255), ItemGroup::None);
    }

    #[test]
    fn test_slot_conversion() {
        assert_eq!(OtbLoader::convert_slot(1), Some(SlotType::Head));
        assert_eq!(OtbLoader::convert_slot(4), Some(SlotType::Armor));
        assert_eq!(OtbLoader::convert_slot(100), None);
    }
}
