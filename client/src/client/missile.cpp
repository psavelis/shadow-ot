/**
 * Shadow OT Client - Missile Implementation
 */

#include "missile.h"
#include <cmath>
#include <algorithm>

namespace shadow {
namespace client {

Missile::Missile() : Thing() {
}

std::shared_ptr<Missile> Missile::create(uint16_t missileId,
                                          const Position& from,
                                          const Position& to) {
    auto missile = std::make_shared<Missile>();
    missile->setMissileId(missileId);
    missile->setTrajectory(from, to);
    return missile;
}

ThingType* Missile::getThingType() const {
    return ThingTypeManager::instance().getMissileType(m_missileId);
}

void Missile::setTrajectory(const Position& from, const Position& to) {
    m_source = from;
    m_destination = to;
    m_position = from;

    // Calculate pixel delta
    int dx = to.x - from.x;
    int dy = to.y - from.y;

    m_deltaX = static_cast<float>(dx * TILE_SIZE);
    m_deltaY = static_cast<float>(dy * TILE_SIZE);

    // Calculate duration based on distance
    float distance = std::sqrt(static_cast<float>(dx * dx + dy * dy));
    m_duration = distance / MISSILE_SPEED;

    // Ensure minimum duration
    if (m_duration < 0.1f) {
        m_duration = 0.1f;
    }

    // Initialize pixel position
    m_pixelX = static_cast<float>(from.x * TILE_SIZE);
    m_pixelY = static_cast<float>(from.y * TILE_SIZE);

    m_progress = 0.0f;
    m_finished = false;
}

void Missile::update(float deltaTime) {
    if (m_finished) return;

    m_progress += deltaTime / m_duration;

    if (m_progress >= 1.0f) {
        m_progress = 1.0f;
        m_finished = true;
    }

    // Update pixel position (linear interpolation)
    float startX = static_cast<float>(m_source.x * TILE_SIZE);
    float startY = static_cast<float>(m_source.y * TILE_SIZE);

    m_pixelX = startX + m_deltaX * m_progress;
    m_pixelY = startY + m_deltaY * m_progress;

    // Update current position for map queries
    if (m_progress < 0.5f) {
        m_position = m_source;
    } else {
        m_position = m_destination;
    }
}

int Missile::getDirectionPattern() const {
    // Calculate direction from source to destination
    // Returns pattern index 0-7 for 8 directions
    //
    // Pattern indices (OTClient convention):
    // 0 = North-West
    // 1 = North
    // 2 = North-East
    // 3 = West
    // 4 = East
    // 5 = South-West
    // 6 = South
    // 7 = South-East

    int dx = m_destination.x - m_source.x;
    int dy = m_destination.y - m_source.y;

    // Normalize to -1, 0, 1
    int ndx = (dx > 0) ? 1 : (dx < 0 ? -1 : 0);
    int ndy = (dy > 0) ? 1 : (dy < 0 ? -1 : 0);

    // Convert to pattern index
    // This maps the 8 directions to sprite patterns
    static const int directionMap[3][3] = {
        // dx=-1    dx=0     dx=1
        {0, 3, 5},  // dy=-1 (NW, W, SW)
        {1, 4, 6},  // dy=0  (N, center, S) - center unused
        {2, 4, 7}   // dy=1  (NE, E, SE)
    };

    // Adjust indices: -1->0, 0->1, 1->2
    int ix = ndx + 1;
    int iy = ndy + 1;

    return directionMap[iy][ix];
}

void Missile::draw(int x, int y, float scale) {
    if (m_finished) return;

    auto* type = getThingType();
    if (!type) return;

    // Get direction-based pattern
    int pattern = getDirectionPattern();

    // Missiles typically use patternX for direction
    int patternX = pattern % type->getPatternX();
    int patternY = pattern / type->getPatternX();

    type->draw(x, y, scale, patternX, patternY, 0, 0);
}

void Missile::drawAtPosition(int baseX, int baseY, float scale) {
    if (m_finished) return;

    // Calculate screen position based on pixel coordinates
    // baseX/baseY is the top-left of the visible area
    int screenX = static_cast<int>((m_pixelX - baseX) * scale);
    int screenY = static_cast<int>((m_pixelY - baseY) * scale);

    draw(screenX, screenY, scale);
}

uint8_t Missile::getLightIntensity() const {
    auto* type = getThingType();
    return type ? type->getLightIntensity() : 0;
}

uint8_t Missile::getLightColor() const {
    auto* type = getThingType();
    return type ? type->getLightColor() : 0;
}

// MissileManager implementation

MissileManager& MissileManager::instance() {
    static MissileManager instance;
    return instance;
}

MissilePtr MissileManager::createMissile(uint16_t missileId,
                                          const Position& from,
                                          const Position& to) {
    auto missile = Missile::create(missileId, from, to);
    m_missiles.push_back(missile);
    return missile;
}

void MissileManager::update(float deltaTime) {
    for (auto& missile : m_missiles) {
        missile->update(deltaTime);
    }

    // Remove finished missiles
    cleanup();
}

void MissileManager::draw(int viewX, int viewY, float scale) {
    for (const auto& missile : m_missiles) {
        if (!missile->isFinished()) {
            missile->drawAtPosition(viewX, viewY, scale);
        }
    }
}

void MissileManager::cleanup() {
    m_missiles.erase(
        std::remove_if(m_missiles.begin(), m_missiles.end(),
            [](const MissilePtr& m) { return m->isFinished(); }),
        m_missiles.end());
}

void MissileManager::clear() {
    m_missiles.clear();
}

} // namespace client
} // namespace shadow

// Global accessor
shadow::client::MissileManager& g_missiles = shadow::client::MissileManager::instance();
