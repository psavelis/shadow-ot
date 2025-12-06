/**
 * Shadow OT Client - Lua Bindings
 *
 * Lua bindings for all game classes and systems.
 */

#pragma once

struct lua_State;

namespace shadow {
namespace client {

// Initialize all Lua bindings
void registerLuaBindings(lua_State* L);

// Individual binding registrations
void registerPositionLuaBindings(lua_State* L);
void registerItemLuaBindings(lua_State* L);
void registerCreatureLuaBindings(lua_State* L);
void registerPlayerLuaBindings(lua_State* L);
void registerTileLuaBindings(lua_State* L);
void registerMapLuaBindings(lua_State* L);
void registerContainerLuaBindings(lua_State* L);
void registerGameLuaBindings(lua_State* L);
void registerUILuaBindings(lua_State* L);
void registerEffectLuaBindings(lua_State* L);

} // namespace client
} // namespace shadow
