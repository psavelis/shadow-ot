/**
 * Shadow OT Client - Local Player
 *
 * The player controlled by this client instance.
 */

#pragma once

#include "player.h"
#include <vector>
#include <functional>

namespace shadow {
namespace client {

class LocalPlayer : public Player {
public:
    LocalPlayer();
    static std::shared_ptr<LocalPlayer> create(uint32_t id);

    bool isLocalPlayer() const override { return true; }

    // Auto-walk path
    void setAutoWalkPath(const std::vector<Position::Direction>& path);
    void cancelAutoWalk();
    bool isAutoWalking() const { return !m_autoWalkPath.empty(); }
    const std::vector<Position::Direction>& getAutoWalkPath() const { return m_autoWalkPath; }

    void nextAutoWalkStep();

    // Pre-walking (client-side prediction)
    void preWalk(Position::Direction dir);
    void cancelPreWalk();
    bool isPreWalking() const { return m_preWalking; }

    // Known spells
    void addKnownSpell(uint16_t spellId);
    void removeKnownSpell(uint16_t spellId);
    bool knowsSpell(uint16_t spellId) const;
    const std::vector<uint16_t>& getKnownSpells() const { return m_knownSpells; }

    // VIP list
    struct VIPEntry {
        uint32_t id;
        std::string name;
        std::string description;
        uint32_t iconId;
        bool notifyLogin;
        bool online;
    };

    void addVIP(const VIPEntry& vip);
    void removeVIP(uint32_t id);
    void setVIPOnline(uint32_t id, bool online);
    const std::vector<VIPEntry>& getVIPList() const { return m_vipList; }

    // Target/follow
    uint32_t getAttackingCreatureId() const { return m_attackingCreatureId; }
    void setAttackingCreatureId(uint32_t id) { m_attackingCreatureId = id; }

    uint32_t getFollowingCreatureId() const { return m_followingCreatureId; }
    void setFollowingCreatureId(uint32_t id) { m_followingCreatureId = id; }

    // Walk lock (prevent walk during certain actions)
    bool isWalkLocked() const { return m_walkLocked; }
    void setWalkLocked(bool locked) { m_walkLocked = locked; }

    // Last known server position (for desyncs)
    const Position& getServerPosition() const { return m_serverPosition; }
    void setServerPosition(const Position& pos) { m_serverPosition = pos; }

    // Update
    void update(float deltaTime) override;

    // Callbacks
    using StatsChangeCallback = std::function<void()>;
    using PositionChangeCallback = std::function<void(const Position&, const Position&)>;

    void setOnStatsChange(StatsChangeCallback cb) { m_onStatsChange = cb; }
    void setOnPositionChange(PositionChangeCallback cb) { m_onPositionChange = cb; }

private:
    std::vector<Position::Direction> m_autoWalkPath;
    int m_autoWalkIndex{0};

    bool m_preWalking{false};
    Position m_preWalkPosition;

    std::vector<uint16_t> m_knownSpells;
    std::vector<VIPEntry> m_vipList;

    uint32_t m_attackingCreatureId{0};
    uint32_t m_followingCreatureId{0};

    bool m_walkLocked{false};
    Position m_serverPosition;

    StatsChangeCallback m_onStatsChange;
    PositionChangeCallback m_onPositionChange;
};

using LocalPlayerPtr = std::shared_ptr<LocalPlayer>;

} // namespace client
} // namespace shadow
