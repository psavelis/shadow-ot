/**
 * Shadow OT Client - Map View Implementation
 */

#include "mapview.h"
#include "map.h"
#include "tile.h"
#include "creature.h"
#include "item.h"
#include "effect.h"
#include "missile.h"
#include "localplayer.h"
#include "game.h"
#include <algorithm>
#include <cmath>

namespace shadow {
namespace client {

// Light color palette (matching Tibia's light colors)
static const LightColor LIGHT_PALETTE[] = {
    {0, 0, 0},       // 0 - Black
    {255, 255, 255}, // 1 - White
    {255, 178, 0},   // 2 - Orange
    {180, 180, 180}, // 3 - Light gray
    {0, 255, 0},     // 4 - Green
    {255, 0, 0},     // 5 - Red
    {0, 0, 255},     // 6 - Blue
    {180, 180, 0},   // 7 - Yellow-ish
    {255, 50, 50},   // 8 - Light red
    {100, 100, 255}, // 9 - Light blue
    {255, 100, 100}, // 10
    {100, 255, 100}, // 11
    {100, 100, 255}, // 12
    {255, 255, 100}, // 13
    {255, 100, 255}, // 14
    {100, 255, 255}, // 15
    // ... more colors up to 215 (Tibia uses 215 as default ambient)
};

LightColor LightColor::fromIndex(uint8_t index) {
    if (index < sizeof(LIGHT_PALETTE) / sizeof(LIGHT_PALETTE[0])) {
        return LIGHT_PALETTE[index];
    }
    // Default to warm light for higher indices
    return {255, 220, 180};
}

MapView& MapView::instance() {
    static MapView instance;
    return instance;
}

void MapView::init() {
    // Initialize light texture (used for light blending)
    // In a full implementation, this would create an FBO for the light map
}

void MapView::terminate() {
    m_lightSources.clear();
    m_lightTexture = nullptr;
}

void MapView::setViewport(int width, int height) {
    m_viewportWidth = width;
    m_viewportHeight = height;

    // Calculate visible tiles based on viewport and scale
    m_visibleWidth = static_cast<int>(std::ceil(width / (TILE_SIZE * m_scale))) + 2;
    m_visibleHeight = static_cast<int>(std::ceil(height / (TILE_SIZE * m_scale))) + 2;
}

void MapView::setScale(float scale) {
    m_scale = std::clamp(scale, MIN_SCALE, MAX_SCALE);

    // Recalculate visible tiles
    if (m_viewportWidth > 0 && m_viewportHeight > 0) {
        m_visibleWidth = static_cast<int>(std::ceil(m_viewportWidth / (TILE_SIZE * m_scale))) + 2;
        m_visibleHeight = static_cast<int>(std::ceil(m_viewportHeight / (TILE_SIZE * m_scale))) + 2;
    }
}

void MapView::zoomIn() {
    setScale(m_scale * 1.25f);
}

void MapView::zoomOut() {
    setScale(m_scale / 1.25f);
}

void MapView::setCurrentFloor(int floor) {
    m_currentFloor = std::clamp(floor, 0, 15);
}

void MapView::setCameraOffset(float x, float y) {
    m_cameraOffsetX = x;
    m_cameraOffsetY = y;
}

void MapView::update(float deltaTime) {
    m_animationTime += deltaTime;

    // Update effects and missiles
    // TODO: g_effects.update(deltaTime);
    // TODO: g_missiles.update(deltaTime);
}

void MapView::render() {
    // Get center position from map
    const Position& centerPos = g_map.getCentralPosition();

    // Calculate visible area
    int halfWidth = m_visibleWidth / 2;
    int halfHeight = m_visibleHeight / 2;

    int startX = centerPos.x - halfWidth;
    int startY = centerPos.y - halfHeight;
    int endX = centerPos.x + halfWidth;
    int endY = centerPos.y + halfHeight;

    // Clear light sources from previous frame
    clearLightSources();

    // Collect light sources from visible area
    for (int z = m_currentFloor; z <= std::min(m_currentFloor + 2, 15); ++z) {
        for (int x = startX; x <= endX; ++x) {
            for (int y = startY; y <= endY; ++y) {
                auto tile = g_map.getTile(Position(x, y, z));
                if (!tile) continue;

                // Add ground light
                auto ground = tile->getGround();
                if (ground && ground->getLightIntensity() > 0) {
                    LightSource light;
                    light.pos = tile->getPosition();
                    light.intensity = ground->getLightIntensity();
                    light.color = ground->getLightColor();
                    light.radius = light.intensity / 2.0f;
                    addLightSource(light);
                }

                // Add creature lights
                for (size_t i = 0; i < tile->getCreatureCount(); ++i) {
                    auto creature = tile->getCreature(i);
                    if (creature && creature->getLightIntensity() > 0) {
                        LightSource light;
                        light.pos = creature->getPosition();
                        light.intensity = creature->getLightIntensity();
                        light.color = creature->getLightColor();
                        light.radius = light.intensity / 2.0f;
                        addLightSource(light);
                    }
                }

                // Add item lights
                for (size_t i = 0; i < tile->getItemCount(); ++i) {
                    auto item = tile->getItem(i);
                    if (item && item->getLightIntensity() > 0) {
                        LightSource light;
                        light.pos = tile->getPosition();
                        light.intensity = item->getLightIntensity();
                        light.color = item->getLightColor();
                        light.radius = light.intensity / 2.0f;
                        addLightSource(light);
                    }
                }
            }
        }
    }

    // Draw ground layer first
    drawGround(startX, startY, endX, endY);

    // Draw items and things on ground
    drawThings(startX, startY, endX, endY);

    // Draw creatures
    drawCreatures(startX, startY, endX, endY);

    // Draw top items (like roofs, etc)
    drawTopThings(startX, startY, endX, endY);

    // Draw effects and missiles
    drawEffectsAndMissiles();

    // Apply lighting
    drawLightMap();

    // Debug grid
    if (m_drawGrid) {
        drawDebugGrid();
    }
}

void MapView::drawGround(int startX, int startY, int endX, int endY) {
    const Position& centerPos = g_map.getCentralPosition();

    // Screen center
    int screenCenterX = m_viewportWidth / 2;
    int screenCenterY = m_viewportHeight / 2;

    // Draw multiple floors if underground
    int floorStart = m_currentFloor;
    int floorEnd = m_currentFloor;

    if (m_drawFloorFading && m_currentFloor <= 7) {
        floorEnd = std::min(m_currentFloor + 2, 7);
    }

    for (int z = floorEnd; z >= floorStart; --z) {
        float alpha = 1.0f;
        if (z != m_currentFloor) {
            alpha = m_floorFadeAlpha * (1.0f - (z - m_currentFloor) * 0.3f);
        }

        for (int y = startY; y <= endY; ++y) {
            for (int x = startX; x <= endX; ++x) {
                auto tile = g_map.getTile(Position(x, y, z));
                if (!tile) continue;

                auto ground = tile->getGround();
                if (!ground) continue;

                // Calculate screen position
                int screenX = screenCenterX + static_cast<int>((x - centerPos.x) * TILE_SIZE * m_scale - m_cameraOffsetX);
                int screenY = screenCenterY + static_cast<int>((y - centerPos.y) * TILE_SIZE * m_scale - m_cameraOffsetY);

                // Floor offset for depth effect
                if (z != m_currentFloor) {
                    int offset = (z - m_currentFloor) * static_cast<int>(TILE_SIZE * m_scale);
                    screenX -= offset;
                    screenY -= offset;
                }

                renderItem(ground, screenX, screenY, m_scale);
            }
        }
    }
}

void MapView::drawThings(int startX, int startY, int endX, int endY) {
    const Position& centerPos = g_map.getCentralPosition();
    int screenCenterX = m_viewportWidth / 2;
    int screenCenterY = m_viewportHeight / 2;

    for (int y = startY; y <= endY; ++y) {
        for (int x = startX; x <= endX; ++x) {
            auto tile = g_map.getTile(Position(x, y, m_currentFloor));
            if (!tile) continue;

            int screenX = screenCenterX + static_cast<int>((x - centerPos.x) * TILE_SIZE * m_scale - m_cameraOffsetX);
            int screenY = screenCenterY + static_cast<int>((y - centerPos.y) * TILE_SIZE * m_scale - m_cameraOffsetY);

            // Draw items (excluding ground and top items)
            for (size_t i = 0; i < tile->getItemCount(); ++i) {
                auto item = tile->getItem(i);
                if (!item) continue;

                auto* type = item->getThingType();
                if (!type) continue;

                // Skip ground and top items
                if (type->isGround() || type->hasAttr(ThingAttr::TopOrder1) ||
                    type->hasAttr(ThingAttr::TopOrder2) || type->hasAttr(ThingAttr::TopOrder3)) {
                    continue;
                }

                renderItem(item, screenX, screenY, m_scale);
            }
        }
    }
}

void MapView::drawTopThings(int startX, int startY, int endX, int endY) {
    const Position& centerPos = g_map.getCentralPosition();
    int screenCenterX = m_viewportWidth / 2;
    int screenCenterY = m_viewportHeight / 2;

    for (int y = startY; y <= endY; ++y) {
        for (int x = startX; x <= endX; ++x) {
            auto tile = g_map.getTile(Position(x, y, m_currentFloor));
            if (!tile) continue;

            int screenX = screenCenterX + static_cast<int>((x - centerPos.x) * TILE_SIZE * m_scale - m_cameraOffsetX);
            int screenY = screenCenterY + static_cast<int>((y - centerPos.y) * TILE_SIZE * m_scale - m_cameraOffsetY);

            // Draw top items
            for (size_t i = 0; i < tile->getItemCount(); ++i) {
                auto item = tile->getItem(i);
                if (!item) continue;

                auto* type = item->getThingType();
                if (!type) continue;

                // Only draw top items
                if (type->hasAttr(ThingAttr::TopOrder1) ||
                    type->hasAttr(ThingAttr::TopOrder2) ||
                    type->hasAttr(ThingAttr::TopOrder3)) {
                    renderItem(item, screenX, screenY, m_scale);
                }
            }
        }
    }
}

void MapView::drawCreatures(int startX, int startY, int endX, int endY) {
    const Position& centerPos = g_map.getCentralPosition();
    int screenCenterX = m_viewportWidth / 2;
    int screenCenterY = m_viewportHeight / 2;

    // Collect creatures and sort by y position for proper overlap
    std::vector<std::pair<int, std::shared_ptr<Creature>>> creatures;

    for (int y = startY; y <= endY; ++y) {
        for (int x = startX; x <= endX; ++x) {
            auto tile = g_map.getTile(Position(x, y, m_currentFloor));
            if (!tile) continue;

            for (size_t i = 0; i < tile->getCreatureCount(); ++i) {
                auto creature = tile->getCreature(i);
                if (creature) {
                    // Sort key: y * 1000 + x to ensure proper drawing order
                    int sortKey = y * 10000 + x;
                    creatures.emplace_back(sortKey, creature);
                }
            }
        }
    }

    // Sort by y position
    std::sort(creatures.begin(), creatures.end(),
        [](const auto& a, const auto& b) { return a.first < b.first; });

    // Draw creatures
    for (const auto& [sortKey, creature] : creatures) {
        const Position& pos = creature->getPosition();

        // Calculate base screen position
        int screenX = screenCenterX + static_cast<int>((pos.x - centerPos.x) * TILE_SIZE * m_scale - m_cameraOffsetX);
        int screenY = screenCenterY + static_cast<int>((pos.y - centerPos.y) * TILE_SIZE * m_scale - m_cameraOffsetY);

        // Add walk offset for smooth movement
        if (creature->isWalking()) {
            float walkOffset = creature->getWalkOffset();
            // Apply walk offset based on direction
            Position::Direction dir = creature->getDirection();
            int offsetX = 0, offsetY = 0;
            switch (dir) {
                case Position::North: offsetY = static_cast<int>(-walkOffset * TILE_SIZE); break;
                case Position::South: offsetY = static_cast<int>(walkOffset * TILE_SIZE); break;
                case Position::West: offsetX = static_cast<int>(-walkOffset * TILE_SIZE); break;
                case Position::East: offsetX = static_cast<int>(walkOffset * TILE_SIZE); break;
                default: break;
            }
            screenX += static_cast<int>(offsetX * m_scale);
            screenY += static_cast<int>(offsetY * m_scale);
        }

        renderCreature(creature, screenX, screenY, m_scale);
    }
}

void MapView::drawEffectsAndMissiles() {
    const Position& centerPos = g_map.getCentralPosition();
    int screenCenterX = m_viewportWidth / 2;
    int screenCenterY = m_viewportHeight / 2;

    // Draw effects
    int halfWidth = m_visibleWidth / 2;
    int halfHeight = m_visibleHeight / 2;

    for (int y = centerPos.y - halfHeight; y <= centerPos.y + halfHeight; ++y) {
        for (int x = centerPos.x - halfWidth; x <= centerPos.x + halfWidth; ++x) {
            Position pos(x, y, m_currentFloor);

            int screenX = screenCenterX + static_cast<int>((x - centerPos.x) * TILE_SIZE * m_scale - m_cameraOffsetX);
            int screenY = screenCenterY + static_cast<int>((y - centerPos.y) * TILE_SIZE * m_scale - m_cameraOffsetY);

            // TODO: g_effects.drawEffects(pos, screenX, screenY, m_scale);
            (void)pos; (void)screenX; (void)screenY;
        }
    }

    // Draw missiles
    // TODO: Implement missile drawing
    // int viewX = static_cast<int>((centerPos.x - halfWidth) * TILE_SIZE + m_cameraOffsetX);
    // int viewY = static_cast<int>((centerPos.y - halfHeight) * TILE_SIZE + m_cameraOffsetY);
    // g_missiles.draw(viewX, viewY, m_scale);
}

void MapView::drawLightMap() {
    // Apply ambient light and point light sources
    // In a full implementation, this would:
    // 1. Create a light map FBO
    // 2. For each pixel, calculate the combined light from ambient + point sources
    // 3. Blend the light map with the scene using multiplicative blending

    // For now, we'll just set a basic ambient light level
    // The actual blending would be done in the shader

    // Example pseudocode for shader-based lighting:
    // for each fragment:
    //   finalColor = sceneColor * ambientLight
    //   for each light source:
    //     dist = distance(fragment, lightPos)
    //     attenuation = 1.0 - (dist / lightRadius)
    //     if attenuation > 0:
    //       finalColor += sceneColor * lightColor * lightIntensity * attenuation
}

void MapView::drawDebugGrid() {
    const Position& centerPos = g_map.getCentralPosition();
    int screenCenterX = m_viewportWidth / 2;
    int screenCenterY = m_viewportHeight / 2;

    int halfWidth = m_visibleWidth / 2;
    int halfHeight = m_visibleHeight / 2;

    // Draw grid lines
    // In a full implementation, this would use the graphics API to draw lines
}

void MapView::renderTile(const std::shared_ptr<Tile>& tile, int x, int y, float scale) {
    if (!tile) return;

    // Draw ground
    auto ground = tile->getGround();
    if (ground) {
        renderItem(ground, x, y, scale);
    }

    // Draw items
    for (size_t i = 0; i < tile->getItemCount(); ++i) {
        auto item = tile->getItem(i);
        if (item) {
            renderItem(item, x, y, scale);
        }
    }

    // Draw creatures
    for (size_t i = 0; i < tile->getCreatureCount(); ++i) {
        auto creature = tile->getCreature(i);
        if (creature) {
            renderCreature(creature, x, y, scale);
        }
    }
}

void MapView::renderCreature(const std::shared_ptr<Creature>& creature, int x, int y, float scale) {
    if (!creature) return;

    // Draw creature sprite
    creature->draw(x, y, scale);

    // Draw health bar if not at full health
    if (creature->getHealthPercent() < 100) {
        // Health bar dimensions
        int barWidth = static_cast<int>(TILE_SIZE * scale);
        int barHeight = 4;
        int barY = y - barHeight - 2;

        // Background (black)
        // drawRect(x, barY, barWidth, barHeight, Color::Black);

        // Health portion (color based on percent)
        int healthWidth = static_cast<int>(barWidth * creature->getHealthPercent() / 100.0f);
        // Color based on health: green > yellow > red
        // drawRect(x, barY, healthWidth, barHeight, healthColor);
    }

    // Draw creature name if it's a player or has a name visible
    if (creature->isPlayer() || creature->isNPC()) {
        // Draw name above creature
        // const std::string& name = creature->getName();
        // drawText(name, x + TILE_SIZE/2, y - 16, TextAlign::Center);
    }

    // Draw speech bubble if creature is speaking
    // TODO: Implement speech bubble when isSpeaking() is added to Creature
    // if (creature->isSpeaking()) {
    //     const std::string& speech = creature->getSpeechText();
    //     drawSpeechBubble(speech, x, y);
    // }
}

void MapView::renderItem(const std::shared_ptr<Item>& item, int x, int y, float scale) {
    if (!item) return;

    // Get animation frame if animated
    int animPhase = 0;
    auto* type = item->getThingType();
    if (type && type->isAnimateAlways()) {
        float frameDuration = 0.5f;  // seconds per frame
        int phases = type->getAnimationPhases();
        if (phases > 1) {
            animPhase = static_cast<int>(m_animationTime / frameDuration) % phases;
        }
    }

    item->draw(x, y, scale);
}

void MapView::addLightSource(const LightSource& light) {
    m_lightSources.push_back(light);
}

void MapView::clearLightSources() {
    m_lightSources.clear();
}

void MapView::setAmbientLight(uint8_t intensity, uint8_t color) {
    m_ambientIntensity = intensity;
    m_ambientColor = color;
}

Position MapView::getPositionFromScreen(int screenX, int screenY) const {
    const Position& centerPos = g_map.getCentralPosition();
    int screenCenterX = m_viewportWidth / 2;
    int screenCenterY = m_viewportHeight / 2;

    // Convert screen coordinates to tile coordinates
    int tileX = centerPos.x + static_cast<int>((screenX - screenCenterX + m_cameraOffsetX) / (TILE_SIZE * m_scale));
    int tileY = centerPos.y + static_cast<int>((screenY - screenCenterY + m_cameraOffsetY) / (TILE_SIZE * m_scale));

    return Position(tileX, tileY, m_currentFloor);
}

void MapView::getScreenFromPosition(const Position& pos, int& screenX, int& screenY) const {
    const Position& centerPos = g_map.getCentralPosition();
    int screenCenterX = m_viewportWidth / 2;
    int screenCenterY = m_viewportHeight / 2;

    screenX = screenCenterX + static_cast<int>((pos.x - centerPos.x) * TILE_SIZE * m_scale - m_cameraOffsetX);
    screenY = screenCenterY + static_cast<int>((pos.y - centerPos.y) * TILE_SIZE * m_scale - m_cameraOffsetY);
}

LightColor MapView::calculateLightAt(int x, int y, int z) const {
    // Start with ambient light
    LightColor result = LightColor::fromIndex(m_ambientColor);
    float ambientFactor = m_ambientIntensity / 255.0f;
    result.r = static_cast<uint8_t>(result.r * ambientFactor);
    result.g = static_cast<uint8_t>(result.g * ambientFactor);
    result.b = static_cast<uint8_t>(result.b * ambientFactor);

    // Add contribution from each light source
    for (const auto& light : m_lightSources) {
        if (light.pos.z != z) continue;

        float dx = static_cast<float>(x - light.pos.x);
        float dy = static_cast<float>(y - light.pos.y);
        float dist = std::sqrt(dx * dx + dy * dy);

        if (dist <= light.radius) {
            float attenuation = 1.0f - (dist / light.radius);
            float intensity = (light.intensity / 255.0f) * attenuation;

            LightColor lightColor = LightColor::fromIndex(light.color);
            result = blendLight(result, {
                static_cast<uint8_t>(lightColor.r * intensity),
                static_cast<uint8_t>(lightColor.g * intensity),
                static_cast<uint8_t>(lightColor.b * intensity)
            });
        }
    }

    return result;
}

LightColor MapView::blendLight(const LightColor& a, const LightColor& b) const {
    // Additive blending, clamped to 255
    return {
        static_cast<uint8_t>(std::min(255, a.r + b.r)),
        static_cast<uint8_t>(std::min(255, a.g + b.g)),
        static_cast<uint8_t>(std::min(255, a.b + b.b))
    };
}

} // namespace client
} // namespace shadow

// Global accessor
shadow::client::MapView& g_mapView = shadow::client::MapView::instance();
