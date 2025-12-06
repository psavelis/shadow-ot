//! Violation Reporter Module
//!
//! Handles recording and reporting of violations.

use chrono::Utc;
use std::collections::HashMap;
use uuid::Uuid;

use crate::{Violation, ViolationAction, ViolationEvidence};
use crate::detection::DetectionResult;

/// Violation reporter
pub struct ViolationReporter {
    /// All violations
    violations: Vec<Violation>,
    /// Violations by character
    by_character: HashMap<Uuid, Vec<usize>>,
    /// Violations by account
    by_account: HashMap<Uuid, Vec<usize>>,
}

impl ViolationReporter {
    /// Create a new violation reporter
    pub fn new() -> Self {
        Self {
            violations: Vec::new(),
            by_character: HashMap::new(),
            by_account: HashMap::new(),
        }
    }

    /// Report a violation
    pub fn report(
        &mut self,
        account_id: Uuid,
        character_id: Uuid,
        character_name: &str,
        detection: DetectionResult,
        action: ViolationAction,
    ) -> Violation {
        let violation = Violation {
            id: Uuid::new_v4(),
            account_id,
            character_id,
            character_name: character_name.to_string(),
            cheat_type: detection.cheat_type,
            severity: detection.severity,
            confidence: detection.confidence,
            evidence: ViolationEvidence {
                data_points: vec![detection.description.clone()],
                screenshot_id: None,
                packet_log_ids: Vec::new(),
                position_history: Vec::new(),
                context: {
                    let mut ctx = HashMap::new();
                    if let Some(speed) = detection.metrics.speed {
                        ctx.insert("speed".to_string(), speed.to_string());
                    }
                    if let Some(aps) = detection.metrics.actions_per_second {
                        ctx.insert("actions_per_second".to_string(), aps.to_string());
                    }
                    ctx
                },
            },
            detected_at: Utc::now(),
            action_taken: action,
            reviewed: false,
            notes: None,
        };

        let index = self.violations.len();
        self.violations.push(violation.clone());

        // Index by character
        self.by_character
            .entry(character_id)
            .or_insert_with(Vec::new)
            .push(index);

        // Index by account
        self.by_account
            .entry(account_id)
            .or_insert_with(Vec::new)
            .push(index);

        violation
    }

    /// Get violations for a character
    pub fn get_violations(&self, character_id: Uuid) -> Vec<&Violation> {
        self.by_character
            .get(&character_id)
            .map(|indices| {
                indices.iter()
                    .filter_map(|&i| self.violations.get(i))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get violations for an account
    pub fn get_account_violations(&self, account_id: Uuid) -> Vec<&Violation> {
        self.by_account
            .get(&account_id)
            .map(|indices| {
                indices.iter()
                    .filter_map(|&i| self.violations.get(i))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get all unreviewed violations
    pub fn get_unreviewed(&self) -> Vec<&Violation> {
        self.violations.iter()
            .filter(|v| !v.reviewed)
            .collect()
    }

    /// Mark a violation as reviewed
    pub fn mark_reviewed(&mut self, violation_id: Uuid, notes: Option<String>) -> bool {
        if let Some(violation) = self.violations.iter_mut()
            .find(|v| v.id == violation_id)
        {
            violation.reviewed = true;
            violation.notes = notes;
            true
        } else {
            false
        }
    }

    /// Get violation count for a character in the last N hours
    pub fn count_recent_violations(
        &self,
        character_id: Uuid,
        hours: i64,
    ) -> usize {
        let cutoff = Utc::now() - chrono::Duration::hours(hours);
        
        self.get_violations(character_id)
            .iter()
            .filter(|v| v.detected_at > cutoff)
            .count()
    }

    /// Get total violations
    pub fn total_count(&self) -> usize {
        self.violations.len()
    }
}

impl Default for ViolationReporter {
    fn default() -> Self {
        Self::new()
    }
}
