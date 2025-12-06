//! OTB (Open Tibia Binary) item database parser
//!
//! OTB files contain server-side item definitions with flags and attributes.

use crate::{AssetError, AssetResult};
use byteorder::{LittleEndian, ReadBytesExt};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use tracing::{debug, trace};

/// OTB node markers
const NODE_START: u8 = 0xFE;
const NODE_END: u8 = 0xFF;
const ESCAPE_CHAR: u8 = 0xFD;

/// OTB root attribute types
#[repr(u8)]
enum RootAttr {
    Version = 0x01,
}

/// OTB item attribute types
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemAttr {
    ServerId = 0x10,
    ClientId = 0x11,
    Name = 0x12,
    Description = 0x13,
    Speed = 0x14,
    Slot = 0x15,
    MaxItems = 0x16,
    Weight = 0x17,
    Weapon = 0x18,
    Ammunition = 0x19,
    Armor = 0x1A,
    MagicLevel = 0x1B,
    MagicField = 0x1C,
    Writable = 0x1D,
    RotateTo = 0x1E,
    Decay = 0x1F,
    SpriteHash = 0x20,
    MinimapColor = 0x21,
    Attr07 = 0x22,
    Attr08 = 0x23,
    Light = 0x24,
    Decay2 = 0x25,
    Weapon2 = 0x26,
    Ammunition2 = 0x27,
    Armor2 = 0x28,
    Writable2 = 0x29,
    Light2 = 0x2A,
    TopOrder = 0x2B,
    Writable3 = 0x2C,
    WareId = 0x2D,
}

/// Item flags in OTB
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ItemFlags: u32 {
        const NONE = 0;
        const BLOCK_SOLID = 1 << 0;
        const BLOCK_PROJECTILE = 1 << 1;
        const BLOCK_PATHFIND = 1 << 2;
        const HAS_HEIGHT = 1 << 3;
        const USEABLE = 1 << 4;
        const PICKUPABLE = 1 << 5;
        const MOVEABLE = 1 << 6;
        const STACKABLE = 1 << 7;
        const FLOOR_CHANGE_DOWN = 1 << 8;
        const FLOOR_CHANGE_NORTH = 1 << 9;
        const FLOOR_CHANGE_EAST = 1 << 10;
        const FLOOR_CHANGE_SOUTH = 1 << 11;
        const FLOOR_CHANGE_WEST = 1 << 12;
        const ALWAYS_ON_TOP = 1 << 13;
        const READABLE = 1 << 14;
        const ROTATABLE = 1 << 15;
        const HANGABLE = 1 << 16;
        const VERTICAL = 1 << 17;
        const HORIZONTAL = 1 << 18;
        const CANNOT_DECAY = 1 << 19;
        const ALLOW_DIST_READ = 1 << 20;
        const UNUSED = 1 << 21;
        const CLIENT_CHARGES = 1 << 22;
        const LOOK_THROUGH = 1 << 23;
        const ANIMATION = 1 << 24;
        const FULL_TILE = 1 << 25;
        const FORCE_USE = 1 << 26;
    }
}

/// Item group types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemGroup {
    None = 0,
    Ground = 1,
    Container = 2,
    Weapon = 3,
    Ammunition = 4,
    Armor = 5,
    Charges = 6,
    Teleport = 7,
    MagicField = 8,
    Writable = 9,
    Key = 10,
    Splash = 11,
    Fluid = 12,
    Door = 13,
    Deprecated = 14,
}

impl ItemGroup {
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => ItemGroup::Ground,
            2 => ItemGroup::Container,
            3 => ItemGroup::Weapon,
            4 => ItemGroup::Ammunition,
            5 => ItemGroup::Armor,
            6 => ItemGroup::Charges,
            7 => ItemGroup::Teleport,
            8 => ItemGroup::MagicField,
            9 => ItemGroup::Writable,
            10 => ItemGroup::Key,
            11 => ItemGroup::Splash,
            12 => ItemGroup::Fluid,
            13 => ItemGroup::Door,
            14 => ItemGroup::Deprecated,
            _ => ItemGroup::None,
        }
    }
}

/// OTB item definition
#[derive(Debug, Clone)]
pub struct OtbItem {
    pub server_id: u16,
    pub client_id: u16,
    pub group: ItemGroup,
    pub flags: ItemFlags,
    pub name: String,
    pub description: String,
    pub speed: u16,
    pub light_level: u8,
    pub light_color: u8,
    pub top_order: u8,
    pub ware_id: u16,
}

impl Default for OtbItem {
    fn default() -> Self {
        Self {
            server_id: 0,
            client_id: 0,
            group: ItemGroup::None,
            flags: ItemFlags::NONE,
            name: String::new(),
            description: String::new(),
            speed: 0,
            light_level: 0,
            light_color: 0,
            top_order: 0,
            ware_id: 0,
        }
    }
}

/// OTB file reader
pub struct OtbFile {
    items: HashMap<u16, OtbItem>,
    client_to_server: HashMap<u16, u16>,
    major_version: u32,
    minor_version: u32,
    build_number: u32,
}

impl OtbFile {
    /// Load OTB file
    pub fn load<P: AsRef<Path>>(path: P) -> AssetResult<Self> {
        let file = File::open(path.as_ref())?;
        let mut reader = BufReader::new(file);

        // Read all data
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;

        // Unescape data
        let data = Self::unescape_data(&data);

        let mut otb = Self {
            items: HashMap::new(),
            client_to_server: HashMap::new(),
            major_version: 0,
            minor_version: 0,
            build_number: 0,
        };

        otb.parse_data(&data)?;

        debug!(
            "Loaded OTB version {}.{}.{} with {} items",
            otb.major_version,
            otb.minor_version,
            otb.build_number,
            otb.items.len()
        );

        Ok(otb)
    }

    /// Unescape OTB data
    fn unescape_data(data: &[u8]) -> Vec<u8> {
        let mut result = Vec::with_capacity(data.len());
        let mut i = 0;

        while i < data.len() {
            if data[i] == ESCAPE_CHAR && i + 1 < data.len() {
                result.push(data[i + 1]);
                i += 2;
            } else {
                result.push(data[i]);
                i += 1;
            }
        }

        result
    }

    /// Parse OTB data
    fn parse_data(&mut self, data: &[u8]) -> AssetResult<()> {
        if data.is_empty() {
            return Err(AssetError::InvalidFormat("Empty OTB file".to_string()));
        }

        let mut pos = 0;

        // Skip initial identifier (4 bytes, usually 0x00000000)
        if pos + 4 > data.len() {
            return Err(AssetError::InvalidFormat("OTB too short".to_string()));
        }
        pos += 4;

        // Expect NODE_START
        if pos >= data.len() || data[pos] != NODE_START {
            return Err(AssetError::InvalidFormat("Expected NODE_START".to_string()));
        }
        pos += 1;

        // Read root node type
        if pos >= data.len() {
            return Err(AssetError::InvalidFormat("Missing root node type".to_string()));
        }
        let _root_type = data[pos];
        pos += 1;

        // Read root node flags (4 bytes)
        if pos + 4 > data.len() {
            return Err(AssetError::InvalidFormat("Missing root flags".to_string()));
        }
        pos += 4;

        // Read root attributes
        while pos < data.len() && data[pos] != NODE_START && data[pos] != NODE_END {
            let attr_type = data[pos];
            pos += 1;

            if attr_type == RootAttr::Version as u8 {
                // Read attribute size
                if pos + 2 > data.len() {
                    break;
                }
                let attr_size = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
                pos += 2;

                if pos + attr_size > data.len() {
                    break;
                }

                // Read version info
                if attr_size >= 12 {
                    self.major_version =
                        u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
                    self.minor_version = u32::from_le_bytes([
                        data[pos + 4],
                        data[pos + 5],
                        data[pos + 6],
                        data[pos + 7],
                    ]);
                    self.build_number = u32::from_le_bytes([
                        data[pos + 8],
                        data[pos + 9],
                        data[pos + 10],
                        data[pos + 11],
                    ]);
                }

                pos += attr_size;
            } else {
                // Skip unknown attribute
                if pos + 2 > data.len() {
                    break;
                }
                let attr_size = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
                pos += 2 + attr_size;
            }
        }

        // Parse item nodes
        while pos < data.len() {
            if data[pos] == NODE_START {
                pos += 1;
                pos = self.parse_item_node(&data, pos)?;
            } else if data[pos] == NODE_END {
                pos += 1;
            } else {
                pos += 1;
            }
        }

        Ok(())
    }

    /// Parse an item node
    fn parse_item_node(&mut self, data: &[u8], mut pos: usize) -> AssetResult<usize> {
        if pos >= data.len() {
            return Ok(pos);
        }

        let mut item = OtbItem::default();

        // Read item group
        item.group = ItemGroup::from_u8(data[pos]);
        pos += 1;

        // Read flags (4 bytes)
        if pos + 4 > data.len() {
            return Ok(pos);
        }
        let flags_raw = u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
        item.flags = ItemFlags::from_bits_truncate(flags_raw);
        pos += 4;

        // Read attributes
        while pos < data.len() && data[pos] != NODE_START && data[pos] != NODE_END {
            let attr_type = data[pos];
            pos += 1;

            if pos + 2 > data.len() {
                break;
            }
            let attr_size = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
            pos += 2;

            if pos + attr_size > data.len() {
                break;
            }

            let attr_data = &data[pos..pos + attr_size];
            pos += attr_size;

            match attr_type {
                x if x == ItemAttr::ServerId as u8 => {
                    if attr_data.len() >= 2 {
                        item.server_id = u16::from_le_bytes([attr_data[0], attr_data[1]]);
                    }
                }
                x if x == ItemAttr::ClientId as u8 => {
                    if attr_data.len() >= 2 {
                        item.client_id = u16::from_le_bytes([attr_data[0], attr_data[1]]);
                    }
                }
                x if x == ItemAttr::Name as u8 => {
                    item.name = String::from_utf8_lossy(attr_data).to_string();
                }
                x if x == ItemAttr::Description as u8 => {
                    item.description = String::from_utf8_lossy(attr_data).to_string();
                }
                x if x == ItemAttr::Speed as u8 => {
                    if attr_data.len() >= 2 {
                        item.speed = u16::from_le_bytes([attr_data[0], attr_data[1]]);
                    }
                }
                x if x == ItemAttr::Light as u8 || x == ItemAttr::Light2 as u8 => {
                    if attr_data.len() >= 2 {
                        item.light_level = attr_data[0];
                        item.light_color = attr_data[1];
                    }
                }
                x if x == ItemAttr::TopOrder as u8 => {
                    if !attr_data.is_empty() {
                        item.top_order = attr_data[0];
                    }
                }
                x if x == ItemAttr::WareId as u8 => {
                    if attr_data.len() >= 2 {
                        item.ware_id = u16::from_le_bytes([attr_data[0], attr_data[1]]);
                    }
                }
                _ => {
                    trace!("Unknown item attribute: 0x{:02X}", attr_type);
                }
            }
        }

        // Store item
        if item.server_id > 0 {
            self.client_to_server.insert(item.client_id, item.server_id);
            self.items.insert(item.server_id, item);
        }

        // Skip child nodes and find NODE_END
        while pos < data.len() {
            if data[pos] == NODE_START {
                pos += 1;
                pos = self.parse_item_node(data, pos)?;
            } else if data[pos] == NODE_END {
                pos += 1;
                break;
            } else {
                pos += 1;
            }
        }

        Ok(pos)
    }

    /// Get item by server ID
    pub fn get_item(&self, server_id: u16) -> Option<&OtbItem> {
        self.items.get(&server_id)
    }

    /// Get server ID from client ID
    pub fn get_server_id(&self, client_id: u16) -> Option<u16> {
        self.client_to_server.get(&client_id).copied()
    }

    /// Get all items
    pub fn items(&self) -> impl Iterator<Item = &OtbItem> {
        self.items.values()
    }

    /// Get item count
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Get version info
    pub fn version(&self) -> (u32, u32, u32) {
        (self.major_version, self.minor_version, self.build_number)
    }

    /// Check if item is ground
    pub fn is_ground(&self, server_id: u16) -> bool {
        self.items
            .get(&server_id)
            .map(|i| i.group == ItemGroup::Ground)
            .unwrap_or(false)
    }

    /// Check if item is walkable
    pub fn is_walkable(&self, server_id: u16) -> bool {
        self.items
            .get(&server_id)
            .map(|i| !i.flags.contains(ItemFlags::BLOCK_SOLID))
            .unwrap_or(true)
    }

    /// Check if item is moveable
    pub fn is_moveable(&self, server_id: u16) -> bool {
        self.items
            .get(&server_id)
            .map(|i| i.flags.contains(ItemFlags::MOVEABLE))
            .unwrap_or(false)
    }

    /// Check if item is stackable
    pub fn is_stackable(&self, server_id: u16) -> bool {
        self.items
            .get(&server_id)
            .map(|i| i.flags.contains(ItemFlags::STACKABLE))
            .unwrap_or(false)
    }

    /// Check if item is pickupable
    pub fn is_pickupable(&self, server_id: u16) -> bool {
        self.items
            .get(&server_id)
            .map(|i| i.flags.contains(ItemFlags::PICKUPABLE))
            .unwrap_or(false)
    }

    /// Get item speed (ground speed for ground tiles)
    pub fn get_speed(&self, server_id: u16) -> u16 {
        self.items.get(&server_id).map(|i| i.speed).unwrap_or(100)
    }
}
