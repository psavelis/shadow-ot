//! Anti-Cheat Rules Engine
//!
//! Configurable rules for detection and action thresholds.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{CheatType, ViolationAction, ViolationSeverity};

/// An anti-cheat rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiCheatRule {
    /// Rule ID
    pub id: String,
    /// Rule name
    pub name: String,
    /// Description
    pub description: String,
    /// Cheat type this rule applies to
    pub cheat_type: CheatType,
    /// Minimum confidence to trigger
    pub min_confidence: f64,
    /// Action to take
    pub action: ViolationAction,
    /// Is rule enabled
    pub enabled: bool,
    /// Additional conditions
    pub conditions: Vec<RuleCondition>,
}

/// Additional conditions for rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleCondition {
    /// Minimum player level
    MinLevel(u32),
    /// Maximum player level
    MaxLevel(u32),
    /// Minimum account age in days
    MinAccountAge(u32),
    /// Minimum violations in time period
    MinViolationsInPeriod { count: u32, hours: u32 },
    /// Player has VIP
    HasVip,
    /// Player does not have VIP
    NoVip,
}

/// Rule engine for evaluating anti-cheat rules
pub struct RuleEngine {
    rules: HashMap<String, AntiCheatRule>,
    default_rules: Vec<AntiCheatRule>,
}

impl RuleEngine {
    /// Create a new rule engine with default rules
    pub fn new() -> Self {
        let mut engine = Self {
            rules: HashMap::new(),
            default_rules: Self::create_default_rules(),
        };

        // Register default rules
        for rule in engine.default_rules.clone() {
            engine.register_rule(rule);
        }

        engine
    }

    /// Create default rule set
    fn create_default_rules() -> Vec<AntiCheatRule> {
        vec![
            // Speed hack rules
            AntiCheatRule {
                id: "speed_critical".to_string(),
                name: "Critical Speed Hack".to_string(),
                description: "Immediate action for obvious speed hacks".to_string(),
                cheat_type: CheatType::SpeedHack,
                min_confidence: 0.95,
                action: ViolationAction::TempBan { hours: 24 },
                enabled: true,
                conditions: Vec::new(),
            },
            AntiCheatRule {
                id: "speed_high".to_string(),
                name: "High Speed Hack".to_string(),
                description: "Kick and flag for review".to_string(),
                cheat_type: CheatType::SpeedHack,
                min_confidence: 0.7,
                action: ViolationAction::Kick,
                enabled: true,
                conditions: Vec::new(),
            },
            AntiCheatRule {
                id: "speed_low".to_string(),
                name: "Low Speed Anomaly".to_string(),
                description: "Log for analysis".to_string(),
                cheat_type: CheatType::SpeedHack,
                min_confidence: 0.3,
                action: ViolationAction::Log,
                enabled: true,
                conditions: Vec::new(),
            },

            // Teleport hack rules
            AntiCheatRule {
                id: "teleport_critical".to_string(),
                name: "Critical Teleport Hack".to_string(),
                description: "Immediate ban for teleport hacks".to_string(),
                cheat_type: CheatType::TeleportHack,
                min_confidence: 0.9,
                action: ViolationAction::PermaBan,
                enabled: true,
                conditions: Vec::new(),
            },

            // Bot detection rules
            AntiCheatRule {
                id: "bot_confirmed".to_string(),
                name: "Confirmed Bot".to_string(),
                description: "High confidence bot detection".to_string(),
                cheat_type: CheatType::Botting,
                min_confidence: 0.9,
                action: ViolationAction::PermaBan,
                enabled: true,
                conditions: Vec::new(),
            },
            AntiCheatRule {
                id: "bot_suspected".to_string(),
                name: "Suspected Bot".to_string(),
                description: "Medium confidence bot detection".to_string(),
                cheat_type: CheatType::Botting,
                min_confidence: 0.6,
                action: ViolationAction::FlagForReview,
                enabled: true,
                conditions: Vec::new(),
            },

            // Attack speed rules
            AntiCheatRule {
                id: "attack_speed".to_string(),
                name: "Attack Speed Hack".to_string(),
                description: "Attacking faster than allowed".to_string(),
                cheat_type: CheatType::AttackSpeedHack,
                min_confidence: 0.8,
                action: ViolationAction::Kick,
                enabled: true,
                conditions: Vec::new(),
            },

            // Packet manipulation
            AntiCheatRule {
                id: "packet_manipulation".to_string(),
                name: "Packet Manipulation".to_string(),
                description: "Invalid packet sequences detected".to_string(),
                cheat_type: CheatType::PacketManipulation,
                min_confidence: 0.7,
                action: ViolationAction::Kick,
                enabled: true,
                conditions: Vec::new(),
            },

            // Item duplication
            AntiCheatRule {
                id: "item_dupe".to_string(),
                name: "Item Duplication".to_string(),
                description: "Detected item duplication attempt".to_string(),
                cheat_type: CheatType::ItemDupe,
                min_confidence: 0.5,
                action: ViolationAction::PermaBan,
                enabled: true,
                conditions: Vec::new(),
            },
        ]
    }

    /// Register a custom rule
    pub fn register_rule(&mut self, rule: AntiCheatRule) {
        self.rules.insert(rule.id.clone(), rule);
    }

    /// Get a rule by ID
    pub fn get_rule(&self, id: &str) -> Option<&AntiCheatRule> {
        self.rules.get(id)
    }

    /// Enable or disable a rule
    pub fn set_rule_enabled(&mut self, id: &str, enabled: bool) -> bool {
        if let Some(rule) = self.rules.get_mut(id) {
            rule.enabled = enabled;
            true
        } else {
            false
        }
    }

    /// Find matching rules for a detection
    pub fn find_matching_rules(
        &self,
        cheat_type: CheatType,
        confidence: f64,
        _severity: ViolationSeverity,
    ) -> Vec<&AntiCheatRule> {
        self.rules.values()
            .filter(|rule| {
                rule.enabled
                    && rule.cheat_type == cheat_type
                    && confidence >= rule.min_confidence
            })
            .collect()
    }

    /// Get the highest priority action for a detection
    pub fn get_action(
        &self,
        cheat_type: CheatType,
        confidence: f64,
        severity: ViolationSeverity,
    ) -> ViolationAction {
        let matching = self.find_matching_rules(cheat_type, confidence, severity);
        
        // Get most severe action
        matching.into_iter()
            .map(|r| &r.action)
            .max_by(|a, b| Self::action_severity(a).cmp(&Self::action_severity(b)))
            .cloned()
            .unwrap_or(ViolationAction::Log)
    }

    /// Get severity ranking for actions
    fn action_severity(action: &ViolationAction) -> u8 {
        match action {
            ViolationAction::Log => 0,
            ViolationAction::Warn => 1,
            ViolationAction::FlagForReview => 2,
            ViolationAction::Kick => 3,
            ViolationAction::Rollback => 4,
            ViolationAction::TempBan { hours } => 5 + (*hours as u8).min(250),
            ViolationAction::PermaBan => 255,
        }
    }

    /// Get all rules
    pub fn all_rules(&self) -> impl Iterator<Item = &AntiCheatRule> {
        self.rules.values()
    }

    /// Get rules by cheat type
    pub fn rules_for_type(&self, cheat_type: CheatType) -> Vec<&AntiCheatRule> {
        self.rules.values()
            .filter(|r| r.cheat_type == cheat_type)
            .collect()
    }
}

impl Default for RuleEngine {
    fn default() -> Self {
        Self::new()
    }
}
