//! Ports - Interfaces that define boundaries between layers
//!
//! Ports follow the hexagonal architecture pattern:
//! - `in_ports`: Driven ports - interfaces the application exposes
//! - `out_ports`: Driving ports - interfaces the application needs

pub mod in_ports;
pub mod out_ports;

pub use in_ports::*;
pub use out_ports::*;
