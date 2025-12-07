/**
 * Shadow OT Client - Game Implementation
 */

#include "game.h"
#include "map.h"
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
    m_gameState = GameState::Connecting;

    // Create login protocol connection
    // This would connect to login server and get character list
}

void Game::loginWorld(const std::string& account, const std::string& password,
                      const std::string& worldHost, uint16_t worldPort,
                      const std::string& worldName, const std::string& characterName) {
    if (m_gameState != GameState::CharacterList) return;

    m_characterName = characterName;
    m_gameState = GameState::EnteringWorld;

    // Create game protocol connection
    // This would connect to game server with selected character
}

void Game::logout() {
    if (m_gameState == GameState::NotConnected) return;

    if (m_gameState == GameState::Online) {
        // Send logout request to server
    }

    processLogout();
}

void Game::cancelLogin() {
    if (m_gameState == GameState::Connecting ||
        m_gameState == GameState::EnteringWorld) {
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
    // Send say message to server (type 1)
}

void Game::yell(const std::string& text) {
    if (!isOnline() || text.empty()) return;
    // Send yell message to server (type 3)
}

void Game::whisper(const std::string& text) {
    if (!isOnline() || text.empty()) return;
    // Send whisper message to server (type 2)
}

void Game::privateMessage(const std::string& receiver, const std::string& text) {
    if (!isOnline() || text.empty()) return;
    // Send private message to server
}

void Game::channelMessage(uint16_t channelId, const std::string& text) {
    if (!isOnline() || text.empty()) return;
    // Send channel message to server
}

void Game::requestChannels() {
    if (!isOnline()) return;
    // Send request channels to server
}

void Game::openChannel(uint16_t channelId) {
    if (!isOnline()) return;
    // Send open channel request to server
}

void Game::closeChannel(uint16_t channelId) {
    if (!isOnline()) return;
    // Send close channel request to server
}

void Game::openPrivateChannel(const std::string& name) {
    if (!isOnline() || name.empty()) return;
    // Send open private channel request to server
}

void Game::requestOutfit() {
    if (!isOnline()) return;
    // Send outfit request to server
}

void Game::setOutfit(const Outfit& outfit) {
    if (!isOnline() || !m_localPlayer) return;
    m_localPlayer->setOutfit(outfit);
    // Send set outfit request to server
}

void Game::requestTrade(const Position& pos, uint16_t itemId, uint8_t stackPos, uint32_t creatureId) {
    if (!isOnline()) return;
    // Send trade request to server
}

void Game::inspectTrade(bool counterOffer, uint8_t index) {
    if (!isOnline()) return;
    // Send inspect trade request to server
}

void Game::acceptTrade() {
    if (!isOnline()) return;
    // Send accept trade request to server
}

void Game::rejectTrade() {
    if (!isOnline()) return;
    // Send reject trade request to server
}

void Game::buyItem(uint16_t itemId, uint8_t subType, uint8_t amount, bool ignoreCapacity, bool buyWithBackpack) {
    if (!isOnline()) return;
    // Send buy item request to server
}

void Game::sellItem(uint16_t itemId, uint8_t subType, uint8_t amount, bool ignoreEquipped) {
    if (!isOnline()) return;
    // Send sell item request to server
}

void Game::closeNpcTrade() {
    if (!isOnline()) return;
    // Send close NPC trade request to server
}

void Game::browseMarket(uint16_t categoryId) {
    if (!isOnline()) return;
    // Send browse market request to server
}

void Game::createMarketOffer(uint8_t type, uint16_t itemId, uint8_t tier, uint16_t amount, uint64_t price, bool anonymous) {
    if (!isOnline()) return;
    // Send create market offer request to server
}

void Game::cancelMarketOffer(uint32_t timestamp, uint16_t counter) {
    if (!isOnline()) return;
    // Send cancel market offer request to server
}

void Game::acceptMarketOffer(uint32_t timestamp, uint16_t counter, uint16_t amount) {
    if (!isOnline()) return;
    // Send accept market offer request to server
}

void Game::inviteToParty(uint32_t creatureId) {
    if (!isOnline()) return;
    // Send party invite request to server
}

void Game::joinParty(uint32_t creatureId) {
    if (!isOnline()) return;
    // Send join party request to server
}

void Game::revokePartyInvite(uint32_t creatureId) {
    if (!isOnline()) return;
    // Send revoke party invite request to server
}

void Game::passPartyLeadership(uint32_t creatureId) {
    if (!isOnline()) return;
    // Send pass leadership request to server
}

void Game::leaveParty() {
    if (!isOnline()) return;
    // Send leave party request to server
}

void Game::enableSharedExperience(bool enable) {
    if (!isOnline()) return;
    // Send shared experience request to server
}

void Game::addVip(const std::string& name) {
    if (!isOnline() || name.empty()) return;
    // Send add VIP request to server
}

void Game::removeVip(uint32_t playerId) {
    if (!isOnline()) return;
    // Send remove VIP request to server
}

void Game::editVip(uint32_t playerId, const std::string& description, uint32_t iconId, bool notifyLogin) {
    if (!isOnline()) return;
    // Send edit VIP request to server
}

void Game::requestQuestLog() {
    if (!isOnline()) return;
    // Send quest log request to server
}

void Game::requestQuestLine(uint16_t questId) {
    if (!isOnline()) return;
    // Send quest line request to server
}

void Game::reportBug(const std::string& comment) {
    if (!isOnline() || comment.empty()) return;
    // Send bug report to server
}

void Game::reportRuleViolation(const std::string& target, uint8_t reason, uint8_t action,
                               const std::string& comment, const std::string& statement,
                               uint16_t channelId, uint32_t translation) {
    if (!isOnline()) return;
    // Send rule violation report to server
}

void Game::ping() {
    if (!isOnline()) return;
    // Send ping to server
    // Server response would update m_latency
}

} // namespace client
} // namespace shadow

// Global accessor
shadow::client::Game& g_game = shadow::client::Game::instance();
