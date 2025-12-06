/**
 * Shadow OT Client - Lua Interface Implementation
 */

#include "luainterface.h"
#include <framework/core/resourcemanager.h>
#include <filesystem>

extern "C" {
#include <lua.h>
#include <lualib.h>
#include <lauxlib.h>
}

namespace fs = std::filesystem;

namespace shadow {
namespace framework {

LuaInterface& LuaInterface::instance() {
    static LuaInterface instance;
    return instance;
}

LuaInterface::~LuaInterface() {
    terminate();
}

bool LuaInterface::init() {
    if (m_state) return true;

    // Create Lua state
    m_state = luaL_newstate();
    if (!m_state) {
        m_lastError = "Failed to create Lua state";
        return false;
    }

    // Open standard libraries
    setupStandardLibraries();

    // Register core bindings
    registerCoreBindings();

    // Add default module paths
    addModulePath("modules");
    addModulePath("data/modules");

    return true;
}

void LuaInterface::terminate() {
    if (m_state) {
        lua_close(m_state);
        m_state = nullptr;
    }

    m_loadedModules.clear();
    m_classes.clear();
}

void LuaInterface::setupStandardLibraries() {
    // Open safe libraries
    luaL_openlibs(m_state);

    // Remove potentially dangerous functions
    lua_pushnil(m_state);
    lua_setglobal(m_state, "os");

    lua_pushnil(m_state);
    lua_setglobal(m_state, "io");

    lua_pushnil(m_state);
    lua_setglobal(m_state, "loadfile");

    lua_pushnil(m_state);
    lua_setglobal(m_state, "dofile");
}

void LuaInterface::registerCoreBindings() {
    // Print function replacement
    lua_pushcfunction(m_state, [](lua_State* L) -> int {
        int n = lua_gettop(L);
        std::string msg;
        for (int i = 1; i <= n; i++) {
            if (lua_isstring(L, i)) {
                if (!msg.empty()) msg += "\t";
                msg += lua_tostring(L, i);
            }
        }
        // Log message (would connect to logging system)
        printf("[Lua] %s\n", msg.c_str());
        return 0;
    });
    lua_setglobal(m_state, "print");

    // Import function for modules
    lua_pushcfunction(m_state, [](lua_State* L) -> int {
        if (!lua_isstring(L, 1)) {
            lua_pushnil(L);
            lua_pushstring(L, "Import requires a module name string");
            return 2;
        }

        const char* moduleName = lua_tostring(L, 1);

        // Get the Lua interface
        auto& lua = LuaInterface::instance();

        // Try to find and load the module
        for (const auto& path : lua.m_modulePaths) {
            std::string modulePath = path + "/" + moduleName;

            // Check for module directory with init.lua
            std::string initPath = modulePath + "/init.lua";
            if (fs::exists(initPath)) {
                if (luaL_dofile(L, initPath.c_str()) == LUA_OK) {
                    return 1;
                }
            }

            // Check for module.lua file
            std::string filePath = modulePath + ".lua";
            if (fs::exists(filePath)) {
                if (luaL_dofile(L, filePath.c_str()) == LUA_OK) {
                    return 1;
                }
            }
        }

        lua_pushnil(L);
        lua_pushfstring(L, "Module not found: %s", moduleName);
        return 2;
    });
    lua_setglobal(m_state, "import");

    // Schedule function for delayed execution
    lua_pushcfunction(m_state, [](lua_State* L) -> int {
        if (!lua_isnumber(L, 1) || !lua_isfunction(L, 2)) {
            lua_pushboolean(L, 0);
            return 1;
        }

        // Store callback for later execution
        // This would integrate with the event dispatcher
        lua_pushboolean(L, 1);
        return 1;
    });
    lua_setglobal(m_state, "scheduleEvent");

    // Add g_game, g_map bindings (placeholders for now)
    lua_newtable(m_state);
    lua_setglobal(m_state, "g_game");

    lua_newtable(m_state);
    lua_setglobal(m_state, "g_map");

    lua_newtable(m_state);
    lua_setglobal(m_state, "g_ui");

    lua_newtable(m_state);
    lua_setglobal(m_state, "g_resources");
}

bool LuaInterface::loadScript(const std::string& filename) {
    extern ResourceManager& g_resources;

    std::string content = g_resources.readFileText(filename);
    if (content.empty()) {
        m_lastError = "Failed to read script: " + filename;
        return false;
    }

    return runScript(content);
}

bool LuaInterface::runScript(const std::string& code) {
    if (!m_state) {
        m_lastError = "Lua state not initialized";
        return false;
    }

    int result = luaL_dostring(m_state, code.c_str());
    return handleError(result);
}

bool LuaInterface::loadModules(const std::string& modulePath) {
    if (!fs::exists(modulePath)) {
        m_lastError = "Module path does not exist: " + modulePath;
        return false;
    }

    // Add to module paths
    addModulePath(modulePath);

    // Find and load all modules
    for (const auto& entry : fs::directory_iterator(modulePath)) {
        if (entry.is_directory()) {
            std::string initPath = entry.path().string() + "/init.lua";
            if (fs::exists(initPath)) {
                std::string moduleName = entry.path().filename().string();

                if (loadScript(initPath)) {
                    m_loadedModules.push_back(moduleName);
                } else {
                    // Log error but continue loading other modules
                }
            }
        }
    }

    return true;
}

void LuaInterface::pushValue(bool value) {
    lua_pushboolean(m_state, value);
}

void LuaInterface::pushValue(int value) {
    lua_pushinteger(m_state, value);
}

void LuaInterface::pushValue(double value) {
    lua_pushnumber(m_state, value);
}

void LuaInterface::pushValue(const std::string& value) {
    lua_pushstring(m_state, value.c_str());
}

void LuaInterface::pushValue(void* ptr) {
    lua_pushlightuserdata(m_state, ptr);
}

void LuaInterface::pushNil() {
    lua_pushnil(m_state);
}

bool LuaInterface::getBool(int index) {
    return lua_toboolean(m_state, index) != 0;
}

int LuaInterface::getInt(int index) {
    return static_cast<int>(lua_tointeger(m_state, index));
}

double LuaInterface::getDouble(int index) {
    return lua_tonumber(m_state, index);
}

std::string LuaInterface::getString(int index) {
    const char* str = lua_tostring(m_state, index);
    return str ? str : "";
}

void* LuaInterface::getPointer(int index) {
    return lua_touserdata(m_state, index);
}

void LuaInterface::createTable() {
    lua_newtable(m_state);
}

void LuaInterface::setTableField(const std::string& key) {
    lua_setfield(m_state, -2, key.c_str());
}

void LuaInterface::getTableField(const std::string& key) {
    lua_getfield(m_state, -1, key.c_str());
}

void LuaInterface::setTableIndex(int index) {
    lua_rawseti(m_state, -2, index);
}

void LuaInterface::getTableIndex(int index) {
    lua_rawgeti(m_state, -1, index);
}

void LuaInterface::setGlobal(const std::string& name) {
    lua_setglobal(m_state, name.c_str());
}

void LuaInterface::getGlobal(const std::string& name) {
    lua_getglobal(m_state, name.c_str());
}

bool LuaInterface::callFunction(const std::string& name, int nargs, int nresults) {
    lua_getglobal(m_state, name.c_str());

    if (!lua_isfunction(m_state, -1)) {
        m_lastError = "Function not found: " + name;
        lua_pop(m_state, 1);
        return false;
    }

    // Move function before arguments
    if (nargs > 0) {
        lua_insert(m_state, -(nargs + 1));
    }

    int result = lua_pcall(m_state, nargs, nresults, 0);
    return handleError(result);
}

bool LuaInterface::callMethod(const std::string& objName, const std::string& methodName,
                              int nargs, int nresults) {
    lua_getglobal(m_state, objName.c_str());

    if (!lua_istable(m_state, -1)) {
        m_lastError = "Object not found: " + objName;
        lua_pop(m_state, 1);
        return false;
    }

    lua_getfield(m_state, -1, methodName.c_str());

    if (!lua_isfunction(m_state, -1)) {
        m_lastError = "Method not found: " + objName + "." + methodName;
        lua_pop(m_state, 2);
        return false;
    }

    // Push self
    lua_pushvalue(m_state, -2);

    // Move self after function, before arguments
    lua_insert(m_state, -(nargs + 2));

    // Move function before self and arguments
    lua_insert(m_state, -(nargs + 2));

    // Remove the original table reference
    lua_remove(m_state, -(nargs + 2));

    int result = lua_pcall(m_state, nargs + 1, nresults, 0);
    return handleError(result);
}

void LuaInterface::setErrorHandler(std::function<void(const std::string&)> handler) {
    m_errorHandler = handler;
}

int LuaInterface::getStackTop() const {
    return lua_gettop(m_state);
}

void LuaInterface::pop(int n) {
    lua_pop(m_state, n);
}

void LuaInterface::clearStack() {
    lua_settop(m_state, 0);
}

void LuaInterface::addModulePath(const std::string& path) {
    // Check if already added
    for (const auto& p : m_modulePaths) {
        if (p == path) return;
    }
    m_modulePaths.push_back(path);

    // Update Lua package.path
    lua_getglobal(m_state, "package");
    lua_getfield(m_state, -1, "path");
    std::string currentPath = lua_tostring(m_state, -1);
    lua_pop(m_state, 1);

    std::string newPath = currentPath + ";" + path + "/?.lua;" + path + "/?/init.lua";
    lua_pushstring(m_state, newPath.c_str());
    lua_setfield(m_state, -2, "path");
    lua_pop(m_state, 1);
}

std::vector<std::string> LuaInterface::getLoadedModules() const {
    return m_loadedModules;
}

void LuaInterface::reloadModule(const std::string& moduleName) {
    // Clear the module from package.loaded
    lua_getglobal(m_state, "package");
    lua_getfield(m_state, -1, "loaded");
    lua_pushnil(m_state);
    lua_setfield(m_state, -2, moduleName.c_str());
    lua_pop(m_state, 2);

    // Reload the module
    for (const auto& path : m_modulePaths) {
        std::string initPath = path + "/" + moduleName + "/init.lua";
        if (fs::exists(initPath)) {
            loadScript(initPath);
            return;
        }
    }
}

void LuaInterface::registerWidgetCallback(const std::string& widgetType, const std::string& event,
                                          WidgetCallback callback) {
    // Store callback for UI event handling
    // This would be called when widgets emit events
}

bool LuaInterface::handleError(int result) {
    if (result != LUA_OK) {
        m_lastError = lua_tostring(m_state, -1);
        lua_pop(m_state, 1);

        if (m_errorHandler) {
            m_errorHandler(m_lastError);
        }

        return false;
    }
    return true;
}

} // namespace framework
} // namespace shadow

// Global accessor
shadow::framework::LuaInterface& g_lua = shadow::framework::LuaInterface::instance();
