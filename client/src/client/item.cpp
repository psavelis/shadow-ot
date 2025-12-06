/**
 * Shadow OT Client - Item Implementation
 */

#include "item.h"
#include "thingtype.h"
#include <framework/graphics/graphics.h>

// g_graphics is declared in shadow::framework namespace
using shadow::framework::g_graphics;

namespace shadow {
namespace client {

Item::Item() = default;

std::shared_ptr<Item> Item::create(uint16_t id) {
    auto item = std::make_shared<Item>();
    item->setId(id);
    return item;
}

void Item::setId(uint16_t id) {
    m_id = id;
    // Reset pattern and animation when ID changes
    m_patternX = 0;
    m_patternY = 0;
    m_patternZ = 0;
    m_animationPhase = 0;
}

ThingType* Item::getItemType() const {
    return ThingTypeManager::instance().getItemType(m_id);
}

bool Item::isStackable() const {
    if (auto type = getItemType()) {
        return type->isStackable();
    }
    return false;
}

bool Item::isGround() const {
    if (auto type = getItemType()) {
        return type->isGround();
    }
    return false;
}

bool Item::isContainer() const {
    if (auto type = getItemType()) {
        return type->isContainer();
    }
    return false;
}

bool Item::isFluid() const {
    if (auto type = getItemType()) {
        return type->isFluid();
    }
    return false;
}

bool Item::isSplash() const {
    if (auto type = getItemType()) {
        return type->isSplash();
    }
    return false;
}

bool Item::isMoveable() const {
    if (auto type = getItemType()) {
        return type->isMoveable();
    }
    return true;
}

bool Item::isPickupable() const {
    if (auto type = getItemType()) {
        return type->isPickupable();
    }
    return false;
}

bool Item::isUseable() const {
    if (auto type = getItemType()) {
        return type->isUseable();
    }
    return false;
}

bool Item::isWriteable() const {
    if (auto type = getItemType()) {
        return type->isWriteable();
    }
    return false;
}

bool Item::isReadable() const {
    if (auto type = getItemType()) {
        return type->isReadable();
    }
    return false;
}

bool Item::blocksSolid() const {
    if (auto type = getItemType()) {
        return type->blocksSolid();
    }
    return false;
}

bool Item::blocksProjectile() const {
    if (auto type = getItemType()) {
        return type->blocksProjectile();
    }
    return false;
}

bool Item::blocksPathfind() const {
    if (auto type = getItemType()) {
        return type->blocksPathfind();
    }
    return false;
}

uint16_t Item::getSpeed() const {
    if (auto type = getItemType()) {
        return type->getSpeed();
    }
    return 0;
}

uint8_t Item::getLightIntensity() const {
    if (auto type = getItemType()) {
        return type->getLightIntensity();
    }
    return 0;
}

uint8_t Item::getLightColor() const {
    if (auto type = getItemType()) {
        return type->getLightColor();
    }
    return 0;
}

int Item::getElevation() const {
    if (auto type = getItemType()) {
        return type->getElevation();
    }
    return 0;
}

uint8_t Item::getContainerSize() const {
    if (auto type = getItemType()) {
        return type->getContainerSize();
    }
    return 0;
}

void Item::draw(int x, int y, float scale) {
    auto type = getItemType();
    if (!type) return;

    // g_graphics is declared in framework/graphics/graphics.h

    // Get sprite from type based on current animation phase and pattern
    int phase = m_animationPhase % type->getAnimationPhases();

    // Calculate pattern based on item properties
    int patX = m_patternX;
    int patY = m_patternY;
    int patZ = m_patternZ;

    // Stackable items use pattern based on count
    if (isStackable() && m_count > 1) {
        if (m_count < 5) {
            patX = m_count - 1;
        } else if (m_count < 10) {
            patX = 4;
        } else if (m_count < 25) {
            patX = 5;
        } else if (m_count < 50) {
            patX = 6;
        } else {
            patX = 7;
        }
    }

    // Fluid items use pattern based on subtype
    if (isFluid() || isSplash()) {
        patX = m_subType % 4;
        patY = m_subType / 4;
    }

    // Draw the sprite
    type->draw(x, y, scale, patX, patY, patZ, phase);
}

void Item::update(float deltaTime) {
    auto type = getItemType();
    if (!type) return;

    int phases = type->getAnimationPhases();
    if (phases <= 1) return;

    // Update animation
    m_animationTimer += deltaTime;

    // Standard animation at ~200ms per frame
    float frameDuration = 0.2f;
    while (m_animationTimer >= frameDuration) {
        m_animationTimer -= frameDuration;
        m_animationPhase = (m_animationPhase + 1) % phases;
    }
}

} // namespace client
} // namespace shadow
