//! Application layer - Use cases, commands, queries, and ports
//!
//! This layer orchestrates domain logic and defines interfaces (ports)
//! that the infrastructure layer must implement.

pub mod ports;
pub mod commands;
pub mod queries;

#[cfg(test)]
mod tests;

pub use ports::*;
pub use commands::*;
pub use queries::*;
