/**
 * Shadow OT Client - Item
 *
 * Game item representation.
 */

#pragma once

#include "thing.h"
#include "thingtype.h"
#include <string>
#include <memory>

namespace shadow {
namespace client {

class Item : public Thing {
public:
    Item();
    static std::shared_ptr<Item> create(uint16_t id);

    bool isItem() const override { return true; }

    // Item ID
    uint16_t getId() const override { return m_id; }
    void setId(uint16_t id);

    // Count/subtype
    uint8_t getCount() const { return m_count; }
    void setCount(uint8_t count) { m_count = count; }

    uint8_t getSubType() const { return m_subType; }
    void setSubType(uint8_t subType) { m_subType = subType; }

    // Properties from item type
    ThingType* getItemType() const;
    bool isStackable() const;
    bool isGround() const;
    bool isContainer() const;
    bool isFluid() const;
    bool isSplash() const;
    bool isMoveable() const;
    bool isPickupable() const;
    bool isUseable() const;
    bool isWriteable() const;
    bool isReadable() const;
    bool blocksSolid() const;
    bool blocksProjectile() const;
    bool blocksPathfind() const;

    // Special properties
    uint16_t getSpeed() const;
    uint8_t getLightIntensity() const override;
    uint8_t getLightColor() const override;
    int getElevation() const;

    // Container specific
    uint8_t getContainerSize() const;

    // Text (for readable items)
    const std::string& getText() const { return m_text; }
    void setText(const std::string& text) { m_text = text; }

    const std::string& getDescription() const { return m_description; }
    void setDescription(const std::string& desc) { m_description = desc; }

    // Drawing
    void draw(int x, int y, float scale = 1.0f) override;

    // Animation
    void update(float deltaTime) override;
    void setAnimationPhase(int phase) { m_animationPhase = phase; }
    int getAnimationPhase() const { return m_animationPhase; }

    // Pattern for items with variants
    void setPatternX(int x) { m_patternX = x; }
    void setPatternY(int y) { m_patternY = y; }
    void setPatternZ(int z) { m_patternZ = z; }
    int getPatternX() const { return m_patternX; }
    int getPatternY() const { return m_patternY; }
    int getPatternZ() const { return m_patternZ; }

    // Action ID (for quests, etc.)
    uint16_t getActionId() const { return m_actionId; }
    void setActionId(uint16_t aid) { m_actionId = aid; }

    uint16_t getUniqueId() const { return m_uniqueId; }
    void setUniqueId(uint16_t uid) { m_uniqueId = uid; }

private:
    uint16_t m_id{0};
    uint8_t m_count{1};
    uint8_t m_subType{0};

    std::string m_text;
    std::string m_description;

    int m_animationPhase{0};
    float m_animationTimer{0};

    int m_patternX{0};
    int m_patternY{0};
    int m_patternZ{0};

    uint16_t m_actionId{0};
    uint16_t m_uniqueId{0};
};

using ItemPtr = std::shared_ptr<Item>;

} // namespace client
} // namespace shadow
