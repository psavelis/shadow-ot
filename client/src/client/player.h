/**
 * Shadow OT Client - Player
 *
 * Player creature with additional stats and inventory.
 */

#pragma once

#include "creature.h"
#include <array>
#include <memory>
#include <unordered_map>
#include <unordered_set>

namespace shadow {
namespace client {

class Item;

// Inventory slots
enum class InventorySlot : uint8_t {
    Head = 1,
    Necklace = 2,
    Backpack = 3,
    Armor = 4,
    RightHand = 5,
    LeftHand = 6,
    Legs = 7,
    Feet = 8,
    Ring = 9,
    Ammo = 10,
    Purse = 11,
    Last = 12
};

// Vocation
enum class Vocation : uint8_t {
    None = 0,
    Sorcerer = 1,
    Druid = 2,
    Paladin = 3,
    Knight = 4,
    MasterSorcerer = 5,
    ElderDruid = 6,
    RoyalPaladin = 7,
    EliteKnight = 8
};

// Skill types
enum class Skill : uint8_t {
    Fist = 0,
    Club = 1,
    Sword = 2,
    Axe = 3,
    Distance = 4,
    Shielding = 5,
    Fishing = 6,
    CriticalHitChance = 7,
    CriticalHitDamage = 8,
    LifeLeechChance = 9,
    LifeLeechAmount = 10,
    ManaLeechChance = 11,
    ManaLeechAmount = 12,
    Last = 13
};

class Player : public Creature {
public:
    Player();
    static std::shared_ptr<Player> create(uint32_t id);

    bool isPlayer() const override { return true; }

    // Level & Experience
    uint32_t getLevel() const { return m_level; }
    void setLevel(uint32_t level) { m_level = level; }

    uint64_t getExperience() const { return m_experience; }
    void setExperience(uint64_t exp) { m_experience = exp; }

    float getLevelPercent() const { return m_levelPercent; }
    void setLevelPercent(float percent) { m_levelPercent = percent; }

    // Health & Mana
    uint32_t getHealth() const { return m_health; }
    uint32_t getMaxHealth() const { return m_maxHealth; }
    void setHealth(uint32_t health, uint32_t maxHealth);

    uint32_t getMana() const { return m_mana; }
    uint32_t getMaxMana() const { return m_maxMana; }
    void setMana(uint32_t mana, uint32_t maxMana);

    // Soul & Capacity
    uint8_t getSoul() const { return m_soul; }
    void setSoul(uint8_t soul) { m_soul = soul; }

    uint32_t getCapacity() const { return m_capacity; }
    uint32_t getFreeCapacity() const { return m_freeCapacity; }
    void setCapacity(uint32_t capacity, uint32_t freeCapacity);

    // Magic level
    uint32_t getMagicLevel() const { return m_magicLevel; }
    uint32_t getBaseMagicLevel() const { return m_baseMagicLevel; }
    void setMagicLevel(uint32_t level, uint32_t baseLevel);

    float getMagicLevelPercent() const { return m_magicLevelPercent; }
    void setMagicLevelPercent(float percent) { m_magicLevelPercent = percent; }

    // Skills
    uint32_t getSkillLevel(Skill skill) const;
    uint32_t getSkillBaseLevel(Skill skill) const;
    float getSkillPercent(Skill skill) const;
    void setSkill(Skill skill, uint32_t level, uint32_t baseLevel, float percent);

    // Vocation
    Vocation getVocation() const { return m_vocation; }
    void setVocation(Vocation vocation) { m_vocation = vocation; }
    std::string getVocationName() const;

    // Inventory
    std::shared_ptr<Item> getInventoryItem(InventorySlot slot) const;
    void setInventoryItem(InventorySlot slot, std::shared_ptr<Item> item);

    // Stamina
    uint16_t getStamina() const { return m_stamina; }
    void setStamina(uint16_t stamina) { m_stamina = stamina; }

    // Offline training
    uint16_t getOfflineTrainingTime() const { return m_offlineTrainingTime; }
    void setOfflineTrainingTime(uint16_t time) { m_offlineTrainingTime = time; }

    // States/conditions
    uint32_t getStates() const { return m_states; }
    void setStates(uint32_t states) { m_states = states; }
    bool hasState(uint32_t state) const { return (m_states & state) != 0; }

    // Premium
    bool isPremium() const { return m_premium; }
    void setPremium(bool premium) { m_premium = premium; }

    // Blessings
    uint8_t getBlessings() const { return m_blessings; }
    void setBlessings(uint8_t blessings) { m_blessings = blessings; }
    bool hasBlessing(uint8_t blessing) const { return (m_blessings & (1 << blessing)) != 0; }

    // Guild
    const std::string& getGuildName() const { return m_guildName; }
    void setGuildName(const std::string& name) { m_guildName = name; }

    const std::string& getGuildRank() const { return m_guildRank; }
    void setGuildRank(const std::string& rank) { m_guildRank = rank; }

    // Combat modes
    enum class AttackMode : uint8_t { Balanced = 0, Offensive = 1, Defensive = 2 };
    enum class ChaseMode : uint8_t { Stand = 0, Chase = 1 };
    enum class SecureMode : uint8_t { Off = 0, On = 1 };
    enum class PvPMode : uint8_t { Dove = 0, White = 1, Yellow = 2, Red = 3 };

    AttackMode getAttackMode() const { return m_attackMode; }
    void setAttackMode(AttackMode mode) { m_attackMode = mode; }

    ChaseMode getChaseMode() const { return m_chaseMode; }
    void setChaseMode(ChaseMode mode) { m_chaseMode = mode; }

    SecureMode getSecureMode() const { return m_secureMode; }
    void setSecureMode(SecureMode mode) { m_secureMode = mode; }

    PvPMode getPvPMode() const { return m_pvpMode; }
    void setPvPMode(PvPMode mode) { m_pvpMode = mode; }

private:
    // Stats
    uint32_t m_level{1};
    uint64_t m_experience{0};
    float m_levelPercent{0};

    uint32_t m_health{100};
    uint32_t m_maxHealth{100};
    uint32_t m_mana{0};
    uint32_t m_maxMana{0};

    uint8_t m_soul{0};
    uint32_t m_capacity{0};
    uint32_t m_freeCapacity{0};

    uint32_t m_magicLevel{0};
    uint32_t m_baseMagicLevel{0};
    float m_magicLevelPercent{0};

    uint16_t m_stamina{0};
    uint16_t m_offlineTrainingTime{0};

    Vocation m_vocation{Vocation::None};

    // Skills
    struct SkillData {
        uint32_t level{10};
        uint32_t baseLevel{10};
        float percent{0};
    };
    std::array<SkillData, static_cast<size_t>(Skill::Last)> m_skills;

    // Inventory
    std::array<std::shared_ptr<Item>, static_cast<size_t>(InventorySlot::Last)> m_inventory;

    // States
    uint32_t m_states{0};
    bool m_premium{false};
    uint8_t m_blessings{0};

    // Guild
    std::string m_guildName;
    std::string m_guildRank;

    // Combat modes
    AttackMode m_attackMode{AttackMode::Balanced};
    ChaseMode m_chaseMode{ChaseMode::Stand};
    SecureMode m_secureMode{SecureMode::On};
    PvPMode m_pvpMode{PvPMode::Dove};

    // Modern Tibia features
    uint32_t m_storeCoins{0};
    uint32_t m_transferableCoins{0};

    // Prey system
    struct PreySlot {
        uint16_t monsterId{0};
        uint8_t bonusType{0};
        uint8_t bonusValue{0};
        uint16_t bonusTimeLeft{0};
        uint8_t freeRerollTimeLeft{0};
        uint8_t state{0}; // 0=locked, 1=inactive, 2=active
    };
    std::array<PreySlot, 3> m_preySlots;

    // Bestiary tracking
    std::unordered_map<uint16_t, uint16_t> m_bestiaryKills; // monsterId -> killCount
    std::unordered_set<uint16_t> m_bestiaryUnlocked; // fully unlocked monsters

    // Bosstiary
    struct BossEntry {
        uint16_t bossId{0};
        uint16_t killCount{0};
        uint8_t tier{0}; // 0=none, 1=bane, 2=prowess, 3=expertise
        uint64_t lastKillTime{0};
    };
    std::unordered_map<uint16_t, BossEntry> m_bosstiary;

    // Forge/Dust system
    uint64_t m_forgeDust{0};
    uint8_t m_forgeDustLevel{0};

    // Charm points
    uint32_t m_charmPoints{0};
    std::unordered_set<uint8_t> m_unlockedCharms;

    // Tournament stats
    uint32_t m_tournamentCoins{0};

public:
    // Store coins
    uint32_t getStoreCoins() const { return m_storeCoins; }
    void setStoreCoins(uint32_t coins) { m_storeCoins = coins; }
    uint32_t getTransferableCoins() const { return m_transferableCoins; }
    void setTransferableCoins(uint32_t coins) { m_transferableCoins = coins; }

    // Prey system
    const PreySlot& getPreySlot(uint8_t index) const { return m_preySlots[index % 3]; }
    void setPreySlot(uint8_t index, const PreySlot& slot) { if (index < 3) m_preySlots[index] = slot; }

    // Bestiary
    uint16_t getBestiaryKills(uint16_t monsterId) const;
    void setBestiaryKills(uint16_t monsterId, uint16_t kills);
    bool isBestiaryUnlocked(uint16_t monsterId) const { return m_bestiaryUnlocked.count(monsterId) > 0; }
    void unlockBestiary(uint16_t monsterId) { m_bestiaryUnlocked.insert(monsterId); }

    // Bosstiary
    const BossEntry* getBossEntry(uint16_t bossId) const;
    void setBossEntry(uint16_t bossId, const BossEntry& entry) { m_bosstiary[bossId] = entry; }

    // Forge
    uint64_t getForgeDust() const { return m_forgeDust; }
    void setForgeDust(uint64_t dust) { m_forgeDust = dust; }
    uint8_t getForgeDustLevel() const { return m_forgeDustLevel; }
    void setForgeDustLevel(uint8_t level) { m_forgeDustLevel = level; }

    // Charms
    uint32_t getCharmPoints() const { return m_charmPoints; }
    void setCharmPoints(uint32_t points) { m_charmPoints = points; }
    bool hasCharm(uint8_t charmId) const { return m_unlockedCharms.count(charmId) > 0; }
    void unlockCharm(uint8_t charmId) { m_unlockedCharms.insert(charmId); }

    // Tournament
    uint32_t getTournamentCoins() const { return m_tournamentCoins; }
    void setTournamentCoins(uint32_t coins) { m_tournamentCoins = coins; }
};

using PlayerPtr = std::shared_ptr<Player>;

} // namespace client
} // namespace shadow
