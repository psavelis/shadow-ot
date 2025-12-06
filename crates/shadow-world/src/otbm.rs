//! OTBM (Open Tibia Binary Map) parser
//!
//! Handles loading and saving of map files in OTBM format.

use crate::item::Item;
use crate::map::Map;
use crate::position::Position;
use crate::tile::{Tile, TileFlags};
use crate::{Result, WorldError};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;
use tracing::{debug, info, warn};

/// OTBM file signature
const OTBM_SIGNATURE: u32 = 0;

/// OTBM node types
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OtbmNodeType {
    RootV1 = 0,
    RootV2 = 1,
    MapData = 2,
    ItemDef = 3,
    TileArea = 4,
    Tile = 5,
    Item = 6,
    TileSquare = 7,
    TileRef = 8,
    Spawns = 9,
    SpawnArea = 10,
    Monster = 11,
    Towns = 12,
    Town = 13,
    HouseTile = 14,
    Waypoints = 15,
    Waypoint = 16,
}

impl TryFrom<u8> for OtbmNodeType {
    type Error = WorldError;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(OtbmNodeType::RootV1),
            1 => Ok(OtbmNodeType::RootV2),
            2 => Ok(OtbmNodeType::MapData),
            3 => Ok(OtbmNodeType::ItemDef),
            4 => Ok(OtbmNodeType::TileArea),
            5 => Ok(OtbmNodeType::Tile),
            6 => Ok(OtbmNodeType::Item),
            7 => Ok(OtbmNodeType::TileSquare),
            8 => Ok(OtbmNodeType::TileRef),
            9 => Ok(OtbmNodeType::Spawns),
            10 => Ok(OtbmNodeType::SpawnArea),
            11 => Ok(OtbmNodeType::Monster),
            12 => Ok(OtbmNodeType::Towns),
            13 => Ok(OtbmNodeType::Town),
            14 => Ok(OtbmNodeType::HouseTile),
            15 => Ok(OtbmNodeType::Waypoints),
            16 => Ok(OtbmNodeType::Waypoint),
            _ => Err(WorldError::OtbmLoad(format!("Unknown node type: {}", value))),
        }
    }
}

/// OTBM item attributes
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OtbmAttribute {
    Description = 1,
    ExtFile = 2,
    TileFlags = 3,
    ActionId = 4,
    UniqueId = 5,
    Text = 6,
    Desc = 7,
    TeleDest = 8,
    Item = 9,
    DepotId = 10,
    ExtSpawnFile = 11,
    RuneCharges = 12,
    ExtHouseFile = 13,
    HouseDoorId = 14,
    Count = 15,
    Duration = 16,
    DecayState = 17,
    WrittenDate = 18,
    WrittenBy = 19,
    SleeperGuid = 20,
    SleepStart = 21,
    Charges = 22,
    ContainerItems = 23,
    Name = 24,
    Article = 25,
    PluralName = 26,
    Weight = 27,
    Attack = 28,
    Defense = 29,
    ExtraDefense = 30,
    Armor = 31,
    HitChance = 32,
    ShootRange = 33,
}

/// Node marker constants
const NODE_START: u8 = 0xFE;
const NODE_END: u8 = 0xFF;
const ESCAPE_CHAR: u8 = 0xFD;

/// OTBM parser
pub struct OtbmLoader {
    /// Buffer for reading
    buffer: Vec<u8>,
    /// Current position in buffer
    position: usize,
    /// OTBM version
    version: u32,
    /// Map width
    width: u16,
    /// Map height
    height: u16,
}

impl OtbmLoader {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            position: 0,
            version: 0,
            width: 0,
            height: 0,
        }
    }

    /// Load map from OTBM file
    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<Map> {
        info!("Loading OTBM map from: {}", path.as_ref().display());

        // Read file
        let file = File::open(&path)
            .map_err(|e| WorldError::OtbmLoad(format!("Failed to open file: {}", e)))?;

        let mut reader = BufReader::new(file);

        // Check for gzip compression
        let mut magic = [0u8; 2];
        reader.read_exact(&mut magic)
            .map_err(|e| WorldError::OtbmLoad(format!("Failed to read magic: {}", e)))?;

        reader.seek(SeekFrom::Start(0))
            .map_err(|e| WorldError::OtbmLoad(format!("Failed to seek: {}", e)))?;

        // Read entire file (decompress if needed)
        if magic == [0x1f, 0x8b] {
            // Gzip compressed
            let mut decoder = GzDecoder::new(reader);
            self.buffer.clear();
            decoder.read_to_end(&mut self.buffer)
                .map_err(|e| WorldError::OtbmLoad(format!("Failed to decompress: {}", e)))?;
        } else {
            // Uncompressed
            self.buffer.clear();
            reader.read_to_end(&mut self.buffer)
                .map_err(|e| WorldError::OtbmLoad(format!("Failed to read file: {}", e)))?;
        }

        self.position = 0;

        // Read signature
        let signature = self.read_u32()?;
        if signature != OTBM_SIGNATURE {
            return Err(WorldError::OtbmLoad(format!(
                "Invalid OTBM signature: expected {}, got {}",
                OTBM_SIGNATURE, signature
            )));
        }

        // Parse root node
        self.parse_root()
    }

    /// Parse the root node
    fn parse_root(&mut self) -> Result<Map> {
        // Read node start
        let marker = self.read_u8()?;
        if marker != NODE_START {
            return Err(WorldError::OtbmLoad("Expected node start".to_string()));
        }

        // Read node type
        let node_type = self.read_u8()?;
        let root_type = OtbmNodeType::try_from(node_type)?;

        self.version = match root_type {
            OtbmNodeType::RootV1 => 1,
            OtbmNodeType::RootV2 => 2,
            _ => return Err(WorldError::OtbmLoad(format!("Invalid root node type: {:?}", root_type))),
        };

        // Read header
        let _version = self.read_u32()?;
        self.width = self.read_u16()?;
        self.height = self.read_u16()?;
        let _items_major = self.read_u32()?;
        let _items_minor = self.read_u32()?;

        info!("OTBM version: {}, size: {}x{}", self.version, self.width, self.height);

        let mut map = Map::new("Loaded Map".to_string());
        map.width = self.width;
        map.height = self.height;

        // Parse children
        self.parse_children(&mut map)?;

        // Read node end
        let end_marker = self.read_u8()?;
        if end_marker != NODE_END {
            return Err(WorldError::OtbmLoad("Expected node end".to_string()));
        }

        Ok(map)
    }

    /// Parse child nodes
    fn parse_children(&mut self, map: &mut Map) -> Result<()> {
        loop {
            if self.position >= self.buffer.len() {
                break;
            }

            let marker = self.peek_u8()?;
            if marker == NODE_END {
                break;
            }

            if marker == NODE_START {
                self.read_u8()?; // consume marker
                self.parse_node(map)?;
            } else {
                break;
            }
        }

        Ok(())
    }

    /// Parse a single node
    fn parse_node(&mut self, map: &mut Map) -> Result<()> {
        let node_type = self.read_u8()?;
        let node_type = OtbmNodeType::try_from(node_type)?;

        match node_type {
            OtbmNodeType::MapData => self.parse_map_data(map)?,
            OtbmNodeType::TileArea => self.parse_tile_area(map)?,
            OtbmNodeType::Towns => self.parse_towns(map)?,
            OtbmNodeType::Waypoints => self.parse_waypoints(map)?,
            _ => {
                debug!("Skipping node type: {:?}", node_type);
                self.skip_node()?;
            }
        }

        // Read node end
        let end_marker = self.read_u8()?;
        if end_marker != NODE_END {
            return Err(WorldError::OtbmLoad("Expected node end".to_string()));
        }

        Ok(())
    }

    /// Parse map data node
    fn parse_map_data(&mut self, map: &mut Map) -> Result<()> {
        // Read attributes
        while self.position < self.buffer.len() {
            let attr = self.peek_u8()?;
            if attr == NODE_START || attr == NODE_END {
                break;
            }

            self.read_u8()?; // consume attribute type

            match attr {
                1 => {
                    // Description
                    let len = self.read_u16()? as usize;
                    let desc = self.read_string(len)?;
                    map.description = desc;
                }
                2 => {
                    // External file (spawns)
                    let len = self.read_u16()? as usize;
                    let _spawn_file = self.read_string(len)?;
                }
                13 => {
                    // External house file
                    let len = self.read_u16()? as usize;
                    let _house_file = self.read_string(len)?;
                }
                _ => {
                    warn!("Unknown map attribute: {}", attr);
                }
            }
        }

        // Parse children (tile areas, etc.)
        self.parse_children(map)?;

        Ok(())
    }

    /// Parse tile area node
    fn parse_tile_area(&mut self, map: &mut Map) -> Result<()> {
        // Read base position
        let base_x = self.read_u16()?;
        let base_y = self.read_u16()?;
        let base_z = self.read_u8()?;

        debug!("Parsing tile area at {}, {}, {}", base_x, base_y, base_z);

        // Parse tile nodes
        loop {
            if self.position >= self.buffer.len() {
                break;
            }

            let marker = self.peek_u8()?;
            if marker == NODE_END {
                break;
            }

            if marker == NODE_START {
                self.read_u8()?; // consume marker
                let node_type = self.read_u8()?;
                let node_type = OtbmNodeType::try_from(node_type)?;

                match node_type {
                    OtbmNodeType::Tile | OtbmNodeType::HouseTile => {
                        self.parse_tile(map, base_x, base_y, base_z, node_type == OtbmNodeType::HouseTile)?;
                    }
                    _ => {
                        self.skip_node()?;
                    }
                }

                // Read node end
                let end = self.read_u8()?;
                if end != NODE_END {
                    return Err(WorldError::OtbmLoad("Expected node end".to_string()));
                }
            } else {
                break;
            }
        }

        Ok(())
    }

    /// Parse a single tile
    fn parse_tile(
        &mut self,
        map: &mut Map,
        base_x: u16,
        base_y: u16,
        base_z: u8,
        is_house_tile: bool,
    ) -> Result<()> {
        let offset_x = self.read_u8()? as u16;
        let offset_y = self.read_u8()? as u16;

        let x = base_x + offset_x;
        let y = base_y + offset_y;
        let pos = Position::new(x, y, base_z);

        let mut tile = Tile::new(pos);
        let mut house_id = None;

        // Parse tile attributes
        while self.position < self.buffer.len() {
            let attr = self.peek_u8()?;
            if attr == NODE_START || attr == NODE_END {
                break;
            }

            self.read_u8()?; // consume attribute type

            match attr {
                3 => {
                    // Tile flags
                    let flags = self.read_u32()?;
                    tile.flags = TileFlags::from_bits(flags);
                }
                4 => {
                    // Action ID
                    let _action_id = self.read_u16()?;
                }
                5 => {
                    // Unique ID
                    let _unique_id = self.read_u16()?;
                }
                9 => {
                    // Item (ground)
                    let item_id = self.read_u16()?;
                    tile.set_ground(Item::new(item_id));
                }
                14 => {
                    // House door ID
                    let _door_id = self.read_u8()?;
                }
                _ => {
                    // Unknown attribute, try to skip
                    warn!("Unknown tile attribute: {}", attr);
                }
            }
        }

        if is_house_tile {
            // Read house ID (part of HouseTile node header)
            house_id = Some(self.read_u32().unwrap_or(0));
        }

        // Parse item nodes
        loop {
            if self.position >= self.buffer.len() {
                break;
            }

            let marker = self.peek_u8()?;
            if marker == NODE_END {
                break;
            }

            if marker == NODE_START {
                self.read_u8()?; // consume marker
                let node_type = self.read_u8()?;
                let node_type = OtbmNodeType::try_from(node_type)?;

                if node_type == OtbmNodeType::Item {
                    let item = self.parse_item()?;
                    if item.item_type_id != 0 {
                        // Check if it's a ground item
                        if tile.ground.is_none() {
                            tile.set_ground(item);
                        } else {
                            tile.add_item(item);
                        }
                    }
                } else {
                    self.skip_node()?;
                }

                // Read node end
                let end = self.read_u8()?;
                if end != NODE_END {
                    return Err(WorldError::OtbmLoad("Expected node end".to_string()));
                }
            } else {
                break;
            }
        }

        // Set house ID if this is a house tile
        if let Some(hid) = house_id {
            tile.house_id = Some(hid);
            map.set_house_tile(pos, hid);
        }

        // Add tile to map using sync method (in real impl would be async)
        // For now, we'll need to handle this differently
        // This is a simplified version

        Ok(())
    }

    /// Parse an item node
    fn parse_item(&mut self) -> Result<Item> {
        let item_id = self.read_u16()?;
        let mut item = Item::new(item_id);

        // Parse item attributes
        while self.position < self.buffer.len() {
            let attr = self.peek_u8()?;
            if attr == NODE_START || attr == NODE_END {
                break;
            }

            self.read_u8()?; // consume attribute type

            match attr {
                4 => {
                    // Action ID
                    item.action_id = self.read_u16()?;
                }
                5 => {
                    // Unique ID
                    item.unique_action_id = self.read_u16()?;
                }
                6 => {
                    // Text
                    let len = self.read_u16()? as usize;
                    let text = self.read_string(len)?;
                    item.text = Some(text);
                }
                12 => {
                    // Rune charges
                    let charges = self.read_u8()?;
                    item.charges = Some(charges as u16);
                }
                15 => {
                    // Count
                    item.count = self.read_u8()? as u16;
                }
                16 => {
                    // Duration
                    item.duration = Some(self.read_u32()?);
                }
                18 => {
                    // Written date
                    item.written_date = Some(self.read_u32()? as i64);
                }
                19 => {
                    // Written by
                    let len = self.read_u16()? as usize;
                    let writer = self.read_string(len)?;
                    item.written_by = Some(writer);
                }
                22 => {
                    // Charges
                    item.charges = Some(self.read_u16()?);
                }
                _ => {
                    warn!("Unknown item attribute: {}", attr);
                }
            }
        }

        Ok(item)
    }

    /// Parse towns node
    fn parse_towns(&mut self, map: &mut Map) -> Result<()> {
        loop {
            if self.position >= self.buffer.len() {
                break;
            }

            let marker = self.peek_u8()?;
            if marker == NODE_END {
                break;
            }

            if marker == NODE_START {
                self.read_u8()?; // consume marker
                let node_type = self.read_u8()?;
                let node_type = OtbmNodeType::try_from(node_type)?;

                if node_type == OtbmNodeType::Town {
                    let town_id = self.read_u32()?;
                    let name_len = self.read_u16()? as usize;
                    let name = self.read_string(name_len)?;
                    let temple_x = self.read_u16()?;
                    let temple_y = self.read_u16()?;
                    let temple_z = self.read_u8()?;

                    debug!("Town {}: {} at ({}, {}, {})", town_id, name, temple_x, temple_y, temple_z);

                    // Store town info (simplified)
                    if town_id == 1 || map.temple_position == Position::default() {
                        map.temple_position = Position::new(temple_x, temple_y, temple_z);
                    }
                }

                // Read node end
                let end = self.read_u8()?;
                if end != NODE_END {
                    return Err(WorldError::OtbmLoad("Expected node end".to_string()));
                }
            } else {
                break;
            }
        }

        Ok(())
    }

    /// Parse waypoints node
    fn parse_waypoints(&mut self, map: &mut Map) -> Result<()> {
        loop {
            if self.position >= self.buffer.len() {
                break;
            }

            let marker = self.peek_u8()?;
            if marker == NODE_END {
                break;
            }

            if marker == NODE_START {
                self.read_u8()?; // consume marker
                let node_type = self.read_u8()?;
                let node_type = OtbmNodeType::try_from(node_type)?;

                if node_type == OtbmNodeType::Waypoint {
                    let name_len = self.read_u16()? as usize;
                    let name = self.read_string(name_len)?;
                    let x = self.read_u16()?;
                    let y = self.read_u16()?;
                    let z = self.read_u8()?;

                    debug!("Waypoint '{}' at ({}, {}, {})", name, x, y, z);
                    map.add_waypoint(name, Position::new(x, y, z));
                }

                // Read node end
                let end = self.read_u8()?;
                if end != NODE_END {
                    return Err(WorldError::OtbmLoad("Expected node end".to_string()));
                }
            } else {
                break;
            }
        }

        Ok(())
    }

    /// Skip current node and its children
    fn skip_node(&mut self) -> Result<()> {
        let mut depth = 1;

        while depth > 0 && self.position < self.buffer.len() {
            let byte = self.read_u8()?;
            match byte {
                NODE_START => depth += 1,
                NODE_END => depth -= 1,
                ESCAPE_CHAR => {
                    self.position += 1; // skip escaped byte
                }
                _ => {}
            }
        }

        Ok(())
    }

    // Helper methods for reading data

    fn read_u8(&mut self) -> Result<u8> {
        if self.position >= self.buffer.len() {
            return Err(WorldError::OtbmLoad("Unexpected end of file".to_string()));
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
            return Err(WorldError::OtbmLoad("Unexpected end of file".to_string()));
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
        String::from_utf8(bytes)
            .map_err(|e| WorldError::OtbmLoad(format!("Invalid string: {}", e)))
    }
}

impl Default for OtbmLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_type_conversion() {
        assert_eq!(OtbmNodeType::try_from(0).unwrap(), OtbmNodeType::RootV1);
        assert_eq!(OtbmNodeType::try_from(5).unwrap(), OtbmNodeType::Tile);
    }
}
