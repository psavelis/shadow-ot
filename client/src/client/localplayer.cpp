/**
 * Shadow OT Client - Local Player Implementation
 */

#include "localplayer.h"
#include "map.h"
#include <algorithm>

namespace shadow {
namespace client {

LocalPlayer::LocalPlayer() : Player() {
}

std::shared_ptr<LocalPlayer> LocalPlayer::create(uint32_t id) {
    auto player = std::make_shared<LocalPlayer>();
    player->setCreatureId(id);
    return player;
}

void LocalPlayer::setAutoWalkPath(const std::vector<Position::Direction>& path) {
    m_autoWalkPath = path;
    m_autoWalkIndex = 0;
}

void LocalPlayer::cancelAutoWalk() {
    m_autoWalkPath.clear();
    m_autoWalkIndex = 0;
}

void LocalPlayer::nextAutoWalkStep() {
    if (m_autoWalkPath.empty() || m_autoWalkIndex >= static_cast<int>(m_autoWalkPath.size())) {
        cancelAutoWalk();
        return;
    }

    Position::Direction dir = m_autoWalkPath[m_autoWalkIndex++];

    // Pre-walk for client-side prediction
    preWalk(dir);

    // Check if path completed
    if (m_autoWalkIndex >= static_cast<int>(m_autoWalkPath.size())) {
        m_autoWalkPath.clear();
        m_autoWalkIndex = 0;
    }
}

void LocalPlayer::preWalk(Position::Direction dir) {
    if (m_preWalking || isWalking()) return;

    m_preWalking = true;
    m_preWalkPosition = m_position;

    // Calculate new position
    Position newPos = m_position.translated(dir);

    // Check if tile is walkable
    auto& map = g_map;
    auto tile = map.getTile(newPos);
    if (tile && tile->isWalkable()) {
        walk(newPos, true);
    } else {
        cancelPreWalk();
    }
}

void LocalPlayer::cancelPreWalk() {
    if (!m_preWalking) return;

    m_preWalking = false;

    // Revert to pre-walk position
    if (isWalking()) {
        cancelWalk();
        m_position = m_preWalkPosition;
    }
}

void LocalPlayer::addKnownSpell(uint16_t spellId) {
    if (!knowsSpell(spellId)) {
        m_knownSpells.push_back(spellId);
    }
}

void LocalPlayer::removeKnownSpell(uint16_t spellId) {
    auto it = std::find(m_knownSpells.begin(), m_knownSpells.end(), spellId);
    if (it != m_knownSpells.end()) {
        m_knownSpells.erase(it);
    }
}

bool LocalPlayer::knowsSpell(uint16_t spellId) const {
    return std::find(m_knownSpells.begin(), m_knownSpells.end(), spellId) != m_knownSpells.end();
}

void LocalPlayer::addVIP(const VIPEntry& vip) {
    // Check if already exists
    for (auto& v : m_vipList) {
        if (v.id == vip.id) {
            v = vip;
            return;
        }
    }
    m_vipList.push_back(vip);
}

void LocalPlayer::removeVIP(uint32_t id) {
    m_vipList.erase(
        std::remove_if(m_vipList.begin(), m_vipList.end(),
                       [id](const VIPEntry& v) { return v.id == id; }),
        m_vipList.end());
}

void LocalPlayer::setVIPOnline(uint32_t id, bool online) {
    for (auto& vip : m_vipList) {
        if (vip.id == id) {
            vip.online = online;
            break;
        }
    }
}

void LocalPlayer::update(float deltaTime) {
    Position oldPos = m_position;

    // Call parent update (handles walking animation, etc.)
    Player::update(deltaTime);

    // Handle pre-walk completion
    if (m_preWalking && !isWalking()) {
        m_preWalking = false;

        // Continue auto-walk if in progress
        if (isAutoWalking()) {
            nextAutoWalkStep();
        }
    }

    // Notify position change
    if (oldPos != m_position && m_onPositionChange) {
        m_onPositionChange(oldPos, m_position);
    }
}

} // namespace client
} // namespace shadow
