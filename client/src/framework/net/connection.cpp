/**
 * Shadow OT Client - Network Connection Implementation
 */

#include "connection.h"
#include <framework/core/application.h>

#ifdef _WIN32
#define WIN32_LEAN_AND_MEAN
#include <winsock2.h>
#include <ws2tcpip.h>
#pragma comment(lib, "ws2_32.lib")
typedef int socklen_t;
#else
#include <sys/socket.h>
#include <netinet/in.h>
#include <netinet/tcp.h>
#include <arpa/inet.h>
#include <netdb.h>
#include <unistd.h>
#include <fcntl.h>
#include <errno.h>
#define SOCKET int
#define INVALID_SOCKET -1
#define SOCKET_ERROR -1
#define closesocket close
#endif

namespace shadow {
namespace framework {

struct Connection::Impl {
    SOCKET socket{INVALID_SOCKET};
    std::string host;
    uint16_t port{0};

#ifdef _WIN32
    static bool wsaInitialized;
    static void initWSA() {
        if (!wsaInitialized) {
            WSADATA wsaData;
            WSAStartup(MAKEWORD(2, 2), &wsaData);
            wsaInitialized = true;
        }
    }
#endif
};

#ifdef _WIN32
bool Connection::Impl::wsaInitialized = false;
#endif

Connection::Connection() : m_impl(std::make_unique<Impl>()) {
#ifdef _WIN32
    Impl::initWSA();
#endif
}

Connection::~Connection() {
    disconnect();
}

void Connection::connect(const std::string& host, uint16_t port) {
    if (m_state == ConnectionState::Connected || m_state == ConnectionState::Connecting) {
        return;
    }

    m_state = ConnectionState::Connecting;
    m_impl->host = host;
    m_impl->port = port;

    // Create socket
    m_impl->socket = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
    if (m_impl->socket == INVALID_SOCKET) {
        handleError("Failed to create socket");
        return;
    }

    // Set non-blocking
#ifdef _WIN32
    u_long mode = 1;
    ioctlsocket(m_impl->socket, FIONBIO, &mode);
#else
    int flags = fcntl(m_impl->socket, F_GETFL, 0);
    fcntl(m_impl->socket, F_SETFL, flags | O_NONBLOCK);
#endif

    // Disable Nagle's algorithm
    int flag = 1;
    setsockopt(m_impl->socket, IPPROTO_TCP, TCP_NODELAY, (char*)&flag, sizeof(flag));

    // Resolve hostname
    struct addrinfo hints{}, *result = nullptr;
    hints.ai_family = AF_INET;
    hints.ai_socktype = SOCK_STREAM;

    std::string portStr = std::to_string(port);
    if (getaddrinfo(host.c_str(), portStr.c_str(), &hints, &result) != 0) {
        handleError("Failed to resolve hostname");
        return;
    }

    // Connect
    int connectResult = ::connect(m_impl->socket, result->ai_addr, static_cast<int>(result->ai_addrlen));
    freeaddrinfo(result);

#ifdef _WIN32
    if (connectResult == SOCKET_ERROR && WSAGetLastError() != WSAEWOULDBLOCK) {
#else
    if (connectResult == -1 && errno != EINPROGRESS) {
#endif
        handleError("Connection failed");
        return;
    }

    // Start read/write threads
    m_running = true;

    m_readThread = std::make_unique<std::thread>([this]() {
        readLoop();
    });

    m_writeThread = std::make_unique<std::thread>([this]() {
        writeLoop();
    });

    // Wait for connection to complete
    fd_set writeSet;
    FD_ZERO(&writeSet);
    FD_SET(m_impl->socket, &writeSet);

    struct timeval timeout;
    timeout.tv_sec = 10;
    timeout.tv_usec = 0;

    int selectResult = select(static_cast<int>(m_impl->socket) + 1, nullptr, &writeSet, nullptr, &timeout);

    if (selectResult > 0) {
        int error = 0;
        socklen_t len = sizeof(error);
        getsockopt(m_impl->socket, SOL_SOCKET, SO_ERROR, (char*)&error, &len);

        if (error == 0) {
            m_state = ConnectionState::Connected;
            if (m_connectCallback) {
                m_connectCallback(true, "");
            }
        } else {
            handleError("Connection failed");
        }
    } else {
        handleError("Connection timeout");
    }
}

void Connection::disconnect() {
    if (m_state == ConnectionState::Disconnected) {
        return;
    }

    m_state = ConnectionState::Disconnecting;
    m_running = false;

    if (m_impl->socket != INVALID_SOCKET) {
        closesocket(m_impl->socket);
        m_impl->socket = INVALID_SOCKET;
    }

    if (m_readThread && m_readThread->joinable()) {
        m_readThread->join();
    }
    if (m_writeThread && m_writeThread->joinable()) {
        m_writeThread->join();
    }

    m_state = ConnectionState::Disconnected;

    if (m_disconnectCallback) {
        m_disconnectCallback();
    }
}

void Connection::send(NetworkMessage& msg) {
    if (m_state != ConnectionState::Connected) {
        return;
    }

    // Encrypt if enabled
    m_cipher.encrypt(msg);

    // Set message size in header
    uint16_t size = static_cast<uint16_t>(msg.getBodySize());
    msg.getBuffer()[0] = size & 0xFF;
    msg.getBuffer()[1] = (size >> 8) & 0xFF;

    std::lock_guard<std::mutex> lock(m_sendMutex);
    m_sendQueue.push(msg);
}

void Connection::poll() {
    // Process received messages
    while (true) {
        NetworkMessage msg;
        {
            std::lock_guard<std::mutex> lock(m_recvMutex);
            if (m_recvQueue.empty()) break;
            msg = m_recvQueue.front();
            m_recvQueue.pop();
        }

        processIncoming(msg);
    }
}

void Connection::setXTEAKey(const std::array<uint32_t, 4>& key) {
    m_cipher.setKey(key);
}

void Connection::enableEncryption(bool enable) {
    m_cipher.setEnabled(enable);
}

void Connection::sendPing() {
    m_lastPingTime = g_app.getMilliseconds();

    NetworkMessage msg;
    msg.writeByte(ClientOpcode::Ping);
    send(msg);
}

void Connection::readLoop() {
    std::vector<uint8_t> buffer(NetworkMessage::MAX_SIZE);

    while (m_running && m_impl->socket != INVALID_SOCKET) {
        fd_set readSet;
        FD_ZERO(&readSet);
        FD_SET(m_impl->socket, &readSet);

        struct timeval timeout;
        timeout.tv_sec = 0;
        timeout.tv_usec = 100000; // 100ms

        int selectResult = select(static_cast<int>(m_impl->socket) + 1, &readSet, nullptr, nullptr, &timeout);

        if (selectResult > 0 && FD_ISSET(m_impl->socket, &readSet)) {
            // Read header (2 bytes)
            int headerRead = recv(m_impl->socket, (char*)buffer.data(), 2, MSG_WAITALL);
            if (headerRead <= 0) {
                if (m_running) {
                    handleError("Connection closed");
                }
                break;
            }

            uint16_t messageSize = buffer[0] | (buffer[1] << 8);
            if (messageSize > NetworkMessage::MAX_SIZE - 2) {
                handleError("Invalid message size");
                break;
            }

            // Read body
            int bodyRead = recv(m_impl->socket, (char*)buffer.data() + 2, messageSize, MSG_WAITALL);
            if (bodyRead != messageSize) {
                handleError("Incomplete message");
                break;
            }

            m_bytesReceived += headerRead + bodyRead;

            // Create message
            NetworkMessage msg;
            std::memcpy(msg.getBuffer(), buffer.data(), 2 + messageSize);
            msg.setSize(2 + messageSize);
            msg.setPosition(2);

            // Decrypt if enabled
            m_cipher.decrypt(msg);

            std::lock_guard<std::mutex> lock(m_recvMutex);
            m_recvQueue.push(msg);
        }
    }
}

void Connection::writeLoop() {
    while (m_running && m_impl->socket != INVALID_SOCKET) {
        NetworkMessage msg;
        {
            std::lock_guard<std::mutex> lock(m_sendMutex);
            if (m_sendQueue.empty()) {
                std::this_thread::sleep_for(std::chrono::milliseconds(10));
                continue;
            }
            msg = m_sendQueue.front();
            m_sendQueue.pop();
        }

        // Send message
        int totalSent = 0;
        int messageSize = static_cast<int>(msg.getSize());

        while (totalSent < messageSize) {
            int sent = ::send(m_impl->socket, (char*)msg.getBuffer() + totalSent,
                            messageSize - totalSent, 0);
            if (sent <= 0) {
                handleError("Send failed");
                return;
            }
            totalSent += sent;
        }

        m_bytesSent += totalSent;
    }
}

void Connection::processIncoming(NetworkMessage& msg) {
    if (m_messageCallback) {
        m_messageCallback(msg);
    }
}

void Connection::handleError(const std::string& error) {
    m_state = ConnectionState::Error;

    if (m_errorCallback) {
        m_errorCallback(error);
    }

    if (m_connectCallback) {
        m_connectCallback(false, error);
    }

    disconnect();
}

// ProtocolGame Implementation

ProtocolGame::ProtocolGame() : m_connection(std::make_unique<Connection>()) {
}

ProtocolGame::~ProtocolGame() {
    disconnect();
}

void ProtocolGame::connect(const std::string& host, uint16_t port,
                           const std::string& accountName, const std::string& password,
                           const std::string& characterName) {
    m_accountName = accountName;
    m_password = password;
    m_characterName = characterName;

    m_connection->setConnectCallback([this](bool success, const std::string& error) {
        if (success) {
            // Send login packet
            NetworkMessage msg;
            msg.writeByte(ClientOpcode::EnterWorld);
            // ... additional login data would go here
            m_connection->send(msg);
        } else {
            if (m_loginCallback) {
                m_loginCallback(false, error);
            }
        }
    });

    m_connection->setMessageCallback([this](NetworkMessage& msg) {
        parseMessage(msg);
    });

    m_connection->setDisconnectCallback([this]() {
        // Handle disconnect
    });

    m_connection->connect(host, port);
}

void ProtocolGame::disconnect() {
    m_connection->disconnect();
}

bool ProtocolGame::isConnected() const {
    return m_connection->isConnected();
}

void ProtocolGame::poll() {
    m_connection->poll();
}

void ProtocolGame::sendWalk(int direction) {
    NetworkMessage msg;
    msg.writeByte(ClientOpcode::WalkNorth + direction);
    m_connection->send(msg);
}

void ProtocolGame::sendAutoWalk(const std::vector<int>& path) {
    NetworkMessage msg;
    msg.writeByte(ClientOpcode::AutoWalk);
    msg.writeByte(static_cast<uint8_t>(path.size()));
    for (int dir : path) {
        msg.writeByte(static_cast<uint8_t>(dir));
    }
    m_connection->send(msg);
}

void ProtocolGame::sendStopWalk() {
    NetworkMessage msg;
    msg.writeByte(ClientOpcode::StopWalk);
    m_connection->send(msg);
}

void ProtocolGame::sendTurn(int direction) {
    NetworkMessage msg;
    msg.writeByte(ClientOpcode::TurnNorth + direction);
    m_connection->send(msg);
}

void ProtocolGame::sendAttack(uint32_t creatureId) {
    NetworkMessage msg;
    msg.writeByte(ClientOpcode::Attack);
    msg.writeU32(creatureId);
    msg.writeU32(0); // Sequence
    m_connection->send(msg);
}

void ProtocolGame::sendFollow(uint32_t creatureId) {
    NetworkMessage msg;
    msg.writeByte(ClientOpcode::Follow);
    msg.writeU32(creatureId);
    msg.writeU32(0);
    m_connection->send(msg);
}

void ProtocolGame::sendCancelAttack() {
    NetworkMessage msg;
    msg.writeByte(ClientOpcode::CancelAttack);
    m_connection->send(msg);
}

void ProtocolGame::sendSay(uint8_t type, const std::string& receiver, uint16_t channelId, const std::string& text) {
    NetworkMessage msg;
    msg.writeByte(ClientOpcode::Say);
    msg.writeByte(type);

    switch (type) {
        case 4: // Private
        case 5: // Private Red
            msg.writeString(receiver);
            break;
        case 7: // Channel
        case 10: // Channel Red
            msg.writeU16(channelId);
            break;
    }

    msg.writeString(text);
    m_connection->send(msg);
}

void ProtocolGame::sendPing() {
    NetworkMessage msg;
    msg.writeByte(ClientOpcode::Ping);
    m_connection->send(msg);
}

void ProtocolGame::sendLogout() {
    NetworkMessage msg;
    msg.writeByte(ClientOpcode::QuitGame);
    m_connection->send(msg);
}

void ProtocolGame::parseMessage(NetworkMessage& msg) {
    uint8_t opcode = msg.readByte();

    switch (opcode) {
        case ServerOpcode::LoginSuccess:
            parseLoginSuccess(msg);
            break;
        case ServerOpcode::LoginError:
            parseLoginError(msg);
            break;
        case ServerOpcode::MapDescription:
            parseMapDescription(msg);
            break;
        case ServerOpcode::MoveNorth:
        case ServerOpcode::MoveEast:
        case ServerOpcode::MoveSouth:
        case ServerOpcode::MoveWest:
            parseMapMove(msg, opcode - ServerOpcode::MoveNorth);
            break;
        case ServerOpcode::PlayerStats:
            parsePlayerStats(msg);
            break;
        case ServerOpcode::PlayerSkills:
            parsePlayerSkills(msg);
            break;
        case ServerOpcode::TextMessage:
            parseTextMessage(msg);
            break;
        case ServerOpcode::Ping:
            sendPing(); // Respond to ping
            break;
        case ServerOpcode::Death:
            if (m_deathCallback) {
                m_deathCallback();
            }
            break;
    }
}

void ProtocolGame::parseLoginSuccess(NetworkMessage& msg) {
    if (m_loginCallback) {
        m_loginCallback(true, "");
    }
}

void ProtocolGame::parseLoginError(NetworkMessage& msg) {
    std::string error = msg.readString();
    if (m_loginCallback) {
        m_loginCallback(false, error);
    }
}

void ProtocolGame::parseMapDescription(NetworkMessage& msg) {
    // Parse map data
    if (m_mapCallback) {
        m_mapCallback();
    }
}

void ProtocolGame::parseMapMove(NetworkMessage& msg, int direction) {
    // Parse map shift
}

void ProtocolGame::parsePlayerStats(NetworkMessage& msg) {
    // Parse player stats
}

void ProtocolGame::parsePlayerSkills(NetworkMessage& msg) {
    // Parse player skills
}

void ProtocolGame::parseTextMessage(NetworkMessage& msg) {
    // Parse text message
}

// ProtocolLogin Implementation

ProtocolLogin::ProtocolLogin() : m_connection(std::make_unique<Connection>()) {
}

ProtocolLogin::~ProtocolLogin() {
    cancel();
}

void ProtocolLogin::login(const std::string& host, uint16_t port,
                          const std::string& accountName, const std::string& password) {
    m_connection->setConnectCallback([this, accountName, password](bool success, const std::string& error) {
        if (success) {
            // Send login packet
            NetworkMessage msg;
            msg.writeByte(ClientOpcode::Login);
            msg.writeU16(0); // OS
            msg.writeU16(1098); // Protocol version
            // RSA-encrypted block would go here
            msg.writeString(accountName);
            msg.writeString(password);
            m_connection->send(msg);
        } else {
            if (m_callback) {
                m_callback(false, error, {});
            }
        }
    });

    m_connection->setMessageCallback([this](NetworkMessage& msg) {
        uint8_t opcode = msg.readByte();
        if (opcode == ServerOpcode::CharacterList) {
            parseCharacterList(msg);
        } else if (opcode == ServerOpcode::LoginError) {
            parseError(msg);
        }
    });

    m_connection->connect(host, port);
}

void ProtocolLogin::cancel() {
    m_connection->disconnect();
}

void ProtocolLogin::poll() {
    m_connection->poll();
}

void ProtocolLogin::parseCharacterList(NetworkMessage& msg) {
    std::vector<Character> characters;

    uint8_t motdLen = msg.readByte();
    if (motdLen > 0) {
        msg.readString(); // MOTD
    }

    uint8_t charCount = msg.readByte();
    for (uint8_t i = 0; i < charCount; ++i) {
        Character c;
        c.name = msg.readString();
        c.world = msg.readString();
        uint32_t ip = msg.readU32();
        c.worldPort = msg.readU16();

        // Convert IP to string
        struct in_addr addr;
        addr.s_addr = ip;
        c.worldIp = inet_ntoa(addr);

        characters.push_back(c);
    }

    uint32_t premiumDays = msg.readU32();

    m_connection->disconnect();

    if (m_callback) {
        m_callback(true, "", characters);
    }
}

void ProtocolLogin::parseError(NetworkMessage& msg) {
    std::string error = msg.readString();
    m_connection->disconnect();

    if (m_callback) {
        m_callback(false, error, {});
    }
}

} // namespace framework
} // namespace shadow
