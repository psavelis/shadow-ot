/**
 * Shadow OT Client - Effect Implementation
 */

#include "effect.h"
#include "tile.h"
#include "map.h"

namespace shadow {
namespace client {

Effect::Effect() : Thing() {
}

std::shared_ptr<Effect> Effect::create(uint16_t effectId) {
    auto effect = std::make_shared<Effect>();
    effect->setEffectId(effectId);
    return effect;
}

ThingType* Effect::getThingType() const {
    return ThingTypeManager::instance().getEffectType(m_effectId);
}

void Effect::update(float deltaTime) {
    if (m_finished) return;

    m_animTimer += deltaTime * 1000.0f;  // Convert to milliseconds

    auto* type = getThingType();
    if (!type) {
        m_finished = true;
        return;
    }

    int phases = type->getAnimationPhases();
    if (phases <= 1) {
        // Single frame effect, show for one cycle duration
        if (m_animTimer >= ANIM_DURATION) {
            m_finished = true;
        }
        return;
    }

    // Calculate current frame
    int frame = static_cast<int>(m_animTimer / ANIM_DURATION);
    if (frame >= phases) {
        // Animation complete
        m_finished = true;
        m_animPhase = phases - 1;
    } else {
        m_animPhase = frame;
    }
}

void Effect::draw(int x, int y, float scale) {
    if (m_finished) return;

    auto* type = getThingType();
    if (!type) return;

    // Effects typically don't have patterns, just animation phases
    type->draw(x, y, scale, 0, 0, 0, m_animPhase);
}

uint8_t Effect::getLightIntensity() const {
    auto* type = getThingType();
    return type ? type->getLightIntensity() : 0;
}

uint8_t Effect::getLightColor() const {
    auto* type = getThingType();
    return type ? type->getLightColor() : 0;
}

// EffectManager implementation

EffectManager& EffectManager::instance() {
    static EffectManager instance;
    return instance;
}

EffectPtr EffectManager::createEffect(uint16_t effectId, const Position& pos) {
    auto effect = Effect::create(effectId);
    effect->setPosition(pos);

    m_effects[pos].push_back(effect);

    // Also add to tile if it exists
    auto& map = g_map;
    auto tile = map.getTile(pos);
    if (tile) {
        tile->addEffect(effect);
    }

    return effect;
}

void EffectManager::update(float deltaTime) {
    for (auto& [pos, effects] : m_effects) {
        for (auto& effect : effects) {
            effect->update(deltaTime);
        }
    }

    // Remove finished effects
    cleanup();
}

void EffectManager::drawEffects(const Position& pos, int x, int y, float scale) {
    auto it = m_effects.find(pos);
    if (it == m_effects.end()) return;

    for (const auto& effect : it->second) {
        if (!effect->isFinished()) {
            effect->draw(x, y, scale);
        }
    }
}

void EffectManager::cleanup() {
    for (auto it = m_effects.begin(); it != m_effects.end();) {
        auto& effects = it->second;

        // Remove finished effects from vector
        effects.erase(
            std::remove_if(effects.begin(), effects.end(),
                [](const EffectPtr& e) { return e->isFinished(); }),
            effects.end());

        // Remove empty position entries
        if (effects.empty()) {
            it = m_effects.erase(it);
        } else {
            ++it;
        }
    }
}

void EffectManager::clear() {
    m_effects.clear();
}

std::vector<EffectPtr> EffectManager::getEffects(const Position& pos) const {
    auto it = m_effects.find(pos);
    if (it != m_effects.end()) {
        return it->second;
    }
    return {};
}

} // namespace client
} // namespace shadow

// Global accessor
shadow::client::EffectManager& g_effects = shadow::client::EffectManager::instance();
