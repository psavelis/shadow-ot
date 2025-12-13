//! Domain layer - Core business entities and value objects
//!
//! This module contains pure domain logic with no external dependencies.
//! Entities here represent the core business concepts of the game.

pub mod entities;
pub mod value_objects;
pub mod errors;

#[cfg(test)]
mod tests;

// Re-export commonly used types
pub use entities::*;
pub use value_objects::*;
pub use errors::DomainError;
pub use errors::DomainError;
