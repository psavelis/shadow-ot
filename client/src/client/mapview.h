/**
 * Shadow OT Client - Map View
 *
 * Map rendering with lighting, floor fading, and viewport management.
 */

#pragma once

#include "position.h"
#include <framework/graphics/graphics.h>
#include <memory>
#include <array>
#include <vector>

namespace shadow {
namespace client {

class Tile;
class Creature;
class Item;

// Light source for dynamic lighting
struct LightSource {
    Position pos;
    uint8_t intensity{0};
    uint8_t color{0};
    float radius{0.0f};
};

// Light color palette (Tibia style)
struct LightColor {
    uint8_t r, g, b;

    static LightColor fromIndex(uint8_t index);
};

class MapView {
public:
    static MapView& instance();

    void init();
    void terminate();

    // Rendering
    void render();
    void renderTile(const std::shared_ptr<Tile>& tile, int x, int y, float scale);
    void renderCreature(const std::shared_ptr<Creature>& creature, int x, int y, float scale);
    void renderItem(const std::shared_ptr<Item>& item, int x, int y, float scale);

    // Lighting
    void renderLighting();
    void addLightSource(const LightSource& light);
    void clearLightSources();
    void setAmbientLight(uint8_t intensity, uint8_t color);

    // Viewport
    void setViewport(int width, int height);
    int getViewportWidth() const { return m_viewportWidth; }
    int getViewportHeight() const { return m_viewportHeight; }

    // Visible tile dimensions
    int getVisibleTileWidth() const { return m_visibleWidth; }
    int getVisibleTileHeight() const { return m_visibleHeight; }

    // Scale/zoom
    void setScale(float scale);
    float getScale() const { return m_scale; }
    void zoomIn();
    void zoomOut();

    // Floor handling
    void setCurrentFloor(int floor);
    int getCurrentFloor() const { return m_currentFloor; }
    void setDrawFloorFading(bool enabled) { m_drawFloorFading = enabled; }
    bool isDrawingFloorFading() const { return m_drawFloorFading; }

    // Camera offset (for smooth movement)
    void setCameraOffset(float x, float y);
    float getCameraOffsetX() const { return m_cameraOffsetX; }
    float getCameraOffsetY() const { return m_cameraOffsetY; }

    // Mouse position to map position conversion
    Position getPositionFromScreen(int screenX, int screenY) const;
    void getScreenFromPosition(const Position& pos, int& screenX, int& screenY) const;

    // Animation
    void setAnimateAlways(bool animate) { m_animateAlways = animate; }
    bool isAnimatingAlways() const { return m_animateAlways; }

    // Debug
    void setDrawGrid(bool draw) { m_drawGrid = draw; }
    bool isDrawingGrid() const { return m_drawGrid; }

    // Update (call each frame)
    void update(float deltaTime);

private:
    MapView() = default;

    void drawGround(int startX, int startY, int endX, int endY);
    void drawThings(int startX, int startY, int endX, int endY);
    void drawTopThings(int startX, int startY, int endX, int endY);
    void drawCreatures(int startX, int startY, int endX, int endY);
    void drawEffectsAndMissiles();
    void drawLightMap();
    void drawDebugGrid();

    // Calculate light at a specific position
    LightColor calculateLightAt(int x, int y, int z) const;

    // Blend two light colors
    LightColor blendLight(const LightColor& a, const LightColor& b) const;

    // Viewport dimensions
    int m_viewportWidth{0};
    int m_viewportHeight{0};

    // Visible tile counts
    int m_visibleWidth{15};
    int m_visibleHeight{11};

    // Rendering scale
    float m_scale{1.0f};
    static constexpr float MIN_SCALE = 0.5f;
    static constexpr float MAX_SCALE = 4.0f;

    // Tile size in pixels
    static constexpr int TILE_SIZE = 32;

    // Current floor being viewed
    int m_currentFloor{7};

    // Floor fading for multi-floor rendering
    bool m_drawFloorFading{true};
    float m_floorFadeAlpha{0.5f};

    // Camera offset for smooth scrolling
    float m_cameraOffsetX{0.0f};
    float m_cameraOffsetY{0.0f};

    // Lighting
    std::vector<LightSource> m_lightSources;
    uint8_t m_ambientIntensity{200};
    uint8_t m_ambientColor{215};
    std::shared_ptr<framework::Texture> m_lightTexture;

    // Animation
    bool m_animateAlways{true};
    float m_animationTime{0.0f};

    // Debug rendering
    bool m_drawGrid{false};
};

} // namespace client
} // namespace shadow

// Global accessor
extern shadow::client::MapView& g_mapView;
