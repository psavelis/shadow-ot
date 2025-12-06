/**
 * Shadow OT Client - Thing
 *
 * Base class for all game objects (items, creatures, effects).
 */

#pragma once

#include <cstdint>
#include <memory>
#include "position.h"

namespace shadow {
namespace client {

class Tile;
class ThingType;

class Thing : public std::enable_shared_from_this<Thing> {
public:
    Thing();
    virtual ~Thing() = default;

    // Position
    const Position& getPosition() const { return m_position; }
    void setPosition(const Position& pos) { m_position = pos; }

    // Parent tile
    std::shared_ptr<Tile> getTile() const { return m_tile.lock(); }
    void setTile(std::shared_ptr<Tile> tile) { m_tile = tile; }

    // Stack position within tile
    int getStackPos() const { return m_stackPos; }
    void setStackPos(int pos) { m_stackPos = pos; }

    // Type identification
    virtual bool isItem() const { return false; }
    virtual bool isCreature() const { return false; }
    virtual bool isEffect() const { return false; }
    virtual bool isMissile() const { return false; }
    virtual bool isPlayer() const { return false; }
    virtual bool isLocalPlayer() const { return false; }
    virtual bool isNPC() const { return false; }
    virtual bool isMonster() const { return false; }

    // Visual
    virtual uint16_t getId() const { return 0; }
    virtual ThingType* getThingType() const { return nullptr; }

    // Drawing
    virtual void draw(int x, int y, float scale = 1.0f) {}

    // Animation
    virtual void update(float deltaTime) {}

    // Light
    virtual uint8_t getLightIntensity() const { return 0; }
    virtual uint8_t getLightColor() const { return 0; }

protected:
    Position m_position;
    std::weak_ptr<Tile> m_tile;
    int m_stackPos{0};
};

using ThingPtr = std::shared_ptr<Thing>;
using ThingWeakPtr = std::weak_ptr<Thing>;

} // namespace client
} // namespace shadow
