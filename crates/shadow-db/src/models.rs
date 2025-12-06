//! Database models for Shadow OT
//!
//! Complete data models covering all game entities

pub mod account;
pub mod character;
pub mod guild;
pub mod house;
pub mod item;
pub mod market;
pub mod quest;
pub mod realm;
pub mod social;
pub mod stats;
pub mod forum;
pub mod blockchain;

pub use account::*;
pub use character::*;
pub use guild::*;
pub use house::*;
pub use item::*;
pub use market::*;
pub use quest::*;
pub use realm::*;
pub use social::*;
pub use stats::*;
pub use forum::*;
pub use blockchain::*;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Base timestamp fields for all models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Timestamps {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
