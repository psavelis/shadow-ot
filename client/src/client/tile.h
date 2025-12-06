/**
 * Shadow OT Client - Tile
 *
 * Single map tile containing ground, items, and creatures.
 */

#pragma once

#include "thing.h"
#include "item.h"
#include "creature.h"
#include <vector>
#include <memory>

namespace shadow {
namespace client {

class Tile : public std::enable_shared_from_this<Tile> {
public:
    Tile(const Position& pos);

    const Position& getPosition() const { return m_position; }

    // Ground item
    ItemPtr getGround() const { return m_ground; }
    void setGround(ItemPtr item);

    // Items
    void addItem(ItemPtr item);
    void removeItem(ItemPtr item);
    ItemPtr getItem(int stackPos) const;
    ItemPtr getTopItem() const;
    const std::vector<ItemPtr>& getItems() const { return m_items; }
    int getItemCount() const { return static_cast<int>(m_items.size()); }

    // Creatures
    void addCreature(CreaturePtr creature);
    void removeCreature(CreaturePtr creature);
    CreaturePtr getCreature(int stackPos) const;
    CreaturePtr getTopCreature() const;
    const std::vector<CreaturePtr>& getCreatures() const { return m_creatures; }
    int getCreatureCount() const { return static_cast<int>(m_creatures.size()); }

    // Effects
    void addEffect(ThingPtr effect);
    void removeEffect(ThingPtr effect);
    const std::vector<ThingPtr>& getEffects() const { return m_effects; }

    // Clear all things from tile
    void clear() {
        m_ground = nullptr;
        m_items.clear();
        m_creatures.clear();
        m_effects.clear();
        m_flags = 0;
    }

    // Get any thing by stack position
    ThingPtr getThing(int stackPos) const;
    int getThingCount() const;

    // Properties
    bool isWalkable() const;
    bool isPathable() const;
    bool isFullGround() const;
    bool hasCreature() const { return !m_creatures.empty(); }
    bool hasTopItem() const { return !m_items.empty(); }

    // Speed modifier for walking
    uint16_t getGroundSpeed() const;

    // Light
    uint8_t getLightIntensity() const;
    uint8_t getLightColor() const;

    // Elevation (for items that raise walkheight)
    int getElevation() const;

    // Drawing
    void draw(int x, int y, float scale = 1.0f);

    // Flags
    bool isBlocking() const { return m_flags & TileFlag_Blocking; }
    bool isBlockingProjectile() const { return m_flags & TileFlag_BlockProjectile; }
    bool isProtectionZone() const { return m_flags & TileFlag_ProtectionZone; }
    bool isNoPvPZone() const { return m_flags & TileFlag_NoPvP; }
    bool isNoLogoutZone() const { return m_flags & TileFlag_NoLogout; }
    bool isBankZone() const { return m_flags & TileFlag_BankZone; }
    bool isRefreshZone() const { return m_flags & TileFlag_RefreshZone; }

    void setFlag(uint32_t flag) { m_flags |= flag; }
    void clearFlag(uint32_t flag) { m_flags &= ~flag; }
    uint32_t getFlags() const { return m_flags; }

    static constexpr uint32_t TileFlag_Blocking = 1 << 0;
    static constexpr uint32_t TileFlag_BlockProjectile = 1 << 1;
    static constexpr uint32_t TileFlag_ProtectionZone = 1 << 2;
    static constexpr uint32_t TileFlag_NoPvP = 1 << 3;
    static constexpr uint32_t TileFlag_NoLogout = 1 << 4;
    static constexpr uint32_t TileFlag_BankZone = 1 << 5;
    static constexpr uint32_t TileFlag_RefreshZone = 1 << 6;

private:
    void updateFlags();

    Position m_position;
    ItemPtr m_ground;
    std::vector<ItemPtr> m_items;
    std::vector<CreaturePtr> m_creatures;
    std::vector<ThingPtr> m_effects;
    uint32_t m_flags{0};
};

using TilePtr = std::shared_ptr<Tile>;

} // namespace client
} // namespace shadow
