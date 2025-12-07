/**
 * Shadow OT Client - Game Implementation
 */

#include "game.h"
#include "map.h"
#include "protocolgame.h"
#include <framework/net/connection.h>

namespace shadow {
namespace client {

Game& Game::instance() {
    static Game instance;
    return instance;
}

void Game::init() {
    m_gameState = GameState::NotConnected;
    m_localPlayer = nullptr;
}

void Game::terminate() {
    logout();
    m_localPlayer = nullptr;
}

void Game::login(const std::string& host, uint16_t port,
                 const std::string& account, const std::string& password) {
    if (m_gameState != GameState::NotConnected) return;

    m_accountName = account;
    m_password = password;
    m_loginHost = host;
    m_loginPort = port;
    m_gameState = GameState::Connecting;

    // Create login protocol and connect
    m_loginProtocol = std::make_unique<framework::ProtocolLogin>();
    m_loginProtocol->setCallback([this](bool success, const std::string& message,
                                        const std::vector<framework::ProtocolLogin::Character>& characters) {
        if (success) {
            m_gameState = GameState::CharacterList;
            if (m_onCharacterList) {
                m_onCharacterList(characters);
            }
        } else {
            m_gameState = GameState::NotConnected;
            if (m_onLoginError) {
                m_onLoginError(message);
            }
        }
        m_loginProtocol.reset();
    });

    m_loginProtocol->login(host, port, account, password);
}

void Game::loginWorld(const std::string& account, const std::string& password,
                      const std::string& worldHost, uint16_t worldPort,
                      const std::string& worldName, const std::string& characterName) {
    if (m_gameState != GameState::CharacterList) return;

    m_characterName = characterName;
    m_accountName = account;
    m_password = password;
    m_gameState = GameState::EnteringWorld;

    // Create game protocol and connect
    m_protocol = std::make_shared<ProtocolGame>();

    // Connect to game world
    if (m_protocol->connect(worldHost, worldPort, account, password, characterName)) {
        // Connection initiated, processLogin will be called on success
    } else {
        m_gameState = GameState::NotConnected;
        if (m_onLoginError) {
            m_onLoginError("Failed to connect to game server");
        }
    }
}

void Game::logout() {
    if (m_gameState == GameState::NotConnected) return;

    if (m_gameState == GameState::Online && m_protocol) {
        m_protocol->sendLogout();
    }

    processLogout();
}

void Game::poll() {
    // Poll login protocol if active
    if (m_loginProtocol) {
        m_loginProtocol->poll();
    }

    // Poll game protocol if active
    if (m_protocol) {
        m_protocol->poll();
    }
}

void Game::cancelLogin() {
    if (m_gameState == GameState::Connecting ||
        m_gameState == GameState::EnteringWorld) {
        // Cancel login protocol if active
        if (m_loginProtocol) {
            m_loginProtocol->cancel();
            m_loginProtocol.reset();
        }
        // Disconnect game protocol if active
        if (m_protocol) {
            m_protocol->disconnect();
            m_protocol.reset();
        }
        processLogout();
    }
}

void Game::processLogin() {
    m_gameState = GameState::Online;

    // Create local player if not exists
    if (!m_localPlayer) {
        m_localPlayer = LocalPlayer::create(0);
    }

    // Initialize map
    g_map.init();

    if (m_onLogin) {
        m_onLogin();
    }

    if (onGameStart) {
        onGameStart();
    }
}

void Game::processLogout() {
    if (m_gameState == GameState::Online && onGameEnd) {
        onGameEnd();
    }

    m_gameState = GameState::NotConnected;

    // Clear map
    g_map.clear();

    // Clear local player data
    m_localPlayer = nullptr;

    if (m_onLogout) {
        m_onLogout();
    }
}

void Game::walk(Position::Direction dir) {
    if (!isOnline() || !m_localPlayer) return;
    if (m_localPlayer->isWalkLocked()) return;

    // Pre-walk for client-side prediction
    m_localPlayer->preWalk(dir);

    // Send walk request to server
    if (m_protocol) {
        m_protocol->sendWalk(dir);
    }
}

void Game::autoWalk(const std::vector<Position> path) {
    if (!isOnline() || !m_localPlayer) return;
    if (path.empty()) return;

    // Convert positions to directions
    std::vector<Position::Direction> directions;
    Position current = m_localPlayer->getPosition();

    for (const auto& pos : path) {
        Position::Direction dir = current.directionTo(pos);
        if (dir != Position::InvalidDirection) {
            directions.push_back(dir);
        }
        current = pos;
    }

    m_localPlayer->setAutoWalkPath(directions);
    m_localPlayer->nextAutoWalkStep();
}

void Game::turn(Position::Direction dir) {
    if (!isOnline() || !m_localPlayer) return;

    m_localPlayer->turn(dir);

    if (m_protocol) {
        m_protocol->sendTurn(dir);
    }
}

void Game::stop() {
    if (!isOnline() || !m_localPlayer) return;

    m_localPlayer->cancelAutoWalk();
    m_localPlayer->cancelPreWalk();
    m_localPlayer->stopWalk();

    if (m_protocol) {
        m_protocol->sendStop();
    }
}

void Game::attack(uint32_t creatureId) {
    if (!isOnline() || !m_localPlayer) return;

    m_localPlayer->setAttackingCreatureId(creatureId);

    if (m_protocol) {
        m_protocol->sendAttack(creatureId);
    }
}

void Game::follow(uint32_t creatureId) {
    if (!isOnline() || !m_localPlayer) return;

    m_localPlayer->setFollowingCreatureId(creatureId);

    if (m_protocol) {
        m_protocol->sendFollow(creatureId);
    }
}

void Game::cancelAttackAndFollow() {
    if (!isOnline() || !m_localPlayer) return;

    m_localPlayer->setAttackingCreatureId(0);
    m_localPlayer->setFollowingCreatureId(0);

    if (m_protocol) {
        m_protocol->sendCancelAttackAndFollow();
    }
}

void Game::setAttackMode(uint8_t mode) {
    if (!isOnline() || !m_localPlayer) return;
    m_localPlayer->setAttackMode(static_cast<Player::AttackMode>(mode));
}

void Game::setChaseMode(uint8_t mode) {
    if (!isOnline() || !m_localPlayer) return;
    m_localPlayer->setChaseMode(static_cast<Player::ChaseMode>(mode));
}

void Game::setSecureMode(uint8_t mode) {
    if (!isOnline() || !m_localPlayer) return;
    m_localPlayer->setSecureMode(static_cast<Player::SecureMode>(mode));
}

void Game::setPvPMode(uint8_t mode) {
    if (!isOnline() || !m_localPlayer) return;
    m_localPlayer->setPvPMode(static_cast<Player::PvPMode>(mode));
}

void Game::look(const Position& pos, uint16_t itemId, uint8_t stackPos) {
    if (!isOnline()) return;
    if (m_protocol) {
        m_protocol->sendLook(pos, itemId, stackPos);
    }
}

void Game::use(const Position& pos, uint16_t itemId, uint8_t stackPos, uint8_t index) {
    if (!isOnline()) return;
    if (m_protocol) {
        m_protocol->sendUse(pos, itemId, stackPos, index);
    }
}

void Game::useWith(const Position& fromPos, uint16_t fromItemId, uint8_t fromStackPos,
                   const Position& toPos, uint16_t toItemId, uint8_t toStackPos) {
    if (!isOnline()) return;
    if (m_protocol) {
        m_protocol->sendUseWith(fromPos, fromItemId, fromStackPos, toPos, toItemId, toStackPos);
    }
}

void Game::move(const Position& fromPos, uint16_t itemId, uint8_t fromStackPos,
                const Position& toPos, uint8_t count) {
    if (!isOnline()) return;
    if (m_protocol) {
        m_protocol->sendMove(fromPos, itemId, fromStackPos, toPos, count);
    }
}

void Game::rotate(const Position& pos, uint16_t itemId, uint8_t stackPos) {
    if (!isOnline()) return;
    if (m_protocol) {
        m_protocol->sendRotate(pos, itemId, stackPos);
    }
}

void Game::openContainer(const Position& pos, uint16_t itemId, uint8_t stackPos) {
    if (!isOnline()) return;
    // Use 'use' to open containers
    if (m_protocol) {
        m_protocol->sendUse(pos, itemId, stackPos, 0);
    }
}

void Game::closeContainer(uint8_t containerId) {
    if (!isOnline()) return;
    if (m_protocol) {
        m_protocol->sendCloseContainer(containerId);
    }
}

void Game::upContainer(uint8_t containerId) {
    if (!isOnline()) return;
    if (m_protocol) {
        m_protocol->sendUpContainer(containerId);
    }
}

void Game::say(const std::string& text) {
    if (!isOnline() || text.empty()) return;
    if (m_protocol) {
        m_protocol->sendSay(SpeakType::Say, text);
    }
}

void Game::yell(const std::string& text) {
    if (!isOnline() || text.empty()) return;
    if (m_protocol) {
        m_protocol->sendSay(SpeakType::Yell, text);
    }
}

void Game::whisper(const std::string& text) {
    if (!isOnline() || text.empty()) return;
    if (m_protocol) {
        m_protocol->sendSay(SpeakType::Whisper, text);
    }
}

void Game::privateMessage(const std::string& receiver, const std::string& text) {
    if (!isOnline() || text.empty()) return;
    if (m_protocol) {
        m_protocol->sendSay(SpeakType::PrivateTo, text, receiver);
    }
}

void Game::channelMessage(uint16_t channelId, const std::string& text) {
    if (!isOnline() || text.empty()) return;
    if (m_protocol) {
        m_protocol->sendSay(SpeakType::Channel, text, "", channelId);
    }
}

void Game::requestChannels() {
    if (!isOnline()) return;
    if (m_protocol) {
        m_protocol->sendRequestChannels();
    }
}

void Game::openChannel(uint16_t channelId) {
    if (!isOnline()) return;
    if (m_protocol) {
        m_protocol->sendOpenChannel(channelId);
    }
}

void Game::closeChannel(uint16_t channelId) {
    if (!isOnline()) return;
    if (m_protocol) {
        m_protocol->sendCloseChannel(channelId);
    }
}

void Game::openPrivateChannel(const std::string& name) {
    if (!isOnline() || name.empty()) return;
    if (m_protocol) {
        m_protocol->sendOpenPrivateChannel(name);
    }
}

void Game::requestOutfit() {
    if (!isOnline()) return;
    if (m_protocol) {
        m_protocol->sendRequestOutfit();
    }
}

void Game::setOutfit(const Outfit& outfit) {
    if (!isOnline() || !m_localPlayer) return;
    m_localPlayer->setOutfit(outfit);
    if (m_protocol) {
        m_protocol->sendSetOutfit(outfit);
    }
}

void Game::requestTrade(const Position& pos, uint16_t itemId, uint8_t stackPos, uint32_t creatureId) {
    if (!isOnline()) return;
    // Player trade request - uses creature id to identify trade partner
    // Note: This would require a sendRequestTrade method in protocol
}

void Game::inspectTrade(bool counterOffer, uint8_t index) {
    if (!isOnline()) return;
    // Note: Trade inspection - would require sendInspectTrade method
}

void Game::acceptTrade() {
    if (!isOnline()) return;
    // Note: Would require sendAcceptTrade method
}

void Game::rejectTrade() {
    if (!isOnline()) return;
    // Note: Would require sendRejectTrade method
}

void Game::buyItem(uint16_t itemId, uint8_t subType, uint8_t amount, bool ignoreCapacity, bool buyWithBackpack) {
    if (!isOnline()) return;
    if (m_protocol) {
        m_protocol->sendBuyItem(itemId, subType, amount, ignoreCapacity, buyWithBackpack);
    }
}

void Game::sellItem(uint16_t itemId, uint8_t subType, uint8_t amount, bool ignoreEquipped) {
    if (!isOnline()) return;
    if (m_protocol) {
        m_protocol->sendSellItem(itemId, subType, amount, ignoreEquipped);
    }
}

void Game::closeNpcTrade() {
    if (!isOnline()) return;
    if (m_protocol) {
        m_protocol->sendCloseNpcTrade();
    }
}

void Game::browseMarket(uint16_t categoryId) {
    if (!isOnline()) return;
    // Note: Market functionality would require sendBrowseMarket method
}

void Game::createMarketOffer(uint8_t type, uint16_t itemId, uint8_t tier, uint16_t amount, uint64_t price, bool anonymous) {
    if (!isOnline()) return;
    // Note: Market offer creation would require sendCreateMarketOffer method
}

void Game::cancelMarketOffer(uint32_t timestamp, uint16_t counter) {
    if (!isOnline()) return;
    // Note: Market offer cancellation would require sendCancelMarketOffer method
}

void Game::acceptMarketOffer(uint32_t timestamp, uint16_t counter, uint16_t amount) {
    if (!isOnline()) return;
    // Note: Market offer acceptance would require sendAcceptMarketOffer method
}

void Game::inviteToParty(uint32_t creatureId) {
    if (!isOnline()) return;
    if (m_protocol) {
        m_protocol->sendInviteToParty(creatureId);
    }
}

void Game::joinParty(uint32_t creatureId) {
    if (!isOnline()) return;
    if (m_protocol) {
        m_protocol->sendJoinParty(creatureId);
    }
}

void Game::revokePartyInvite(uint32_t creatureId) {
    if (!isOnline()) return;
    if (m_protocol) {
        m_protocol->sendRevokeInvitation(creatureId);
    }
}

void Game::passPartyLeadership(uint32_t creatureId) {
    if (!isOnline()) return;
    if (m_protocol) {
        m_protocol->sendPassLeadership(creatureId);
    }
}

void Game::leaveParty() {
    if (!isOnline()) return;
    if (m_protocol) {
        m_protocol->sendLeaveParty();
    }
}

void Game::enableSharedExperience(bool enable) {
    if (!isOnline()) return;
    if (m_protocol) {
        m_protocol->sendEnableSharedExp(enable);
    }
}

void Game::addVip(const std::string& name) {
    if (!isOnline() || name.empty()) return;
    if (m_protocol) {
        m_protocol->sendAddVip(name);
    }
}

void Game::removeVip(uint32_t playerId) {
    if (!isOnline()) return;
    if (m_protocol) {
        m_protocol->sendRemoveVip(playerId);
    }
}

void Game::editVip(uint32_t playerId, const std::string& description, uint32_t iconId, bool notifyLogin) {
    if (!isOnline()) return;
    // Note: VIP editing would require sendEditVip method
}

void Game::requestQuestLog() {
    if (!isOnline()) return;
    if (m_protocol) {
        m_protocol->sendRequestQuestLog();
    }
}

void Game::requestQuestLine(uint16_t questId) {
    if (!isOnline()) return;
    if (m_protocol) {
        m_protocol->sendRequestQuestLine(questId);
    }
}

void Game::reportBug(const std::string& comment) {
    if (!isOnline() || comment.empty()) return;
    if (m_protocol) {
        m_protocol->sendBugReport(comment);
    }
}

void Game::reportRuleViolation(const std::string& target, uint8_t reason, uint8_t action,
                               const std::string& comment, const std::string& statement,
                               uint16_t channelId, uint32_t translation) {
    if (!isOnline()) return;
    // Note: Rule violation reporting would require sendReportRuleViolation method
}

void Game::ping() {
    if (!isOnline()) return;
    if (m_protocol) {
        m_protocol->sendPing();
    }
}

} // namespace client
} // namespace shadow

// Global accessor
shadow::client::Game& g_game = shadow::client::Game::instance();
