/**
 * Shadow OT Client - Network Protocol Implementation
 */

#include "protocol.h"
#include <cstring>
#include <algorithm>

#ifdef _WIN32
#include <winsock2.h>
#else
#include <arpa/inet.h>
#endif

namespace shadow {
namespace framework {

// XTEA implementation
static constexpr uint32_t XTEA_DELTA = 0x9E3779B9;
static constexpr uint32_t XTEA_ROUNDS = 32;

NetworkMessage::NetworkMessage() {
    reset();
}

void NetworkMessage::reset() {
    m_size = HEADER_SIZE;
    m_position = HEADER_SIZE;
    std::fill(m_buffer.begin(), m_buffer.end(), 0);
}

void NetworkMessage::writeByte(uint8_t value) {
    if (m_size < MAX_SIZE) {
        m_buffer[m_size++] = value;
    }
}

void NetworkMessage::writeU16(uint16_t value) {
    if (m_size + 2 <= MAX_SIZE) {
        m_buffer[m_size++] = value & 0xFF;
        m_buffer[m_size++] = (value >> 8) & 0xFF;
    }
}

void NetworkMessage::writeU32(uint32_t value) {
    if (m_size + 4 <= MAX_SIZE) {
        m_buffer[m_size++] = value & 0xFF;
        m_buffer[m_size++] = (value >> 8) & 0xFF;
        m_buffer[m_size++] = (value >> 16) & 0xFF;
        m_buffer[m_size++] = (value >> 24) & 0xFF;
    }
}

void NetworkMessage::writeU64(uint64_t value) {
    writeU32(static_cast<uint32_t>(value));
    writeU32(static_cast<uint32_t>(value >> 32));
}

void NetworkMessage::writeString(const std::string& value) {
    size_t len = std::min(value.length(), static_cast<size_t>(0xFFFF));
    writeU16(static_cast<uint16_t>(len));
    writeBytes(reinterpret_cast<const uint8_t*>(value.data()), len);
}

void NetworkMessage::writeBytes(const uint8_t* data, size_t length) {
    if (m_size + length <= MAX_SIZE) {
        std::memcpy(&m_buffer[m_size], data, length);
        m_size += length;
    }
}

void NetworkMessage::writePosition(uint16_t x, uint16_t y, uint8_t z) {
    writeU16(x);
    writeU16(y);
    writeByte(z);
}

uint8_t NetworkMessage::readByte() {
    if (m_position < m_size) {
        return m_buffer[m_position++];
    }
    return 0;
}

uint16_t NetworkMessage::readU16() {
    if (m_position + 2 <= m_size) {
        uint16_t value = m_buffer[m_position] | (m_buffer[m_position + 1] << 8);
        m_position += 2;
        return value;
    }
    return 0;
}

uint32_t NetworkMessage::readU32() {
    if (m_position + 4 <= m_size) {
        uint32_t value = m_buffer[m_position] |
                        (m_buffer[m_position + 1] << 8) |
                        (m_buffer[m_position + 2] << 16) |
                        (m_buffer[m_position + 3] << 24);
        m_position += 4;
        return value;
    }
    return 0;
}

uint64_t NetworkMessage::readU64() {
    uint64_t low = readU32();
    uint64_t high = readU32();
    return low | (high << 32);
}

std::string NetworkMessage::readString() {
    uint16_t len = readU16();
    if (m_position + len <= m_size) {
        std::string result(reinterpret_cast<char*>(&m_buffer[m_position]), len);
        m_position += len;
        return result;
    }
    return "";
}

void NetworkMessage::readBytes(uint8_t* data, size_t length) {
    if (m_position + length <= m_size) {
        std::memcpy(data, &m_buffer[m_position], length);
        m_position += length;
    }
}

void NetworkMessage::readPosition(uint16_t& x, uint16_t& y, uint8_t& z) {
    x = readU16();
    y = readU16();
    z = readByte();
}

uint8_t NetworkMessage::peekByte() const {
    if (m_position < m_size) {
        return m_buffer[m_position];
    }
    return 0;
}

uint16_t NetworkMessage::peekU16() const {
    if (m_position + 2 <= m_size) {
        return m_buffer[m_position] | (m_buffer[m_position + 1] << 8);
    }
    return 0;
}

// XTEA Cipher

XTEACipher::XTEACipher() {
    m_key.fill(0);
}

void XTEACipher::setKey(const std::array<uint32_t, 4>& key) {
    m_key = key;
    m_enabled = true;
}

void XTEACipher::encrypt(NetworkMessage& msg) const {
    if (!m_enabled) return;

    // Pad to 8-byte boundary
    size_t msgSize = msg.getBodySize();
    size_t paddedSize = ((msgSize + 7) / 8) * 8;

    // Get data pointer
    uint8_t* data = msg.getBodyBuffer();

    // Encrypt each 8-byte block
    for (size_t i = 0; i < paddedSize; i += 8) {
        uint32_t v0 = data[i] | (data[i+1] << 8) | (data[i+2] << 16) | (data[i+3] << 24);
        uint32_t v1 = data[i+4] | (data[i+5] << 8) | (data[i+6] << 16) | (data[i+7] << 24);

        uint32_t sum = 0;
        for (uint32_t j = 0; j < XTEA_ROUNDS; ++j) {
            v0 += (((v1 << 4) ^ (v1 >> 5)) + v1) ^ (sum + m_key[sum & 3]);
            sum += XTEA_DELTA;
            v1 += (((v0 << 4) ^ (v0 >> 5)) + v0) ^ (sum + m_key[(sum >> 11) & 3]);
        }

        data[i] = v0 & 0xFF;
        data[i+1] = (v0 >> 8) & 0xFF;
        data[i+2] = (v0 >> 16) & 0xFF;
        data[i+3] = (v0 >> 24) & 0xFF;
        data[i+4] = v1 & 0xFF;
        data[i+5] = (v1 >> 8) & 0xFF;
        data[i+6] = (v1 >> 16) & 0xFF;
        data[i+7] = (v1 >> 24) & 0xFF;
    }

    msg.setSize(NetworkMessage::HEADER_SIZE + paddedSize);
}

void XTEACipher::decrypt(NetworkMessage& msg) const {
    if (!m_enabled) return;

    size_t msgSize = msg.getBodySize();
    if (msgSize % 8 != 0) return; // Invalid size

    uint8_t* data = msg.getBodyBuffer();

    // Decrypt each 8-byte block
    for (size_t i = 0; i < msgSize; i += 8) {
        uint32_t v0 = data[i] | (data[i+1] << 8) | (data[i+2] << 16) | (data[i+3] << 24);
        uint32_t v1 = data[i+4] | (data[i+5] << 8) | (data[i+6] << 16) | (data[i+7] << 24);

        uint32_t sum = XTEA_DELTA * XTEA_ROUNDS;
        for (uint32_t j = 0; j < XTEA_ROUNDS; ++j) {
            v1 -= (((v0 << 4) ^ (v0 >> 5)) + v0) ^ (sum + m_key[(sum >> 11) & 3]);
            sum -= XTEA_DELTA;
            v0 -= (((v1 << 4) ^ (v1 >> 5)) + v1) ^ (sum + m_key[sum & 3]);
        }

        data[i] = v0 & 0xFF;
        data[i+1] = (v0 >> 8) & 0xFF;
        data[i+2] = (v0 >> 16) & 0xFF;
        data[i+3] = (v0 >> 24) & 0xFF;
        data[i+4] = v1 & 0xFF;
        data[i+5] = (v1 >> 8) & 0xFF;
        data[i+6] = (v1 >> 16) & 0xFF;
        data[i+7] = (v1 >> 24) & 0xFF;
    }
}

// RSA Cipher

struct RSACipher::Impl {
    // In a real implementation, this would use OpenSSL or similar
    // For now, we store the public key parameters
    std::string modulus;
    std::string exponent;
};

RSACipher& RSACipher::instance() {
    static RSACipher instance;
    return instance;
}

void RSACipher::setPublicKey(const std::string& n, const std::string& e) {
    if (!m_impl) {
        m_impl = std::make_unique<Impl>();
    }
    m_impl->modulus = n;
    m_impl->exponent = e;
}

bool RSACipher::encrypt(uint8_t* data, size_t length) const {
    if (!m_impl || length != 128) {
        return false;
    }

    // RSA encryption would be implemented here using OpenSSL
    // For now, return true as placeholder
    // In production, use:
    // - OpenSSL's RSA_public_encrypt
    // - Or a lightweight RSA implementation

    return true;
}

} // namespace framework
} // namespace shadow
