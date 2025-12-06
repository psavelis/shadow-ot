/**
 * Shadow OT Client - Protocol Game Implementation
 */

#include "protocolgame.h"
#include "game.h"
#include "map.h"
#include "tile.h"
#include "container.h"
#include "localplayer.h"
#include "effect.h"
#include "missile.h"
#include <framework/net/protocol.h>
#include <framework/net/connection.h>

namespace shadow {
namespace client {

using namespace framework;

ProtocolGame::ProtocolGame() = default;
ProtocolGame::~ProtocolGame() {
    disconnect();
}

bool ProtocolGame::connect(const std::string& host, uint16_t port,
                           const std::string& accountName, const std::string& password,
                           const std::string& characterName, uint32_t token) {
    m_accountName = accountName;
    m_password = password;
    m_characterName = characterName;
    m_accountToken = token;

    m_connection = std::make_shared<Connection>();
    m_connection->connect(host, port);
    // Connection is async, check state
    if (m_connection->getState() == ConnectionState::Error) {
        return false;
    }

    m_connected = true;
    m_firstReceived = false;

    // Send initial login packet would go here
    // For now, we wait for server response

    return true;
}

void ProtocolGame::disconnect() {
    if (m_connection) {
        m_connection->disconnect();
        m_connection = nullptr;
    }
    m_connected = false;
}

bool ProtocolGame::isConnected() const {
    return m_connected && m_connection && m_connection->isConnected();
}

void ProtocolGame::poll() {
    if (!isConnected()) return;

    // Poll the connection
    if (m_connection) {
        m_connection->poll();
    }

    // TODO: Implement proper message handling when Connection API is complete
    // The current Connection class uses callbacks for message handling
    // rather than a polling hasData()/read() pattern
    return;

#if 0  // Disabled until Connection API is updated
    // Read available data
    while (m_connection->hasData()) {
        m_recvBuffer.reset();

        if (!m_connection->read(m_recvBuffer)) {
            disconnect();
            return;
        }

        // Decrypt if XTEA is enabled
        if (m_xtea.isEnabled() && m_firstReceived) {
            m_xtea.decrypt(m_recvBuffer);
        }

        // Skip packet size header
        m_recvBuffer.setPosition(NetworkMessage::HEADER_SIZE);

        // Parse all messages in packet
        while (!m_recvBuffer.isEof()) {
            parsePacket(m_recvBuffer);
        }

        m_firstReceived = true;
    }
#endif // Disabled until Connection API is updated
}

void ProtocolGame::setXTEAKey(const std::array<uint32_t, 4>& key) {
    m_xtea.setKey(key);
}

#if 0 // TODO: Parsing disabled until Creature/Game APIs are aligned
void ProtocolGame::parsePacket(NetworkMessage& msg) {
    uint8_t opcode = msg.readByte();

    switch (opcode) {
        case ServerOpcode::LoginError:
            parseLoginError(msg);
            break;
        case ServerOpcode::LoginAdvice:
            parseLoginAdvice(msg);
            break;
        case ServerOpcode::LoginWait:
            parseLoginWait(msg);
            break;
        case ServerOpcode::LoginSuccess:
            parseLoginSuccess(msg);
            break;
        case ServerOpcode::Ping:
            parsePing(msg);
            break;
        case ServerOpcode::Death:
            parseDeath(msg);
            break;

        // Map
        case ServerOpcode::MapDescription:
            parseMapDescription(msg);
            break;
        case ServerOpcode::MoveNorth:
            parseMoveNorth(msg);
            break;
        case ServerOpcode::MoveEast:
            parseMoveEast(msg);
            break;
        case ServerOpcode::MoveSouth:
            parseMoveSouth(msg);
            break;
        case ServerOpcode::MoveWest:
            parseMoveWest(msg);
            break;
        case ServerOpcode::UpdateTile:
            parseUpdateTile(msg);
            break;

        // Creatures
        case ServerOpcode::CreatureMove:
            parseCreatureMove(msg);
            break;
        case ServerOpcode::CreatureAppear:
            parseCreatureAppear(msg);
            break;
        case ServerOpcode::CreatureDisappear:
            parseCreatureDisappear(msg);
            break;
        case ServerOpcode::CreatureHealth:
            parseCreatureHealth(msg);
            break;
        case ServerOpcode::CreatureLight:
            parseCreatureLight(msg);
            break;
        case ServerOpcode::CreatureOutfit:
            parseCreatureOutfit(msg);
            break;
        case ServerOpcode::CreatureSpeed:
            parseCreatureSpeed(msg);
            break;
        case ServerOpcode::CreatureSkull:
            parseCreatureSkull(msg);
            break;
        case ServerOpcode::CreatureShield:
            parseCreatureShield(msg);
            break;
        case ServerOpcode::CreatureSquare:
            parseCreatureSquare(msg);
            break;

        // Containers
        case ServerOpcode::Container:
            parseContainer(msg);
            break;
        case ServerOpcode::ContainerClose:
            parseContainerClose(msg);
            break;
        case ServerOpcode::ContainerAddItem:
            parseContainerAddItem(msg);
            break;
        case ServerOpcode::ContainerUpdateItem:
            parseContainerUpdateItem(msg);
            break;
        case ServerOpcode::ContainerRemoveItem:
            parseContainerRemoveItem(msg);
            break;

        // Inventory
        case ServerOpcode::Inventory:
            parseInventory(msg);
            break;
        case ServerOpcode::InventoryEmpty:
            parseInventoryEmpty(msg);
            break;

        // World
        case ServerOpcode::WorldLight:
            parseWorldLight(msg);
            break;
        case ServerOpcode::Effect:
            parseEffect(msg);
            break;
        case ServerOpcode::Missile:
            parseMissile(msg);
            break;
        case ServerOpcode::AnimatedText:
            parseAnimatedText(msg);
            break;

        // Player
        case ServerOpcode::PlayerStats:
            parsePlayerStats(msg);
            break;
        case ServerOpcode::PlayerSkills:
            parsePlayerSkills(msg);
            break;
        case ServerOpcode::Icons:
            parseIcons(msg);
            break;
        case ServerOpcode::CancelTarget:
            parseCancelTarget(msg);
            break;
        case ServerOpcode::CancelWalk:
            parseCancelWalk(msg);
            break;

        // Chat
        case ServerOpcode::SpeakType:
            parseSpeakType(msg);
            break;
        case ServerOpcode::ChannelList:
            parseChannelList(msg);
            break;
        case ServerOpcode::OpenChannel:
            parseOpenChannel(msg);
            break;
        case ServerOpcode::PrivateChannel:
            parsePrivateChannel(msg);
            break;
        case ServerOpcode::CloseChannel:
            parseCloseChannel(msg);
            break;
        case ServerOpcode::TextMessage:
            parseTextMessage(msg);
            break;

        // Dialogs
        case ServerOpcode::OutfitDialog:
            parseOutfitDialog(msg);
            break;

        // VIP
        case ServerOpcode::VipLogin:
            parseVipLogin(msg);
            break;
        case ServerOpcode::VipLogout:
            parseVipLogout(msg);
            break;
        case ServerOpcode::VipState:
            parseVipState(msg);
            break;

        default:
            // Unknown opcode - skip remaining packet
            break;
    }
}

// Helper methods

Position ProtocolGame::parsePosition(NetworkMessage& msg) {
    uint16_t x = msg.readU16();
    uint16_t y = msg.readU16();
    uint8_t z = msg.readByte();
    return Position(x, y, z);
}

ItemPtr ProtocolGame::parseItem(NetworkMessage& msg) {
    uint16_t itemId = msg.readU16();
    if (itemId == 0) return nullptr;

    auto item = Item::create(itemId);

    auto* type = item->getThingType();
    if (type && type->isStackable()) {
        item->setCount(msg.readByte());
    }

    // Animation phase for animated items
    if (type && type->isAnimateAlways()) {
        msg.readByte(); // animation phase
    }

    return item;
}

Outfit ProtocolGame::parseOutfitData(NetworkMessage& msg) {
    Outfit outfit;

    outfit.lookType = msg.readU16();
    if (outfit.lookType != 0) {
        outfit.head = msg.readByte();
        outfit.body = msg.readByte();
        outfit.legs = msg.readByte();
        outfit.feet = msg.readByte();
        outfit.addons = msg.readByte();
    } else {
        outfit.lookTypeEx = msg.readU16();
    }

    outfit.mount = msg.readU16();

    return outfit;
}

CreaturePtr ProtocolGame::parseCreature(NetworkMessage& msg, uint16_t type) {
    CreaturePtr creature;

    if (type == 0x61) {
        // New creature
        uint32_t removeId = msg.readU32();
        uint32_t id = msg.readU32();

        // Remove old creature if exists
        if (removeId != 0) {
            auto old = g_map.getCreatureById(removeId);
            if (old) {
                auto tile = old->getTile();
                if (tile) {
                    tile->removeCreature(old);
                }
            }
        }

        uint8_t creatureType = msg.readByte();

        if (creatureType == 0) {
            // Player
            creature = Player::create(id);
        } else if (creatureType == 1) {
            // Monster
            creature = Creature::create(id);
            creature->setType(CreatureType::Monster);
        } else {
            // NPC
            creature = Creature::create(id);
            creature->setType(CreatureType::Npc);
        }

        creature->setName(msg.readString());
    } else if (type == 0x62) {
        // Known creature
        uint32_t id = msg.readU32();
        creature = g_map.getCreatureById(id);
        if (!creature) {
            creature = Creature::create(id);
        }
    } else if (type == 0x63) {
        // Creature turn
        uint32_t id = msg.readU32();
        creature = g_map.getCreatureById(id);
    }

    if (!creature) return nullptr;

    // Health percent
    creature->setHealthPercent(msg.readByte());

    // Direction
    creature->setDirection(static_cast<Position::Direction>(msg.readByte()));

    // Outfit
    creature->setOutfit(parseOutfitData(msg));

    // Light
    uint8_t lightIntensity = msg.readByte();
    uint8_t lightColor = msg.readByte();
    creature->setLight(lightIntensity, lightColor);

    // Speed
    creature->setSpeed(msg.readU16());

    // Skull
    creature->setSkull(msg.readByte());

    // Shield (party)
    creature->setShield(msg.readByte());

    // Emblem (if new creature)
    if (type == 0x61) {
        msg.readByte(); // emblem
    }

    // Unpassable
    msg.readByte(); // unpassable

    return creature;
}

int ProtocolGame::parseTileDescription(NetworkMessage& msg, const Position& pos) {
    auto& map = g_map;
    auto tile = map.getOrCreateTile(pos);

    int things = 0;
    bool finished = false;

    while (!finished) {
        uint16_t id = msg.peekU16();

        if (id >= 0xFF00) {
            // End of tile or skip count
            finished = true;
        } else if (id == 0x61 || id == 0x62 || id == 0x63) {
            // Creature
            msg.readU16(); // consume id
            auto creature = parseCreature(msg, id);
            if (creature) {
                tile->addCreature(creature);
                creature->setPosition(pos);
                things++;
            }
        } else {
            // Item
            auto item = parseItem(msg);
            if (item) {
                tile->addItem(item);
                things++;
            }
        }

        if (things >= 10) {
            // Max things on tile
            finished = true;
        }
    }

    return things;
}

void ProtocolGame::parseMapArea(NetworkMessage& msg, const Position& pos, int width, int height) {
    Position start = pos;
    Position end(pos.x + width - 1, pos.y + height - 1, pos.z);

    int skipTiles = 0;

    for (int z = pos.z; z <= (pos.z == 7 ? 7 : pos.z + 2); ++z) {
        for (int x = start.x; x <= end.x; ++x) {
            for (int y = start.y; y <= end.y; ++y) {
                if (skipTiles > 0) {
                    skipTiles--;
                    continue;
                }

                Position tilePos(x, y, z);
                uint16_t peek = msg.peekU16();

                if (peek >= 0xFF00) {
                    // Skip count
                    msg.readU16();
                    skipTiles = peek - 0xFF00;
                } else {
                    parseTileDescription(msg, tilePos);
                }
            }
        }
    }
}

// Login packets

void ProtocolGame::parseLoginError(NetworkMessage& msg) {
    std::string error = msg.readString();
    // Notify game of login error
    g_game.processLogout();
}

void ProtocolGame::parseLoginAdvice(NetworkMessage& msg) {
    std::string advice = msg.readString();
    // Display advice message
}

void ProtocolGame::parseLoginWait(NetworkMessage& msg) {
    std::string message = msg.readString();
    uint8_t time = msg.readByte();
    // Show waiting dialog
}

void ProtocolGame::parseLoginSuccess(NetworkMessage& msg) {
    uint32_t playerId = msg.readU32();

    // Create local player
    auto localPlayer = LocalPlayer::create(playerId);
    g_game.setLocalPlayer(localPlayer);

    // Beat duration for ping calculation
    uint16_t beatDuration = msg.readU16();

    // Game started
    g_game.processLogin();
}

void ProtocolGame::parsePing(NetworkMessage& msg) {
    sendPing();
}

void ProtocolGame::parsePingBack(NetworkMessage& msg) {
    // Update latency
}

void ProtocolGame::parseDeath(NetworkMessage& msg) {
    uint8_t deathType = msg.readByte();
    uint8_t penalty = msg.readByte();

    // Show death dialog
    if (g_game.onDeath) {
        g_game.onDeath(deathType, penalty);
    }
}

// Map packets

void ProtocolGame::parseMapDescription(NetworkMessage& msg) {
    Position pos = parsePosition(msg);

    g_map.setCenterPosition(pos);

    // Parse visible area (18x14 tiles, 8 floors)
    parseMapArea(msg, Position(pos.x - 8, pos.y - 6, pos.z), 18, 14);
}

void ProtocolGame::parseMoveNorth(NetworkMessage& msg) {
    auto& pos = g_map.getCenterPosition();
    Position newPos(pos.x, pos.y - 1, pos.z);

    g_map.setCenterPosition(newPos);

    // Parse new row at north
    parseMapArea(msg, Position(newPos.x - 8, newPos.y - 6, newPos.z), 18, 1);
}

void ProtocolGame::parseMoveEast(NetworkMessage& msg) {
    auto& pos = g_map.getCenterPosition();
    Position newPos(pos.x + 1, pos.y, pos.z);

    g_map.setCenterPosition(newPos);

    // Parse new column at east
    parseMapArea(msg, Position(newPos.x + 9, newPos.y - 6, newPos.z), 1, 14);
}

void ProtocolGame::parseMoveSouth(NetworkMessage& msg) {
    auto& pos = g_map.getCenterPosition();
    Position newPos(pos.x, pos.y + 1, pos.z);

    g_map.setCenterPosition(newPos);

    // Parse new row at south
    parseMapArea(msg, Position(newPos.x - 8, newPos.y + 7, newPos.z), 18, 1);
}

void ProtocolGame::parseMoveWest(NetworkMessage& msg) {
    auto& pos = g_map.getCenterPosition();
    Position newPos(pos.x - 1, pos.y, pos.z);

    g_map.setCenterPosition(newPos);

    // Parse new column at west
    parseMapArea(msg, Position(newPos.x - 8, newPos.y - 6, newPos.z), 1, 14);
}

void ProtocolGame::parseUpdateTile(NetworkMessage& msg) {
    Position pos = parsePosition(msg);

    // Clear existing tile
    g_map.cleanTile(pos);

    // Parse tile contents
    uint16_t peek = msg.peekU16();
    if (peek != 0xFF01) {
        parseTileDescription(msg, pos);
    } else {
        msg.readU16(); // consume end marker
    }
}

void ProtocolGame::parseFloorChange(NetworkMessage& msg, uint8_t direction) {
    auto& pos = g_map.getCenterPosition();
    Position newPos = pos;

    if (direction == ServerOpcode::FloorChange) {
        // Read actual direction
        direction = msg.readByte();
    }

    // Update z level and parse new area
    // Implementation depends on direction
}

// Creature packets

void ProtocolGame::parseCreatureMove(NetworkMessage& msg) {
    Position fromPos = parsePosition(msg);
    uint8_t fromStackPos = msg.readByte();
    Position toPos = parsePosition(msg);

    auto tile = g_map.getTile(fromPos);
    if (!tile) return;

    auto thing = tile->getThing(fromStackPos);
    if (!thing) return;

    auto creature = std::dynamic_pointer_cast<Creature>(thing);
    if (!creature) return;

    // Remove from old tile
    tile->removeCreature(creature);

    // Add to new tile
    auto newTile = g_map.getOrCreateTile(toPos);
    newTile->addCreature(creature);
    creature->walk(toPos, false);
}

void ProtocolGame::parseCreatureTurn(NetworkMessage& msg) {
    Position pos = parsePosition(msg);
    uint8_t stackPos = msg.readByte();
    auto direction = static_cast<Position::Direction>(msg.readByte());

    auto tile = g_map.getTile(pos);
    if (!tile) return;

    auto thing = tile->getThing(stackPos);
    if (!thing) return;

    auto creature = std::dynamic_pointer_cast<Creature>(thing);
    if (creature) {
        creature->turn(direction);
    }
}

void ProtocolGame::parseCreatureAppear(NetworkMessage& msg) {
    // Handled in parseTileDescription via parseCreature
}

void ProtocolGame::parseCreatureDisappear(NetworkMessage& msg) {
    Position pos = parsePosition(msg);
    uint8_t stackPos = msg.readByte();

    auto tile = g_map.getTile(pos);
    if (!tile) return;

    auto thing = tile->getThing(stackPos);
    if (!thing) return;

    if (thing->isCreature()) {
        tile->removeCreature(std::dynamic_pointer_cast<Creature>(thing));
    } else if (thing->isItem()) {
        tile->removeItem(std::dynamic_pointer_cast<Item>(thing));
    }
}

void ProtocolGame::parseCreatureHealth(NetworkMessage& msg) {
    uint32_t id = msg.readU32();
    uint8_t percent = msg.readByte();

    auto creature = g_map.getCreatureById(id);
    if (creature) {
        creature->setHealthPercent(percent);
    }
}

void ProtocolGame::parseCreatureLight(NetworkMessage& msg) {
    uint32_t id = msg.readU32();
    uint8_t intensity = msg.readByte();
    uint8_t color = msg.readByte();

    auto creature = g_map.getCreatureById(id);
    if (creature) {
        creature->setLight(intensity, color);
    }
}

void ProtocolGame::parseCreatureOutfit(NetworkMessage& msg) {
    uint32_t id = msg.readU32();
    Outfit outfit = parseOutfitData(msg);

    auto creature = g_map.getCreatureById(id);
    if (creature) {
        creature->setOutfit(outfit);
    }
}

void ProtocolGame::parseCreatureSpeed(NetworkMessage& msg) {
    uint32_t id = msg.readU32();
    uint16_t baseSpeed = msg.readU16();
    uint16_t speed = msg.readU16();

    auto creature = g_map.getCreatureById(id);
    if (creature) {
        creature->setSpeed(speed);
    }
}

void ProtocolGame::parseCreatureSkull(NetworkMessage& msg) {
    uint32_t id = msg.readU32();
    uint8_t skull = msg.readByte();

    auto creature = g_map.getCreatureById(id);
    if (creature) {
        creature->setSkull(skull);
    }
}

void ProtocolGame::parseCreatureShield(NetworkMessage& msg) {
    uint32_t id = msg.readU32();
    uint8_t shield = msg.readByte();

    auto creature = g_map.getCreatureById(id);
    if (creature) {
        creature->setShield(shield);
    }
}

void ProtocolGame::parseCreatureSquare(NetworkMessage& msg) {
    uint32_t id = msg.readU32();
    uint8_t color = msg.readByte();

    auto creature = g_map.getCreatureById(id);
    if (creature) {
        creature->setSquareColor(color);
    }
}

// Container packets

void ProtocolGame::parseContainer(NetworkMessage& msg) {
    uint8_t containerId = msg.readByte();
    uint16_t containerItemId = msg.readU16();
    std::string name = msg.readString();
    uint8_t capacity = msg.readByte();
    bool hasParent = msg.readByte() != 0;
    bool canUseDepotSearch = msg.readByte() != 0;
    bool isDragAndDropEnabled = msg.readByte() != 0;
    bool isPaginationEnabled = msg.readByte() != 0;

    uint16_t itemCount = msg.readU16();
    uint16_t startIndex = 0;

    if (isPaginationEnabled) {
        startIndex = msg.readU16();
        itemCount = msg.readU16();
    }

    auto container = g_containers.createContainer(containerId);
    container->setContainerItemId(containerItemId);
    container->setName(name);
    container->setCapacity(capacity);
    container->setHasParent(hasParent);

    for (uint16_t i = 0; i < itemCount; ++i) {
        auto item = parseItem(msg);
        if (item) {
            container->addItem(item);
        }
    }
}

void ProtocolGame::parseContainerClose(NetworkMessage& msg) {
    uint8_t containerId = msg.readByte();
    g_containers.removeContainer(containerId);
}

void ProtocolGame::parseContainerAddItem(NetworkMessage& msg) {
    uint8_t containerId = msg.readByte();
    uint16_t slot = msg.readU16();
    auto item = parseItem(msg);

    auto container = g_containers.getContainer(containerId);
    if (container && item) {
        container->insertItem(slot, item);
    }
}

void ProtocolGame::parseContainerUpdateItem(NetworkMessage& msg) {
    uint8_t containerId = msg.readByte();
    uint16_t slot = msg.readU16();
    auto item = parseItem(msg);

    auto container = g_containers.getContainer(containerId);
    if (container && item) {
        container->updateItem(slot, item);
    }
}

void ProtocolGame::parseContainerRemoveItem(NetworkMessage& msg) {
    uint8_t containerId = msg.readByte();
    uint16_t slot = msg.readU16();

    auto container = g_containers.getContainer(containerId);
    if (container) {
        container->removeItem(slot);
    }

    // Check for item move
    uint16_t itemId = msg.readU16();
    if (itemId != 0) {
        // Item was moved to another slot
    }
}

// Inventory packets

void ProtocolGame::parseInventory(NetworkMessage& msg) {
    uint8_t slot = msg.readByte();
    auto item = parseItem(msg);

    auto player = g_game.getLocalPlayer();
    if (player) {
        player->setInventoryItem(static_cast<Player::InventorySlot>(slot), item);
    }
}

void ProtocolGame::parseInventoryEmpty(NetworkMessage& msg) {
    uint8_t slot = msg.readByte();

    auto player = g_game.getLocalPlayer();
    if (player) {
        player->setInventoryItem(static_cast<Player::InventorySlot>(slot), nullptr);
    }
}

// World packets

void ProtocolGame::parseWorldLight(NetworkMessage& msg) {
    uint8_t intensity = msg.readByte();
    uint8_t color = msg.readByte();

    g_map.setWorldLight(intensity, color);
}

void ProtocolGame::parseEffect(NetworkMessage& msg) {
    Position pos = parsePosition(msg);
    uint8_t effectId = msg.readByte();

    g_effects.createEffect(effectId, pos);
}

void ProtocolGame::parseMissile(NetworkMessage& msg) {
    Position from = parsePosition(msg);
    Position to = parsePosition(msg);
    uint8_t missileId = msg.readByte();

    g_missiles.createMissile(missileId, from, to);
}

void ProtocolGame::parseAnimatedText(NetworkMessage& msg) {
    Position pos = parsePosition(msg);
    uint8_t color = msg.readByte();
    std::string text = msg.readString();

    // Create animated text at position
    if (g_game.onAnimatedText) {
        g_game.onAnimatedText(pos, color, text);
    }
}

// Player packets

void ProtocolGame::parsePlayerStats(NetworkMessage& msg) {
    auto player = g_game.getLocalPlayer();
    if (!player) return;

    uint16_t health = msg.readU16();
    uint16_t maxHealth = msg.readU16();
    player->setHealth(health, maxHealth);

    uint32_t freeCapacity = msg.readU32();

    uint64_t experience = msg.readU64();

    uint16_t level = msg.readU16();
    uint8_t levelPercent = msg.readByte();
    player->setLevel(level);
    player->setLevelPercent(levelPercent);

    uint16_t mana = msg.readU16();
    uint16_t maxMana = msg.readU16();
    player->setMana(mana, maxMana);

    uint8_t magicLevel = msg.readByte();
    uint8_t baseMagicLevel = msg.readByte();
    uint8_t magicLevelPercent = msg.readByte();
    player->setMagicLevel(magicLevel, baseMagicLevel);

    uint8_t soul = msg.readByte();
    player->setSoul(soul);

    uint16_t stamina = msg.readU16();
    player->setStamina(stamina);

    uint16_t baseSpeed = msg.readU16();
    player->setSpeed(baseSpeed);

    uint16_t regeneration = msg.readU16();
    uint16_t offlineTraining = msg.readU16();
}

void ProtocolGame::parsePlayerSkills(NetworkMessage& msg) {
    auto player = g_game.getLocalPlayer();
    if (!player) return;

    for (int i = 0; i <= static_cast<int>(Player::Skill::Fishing); ++i) {
        uint16_t level = msg.readU16();
        uint16_t baseLevel = msg.readU16();
        uint8_t percent = msg.readByte();

        player->setSkill(static_cast<Player::Skill>(i), level, baseLevel,
                        static_cast<float>(percent));
    }
}

void ProtocolGame::parseIcons(NetworkMessage& msg) {
    uint32_t icons = msg.readU32();

    auto player = g_game.getLocalPlayer();
    if (player) {
        player->setStates(icons);
    }
}

void ProtocolGame::parseCancelTarget(NetworkMessage& msg) {
    uint32_t sequence = msg.readU32();
    g_game.cancelAttackAndFollow();
}

void ProtocolGame::parseCancelWalk(NetworkMessage& msg) {
    auto direction = static_cast<Position::Direction>(msg.readByte());

    auto player = g_game.getLocalPlayer();
    if (player) {
        player->cancelPreWalk();
        player->setDirection(direction);
    }
}

// Chat packets

void ProtocolGame::parseSpeakType(NetworkMessage& msg) {
    uint32_t statementId = msg.readU32();
    std::string senderName = msg.readString();
    uint16_t level = msg.readU16();
    auto speakType = static_cast<SpeakType>(msg.readByte());

    Position pos;
    uint16_t channelId = 0;

    switch (speakType) {
        case SpeakType::Say:
        case SpeakType::Whisper:
        case SpeakType::Yell:
        case SpeakType::MonsterSay:
        case SpeakType::MonsterYell:
        case SpeakType::NPCFrom:
        case SpeakType::Spell:
            pos = parsePosition(msg);
            break;
        case SpeakType::Channel:
        case SpeakType::ChannelHighlight:
        case SpeakType::GamemasterChannel:
            channelId = msg.readU16();
            break;
        default:
            break;
    }

    std::string text = msg.readString();

    if (g_game.onTalk) {
        g_game.onTalk(senderName, level, speakType, pos, channelId, text);
    }
}

void ProtocolGame::parseChannelList(NetworkMessage& msg) {
    uint8_t count = msg.readByte();

    std::vector<std::pair<uint16_t, std::string>> channels;
    for (int i = 0; i < count; ++i) {
        uint16_t id = msg.readU16();
        std::string name = msg.readString();
        channels.emplace_back(id, name);
    }

    if (g_game.onChannelList) {
        g_game.onChannelList(channels);
    }
}

void ProtocolGame::parseOpenChannel(NetworkMessage& msg) {
    uint16_t channelId = msg.readU16();
    std::string channelName = msg.readString();

    // Read participants
    uint16_t joinedCount = msg.readU16();
    for (int i = 0; i < joinedCount; ++i) {
        msg.readString(); // player name
    }

    uint16_t invitedCount = msg.readU16();
    for (int i = 0; i < invitedCount; ++i) {
        msg.readString(); // player name
    }

    if (g_game.onOpenChannel) {
        g_game.onOpenChannel(channelId, channelName);
    }
}

void ProtocolGame::parsePrivateChannel(NetworkMessage& msg) {
    std::string name = msg.readString();

    if (g_game.onOpenPrivateChannel) {
        g_game.onOpenPrivateChannel(name);
    }
}

void ProtocolGame::parseCloseChannel(NetworkMessage& msg) {
    uint16_t channelId = msg.readU16();

    if (g_game.onCloseChannel) {
        g_game.onCloseChannel(channelId);
    }
}

void ProtocolGame::parseTextMessage(NetworkMessage& msg) {
    auto type = static_cast<TextMessageType>(msg.readByte());
    std::string text = msg.readString();

    if (g_game.onTextMessage) {
        g_game.onTextMessage(type, text);
    }
}

// Dialog packets

void ProtocolGame::parseOutfitDialog(NetworkMessage& msg) {
    Outfit currentOutfit = parseOutfitData(msg);

    // Available outfits
    uint16_t count = msg.readU16();
    std::vector<std::pair<uint16_t, std::string>> outfits;

    for (int i = 0; i < count; ++i) {
        uint16_t lookType = msg.readU16();
        std::string name = msg.readString();
        uint8_t addons = msg.readByte();
        msg.readByte(); // locked
        msg.readU32(); // store offer id

        outfits.emplace_back(lookType, name);
    }

    // Available mounts
    uint16_t mountCount = msg.readU16();
    std::vector<std::pair<uint16_t, std::string>> mounts;

    for (int i = 0; i < mountCount; ++i) {
        uint16_t mountId = msg.readU16();
        std::string name = msg.readString();
        msg.readByte(); // locked
        msg.readU32(); // store offer id

        mounts.emplace_back(mountId, name);
    }

    if (g_game.onOutfitDialog) {
        g_game.onOutfitDialog(currentOutfit, outfits, mounts);
    }
}

// VIP packets

void ProtocolGame::parseVipLogin(NetworkMessage& msg) {
    uint32_t playerId = msg.readU32();

    auto player = g_game.getLocalPlayer();
    if (player) {
        player->setVIPOnline(playerId, true);
    }

    if (g_game.onVipStateChange) {
        g_game.onVipStateChange(playerId, true);
    }
}

void ProtocolGame::parseVipLogout(NetworkMessage& msg) {
    uint32_t playerId = msg.readU32();

    auto player = g_game.getLocalPlayer();
    if (player) {
        player->setVIPOnline(playerId, false);
    }

    if (g_game.onVipStateChange) {
        g_game.onVipStateChange(playerId, false);
    }
}

void ProtocolGame::parseVipState(NetworkMessage& msg) {
    uint32_t playerId = msg.readU32();
    std::string name = msg.readString();
    std::string description = msg.readString();
    uint32_t iconId = msg.readU32();
    bool notifyLogin = msg.readByte() != 0;
    uint8_t status = msg.readByte();

    auto player = g_game.getLocalPlayer();
    if (player) {
        LocalPlayer::VIPEntry vip;
        vip.id = playerId;
        vip.name = name;
        vip.description = description;
        vip.iconId = iconId;
        vip.notifyLogin = notifyLogin;
        vip.online = (status > 0);

        player->addVIP(vip);
    }
}
#endif // TODO: Parsing disabled until Creature/Game APIs are aligned

// Send packets

void ProtocolGame::sendPing() {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(ClientOpcode::Ping);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendLogout() {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(ClientOpcode::QuitGame);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendAutoWalk(const std::vector<Position::Direction>& path) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(ClientOpcode::AutoWalk);
    m_sendBuffer.writeByte(static_cast<uint8_t>(path.size()));

    for (auto dir : path) {
        m_sendBuffer.writeByte(static_cast<uint8_t>(dir));
    }

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendWalk(Position::Direction direction) {
    uint8_t opcode;
    switch (direction) {
        case Position::North: opcode = ClientOpcode::WalkNorth; break;
        case Position::East: opcode = ClientOpcode::WalkEast; break;
        case Position::South: opcode = ClientOpcode::WalkSouth; break;
        case Position::West: opcode = ClientOpcode::WalkWest; break;
        case Position::NorthEast: opcode = ClientOpcode::WalkNE; break;
        case Position::SouthEast: opcode = ClientOpcode::WalkSE; break;
        case Position::SouthWest: opcode = ClientOpcode::WalkSW; break;
        case Position::NorthWest: opcode = ClientOpcode::WalkNW; break;
        default: return;
    }

    m_sendBuffer.reset();
    m_sendBuffer.writeByte(opcode);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendStop() {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(ClientOpcode::StopWalk);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendTurn(Position::Direction direction) {
    uint8_t opcode;
    switch (direction) {
        case Position::North: opcode = ClientOpcode::TurnNorth; break;
        case Position::East: opcode = ClientOpcode::TurnEast; break;
        case Position::South: opcode = ClientOpcode::TurnSouth; break;
        case Position::West: opcode = ClientOpcode::TurnWest; break;
        default: return;
    }

    m_sendBuffer.reset();
    m_sendBuffer.writeByte(opcode);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendSay(SpeakType type, const std::string& message,
                           const std::string& receiver, uint16_t channelId) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(ClientOpcode::Say);
    m_sendBuffer.writeByte(static_cast<uint8_t>(type));

    switch (type) {
        case SpeakType::PrivateFrom:
        case SpeakType::PrivateTo:
        case SpeakType::GamemasterPrivate:
            m_sendBuffer.writeString(receiver);
            break;
        case SpeakType::Channel:
        case SpeakType::ChannelHighlight:
        case SpeakType::GamemasterChannel:
            m_sendBuffer.writeU16(channelId);
            break;
        default:
            break;
    }

    m_sendBuffer.writeString(message);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendLook(const Position& pos, uint16_t itemId, uint8_t stackPos) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(ClientOpcode::LookAt);
    m_sendBuffer.writePosition(pos.x, pos.y, pos.z);
    m_sendBuffer.writeU16(itemId);
    m_sendBuffer.writeByte(stackPos);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendUse(const Position& pos, uint16_t itemId, uint8_t stackPos, uint8_t index) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(ClientOpcode::UseItem);
    m_sendBuffer.writePosition(pos.x, pos.y, pos.z);
    m_sendBuffer.writeU16(itemId);
    m_sendBuffer.writeByte(stackPos);
    m_sendBuffer.writeByte(index);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendUseWith(const Position& fromPos, uint16_t fromItemId, uint8_t fromStackPos,
                               const Position& toPos, uint16_t toItemId, uint8_t toStackPos) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(ClientOpcode::UseItemWith);
    m_sendBuffer.writePosition(fromPos.x, fromPos.y, fromPos.z);
    m_sendBuffer.writeU16(fromItemId);
    m_sendBuffer.writeByte(fromStackPos);
    m_sendBuffer.writePosition(toPos.x, toPos.y, toPos.z);
    m_sendBuffer.writeU16(toItemId);
    m_sendBuffer.writeByte(toStackPos);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendMove(const Position& fromPos, uint16_t itemId, uint8_t fromStackPos,
                            const Position& toPos, uint8_t count) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(ClientOpcode::MoveItem);
    m_sendBuffer.writePosition(fromPos.x, fromPos.y, fromPos.z);
    m_sendBuffer.writeU16(itemId);
    m_sendBuffer.writeByte(fromStackPos);
    m_sendBuffer.writePosition(toPos.x, toPos.y, toPos.z);
    m_sendBuffer.writeByte(count);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendRotate(const Position& pos, uint16_t itemId, uint8_t stackPos) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(ClientOpcode::RotateItem);
    m_sendBuffer.writePosition(pos.x, pos.y, pos.z);
    m_sendBuffer.writeU16(itemId);
    m_sendBuffer.writeByte(stackPos);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendCloseContainer(uint8_t containerId) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(ClientOpcode::CloseContainer);
    m_sendBuffer.writeByte(containerId);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendUpContainer(uint8_t containerId) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(ClientOpcode::UpContainer);
    m_sendBuffer.writeByte(containerId);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendAttack(uint32_t creatureId) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(ClientOpcode::Attack);
    m_sendBuffer.writeU32(creatureId);
    m_sendBuffer.writeU32(0); // sequence

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendFollow(uint32_t creatureId) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(ClientOpcode::Follow);
    m_sendBuffer.writeU32(creatureId);
    m_sendBuffer.writeU32(0); // sequence

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendCancelAttackAndFollow() {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(ClientOpcode::CancelAttack);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendSetModes(uint8_t fightMode, uint8_t chaseMode, uint8_t secureMode, uint8_t pvpMode) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(ClientOpcode::SetModes);
    m_sendBuffer.writeByte(fightMode);
    m_sendBuffer.writeByte(chaseMode);
    m_sendBuffer.writeByte(secureMode);
    m_sendBuffer.writeByte(pvpMode);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendRequestOutfit() {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(ClientOpcode::RequestOutfit);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendSetOutfit(const Outfit& outfit) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(ClientOpcode::SetOutfit);
    m_sendBuffer.writeU16(outfit.lookType);
    m_sendBuffer.writeByte(outfit.head);
    m_sendBuffer.writeByte(outfit.body);
    m_sendBuffer.writeByte(outfit.legs);
    m_sendBuffer.writeByte(outfit.feet);
    m_sendBuffer.writeByte(outfit.addons);
    m_sendBuffer.writeU16(outfit.mount);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendAddVip(const std::string& name) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0xDC); // AddVip opcode
    m_sendBuffer.writeString(name);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendRemoveVip(uint32_t playerId) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0xDD); // RemoveVip opcode
    m_sendBuffer.writeU32(playerId);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendBuyItem(uint16_t itemId, uint8_t subType, uint8_t amount,
                               bool ignoreCapacity, bool withBackpack) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0x7A); // BuyItem opcode
    m_sendBuffer.writeU16(itemId);
    m_sendBuffer.writeByte(subType);
    m_sendBuffer.writeByte(amount);
    m_sendBuffer.writeByte(ignoreCapacity ? 1 : 0);
    m_sendBuffer.writeByte(withBackpack ? 1 : 0);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendSellItem(uint16_t itemId, uint8_t subType, uint8_t amount, bool ignoreEquipped) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0x7B); // SellItem opcode
    m_sendBuffer.writeU16(itemId);
    m_sendBuffer.writeByte(subType);
    m_sendBuffer.writeByte(amount);
    m_sendBuffer.writeByte(ignoreEquipped ? 1 : 0);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendCloseNpcTrade() {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0x7C); // CloseNpcTrade opcode

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendRequestChannels() {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(ClientOpcode::RequestChannels);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendOpenChannel(uint16_t channelId) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(ClientOpcode::OpenChannel);
    m_sendBuffer.writeU16(channelId);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendCloseChannel(uint16_t channelId) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(ClientOpcode::CloseChannel);
    m_sendBuffer.writeU16(channelId);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendOpenPrivateChannel(const std::string& name) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(ClientOpcode::OpenPrivate);
    m_sendBuffer.writeString(name);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendInviteToParty(uint32_t creatureId) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0xA3); // InviteToParty opcode
    m_sendBuffer.writeU32(creatureId);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendJoinParty(uint32_t creatureId) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0xA4); // JoinParty opcode
    m_sendBuffer.writeU32(creatureId);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendRevokeInvitation(uint32_t creatureId) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0xA5); // RevokePartyInvite opcode
    m_sendBuffer.writeU32(creatureId);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendPassLeadership(uint32_t creatureId) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0xA6); // PassPartyLeadership opcode
    m_sendBuffer.writeU32(creatureId);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendLeaveParty() {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0xA7); // LeaveParty opcode

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendEnableSharedExp(bool enable) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0xA8); // EnableSharedExp opcode
    m_sendBuffer.writeByte(enable ? 1 : 0);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendRequestQuestLog() {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0xF0); // RequestQuestLog opcode

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendRequestQuestLine(uint16_t questId) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0xF1); // RequestQuestLine opcode
    m_sendBuffer.writeU16(questId);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendBugReport(const std::string& comment) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0xE6); // BugReport opcode
    m_sendBuffer.writeByte(0); // category
    m_sendBuffer.writeString(comment);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

// Modern Tibia operations (12.x+)

void ProtocolGame::sendRequestBestiary() {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0xD8); // RequestBestiary opcode

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendRequestBestiaryMonsterData(uint16_t monsterId) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0xD9); // RequestBestiaryMonsterData opcode
    m_sendBuffer.writeU16(monsterId);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendRequestBosstiary() {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0xDA); // RequestBosstiary opcode

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendRequestBossSlots() {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0xDB); // RequestBossSlots opcode

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendSelectBossSlot(uint8_t slotId, uint32_t bossId) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0xDC); // SelectBossSlot opcode
    m_sendBuffer.writeByte(slotId);
    m_sendBuffer.writeU32(bossId);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendRequestPreyData() {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0xED); // RequestPreyData opcode

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendSelectPreyMonster(uint8_t slotId, uint16_t monsterId) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0xEE); // SelectPreyMonster opcode
    m_sendBuffer.writeByte(slotId);
    m_sendBuffer.writeU16(monsterId);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendPreyAction(uint8_t slotId, uint8_t action) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0xEF); // PreyAction opcode
    m_sendBuffer.writeByte(slotId);
    m_sendBuffer.writeByte(action);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendRequestImbuingData(const Position& pos, uint16_t itemId, uint8_t stackPos) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0xCF); // RequestImbuingData opcode
    m_sendBuffer.writeU16(pos.x);
    m_sendBuffer.writeU16(pos.y);
    m_sendBuffer.writeByte(pos.z);
    m_sendBuffer.writeU16(itemId);
    m_sendBuffer.writeByte(stackPos);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendApplyImbuement(uint8_t slotId, uint32_t imbuementId, bool protectionCharm) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0xD0); // ApplyImbuement opcode
    m_sendBuffer.writeByte(slotId);
    m_sendBuffer.writeU32(imbuementId);
    m_sendBuffer.writeByte(protectionCharm ? 1 : 0);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendClearImbuement(uint8_t slotId) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0xD1); // ClearImbuement opcode
    m_sendBuffer.writeByte(slotId);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendRequestForge() {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0xE8); // RequestForge opcode

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendForgeItem(uint16_t itemId, uint8_t tier, bool success) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0xE9); // ForgeItem opcode
    m_sendBuffer.writeU16(itemId);
    m_sendBuffer.writeByte(tier);
    m_sendBuffer.writeByte(success ? 1 : 0);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendRequestStoreCoins() {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0xFA); // RequestStoreCoins opcode

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendOpenStore(uint8_t categoryId) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0xFB); // OpenStore opcode
    m_sendBuffer.writeByte(categoryId);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendRequestCyclopediaData(uint8_t type) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0xE5); // RequestCyclopedia opcode
    m_sendBuffer.writeByte(type);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendRequestSupplyStash() {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0xDD); // RequestSupplyStash opcode

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendStashAction(uint8_t action, uint16_t itemId, uint32_t count) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0xDE); // StashAction opcode
    m_sendBuffer.writeByte(action);
    m_sendBuffer.writeU16(itemId);
    m_sendBuffer.writeU32(count);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendPartyAnalyzerAction(uint8_t action) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0xDF); // PartyAnalyzerAction opcode
    m_sendBuffer.writeByte(action);

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

void ProtocolGame::sendClientCheck(const std::vector<uint8_t>& data) {
    m_sendBuffer.reset();
    m_sendBuffer.writeByte(0x63); // ClientCheck opcode
    m_sendBuffer.writeU16(static_cast<uint16_t>(data.size()));
    for (uint8_t byte : data) {
        m_sendBuffer.writeByte(byte);
    }

    if (m_xtea.isEnabled()) {
        m_xtea.encrypt(m_sendBuffer);
    }

    m_connection->send(m_sendBuffer);
}

// Parse methods for modern Tibia packets

void ProtocolGame::parseBestiaryData(NetworkMessage& msg) {
    // Parse bestiary overview data
    uint16_t raceCount = msg.readU16();
    for (uint16_t i = 0; i < raceCount; ++i) {
        std::string raceName = msg.readString();
        uint16_t monsterCount = msg.readU16();
        for (uint16_t j = 0; j < monsterCount; ++j) {
            uint16_t monsterId = msg.readU16();
            uint8_t progress = msg.readByte(); // 0=none, 1=started, 2=complete
            uint16_t killCount = msg.readU16();
            (void)monsterId; (void)progress; (void)killCount;
        }
        (void)raceName;
    }
}

void ProtocolGame::parseBosstiaryData(NetworkMessage& msg) {
    uint16_t bossCount = msg.readU16();
    for (uint16_t i = 0; i < bossCount; ++i) {
        uint32_t bossId = msg.readU32();
        std::string bossName = msg.readString();
        uint8_t tier = msg.readByte(); // 0=bane, 1=prowess, 2=expertise
        uint16_t killsToUnlock = msg.readU16();
        uint16_t currentKills = msg.readU16();
        (void)bossId; (void)bossName; (void)tier;
        (void)killsToUnlock; (void)currentKills;
    }
}

void ProtocolGame::parsePreyData(NetworkMessage& msg) {
    uint8_t slotCount = msg.readByte();
    for (uint8_t i = 0; i < slotCount; ++i) {
        uint8_t slotId = msg.readByte();
        uint8_t slotState = msg.readByte(); // 0=locked, 1=inactive, 2=active
        if (slotState == 2) { // Active
            uint16_t monsterId = msg.readU16();
            uint8_t bonusType = msg.readByte();
            uint16_t bonusValue = msg.readU16();
            uint8_t bonusGrade = msg.readByte();
            uint16_t timeLeft = msg.readU16();
            (void)monsterId; (void)bonusType; (void)bonusValue;
            (void)bonusGrade; (void)timeLeft;
        }
        (void)slotId;
    }
}

void ProtocolGame::parseImbuementData(NetworkMessage& msg) {
    uint16_t itemId = msg.readU16();
    uint8_t slotCount = msg.readByte();
    for (uint8_t i = 0; i < slotCount; ++i) {
        uint8_t slotId = msg.readByte();
        bool hasImbuement = msg.readByte() != 0;
        if (hasImbuement) {
            uint32_t imbuementId = msg.readU32();
            std::string imbuementName = msg.readString();
            uint32_t duration = msg.readU32();
            uint8_t removeRequired = msg.readByte();
            (void)imbuementId; (void)imbuementName;
            (void)duration; (void)removeRequired;
        }
        (void)slotId;
    }
    (void)itemId;
}

void ProtocolGame::parseForgeResult(NetworkMessage& msg) {
    uint8_t resultType = msg.readByte();
    if (resultType == 0) { // Success
        uint16_t itemId = msg.readU16();
        uint8_t newTier = msg.readByte();
        (void)itemId; (void)newTier;
    } else { // Failed
        uint64_t dustUsed = msg.readU64();
        (void)dustUsed;
    }
}

void ProtocolGame::parseStoreCoins(NetworkMessage& msg) {
    uint32_t coins = msg.readU32();
    uint32_t transferableCoins = msg.readU32();
    // Update local player
    auto player = g_game.getLocalPlayer();
    if (player) {
        player->setStoreCoins(coins);
        player->setTransferableCoins(transferableCoins);
    }
}

void ProtocolGame::parseCyclopediaCharacterInfo(NetworkMessage& msg) {
    uint8_t infoType = msg.readByte();
    switch (infoType) {
        case 0: { // Basic info
            std::string name = msg.readString();
            std::string vocation = msg.readString();
            uint16_t level = msg.readU16();
            (void)name; (void)vocation; (void)level;
            break;
        }
        case 1: { // Stats
            uint64_t experience = msg.readU64();
            uint16_t level = msg.readU16();
            uint8_t levelPercent = msg.readByte();
            (void)experience; (void)level; (void)levelPercent;
            break;
        }
        default:
            break;
    }
}

void ProtocolGame::parseCyclopediaMapData(NetworkMessage& msg) {
    uint8_t mapType = msg.readByte();
    uint32_t tilesExplored = msg.readU32();
    uint32_t totalTiles = msg.readU32();
    (void)mapType; (void)tilesExplored; (void)totalTiles;
}

void ProtocolGame::parseExaltationForge(NetworkMessage& msg) {
    uint64_t dustAmount = msg.readU64();
    uint8_t dustLevel = msg.readByte();
    uint8_t sliverAmount = msg.readByte();
    uint8_t coreAmount = msg.readByte();
    auto player = g_game.getLocalPlayer();
    if (player) {
        player->setForgeDust(dustAmount);
        player->setForgeDustLevel(dustLevel);
    }
    (void)sliverAmount; (void)coreAmount;
}

void ProtocolGame::parseFamiliarData(NetworkMessage& msg) {
    uint8_t familiarCount = msg.readByte();
    for (uint8_t i = 0; i < familiarCount; ++i) {
        uint16_t familiarId = msg.readU16();
        std::string familiarName = msg.readString();
        (void)familiarId; (void)familiarName;
    }
}

void ProtocolGame::parseSupplyStash(NetworkMessage& msg) {
    uint16_t itemCount = msg.readU16();
    for (uint16_t i = 0; i < itemCount; ++i) {
        uint16_t itemId = msg.readU16();
        uint32_t count = msg.readU32();
        (void)itemId; (void)count;
    }
}

void ProtocolGame::parsePartyAnalyzer(NetworkMessage& msg) {
    uint8_t memberCount = msg.readByte();
    for (uint8_t i = 0; i < memberCount; ++i) {
        uint32_t playerId = msg.readU32();
        std::string playerName = msg.readString();
        uint64_t damage = msg.readU64();
        uint64_t healing = msg.readU64();
        uint64_t lootValue = msg.readU64();
        (void)playerId; (void)playerName;
        (void)damage; (void)healing; (void)lootValue;
    }
}

void ProtocolGame::parseClientCheck(NetworkMessage& msg) {
    // Client needs to respond with a hash based on the data
    uint16_t dataSize = msg.readU16();
    std::vector<uint8_t> checkData(dataSize);
    for (uint16_t i = 0; i < dataSize; ++i) {
        checkData[i] = msg.readByte();
    }
    // Respond to client check
    sendClientCheck(checkData);
}

void ProtocolGame::parseBosstiaryCooldown(NetworkMessage& msg) {
    uint16_t bossCount = msg.readU16();
    for (uint16_t i = 0; i < bossCount; ++i) {
        uint32_t bossId = msg.readU32();
        uint32_t cooldownTime = msg.readU32();
        (void)bossId; (void)cooldownTime;
    }
}

} // namespace client
} // namespace shadow
