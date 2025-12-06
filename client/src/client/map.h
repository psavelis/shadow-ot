/**
 * Shadow OT Client - Map
 *
 * Game world map with tile management.
 */

#pragma once

#include "tile.h"
#include "position.h"
#include <map>
#include <memory>
#include <vector>
#include <functional>

namespace shadow {
namespace client {

class Creature;
class LocalPlayer;

// Visible area constants
constexpr int MAP_MAX_Z = 15;
constexpr int MAP_SEA_LEVEL = 7;
constexpr int MAP_UNDERGROUND_FLOOR_RANGE = 2;
constexpr int MAP_AWARE_RANGE_X = 8;
constexpr int MAP_AWARE_RANGE_Y = 6;

class Map {
public:
    static Map& instance();

    void init();
    void terminate();
    void clear();

    // Tile access
    TilePtr getTile(const Position& pos);
    TilePtr getOrCreateTile(const Position& pos);
    void addTile(TilePtr tile);
    void removeTile(const Position& pos);
    void cleanTile(const Position& pos);  // Clear all things from tile

    // Creature tracking
    void addCreature(std::shared_ptr<Creature> creature);
    void removeCreature(uint32_t creatureId);
    std::shared_ptr<Creature> getCreatureById(uint32_t id);
    std::vector<std::shared_ptr<Creature>> getCreaturesInRange(const Position& pos, int range);

    // Central position (where local player is)
    const Position& getCentralPosition() const { return m_centralPosition; }
    void setCentralPosition(const Position& pos);

    // Minimap colors
    struct MinimapTile {
        uint8_t color{0};
        uint8_t flags{0};
        uint16_t speed{100};
    };
    MinimapTile getMinimapTile(const Position& pos) const;
    void setMinimapTile(const Position& pos, const MinimapTile& tile);

    // Light
    struct LightInfo {
        uint8_t intensity{0};
        uint8_t color{215};  // Default torch color
    };
    LightInfo getLight() const { return m_ambientLight; }
    void setAmbientLight(uint8_t intensity, uint8_t color);
    void setWorldLight(uint8_t intensity, uint8_t color) { setAmbientLight(intensity, color); }

    // Path finding
    std::vector<Position::Direction> findPath(const Position& start, const Position& end,
                                               int maxDistance = 100);

    // Floor visibility
    int getFirstAwareFloor() const;
    int getLastAwareFloor() const;
    bool isFloorVisible(int z) const;

    // Drawing area
    struct DrawInfo {
        int startX, startY;
        int endX, endY;
        int offsetX, offsetY;
    };
    DrawInfo getDrawInfo() const;

    // Callbacks
    using PositionChangeCallback = std::function<void(const Position&, const Position&)>;
    void setOnPositionChange(PositionChangeCallback cb) { m_onPositionChange = cb; }

    // Known tiles count
    size_t getTileCount() const { return m_tiles.size(); }

private:
    Map() = default;
    Map(const Map&) = delete;
    Map& operator=(const Map&) = delete;

    // A* pathfinding helper
    struct PathNode {
        Position pos;
        float g, h, f;
        PathNode* parent;
    };

    // Tiles indexed by position
    std::map<Position, TilePtr> m_tiles;

    // Creatures indexed by ID
    std::map<uint32_t, std::weak_ptr<Creature>> m_creatures;

    // Minimap data (simple color/flag storage)
    std::map<Position, MinimapTile> m_minimapTiles;

    // Central position
    Position m_centralPosition;

    // Ambient light
    LightInfo m_ambientLight;

    PositionChangeCallback m_onPositionChange;
};

} // namespace client
} // namespace shadow

// Global accessor
extern shadow::client::Map& g_map;
