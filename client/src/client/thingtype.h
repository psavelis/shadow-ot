/**
 * Shadow OT Client - Thing Type
 *
 * Sprite and property definitions for items and creatures.
 */

#pragma once

#include <cstdint>
#include <memory>
#include <vector>
#include <map>
#include <string>
#include <framework/graphics/graphics.h>

namespace shadow {
namespace client {

// Item/creature type categories
enum class ThingCategory : uint8_t {
    Item = 0,
    Creature = 1,
    Effect = 2,
    Missile = 3
};

// Flags for item types
enum class ThingAttr : uint8_t {
    Ground = 0,
    TopOrder1 = 1,
    TopOrder2 = 2,
    TopOrder3 = 3,
    Container = 4,
    Stackable = 5,
    UseBounce = 6,
    UseTarget = 7,
    Useable = 8,
    Writable = 9,
    Readable = 10,
    FluidContainer = 11,
    Splash = 12,
    NotWalkable = 13,
    NotMoveable = 14,
    BlockProjectile = 15,
    NotPathable = 16,
    NoMoveAnimation = 17,
    Pickupable = 18,
    Hangable = 19,
    HookSouth = 20,
    HookEast = 21,
    Rotateable = 22,
    Light = 23,
    DontHide = 24,
    Translucent = 25,
    Displaced = 26,
    Elevation = 27,
    LyingCorpse = 28,
    AnimateAlways = 29,
    Minimap = 30,
    LensHelp = 31,
    FullGround = 32,
    Look = 33,
    Cloth = 34,
    Market = 35,
    DefaultAction = 36,
    Wrappable = 37,
    Unwrappable = 38,
    TopEffect = 39,

    // Animation
    Animation = 100
};

class ThingType {
public:
    ThingType();

    bool load(const uint8_t* data, size_t size);

    // Basic info
    uint16_t getId() const { return m_id; }
    void setId(uint16_t id) { m_id = id; }

    ThingCategory getCategory() const { return m_category; }
    void setCategory(ThingCategory cat) { m_category = cat; }

    // Dimensions
    int getWidth() const { return m_width; }
    int getHeight() const { return m_height; }
    int getExactSize() const { return m_exactSize; }
    int getLayers() const { return m_layers; }
    int getPatternX() const { return m_patternX; }
    int getPatternY() const { return m_patternY; }
    int getPatternZ() const { return m_patternZ; }
    int getAnimationPhases() const { return m_animPhases; }

    // Displacement
    int getDisplacementX() const { return m_displacementX; }
    int getDisplacementY() const { return m_displacementY; }

    // Flags
    bool hasAttr(ThingAttr attr) const;
    bool isGround() const { return hasAttr(ThingAttr::Ground); }
    bool isStackable() const { return hasAttr(ThingAttr::Stackable); }
    bool isContainer() const { return hasAttr(ThingAttr::Container); }
    bool isFluid() const { return hasAttr(ThingAttr::FluidContainer); }
    bool isSplash() const { return hasAttr(ThingAttr::Splash); }
    bool isMoveable() const { return !hasAttr(ThingAttr::NotMoveable); }
    bool isPickupable() const { return hasAttr(ThingAttr::Pickupable); }
    bool isUseable() const { return hasAttr(ThingAttr::Useable); }
    bool isWriteable() const { return hasAttr(ThingAttr::Writable); }
    bool isReadable() const { return hasAttr(ThingAttr::Readable); }
    bool blocksSolid() const { return hasAttr(ThingAttr::NotWalkable); }
    bool blocksProjectile() const { return hasAttr(ThingAttr::BlockProjectile); }
    bool blocksPathfind() const { return hasAttr(ThingAttr::NotPathable); }
    bool isFullGround() const { return hasAttr(ThingAttr::FullGround); }
    bool isAnimateAlways() const { return hasAttr(ThingAttr::AnimateAlways); }

    // Light
    uint8_t getLightIntensity() const { return m_lightIntensity; }
    uint8_t getLightColor() const { return m_lightColor; }

    // Speed (for ground)
    uint16_t getSpeed() const { return m_speed; }

    // Elevation
    int getElevation() const { return m_elevation; }

    // Container size
    uint8_t getContainerSize() const { return m_containerSize; }

    // Minimap color
    uint8_t getMinimapColor() const { return m_minimapColor; }

    // Drawing
    void draw(int x, int y, float scale = 1.0f,
              int patternX = 0, int patternY = 0, int patternZ = 0,
              int animationPhase = 0);

    // Sprites
    void setSprite(int index, std::shared_ptr<framework::Texture> sprite);
    std::shared_ptr<framework::Texture> getSprite(int index) const;

private:
    uint16_t m_id{0};
    ThingCategory m_category{ThingCategory::Item};

    // Dimensions
    int m_width{1};
    int m_height{1};
    int m_exactSize{32};
    int m_layers{1};
    int m_patternX{1};
    int m_patternY{1};
    int m_patternZ{1};
    int m_animPhases{1};

    // Displacement
    int m_displacementX{0};
    int m_displacementY{0};

    // Attributes
    std::map<ThingAttr, bool> m_attrs;

    // Properties
    uint16_t m_speed{0};
    uint8_t m_lightIntensity{0};
    uint8_t m_lightColor{0};
    int m_elevation{0};
    uint8_t m_containerSize{0};
    uint8_t m_minimapColor{0};

    // Sprites
    std::vector<std::shared_ptr<framework::Texture>> m_sprites;
};

// Alias for items
using ItemType = ThingType;
// Note: CreatureType enum is defined in creature.h for entity type distinction

// Global manager
class ThingTypeManager {
public:
    static ThingTypeManager& instance();

    bool loadDat(const std::string& filename);
    bool loadSpr(const std::string& filename);

    // Get types by ID
    ThingType* getItemType(uint16_t id);
    ThingType* getCreatureType(uint16_t id);
    ThingType* getEffectType(uint16_t id);
    ThingType* getMissileType(uint16_t id);

    // Sprite loading
    std::shared_ptr<framework::Texture> loadSprite(uint32_t spriteId);

    // Stats
    uint16_t getItemCount() const { return static_cast<uint16_t>(m_items.size()); }
    uint16_t getCreatureCount() const { return static_cast<uint16_t>(m_creatures.size()); }

private:
    ThingTypeManager() = default;

    std::vector<std::unique_ptr<ThingType>> m_items;
    std::vector<std::unique_ptr<ThingType>> m_creatures;
    std::vector<std::unique_ptr<ThingType>> m_effects;
    std::vector<std::unique_ptr<ThingType>> m_missiles;

    // Sprite file data
    std::vector<uint8_t> m_sprData;
    std::vector<uint32_t> m_spriteOffsets;
    std::map<uint32_t, std::shared_ptr<framework::Texture>> m_spriteCache;
};

} // namespace client
} // namespace shadow
