/**
 * Shadow OT Client - Protocol Game
 *
 * Game server protocol implementation - handles all game opcodes.
 */

#pragma once

#include <framework/net/protocol.h>
#include <framework/net/connection.h>
#include "position.h"
#include "creature.h"
#include "item.h"
#include <functional>
#include <memory>
#include <string>

namespace shadow {
namespace client {

// Forward declarations
class Tile;
class Container;
class LocalPlayer;

// Speak types
enum class SpeakType : uint8_t {
    Say = 1,
    Whisper = 2,
    Yell = 3,
    PrivateFrom = 4,
    PrivateTo = 5,
    ChannelManagement = 6,
    Channel = 7,
    ChannelHighlight = 8,
    Spell = 9,
    NPCFrom = 10,
    NPCTo = 11,
    GamemasterBroadcast = 12,
    GamemasterChannel = 13,
    GamemasterPrivate = 14,
    MonsterSay = 16,
    MonsterYell = 17,
    RVRChannel = 18,
    RVRAnswer = 19,
    RVRContinue = 20,
    Red = 21,
    Orange = 22
};

// Text message types
enum class TextMessageType : uint8_t {
    ConsoleBlue = 4,
    ConsoleRed = 12,
    StatusDefault = 17,
    StatusWarning = 18,
    EventAdvance = 19,
    StatusSmall = 20,
    InfoDescription = 21,
    DamageDealt = 22,
    DamageReceived = 23,
    Heal = 24,
    Experience = 25,
    DamageOther = 26,
    HealOther = 27,
    ExperienceOther = 28,
    Loot = 29,
    TradeNpc = 30,
    Report = 38,
    HotkeyUse = 39,
    TutorialHint = 40,
    Thankyou = 41
};

class ProtocolGame {
public:
    ProtocolGame();
    ~ProtocolGame();

    // Connection
    bool connect(const std::string& host, uint16_t port,
                 const std::string& accountName, const std::string& password,
                 const std::string& characterName, uint32_t token = 0);
    void disconnect();
    bool isConnected() const;

    // Main loop processing
    void poll();

    // Message callback for Connection
    void onRecvMessage(framework::NetworkMessage& msg);

    // Send packets
    void sendPing();
    void sendLogout();
    void sendAutoWalk(const std::vector<Position::Direction>& path);
    void sendWalk(Position::Direction direction);
    void sendStop();
    void sendTurn(Position::Direction direction);
    void sendSay(SpeakType type, const std::string& message,
                 const std::string& receiver = "", uint16_t channelId = 0);
    void sendLook(const Position& pos, uint16_t itemId, uint8_t stackPos);
    void sendUse(const Position& pos, uint16_t itemId, uint8_t stackPos, uint8_t index);
    void sendUseWith(const Position& fromPos, uint16_t fromItemId, uint8_t fromStackPos,
                     const Position& toPos, uint16_t toItemId, uint8_t toStackPos);
    void sendMove(const Position& fromPos, uint16_t itemId, uint8_t fromStackPos,
                  const Position& toPos, uint8_t count);
    void sendRotate(const Position& pos, uint16_t itemId, uint8_t stackPos);
    void sendCloseContainer(uint8_t containerId);
    void sendUpContainer(uint8_t containerId);
    void sendAttack(uint32_t creatureId);
    void sendFollow(uint32_t creatureId);
    void sendCancelAttackAndFollow();
    void sendSetModes(uint8_t fightMode, uint8_t chaseMode, uint8_t secureMode, uint8_t pvpMode);
    void sendRequestOutfit();
    void sendSetOutfit(const Outfit& outfit);
    void sendAddVip(const std::string& name);
    void sendRemoveVip(uint32_t playerId);
    void sendBuyItem(uint16_t itemId, uint8_t subType, uint8_t amount, bool ignoreCapacity, bool withBackpack);
    void sendSellItem(uint16_t itemId, uint8_t subType, uint8_t amount, bool ignoreEquipped);
    void sendCloseNpcTrade();
    void sendRequestChannels();
    void sendOpenChannel(uint16_t channelId);
    void sendCloseChannel(uint16_t channelId);
    void sendOpenPrivateChannel(const std::string& name);
    void sendInviteToParty(uint32_t creatureId);
    void sendJoinParty(uint32_t creatureId);
    void sendRevokeInvitation(uint32_t creatureId);
    void sendPassLeadership(uint32_t creatureId);
    void sendLeaveParty();
    void sendEnableSharedExp(bool enable);
    void sendRequestQuestLog();
    void sendRequestQuestLine(uint16_t questId);
    void sendBugReport(const std::string& comment);

    // Modern Tibia operations (12.x+)
    void sendRequestBestiary();
    void sendRequestBestiaryMonsterData(uint16_t monsterId);
    void sendRequestBosstiary();
    void sendRequestBossSlots();
    void sendSelectBossSlot(uint8_t slotId, uint32_t bossId);
    void sendRequestPreyData();
    void sendSelectPreyMonster(uint8_t slotId, uint16_t monsterId);
    void sendPreyAction(uint8_t slotId, uint8_t action);
    void sendRequestImbuingData(const Position& pos, uint16_t itemId, uint8_t stackPos);
    void sendApplyImbuement(uint8_t slotId, uint32_t imbuementId, bool protectionCharm);
    void sendClearImbuement(uint8_t slotId);
    void sendRequestForge();
    void sendForgeItem(uint16_t itemId, uint8_t tier, bool success);
    void sendRequestStoreCoins();
    void sendOpenStore(uint8_t categoryId);
    void sendRequestCyclopediaData(uint8_t type);
    void sendRequestSupplyStash();
    void sendStashAction(uint8_t action, uint16_t itemId, uint32_t count);
    void sendPartyAnalyzerAction(uint8_t action);
    void sendClientCheck(const std::vector<uint8_t>& data);

    // XTEA key
    void setXTEAKey(const std::array<uint32_t, 4>& key);

private:
    // Packet parsing
    void parsePacket(framework::NetworkMessage& msg);

    // Server message parsers
    void parseLoginError(framework::NetworkMessage& msg);
    void parseLoginAdvice(framework::NetworkMessage& msg);
    void parseLoginWait(framework::NetworkMessage& msg);
    void parseLoginSuccess(framework::NetworkMessage& msg);
    void parsePing(framework::NetworkMessage& msg);
    void parsePingBack(framework::NetworkMessage& msg);
    void parseDeath(framework::NetworkMessage& msg);

    // Map packets
    void parseMapDescription(framework::NetworkMessage& msg);
    void parseMoveNorth(framework::NetworkMessage& msg);
    void parseMoveEast(framework::NetworkMessage& msg);
    void parseMoveSouth(framework::NetworkMessage& msg);
    void parseMoveWest(framework::NetworkMessage& msg);
    void parseUpdateTile(framework::NetworkMessage& msg);
    void parseFloorChange(framework::NetworkMessage& msg, uint8_t direction);

    // Creature packets
    void parseCreatureMove(framework::NetworkMessage& msg);
    void parseCreatureTurn(framework::NetworkMessage& msg);
    void parseCreatureAppear(framework::NetworkMessage& msg);
    void parseCreatureDisappear(framework::NetworkMessage& msg);
    void parseCreatureHealth(framework::NetworkMessage& msg);
    void parseCreatureLight(framework::NetworkMessage& msg);
    void parseCreatureOutfit(framework::NetworkMessage& msg);
    void parseCreatureSpeed(framework::NetworkMessage& msg);
    void parseCreatureSkull(framework::NetworkMessage& msg);
    void parseCreatureShield(framework::NetworkMessage& msg);
    void parseCreatureSquare(framework::NetworkMessage& msg);

    // Container packets
    void parseContainer(framework::NetworkMessage& msg);
    void parseContainerClose(framework::NetworkMessage& msg);
    void parseContainerAddItem(framework::NetworkMessage& msg);
    void parseContainerUpdateItem(framework::NetworkMessage& msg);
    void parseContainerRemoveItem(framework::NetworkMessage& msg);

    // Inventory packets
    void parseInventory(framework::NetworkMessage& msg);
    void parseInventoryEmpty(framework::NetworkMessage& msg);

    // World packets
    void parseWorldLight(framework::NetworkMessage& msg);
    void parseEffect(framework::NetworkMessage& msg);
    void parseMissile(framework::NetworkMessage& msg);
    void parseAnimatedText(framework::NetworkMessage& msg);

    // Player packets
    void parsePlayerStats(framework::NetworkMessage& msg);
    void parsePlayerSkills(framework::NetworkMessage& msg);
    void parseIcons(framework::NetworkMessage& msg);
    void parseCancelTarget(framework::NetworkMessage& msg);
    void parseCancelWalk(framework::NetworkMessage& msg);

    // Chat packets
    void parseSpeakType(framework::NetworkMessage& msg);
    void parseChannelList(framework::NetworkMessage& msg);
    void parseOpenChannel(framework::NetworkMessage& msg);
    void parsePrivateChannel(framework::NetworkMessage& msg);
    void parseCloseChannel(framework::NetworkMessage& msg);
    void parseTextMessage(framework::NetworkMessage& msg);

    // Dialog packets
    void parseOutfitDialog(framework::NetworkMessage& msg);

    // VIP packets
    void parseVipLogin(framework::NetworkMessage& msg);
    void parseVipLogout(framework::NetworkMessage& msg);
    void parseVipState(framework::NetworkMessage& msg);

    // Modern Tibia packets (12.x+)
    void parseBestiaryData(framework::NetworkMessage& msg);
    void parseBosstiaryData(framework::NetworkMessage& msg);
    void parsePreyData(framework::NetworkMessage& msg);
    void parseImbuementData(framework::NetworkMessage& msg);
    void parseForgeResult(framework::NetworkMessage& msg);
    void parseStoreCoins(framework::NetworkMessage& msg);
    void parseCyclopediaCharacterInfo(framework::NetworkMessage& msg);
    void parseCyclopediaMapData(framework::NetworkMessage& msg);
    void parseExaltationForge(framework::NetworkMessage& msg);
    void parseFamiliarData(framework::NetworkMessage& msg);
    void parseSupplyStash(framework::NetworkMessage& msg);
    void parsePartyAnalyzer(framework::NetworkMessage& msg);
    void parseClientCheck(framework::NetworkMessage& msg);
    void parseBosstiaryCooldown(framework::NetworkMessage& msg);

    // Helper methods for parsing
    Position parsePosition(framework::NetworkMessage& msg);
    ItemPtr parseItem(framework::NetworkMessage& msg);
    CreaturePtr parseCreature(framework::NetworkMessage& msg, uint16_t type);
    Outfit parseOutfitData(framework::NetworkMessage& msg);
    void parseMapArea(framework::NetworkMessage& msg, const Position& pos, int width, int height);
    int parseTileDescription(framework::NetworkMessage& msg, const Position& pos);

    // Network
    std::shared_ptr<framework::Connection> m_connection;
    framework::XTEACipher m_xtea;
    framework::NetworkMessage m_sendBuffer;
    framework::NetworkMessage m_recvBuffer;

    // State
    std::string m_accountName;
    std::string m_password;
    std::string m_characterName;
    uint32_t m_accountToken{0};
    bool m_connected{false};
    bool m_firstReceived{false};
};

} // namespace client
} // namespace shadow
