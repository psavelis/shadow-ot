//! Network message codec for Tibia protocol

use bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

use crate::crypto::{adler32, Xtea};
use crate::version::ProtocolVersion;
use crate::{ProtocolError, Result, HEADER_SIZE, MAX_PACKET_SIZE};

/// Network message with raw bytes
#[derive(Debug, Clone)]
pub struct NetworkMessage {
    pub data: BytesMut,
}

impl NetworkMessage {
    pub fn new() -> Self {
        Self {
            data: BytesMut::with_capacity(1024),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: BytesMut::with_capacity(capacity),
        }
    }

    pub fn from_bytes(data: BytesMut) -> Self {
        Self { data }
    }

    // Reading methods
    pub fn get_u8(&mut self) -> Result<u8> {
        if self.data.remaining() < 1 {
            return Err(ProtocolError::BufferUnderflow {
                needed: 1,
                available: self.data.remaining(),
            });
        }
        Ok(self.data.get_u8())
    }

    pub fn get_u16(&mut self) -> Result<u16> {
        if self.data.remaining() < 2 {
            return Err(ProtocolError::BufferUnderflow {
                needed: 2,
                available: self.data.remaining(),
            });
        }
        Ok(self.data.get_u16_le())
    }

    pub fn get_u32(&mut self) -> Result<u32> {
        if self.data.remaining() < 4 {
            return Err(ProtocolError::BufferUnderflow {
                needed: 4,
                available: self.data.remaining(),
            });
        }
        Ok(self.data.get_u32_le())
    }

    pub fn get_u64(&mut self) -> Result<u64> {
        if self.data.remaining() < 8 {
            return Err(ProtocolError::BufferUnderflow {
                needed: 8,
                available: self.data.remaining(),
            });
        }
        Ok(self.data.get_u64_le())
    }

    pub fn get_i32(&mut self) -> Result<i32> {
        if self.data.remaining() < 4 {
            return Err(ProtocolError::BufferUnderflow {
                needed: 4,
                available: self.data.remaining(),
            });
        }
        Ok(self.data.get_i32_le())
    }

    pub fn get_string(&mut self) -> Result<String> {
        let len = self.get_u16()? as usize;
        if self.data.remaining() < len {
            return Err(ProtocolError::BufferUnderflow {
                needed: len,
                available: self.data.remaining(),
            });
        }
        let bytes = self.data.split_to(len);
        String::from_utf8(bytes.to_vec()).map_err(|_| ProtocolError::InvalidString)
    }

    pub fn get_bytes(&mut self, len: usize) -> Result<BytesMut> {
        if self.data.remaining() < len {
            return Err(ProtocolError::BufferUnderflow {
                needed: len,
                available: self.data.remaining(),
            });
        }
        Ok(self.data.split_to(len))
    }

    pub fn peek_u8(&self) -> Result<u8> {
        if self.data.is_empty() {
            return Err(ProtocolError::BufferUnderflow {
                needed: 1,
                available: 0,
            });
        }
        Ok(self.data[0])
    }

    // Writing methods
    pub fn put_u8(&mut self, value: u8) {
        self.data.put_u8(value);
    }

    pub fn put_u16(&mut self, value: u16) {
        self.data.put_u16_le(value);
    }

    pub fn put_u32(&mut self, value: u32) {
        self.data.put_u32_le(value);
    }

    pub fn put_u64(&mut self, value: u64) {
        self.data.put_u64_le(value);
    }

    pub fn put_i32(&mut self, value: i32) {
        self.data.put_i32_le(value);
    }

    pub fn put_f64(&mut self, value: f64) {
        self.data.put_f64_le(value);
    }

    pub fn put_string(&mut self, value: &str) {
        let bytes = value.as_bytes();
        self.data.put_u16_le(bytes.len() as u16);
        self.data.put_slice(bytes);
    }

    pub fn put_bytes(&mut self, bytes: &[u8]) {
        self.data.put_slice(bytes);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn remaining(&self) -> usize {
        self.data.remaining()
    }

    /// Pad message to multiple of 8 bytes for XTEA
    pub fn pad_for_xtea(&mut self) {
        let remainder = self.data.len() % 8;
        if remainder != 0 {
            let padding = 8 - remainder;
            for _ in 0..padding {
                self.data.put_u8(0x33); // Standard Tibia padding byte
            }
        }
    }
}

impl Default for NetworkMessage {
    fn default() -> Self {
        Self::new()
    }
}

/// Codec for decoding/encoding Tibia network packets
pub struct TibiaCodec {
    version: ProtocolVersion,
    xtea: Option<Xtea>,
    use_checksum: bool,
    sequence: u32,
}

impl TibiaCodec {
    pub fn new(version: ProtocolVersion) -> Self {
        Self {
            use_checksum: version.uses_checksum(),
            version,
            xtea: None,
            sequence: 0,
        }
    }

    pub fn set_xtea_key(&mut self, key: [u32; 4]) {
        self.xtea = Some(Xtea::new(key));
    }

    pub fn next_sequence(&mut self) -> u32 {
        let seq = self.sequence;
        self.sequence = self.sequence.wrapping_add(1);
        seq
    }
}

impl Decoder for TibiaCodec {
    type Item = NetworkMessage;
    type Error = ProtocolError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>> {
        // Need at least header to determine packet size
        if src.len() < HEADER_SIZE {
            return Ok(None);
        }

        // Read packet length (little-endian u16)
        let packet_len = u16::from_le_bytes([src[0], src[1]]) as usize;

        if packet_len > MAX_PACKET_SIZE {
            return Err(ProtocolError::PacketTooLarge {
                size: packet_len,
                max: MAX_PACKET_SIZE,
            });
        }

        // Wait for full packet
        let total_len = HEADER_SIZE + packet_len;
        if src.len() < total_len {
            src.reserve(total_len - src.len());
            return Ok(None);
        }

        // Extract the packet
        let _ = src.split_to(HEADER_SIZE); // Remove header
        let mut packet_data = src.split_to(packet_len);

        // Handle checksum if enabled
        if self.use_checksum && packet_data.len() >= 4 {
            let checksum = u32::from_le_bytes([
                packet_data[0],
                packet_data[1],
                packet_data[2],
                packet_data[3],
            ]);
            let data_for_checksum = &packet_data[4..];
            if !crate::crypto::verify_adler32(data_for_checksum, checksum) {
                return Err(ProtocolError::InvalidChecksum);
            }
            packet_data.advance(4); // Remove checksum
        }

        // Decrypt if XTEA key is set
        if let Some(ref xtea) = self.xtea {
            xtea.decrypt(&mut packet_data)?;

            // Remove internal length and padding
            if packet_data.len() >= 2 {
                let internal_len = u16::from_le_bytes([packet_data[0], packet_data[1]]) as usize;
                packet_data.advance(2);
                packet_data.truncate(internal_len);
            }
        }

        Ok(Some(NetworkMessage::from_bytes(packet_data)))
    }
}

impl Encoder<NetworkMessage> for TibiaCodec {
    type Error = ProtocolError;

    fn encode(&mut self, mut msg: NetworkMessage, dst: &mut BytesMut) -> Result<()> {
        let mut payload = BytesMut::new();

        // Add internal length and pad for XTEA if enabled
        if self.xtea.is_some() {
            payload.put_u16_le(msg.len() as u16);
            payload.put_slice(&msg.data);

            // Pad to multiple of 8
            let remainder = payload.len() % 8;
            if remainder != 0 {
                let padding = 8 - remainder;
                for _ in 0..padding {
                    payload.put_u8(0x33);
                }
            }

            // Encrypt
            if let Some(ref xtea) = self.xtea {
                xtea.encrypt(&mut payload)?;
            }
        } else {
            payload.put_slice(&msg.data);
        }

        // Add checksum if enabled
        let mut final_payload = BytesMut::new();
        if self.use_checksum {
            let checksum = adler32(&payload);
            final_payload.put_u32_le(checksum);
        }
        final_payload.put_slice(&payload);

        // Add packet header (length)
        dst.put_u16_le(final_payload.len() as u16);
        dst.put_slice(&final_payload);

        Ok(())
    }
}

/// Position in the game world
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: u16,
    pub y: u16,
    pub z: u8,
}

impl Position {
    pub fn new(x: u16, y: u16, z: u8) -> Self {
        Self { x, y, z }
    }

    pub fn read(msg: &mut NetworkMessage) -> Result<Self> {
        Ok(Self {
            x: msg.get_u16()?,
            y: msg.get_u16()?,
            z: msg.get_u8()?,
        })
    }

    pub fn write(&self, msg: &mut NetworkMessage) {
        msg.put_u16(self.x);
        msg.put_u16(self.y);
        msg.put_u8(self.z);
    }

    pub fn distance_to(&self, other: &Position) -> u32 {
        let dx = (self.x as i32 - other.x as i32).unsigned_abs();
        let dy = (self.y as i32 - other.y as i32).unsigned_abs();
        let dz = (self.z as i32 - other.z as i32).unsigned_abs();
        dx.max(dy).max(dz)
    }
}
