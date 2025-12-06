//! Sprite and animation types

use serde::{Deserialize, Serialize};

/// Sprite definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sprite {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub layers: u8,
    pub pattern_x: u8,
    pub pattern_y: u8,
    pub pattern_z: u8,
    pub frames: u8,
    pub sprite_ids: Vec<u32>,
}

impl Sprite {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            width: 1,
            height: 1,
            layers: 1,
            pattern_x: 1,
            pattern_y: 1,
            pattern_z: 1,
            frames: 1,
            sprite_ids: Vec::new(),
        }
    }

    /// Get total sprite count needed
    pub fn sprite_count(&self) -> usize {
        (self.width as usize)
            * (self.height as usize)
            * (self.layers as usize)
            * (self.pattern_x as usize)
            * (self.pattern_y as usize)
            * (self.pattern_z as usize)
            * (self.frames as usize)
    }

    /// Get sprite ID for specific frame/layer/pattern
    pub fn get_sprite_id(
        &self,
        frame: u8,
        layer: u8,
        pattern_x: u8,
        pattern_y: u8,
        pattern_z: u8,
        x: u32,
        y: u32,
    ) -> Option<u32> {
        let index = self.calculate_index(frame, layer, pattern_x, pattern_y, pattern_z, x, y)?;
        self.sprite_ids.get(index).copied()
    }

    fn calculate_index(
        &self,
        frame: u8,
        layer: u8,
        pattern_x: u8,
        pattern_y: u8,
        pattern_z: u8,
        x: u32,
        y: u32,
    ) -> Option<usize> {
        if x >= self.width
            || y >= self.height
            || layer >= self.layers
            || pattern_x >= self.pattern_x
            || pattern_y >= self.pattern_y
            || pattern_z >= self.pattern_z
            || frame >= self.frames
        {
            return None;
        }

        let idx = ((frame as usize)
            * (self.pattern_z as usize)
            * (self.pattern_y as usize)
            * (self.pattern_x as usize)
            * (self.layers as usize)
            * (self.height as usize)
            * (self.width as usize))
            + ((pattern_z as usize)
                * (self.pattern_y as usize)
                * (self.pattern_x as usize)
                * (self.layers as usize)
                * (self.height as usize)
                * (self.width as usize))
            + ((pattern_y as usize)
                * (self.pattern_x as usize)
                * (self.layers as usize)
                * (self.height as usize)
                * (self.width as usize))
            + ((pattern_x as usize)
                * (self.layers as usize)
                * (self.height as usize)
                * (self.width as usize))
            + ((layer as usize) * (self.height as usize) * (self.width as usize))
            + ((y as usize) * (self.width as usize))
            + (x as usize);

        Some(idx)
    }
}

/// Sprite sheet for texture atlases
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteSheet {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub sprites: Vec<SpriteSheetEntry>,
}

/// Entry in a sprite sheet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteSheetEntry {
    pub sprite_id: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl SpriteSheet {
    pub fn new(name: String, width: u32, height: u32) -> Self {
        Self {
            name,
            width,
            height,
            sprites: Vec::new(),
        }
    }

    pub fn add_sprite(&mut self, sprite_id: u32, x: u32, y: u32, width: u32, height: u32) {
        self.sprites.push(SpriteSheetEntry {
            sprite_id,
            x,
            y,
            width,
            height,
        });
    }

    pub fn get_sprite(&self, sprite_id: u32) -> Option<&SpriteSheetEntry> {
        self.sprites.iter().find(|s| s.sprite_id == sprite_id)
    }
}

/// Animation definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
    pub id: u32,
    pub frames: Vec<AnimationFrame>,
    pub loop_count: i32,
    pub async_animation: bool,
    pub start_frame: u8,
}

/// Single animation frame
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationFrame {
    pub sprite_id: u32,
    pub min_duration_ms: u32,
    pub max_duration_ms: u32,
}

impl Animation {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            frames: Vec::new(),
            loop_count: -1, // Infinite
            async_animation: false,
            start_frame: 0,
        }
    }

    pub fn add_frame(&mut self, sprite_id: u32, min_duration_ms: u32, max_duration_ms: u32) {
        self.frames.push(AnimationFrame {
            sprite_id,
            min_duration_ms,
            max_duration_ms,
        });
    }

    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    pub fn get_frame(&self, index: usize) -> Option<&AnimationFrame> {
        self.frames.get(index)
    }

    pub fn total_duration_ms(&self) -> u32 {
        self.frames
            .iter()
            .map(|f| (f.min_duration_ms + f.max_duration_ms) / 2)
            .sum()
    }
}

/// Frame group for complex animations (idle, walking, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameGroup {
    pub group_type: FrameGroupType,
    pub sprite: Sprite,
    pub animation: Option<Animation>,
}

/// Frame group types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrameGroupType {
    Idle = 0,
    Walking = 1,
}

impl FrameGroupType {
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => FrameGroupType::Walking,
            _ => FrameGroupType::Idle,
        }
    }
}
