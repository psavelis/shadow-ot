//! Infrastructure layer - External adapters and implementations
//!
//! This layer contains implementations of the ports defined in the application layer.
//! It handles database access, external APIs, and other infrastructure concerns.

pub mod adapters;

pub use adapters::*;
