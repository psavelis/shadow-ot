/**
 * Shadow OT Client - Player Implementation
 */

#include "player.h"
#include "item.h"

namespace shadow {
namespace client {

Player::Player() : Creature() {
}

std::shared_ptr<Player> Player::create(uint32_t id) {
    auto player = std::make_shared<Player>();
    player->setCreatureId(id);
    return player;
}

void Player::setHealth(uint32_t health, uint32_t maxHealth) {
    m_health = health;
    m_maxHealth = maxHealth;

    // Update health percent for health bar
    if (maxHealth > 0) {
        setHealthPercent(static_cast<int>(health * 100 / maxHealth));
    }
}

void Player::setMana(uint32_t mana, uint32_t maxMana) {
    m_mana = mana;
    m_maxMana = maxMana;
}

void Player::setCapacity(uint32_t capacity, uint32_t freeCapacity) {
    m_capacity = capacity;
    m_freeCapacity = freeCapacity;
}

void Player::setMagicLevel(uint32_t level, uint32_t baseLevel) {
    m_magicLevel = level;
    m_baseMagicLevel = baseLevel;
}

uint32_t Player::getSkillLevel(Skill skill) const {
    size_t idx = static_cast<size_t>(skill);
    if (idx < m_skills.size()) {
        return m_skills[idx].level;
    }
    return 0;
}

uint32_t Player::getSkillBaseLevel(Skill skill) const {
    size_t idx = static_cast<size_t>(skill);
    if (idx < m_skills.size()) {
        return m_skills[idx].baseLevel;
    }
    return 0;
}

float Player::getSkillPercent(Skill skill) const {
    size_t idx = static_cast<size_t>(skill);
    if (idx < m_skills.size()) {
        return m_skills[idx].percent;
    }
    return 0;
}

void Player::setSkill(Skill skill, uint32_t level, uint32_t baseLevel, float percent) {
    size_t idx = static_cast<size_t>(skill);
    if (idx < m_skills.size()) {
        m_skills[idx].level = level;
        m_skills[idx].baseLevel = baseLevel;
        m_skills[idx].percent = percent;
    }
}

std::string Player::getVocationName() const {
    switch (m_vocation) {
        case Vocation::None: return "None";
        case Vocation::Sorcerer: return "Sorcerer";
        case Vocation::Druid: return "Druid";
        case Vocation::Paladin: return "Paladin";
        case Vocation::Knight: return "Knight";
        case Vocation::MasterSorcerer: return "Master Sorcerer";
        case Vocation::ElderDruid: return "Elder Druid";
        case Vocation::RoyalPaladin: return "Royal Paladin";
        case Vocation::EliteKnight: return "Elite Knight";
        default: return "Unknown";
    }
}

std::shared_ptr<Item> Player::getInventoryItem(InventorySlot slot) const {
    size_t idx = static_cast<size_t>(slot);
    if (idx < m_inventory.size()) {
        return m_inventory[idx];
    }
    return nullptr;
}

void Player::setInventoryItem(InventorySlot slot, std::shared_ptr<Item> item) {
    size_t idx = static_cast<size_t>(slot);
    if (idx < m_inventory.size()) {
        m_inventory[idx] = item;
    }
}

uint16_t Player::getBestiaryKills(uint16_t monsterId) const {
    auto it = m_bestiaryKills.find(monsterId);
    return (it != m_bestiaryKills.end()) ? it->second : 0;
}

void Player::setBestiaryKills(uint16_t monsterId, uint16_t kills) {
    m_bestiaryKills[monsterId] = kills;
}

const Player::BossEntry* Player::getBossEntry(uint16_t bossId) const {
    auto it = m_bosstiary.find(bossId);
    return (it != m_bosstiary.end()) ? &it->second : nullptr;
}

} // namespace client
} // namespace shadow
