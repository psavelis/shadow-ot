//! Behavior Analysis Module
//!
//! Analyzes player behavior patterns to detect bots and automation.

use chrono::{Duration, Utc};

use crate::{CheatType, DetectionMetrics, PlayerAction, PlayerMonitor, ViolationSeverity};
use crate::detection::DetectionResult;

/// Behavior analyzer for bot detection
pub struct BehaviorAnalyzer {
    /// Sensitivity threshold (0.0 - 1.0)
    sensitivity: f64,
}

impl BehaviorAnalyzer {
    /// Create a new behavior analyzer
    pub fn new(sensitivity: f64) -> Self {
        Self {
            sensitivity: sensitivity.clamp(0.0, 1.0),
        }
    }

    /// Analyze player behavior for bot patterns
    pub fn analyze(&self, monitor: &PlayerMonitor) -> Option<DetectionResult> {
        let mut bot_score = 0.0;
        let mut indicators = Vec::new();

        // Check movement patterns
        if let Some(movement_score) = self.analyze_movement(monitor) {
            bot_score += movement_score * 0.3;
            if movement_score > 0.5 {
                indicators.push("Mechanical movement patterns");
            }
        }

        // Check action timing
        if let Some(timing_score) = self.analyze_timing(monitor) {
            bot_score += timing_score * 0.3;
            if timing_score > 0.5 {
                indicators.push("Inhuman action timing");
            }
        }

        // Check repetition patterns
        if let Some(repetition_score) = self.analyze_repetition(monitor) {
            bot_score += repetition_score * 0.2;
            if repetition_score > 0.5 {
                indicators.push("Repetitive behavior patterns");
            }
        }

        // Check reaction times
        if let Some(reaction_score) = self.analyze_reaction_times(monitor) {
            bot_score += reaction_score * 0.2;
            if reaction_score > 0.5 {
                indicators.push("Inhuman reaction times");
            }
        }

        // Apply sensitivity threshold
        let adjusted_score = bot_score * self.sensitivity;

        if adjusted_score > 0.5 {
            let severity = if adjusted_score > 0.9 {
                ViolationSeverity::Critical
            } else if adjusted_score > 0.7 {
                ViolationSeverity::High
            } else {
                ViolationSeverity::Medium
            };

            return Some(DetectionResult {
                cheat_type: CheatType::Botting,
                severity,
                confidence: adjusted_score,
                description: format!(
                    "Bot behavior detected: {}",
                    indicators.join(", ")
                ),
                metrics: DetectionMetrics {
                    pattern_score: Some(adjusted_score),
                    ..Default::default()
                },
            });
        }

        None
    }

    /// Analyze movement patterns for mechanical behavior
    fn analyze_movement(&self, monitor: &PlayerMonitor) -> Option<f64> {
        if monitor.position_history.len() < 10 {
            return None;
        }

        let positions = &monitor.position_history;
        let mut perfect_diagonals = 0;
        let mut total_moves = 0;

        for i in 1..positions.len() {
            let (x1, y1, _, _) = positions[i - 1];
            let (x2, y2, _, _) = positions[i];
            
            let dx = (x2 - x1).abs();
            let dy = (y2 - y1).abs();

            if dx > 0 || dy > 0 {
                total_moves += 1;
                
                // Perfect diagonal movement is suspicious
                if dx == dy && dx > 0 {
                    perfect_diagonals += 1;
                }
            }
        }

        if total_moves > 5 {
            let diagonal_ratio = perfect_diagonals as f64 / total_moves as f64;
            // Humans rarely move in perfect diagonals consistently
            if diagonal_ratio > 0.3 {
                return Some(diagonal_ratio.min(1.0));
            }
        }

        Some(0.0)
    }

    /// Analyze action timing for inhuman precision
    fn analyze_timing(&self, monitor: &PlayerMonitor) -> Option<f64> {
        if monitor.action_history.len() < 20 {
            return None;
        }

        // Calculate intervals between actions
        let mut intervals: Vec<i64> = Vec::new();
        for i in 1..monitor.action_history.len() {
            let (_, t1) = &monitor.action_history[i - 1];
            let (_, t2) = &monitor.action_history[i];
            intervals.push((*t2 - *t1).num_milliseconds());
        }

        if intervals.len() < 10 {
            return Some(0.0);
        }

        // Calculate variance of intervals
        let mean: f64 = intervals.iter().sum::<i64>() as f64 / intervals.len() as f64;
        let variance: f64 = intervals.iter()
            .map(|&x| (x as f64 - mean).powi(2))
            .sum::<f64>() / intervals.len() as f64;
        let std_dev = variance.sqrt();

        // Very low variance suggests bot (humans have natural variation)
        let cv = if mean > 0.0 { std_dev / mean } else { 0.0 }; // Coefficient of variation

        // CV below 0.1 is suspiciously consistent
        if cv < 0.1 && mean < 1000.0 { // Only for fast actions
            return Some((0.1 - cv) * 10.0); // Higher score for lower variance
        }

        Some(0.0)
    }

    /// Analyze repetitive action patterns
    fn analyze_repetition(&self, monitor: &PlayerMonitor) -> Option<f64> {
        if monitor.action_history.len() < 30 {
            return None;
        }

        // Look for repeating sequences
        let actions: Vec<PlayerAction> = monitor.action_history
            .iter()
            .map(|(a, _)| *a)
            .collect();

        // Check for exact repetitions of sequences
        let sequence_len = 5;
        if actions.len() < sequence_len * 3 {
            return Some(0.0);
        }

        let mut repetition_count = 0;
        let check_range = actions.len() - sequence_len;
        
        for i in 0..check_range.saturating_sub(sequence_len) {
            let sequence: &[PlayerAction] = &actions[i..i + sequence_len];
            
            // Check how many times this exact sequence appears
            for j in (i + sequence_len)..check_range {
                if actions[j..j + sequence_len] == *sequence {
                    repetition_count += 1;
                }
            }
        }

        let repetition_ratio = repetition_count as f64 / (check_range as f64).max(1.0);
        
        // High repetition suggests automation
        if repetition_ratio > 0.2 {
            return Some((repetition_ratio * 2.0).min(1.0));
        }

        Some(0.0)
    }

    /// Analyze reaction times
    fn analyze_reaction_times(&self, _monitor: &PlayerMonitor) -> Option<f64> {
        // Would analyze reaction times to events
        // Requires event system integration
        Some(0.0)
    }

    /// Check for AFK farming patterns
    pub fn check_afk_farming(&self, monitor: &PlayerMonitor) -> Option<DetectionResult> {
        let now = Utc::now();
        let window = Duration::minutes(30);
        let cutoff = now - window;

        // Get actions in last 30 minutes
        let recent_actions: Vec<_> = monitor.action_history.iter()
            .filter(|(_, t)| *t > cutoff)
            .collect();

        if recent_actions.len() < 50 {
            return None;
        }

        // Check if only combat actions (no talking, trading, etc.)
        let combat_actions = recent_actions.iter()
            .filter(|(a, _)| matches!(a, PlayerAction::Attack | PlayerAction::CastSpell))
            .count();

        let combat_ratio = combat_actions as f64 / recent_actions.len() as f64;

        // If 95%+ actions are just combat for 30 minutes, suspicious
        if combat_ratio > 0.95 {
            return Some(DetectionResult {
                cheat_type: CheatType::Botting,
                severity: ViolationSeverity::Medium,
                confidence: combat_ratio,
                description: "Possible AFK farming: 95%+ combat actions for 30 minutes".to_string(),
                metrics: DetectionMetrics {
                    pattern_score: Some(combat_ratio),
                    time_window_ms: Some(30 * 60 * 1000),
                    ..Default::default()
                },
            });
        }

        None
    }
}
