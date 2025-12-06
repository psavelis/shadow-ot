/**
 * Shadow OT Client - Creature Implementation
 */

#include "creature.h"
#include "thingtype.h"
#include "tile.h"
#include <framework/graphics/graphics.h>
#include <algorithm>
#include <cmath>

// g_graphics is declared in shadow::framework namespace
using shadow::framework::g_graphics;

namespace shadow {
namespace client {

Creature::Creature() = default;

std::shared_ptr<Creature> Creature::create(uint32_t id) {
    auto creature = std::make_shared<Creature>();
    creature->setCreatureId(id);
    return creature;
}

void Creature::turn(Position::Direction dir) {
    if (m_direction != dir) {
        m_direction = dir;
    }
}

void Creature::walk(const Position& newPos, bool preWalk) {
    // Calculate walk duration based on speed and ground speed
    // Base formula: stepDuration = groundSpeed * 1000 / speed

    m_walking = true;
    m_walkTarget = newPos;
    m_walkOffset = 0;
    m_walkTimer = 0;

    // Calculate direction
    m_direction = m_position.directionTo(newPos);

    // Estimate walk duration (will be adjusted by ground speed)
    uint16_t groundSpeed = 150; // Default
    if (auto tile = getTile()) {
        groundSpeed = tile->getGroundSpeed();
    }

    if (m_speed > 0) {
        m_walkDuration = static_cast<float>(groundSpeed) / m_speed;
    } else {
        m_walkDuration = 0.5f;
    }

    // Minimum duration
    m_walkDuration = std::max(m_walkDuration, 0.1f);
}

void Creature::cancelWalk() {
    m_walking = false;
    m_walkOffset = 0;
    m_walkTimer = 0;
}

void Creature::stopWalk() {
    if (m_walking) {
        m_position = m_walkTarget;
        m_walking = false;
        m_walkOffset = 0;
        m_walkTimer = 0;
    }
}

int Creature::getWalkOffsetX() const {
    if (!m_walking) return 0;

    float progress = m_walkOffset;
    int dx = static_cast<int>(m_walkTarget.x) - static_cast<int>(m_position.x);

    return static_cast<int>(-dx * 32 * (1.0f - progress));
}

int Creature::getWalkOffsetY() const {
    if (!m_walking) return 0;

    float progress = m_walkOffset;
    int dy = static_cast<int>(m_walkTarget.y) - static_cast<int>(m_position.y);

    return static_cast<int>(-dy * 32 * (1.0f - progress));
}

void Creature::say(const std::string& text, int type) {
    m_speechText = text;
    m_speechTimer = 5.0f; // Display for 5 seconds
}

void Creature::clearSpeech() {
    m_speechText.clear();
    m_speechTimer = 0;
}

void Creature::update(float deltaTime) {
    // Update walking
    if (m_walking) {
        m_walkTimer += deltaTime;

        if (m_walkTimer >= m_walkDuration) {
            // Walk complete
            m_position = m_walkTarget;
            m_walking = false;
            m_walkOffset = 0;
            m_walkTimer = 0;
        } else {
            m_walkOffset = m_walkTimer / m_walkDuration;
        }
    }

    // Update speech
    if (!m_speechText.empty()) {
        m_speechTimer -= deltaTime;
        if (m_speechTimer <= 0) {
            clearSpeech();
        }
    }

    // Update animation
    m_animationTimer += deltaTime;

    // Walking animation
    if (m_walking) {
        float frameTime = 0.1f; // 100ms per frame
        while (m_animationTimer >= frameTime) {
            m_animationTimer -= frameTime;
            m_animationPhase = (m_animationPhase + 1) % 4; // 4 walk phases
        }
    } else {
        // Idle animation (if any)
        m_animationPhase = 0;
    }
}

void Creature::draw(int x, int y, float scale) {
    // g_graphics is declared in framework/graphics/graphics.h

    // Apply walk offset
    x += getWalkOffsetX();
    y += getWalkOffsetY();

    // Get creature type for sprites
    auto& typeMgr = ThingTypeManager::instance();

    // Draw outfit
    if (m_outfit.lookType != 0) {
        // Creature outfit
        auto type = typeMgr.getCreatureType(m_outfit.lookType);
        if (type) {
            // Direction pattern
            int dirPattern = 0;
            switch (m_direction) {
                case Position::North: dirPattern = 0; break;
                case Position::East: dirPattern = 1; break;
                case Position::South: dirPattern = 2; break;
                case Position::West: dirPattern = 3; break;
                default: dirPattern = 2; break;
            }

            // Draw base outfit
            type->draw(x, y, scale, dirPattern, 0, 0, m_animationPhase);

            // Draw colored layers (head, body, legs, feet)
            // This would involve colorizing sprites based on outfit colors
            // Simplified: just draw base

            // Draw mount if any
            if (m_outfit.mount != 0) {
                auto mountType = typeMgr.getCreatureType(m_outfit.mount);
                if (mountType) {
                    mountType->draw(x, y, scale, dirPattern, 0, 0, m_animationPhase);
                }
            }
        }
    } else if (m_outfit.lookTypeEx != 0) {
        // Item look
        auto type = typeMgr.getItemType(m_outfit.lookTypeEx);
        if (type) {
            type->draw(x, y, scale, 0, 0, 0, m_animationPhase);
        }
    }

    // Draw health bar
    if (m_healthPercent < 100) {
        int barWidth = 27;
        int barHeight = 4;
        int barX = x - barWidth / 2;
        int barY = y - 12;

        // Background
        g_graphics.drawFilledRect({barX - 1, barY - 1, barWidth + 2, barHeight + 2},
                                   framework::Color{0, 0, 0, 180});

        // Health fill
        framework::Color healthColor;
        if (m_healthPercent > 60) {
            healthColor = {0, 188, 0, 255}; // Green
        } else if (m_healthPercent > 30) {
            healthColor = {255, 165, 0, 255}; // Orange
        } else {
            healthColor = {255, 0, 0, 255}; // Red
        }

        int fillWidth = barWidth * m_healthPercent / 100;
        g_graphics.drawFilledRect({barX, barY, fillWidth, barHeight}, healthColor);
    }

    // Draw skull/shield icons
    if (m_skull != Skull::None || m_shield != Shield::None) {
        // Icons would be drawn from sprite sheet
        int iconX = x + 12;
        int iconY = y - 14;
        // g_graphics.drawTexture(skullTexture, {iconX, iconY, 12, 12});
    }

    // Draw combat square
    if (m_hasSquare) {
        framework::Color squareColor{m_squareColor, m_squareColor, m_squareColor, 180};
        g_graphics.drawRect({x - 16, y - 16, 32, 32}, squareColor);
    }

    // Draw name
    if (!m_name.empty()) {
        framework::Size nameSize = g_graphics.measureText(m_name, 10);
        int nameX = x - nameSize.width / 2;
        int nameY = y - 20;

        // Name background
        g_graphics.drawFilledRect({nameX - 2, nameY - 1, nameSize.width + 4, nameSize.height + 2},
                                   framework::Color{0, 0, 0, 128});

        // Name text
        g_graphics.drawText(m_name, nameX, nameY, framework::Color{255, 255, 255, 255}, 10);
    }

    // Draw speech bubble
    if (!m_speechText.empty()) {
        framework::Size textSize = g_graphics.measureText(m_speechText, 10);
        int bubbleX = x - textSize.width / 2 - 4;
        int bubbleY = y - 40 - textSize.height;

        // Bubble background
        g_graphics.drawFilledRect({bubbleX, bubbleY, textSize.width + 8, textSize.height + 4},
                                   framework::Color{255, 255, 255, 220});
        g_graphics.drawRect({bubbleX, bubbleY, textSize.width + 8, textSize.height + 4},
                             framework::Color{0, 0, 0, 255});

        // Speech text
        g_graphics.drawText(m_speechText, bubbleX + 4, bubbleY + 2,
                            framework::Color{0, 0, 0, 255}, 10);
    }
}

} // namespace client
} // namespace shadow
