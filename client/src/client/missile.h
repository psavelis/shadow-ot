/**
 * Shadow OT Client - Missile
 *
 * Projectile effects that travel between positions (arrows, spells, etc).
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

class Missile : public Thing {
public:
    Missile();
    ~Missile() override = default;

    static std::shared_ptr<Missile> create(uint16_t missileId,
                                           const Position& from,
                                           const Position& to);

    // Thing interface
    bool isMissile() const override { return true; }
    uint16_t getId() const override { return m_missileId; }
    ThingType* getThingType() const override;

    // Missile ID
    void setMissileId(uint16_t id) { m_missileId = id; }
    uint16_t getMissileId() const { return m_missileId; }

    // Trajectory
    const Position& getSource() const { return m_source; }
    const Position& getDestination() const { return m_destination; }
    void setTrajectory(const Position& from, const Position& to);

    // Animation/movement
    void update(float deltaTime) override;
    bool isFinished() const { return m_finished; }

    // Current progress (0.0 = at source, 1.0 = at destination)
    float getProgress() const { return m_progress; }

    // Get direction pattern for sprite selection
    int getDirectionPattern() const;

    // Current pixel position for rendering
    float getPixelX() const { return m_pixelX; }
    float getPixelY() const { return m_pixelY; }

    // Drawing
    void draw(int x, int y, float scale = 1.0f) override;

    // Draw at current calculated pixel position
    void drawAtPosition(int baseX, int baseY, float scale = 1.0f);

    // Light
    uint8_t getLightIntensity() const override;
    uint8_t getLightColor() const override;

private:
    uint16_t m_missileId{0};
    Position m_source;
    Position m_destination;

    float m_progress{0.0f};
    float m_pixelX{0.0f};
    float m_pixelY{0.0f};
    bool m_finished{false};

    // Calculated values
    float m_deltaX{0.0f};
    float m_deltaY{0.0f};
    float m_duration{0.0f};  // Total flight time

    // Speed in tiles per second
    static constexpr float MISSILE_SPEED = 10.0f;
    static constexpr int TILE_SIZE = 32;
};

using MissilePtr = std::shared_ptr<Missile>;

// Missile Manager - handles all active missiles
class MissileManager {
public:
    static MissileManager& instance();

    // Create and add missile
    MissilePtr createMissile(uint16_t missileId,
                             const Position& from,
                             const Position& to);

    // Update all missiles
    void update(float deltaTime);

    // Draw all missiles (should be called after map tiles)
    void draw(int viewX, int viewY, float scale = 1.0f);

    // Remove finished missiles
    void cleanup();

    // Clear all missiles
    void clear();

    // Get active missile count
    size_t getActiveCount() const { return m_missiles.size(); }

private:
    MissileManager() = default;

    std::vector<MissilePtr> m_missiles;
};

} // namespace client
} // namespace shadow

// Global accessor
extern shadow::client::MissileManager& g_missiles;
