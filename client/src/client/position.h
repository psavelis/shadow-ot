/**
 * Shadow OT Client - Position
 *
 * 3D world position representation.
 */

#pragma once

#include <cstdint>
#include <string>
#include <cmath>
#include <vector>

namespace shadow {
namespace client {

struct Position {
    uint16_t x{0};
    uint16_t y{0};
    uint8_t z{0};

    Position() = default;
    Position(uint16_t x, uint16_t y, uint8_t z) : x(x), y(y), z(z) {}

    bool isValid() const { return x != 0xFFFF && y != 0xFFFF && z != 0xFF; }
    bool isMapPosition() const { return z < 16; }

    static Position invalid() { return Position(0xFFFF, 0xFFFF, 0xFF); }

    bool operator==(const Position& other) const {
        return x == other.x && y == other.y && z == other.z;
    }

    bool operator!=(const Position& other) const {
        return !(*this == other);
    }

    bool operator<(const Position& other) const {
        if (z != other.z) return z < other.z;
        if (y != other.y) return y < other.y;
        return x < other.x;
    }

    Position operator+(const Position& other) const {
        return Position(x + other.x, y + other.y, z + other.z);
    }

    Position operator-(const Position& other) const {
        return Position(x - other.x, y - other.y, z - other.z);
    }

    // Distance calculations
    double distanceTo(const Position& other) const {
        int dx = static_cast<int>(x) - static_cast<int>(other.x);
        int dy = static_cast<int>(y) - static_cast<int>(other.y);
        int dz = static_cast<int>(z) - static_cast<int>(other.z);
        return std::sqrt(dx * dx + dy * dy + dz * dz);
    }

    int manhattanDistance(const Position& other) const {
        return std::abs(static_cast<int>(x) - static_cast<int>(other.x)) +
               std::abs(static_cast<int>(y) - static_cast<int>(other.y)) +
               std::abs(static_cast<int>(z) - static_cast<int>(other.z));
    }

    // Direction
    enum Direction {
        North = 0,
        East = 1,
        South = 2,
        West = 3,
        NorthEast = 4,
        SouthEast = 5,
        SouthWest = 6,
        NorthWest = 7,
        InvalidDirection = 255
    };

    Direction directionTo(const Position& other) const {
        int dx = static_cast<int>(other.x) - static_cast<int>(x);
        int dy = static_cast<int>(other.y) - static_cast<int>(y);

        if (dx == 0 && dy < 0) return North;
        if (dx > 0 && dy == 0) return East;
        if (dx == 0 && dy > 0) return South;
        if (dx < 0 && dy == 0) return West;
        if (dx > 0 && dy < 0) return NorthEast;
        if (dx > 0 && dy > 0) return SouthEast;
        if (dx < 0 && dy > 0) return SouthWest;
        if (dx < 0 && dy < 0) return NorthWest;

        return InvalidDirection;
    }

    Position translated(Direction dir, int distance = 1) const {
        Position pos = *this;
        switch (dir) {
            case North: pos.y -= distance; break;
            case East: pos.x += distance; break;
            case South: pos.y += distance; break;
            case West: pos.x -= distance; break;
            case NorthEast: pos.x += distance; pos.y -= distance; break;
            case SouthEast: pos.x += distance; pos.y += distance; break;
            case SouthWest: pos.x -= distance; pos.y += distance; break;
            case NorthWest: pos.x -= distance; pos.y -= distance; break;
            default: break;
        }
        return pos;
    }

    // Pathfinding helpers
    std::vector<Position> getAdjacentPositions() const {
        return {
            translated(North), translated(East), translated(South), translated(West),
            translated(NorthEast), translated(SouthEast), translated(SouthWest), translated(NorthWest)
        };
    }

    std::string toString() const {
        return "(" + std::to_string(x) + ", " + std::to_string(y) + ", " + std::to_string(z) + ")";
    }
};

} // namespace client
} // namespace shadow
