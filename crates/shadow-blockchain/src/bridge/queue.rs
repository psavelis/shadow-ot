//! Bridge Queue Management
//!
//! Handles queuing and processing of bridge requests.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::VecDeque;

use crate::{BridgeRequest, BridgeStatus, Chain, Result, BlockchainError};

/// Priority levels for bridge requests
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum BridgePriority {
    Low = 0,
    Normal = 1,
    High = 2,
}

/// A bridge request in the queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedBridge {
    pub request: BridgeRequest,
    pub user_id: Uuid,
    pub priority: BridgePriority,
    pub attempts: u32,
    pub max_attempts: u32,
    pub queued_at: chrono::DateTime<chrono::Utc>,
    pub last_attempt_at: Option<chrono::DateTime<chrono::Utc>>,
    pub error_message: Option<String>,
}

impl QueuedBridge {
    pub fn new(request: BridgeRequest, user_id: Uuid) -> Self {
        Self {
            request,
            user_id,
            priority: BridgePriority::Normal,
            attempts: 0,
            max_attempts: 3,
            queued_at: chrono::Utc::now(),
            last_attempt_at: None,
            error_message: None,
        }
    }

    pub fn with_priority(mut self, priority: BridgePriority) -> Self {
        self.priority = priority;
        self
    }

    pub fn can_retry(&self) -> bool {
        self.attempts < self.max_attempts
    }

    pub fn record_attempt(&mut self, success: bool, error: Option<&str>) {
        self.attempts += 1;
        self.last_attempt_at = Some(chrono::Utc::now());
        if !success {
            self.error_message = error.map(String::from);
        }
    }
}

/// Queue for managing bridge requests
pub struct BridgeQueue {
    high_priority: std::sync::RwLock<VecDeque<QueuedBridge>>,
    normal_priority: std::sync::RwLock<VecDeque<QueuedBridge>>,
    low_priority: std::sync::RwLock<VecDeque<QueuedBridge>>,
    processing: std::sync::RwLock<std::collections::HashMap<Uuid, QueuedBridge>>,
    max_queue_size: usize,
}

impl BridgeQueue {
    pub fn new(max_queue_size: usize) -> Self {
        Self {
            high_priority: std::sync::RwLock::new(VecDeque::new()),
            normal_priority: std::sync::RwLock::new(VecDeque::new()),
            low_priority: std::sync::RwLock::new(VecDeque::new()),
            processing: std::sync::RwLock::new(std::collections::HashMap::new()),
            max_queue_size,
        }
    }

    /// Add a bridge request to the queue
    pub fn enqueue(&self, queued: QueuedBridge) -> Result<()> {
        let queue = match queued.priority {
            BridgePriority::High => &self.high_priority,
            BridgePriority::Normal => &self.normal_priority,
            BridgePriority::Low => &self.low_priority,
        };

        let mut q = queue.write()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

        if q.len() >= self.max_queue_size {
            return Err(BlockchainError::QueueFull);
        }

        q.push_back(queued);
        Ok(())
    }

    /// Get the next request to process
    pub fn dequeue(&self) -> Result<Option<QueuedBridge>> {
        // Try high priority first
        {
            let mut q = self.high_priority.write()
                .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;
            if let Some(queued) = q.pop_front() {
                return Ok(Some(queued));
            }
        }

        // Then normal
        {
            let mut q = self.normal_priority.write()
                .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;
            if let Some(queued) = q.pop_front() {
                return Ok(Some(queued));
            }
        }

        // Finally low
        {
            let mut q = self.low_priority.write()
                .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;
            if let Some(queued) = q.pop_front() {
                return Ok(Some(queued));
            }
        }

        Ok(None)
    }

    /// Mark a request as being processed
    pub fn mark_processing(&self, queued: QueuedBridge) -> Result<()> {
        let mut processing = self.processing.write()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;
        processing.insert(queued.request.request_id, queued);
        Ok(())
    }

    /// Mark processing complete
    pub fn mark_complete(&self, request_id: Uuid) -> Result<Option<QueuedBridge>> {
        let mut processing = self.processing.write()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;
        Ok(processing.remove(&request_id))
    }

    /// Requeue a failed request
    pub fn requeue(&self, mut queued: QueuedBridge) -> Result<bool> {
        if !queued.can_retry() {
            return Ok(false);
        }

        // Lower priority on retry
        queued.priority = match queued.priority {
            BridgePriority::High => BridgePriority::Normal,
            _ => BridgePriority::Low,
        };

        self.enqueue(queued)?;
        Ok(true)
    }

    /// Get queue statistics
    pub fn stats(&self) -> Result<BridgeQueueStats> {
        let high = self.high_priority.read()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?
            .len();
        let normal = self.normal_priority.read()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?
            .len();
        let low = self.low_priority.read()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?
            .len();
        let processing = self.processing.read()
            .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?
            .len();

        Ok(BridgeQueueStats {
            high_priority: high,
            normal_priority: normal,
            low_priority: low,
            total_queued: high + normal + low,
            processing,
        })
    }

    /// Get all requests for a specific route
    pub fn get_by_route(&self, source: Chain, target: Chain) -> Result<Vec<QueuedBridge>> {
        let mut result = Vec::new();

        for queue in [&self.high_priority, &self.normal_priority, &self.low_priority] {
            let q = queue.read()
                .map_err(|_| BlockchainError::InternalError("Lock poisoned".into()))?;

            for queued in q.iter() {
                if queued.request.source_chain == source && queued.request.target_chain == target {
                    result.push(queued.clone());
                }
            }
        }

        Ok(result)
    }
}

impl Default for BridgeQueue {
    fn default() -> Self {
        Self::new(10000)
    }
}

/// Queue statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeQueueStats {
    pub high_priority: usize,
    pub normal_priority: usize,
    pub low_priority: usize,
    pub total_queued: usize,
    pub processing: usize,
}
