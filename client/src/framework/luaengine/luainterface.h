/**
 * Shadow OT Client - Lua Interface
 *
 * LuaJIT integration for scripting and module support.
 */

#pragma once

#include <string>
#include <vector>
#include <functional>
#include <memory>
#include <map>
#include <any>

struct lua_State;

namespace shadow {
namespace framework {

class LuaInterface {
public:
    static LuaInterface& instance();

    bool init();
    void terminate();

    // Script execution
    bool loadScript(const std::string& filename);
    bool runScript(const std::string& code);
    bool loadModules(const std::string& modulePath);

    // Global functions
    template<typename Func>
    void registerGlobalFunction(const std::string& name, Func func);

    // Class registration
    template<typename T>
    void registerClass();

    template<typename T, typename Func>
    void bindClassMemberFunction(const std::string& name, Func func);

    template<typename T>
    void bindClassProperty(const std::string& name,
                          std::function<std::any(T*)> getter,
                          std::function<void(T*, std::any)> setter = nullptr);

    // Value manipulation
    void pushValue(bool value);
    void pushValue(int value);
    void pushValue(double value);
    void pushValue(const std::string& value);
    void pushValue(void* ptr);
    void pushNil();

    bool getBool(int index);
    int getInt(int index);
    double getDouble(int index);
    std::string getString(int index);
    void* getPointer(int index);

    // Table operations
    void createTable();
    void setTableField(const std::string& key);
    void getTableField(const std::string& key);
    void setTableIndex(int index);
    void getTableIndex(int index);

    // Global variables
    void setGlobal(const std::string& name);
    void getGlobal(const std::string& name);

    // Function calls
    bool callFunction(const std::string& name, int nargs = 0, int nresults = 0);
    bool callMethod(const std::string& objName, const std::string& methodName,
                   int nargs = 0, int nresults = 0);

    // Error handling
    const std::string& getLastError() const { return m_lastError; }
    void setErrorHandler(std::function<void(const std::string&)> handler);

    // Stack management
    int getStackTop() const;
    void pop(int n = 1);
    void clearStack();

    // Module system
    void addModulePath(const std::string& path);
    std::vector<std::string> getLoadedModules() const;
    void reloadModule(const std::string& moduleName);

    // Hooks for UI binding
    using WidgetCallback = std::function<void(void*)>;
    void registerWidgetCallback(const std::string& widgetType, const std::string& event,
                               WidgetCallback callback);

    // Direct Lua state access (advanced usage)
    lua_State* getState() const { return m_state; }

private:
    LuaInterface() = default;
    ~LuaInterface();
    LuaInterface(const LuaInterface&) = delete;
    LuaInterface& operator=(const LuaInterface&) = delete;

    void setupStandardLibraries();
    void registerCoreBindings();
    bool handleError(int result);

    lua_State* m_state{nullptr};
    std::string m_lastError;
    std::function<void(const std::string&)> m_errorHandler;
    std::vector<std::string> m_modulePaths;
    std::vector<std::string> m_loadedModules;

    // Class metadata storage
    struct ClassMeta {
        std::string name;
        std::map<std::string, std::any> methods;
        std::map<std::string, std::pair<std::any, std::any>> properties;
    };
    std::map<std::string, ClassMeta> m_classes;
};

} // namespace framework
} // namespace shadow

// Global accessor
extern shadow::framework::LuaInterface& g_lua;
