//! Position and coordinate system

use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

/// A position in the game world
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Position {
    pub x: u16,
    pub y: u16,
    pub z: u8,
}

impl Position {
    pub const fn new(x: u16, y: u16, z: u8) -> Self {
        Self { x, y, z }
    }

    /// Create position from inventory slot
    pub fn from_slot(slot: u8) -> Self {
        Self {
            x: 0xFFFF,
            y: slot as u16,
            z: 0,
        }
    }

    /// Create position from container
    pub fn from_container(container_id: u8, slot: u8) -> Self {
        Self {
            x: 0xFFFF,
            y: (0x40 | container_id) as u16,
            z: slot,
        }
    }

    /// Check if this is an inventory position
    pub fn is_inventory(&self) -> bool {
        self.x == 0xFFFF && self.y < 0x40
    }

    /// Check if this is a container position
    pub fn is_container(&self) -> bool {
        self.x == 0xFFFF && (self.y & 0x40) != 0
    }

    /// Check if this is a ground position
    pub fn is_ground(&self) -> bool {
        self.x != 0xFFFF
    }

    /// Get inventory slot if this is an inventory position
    pub fn get_slot(&self) -> Option<u8> {
        if self.is_inventory() {
            Some(self.y as u8)
        } else {
            None
        }
    }

    /// Get container info if this is a container position
    pub fn get_container(&self) -> Option<(u8, u8)> {
        if self.is_container() {
            Some(((self.y & 0x0F) as u8, self.z))
        } else {
            None
        }
    }

    /// Calculate distance to another position (Chebyshev distance)
    pub fn distance_to(&self, other: &Position) -> u32 {
        let dx = (self.x as i32 - other.x as i32).unsigned_abs();
        let dy = (self.y as i32 - other.y as i32).unsigned_abs();
        dx.max(dy)
    }

    /// Calculate Manhattan distance
    pub fn manhattan_distance(&self, other: &Position) -> u32 {
        let dx = (self.x as i32 - other.x as i32).unsigned_abs();
        let dy = (self.y as i32 - other.y as i32).unsigned_abs();
        dx + dy
    }

    /// Check if position is adjacent (including diagonals)
    pub fn is_adjacent(&self, other: &Position) -> bool {
        self.z == other.z && self.distance_to(other) == 1
    }

    /// Check if position is in range
    pub fn in_range(&self, other: &Position, range: u32) -> bool {
        self.z == other.z && self.distance_to(other) <= range
    }

    /// Get direction to another position
    pub fn direction_to(&self, other: &Position) -> Direction {
        let dx = other.x as i32 - self.x as i32;
        let dy = other.y as i32 - self.y as i32;

        match (dx.signum(), dy.signum()) {
            (0, -1) => Direction::North,
            (1, -1) => Direction::NorthEast,
            (1, 0) => Direction::East,
            (1, 1) => Direction::SouthEast,
            (0, 1) => Direction::South,
            (-1, 1) => Direction::SouthWest,
            (-1, 0) => Direction::West,
            (-1, -1) => Direction::NorthWest,
            _ => Direction::South, // Same position
        }
    }

    /// Move in a direction
    pub fn moved(&self, direction: Direction) -> Self {
        let (dx, dy) = direction.offset();
        Self {
            x: (self.x as i32 + dx).max(0) as u16,
            y: (self.y as i32 + dy).max(0) as u16,
            z: self.z,
        }
    }

    /// Get neighbors in all 8 directions
    pub fn neighbors(&self) -> Vec<Position> {
        Direction::all()
            .iter()
            .map(|dir| self.moved(*dir))
            .collect()
    }

    /// Check if position is valid
    pub fn is_valid(&self) -> bool {
        self.z <= crate::MAP_MAX_Z
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

/// Direction enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
    NorthEast = 4,
    SouthEast = 5,
    SouthWest = 6,
    NorthWest = 7,
}

impl Direction {
    /// Get all directions
    pub fn all() -> &'static [Direction; 8] {
        &[
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
            Direction::NorthEast,
            Direction::SouthEast,
            Direction::SouthWest,
            Direction::NorthWest,
        ]
    }

    /// Get cardinal directions only
    pub fn cardinal() -> &'static [Direction; 4] {
        &[
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }

    /// Get diagonal directions only
    pub fn diagonal() -> &'static [Direction; 4] {
        &[
            Direction::NorthEast,
            Direction::SouthEast,
            Direction::SouthWest,
            Direction::NorthWest,
        ]
    }

    /// Get the offset for this direction
    pub fn offset(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::NorthEast => (1, -1),
            Direction::SouthEast => (1, 1),
            Direction::SouthWest => (-1, 1),
            Direction::NorthWest => (-1, -1),
        }
    }

    /// Get the opposite direction
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::NorthEast => Direction::SouthWest,
            Direction::SouthEast => Direction::NorthWest,
            Direction::SouthWest => Direction::NorthEast,
            Direction::NorthWest => Direction::SouthEast,
        }
    }

    /// Check if this is a diagonal direction
    pub fn is_diagonal(&self) -> bool {
        matches!(
            self,
            Direction::NorthEast
                | Direction::SouthEast
                | Direction::SouthWest
                | Direction::NorthWest
        )
    }
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            4 => Direction::NorthEast,
            5 => Direction::SouthEast,
            6 => Direction::SouthWest,
            7 => Direction::NorthWest,
            _ => Direction::South,
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::South
    }
}

/// Area of effect structure
#[derive(Debug, Clone)]
pub struct Area {
    pub center: Position,
    pub positions: Vec<Position>,
}

impl Area {
    /// Create a circular area
    pub fn circle(center: Position, radius: u8) -> Self {
        let mut positions = Vec::new();
        let r = radius as i32;

        for dx in -r..=r {
            for dy in -r..=r {
                if dx * dx + dy * dy <= r * r {
                    let x = (center.x as i32 + dx).max(0) as u16;
                    let y = (center.y as i32 + dy).max(0) as u16;
                    positions.push(Position::new(x, y, center.z));
                }
            }
        }

        Self { center, positions }
    }

    /// Create a square area
    pub fn square(center: Position, size: u8) -> Self {
        let mut positions = Vec::new();
        let half = (size / 2) as i32;

        for dx in -half..=half {
            for dy in -half..=half {
                let x = (center.x as i32 + dx).max(0) as u16;
                let y = (center.y as i32 + dy).max(0) as u16;
                positions.push(Position::new(x, y, center.z));
            }
        }

        Self { center, positions }
    }

    /// Create a beam/line area
    pub fn beam(from: Position, to: Position, width: u8) -> Self {
        let mut positions = Vec::new();
        let direction = from.direction_to(&to);

        let mut current = from;
        while current != to && positions.len() < 100 {
            for offset in -(width as i32 / 2)..=(width as i32 / 2) {
                let perpendicular = match direction {
                    Direction::North | Direction::South => (offset, 0),
                    Direction::East | Direction::West => (0, offset),
                    _ => (0, 0),
                };
                let x = (current.x as i32 + perpendicular.0).max(0) as u16;
                let y = (current.y as i32 + perpendicular.1).max(0) as u16;
                positions.push(Position::new(x, y, current.z));
            }
            current = current.moved(direction);
        }

        Self {
            center: from,
            positions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let a = Position::new(100, 100, 7);
        let b = Position::new(103, 104, 7);
        assert_eq!(a.distance_to(&b), 4);
    }

    #[test]
    fn test_direction() {
        let a = Position::new(100, 100, 7);
        let b = Position::new(101, 99, 7);
        assert_eq!(a.direction_to(&b), Direction::NorthEast);
    }

    #[test]
    fn test_moved() {
        let pos = Position::new(100, 100, 7);
        let moved = pos.moved(Direction::North);
        assert_eq!(moved, Position::new(100, 99, 7));
    }
}
