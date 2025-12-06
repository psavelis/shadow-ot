/**
 * Shadow OT Client - Map Implementation
 */

#include "map.h"
#include "creature.h"
#include <algorithm>
#include <queue>
#include <unordered_set>
#include <cmath>

namespace shadow {
namespace client {

Map& Map::instance() {
    static Map instance;
    return instance;
}

void Map::init() {
    clear();
}

void Map::terminate() {
    clear();
}

void Map::clear() {
    m_tiles.clear();
    m_creatures.clear();
    m_minimapTiles.clear();
    m_centralPosition = Position();
}

TilePtr Map::getTile(const Position& pos) {
    auto it = m_tiles.find(pos);
    if (it != m_tiles.end()) {
        return it->second;
    }
    return nullptr;
}

TilePtr Map::getOrCreateTile(const Position& pos) {
    auto tile = getTile(pos);
    if (!tile) {
        tile = std::make_shared<Tile>(pos);
        m_tiles[pos] = tile;
    }
    return tile;
}

void Map::addTile(TilePtr tile) {
    if (!tile) return;
    m_tiles[tile->getPosition()] = tile;
}

void Map::removeTile(const Position& pos) {
    m_tiles.erase(pos);
}

void Map::addCreature(std::shared_ptr<Creature> creature) {
    if (!creature) return;
    m_creatures[creature->getCreatureId()] = creature;
}

void Map::removeCreature(uint32_t creatureId) {
    m_creatures.erase(creatureId);
}

std::shared_ptr<Creature> Map::getCreatureById(uint32_t id) {
    auto it = m_creatures.find(id);
    if (it != m_creatures.end()) {
        return it->second.lock();
    }
    return nullptr;
}

std::vector<std::shared_ptr<Creature>> Map::getCreaturesInRange(const Position& pos, int range) {
    std::vector<std::shared_ptr<Creature>> result;

    for (auto& [id, weakCreature] : m_creatures) {
        auto creature = weakCreature.lock();
        if (creature) {
            int dx = std::abs(static_cast<int>(creature->getPosition().x) - static_cast<int>(pos.x));
            int dy = std::abs(static_cast<int>(creature->getPosition().y) - static_cast<int>(pos.y));

            if (dx <= range && dy <= range && creature->getPosition().z == pos.z) {
                result.push_back(creature);
            }
        }
    }

    return result;
}

void Map::setCentralPosition(const Position& pos) {
    Position oldPos = m_centralPosition;
    m_centralPosition = pos;

    if (m_onPositionChange && oldPos != pos) {
        m_onPositionChange(oldPos, pos);
    }
}

Map::MinimapTile Map::getMinimapTile(const Position& pos) const {
    auto it = m_minimapTiles.find(pos);
    if (it != m_minimapTiles.end()) {
        return it->second;
    }
    return MinimapTile{};
}

void Map::setMinimapTile(const Position& pos, const MinimapTile& tile) {
    m_minimapTiles[pos] = tile;
}

void Map::setAmbientLight(uint8_t intensity, uint8_t color) {
    m_ambientLight.intensity = intensity;
    m_ambientLight.color = color;
}

std::vector<Position::Direction> Map::findPath(const Position& start, const Position& end, int maxDistance) {
    std::vector<Position::Direction> path;

    if (start == end) return path;
    if (start.z != end.z) return path;

    // A* pathfinding
    struct Node {
        Position pos;
        float g, h, f;
        Position::Direction fromDir;
        Position parent;
    };

    auto heuristic = [](const Position& a, const Position& b) {
        return static_cast<float>(std::abs(static_cast<int>(a.x) - static_cast<int>(b.x)) +
                                  std::abs(static_cast<int>(a.y) - static_cast<int>(b.y)));
    };

    std::map<Position, Node> openSet;
    std::map<Position, Node> closedSet;

    Node startNode;
    startNode.pos = start;
    startNode.g = 0;
    startNode.h = heuristic(start, end);
    startNode.f = startNode.h;
    startNode.fromDir = Position::InvalidDirection;
    startNode.parent = Position::invalid();

    openSet[start] = startNode;

    int iterations = 0;
    int maxIterations = maxDistance * maxDistance * 4;

    while (!openSet.empty() && iterations < maxIterations) {
        iterations++;

        // Find node with lowest f score
        auto bestIt = openSet.begin();
        for (auto it = openSet.begin(); it != openSet.end(); ++it) {
            if (it->second.f < bestIt->second.f) {
                bestIt = it;
            }
        }

        Node current = bestIt->second;
        openSet.erase(bestIt);
        closedSet[current.pos] = current;

        // Check if reached goal
        if (current.pos == end) {
            // Reconstruct path
            std::vector<Position::Direction> reversePath;
            Position pos = end;

            while (pos != start && pos.isValid()) {
                auto it = closedSet.find(pos);
                if (it == closedSet.end()) break;

                if (it->second.fromDir != Position::InvalidDirection) {
                    reversePath.push_back(it->second.fromDir);
                }
                pos = it->second.parent;
            }

            // Reverse to get path from start to end
            path.reserve(reversePath.size());
            for (auto it = reversePath.rbegin(); it != reversePath.rend(); ++it) {
                path.push_back(*it);
            }

            return path;
        }

        // Check neighbors
        Position::Direction directions[] = {
            Position::North, Position::East, Position::South, Position::West,
            Position::NorthEast, Position::SouthEast, Position::SouthWest, Position::NorthWest
        };

        for (auto dir : directions) {
            Position neighbor = current.pos.translated(dir);

            // Skip if out of range
            if (std::abs(static_cast<int>(neighbor.x) - static_cast<int>(start.x)) > maxDistance ||
                std::abs(static_cast<int>(neighbor.y) - static_cast<int>(start.y)) > maxDistance) {
                continue;
            }

            // Skip if already in closed set
            if (closedSet.find(neighbor) != closedSet.end()) {
                continue;
            }

            // Check if walkable
            TilePtr tile = getTile(neighbor);
            if (!tile || !tile->isPathable()) {
                // Allow destination even if blocked
                if (neighbor != end) continue;
            }

            // Calculate cost
            float moveCost = (dir >= Position::NorthEast) ? 1.414f : 1.0f; // Diagonal is more expensive
            float g = current.g + moveCost;

            // Check if neighbor is in open set with better cost
            auto it = openSet.find(neighbor);
            if (it != openSet.end() && it->second.g <= g) {
                continue;
            }

            // Add/update neighbor
            Node neighborNode;
            neighborNode.pos = neighbor;
            neighborNode.g = g;
            neighborNode.h = heuristic(neighbor, end);
            neighborNode.f = g + neighborNode.h;
            neighborNode.fromDir = dir;
            neighborNode.parent = current.pos;

            openSet[neighbor] = neighborNode;
        }
    }

    return path; // Empty if no path found
}

int Map::getFirstAwareFloor() const {
    if (m_centralPosition.z > MAP_SEA_LEVEL) {
        return std::max(0, static_cast<int>(m_centralPosition.z) - MAP_UNDERGROUND_FLOOR_RANGE);
    }
    return 0;
}

int Map::getLastAwareFloor() const {
    if (m_centralPosition.z > MAP_SEA_LEVEL) {
        return std::min(MAP_MAX_Z, static_cast<int>(m_centralPosition.z) + MAP_UNDERGROUND_FLOOR_RANGE);
    }
    return MAP_SEA_LEVEL;
}

bool Map::isFloorVisible(int z) const {
    return z >= getFirstAwareFloor() && z <= getLastAwareFloor();
}

Map::DrawInfo Map::getDrawInfo() const {
    DrawInfo info;

    info.startX = m_centralPosition.x - MAP_AWARE_RANGE_X;
    info.startY = m_centralPosition.y - MAP_AWARE_RANGE_Y;
    info.endX = m_centralPosition.x + MAP_AWARE_RANGE_X;
    info.endY = m_centralPosition.y + MAP_AWARE_RANGE_Y;

    // Calculate pixel offset for centered drawing
    info.offsetX = MAP_AWARE_RANGE_X * 32;
    info.offsetY = MAP_AWARE_RANGE_Y * 32;

    return info;
}

} // namespace client
} // namespace shadow

// Global accessor
shadow::client::Map& g_map = shadow::client::Map::instance();
