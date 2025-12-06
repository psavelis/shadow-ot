/**
 * Shadow OT Client - Creature
 *
 * Base class for all creatures (players, NPCs, monsters).
 */

#pragma once

#include "thing.h"
#include <string>
#include <memory>
#include <vector>
#include <cstdint>

namespace shadow {
namespace client {

struct Outfit {
    uint16_t lookType{0};
    uint16_t lookTypeEx{0};  // Item look
    uint8_t head{0};
    uint8_t body{0};
    uint8_t legs{0};
    uint8_t feet{0};
    uint8_t addons{0};
    uint16_t mount{0};

    bool hasMount() const { return mount != 0; }
    bool isItem() const { return lookType == 0 && lookTypeEx != 0; }
};

class Creature : public Thing {
public:
    Creature();
    virtual ~Creature() = default;

    static std::shared_ptr<Creature> create(uint32_t id);

    bool isCreature() const override { return true; }

    // Identification
    uint32_t getCreatureId() const { return m_id; }
    void setCreatureId(uint32_t id) { m_id = id; }

    const std::string& getName() const { return m_name; }
    void setName(const std::string& name) { m_name = name; }

    // Health
    int getHealthPercent() const { return m_healthPercent; }
    void setHealthPercent(int percent) { m_healthPercent = std::max(0, std::min(100, percent)); }

    // Direction
    Position::Direction getDirection() const { return m_direction; }
    void setDirection(Position::Direction dir) { m_direction = dir; }

    void turn(Position::Direction dir);

    // Movement
    uint16_t getSpeed() const { return m_speed; }
    void setSpeed(uint16_t speed) { m_speed = speed; }

    bool isWalking() const { return m_walking; }
    void walk(const Position& newPos, bool preWalk = false);
    void cancelWalk();
    void stopWalk();

    float getWalkOffset() const { return m_walkOffset; }
    int getWalkOffsetX() const;
    int getWalkOffsetY() const;

    // Outfit
    const Outfit& getOutfit() const { return m_outfit; }
    void setOutfit(const Outfit& outfit) { m_outfit = outfit; }

    // Skull/shield/emblem
    enum class Skull : uint8_t {
        None = 0, Yellow, Green, White, Red, Black, Orange
    };

    enum class Shield : uint8_t {
        None = 0, Whiteyellow, Whiteblue, Blue, Yellow,
        GreenShared, Yellow2, BlueNoPvp, YellowNoPvp
    };

    Skull getSkull() const { return m_skull; }
    void setSkull(Skull skull) { m_skull = skull; }

    Shield getShield() const { return m_shield; }
    void setShield(Shield shield) { m_shield = shield; }

    uint8_t getEmblem() const { return m_emblem; }
    void setEmblem(uint8_t emblem) { m_emblem = emblem; }

    uint8_t getIcon() const { return m_icon; }
    void setIcon(uint8_t icon) { m_icon = icon; }

    // Light
    uint8_t getLightIntensity() const override { return m_lightIntensity; }
    void setLightIntensity(uint8_t intensity) { m_lightIntensity = intensity; }

    uint8_t getLightColor() const override { return m_lightColor; }
    void setLightColor(uint8_t color) { m_lightColor = color; }

    // Combat square
    bool hasSquare() const { return m_hasSquare; }
    uint8_t getSquareColor() const { return m_squareColor; }
    void setSquare(uint8_t color) { m_hasSquare = true; m_squareColor = color; }
    void clearSquare() { m_hasSquare = false; }

    // Speech bubble
    void say(const std::string& text, int type);
    void clearSpeech();
    const std::string& getSpeechText() const { return m_speechText; }
    bool hasSpeech() const { return !m_speechText.empty(); }

    // Animation
    void update(float deltaTime) override;
    void draw(int x, int y, float scale = 1.0f) override;

    // Visibility
    bool isInvisible() const { return m_invisible; }
    void setInvisible(bool invisible) { m_invisible = invisible; }

    // Type flags
    bool isUnpassable() const { return m_unpassable; }
    void setUnpassable(bool unpassable) { m_unpassable = unpassable; }

protected:
    uint32_t m_id{0};
    std::string m_name;
    int m_healthPercent{100};
    Position::Direction m_direction{Position::South};
    uint16_t m_speed{220};

    Outfit m_outfit;
    Skull m_skull{Skull::None};
    Shield m_shield{Shield::None};
    uint8_t m_emblem{0};
    uint8_t m_icon{0};

    uint8_t m_lightIntensity{0};
    uint8_t m_lightColor{0};

    // Walking state
    bool m_walking{false};
    Position m_walkTarget;
    float m_walkOffset{0};
    float m_walkTimer{0};
    float m_walkDuration{0};

    // Square (battle marker)
    bool m_hasSquare{false};
    uint8_t m_squareColor{0};

    // Speech
    std::string m_speechText;
    float m_speechTimer{0};

    // Flags
    bool m_invisible{false};
    bool m_unpassable{true};

    // Animation
    int m_animationPhase{0};
    float m_animationTimer{0};
};

using CreaturePtr = std::shared_ptr<Creature>;

} // namespace client
} // namespace shadow
