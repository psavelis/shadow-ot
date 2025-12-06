/**
 * Shadow OT Client - Lua Bindings Implementation
 */

#include "luabindings.h"
#include "position.h"
#include "item.h"
#include "creature.h"
#include "player.h"
#include "localplayer.h"
#include "tile.h"
#include "map.h"
#include "container.h"
#include "game.h"
#include "effect.h"
#include "missile.h"
#include <framework/ui/uimanager.h>
#include <framework/ui/uiwidget.h>

extern "C" {
#include <lua.h>
#include <lualib.h>
#include <lauxlib.h>
}

namespace shadow {
namespace client {

// Helper macros for binding
#define LUA_REGISTER_CLASS(L, name) \
    luaL_newmetatable(L, name); \
    lua_pushvalue(L, -1); \
    lua_setfield(L, -2, "__index")

#define LUA_REGISTER_METHOD(L, name, func) \
    lua_pushcfunction(L, func); \
    lua_setfield(L, -2, name)

// Position bindings

static int l_Position_new(lua_State* L) {
    int x = luaL_optinteger(L, 1, 0);
    int y = luaL_optinteger(L, 2, 0);
    int z = luaL_optinteger(L, 3, 7);

    auto* pos = static_cast<Position*>(lua_newuserdata(L, sizeof(Position)));
    new (pos) Position(x, y, z);

    luaL_getmetatable(L, "Position");
    lua_setmetatable(L, -2);
    return 1;
}

static int l_Position_getX(lua_State* L) {
    auto* pos = static_cast<Position*>(luaL_checkudata(L, 1, "Position"));
    lua_pushinteger(L, pos->x);
    return 1;
}

static int l_Position_getY(lua_State* L) {
    auto* pos = static_cast<Position*>(luaL_checkudata(L, 1, "Position"));
    lua_pushinteger(L, pos->y);
    return 1;
}

static int l_Position_getZ(lua_State* L) {
    auto* pos = static_cast<Position*>(luaL_checkudata(L, 1, "Position"));
    lua_pushinteger(L, pos->z);
    return 1;
}

static int l_Position_distanceTo(lua_State* L) {
    auto* pos = static_cast<Position*>(luaL_checkudata(L, 1, "Position"));
    auto* other = static_cast<Position*>(luaL_checkudata(L, 2, "Position"));
    lua_pushnumber(L, pos->distanceTo(*other));
    return 1;
}

static int l_Position_translated(lua_State* L) {
    auto* pos = static_cast<Position*>(luaL_checkudata(L, 1, "Position"));
    int dx = luaL_checkinteger(L, 2);
    int dy = luaL_checkinteger(L, 3);
    int dz = luaL_optinteger(L, 4, 0);

    auto* result = static_cast<Position*>(lua_newuserdata(L, sizeof(Position)));
    new (result) Position(pos->x + dx, pos->y + dy, pos->z + dz);

    luaL_getmetatable(L, "Position");
    lua_setmetatable(L, -2);
    return 1;
}

void registerPositionLuaBindings(lua_State* L) {
    LUA_REGISTER_CLASS(L, "Position");

    LUA_REGISTER_METHOD(L, "getX", l_Position_getX);
    LUA_REGISTER_METHOD(L, "getY", l_Position_getY);
    LUA_REGISTER_METHOD(L, "getZ", l_Position_getZ);
    LUA_REGISTER_METHOD(L, "distanceTo", l_Position_distanceTo);
    LUA_REGISTER_METHOD(L, "translated", l_Position_translated);

    lua_pop(L, 1);

    // Constructor
    lua_pushcfunction(L, l_Position_new);
    lua_setglobal(L, "Position");
}

// Item bindings

static Item* l_Item_check(lua_State* L, int index) {
    auto** item = static_cast<Item**>(luaL_checkudata(L, index, "Item"));
    return *item;
}

static int l_Item_getId(lua_State* L) {
    auto* item = l_Item_check(L, 1);
    lua_pushinteger(L, item ? item->getId() : 0);
    return 1;
}

static int l_Item_getCount(lua_State* L) {
    auto* item = l_Item_check(L, 1);
    lua_pushinteger(L, item ? item->getCount() : 0);
    return 1;
}

static int l_Item_isStackable(lua_State* L) {
    auto* item = l_Item_check(L, 1);
    lua_pushboolean(L, item ? item->isStackable() : false);
    return 1;
}

static int l_Item_isContainer(lua_State* L) {
    auto* item = l_Item_check(L, 1);
    lua_pushboolean(L, item ? item->isContainer() : false);
    return 1;
}

static int l_Item_isPickupable(lua_State* L) {
    auto* item = l_Item_check(L, 1);
    lua_pushboolean(L, item ? item->isPickupable() : false);
    return 1;
}

static int l_Item_isUseable(lua_State* L) {
    auto* item = l_Item_check(L, 1);
    lua_pushboolean(L, item ? item->isUseable() : false);
    return 1;
}

void registerItemLuaBindings(lua_State* L) {
    LUA_REGISTER_CLASS(L, "Item");

    LUA_REGISTER_METHOD(L, "getId", l_Item_getId);
    LUA_REGISTER_METHOD(L, "getCount", l_Item_getCount);
    LUA_REGISTER_METHOD(L, "isStackable", l_Item_isStackable);
    LUA_REGISTER_METHOD(L, "isContainer", l_Item_isContainer);
    LUA_REGISTER_METHOD(L, "isPickupable", l_Item_isPickupable);
    LUA_REGISTER_METHOD(L, "isUseable", l_Item_isUseable);

    lua_pop(L, 1);
}

// Creature bindings

static Creature* l_Creature_check(lua_State* L, int index) {
    auto** creature = static_cast<Creature**>(luaL_checkudata(L, index, "Creature"));
    return *creature;
}

static int l_Creature_getId(lua_State* L) {
    auto* creature = l_Creature_check(L, 1);
    lua_pushinteger(L, creature ? creature->getCreatureId() : 0);
    return 1;
}

static int l_Creature_getName(lua_State* L) {
    auto* creature = l_Creature_check(L, 1);
    lua_pushstring(L, creature ? creature->getName().c_str() : "");
    return 1;
}

static int l_Creature_getHealthPercent(lua_State* L) {
    auto* creature = l_Creature_check(L, 1);
    lua_pushinteger(L, creature ? creature->getHealthPercent() : 0);
    return 1;
}

static int l_Creature_getSpeed(lua_State* L) {
    auto* creature = l_Creature_check(L, 1);
    lua_pushinteger(L, creature ? creature->getSpeed() : 0);
    return 1;
}

static int l_Creature_getDirection(lua_State* L) {
    auto* creature = l_Creature_check(L, 1);
    lua_pushinteger(L, creature ? static_cast<int>(creature->getDirection()) : 0);
    return 1;
}

static int l_Creature_getPosition(lua_State* L) {
    auto* creature = l_Creature_check(L, 1);
    if (!creature) {
        lua_pushnil(L);
        return 1;
    }

    const auto& pos = creature->getPosition();
    auto* result = static_cast<Position*>(lua_newuserdata(L, sizeof(Position)));
    new (result) Position(pos);

    luaL_getmetatable(L, "Position");
    lua_setmetatable(L, -2);
    return 1;
}

static int l_Creature_isWalking(lua_State* L) {
    auto* creature = l_Creature_check(L, 1);
    lua_pushboolean(L, creature ? creature->isWalking() : false);
    return 1;
}

static int l_Creature_isPlayer(lua_State* L) {
    auto* creature = l_Creature_check(L, 1);
    lua_pushboolean(L, creature ? creature->isPlayer() : false);
    return 1;
}

static int l_Creature_isMonster(lua_State* L) {
    auto* creature = l_Creature_check(L, 1);
    lua_pushboolean(L, creature ? creature->isMonster() : false);
    return 1;
}

static int l_Creature_isNPC(lua_State* L) {
    auto* creature = l_Creature_check(L, 1);
    lua_pushboolean(L, creature ? creature->isNPC() : false);
    return 1;
}

void registerCreatureLuaBindings(lua_State* L) {
    LUA_REGISTER_CLASS(L, "Creature");

    LUA_REGISTER_METHOD(L, "getId", l_Creature_getId);
    LUA_REGISTER_METHOD(L, "getName", l_Creature_getName);
    LUA_REGISTER_METHOD(L, "getHealthPercent", l_Creature_getHealthPercent);
    LUA_REGISTER_METHOD(L, "getSpeed", l_Creature_getSpeed);
    LUA_REGISTER_METHOD(L, "getDirection", l_Creature_getDirection);
    LUA_REGISTER_METHOD(L, "getPosition", l_Creature_getPosition);
    LUA_REGISTER_METHOD(L, "isWalking", l_Creature_isWalking);
    LUA_REGISTER_METHOD(L, "isPlayer", l_Creature_isPlayer);
    LUA_REGISTER_METHOD(L, "isMonster", l_Creature_isMonster);
    LUA_REGISTER_METHOD(L, "isNPC", l_Creature_isNPC);

    lua_pop(L, 1);
}

// Player bindings

static Player* l_Player_check(lua_State* L, int index) {
    auto** player = static_cast<Player**>(luaL_checkudata(L, index, "Player"));
    return *player;
}

static int l_Player_getHealth(lua_State* L) {
    auto* player = l_Player_check(L, 1);
    lua_pushinteger(L, player ? player->getHealth() : 0);
    return 1;
}

static int l_Player_getMaxHealth(lua_State* L) {
    auto* player = l_Player_check(L, 1);
    lua_pushinteger(L, player ? player->getMaxHealth() : 0);
    return 1;
}

static int l_Player_getMana(lua_State* L) {
    auto* player = l_Player_check(L, 1);
    lua_pushinteger(L, player ? player->getMana() : 0);
    return 1;
}

static int l_Player_getMaxMana(lua_State* L) {
    auto* player = l_Player_check(L, 1);
    lua_pushinteger(L, player ? player->getMaxMana() : 0);
    return 1;
}

static int l_Player_getLevel(lua_State* L) {
    auto* player = l_Player_check(L, 1);
    lua_pushinteger(L, player ? player->getLevel() : 0);
    return 1;
}

static int l_Player_getMagicLevel(lua_State* L) {
    auto* player = l_Player_check(L, 1);
    lua_pushinteger(L, player ? player->getMagicLevel() : 0);
    return 1;
}

static int l_Player_getSkillLevel(lua_State* L) {
    auto* player = l_Player_check(L, 1);
    int skill = luaL_checkinteger(L, 2);
    lua_pushinteger(L, player ? player->getSkillLevel(static_cast<Skill>(skill)) : 0);
    return 1;
}

static int l_Player_getVocation(lua_State* L) {
    auto* player = l_Player_check(L, 1);
    lua_pushstring(L, player ? player->getVocationName().c_str() : "");
    return 1;
}

static int l_Player_getSoul(lua_State* L) {
    auto* player = l_Player_check(L, 1);
    lua_pushinteger(L, player ? player->getSoul() : 0);
    return 1;
}

static int l_Player_getCapacity(lua_State* L) {
    auto* player = l_Player_check(L, 1);
    lua_pushinteger(L, player ? player->getCapacity() : 0);
    return 1;
}

static int l_Player_getFreeCapacity(lua_State* L) {
    auto* player = l_Player_check(L, 1);
    lua_pushinteger(L, player ? player->getFreeCapacity() : 0);
    return 1;
}

static int l_Player_getStamina(lua_State* L) {
    auto* player = l_Player_check(L, 1);
    lua_pushinteger(L, player ? player->getStamina() : 0);
    return 1;
}

static int l_Player_getExperience(lua_State* L) {
    auto* player = l_Player_check(L, 1);
    lua_pushnumber(L, player ? static_cast<double>(player->getExperience()) : 0);
    return 1;
}

static int l_Player_isPremium(lua_State* L) {
    auto* player = l_Player_check(L, 1);
    lua_pushboolean(L, player ? player->isPremium() : false);
    return 1;
}

static int l_Player_getStoreCoins(lua_State* L) {
    auto* player = l_Player_check(L, 1);
    lua_pushinteger(L, player ? player->getStoreCoins() : 0);
    return 1;
}

static int l_Player_getTransferableCoins(lua_State* L) {
    auto* player = l_Player_check(L, 1);
    lua_pushinteger(L, player ? player->getTransferableCoins() : 0);
    return 1;
}

static int l_Player_getBlessings(lua_State* L) {
    auto* player = l_Player_check(L, 1);
    lua_pushinteger(L, player ? player->getBlessings() : 0);
    return 1;
}

static int l_Player_hasBlessing(lua_State* L) {
    auto* player = l_Player_check(L, 1);
    int blessing = luaL_checkinteger(L, 2);
    lua_pushboolean(L, player ? player->hasBlessing(blessing) : false);
    return 1;
}

static int l_Player_getCharmPoints(lua_State* L) {
    auto* player = l_Player_check(L, 1);
    lua_pushinteger(L, player ? player->getCharmPoints() : 0);
    return 1;
}

static int l_Player_getForgeDust(lua_State* L) {
    auto* player = l_Player_check(L, 1);
    lua_pushnumber(L, player ? static_cast<double>(player->getForgeDust()) : 0);
    return 1;
}

static int l_Player_getForgeDustLevel(lua_State* L) {
    auto* player = l_Player_check(L, 1);
    lua_pushinteger(L, player ? player->getForgeDustLevel() : 0);
    return 1;
}

static int l_Player_getBestiaryKills(lua_State* L) {
    auto* player = l_Player_check(L, 1);
    int monsterId = luaL_checkinteger(L, 2);
    lua_pushinteger(L, player ? player->getBestiaryKills(monsterId) : 0);
    return 1;
}

static int l_Player_isBestiaryUnlocked(lua_State* L) {
    auto* player = l_Player_check(L, 1);
    int monsterId = luaL_checkinteger(L, 2);
    lua_pushboolean(L, player ? player->isBestiaryUnlocked(monsterId) : false);
    return 1;
}

static int l_Player_hasCharm(lua_State* L) {
    auto* player = l_Player_check(L, 1);
    int charmId = luaL_checkinteger(L, 2);
    lua_pushboolean(L, player ? player->hasCharm(charmId) : false);
    return 1;
}

static int l_Player_getGuildName(lua_State* L) {
    auto* player = l_Player_check(L, 1);
    lua_pushstring(L, player ? player->getGuildName().c_str() : "");
    return 1;
}

static int l_Player_getGuildRank(lua_State* L) {
    auto* player = l_Player_check(L, 1);
    lua_pushstring(L, player ? player->getGuildRank().c_str() : "");
    return 1;
}

void registerPlayerLuaBindings(lua_State* L) {
    LUA_REGISTER_CLASS(L, "Player");

    // Inherit from Creature
    LUA_REGISTER_METHOD(L, "getId", l_Creature_getId);
    LUA_REGISTER_METHOD(L, "getName", l_Creature_getName);
    LUA_REGISTER_METHOD(L, "getHealthPercent", l_Creature_getHealthPercent);
    LUA_REGISTER_METHOD(L, "getSpeed", l_Creature_getSpeed);
    LUA_REGISTER_METHOD(L, "getDirection", l_Creature_getDirection);
    LUA_REGISTER_METHOD(L, "getPosition", l_Creature_getPosition);
    LUA_REGISTER_METHOD(L, "isWalking", l_Creature_isWalking);

    // Player-specific - Stats
    LUA_REGISTER_METHOD(L, "getHealth", l_Player_getHealth);
    LUA_REGISTER_METHOD(L, "getMaxHealth", l_Player_getMaxHealth);
    LUA_REGISTER_METHOD(L, "getMana", l_Player_getMana);
    LUA_REGISTER_METHOD(L, "getMaxMana", l_Player_getMaxMana);
    LUA_REGISTER_METHOD(L, "getLevel", l_Player_getLevel);
    LUA_REGISTER_METHOD(L, "getMagicLevel", l_Player_getMagicLevel);
    LUA_REGISTER_METHOD(L, "getSkillLevel", l_Player_getSkillLevel);
    LUA_REGISTER_METHOD(L, "getVocation", l_Player_getVocation);
    LUA_REGISTER_METHOD(L, "getSoul", l_Player_getSoul);
    LUA_REGISTER_METHOD(L, "getCapacity", l_Player_getCapacity);
    LUA_REGISTER_METHOD(L, "getFreeCapacity", l_Player_getFreeCapacity);
    LUA_REGISTER_METHOD(L, "getStamina", l_Player_getStamina);
    LUA_REGISTER_METHOD(L, "getExperience", l_Player_getExperience);
    LUA_REGISTER_METHOD(L, "isPremium", l_Player_isPremium);

    // Store/Currency
    LUA_REGISTER_METHOD(L, "getStoreCoins", l_Player_getStoreCoins);
    LUA_REGISTER_METHOD(L, "getTransferableCoins", l_Player_getTransferableCoins);

    // Blessings
    LUA_REGISTER_METHOD(L, "getBlessings", l_Player_getBlessings);
    LUA_REGISTER_METHOD(L, "hasBlessing", l_Player_hasBlessing);

    // Bestiary/Charms
    LUA_REGISTER_METHOD(L, "getBestiaryKills", l_Player_getBestiaryKills);
    LUA_REGISTER_METHOD(L, "isBestiaryUnlocked", l_Player_isBestiaryUnlocked);
    LUA_REGISTER_METHOD(L, "getCharmPoints", l_Player_getCharmPoints);
    LUA_REGISTER_METHOD(L, "hasCharm", l_Player_hasCharm);

    // Forge
    LUA_REGISTER_METHOD(L, "getForgeDust", l_Player_getForgeDust);
    LUA_REGISTER_METHOD(L, "getForgeDustLevel", l_Player_getForgeDustLevel);

    // Guild
    LUA_REGISTER_METHOD(L, "getGuildName", l_Player_getGuildName);
    LUA_REGISTER_METHOD(L, "getGuildRank", l_Player_getGuildRank);

    lua_pop(L, 1);
}

// Tile bindings

static Tile* l_Tile_check(lua_State* L, int index) {
    auto** tile = static_cast<Tile**>(luaL_checkudata(L, index, "Tile"));
    return *tile;
}

static int l_Tile_getPosition(lua_State* L) {
    auto* tile = l_Tile_check(L, 1);
    if (!tile) {
        lua_pushnil(L);
        return 1;
    }

    const auto& pos = tile->getPosition();
    auto* result = static_cast<Position*>(lua_newuserdata(L, sizeof(Position)));
    new (result) Position(pos);

    luaL_getmetatable(L, "Position");
    lua_setmetatable(L, -2);
    return 1;
}

static int l_Tile_isWalkable(lua_State* L) {
    auto* tile = l_Tile_check(L, 1);
    lua_pushboolean(L, tile ? tile->isWalkable() : false);
    return 1;
}

static int l_Tile_isPathable(lua_State* L) {
    auto* tile = l_Tile_check(L, 1);
    lua_pushboolean(L, tile ? tile->isPathable() : false);
    return 1;
}

static int l_Tile_getTopCreature(lua_State* L) {
    auto* tile = l_Tile_check(L, 1);
    if (!tile || tile->getCreatureCount() == 0) {
        lua_pushnil(L);
        return 1;
    }

    auto creature = tile->getCreature(0);
    auto** ud = static_cast<Creature**>(lua_newuserdata(L, sizeof(Creature*)));
    *ud = creature.get();

    luaL_getmetatable(L, "Creature");
    lua_setmetatable(L, -2);
    return 1;
}

static int l_Tile_getTopItem(lua_State* L) {
    auto* tile = l_Tile_check(L, 1);
    if (!tile || tile->getItemCount() == 0) {
        lua_pushnil(L);
        return 1;
    }

    auto item = tile->getItem(0);
    auto** ud = static_cast<Item**>(lua_newuserdata(L, sizeof(Item*)));
    *ud = item.get();

    luaL_getmetatable(L, "Item");
    lua_setmetatable(L, -2);
    return 1;
}

static int l_Tile_getGround(lua_State* L) {
    auto* tile = l_Tile_check(L, 1);
    if (!tile) {
        lua_pushnil(L);
        return 1;
    }

    auto ground = tile->getGround();
    if (!ground) {
        lua_pushnil(L);
        return 1;
    }

    auto** ud = static_cast<Item**>(lua_newuserdata(L, sizeof(Item*)));
    *ud = ground.get();

    luaL_getmetatable(L, "Item");
    lua_setmetatable(L, -2);
    return 1;
}

void registerTileLuaBindings(lua_State* L) {
    LUA_REGISTER_CLASS(L, "Tile");

    LUA_REGISTER_METHOD(L, "getPosition", l_Tile_getPosition);
    LUA_REGISTER_METHOD(L, "isWalkable", l_Tile_isWalkable);
    LUA_REGISTER_METHOD(L, "isPathable", l_Tile_isPathable);
    LUA_REGISTER_METHOD(L, "getTopCreature", l_Tile_getTopCreature);
    LUA_REGISTER_METHOD(L, "getTopItem", l_Tile_getTopItem);
    LUA_REGISTER_METHOD(L, "getGround", l_Tile_getGround);

    lua_pop(L, 1);
}

// Map bindings (g_map)

static int l_map_getTile(lua_State* L) {
    int x = luaL_checkinteger(L, 1);
    int y = luaL_checkinteger(L, 2);
    int z = luaL_checkinteger(L, 3);

    auto tile = g_map.getTile(Position(x, y, z));
    if (!tile) {
        lua_pushnil(L);
        return 1;
    }

    auto** ud = static_cast<Tile**>(lua_newuserdata(L, sizeof(Tile*)));
    *ud = tile.get();

    luaL_getmetatable(L, "Tile");
    lua_setmetatable(L, -2);
    return 1;
}

static int l_map_getCenterPosition(lua_State* L) {
    const auto& pos = g_map.getCentralPosition();
    auto* result = static_cast<Position*>(lua_newuserdata(L, sizeof(Position)));
    new (result) Position(pos.x, pos.y, pos.z);

    luaL_getmetatable(L, "Position");
    lua_setmetatable(L, -2);
    return 1;
}

static int l_map_getCreatureById(lua_State* L) {
    uint32_t id = luaL_checkinteger(L, 1);
    auto creature = g_map.getCreatureById(id);

    if (!creature) {
        lua_pushnil(L);
        return 1;
    }

    auto** ud = static_cast<Creature**>(lua_newuserdata(L, sizeof(Creature*)));
    *ud = creature.get();

    luaL_getmetatable(L, "Creature");
    lua_setmetatable(L, -2);
    return 1;
}

static int l_map_findPath(lua_State* L) {
    auto* start = static_cast<Position*>(luaL_checkudata(L, 1, "Position"));
    auto* goal = static_cast<Position*>(luaL_checkudata(L, 2, "Position"));

    auto path = g_map.findPath(*start, *goal);

    // findPath returns a list of directions, convert to positions
    lua_newtable(L);
    int i = 1;
    Position current = *start;
    for (const auto& dir : path) {
        current = current.translated(dir);
        auto* p = static_cast<Position*>(lua_newuserdata(L, sizeof(Position)));
        new (p) Position(current.x, current.y, current.z);
        luaL_getmetatable(L, "Position");
        lua_setmetatable(L, -2);
        lua_rawseti(L, -2, i++);
    }

    return 1;
}

void registerMapLuaBindings(lua_State* L) {
    lua_newtable(L);

    lua_pushcfunction(L, l_map_getTile);
    lua_setfield(L, -2, "getTile");

    lua_pushcfunction(L, l_map_getCenterPosition);
    lua_setfield(L, -2, "getCenterPosition");

    lua_pushcfunction(L, l_map_getCreatureById);
    lua_setfield(L, -2, "getCreatureById");

    lua_pushcfunction(L, l_map_findPath);
    lua_setfield(L, -2, "findPath");

    lua_setglobal(L, "g_map");
}

// Container bindings

static Container* l_Container_check(lua_State* L, int index) {
    auto** container = static_cast<Container**>(luaL_checkudata(L, index, "Container"));
    return *container;
}

static int l_Container_getId(lua_State* L) {
    auto* container = l_Container_check(L, 1);
    lua_pushinteger(L, container ? container->getId() : 0);
    return 1;
}

static int l_Container_getCapacity(lua_State* L) {
    auto* container = l_Container_check(L, 1);
    lua_pushinteger(L, container ? container->getCapacity() : 0);
    return 1;
}

static int l_Container_getItemCount(lua_State* L) {
    auto* container = l_Container_check(L, 1);
    lua_pushinteger(L, container ? container->getItemCount() : 0);
    return 1;
}

static int l_Container_getName(lua_State* L) {
    auto* container = l_Container_check(L, 1);
    lua_pushstring(L, container ? container->getName().c_str() : "");
    return 1;
}

static int l_Container_getItem(lua_State* L) {
    auto* container = l_Container_check(L, 1);
    int slot = luaL_checkinteger(L, 2);

    if (!container) {
        lua_pushnil(L);
        return 1;
    }

    auto item = container->getItem(slot);
    if (!item) {
        lua_pushnil(L);
        return 1;
    }

    auto** ud = static_cast<Item**>(lua_newuserdata(L, sizeof(Item*)));
    *ud = item.get();

    luaL_getmetatable(L, "Item");
    lua_setmetatable(L, -2);
    return 1;
}

static int l_containers_getContainer(lua_State* L) {
    uint8_t id = luaL_checkinteger(L, 1);
    auto container = g_containers.getContainer(id);

    if (!container) {
        lua_pushnil(L);
        return 1;
    }

    auto** ud = static_cast<Container**>(lua_newuserdata(L, sizeof(Container*)));
    *ud = container.get();

    luaL_getmetatable(L, "Container");
    lua_setmetatable(L, -2);
    return 1;
}

void registerContainerLuaBindings(lua_State* L) {
    LUA_REGISTER_CLASS(L, "Container");

    LUA_REGISTER_METHOD(L, "getId", l_Container_getId);
    LUA_REGISTER_METHOD(L, "getCapacity", l_Container_getCapacity);
    LUA_REGISTER_METHOD(L, "getItemCount", l_Container_getItemCount);
    LUA_REGISTER_METHOD(L, "getName", l_Container_getName);
    LUA_REGISTER_METHOD(L, "getItem", l_Container_getItem);

    lua_pop(L, 1);

    // g_containers global
    lua_newtable(L);
    lua_pushcfunction(L, l_containers_getContainer);
    lua_setfield(L, -2, "getContainer");
    lua_setglobal(L, "g_containers");
}

// Game bindings (g_game)

static int l_game_isOnline(lua_State* L) {
    lua_pushboolean(L, g_game.isOnline());
    return 1;
}

static int l_game_getLocalPlayer(lua_State* L) {
    auto player = g_game.getLocalPlayer();
    if (!player) {
        lua_pushnil(L);
        return 1;
    }

    auto** ud = static_cast<LocalPlayer**>(lua_newuserdata(L, sizeof(LocalPlayer*)));
    *ud = player.get();

    luaL_getmetatable(L, "Player");
    lua_setmetatable(L, -2);
    return 1;
}

static int l_game_walk(lua_State* L) {
    int dir = luaL_checkinteger(L, 1);
    g_game.walk(static_cast<Position::Direction>(dir));
    return 0;
}

static int l_game_turn(lua_State* L) {
    int dir = luaL_checkinteger(L, 1);
    g_game.turn(static_cast<Position::Direction>(dir));
    return 0;
}

static int l_game_stop(lua_State* L) {
    g_game.stop();
    return 0;
}

static int l_game_attack(lua_State* L) {
    uint32_t creatureId = luaL_checkinteger(L, 1);
    g_game.attack(creatureId);
    return 0;
}

static int l_game_follow(lua_State* L) {
    uint32_t creatureId = luaL_checkinteger(L, 1);
    g_game.follow(creatureId);
    return 0;
}

static int l_game_cancelAttackAndFollow(lua_State* L) {
    g_game.cancelAttackAndFollow();
    return 0;
}

static int l_game_say(lua_State* L) {
    const char* text = luaL_checkstring(L, 1);
    g_game.say(text);
    return 0;
}

static int l_game_yell(lua_State* L) {
    const char* text = luaL_checkstring(L, 1);
    g_game.yell(text);
    return 0;
}

static int l_game_whisper(lua_State* L) {
    const char* text = luaL_checkstring(L, 1);
    g_game.whisper(text);
    return 0;
}

static int l_game_privateMessage(lua_State* L) {
    const char* receiver = luaL_checkstring(L, 1);
    const char* text = luaL_checkstring(L, 2);
    g_game.privateMessage(receiver, text);
    return 0;
}

static int l_game_use(lua_State* L) {
    auto* pos = static_cast<Position*>(luaL_checkudata(L, 1, "Position"));
    uint16_t itemId = luaL_checkinteger(L, 2);
    uint8_t stackPos = luaL_checkinteger(L, 3);
    uint8_t index = luaL_optinteger(L, 4, 0);

    g_game.use(*pos, itemId, stackPos, index);
    return 0;
}

static int l_game_look(lua_State* L) {
    auto* pos = static_cast<Position*>(luaL_checkudata(L, 1, "Position"));
    uint16_t itemId = luaL_checkinteger(L, 2);
    uint8_t stackPos = luaL_checkinteger(L, 3);

    g_game.look(*pos, itemId, stackPos);
    return 0;
}

static int l_game_move(lua_State* L) {
    auto* fromPos = static_cast<Position*>(luaL_checkudata(L, 1, "Position"));
    uint16_t itemId = luaL_checkinteger(L, 2);
    uint8_t fromStackPos = luaL_checkinteger(L, 3);
    auto* toPos = static_cast<Position*>(luaL_checkudata(L, 4, "Position"));
    uint8_t count = luaL_optinteger(L, 5, 1);

    g_game.move(*fromPos, itemId, fromStackPos, *toPos, count);
    return 0;
}

static int l_game_openContainer(lua_State* L) {
    auto* pos = static_cast<Position*>(luaL_checkudata(L, 1, "Position"));
    uint16_t itemId = luaL_checkinteger(L, 2);
    uint8_t stackPos = luaL_checkinteger(L, 3);

    g_game.openContainer(*pos, itemId, stackPos);
    return 0;
}

static int l_game_closeContainer(lua_State* L) {
    uint8_t containerId = luaL_checkinteger(L, 1);
    g_game.closeContainer(containerId);
    return 0;
}

static int l_game_setAttackMode(lua_State* L) {
    uint8_t mode = luaL_checkinteger(L, 1);
    g_game.setAttackMode(mode);
    return 0;
}

static int l_game_setChaseMode(lua_State* L) {
    uint8_t mode = luaL_checkinteger(L, 1);
    g_game.setChaseMode(mode);
    return 0;
}

static int l_game_setSecureMode(lua_State* L) {
    uint8_t mode = luaL_checkinteger(L, 1);
    g_game.setSecureMode(mode);
    return 0;
}

static int l_game_logout(lua_State* L) {
    g_game.logout();
    return 0;
}

void registerGameLuaBindings(lua_State* L) {
    lua_newtable(L);

    lua_pushcfunction(L, l_game_isOnline);
    lua_setfield(L, -2, "isOnline");

    lua_pushcfunction(L, l_game_getLocalPlayer);
    lua_setfield(L, -2, "getLocalPlayer");

    lua_pushcfunction(L, l_game_walk);
    lua_setfield(L, -2, "walk");

    lua_pushcfunction(L, l_game_turn);
    lua_setfield(L, -2, "turn");

    lua_pushcfunction(L, l_game_stop);
    lua_setfield(L, -2, "stop");

    lua_pushcfunction(L, l_game_attack);
    lua_setfield(L, -2, "attack");

    lua_pushcfunction(L, l_game_follow);
    lua_setfield(L, -2, "follow");

    lua_pushcfunction(L, l_game_cancelAttackAndFollow);
    lua_setfield(L, -2, "cancelAttackAndFollow");

    lua_pushcfunction(L, l_game_say);
    lua_setfield(L, -2, "say");

    lua_pushcfunction(L, l_game_yell);
    lua_setfield(L, -2, "yell");

    lua_pushcfunction(L, l_game_whisper);
    lua_setfield(L, -2, "whisper");

    lua_pushcfunction(L, l_game_privateMessage);
    lua_setfield(L, -2, "privateMessage");

    lua_pushcfunction(L, l_game_use);
    lua_setfield(L, -2, "use");

    lua_pushcfunction(L, l_game_look);
    lua_setfield(L, -2, "look");

    lua_pushcfunction(L, l_game_move);
    lua_setfield(L, -2, "move");

    lua_pushcfunction(L, l_game_openContainer);
    lua_setfield(L, -2, "openContainer");

    lua_pushcfunction(L, l_game_closeContainer);
    lua_setfield(L, -2, "closeContainer");

    lua_pushcfunction(L, l_game_setAttackMode);
    lua_setfield(L, -2, "setAttackMode");

    lua_pushcfunction(L, l_game_setChaseMode);
    lua_setfield(L, -2, "setChaseMode");

    lua_pushcfunction(L, l_game_setSecureMode);
    lua_setfield(L, -2, "setSecureMode");

    lua_pushcfunction(L, l_game_logout);
    lua_setfield(L, -2, "logout");

    // Direction constants
    lua_newtable(L);
    lua_pushinteger(L, static_cast<int>(Position::North));
    lua_setfield(L, -2, "North");
    lua_pushinteger(L, static_cast<int>(Position::East));
    lua_setfield(L, -2, "East");
    lua_pushinteger(L, static_cast<int>(Position::South));
    lua_setfield(L, -2, "South");
    lua_pushinteger(L, static_cast<int>(Position::West));
    lua_setfield(L, -2, "West");
    lua_pushinteger(L, static_cast<int>(Position::NorthEast));
    lua_setfield(L, -2, "NorthEast");
    lua_pushinteger(L, static_cast<int>(Position::SouthEast));
    lua_setfield(L, -2, "SouthEast");
    lua_pushinteger(L, static_cast<int>(Position::SouthWest));
    lua_setfield(L, -2, "SouthWest");
    lua_pushinteger(L, static_cast<int>(Position::NorthWest));
    lua_setfield(L, -2, "NorthWest");
    lua_setfield(L, -2, "Direction");

    lua_setglobal(L, "g_game");
}

// UI bindings

static int l_ui_createWidget(lua_State* L) {
    const char* type = luaL_checkstring(L, 1);
    const char* name = luaL_optstring(L, 2, "");

    auto widget = framework::UIManager::instance().createWidget(type);
    if (widget && name[0] != '\0') {
        widget->setId(name);
    }

    if (!widget) {
        lua_pushnil(L);
        return 1;
    }

    auto** ud = static_cast<framework::UIWidget**>(
        lua_newuserdata(L, sizeof(framework::UIWidget*)));
    *ud = widget.get();

    luaL_getmetatable(L, "UIWidget");
    lua_setmetatable(L, -2);
    return 1;
}

static int l_ui_getRootWidget(lua_State* L) {
    auto root = framework::UIManager::instance().getRootWidget();
    if (!root) {
        lua_pushnil(L);
        return 1;
    }

    auto** ud = static_cast<framework::UIWidget**>(
        lua_newuserdata(L, sizeof(framework::UIWidget*)));
    *ud = root.get();

    luaL_getmetatable(L, "UIWidget");
    lua_setmetatable(L, -2);
    return 1;
}

static int l_ui_loadUI(lua_State* L) {
    const char* filename = luaL_checkstring(L, 1);
    auto widget = framework::UIManager::instance().loadUI(filename);

    if (!widget) {
        lua_pushnil(L);
        return 1;
    }

    auto** ud = static_cast<framework::UIWidget**>(
        lua_newuserdata(L, sizeof(framework::UIWidget*)));
    *ud = widget.get();

    luaL_getmetatable(L, "UIWidget");
    lua_setmetatable(L, -2);
    return 1;
}

// UIWidget methods

static framework::UIWidget* l_UIWidget_check(lua_State* L, int index) {
    auto** widget = static_cast<framework::UIWidget**>(luaL_checkudata(L, index, "UIWidget"));
    return *widget;
}

static int l_UIWidget_getId(lua_State* L) {
    auto* widget = l_UIWidget_check(L, 1);
    lua_pushstring(L, widget ? widget->getId().c_str() : "");
    return 1;
}

static int l_UIWidget_setId(lua_State* L) {
    auto* widget = l_UIWidget_check(L, 1);
    const char* id = luaL_checkstring(L, 2);
    if (widget) widget->setId(id);
    return 0;
}

static int l_UIWidget_getWidth(lua_State* L) {
    auto* widget = l_UIWidget_check(L, 1);
    lua_pushinteger(L, widget ? widget->getWidth() : 0);
    return 1;
}

static int l_UIWidget_getHeight(lua_State* L) {
    auto* widget = l_UIWidget_check(L, 1);
    lua_pushinteger(L, widget ? widget->getHeight() : 0);
    return 1;
}

static int l_UIWidget_setSize(lua_State* L) {
    auto* widget = l_UIWidget_check(L, 1);
    int width = luaL_checkinteger(L, 2);
    int height = luaL_checkinteger(L, 3);
    if (widget) widget->setSize(width, height);
    return 0;
}

static int l_UIWidget_setPosition(lua_State* L) {
    auto* widget = l_UIWidget_check(L, 1);
    int x = luaL_checkinteger(L, 2);
    int y = luaL_checkinteger(L, 3);
    if (widget) widget->setPosition(x, y);
    return 0;
}

static int l_UIWidget_setVisible(lua_State* L) {
    auto* widget = l_UIWidget_check(L, 1);
    bool visible = lua_toboolean(L, 2);
    if (widget) widget->setVisible(visible);
    return 0;
}

static int l_UIWidget_isVisible(lua_State* L) {
    auto* widget = l_UIWidget_check(L, 1);
    lua_pushboolean(L, widget ? widget->isVisible() : false);
    return 1;
}

static int l_UIWidget_setEnabled(lua_State* L) {
    auto* widget = l_UIWidget_check(L, 1);
    bool enabled = lua_toboolean(L, 2);
    if (widget) widget->setEnabled(enabled);
    return 0;
}

static int l_UIWidget_isEnabled(lua_State* L) {
    auto* widget = l_UIWidget_check(L, 1);
    lua_pushboolean(L, widget ? widget->isEnabled() : false);
    return 1;
}

static int l_UIWidget_setText(lua_State* L) {
    // auto* widget = l_UIWidget_check(L, 1);
    // const char* text = luaL_checkstring(L, 2);
    // TODO: UIWidget doesn't have setText - need to cast to specific widget type
    (void)L;
    return 0;
}

static int l_UIWidget_getText(lua_State* L) {
    // auto* widget = l_UIWidget_check(L, 1);
    // TODO: UIWidget doesn't have getText - need to cast to specific widget type
    lua_pushstring(L, "");
    return 1;
}

static int l_UIWidget_focus(lua_State* L) {
    auto* widget = l_UIWidget_check(L, 1);
    if (widget) widget->focus();
    return 0;
}

static int l_UIWidget_destroy(lua_State* L) {
    auto* widget = l_UIWidget_check(L, 1);
    if (widget) widget->destroy();
    return 0;
}

void registerUILuaBindings(lua_State* L) {
    // UIWidget metatable
    LUA_REGISTER_CLASS(L, "UIWidget");

    LUA_REGISTER_METHOD(L, "getId", l_UIWidget_getId);
    LUA_REGISTER_METHOD(L, "setId", l_UIWidget_setId);
    LUA_REGISTER_METHOD(L, "getWidth", l_UIWidget_getWidth);
    LUA_REGISTER_METHOD(L, "getHeight", l_UIWidget_getHeight);
    LUA_REGISTER_METHOD(L, "setSize", l_UIWidget_setSize);
    LUA_REGISTER_METHOD(L, "setPosition", l_UIWidget_setPosition);
    LUA_REGISTER_METHOD(L, "setVisible", l_UIWidget_setVisible);
    LUA_REGISTER_METHOD(L, "isVisible", l_UIWidget_isVisible);
    LUA_REGISTER_METHOD(L, "setEnabled", l_UIWidget_setEnabled);
    LUA_REGISTER_METHOD(L, "isEnabled", l_UIWidget_isEnabled);
    LUA_REGISTER_METHOD(L, "setText", l_UIWidget_setText);
    LUA_REGISTER_METHOD(L, "getText", l_UIWidget_getText);
    LUA_REGISTER_METHOD(L, "focus", l_UIWidget_focus);
    LUA_REGISTER_METHOD(L, "destroy", l_UIWidget_destroy);

    lua_pop(L, 1);

    // g_ui global
    lua_newtable(L);

    lua_pushcfunction(L, l_ui_createWidget);
    lua_setfield(L, -2, "createWidget");

    lua_pushcfunction(L, l_ui_getRootWidget);
    lua_setfield(L, -2, "getRootWidget");

    lua_pushcfunction(L, l_ui_loadUI);
    lua_setfield(L, -2, "loadUI");

    lua_setglobal(L, "g_ui");
}

// Effect bindings

static int l_effects_create(lua_State* L) {
    // TODO: Implement global effect manager
    // uint16_t effectId = luaL_checkinteger(L, 1);
    // auto* pos = static_cast<Position*>(luaL_checkudata(L, 2, "Position"));
    // g_effects.createEffect(effectId, *pos);
    (void)L;
    return 0;
}

static int l_missiles_create(lua_State* L) {
    // TODO: Implement global missile manager
    // uint16_t missileId = luaL_checkinteger(L, 1);
    // auto* from = static_cast<Position*>(luaL_checkudata(L, 2, "Position"));
    // auto* to = static_cast<Position*>(luaL_checkudata(L, 3, "Position"));
    // g_missiles.createMissile(missileId, *from, *to);
    (void)L;
    return 0;
}

void registerEffectLuaBindings(lua_State* L) {
    // g_effects global
    lua_newtable(L);
    lua_pushcfunction(L, l_effects_create);
    lua_setfield(L, -2, "create");
    lua_setglobal(L, "g_effects");

    // g_missiles global
    lua_newtable(L);
    lua_pushcfunction(L, l_missiles_create);
    lua_setfield(L, -2, "create");
    lua_setglobal(L, "g_missiles");
}

// Main registration function

void registerLuaBindings(lua_State* L) {
    registerPositionLuaBindings(L);
    registerItemLuaBindings(L);
    registerCreatureLuaBindings(L);
    registerPlayerLuaBindings(L);
    registerTileLuaBindings(L);
    registerMapLuaBindings(L);
    registerContainerLuaBindings(L);
    registerGameLuaBindings(L);
    registerUILuaBindings(L);
    registerEffectLuaBindings(L);
}

} // namespace client
} // namespace shadow
