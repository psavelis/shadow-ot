/**
 * Shadow OT Client
 *
 * Modern Open Tibia client based on OTClient Redemption with Shadow OT extensions.
 */

#include <framework/core/application.h>
#include <framework/core/resourcemanager.h>
#include <framework/core/eventdispatcher.h>
#include <framework/core/configmanager.h>
#include <framework/graphics/graphics.h>
#include <framework/luaengine/luainterface.h>
#include <framework/platform/platform.h>

#include <shadow/realms/realmmanager.h>
#include <shadow/blockchain/wallet.h>

#include <iostream>
#include <string>

// Use framework globals
using shadow::framework::g_graphics;

#ifdef SHADOW_PLATFORM_WEB
#include <emscripten.h>
#endif

namespace {

void printBanner() {
    std::cout << R"(
   _____ _               _                 ____ _____
  / ____| |             | |               / __ \_   _|
 | (___ | |__   __ _  __| | _____      __| |  | || |
  \___ \| '_ \ / _` |/ _` |/ _ \ \ /\ / /| |  | || |
  ____) | | | | (_| | (_| | (_) \ V  V / | |__| || |_
 |_____/|_| |_|\__,_|\__,_|\___/ \_/\_/   \____/_____|

    Modern Open Tibia Client - v)" << SHADOW_VERSION << R"(
    Platform: )" << SHADOW_PLATFORM << R"(
)" << std::endl;
}

bool initializeFramework() {
    // Initialize core systems - graphics needs the window from app
    if (!g_graphics.init()) {
        std::cerr << "Failed to initialize graphics" << std::endl;
        return false;
    }

    // Pass the window to graphics for buffer swapping
    g_graphics.setWindow(g_app.getWindow());

    // Set up viewport and projection
    int width = g_app.getWindowWidth();
    int height = g_app.getWindowHeight();
    g_graphics.setViewport(0, 0, width, height);
    g_graphics.setOrtho(width, height);

    if (!g_lua.init()) {
        std::cerr << "Failed to initialize Lua engine" << std::endl;
        return false;
    }

    // Initialize Shadow OT extensions
    shadow::realms::RealmManager::instance();
    shadow::blockchain::Wallet::instance();

    return true;
}

bool loadModules() {
    // Load core modules from the modules directory
    // These will be loaded if they exist
    g_lua.loadModules("modules");
    return true;
}

#ifdef SHADOW_PLATFORM_WEB
void webMainLoop() {
    g_app.poll();
    g_dispatcher.poll();

    g_graphics.beginFrame();
    g_graphics.clear(shadow::framework::Color(16, 24, 48, 255));
    g_graphics.drawFilledRect(
        shadow::framework::Rect(100, 100, 200, 100),
        shadow::framework::Color(64, 128, 192, 255)
    );
    g_graphics.endFrame();
    g_graphics.render();
}
#endif

} // anonymous namespace

int main(int argc, char* argv[]) {
    printBanner();

    // Parse command line arguments
    std::vector<std::string> args(argv, argv + argc);

    // Initialize application
    if (!g_app.init(args)) {
        std::cerr << "Failed to initialize application" << std::endl;
        return 1;
    }

    // Initialize framework
    if (!initializeFramework()) {
        std::cerr << "Failed to initialize framework" << std::endl;
        return 1;
    }

    // Load configuration
    g_configs.load("config.lua");

    // Load modules
    if (!loadModules()) {
        std::cerr << "Failed to load modules" << std::endl;
        return 1;
    }

    std::cout << "Shadow OT Client initialized successfully" << std::endl;

#ifdef SHADOW_PLATFORM_WEB
    // Web platform uses emscripten main loop
    emscripten_set_main_loop(webMainLoop, 0, 1);
#else
    // Desktop main loop
    while (!g_app.shouldClose()) {
        g_app.poll();
        g_dispatcher.poll();

        // Begin frame rendering
        g_graphics.beginFrame();

        // Clear to dark blue background (classic Tibia loading color)
        g_graphics.clear(shadow::framework::Color(16, 24, 48, 255));

        // Draw a visible test rectangle to confirm rendering works
        // This will be replaced by proper UI once modules are loaded
        g_graphics.drawFilledRect(
            shadow::framework::Rect(100, 100, 200, 100),
            shadow::framework::Color(64, 128, 192, 255)
        );

        // End frame and swap buffers
        g_graphics.endFrame();
        g_graphics.render();
    }
#endif

    // Cleanup
    g_lua.terminate();
    g_graphics.terminate();
    g_app.terminate();

    return 0;
}
