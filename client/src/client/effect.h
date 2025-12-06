/**
 * Shadow OT Client - Effect
 *
 * Visual effects displayed on map tiles (magic effects, combat effects).
 */

#pragma once

#include "thing.h"
#include "thingtype.h"
#include <cstdint>
#include <memory>
#include <vector>
#include <map>

namespace shadow {
namespace client {

class Effect : public Thing {
public:
    Effect();
    ~Effect() override = default;

    static std::shared_ptr<Effect> create(uint16_t effectId);

    // Thing interface
    bool isEffect() const override { return true; }
    uint16_t getId() const override { return m_effectId; }
    ThingType* getThingType() const override;

    // Effect ID
    void setEffectId(uint16_t id) { m_effectId = id; }
    uint16_t getEffectId() const { return m_effectId; }

    // Animation
    void update(float deltaTime) override;
    bool isFinished() const { return m_finished; }

    // Current animation frame
    int getAnimationPhase() const { return m_animPhase; }

    // Drawing
    void draw(int x, int y, float scale = 1.0f) override;

    // Light
    uint8_t getLightIntensity() const override;
    uint8_t getLightColor() const override;

private:
    uint16_t m_effectId{0};
    int m_animPhase{0};
    float m_animTimer{0.0f};
    bool m_finished{false};

    // Animation duration per frame (ms)
    static constexpr float ANIM_DURATION = 75.0f;
};

using EffectPtr = std::shared_ptr<Effect>;

// Effect Manager - handles active effects on tiles
class EffectManager {
public:
    static EffectManager& instance();

    // Create and add effect to position
    EffectPtr createEffect(uint16_t effectId, const Position& pos);

    // Update all effects
    void update(float deltaTime);

    // Draw effects at position
    void drawEffects(const Position& pos, int x, int y, float scale = 1.0f);

    // Remove finished effects
    void cleanup();

    // Clear all effects
    void clear();

    // Get effects at position
    std::vector<EffectPtr> getEffects(const Position& pos) const;

private:
    EffectManager() = default;

    // Effects grouped by position for efficient lookup
    std::map<Position, std::vector<EffectPtr>> m_effects;
};

} // namespace client
} // namespace shadow
