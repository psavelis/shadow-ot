/**
 * Shadow OT Client - Network Connection
 *
 * Async TCP connection with protocol handling.
 */

#pragma once

#include <string>
#include <memory>
#include <functional>
#include <queue>
#include <mutex>
#include <atomic>
#include <thread>

#include "protocol.h"

namespace shadow {
namespace framework {

enum class ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Disconnecting,
    Error
};

class Connection {
public:
    Connection();
    ~Connection();

    // Connection management
    void connect(const std::string& host, uint16_t port);
    void disconnect();
    bool isConnected() const { return m_state == ConnectionState::Connected; }
    ConnectionState getState() const { return m_state; }

    // Message handling
    void send(NetworkMessage& msg);
    void poll();

    // Encryption
    void setXTEAKey(const std::array<uint32_t, 4>& key);
    void enableEncryption(bool enable);
    bool isEncrypted() const { return m_cipher.isEnabled(); }

    // Callbacks
    using ConnectCallback = std::function<void(bool success, const std::string& error)>;
    using DisconnectCallback = std::function<void()>;
    using MessageCallback = std::function<void(NetworkMessage&)>;
    using ErrorCallback = std::function<void(const std::string& error)>;

    void setConnectCallback(ConnectCallback callback) { m_connectCallback = callback; }
    void setDisconnectCallback(DisconnectCallback callback) { m_disconnectCallback = callback; }
    void setMessageCallback(MessageCallback callback) { m_messageCallback = callback; }
    void setErrorCallback(ErrorCallback callback) { m_errorCallback = callback; }

    // Stats
    uint64_t getBytesSent() const { return m_bytesSent; }
    uint64_t getBytesReceived() const { return m_bytesReceived; }
    int getPing() const { return m_ping; }
    void sendPing();

private:
    void readLoop();
    void writeLoop();
    void processIncoming(NetworkMessage& msg);
    void handleError(const std::string& error);

    std::atomic<ConnectionState> m_state{ConnectionState::Disconnected};
    XTEACipher m_cipher;

    // Threading
    std::unique_ptr<std::thread> m_readThread;
    std::unique_ptr<std::thread> m_writeThread;
    std::mutex m_sendMutex;
    std::mutex m_recvMutex;
    std::queue<NetworkMessage> m_sendQueue;
    std::queue<NetworkMessage> m_recvQueue;
    std::atomic<bool> m_running{false};

    // Callbacks
    ConnectCallback m_connectCallback;
    DisconnectCallback m_disconnectCallback;
    MessageCallback m_messageCallback;
    ErrorCallback m_errorCallback;

    // Stats
    std::atomic<uint64_t> m_bytesSent{0};
    std::atomic<uint64_t> m_bytesReceived{0};
    std::atomic<int> m_ping{0};
    uint64_t m_lastPingTime{0};

    // Socket implementation
    struct Impl;
    std::unique_ptr<Impl> m_impl;
};

// Protocol game handler
class ProtocolGame {
public:
    ProtocolGame();
    ~ProtocolGame();

    // Connection
    void connect(const std::string& host, uint16_t port,
                 const std::string& accountName, const std::string& password,
                 const std::string& characterName);
    void disconnect();
    bool isConnected() const;

    // Process network messages
    void poll();

    // Send game actions
    void sendWalk(int direction);
    void sendAutoWalk(const std::vector<int>& path);
    void sendStopWalk();
    void sendTurn(int direction);

    void sendLook(uint16_t x, uint16_t y, uint8_t z, uint16_t itemId, uint8_t stackPos);
    void sendUse(uint16_t x, uint16_t y, uint8_t z, uint16_t itemId, uint8_t stackPos, uint8_t index);
    void sendUseWith(uint16_t fromX, uint16_t fromY, uint8_t fromZ, uint16_t fromId, uint8_t fromStack,
                     uint16_t toX, uint16_t toY, uint8_t toZ, uint16_t toId, uint8_t toStack);
    void sendMove(uint16_t fromX, uint16_t fromY, uint8_t fromZ, uint16_t itemId, uint8_t fromStack,
                  uint16_t toX, uint16_t toY, uint8_t toZ, uint8_t count);

    void sendAttack(uint32_t creatureId);
    void sendFollow(uint32_t creatureId);
    void sendCancelAttack();
    void sendSetModes(uint8_t attack, uint8_t chase, uint8_t secure);

    void sendSay(uint8_t type, const std::string& receiver, uint16_t channelId, const std::string& text);
    void sendOpenChannel(uint16_t channelId);
    void sendCloseChannel(uint16_t channelId);
    void sendRequestChannels();

    void sendRequestOutfit();
    void sendSetOutfit(uint16_t lookType, uint8_t head, uint8_t body, uint8_t legs, uint8_t feet,
                       uint8_t addons, uint16_t mount);

    void sendCloseContainer(uint8_t containerId);
    void sendUpContainer(uint8_t containerId);

    void sendPing();
    void sendLogout();

    // Callbacks for game events
    using LoginCallback = std::function<void(bool success, const std::string& message)>;
    using MapCallback = std::function<void()>;
    using DeathCallback = std::function<void()>;

    void setLoginCallback(LoginCallback cb) { m_loginCallback = cb; }
    void setMapCallback(MapCallback cb) { m_mapCallback = cb; }
    void setDeathCallback(DeathCallback cb) { m_deathCallback = cb; }

private:
    void parseMessage(NetworkMessage& msg);
    void parseLoginSuccess(NetworkMessage& msg);
    void parseLoginError(NetworkMessage& msg);
    void parseMapDescription(NetworkMessage& msg);
    void parseMapMove(NetworkMessage& msg, int direction);
    void parseCreatureAppear(NetworkMessage& msg);
    void parseCreatureDisappear(NetworkMessage& msg);
    void parseCreatureMove(NetworkMessage& msg);
    void parseContainer(NetworkMessage& msg);
    void parseInventory(NetworkMessage& msg);
    void parsePlayerStats(NetworkMessage& msg);
    void parsePlayerSkills(NetworkMessage& msg);
    void parseTextMessage(NetworkMessage& msg);
    void parseSay(NetworkMessage& msg);

    std::unique_ptr<Connection> m_connection;
    std::string m_accountName;
    std::string m_password;
    std::string m_characterName;

    LoginCallback m_loginCallback;
    MapCallback m_mapCallback;
    DeathCallback m_deathCallback;
};

// Login protocol handler
class ProtocolLogin {
public:
    ProtocolLogin();
    ~ProtocolLogin();

    struct Character {
        std::string name;
        std::string world;
        std::string worldIp;
        uint16_t worldPort;
    };

    void login(const std::string& host, uint16_t port,
               const std::string& accountName, const std::string& password);
    void cancel();

    using CharacterListCallback = std::function<void(bool success, const std::string& message,
                                                     const std::vector<Character>& characters)>;
    void setCallback(CharacterListCallback cb) { m_callback = cb; }

    void poll();

private:
    void parseCharacterList(NetworkMessage& msg);
    void parseError(NetworkMessage& msg);

    std::unique_ptr<Connection> m_connection;
    CharacterListCallback m_callback;
};

} // namespace framework
} // namespace shadow
