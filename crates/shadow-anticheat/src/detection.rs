//! Cheat Detection Module
//!
//! Core detection algorithms for various cheat types.

use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::{AntiCheatConfig, CheatType, PlayerAction, PlayerMonitor, ViolationSeverity};

/// Result of a detection check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionResult {
    /// Type of cheat detected
    pub cheat_type: CheatType,
    /// Severity level
    pub severity: ViolationSeverity,
    /// Confidence level (0.0 - 1.0)
    pub confidence: f64,
    /// Description of what was detected
    pub description: String,
    /// Raw metrics that triggered detection
    pub metrics: DetectionMetrics,
}

/// Metrics that contributed to detection
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DetectionMetrics {
    /// Speed measured (tiles/second)
    pub speed: Option<f64>,
    /// Expected max speed
    pub expected_max_speed: Option<f64>,
    /// Actions per second
    pub actions_per_second: Option<f64>,
    /// Distance jumped
    pub distance: Option<f64>,
    /// Time window analyzed (ms)
    pub time_window_ms: Option<u64>,
    /// Pattern match score
    pub pattern_score: Option<f64>,
}

/// Cheat detector engine
pub struct CheatDetector {
    config: AntiCheatConfig,
}

impl CheatDetector {
    /// Create a new cheat detector
    pub fn new(config: AntiCheatConfig) -> Self {
        Self { config }
    }

    /// Check for speed hacks
    pub fn check_speed(&self, monitor: &PlayerMonitor) -> Option<DetectionResult> {
        let speed = monitor.last_movement_speed()?;
        
        if speed > self.config.max_movement_speed {
            let severity = if speed > self.config.max_movement_speed * 3.0 {
                ViolationSeverity::Critical
            } else if speed > self.config.max_movement_speed * 2.0 {
                ViolationSeverity::High
            } else if speed > self.config.max_movement_speed * 1.5 {
                ViolationSeverity::Medium
            } else {
                ViolationSeverity::Low
            };

            let confidence = ((speed - self.config.max_movement_speed) 
                / self.config.max_movement_speed).min(1.0);

            return Some(DetectionResult {
                cheat_type: CheatType::SpeedHack,
                severity,
                confidence,
                description: format!(
                    "Speed violation: {:.2} tiles/s (max: {:.2})",
                    speed, self.config.max_movement_speed
                ),
                metrics: DetectionMetrics {
                    speed: Some(speed),
                    expected_max_speed: Some(self.config.max_movement_speed),
                    ..Default::default()
                },
            });
        }

        None
    }

    /// Check for teleport hacks (sudden position changes)
    pub fn check_teleport(&self, monitor: &PlayerMonitor) -> Option<DetectionResult> {
        if monitor.position_history.len() < 2 {
            return None;
        }

        let len = monitor.position_history.len();
        let (x1, y1, z1, t1) = &monitor.position_history[len - 2];
        let (x2, y2, z2, t2) = &monitor.position_history[len - 1];

        // Calculate 3D distance
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let dz = (z2 - z1).abs();
        
        // Check for instant large distance jumps
        let distance_2d = ((dx.pow(2) + dy.pow(2)) as f64).sqrt();
        let time_diff = (*t2 - *t1).num_milliseconds();

        // If moved more than 10 tiles in under 100ms, suspicious
        if distance_2d > 10.0 && time_diff < 100 {
            let confidence = (distance_2d / 50.0).min(1.0);
            
            return Some(DetectionResult {
                cheat_type: CheatType::TeleportHack,
                severity: ViolationSeverity::Critical,
                confidence,
                description: format!(
                    "Teleport detected: {:.1} tiles in {}ms",
                    distance_2d, time_diff
                ),
                metrics: DetectionMetrics {
                    distance: Some(distance_2d),
                    time_window_ms: Some(time_diff as u64),
                    ..Default::default()
                },
            });
        }

        // Check for floor changes without stairs/ladder
        if dz > 0 && distance_2d < 1.0 {
            // This would need map data to verify if there's actually stairs
            // For now, just flag for review
            return Some(DetectionResult {
                cheat_type: CheatType::TeleportHack,
                severity: ViolationSeverity::Medium,
                confidence: 0.5,
                description: format!(
                    "Suspicious floor change: {} -> {} with minimal movement",
                    z1, z2
                ),
                metrics: DetectionMetrics {
                    distance: Some(distance_2d),
                    ..Default::default()
                },
            });
        }

        None
    }

    /// Check for attack speed hacks
    pub fn check_attack_speed(&self, monitor: &PlayerMonitor) -> Option<DetectionResult> {
        self.check_action_speed(
            monitor,
            PlayerAction::Attack,
            self.config.max_attack_speed,
            CheatType::AttackSpeedHack,
        )
    }

    /// Check for spell speed hacks
    pub fn check_spell_speed(&self, monitor: &PlayerMonitor) -> Option<DetectionResult> {
        self.check_action_speed(
            monitor,
            PlayerAction::CastSpell,
            self.config.max_spell_speed,
            CheatType::SpellSpeedHack,
        )
    }

    /// Generic action speed check
    fn check_action_speed(
        &self,
        monitor: &PlayerMonitor,
        action_type: PlayerAction,
        max_speed: f64,
        cheat_type: CheatType,
    ) -> Option<DetectionResult> {
        let now = Utc::now();
        let window = Duration::seconds(5);
        let cutoff = now - window;

        // Count actions in window
        let actions_in_window: Vec<_> = monitor.action_history.iter()
            .filter(|(a, t)| *a == action_type && *t > cutoff)
            .collect();

        if actions_in_window.len() < 2 {
            return None;
        }

        // Calculate actions per second
        let aps = actions_in_window.len() as f64 / 5.0;

        if aps > max_speed {
            let severity = if aps > max_speed * 2.0 {
                ViolationSeverity::Critical
            } else if aps > max_speed * 1.5 {
                ViolationSeverity::High
            } else {
                ViolationSeverity::Medium
            };

            let confidence = ((aps - max_speed) / max_speed).min(1.0);

            return Some(DetectionResult {
                cheat_type,
                severity,
                confidence,
                description: format!(
                    "Action speed violation: {:.2}/s (max: {:.2}/s)",
                    aps, max_speed
                ),
                metrics: DetectionMetrics {
                    actions_per_second: Some(aps),
                    expected_max_speed: Some(max_speed),
                    time_window_ms: Some(5000),
                    ..Default::default()
                },
            });
        }

        None
    }

    /// Check for wall hacks (walking through blocked tiles)
    pub fn check_wallhack(
        &self,
        _monitor: &PlayerMonitor,
        _is_tile_walkable: impl Fn(i32, i32, i32) -> bool,
    ) -> Option<DetectionResult> {
        // Would check if player moved through non-walkable tiles
        // Requires map data integration
        None
    }
}
