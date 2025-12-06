/**
 * Shadow OT Client - Network Protocol
 *
 * Tibia protocol implementation with XTEA encryption.
 */

#pragma once

#include <string>
#include <vector>
#include <memory>
#include <functional>
#include <cstdint>
#include <array>

namespace shadow {
namespace framework {

// Network message for read/write operations
class NetworkMessage {
public:
    static constexpr size_t MAX_SIZE = 65535;
    static constexpr size_t HEADER_SIZE = 2;

    NetworkMessage();
    void reset();

    // Writing
    void writeByte(uint8_t value);
    void writeU16(uint16_t value);
    void writeU32(uint32_t value);
    void writeU64(uint64_t value);
    void writeString(const std::string& value);
    void writeBytes(const uint8_t* data, size_t length);
    void writePosition(uint16_t x, uint16_t y, uint8_t z);

    // Reading
    uint8_t readByte();
    uint16_t readU16();
    uint32_t readU32();
    uint64_t readU64();
    std::string readString();
    void readBytes(uint8_t* data, size_t length);
    void readPosition(uint16_t& x, uint16_t& y, uint8_t& z);

    // Peek without advancing
    uint8_t peekByte() const;
    uint16_t peekU16() const;

    // Buffer access
    uint8_t* getBuffer() { return m_buffer.data(); }
    const uint8_t* getBuffer() const { return m_buffer.data(); }
    uint8_t* getBodyBuffer() { return m_buffer.data() + HEADER_SIZE; }

    size_t getSize() const { return m_size; }
    size_t getBodySize() const { return m_size > HEADER_SIZE ? m_size - HEADER_SIZE : 0; }
    size_t getPosition() const { return m_position; }
    size_t getRemainingSize() const { return m_size > m_position ? m_size - m_position : 0; }

    void setSize(size_t size) { m_size = size; }
    void setPosition(size_t pos) { m_position = pos; }

    bool isEof() const { return m_position >= m_size; }

private:
    std::array<uint8_t, MAX_SIZE> m_buffer;
    size_t m_size{0};
    size_t m_position{HEADER_SIZE};
};

// XTEA encryption
class XTEACipher {
public:
    XTEACipher();

    void setKey(const std::array<uint32_t, 4>& key);
    void encrypt(NetworkMessage& msg) const;
    void decrypt(NetworkMessage& msg) const;

    bool isEnabled() const { return m_enabled; }
    void setEnabled(bool enabled) { m_enabled = enabled; }

private:
    std::array<uint32_t, 4> m_key;
    bool m_enabled{false};
};

// RSA encryption for login
class RSACipher {
public:
    static RSACipher& instance();

    void setPublicKey(const std::string& n, const std::string& e);
    bool encrypt(uint8_t* data, size_t length) const;

private:
    RSACipher() = default;
    struct Impl;
    std::unique_ptr<Impl> m_impl;
};

// Protocol opcodes
namespace ClientOpcode {
    constexpr uint8_t Login = 0x01;
    constexpr uint8_t SecondaryLogin = 0x02;
    constexpr uint8_t EnterWorld = 0x0A;
    constexpr uint8_t QuitGame = 0x14;
    constexpr uint8_t Ping = 0x1E;

    constexpr uint8_t AutoWalk = 0x64;
    constexpr uint8_t WalkNorth = 0x65;
    constexpr uint8_t WalkEast = 0x66;
    constexpr uint8_t WalkSouth = 0x67;
    constexpr uint8_t WalkWest = 0x68;
    constexpr uint8_t StopWalk = 0x69;
    constexpr uint8_t WalkNE = 0x6A;
    constexpr uint8_t WalkSE = 0x6B;
    constexpr uint8_t WalkSW = 0x6C;
    constexpr uint8_t WalkNW = 0x6D;
    constexpr uint8_t TurnNorth = 0x6F;
    constexpr uint8_t TurnEast = 0x70;
    constexpr uint8_t TurnSouth = 0x71;
    constexpr uint8_t TurnWest = 0x72;

    constexpr uint8_t MoveItem = 0x78;
    constexpr uint8_t LookAt = 0x8C;
    constexpr uint8_t UseItem = 0x82;
    constexpr uint8_t UseItemWith = 0x83;
    constexpr uint8_t UseBattleWindow = 0x84;
    constexpr uint8_t RotateItem = 0x85;
    constexpr uint8_t CloseContainer = 0x87;
    constexpr uint8_t UpContainer = 0x88;

    constexpr uint8_t Say = 0x96;
    constexpr uint8_t RequestChannels = 0x97;
    constexpr uint8_t OpenChannel = 0x98;
    constexpr uint8_t CloseChannel = 0x99;
    constexpr uint8_t OpenPrivate = 0x9A;

    constexpr uint8_t Attack = 0xA1;
    constexpr uint8_t Follow = 0xA2;
    constexpr uint8_t CancelAttack = 0xBE;
    constexpr uint8_t SetModes = 0xA0;

    constexpr uint8_t RequestOutfit = 0xD2;
    constexpr uint8_t SetOutfit = 0xD3;

    constexpr uint8_t EditText = 0x89;
    constexpr uint8_t EditList = 0x8A;
}

namespace ServerOpcode {
    constexpr uint8_t LoginError = 0x0A;
    constexpr uint8_t LoginAdvice = 0x0B;
    constexpr uint8_t LoginWait = 0x0C;
    constexpr uint8_t LoginSuccess = 0x0D;
    constexpr uint8_t PendingGame = 0x0E;
    constexpr uint8_t CharacterList = 0x14;

    constexpr uint8_t Ping = 0x1D;
    constexpr uint8_t Death = 0x28;

    constexpr uint8_t MapDescription = 0x64;
    constexpr uint8_t MoveNorth = 0x65;
    constexpr uint8_t MoveEast = 0x66;
    constexpr uint8_t MoveSouth = 0x67;
    constexpr uint8_t MoveWest = 0x68;
    constexpr uint8_t UpdateTile = 0x69;

    constexpr uint8_t CreatureAppear = 0x6A;
    constexpr uint8_t CreatureDisappear = 0x6B;
    constexpr uint8_t CreatureTurn = 0x6B;
    constexpr uint8_t CreatureMove = 0x6D;

    constexpr uint8_t Container = 0x6E;
    constexpr uint8_t ContainerClose = 0x6F;
    constexpr uint8_t ContainerAddItem = 0x70;
    constexpr uint8_t ContainerUpdateItem = 0x71;
    constexpr uint8_t ContainerRemoveItem = 0x72;

    constexpr uint8_t Inventory = 0x78;
    constexpr uint8_t InventoryEmpty = 0x79;

    constexpr uint8_t WorldLight = 0x82;
    constexpr uint8_t Effect = 0x83;
    constexpr uint8_t AnimatedText = 0x84;
    constexpr uint8_t Missile = 0x85;
    constexpr uint8_t CreatureSquare = 0x86;

    constexpr uint8_t CreatureHealth = 0x8C;
    constexpr uint8_t CreatureLight = 0x8D;
    constexpr uint8_t CreatureOutfit = 0x8E;
    constexpr uint8_t CreatureSpeed = 0x8F;
    constexpr uint8_t CreatureSkull = 0x90;
    constexpr uint8_t CreatureShield = 0x91;

    constexpr uint8_t PlayerStats = 0xA0;
    constexpr uint8_t PlayerSkills = 0xA1;
    constexpr uint8_t Icons = 0xA2;
    constexpr uint8_t CancelTarget = 0xA3;

    constexpr uint8_t SpeakType = 0xAA;
    constexpr uint8_t ChannelList = 0xAB;
    constexpr uint8_t OpenChannel = 0xAC;
    constexpr uint8_t PrivateChannel = 0xAD;
    constexpr uint8_t CloseChannel = 0xB2;

    constexpr uint8_t TextMessage = 0xB4;
    constexpr uint8_t CancelWalk = 0xB5;
    constexpr uint8_t FloorChange = 0xBF;

    constexpr uint8_t OutfitDialog = 0xC8;
    constexpr uint8_t VipLogin = 0xD2;
    constexpr uint8_t VipLogout = 0xD3;
    constexpr uint8_t VipState = 0xD4;
}

} // namespace framework
} // namespace shadow
