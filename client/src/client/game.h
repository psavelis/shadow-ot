/**
 * Shadow OT Client - Game
 *
 * Main game state and coordination.
 */

#pragma once

#include "localplayer.h"
#include "position.h"
#include "creature.h"
#include "protocolgame.h"
#include <string>
#include <memory>
#include <functional>
#include <vector>
#include <utility>

namespace shadow {
namespace client {

enum class GameState {
    NotConnected,
    Connecting,
    CharacterList,
    EnteringWorld,
    Online,
    Disconnecting
};

class Game {
public:
    static Game& instance();

    void init();
    void terminate();

    // Connection
    void login(const std::string& host, uint16_t port,
               const std::string& account, const std::string& password);
    void loginWorld(const std::string& account, const std::string& password,
                    const std::string& worldHost, uint16_t worldPort,
                    const std::string& worldName, const std::string& characterName);
    void logout();
    void cancelLogin();

    bool isOnline() const { return m_gameState == GameState::Online; }
    bool isConnecting() const { return m_gameState == GameState::Connecting; }
    GameState getGameState() const { return m_gameState; }

    // Local player
    LocalPlayerPtr getLocalPlayer() const { return m_localPlayer; }

    // Movement
    void walk(Position::Direction dir);
    void autoWalk(const std::vector<Position> path);
    void turn(Position::Direction dir);
    void stop();

    // Combat
    void attack(uint32_t creatureId);
    void follow(uint32_t creatureId);
    void cancelAttackAndFollow();

    void setAttackMode(uint8_t mode);
    void setChaseMode(uint8_t mode);
    void setSecureMode(uint8_t mode);
    void setPvPMode(uint8_t mode);

    // Item operations
    void look(const Position& pos, uint16_t itemId, uint8_t stackPos);
    void use(const Position& pos, uint16_t itemId, uint8_t stackPos, uint8_t index = 0);
    void useWith(const Position& fromPos, uint16_t fromItemId, uint8_t fromStackPos,
                 const Position& toPos, uint16_t toItemId, uint8_t toStackPos);
    void move(const Position& fromPos, uint16_t itemId, uint8_t fromStackPos,
              const Position& toPos, uint8_t count);
    void rotate(const Position& pos, uint16_t itemId, uint8_t stackPos);

    // Container
    void openContainer(const Position& pos, uint16_t itemId, uint8_t stackPos);
    void closeContainer(uint8_t containerId);
    void upContainer(uint8_t containerId);

    // Speaking
    void say(const std::string& text);
    void yell(const std::string& text);
    void whisper(const std::string& text);
    void privateMessage(const std::string& receiver, const std::string& text);
    void channelMessage(uint16_t channelId, const std::string& text);

    // Channels
    void requestChannels();
    void openChannel(uint16_t channelId);
    void closeChannel(uint16_t channelId);
    void openPrivateChannel(const std::string& name);

    // Outfit
    void requestOutfit();
    void setOutfit(const Outfit& outfit);

    // Trade
    void requestTrade(const Position& pos, uint16_t itemId, uint8_t stackPos, uint32_t creatureId);
    void inspectTrade(bool counterOffer, uint8_t index);
    void acceptTrade();
    void rejectTrade();

    // NPC trade
    void buyItem(uint16_t itemId, uint8_t subType, uint8_t amount, bool ignoreCapacity, bool buyWithBackpack);
    void sellItem(uint16_t itemId, uint8_t subType, uint8_t amount, bool ignoreEquipped);
    void closeNpcTrade();

    // Market
    void browseMarket(uint16_t categoryId);
    void createMarketOffer(uint8_t type, uint16_t itemId, uint8_t tier, uint16_t amount, uint64_t price, bool anonymous);
    void cancelMarketOffer(uint32_t timestamp, uint16_t counter);
    void acceptMarketOffer(uint32_t timestamp, uint16_t counter, uint16_t amount);

    // Party
    void inviteToParty(uint32_t creatureId);
    void joinParty(uint32_t creatureId);
    void revokePartyInvite(uint32_t creatureId);
    void passPartyLeadership(uint32_t creatureId);
    void leaveParty();
    void enableSharedExperience(bool enable);

    // VIP
    void addVip(const std::string& name);
    void removeVip(uint32_t playerId);
    void editVip(uint32_t playerId, const std::string& description, uint32_t iconId, bool notifyLogin);

    // Quest log
    void requestQuestLog();
    void requestQuestLine(uint16_t questId);

    // Bug report
    void reportBug(const std::string& comment);
    void reportRuleViolation(const std::string& target, uint8_t reason, uint8_t action,
                             const std::string& comment, const std::string& statement,
                             uint16_t channelId, uint32_t translation);

    // Ping
    void ping();
    int getLatency() const { return m_latency; }

    // Protocol version
    void setProtocolVersion(int version) { m_protocolVersion = version; }
    int getProtocolVersion() const { return m_protocolVersion; }

    // Callbacks
    using LoginCallback = std::function<void()>;
    using LogoutCallback = std::function<void()>;
    using DeathCallback = std::function<void(uint8_t deathType, uint8_t penalty)>;
    using TextMessageCallback = std::function<void(uint8_t type, const std::string& message)>;
    using AnimatedTextCallback = std::function<void(const Position& pos, uint8_t color, const std::string& text)>;
    using TalkCallback = std::function<void(const std::string& name, uint16_t level, uint8_t speakType,
                                            const Position& pos, uint16_t channelId, const std::string& text)>;
    using ChannelListCallback = std::function<void(const std::vector<std::pair<uint16_t, std::string>>& channels)>;
    using OpenChannelCallback = std::function<void(uint16_t channelId, const std::string& name)>;
    using CloseChannelCallback = std::function<void(uint16_t channelId)>;
    using VipStateCallback = std::function<void(uint32_t playerId, bool online)>;
    using OutfitDialogCallback = std::function<void(const Outfit& current,
                                                     const std::vector<std::pair<uint16_t, std::string>>& outfits,
                                                     const std::vector<std::pair<uint16_t, std::string>>& mounts)>;

    void setOnLogin(LoginCallback cb) { m_onLogin = cb; }
    void setOnLogout(LogoutCallback cb) { m_onLogout = cb; }
    void setOnDeath(DeathCallback cb) { m_onDeath = cb; }
    void setOnTextMessage(TextMessageCallback cb) { m_onTextMessage = cb; }

    // Public callbacks used by protocol parser
    DeathCallback onDeath;
    AnimatedTextCallback onAnimatedText;
    TalkCallback onTalk;
    ChannelListCallback onChannelList;
    OpenChannelCallback onOpenChannel;
    OpenChannelCallback onOpenPrivateChannel;
    CloseChannelCallback onCloseChannel;
    TextMessageCallback onTextMessage;
    VipStateCallback onVipStateChange;
    OutfitDialogCallback onOutfitDialog;

    // For module connection
    using GameStartCallback = std::function<void()>;
    using GameEndCallback = std::function<void()>;
    GameStartCallback onGameStart;
    GameEndCallback onGameEnd;

    // Protocol access for session management
    void setLocalPlayer(LocalPlayerPtr player) { m_localPlayer = player; }
    void setProtocol(std::shared_ptr<ProtocolGame> protocol) { m_protocol = protocol; }
    std::shared_ptr<ProtocolGame> getProtocol() const { return m_protocol; }
    void processLogin();
    void processLogout();

private:
    Game() = default;
    Game(const Game&) = delete;
    Game& operator=(const Game&) = delete;

    GameState m_gameState{GameState::NotConnected};
    LocalPlayerPtr m_localPlayer;
    std::shared_ptr<ProtocolGame> m_protocol;

    std::string m_accountName;
    std::string m_password;
    std::string m_characterName;

    int m_protocolVersion{1098};
    int m_latency{0};

    LoginCallback m_onLogin;
    LogoutCallback m_onLogout;
    DeathCallback m_onDeath;
    TextMessageCallback m_onTextMessage;
};

} // namespace client
} // namespace shadow

// Global accessor
extern shadow::client::Game& g_game;
