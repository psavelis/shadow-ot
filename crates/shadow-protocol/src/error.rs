//! Protocol error types

use thiserror::Error;

pub type Result<T> = std::result::Result<T, ProtocolError>;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("Invalid packet: {0}")]
    InvalidPacket(String),

    #[error("Encryption error: {0}")]
    Encryption(String),

    #[error("Decryption error: {0}")]
    Decryption(String),

    #[error("Invalid checksum")]
    InvalidChecksum,

    #[error("Packet too large: {size} bytes (max: {max})")]
    PacketTooLarge { size: usize, max: usize },

    #[error("Unknown packet type: 0x{0:02X}")]
    UnknownPacket(u8),

    #[error("Unsupported protocol version: {0}")]
    UnsupportedVersion(u16),

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid RSA key")]
    InvalidRsaKey,

    #[error("Buffer underflow: needed {needed} bytes, had {available}")]
    BufferUnderflow { needed: usize, available: usize },

    #[error("Invalid string encoding")]
    InvalidString,
}
