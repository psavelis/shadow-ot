//! Shadow OT Anti-Cheat System
//!
//! Comprehensive anti-cheat protection including:
//! - Speed hack detection
//! - Teleport hack detection
//! - Bot detection
//! - Packet manipulation detection
//! - Memory manipulation detection (client-side)
//! - Automated behavior analysis

pub mod analysis;
pub mod detection;
pub mod reporter;
pub mod rules;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

pub use analysis::BehaviorAnalyzer;
pub use detection::{CheatDetector, DetectionResult};
pub use reporter::ViolationReporter;
pub use rules::{AntiCheatRule, RuleEngine};

/// Anti-cheat system errors
#[derive(Debug, Error)]
pub enum AntiCheatError {
    #[error("Detection system error: {0}")]
    DetectionError(String),
    
    #[error("Analysis error: {0}")]
    AnalysisError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
}

/// Type of cheat detected
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CheatType {
    /// Moving faster than allowed
    SpeedHack,
    /// Teleporting to invalid locations
    TeleportHack,
    /// Walking through walls
    WallHack,
    /// Automated bot behavior
    Botting,
    /// Invalid packet sequences
    PacketManipulation,
    /// Attacking too fast
    AttackSpeedHack,
    /// Casting spells too fast
    SpellSpeedHack,
    /// Using items too fast
    ItemSpeedHack,
    /// Invalid position updates
    PositionHack,
    /// Duplicating items
    ItemDupe,
    /// Exploiting game mechanics
    Exploit,
    /// Modified client
    ModifiedClient,
    /// Multi-client abuse
    MultiClient,
    /// Account sharing
    AccountSharing,
    /// Unknown/other
    Unknown,
}

/// Severity level of a violation
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ViolationSeverity {
    /// Minor infraction, log only
    Low,
    /// Moderate infraction, warning
    Medium,
    /// Severe infraction, temporary action
    High,
    /// Critical infraction, immediate action
    Critical,
}

/// Action to take for a violation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViolationAction {
    /// Just log the violation
    Log,
    /// Warn the player
    Warn,
    /// Kick from server
    Kick,
    /// Temporary ban
    TempBan { hours: u32 },
    /// Permanent ban
    PermaBan,
    /// Rollback actions
    Rollback,
    /// Flag for manual review
    FlagForReview,
}

/// A recorded violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Violation {
    /// Unique ID
    pub id: Uuid,
    /// Account ID
    pub account_id: Uuid,
    /// Character ID
    pub character_id: Uuid,
    /// Character name
    pub character_name: String,
    /// Type of cheat
    pub cheat_type: CheatType,
    /// Severity
    pub severity: ViolationSeverity,
    /// Confidence level (0.0 - 1.0)
    pub confidence: f64,
    /// Evidence/details
    pub evidence: ViolationEvidence,
    /// When detected
    pub detected_at: DateTime<Utc>,
    /// Action taken
    pub action_taken: ViolationAction,
    /// Was reviewed by staff
    pub reviewed: bool,
    /// Reviewer notes
    pub notes: Option<String>,
}

/// Evidence for a violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViolationEvidence {
    /// Raw data points
    pub data_points: Vec<String>,
    /// Screenshot reference (if available)
    pub screenshot_id: Option<String>,
    /// Packet log references
    pub packet_log_ids: Vec<String>,
    /// Position history
    pub position_history: Vec<(i32, i32, i32, DateTime<Utc>)>,
    /// Additional context
    pub context: HashMap<String, String>,
}

impl ViolationEvidence {
    pub fn new() -> Self {
        Self {
            data_points: Vec::new(),
            screenshot_id: None,
            packet_log_ids: Vec::new(),
            position_history: Vec::new(),
            context: HashMap::new(),
        }
    }
}

impl Default for ViolationEvidence {
    fn default() -> Self {
        Self::new()
    }
}

/// Player monitoring state
#[derive(Debug, Clone)]
pub struct PlayerMonitor {
    /// Character ID
    pub character_id: Uuid,
    /// Recent positions with timestamps
    pub position_history: Vec<(i32, i32, i32, DateTime<Utc>)>,
    /// Recent actions with timestamps
    pub action_history: Vec<(PlayerAction, DateTime<Utc>)>,
    /// Current violation score (0-100)
    pub violation_score: f64,
    /// Recent violations
    pub recent_violations: Vec<CheatType>,
    /// Is flagged for close monitoring
    pub flagged: bool,
    /// Last update time
    pub last_update: DateTime<Utc>,
}

impl PlayerMonitor {
    pub fn new(character_id: Uuid) -> Self {
        Self {
            character_id,
            position_history: Vec::new(),
            action_history: Vec::new(),
            violation_score: 0.0,
            recent_violations: Vec::new(),
            flagged: false,
            last_update: Utc::now(),
        }
    }

    /// Add a position update
    pub fn add_position(&mut self, x: i32, y: i32, z: i32) {
        let now = Utc::now();
        self.position_history.push((x, y, z, now));
        self.last_update = now;
        
        // Keep only last 100 positions
        if self.position_history.len() > 100 {
            self.position_history.remove(0);
        }
    }

    /// Add an action
    pub fn add_action(&mut self, action: PlayerAction) {
        let now = Utc::now();
        self.action_history.push((action, now));
        self.last_update = now;
        
        // Keep only last 200 actions
        if self.action_history.len() > 200 {
            self.action_history.remove(0);
        }
    }

    /// Calculate movement speed between last two positions
    pub fn last_movement_speed(&self) -> Option<f64> {
        if self.position_history.len() < 2 {
            return None;
        }

        let len = self.position_history.len();
        let (x1, y1, _, t1) = &self.position_history[len - 2];
        let (x2, y2, _, t2) = &self.position_history[len - 1];
        
        let distance = (((x2 - x1).pow(2) + (y2 - y1).pow(2)) as f64).sqrt();
        let duration = (*t2 - *t1).num_milliseconds() as f64 / 1000.0;
        
        if duration > 0.0 {
            Some(distance / duration)
        } else {
            None
        }
    }
}

/// Player actions that can be monitored
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerAction {
    Move,
    Attack,
    CastSpell,
    UseItem,
    Trade,
    PickupItem,
    DropItem,
    Say,
    PrivateMessage,
    Login,
    Logout,
}

/// Anti-cheat configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiCheatConfig {
    /// Enable anti-cheat system
    pub enabled: bool,
    /// Maximum allowed movement speed (tiles per second)
    pub max_movement_speed: f64,
    /// Maximum attack speed (attacks per second)
    pub max_attack_speed: f64,
    /// Maximum spell cast speed (casts per second)
    pub max_spell_speed: f64,
    /// Bot detection sensitivity (0.0 - 1.0)
    pub bot_sensitivity: f64,
    /// Auto-ban threshold score
    pub auto_ban_threshold: f64,
    /// Enable logging
    pub logging_enabled: bool,
    /// Log packet data
    pub log_packets: bool,
    /// Take screenshots on detection
    pub capture_screenshots: bool,
}

impl Default for AntiCheatConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_movement_speed: 20.0, // tiles per second
            max_attack_speed: 2.0, // attacks per second
            max_spell_speed: 1.0, // casts per second
            bot_sensitivity: 0.7,
            auto_ban_threshold: 90.0,
            logging_enabled: true,
            log_packets: false,
            capture_screenshots: true,
        }
    }
}

/// Main anti-cheat system
pub struct AntiCheatSystem {
    /// Configuration
    config: AntiCheatConfig,
    /// Player monitors
    monitors: HashMap<Uuid, PlayerMonitor>,
    /// Cheat detector
    detector: CheatDetector,
    /// Behavior analyzer
    analyzer: BehaviorAnalyzer,
    /// Violation reporter
    reporter: ViolationReporter,
    /// Rule engine
    rules: RuleEngine,
}

impl AntiCheatSystem {
    /// Create a new anti-cheat system
    pub fn new(config: AntiCheatConfig) -> Self {
        Self {
            config: config.clone(),
            monitors: HashMap::new(),
            detector: CheatDetector::new(config.clone()),
            analyzer: BehaviorAnalyzer::new(config.bot_sensitivity),
            reporter: ViolationReporter::new(),
            rules: RuleEngine::new(),
        }
    }

    /// Get or create a player monitor
    pub fn get_monitor(&mut self, character_id: Uuid) -> &mut PlayerMonitor {
        self.monitors.entry(character_id)
            .or_insert_with(|| PlayerMonitor::new(character_id))
    }

    /// Process a position update
    pub fn process_position(
        &mut self,
        character_id: Uuid,
        x: i32,
        y: i32,
        z: i32,
    ) -> Option<DetectionResult> {
        if !self.config.enabled {
            return None;
        }

        // First update the monitor
        {
            let monitor = self.get_monitor(character_id);
            monitor.add_position(x, y, z);
        }

        // Then check for speed hack with a fresh immutable borrow
        let monitor = self.monitors.get(&character_id)?;
        self.detector.check_speed(monitor)
    }

    /// Process an action
    pub fn process_action(
        &mut self,
        character_id: Uuid,
        action: PlayerAction,
    ) -> Option<DetectionResult> {
        if !self.config.enabled {
            return None;
        }

        // First update the monitor
        {
            let monitor = self.get_monitor(character_id);
            monitor.add_action(action);
        }

        // Then check for action-specific hacks with a fresh immutable borrow
        let monitor = self.monitors.get(&character_id)?;
        match action {
            PlayerAction::Attack => self.detector.check_attack_speed(monitor),
            PlayerAction::CastSpell => self.detector.check_spell_speed(monitor),
            _ => None,
        }
    }

    /// Analyze player behavior for bot detection
    pub fn analyze_behavior(&mut self, character_id: Uuid) -> Option<DetectionResult> {
        if !self.config.enabled {
            return None;
        }

        let monitor = self.monitors.get(&character_id)?;
        self.analyzer.analyze(monitor)
    }

    /// Report a violation
    pub fn report_violation(
        &mut self,
        account_id: Uuid,
        character_id: Uuid,
        character_name: &str,
        detection: DetectionResult,
    ) -> Violation {
        let action = self.determine_action(&detection);
        
        self.reporter.report(
            account_id,
            character_id,
            character_name,
            detection,
            action,
        )
    }

    /// Determine action based on detection
    fn determine_action(&self, detection: &DetectionResult) -> ViolationAction {
        match detection.severity {
            ViolationSeverity::Low => ViolationAction::Log,
            ViolationSeverity::Medium => ViolationAction::Warn,
            ViolationSeverity::High => ViolationAction::Kick,
            ViolationSeverity::Critical => {
                if detection.confidence > 0.95 {
                    ViolationAction::PermaBan
                } else {
                    ViolationAction::TempBan { hours: 24 }
                }
            }
        }
    }

    /// Get violation history for a character
    pub fn get_violations(&self, character_id: Uuid) -> Vec<&Violation> {
        self.reporter.get_violations(character_id)
    }

    /// Clean up old monitoring data
    pub fn cleanup(&mut self) {
        let cutoff = Utc::now() - chrono::Duration::hours(1);
        
        self.monitors.retain(|_, monitor| {
            monitor.last_update > cutoff
        });
    }
}
