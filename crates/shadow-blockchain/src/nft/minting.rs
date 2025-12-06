//! NFT Minting Queue
//!
//! Asynchronous minting with queue management and retry logic.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::VecDeque;

use crate::{AssetType, Chain, MintResult, NftMetadata, Result, BlockchainError};

/// Status of a mint request
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MintStatus {
    /// Request is queued
    Pending,
    /// Currently being processed
    Processing,
    /// Successfully minted
    Completed,
    /// Mint failed
    Failed,
    /// Manually cancelled
    Cancelled,
}

/// A request to mint an NFT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintRequest {
    pub id: Uuid,
    pub user_id: Uuid,
    pub chain: Chain,
    pub to_address: String,
    pub asset: AssetType,
    pub metadata: Option<NftMetadata>,
    pub status: MintStatus,
    pub priority: MintPriority,
    pub retry_count: u32,
    pub max_retries: u32,
    pub error_message: Option<String>,
    pub result: Option<MintResult>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Priority levels for minting
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MintPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

impl MintRequest {
    pub fn new(
        user_id: Uuid,
        chain: Chain,
        to_address: String,
        asset: AssetType,
    ) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            chain,
            to_address,
            asset,
            metadata: None,
            status: MintStatus::Pending,
            priority: MintPriority::Normal,
            retry_count: 0,
            max_retries: 3,
            error_message: None,
            result: None,
            created_at: now,
            updated_at: now,
            completed_at: None,
        }
    }

    pub fn with_metadata(mut self, metadata: NftMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn with_priority(mut self, priority: MintPriority) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_max_retries(mut self, max: u32) -> Self {
        self.max_retries = max;
        self
    }

    pub fn can_retry(&self) -> bool {
        self.retry_count < self.max_retries
    }

    pub fn mark_processing(&mut self) {
        self.status = MintStatus::Processing;
        self.updated_at = chrono::Utc::now();
    }

    pub fn mark_completed(&mut self, result: MintResult) {
        self.status = MintStatus::Completed;
        self.result = Some(result);
        self.completed_at = Some(chrono::Utc::now());
        self.updated_at = chrono::Utc::now();
    }

    pub fn mark_failed(&mut self, error: &str) {
        self.retry_count += 1;
        if self.can_retry() {
            self.status = MintStatus::Pending;
        } else {
            self.status = MintStatus::Failed;
        }
        self.error_message = Some(error.to_string());
        self.updated_at = chrono::Utc::now();
    }

    pub fn cancel(&mut self) {
        self.status = MintStatus::Cancelled;
        self.updated_at = chrono::Utc::now();
    }
}

/// Queue for managing mint requests
pub struct MintQueue {
    /// High priority queue
    high_priority: std::sync::RwLock<VecDeque<MintRequest>>,
    /// Normal priority queue
    normal_priority: std::sync::RwLock<VecDeque<MintRequest>>,
    /// Low priority queue
    low_priority: std::sync::RwLock<VecDeque<MintRequest>>,
    /// All requests by ID
    requests: std::sync::RwLock<std::collections::HashMap<Uuid, MintRequest>>,
    /// Max queue size per priority
    max_queue_size: usize,
}

impl MintQueue {
    pub fn new(max_queue_size: usize) -> Self {
        Self {
            high_priority: std::sync::RwLock::new(VecDeque::new()),
            normal_priority: std::sync::RwLock::new(VecDeque::new()),
            low_priority: std::sync::RwLock::new(VecDeque::new()),
            requests: std::sync::RwLock::new(std::collections::HashMap::new()),
            max_queue_size,
        }
    }

    /// Add a mint request to the queue
    pub fn enqueue(&self, request: MintRequest) -> Result<Uuid> {
        let id = request.id;
        let priority = request.priority;

        // Store in requests map
        {
            let mut requests = self.requests.write()
                .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;
            requests.insert(id, request.clone());
        }

        // Add to appropriate priority queue
        let queue = match priority {
            MintPriority::Critical | MintPriority::High => &self.high_priority,
            MintPriority::Normal => &self.normal_priority,
            MintPriority::Low => &self.low_priority,
        };

        {
            let mut q = queue.write()
                .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

            if q.len() >= self.max_queue_size {
                return Err(BlockchainError::QueueFull);
            }

            q.push_back(request);
        }

        tracing::debug!("Enqueued mint request {} with {:?} priority", id, priority);
        Ok(id)
    }

    /// Get the next request to process
    pub fn dequeue(&self) -> Result<Option<MintRequest>> {
        // Try high priority first
        {
            let mut q = self.high_priority.write()
                .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;
            if let Some(request) = q.pop_front() {
                return Ok(Some(request));
            }
        }

        // Then normal priority
        {
            let mut q = self.normal_priority.write()
                .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;
            if let Some(request) = q.pop_front() {
                return Ok(Some(request));
            }
        }

        // Finally low priority
        {
            let mut q = self.low_priority.write()
                .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;
            if let Some(request) = q.pop_front() {
                return Ok(Some(request));
            }
        }

        Ok(None)
    }

    /// Get a request by ID
    pub fn get(&self, id: Uuid) -> Result<Option<MintRequest>> {
        let requests = self.requests.read()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;
        Ok(requests.get(&id).cloned())
    }

    /// Update a request
    pub fn update(&self, request: MintRequest) -> Result<()> {
        let mut requests = self.requests.write()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;
        requests.insert(request.id, request);
        Ok(())
    }

    /// Get requests by user
    pub fn get_user_requests(&self, user_id: Uuid) -> Result<Vec<MintRequest>> {
        let requests = self.requests.read()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

        Ok(requests
            .values()
            .filter(|r| r.user_id == user_id)
            .cloned()
            .collect())
    }

    /// Get queue statistics
    pub fn stats(&self) -> Result<QueueStats> {
        let high = self.high_priority.read()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?
            .len();
        let normal = self.normal_priority.read()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?
            .len();
        let low = self.low_priority.read()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?
            .len();

        let requests = self.requests.read()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

        let pending = requests.values().filter(|r| r.status == MintStatus::Pending).count();
        let processing = requests.values().filter(|r| r.status == MintStatus::Processing).count();
        let completed = requests.values().filter(|r| r.status == MintStatus::Completed).count();
        let failed = requests.values().filter(|r| r.status == MintStatus::Failed).count();

        Ok(QueueStats {
            high_priority_queued: high,
            normal_priority_queued: normal,
            low_priority_queued: low,
            total_queued: high + normal + low,
            pending,
            processing,
            completed,
            failed,
        })
    }

    /// Clear completed and failed requests older than the given duration
    pub fn cleanup(&self, older_than: chrono::Duration) -> Result<usize> {
        let cutoff = chrono::Utc::now() - older_than;

        let mut requests = self.requests.write()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

        let before = requests.len();
        requests.retain(|_, r| {
            !(r.status == MintStatus::Completed || r.status == MintStatus::Failed)
                || r.updated_at > cutoff
        });

        let removed = before - requests.len();
        if removed > 0 {
            tracing::info!("Cleaned up {} old mint requests", removed);
        }

        Ok(removed)
    }
}

impl Default for MintQueue {
    fn default() -> Self {
        Self::new(10000)
    }
}

/// Queue statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueStats {
    pub high_priority_queued: usize,
    pub normal_priority_queued: usize,
    pub low_priority_queued: usize,
    pub total_queued: usize,
    pub pending: usize,
    pub processing: usize,
    pub completed: usize,
    pub failed: usize,
}
