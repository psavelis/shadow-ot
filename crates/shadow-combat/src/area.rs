//! Area effects - spell areas and damage zones

use shadow_world::position::{Direction, Position};
use serde::{Deserialize, Serialize};

/// Area effect types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AreaType {
    /// Single target
    Single,
    /// Circular area
    Circle { radius: u8 },
    /// Square area
    Square { size: u8 },
    /// Beam/line in direction
    Beam { length: u8, width: u8 },
    /// Wave/cone in direction
    Wave { length: u8, spread: u8 },
    /// Ring around caster (not center)
    Ring { inner_radius: u8, outer_radius: u8 },
    /// Cross pattern
    Cross { length: u8 },
    /// Custom from matrix
    Custom { width: u8, height: u8 },
}

/// Area effect definition
#[derive(Debug, Clone)]
pub struct AreaEffect {
    pub area_type: AreaType,
    pub center: Position,
    pub direction: Option<Direction>,
    pub positions: Vec<Position>,
    pub damage_map: Vec<(Position, u8)>, // Position and damage percentage
}

impl AreaEffect {
    /// Create area effect
    pub fn new(area_type: AreaType, center: Position, direction: Option<Direction>) -> Self {
        let mut effect = Self {
            area_type,
            center,
            direction,
            positions: Vec::new(),
            damage_map: Vec::new(),
        };
        effect.calculate_positions();
        effect
    }

    /// Calculate affected positions based on area type
    fn calculate_positions(&mut self) {
        self.positions.clear();
        self.damage_map.clear();

        match self.area_type {
            AreaType::Single => {
                self.positions.push(self.center);
                self.damage_map.push((self.center, 100));
            }

            AreaType::Circle { radius } => {
                let r = radius as i32;
                for dy in -r..=r {
                    for dx in -r..=r {
                        if dx * dx + dy * dy <= r * r {
                            let pos = self.offset_position(dx, dy);
                            let distance = ((dx * dx + dy * dy) as f32).sqrt();
                            let damage_pct = self.calculate_distance_damage(distance, radius as f32);
                            self.positions.push(pos);
                            self.damage_map.push((pos, damage_pct));
                        }
                    }
                }
            }

            AreaType::Square { size } => {
                let half = (size / 2) as i32;
                for dy in -half..=half {
                    for dx in -half..=half {
                        let pos = self.offset_position(dx, dy);
                        self.positions.push(pos);
                        self.damage_map.push((pos, 100));
                    }
                }
            }

            AreaType::Beam { length, width } => {
                let dir = self.direction.unwrap_or(Direction::North);
                let (dx, dy) = dir.offset();
                let half_width = (width / 2) as i32;

                for dist in 1..=length as i32 {
                    for offset in -half_width..=half_width {
                        let (px, py) = match dir {
                            Direction::North | Direction::South => {
                                (self.center.x as i32 + offset, self.center.y as i32 + (dy * dist))
                            }
                            Direction::East | Direction::West => {
                                (self.center.x as i32 + (dx * dist), self.center.y as i32 + offset)
                            }
                            _ => {
                                // Diagonal beams
                                (self.center.x as i32 + (dx * dist), self.center.y as i32 + (dy * dist))
                            }
                        };

                        if px >= 0 && py >= 0 {
                            let pos = Position::new(px as u16, py as u16, self.center.z);
                            self.positions.push(pos);
                            self.damage_map.push((pos, 100));
                        }
                    }
                }
            }

            AreaType::Wave { length, spread } => {
                let dir = self.direction.unwrap_or(Direction::North);
                let (dx, dy) = dir.offset();

                for dist in 1..=length as i32 {
                    // Width increases with distance
                    let width = ((dist as f32 / length as f32) * spread as f32).ceil() as i32;

                    for offset in -width..=width {
                        let (px, py) = self.calculate_wave_position(dx, dy, dist, offset, dir);

                        if px >= 0 && py >= 0 {
                            let pos = Position::new(px as u16, py as u16, self.center.z);
                            let damage_pct = self.calculate_distance_damage(dist as f32, length as f32);
                            self.positions.push(pos);
                            self.damage_map.push((pos, damage_pct));
                        }
                    }
                }
            }

            AreaType::Ring { inner_radius, outer_radius } => {
                let outer = outer_radius as i32;
                let inner = inner_radius as i32;

                for dy in -outer..=outer {
                    for dx in -outer..=outer {
                        let dist_sq = dx * dx + dy * dy;
                        if dist_sq <= outer * outer && dist_sq >= inner * inner {
                            let pos = self.offset_position(dx, dy);
                            self.positions.push(pos);
                            self.damage_map.push((pos, 100));
                        }
                    }
                }
            }

            AreaType::Cross { length } => {
                // Center
                self.positions.push(self.center);
                self.damage_map.push((self.center, 100));

                // Cardinal directions
                for dir in Direction::cardinal() {
                    for dist in 1..=length as i32 {
                        let (dx, dy) = dir.offset();
                        let pos = self.offset_position(dx * dist, dy * dist);
                        self.positions.push(pos);
                        self.damage_map.push((pos, 100));
                    }
                }
            }

            AreaType::Custom { .. } => {
                // Custom areas need to be set manually via set_positions
            }
        }
    }

    /// Calculate wave position
    fn calculate_wave_position(&self, dx: i32, dy: i32, dist: i32, offset: i32, dir: Direction) -> (i32, i32) {
        match dir {
            Direction::North => (self.center.x as i32 + offset, self.center.y as i32 - dist),
            Direction::South => (self.center.x as i32 + offset, self.center.y as i32 + dist),
            Direction::East => (self.center.x as i32 + dist, self.center.y as i32 + offset),
            Direction::West => (self.center.x as i32 - dist, self.center.y as i32 + offset),
            Direction::NorthEast => (self.center.x as i32 + dist, self.center.y as i32 - dist + offset),
            Direction::SouthEast => (self.center.x as i32 + dist, self.center.y as i32 + dist + offset),
            Direction::SouthWest => (self.center.x as i32 - dist, self.center.y as i32 + dist + offset),
            Direction::NorthWest => (self.center.x as i32 - dist, self.center.y as i32 - dist + offset),
        }
    }

    /// Offset position from center
    fn offset_position(&self, dx: i32, dy: i32) -> Position {
        Position::new(
            (self.center.x as i32 + dx).max(0) as u16,
            (self.center.y as i32 + dy).max(0) as u16,
            self.center.z,
        )
    }

    /// Calculate damage percentage based on distance
    fn calculate_distance_damage(&self, distance: f32, max_distance: f32) -> u8 {
        // Full damage at center, decreasing towards edge
        let ratio = 1.0 - (distance / max_distance).min(1.0) * 0.3; // 30% reduction at edge
        (ratio * 100.0) as u8
    }

    /// Set custom positions
    pub fn set_positions(&mut self, positions: Vec<Position>) {
        self.positions = positions.clone();
        self.damage_map = positions.into_iter().map(|p| (p, 100)).collect();
    }

    /// Get positions
    pub fn get_positions(&self) -> &[Position] {
        &self.positions
    }

    /// Get damage at position
    pub fn get_damage_percent(&self, pos: &Position) -> u8 {
        self.damage_map
            .iter()
            .find(|(p, _)| p == pos)
            .map(|(_, pct)| *pct)
            .unwrap_or(0)
    }

    /// Check if position is affected
    pub fn contains(&self, pos: &Position) -> bool {
        self.positions.contains(pos)
    }

    /// Get number of affected positions
    pub fn size(&self) -> usize {
        self.positions.len()
    }
}

/// Predefined area matrices
pub mod areas {
    /// Great fireball area
    pub const GREAT_FIREBALL: [[u8; 5]; 5] = [
        [0, 1, 1, 1, 0],
        [1, 1, 1, 1, 1],
        [1, 1, 3, 1, 1],
        [1, 1, 1, 1, 1],
        [0, 1, 1, 1, 0],
    ];

    /// Explosion area
    pub const EXPLOSION: [[u8; 5]; 5] = [
        [0, 0, 1, 0, 0],
        [0, 1, 1, 1, 0],
        [1, 1, 3, 1, 1],
        [0, 1, 1, 1, 0],
        [0, 0, 1, 0, 0],
    ];

    /// Energy beam area (north)
    pub const ENERGY_BEAM_N: [[u8; 3]; 8] = [
        [0, 0, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 3, 0],
    ];

    /// Wrath of nature (large area)
    pub const WRATH_OF_NATURE: [[u8; 7]; 7] = [
        [0, 0, 1, 1, 1, 0, 0],
        [0, 1, 1, 1, 1, 1, 0],
        [1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 3, 1, 1, 1],
        [1, 1, 1, 1, 1, 1, 1],
        [0, 1, 1, 1, 1, 1, 0],
        [0, 0, 1, 1, 1, 0, 0],
    ];
}

/// Parse area from matrix
pub fn parse_area_matrix(matrix: &[&[u8]], center: Position, direction: Option<Direction>) -> AreaEffect {
    let height = matrix.len();
    let width = if height > 0 { matrix[0].len() } else { 0 };

    let mut effect = AreaEffect {
        area_type: AreaType::Custom {
            width: width as u8,
            height: height as u8,
        },
        center,
        direction,
        positions: Vec::new(),
        damage_map: Vec::new(),
    };

    // Find center marker (3) or use geometric center
    let mut center_x = width / 2;
    let mut center_y = height / 2;

    for (y, row) in matrix.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            if val == 3 {
                center_x = x;
                center_y = y;
            }
        }
    }

    // Add affected positions
    for (y, row) in matrix.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            if val > 0 {
                let dx = x as i32 - center_x as i32;
                let dy = y as i32 - center_y as i32;

                let pos = Position::new(
                    (center.x as i32 + dx).max(0) as u16,
                    (center.y as i32 + dy).max(0) as u16,
                    center.z,
                );

                effect.positions.push(pos);
                effect.damage_map.push((pos, 100));
            }
        }
    }

    effect
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_area() {
        let center = Position::new(100, 100, 7);
        let area = AreaEffect::new(AreaType::Circle { radius: 3 }, center, None);

        assert!(area.contains(&center));
        assert!(area.positions.len() > 1);
    }

    #[test]
    fn test_beam_area() {
        let center = Position::new(100, 100, 7);
        let area = AreaEffect::new(
            AreaType::Beam { length: 5, width: 1 },
            center,
            Some(Direction::North),
        );

        assert!(!area.contains(&center)); // Beam starts after center
        assert_eq!(area.positions.len(), 5);
    }

    #[test]
    fn test_cross_area() {
        let center = Position::new(100, 100, 7);
        let area = AreaEffect::new(AreaType::Cross { length: 3 }, center, None);

        assert!(area.contains(&center));
        // Center + 4 directions * 3 length = 13
        assert_eq!(area.positions.len(), 13);
    }
}
