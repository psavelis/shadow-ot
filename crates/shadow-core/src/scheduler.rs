//! Task scheduler for periodic and delayed operations

use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::{mpsc, Mutex};
use uuid::Uuid;

/// A scheduled task
#[derive(Debug)]
pub struct ScheduledTask {
    pub id: Uuid,
    pub name: String,
    pub execute_at: Instant,
    pub interval: Option<Duration>,
    pub task_type: TaskType,
}

#[derive(Debug, Clone)]
pub enum TaskType {
    SavePlayers,
    SaveWorld,
    RespawnCreatures,
    DecayItems,
    CleanupSessions,
    ProcessRents,
    UpdateHighscores,
    SeasonalEvent(String),
    Custom(String),
}

impl PartialEq for ScheduledTask {
    fn eq(&self, other: &Self) -> bool {
        self.execute_at == other.execute_at
    }
}

impl Eq for ScheduledTask {}

impl PartialOrd for ScheduledTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScheduledTask {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap behavior
        other.execute_at.cmp(&self.execute_at)
    }
}

/// Task scheduler for managing periodic operations
pub struct Scheduler {
    tasks: Arc<Mutex<BinaryHeap<ScheduledTask>>>,
    task_tx: mpsc::Sender<ScheduledTask>,
}

impl Scheduler {
    pub fn new() -> (Self, mpsc::Receiver<ScheduledTask>) {
        let (task_tx, task_rx) = mpsc::channel(100);
        let scheduler = Self {
            tasks: Arc::new(Mutex::new(BinaryHeap::new())),
            task_tx,
        };
        (scheduler, task_rx)
    }

    /// Schedule a one-time task
    pub async fn schedule_once(&self, name: &str, delay: Duration, task_type: TaskType) -> Uuid {
        let task = ScheduledTask {
            id: Uuid::new_v4(),
            name: name.to_string(),
            execute_at: Instant::now() + delay,
            interval: None,
            task_type,
        };
        let id = task.id;
        self.tasks.lock().await.push(task);
        id
    }

    /// Schedule a recurring task
    pub async fn schedule_recurring(
        &self,
        name: &str,
        initial_delay: Duration,
        interval: Duration,
        task_type: TaskType,
    ) -> Uuid {
        let task = ScheduledTask {
            id: Uuid::new_v4(),
            name: name.to_string(),
            execute_at: Instant::now() + initial_delay,
            interval: Some(interval),
            task_type,
        };
        let id = task.id;
        self.tasks.lock().await.push(task);
        id
    }

    /// Cancel a scheduled task
    pub async fn cancel(&self, task_id: Uuid) -> bool {
        let mut tasks = self.tasks.lock().await;
        let original_len = tasks.len();
        let filtered: Vec<_> = tasks.drain().filter(|t| t.id != task_id).collect();
        *tasks = filtered.into_iter().collect();
        tasks.len() < original_len
    }

    /// Process due tasks
    pub async fn process_due(&self) -> Vec<ScheduledTask> {
        let mut tasks = self.tasks.lock().await;
        let mut due = Vec::new();
        let now = Instant::now();

        while let Some(task) = tasks.peek() {
            if task.execute_at <= now {
                let mut task = tasks.pop().unwrap();

                // Reschedule if recurring
                if let Some(interval) = task.interval {
                    let rescheduled = ScheduledTask {
                        id: task.id,
                        name: task.name.clone(),
                        execute_at: Instant::now() + interval,
                        interval: Some(interval),
                        task_type: task.task_type.clone(),
                    };
                    tasks.push(rescheduled);
                }

                due.push(task);
            } else {
                break;
            }
        }

        due
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new().0
    }
}
