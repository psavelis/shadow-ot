//! Pathfinding system using A* algorithm

use crate::map::Map;
use crate::position::{Direction, Position};
use crate::Result;
use pathfinding::prelude::astar;
use std::collections::HashSet;

/// Maximum path length to prevent infinite loops
pub const MAX_PATH_LENGTH: usize = 128;

/// Maximum nodes to explore
pub const MAX_NODES_EXPLORED: usize = 5000;

/// Pathfinding configuration
#[derive(Debug, Clone)]
pub struct PathfinderConfig {
    /// Maximum path length
    pub max_length: usize,
    /// Maximum nodes to explore
    pub max_nodes: usize,
    /// Allow diagonal movement
    pub allow_diagonal: bool,
    /// Extra cost for diagonal movement
    pub diagonal_cost: u32,
    /// Whether to check for creatures blocking
    pub check_creatures: bool,
    /// Whether to check for field items
    pub check_fields: bool,
    /// Positions to avoid (e.g., dangerous tiles)
    pub avoid_positions: HashSet<Position>,
}

impl Default for PathfinderConfig {
    fn default() -> Self {
        Self {
            max_length: MAX_PATH_LENGTH,
            max_nodes: MAX_NODES_EXPLORED,
            allow_diagonal: true,
            diagonal_cost: 3,
            check_creatures: true,
            check_fields: true,
            avoid_positions: HashSet::new(),
        }
    }
}

/// Pathfinding result
#[derive(Debug, Clone)]
pub struct PathResult {
    /// The path as a list of directions
    pub directions: Vec<Direction>,
    /// The path as a list of positions
    pub positions: Vec<Position>,
    /// Total cost of the path
    pub cost: u32,
    /// Whether the destination was reached
    pub found: bool,
}

impl PathResult {
    pub fn empty() -> Self {
        Self {
            directions: Vec::new(),
            positions: Vec::new(),
            cost: 0,
            found: false,
        }
    }

    pub fn not_found() -> Self {
        Self::empty()
    }

    pub fn is_empty(&self) -> bool {
        self.directions.is_empty()
    }

    pub fn len(&self) -> usize {
        self.directions.len()
    }
}

/// Pathfinder struct
pub struct Pathfinder {
    config: PathfinderConfig,
}

impl Pathfinder {
    pub fn new() -> Self {
        Self {
            config: PathfinderConfig::default(),
        }
    }

    pub fn with_config(config: PathfinderConfig) -> Self {
        Self { config }
    }

    /// Find path between two positions
    pub async fn find_path(
        &self,
        map: &Map,
        from: Position,
        to: Position,
    ) -> PathResult {
        if from == to {
            return PathResult {
                directions: Vec::new(),
                positions: vec![from],
                cost: 0,
                found: true,
            };
        }

        // Check if destination floor matches
        if from.z != to.z {
            return PathResult::not_found();
        }

        // Check if destination is walkable
        if !map.is_walkable(&to).await {
            return PathResult::not_found();
        }

        // Use A* algorithm
        let result = self.astar_search(map, from, to).await;

        result.unwrap_or_else(PathResult::not_found)
    }

    /// A* search implementation
    async fn astar_search(
        &self,
        map: &Map,
        start: Position,
        goal: Position,
    ) -> Option<PathResult> {
        // Node for A* with visited tracking
        let mut visited_count = 0;
        let max_nodes = self.config.max_nodes;

        // Clone config for the closure
        let allow_diagonal = self.config.allow_diagonal;
        let diagonal_cost = self.config.diagonal_cost;
        let check_creatures = self.config.check_creatures;
        let avoid_positions = self.config.avoid_positions.clone();

        // We need to collect walkable neighbors synchronously
        // Pre-compute a neighborhood cache
        let positions_to_check = self.get_search_area(&start, &goal);
        let mut walkable_cache: HashSet<Position> = HashSet::new();

        for pos in positions_to_check {
            if map.is_walkable(&pos).await {
                // Check creatures if needed
                if check_creatures {
                    if let Some(tile) = map.get_tile(&pos).await {
                        let tile = tile.read().await;
                        if pos != goal && tile.has_creatures() {
                            continue;
                        }
                    }
                }
                if !avoid_positions.contains(&pos) {
                    walkable_cache.insert(pos);
                }
            }
        }

        // Always include start and goal
        walkable_cache.insert(start);

        // Now run A* with the cache
        let result = astar(
            &start,
            |pos| {
                let mut neighbors = Vec::new();
                visited_count += 1;

                if visited_count > max_nodes {
                    return neighbors;
                }

                let directions: &[Direction] = if allow_diagonal {
                    Direction::all()
                } else {
                    Direction::cardinal()
                };

                for dir in directions {
                    let next = pos.moved(*dir);

                    if walkable_cache.contains(&next) {
                        let cost = if dir.is_diagonal() {
                            diagonal_cost
                        } else {
                            1
                        };
                        neighbors.push((next, cost));
                    }
                }

                neighbors
            },
            |pos| pos.distance_to(&goal),
            |pos| *pos == goal,
        );

        result.map(|(path, cost)| {
            let directions = path
                .windows(2)
                .map(|w| w[0].direction_to(&w[1]))
                .collect();

            PathResult {
                directions,
                positions: path,
                cost,
                found: true,
            }
        })
    }

    /// Get the search area between two positions
    fn get_search_area(&self, start: &Position, goal: &Position) -> Vec<Position> {
        let min_x = start.x.min(goal.x).saturating_sub(10);
        let max_x = start.x.max(goal.x).saturating_add(10);
        let min_y = start.y.min(goal.y).saturating_sub(10);
        let max_y = start.y.max(goal.y).saturating_add(10);

        let mut positions = Vec::new();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                positions.push(Position::new(x, y, start.z));
            }
        }
        positions
    }

    /// Find path to closest position in a list
    pub async fn find_path_to_nearest(
        &self,
        map: &Map,
        from: Position,
        targets: &[Position],
    ) -> PathResult {
        let mut best_result = PathResult::not_found();
        let mut best_cost = u32::MAX;

        for target in targets {
            let result = self.find_path(map, from, *target).await;
            if result.found && result.cost < best_cost {
                best_cost = result.cost;
                best_result = result;
            }
        }

        best_result
    }

    /// Check if there's a clear line of sight between two positions
    pub async fn has_line_of_sight(
        &self,
        map: &Map,
        from: &Position,
        to: &Position,
    ) -> bool {
        if from.z != to.z {
            return false;
        }

        let positions = self.get_line_positions(from, to);

        for pos in positions.iter().skip(1) {
            if map.blocks_projectile(pos).await {
                return false;
            }
        }

        true
    }

    /// Get positions along a line (Bresenham's algorithm)
    pub fn get_line_positions(&self, from: &Position, to: &Position) -> Vec<Position> {
        let mut positions = Vec::new();

        let dx = (to.x as i32 - from.x as i32).abs();
        let dy = (to.y as i32 - from.y as i32).abs();
        let sx = if from.x < to.x { 1i32 } else { -1i32 };
        let sy = if from.y < to.y { 1i32 } else { -1i32 };
        let mut err = dx - dy;

        let mut x = from.x as i32;
        let mut y = from.y as i32;
        let goal_x = to.x as i32;
        let goal_y = to.y as i32;

        loop {
            positions.push(Position::new(x as u16, y as u16, from.z));

            if x == goal_x && y == goal_y {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }

        positions
    }

    /// Get positions in a cone/fan area
    pub fn get_cone_positions(
        &self,
        from: &Position,
        direction: Direction,
        range: u8,
        spread: u8,
    ) -> Vec<Position> {
        let mut positions = Vec::new();
        let (dx, dy) = direction.offset();

        for dist in 1..=range {
            // Calculate width at this distance
            let width = (dist as f32 * (spread as f32 / range as f32)).ceil() as i32;

            for offset in -width..=width {
                let (px, py) = match direction {
                    Direction::North | Direction::South => {
                        (from.x as i32 + offset, from.y as i32 + (dy * dist as i32))
                    }
                    Direction::East | Direction::West => {
                        (from.x as i32 + (dx * dist as i32), from.y as i32 + offset)
                    }
                    Direction::NorthEast => {
                        (from.x as i32 + dist as i32, from.y as i32 - dist as i32 + offset)
                    }
                    Direction::SouthEast => {
                        (from.x as i32 + dist as i32, from.y as i32 + dist as i32 + offset)
                    }
                    Direction::SouthWest => {
                        (from.x as i32 - dist as i32, from.y as i32 + dist as i32 + offset)
                    }
                    Direction::NorthWest => {
                        (from.x as i32 - dist as i32, from.y as i32 - dist as i32 + offset)
                    }
                };

                if px >= 0 && py >= 0 {
                    positions.push(Position::new(px as u16, py as u16, from.z));
                }
            }
        }

        positions
    }

    /// Find path that avoids certain tiles
    pub async fn find_safe_path(
        &self,
        map: &Map,
        from: Position,
        to: Position,
        danger_zones: &[Position],
    ) -> PathResult {
        let mut config = self.config.clone();
        for pos in danger_zones {
            config.avoid_positions.insert(*pos);
        }

        let pathfinder = Pathfinder::with_config(config);
        pathfinder.find_path(map, from, to).await
    }
}

impl Default for Pathfinder {
    fn default() -> Self {
        Self::new()
    }
}

/// Quick pathfinding functions without instantiating Pathfinder
pub async fn find_path(map: &Map, from: Position, to: Position) -> PathResult {
    Pathfinder::new().find_path(map, from, to).await
}

pub async fn has_line_of_sight(map: &Map, from: &Position, to: &Position) -> bool {
    Pathfinder::new().has_line_of_sight(map, from, to).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_positions() {
        let pathfinder = Pathfinder::new();
        let from = Position::new(0, 0, 7);
        let to = Position::new(5, 5, 7);

        let line = pathfinder.get_line_positions(&from, &to);
        assert!(!line.is_empty());
        assert_eq!(line.first().unwrap(), &from);
        assert_eq!(line.last().unwrap(), &to);
    }

    #[test]
    fn test_empty_path() {
        let result = PathResult::empty();
        assert!(result.is_empty());
        assert!(!result.found);
    }
}
