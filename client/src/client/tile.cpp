/**
 * Shadow OT Client - Tile Implementation
 */

#include "tile.h"
#include "thingtype.h"
#include <framework/graphics/graphics.h>
#include <algorithm>

namespace shadow {
namespace client {

Tile::Tile(const Position& pos) : m_position(pos) {}

void Tile::setGround(ItemPtr item) {
    m_ground = item;
    if (item) {
        item->setTile(shared_from_this());
        item->setPosition(m_position);
        item->setStackPos(0);
    }
    updateFlags();
}

void Tile::addItem(ItemPtr item) {
    if (!item) return;

    item->setTile(shared_from_this());
    item->setPosition(m_position);

    // Insert based on item type
    // Ground items go first, then by layer
    if (item->isGround()) {
        setGround(item);
    } else {
        m_items.push_back(item);
        item->setStackPos(static_cast<int>(m_items.size()));
    }

    updateFlags();
}

void Tile::removeItem(ItemPtr item) {
    if (!item) return;

    if (m_ground == item) {
        m_ground = nullptr;
    } else {
        auto it = std::find(m_items.begin(), m_items.end(), item);
        if (it != m_items.end()) {
            m_items.erase(it);
        }
    }

    // Update stack positions
    for (size_t i = 0; i < m_items.size(); i++) {
        m_items[i]->setStackPos(static_cast<int>(i + 1));
    }

    updateFlags();
}

ItemPtr Tile::getItem(int stackPos) const {
    if (stackPos == 0) return m_ground;
    if (stackPos > 0 && stackPos <= static_cast<int>(m_items.size())) {
        return m_items[stackPos - 1];
    }
    return nullptr;
}

ItemPtr Tile::getTopItem() const {
    if (!m_items.empty()) {
        return m_items.back();
    }
    return m_ground;
}

void Tile::addCreature(CreaturePtr creature) {
    if (!creature) return;

    creature->setTile(shared_from_this());
    creature->setPosition(m_position);
    m_creatures.push_back(creature);

    updateFlags();
}

void Tile::removeCreature(CreaturePtr creature) {
    if (!creature) return;

    auto it = std::find(m_creatures.begin(), m_creatures.end(), creature);
    if (it != m_creatures.end()) {
        m_creatures.erase(it);
    }

    updateFlags();
}

CreaturePtr Tile::getCreature(int stackPos) const {
    if (stackPos >= 0 && stackPos < static_cast<int>(m_creatures.size())) {
        return m_creatures[stackPos];
    }
    return nullptr;
}

CreaturePtr Tile::getTopCreature() const {
    if (!m_creatures.empty()) {
        return m_creatures.back();
    }
    return nullptr;
}

void Tile::addEffect(ThingPtr effect) {
    if (!effect) return;

    effect->setTile(shared_from_this());
    effect->setPosition(m_position);
    m_effects.push_back(effect);
}

void Tile::removeEffect(ThingPtr effect) {
    if (!effect) return;

    auto it = std::find(m_effects.begin(), m_effects.end(), effect);
    if (it != m_effects.end()) {
        m_effects.erase(it);
    }
}

ThingPtr Tile::getThing(int stackPos) const {
    // Stack order: ground, items (bottom to top), creatures, effects

    if (stackPos == 0) return m_ground;

    int pos = 1;

    // Items
    for (const auto& item : m_items) {
        if (pos == stackPos) return item;
        pos++;
    }

    // Creatures
    for (const auto& creature : m_creatures) {
        if (pos == stackPos) return creature;
        pos++;
    }

    // Effects
    for (const auto& effect : m_effects) {
        if (pos == stackPos) return effect;
        pos++;
    }

    return nullptr;
}

int Tile::getThingCount() const {
    int count = m_ground ? 1 : 0;
    count += static_cast<int>(m_items.size());
    count += static_cast<int>(m_creatures.size());
    count += static_cast<int>(m_effects.size());
    return count;
}

bool Tile::isWalkable() const {
    // Check if tile is walkable
    if (!m_ground) return false;
    if (m_flags & TileFlag_Blocking) return false;

    // Check ground item
    if (m_ground && m_ground->blocksSolid()) return false;

    // Check items
    for (const auto& item : m_items) {
        if (item->blocksSolid()) return false;
    }

    // Check creatures
    for (const auto& creature : m_creatures) {
        if (creature->isUnpassable()) return false;
    }

    return true;
}

bool Tile::isPathable() const {
    if (!isWalkable()) return false;

    // Additional pathfind check
    if (m_ground && m_ground->blocksPathfind()) return false;

    for (const auto& item : m_items) {
        if (item->blocksPathfind()) return false;
    }

    return true;
}

bool Tile::isFullGround() const {
    if (!m_ground) return false;

    auto type = m_ground->getItemType();
    return type && type->isFullGround();
}

uint16_t Tile::getGroundSpeed() const {
    if (!m_ground) return 150;
    return m_ground->getSpeed();
}

uint8_t Tile::getLightIntensity() const {
    uint8_t maxIntensity = 0;

    if (m_ground) {
        maxIntensity = std::max(maxIntensity, m_ground->getLightIntensity());
    }

    for (const auto& item : m_items) {
        maxIntensity = std::max(maxIntensity, item->getLightIntensity());
    }

    for (const auto& creature : m_creatures) {
        maxIntensity = std::max(maxIntensity, creature->getLightIntensity());
    }

    return maxIntensity;
}

uint8_t Tile::getLightColor() const {
    // Return color from brightest light source
    uint8_t maxIntensity = 0;
    uint8_t color = 0;

    if (m_ground) {
        uint8_t intensity = m_ground->getLightIntensity();
        if (intensity > maxIntensity) {
            maxIntensity = intensity;
            color = m_ground->getLightColor();
        }
    }

    for (const auto& item : m_items) {
        uint8_t intensity = item->getLightIntensity();
        if (intensity > maxIntensity) {
            maxIntensity = intensity;
            color = item->getLightColor();
        }
    }

    for (const auto& creature : m_creatures) {
        uint8_t intensity = creature->getLightIntensity();
        if (intensity > maxIntensity) {
            maxIntensity = intensity;
            color = creature->getLightColor();
        }
    }

    return color;
}

int Tile::getElevation() const {
    int elevation = 0;

    for (const auto& item : m_items) {
        elevation += item->getElevation();
    }

    return elevation;
}

void Tile::draw(int x, int y, float scale) {
    // Draw order: ground, bottom items, creatures, top items, effects

    int elevation = 0;

    // Draw ground
    if (m_ground) {
        m_ground->draw(x, y, scale);
    }

    // Draw items (bottom layers first)
    for (const auto& item : m_items) {
        if (!item->isGround()) {
            item->draw(x, y - elevation, scale);
            elevation += item->getElevation();
        }
    }

    // Draw creatures
    for (const auto& creature : m_creatures) {
        creature->draw(x, y - elevation, scale);
    }

    // Draw effects
    for (const auto& effect : m_effects) {
        effect->draw(x, y, scale);
    }
}

void Tile::updateFlags() {
    m_flags = 0;

    // Update flags based on items and creatures
    if (m_ground) {
        if (m_ground->blocksSolid()) m_flags |= TileFlag_Blocking;
        if (m_ground->blocksProjectile()) m_flags |= TileFlag_BlockProjectile;
    }

    for (const auto& item : m_items) {
        if (item->blocksSolid()) m_flags |= TileFlag_Blocking;
        if (item->blocksProjectile()) m_flags |= TileFlag_BlockProjectile;
    }

    for (const auto& creature : m_creatures) {
        if (creature->isUnpassable()) m_flags |= TileFlag_Blocking;
    }
}

} // namespace client
} // namespace shadow
