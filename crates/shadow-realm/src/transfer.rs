//! Cross-Realm Transfer System
//!
//! Handles transferring characters between realms.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::RealmError;

/// Transfer request status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransferStatus {
    /// Transfer pending
    Pending,
    /// Transfer approved
    Approved,
    /// Transfer in progress
    InProgress,
    /// Transfer completed
    Completed,
    /// Transfer denied
    Denied,
    /// Transfer cancelled
    Cancelled,
    /// Transfer failed
    Failed,
}

/// Character transfer request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRequest {
    /// Request ID
    pub id: Uuid,
    /// Account ID
    pub account_id: Uuid,
    /// Character ID
    pub character_id: Uuid,
    /// Character name
    pub character_name: String,
    /// Source realm
    pub from_realm: Uuid,
    /// Destination realm
    pub to_realm: Uuid,
    /// Request status
    pub status: TransferStatus,
    /// When requested
    pub requested_at: DateTime<Utc>,
    /// When processed
    pub processed_at: Option<DateTime<Utc>>,
    /// Transfer cost (premium currency)
    pub cost: u32,
    /// Is paid
    pub paid: bool,
    /// Denial reason
    pub denial_reason: Option<String>,
    /// Admin notes
    pub notes: Option<String>,
}

impl TransferRequest {
    /// Create a new transfer request
    pub fn new(
        account_id: Uuid,
        character_id: Uuid,
        character_name: &str,
        from_realm: Uuid,
        to_realm: Uuid,
        cost: u32,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            account_id,
            character_id,
            character_name: character_name.to_string(),
            from_realm,
            to_realm,
            status: TransferStatus::Pending,
            requested_at: Utc::now(),
            processed_at: None,
            cost,
            paid: false,
            denial_reason: None,
            notes: None,
        }
    }

    /// Check if transfer can proceed
    pub fn can_proceed(&self) -> bool {
        matches!(self.status, TransferStatus::Approved) && self.paid
    }

    /// Mark as paid
    pub fn mark_paid(&mut self) {
        self.paid = true;
    }

    /// Approve the transfer
    pub fn approve(&mut self) {
        self.status = TransferStatus::Approved;
        self.processed_at = Some(Utc::now());
    }

    /// Deny the transfer
    pub fn deny(&mut self, reason: &str) {
        self.status = TransferStatus::Denied;
        self.denial_reason = Some(reason.to_string());
        self.processed_at = Some(Utc::now());
    }

    /// Complete the transfer
    pub fn complete(&mut self) {
        self.status = TransferStatus::Completed;
        self.processed_at = Some(Utc::now());
    }

    /// Mark as failed
    pub fn fail(&mut self, reason: &str) {
        self.status = TransferStatus::Failed;
        self.denial_reason = Some(reason.to_string());
    }
}

/// Cross-realm transfer manager
pub struct CrossRealmTransfer {
    /// Pending transfer requests
    requests: HashMap<Uuid, TransferRequest>,
    /// Transfer cooldown per character (hours)
    cooldown_hours: u32,
    /// Last transfer time per character
    last_transfer: HashMap<Uuid, DateTime<Utc>>,
    /// Base transfer cost
    base_cost: u32,
    /// Is transfers enabled
    enabled: bool,
    /// Require admin approval
    require_approval: bool,
}

impl CrossRealmTransfer {
    /// Create a new transfer manager
    pub fn new() -> Self {
        Self {
            requests: HashMap::new(),
            cooldown_hours: 168, // 1 week
            last_transfer: HashMap::new(),
            base_cost: 750, // Premium currency
            enabled: true,
            require_approval: false,
        }
    }

    /// Request a character transfer
    pub fn request_transfer(
        &mut self,
        account_id: Uuid,
        character_id: Uuid,
        character_name: &str,
        from_realm: Uuid,
        to_realm: Uuid,
    ) -> Result<TransferRequest, RealmError> {
        if !self.enabled {
            return Err(RealmError::TransferNotAllowed);
        }

        // Check cooldown
        if let Some(last) = self.last_transfer.get(&character_id) {
            let cooldown = Duration::hours(self.cooldown_hours as i64);
            if Utc::now() - *last < cooldown {
                return Err(RealmError::TransferNotAllowed);
            }
        }

        // Check for existing pending request
        let has_pending = self.requests.values()
            .any(|r| r.character_id == character_id && 
                 matches!(r.status, TransferStatus::Pending | TransferStatus::Approved));
        
        if has_pending {
            return Err(RealmError::TransferNotAllowed);
        }

        let request = TransferRequest::new(
            account_id,
            character_id,
            character_name,
            from_realm,
            to_realm,
            self.base_cost,
        );

        let result = request.clone();
        self.requests.insert(request.id, request);

        Ok(result)
    }

    /// Process a paid transfer
    pub fn process_transfer(
        &mut self,
        request_id: Uuid,
    ) -> Result<TransferRequest, RealmError> {
        let request = self.requests.get_mut(&request_id)
            .ok_or(RealmError::TransferNotAllowed)?;

        if !request.paid {
            return Err(RealmError::TransferNotAllowed);
        }

        // Auto-approve if not requiring admin approval
        if !self.require_approval && request.status == TransferStatus::Pending {
            request.approve();
        }

        if !request.can_proceed() {
            return Err(RealmError::TransferNotAllowed);
        }

        request.status = TransferStatus::InProgress;

        // Actual transfer would happen here:
        // 1. Save character data from source realm
        // 2. Transfer data to destination realm
        // 3. Update realm assignments in database
        // 4. Mark transfer complete

        request.complete();
        
        // Update cooldown
        self.last_transfer.insert(request.character_id, Utc::now());

        Ok(request.clone())
    }

    /// Get pending requests for an account
    pub fn get_account_requests(&self, account_id: Uuid) -> Vec<&TransferRequest> {
        self.requests.values()
            .filter(|r| r.account_id == account_id)
            .collect()
    }

    /// Get all pending requests (for admin)
    pub fn get_pending_requests(&self) -> Vec<&TransferRequest> {
        self.requests.values()
            .filter(|r| r.status == TransferStatus::Pending)
            .collect()
    }

    /// Admin approve a request
    pub fn admin_approve(&mut self, request_id: Uuid) -> Result<(), RealmError> {
        let request = self.requests.get_mut(&request_id)
            .ok_or(RealmError::TransferNotAllowed)?;
        request.approve();
        Ok(())
    }

    /// Admin deny a request
    pub fn admin_deny(&mut self, request_id: Uuid, reason: &str) -> Result<(), RealmError> {
        let request = self.requests.get_mut(&request_id)
            .ok_or(RealmError::TransferNotAllowed)?;
        request.deny(reason);
        Ok(())
    }

    /// Cancel a request
    pub fn cancel_request(&mut self, request_id: Uuid, account_id: Uuid) -> Result<(), RealmError> {
        let request = self.requests.get_mut(&request_id)
            .ok_or(RealmError::TransferNotAllowed)?;

        if request.account_id != account_id {
            return Err(RealmError::TransferNotAllowed);
        }

        if !matches!(request.status, TransferStatus::Pending | TransferStatus::Approved) {
            return Err(RealmError::TransferNotAllowed);
        }

        request.status = TransferStatus::Cancelled;
        Ok(())
    }

    /// Get transfer cost
    pub fn get_cost(&self) -> u32 {
        self.base_cost
    }

    /// Check if transfer is on cooldown
    pub fn is_on_cooldown(&self, character_id: Uuid) -> bool {
        if let Some(last) = self.last_transfer.get(&character_id) {
            let cooldown = Duration::hours(self.cooldown_hours as i64);
            Utc::now() - *last < cooldown
        } else {
            false
        }
    }

    /// Get remaining cooldown
    pub fn cooldown_remaining(&self, character_id: Uuid) -> Option<Duration> {
        self.last_transfer.get(&character_id).and_then(|last| {
            let cooldown = Duration::hours(self.cooldown_hours as i64);
            let elapsed = Utc::now() - *last;
            if elapsed < cooldown {
                Some(cooldown - elapsed)
            } else {
                None
            }
        })
    }
}

impl Default for CrossRealmTransfer {
    fn default() -> Self {
        Self::new()
    }
}
