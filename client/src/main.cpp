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
#include <framework/ui/uimanager.h>

#include <shadow/realms/realmmanager.h>
#include <shadow/blockchain/wallet.h>
#include <client/game.h>

#include <iostream>
#include <string>

// Use framework globals
using shadow::framework::g_graphics;
using shadow::framework::Color;
using shadow::framework::Rect;

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
    // Desktop main loop with proper login screen
    int windowWidth = g_app.getWindowWidth();
    int windowHeight = g_app.getWindowHeight();

    while (!g_app.shouldClose()) {
        g_app.poll();
        g_dispatcher.poll();

        // Begin frame rendering
        g_graphics.beginFrame();

        // Clear to Tibia classic dark blue gradient
        g_graphics.clear(Color(16, 24, 48, 255));

        // Draw background decorative elements
        // Top gradient bar
        g_graphics.drawFilledRect(Rect(0, 0, windowWidth, 60), Color(8, 16, 32, 255));
        g_graphics.drawFilledRect(Rect(0, 58, windowWidth, 2), Color(64, 96, 128, 255));

        // Shadow OT Logo area (center top)
        int logoX = (windowWidth - 400) / 2;
        g_graphics.drawFilledRect(Rect(logoX, 80, 400, 80), Color(24, 40, 64, 255));
        g_graphics.drawRect(Rect(logoX, 80, 400, 80), Color(64, 96, 128, 255));

        // Login panel (centered)
        int panelWidth = 350;
        int panelHeight = 280;
        int panelX = (windowWidth - panelWidth) / 2;
        int panelY = 200;

        // Panel background
        g_graphics.drawFilledRect(Rect(panelX, panelY, panelWidth, panelHeight), Color(32, 48, 72, 230));
        g_graphics.drawRect(Rect(panelX, panelY, panelWidth, panelHeight), Color(80, 120, 160, 255));

        // Panel header
        g_graphics.drawFilledRect(Rect(panelX, panelY, panelWidth, 35), Color(48, 72, 96, 255));
        g_graphics.drawFilledRect(Rect(panelX, panelY + 33, panelWidth, 2), Color(64, 96, 128, 255));

        // Account field area
        int fieldX = panelX + 25;
        int fieldY = panelY + 60;
        int fieldW = panelWidth - 50;
        int fieldH = 30;
        g_graphics.drawFilledRect(Rect(fieldX, fieldY, fieldW, fieldH), Color(16, 24, 40, 255));
        g_graphics.drawRect(Rect(fieldX, fieldY, fieldW, fieldH), Color(64, 96, 128, 255));

        // Password field area
        fieldY += 55;
        g_graphics.drawFilledRect(Rect(fieldX, fieldY, fieldW, fieldH), Color(16, 24, 40, 255));
        g_graphics.drawRect(Rect(fieldX, fieldY, fieldW, fieldH), Color(64, 96, 128, 255));

        // Server field area
        fieldY += 55;
        g_graphics.drawFilledRect(Rect(fieldX, fieldY, fieldW, fieldH), Color(16, 24, 40, 255));
        g_graphics.drawRect(Rect(fieldX, fieldY, fieldW, fieldH), Color(64, 96, 128, 255));

        // Login button
        int btnX = panelX + 75;
        int btnY = panelY + panelHeight - 50;
        int btnW = 100;
        int btnH = 32;
        g_graphics.drawFilledRect(Rect(btnX, btnY, btnW, btnH), Color(32, 96, 64, 255));
        g_graphics.drawRect(Rect(btnX, btnY, btnW, btnH), Color(64, 160, 96, 255));

        // Exit button
        btnX = panelX + panelWidth - 175;
        g_graphics.drawFilledRect(Rect(btnX, btnY, btnW, btnH), Color(96, 32, 32, 255));
        g_graphics.drawRect(Rect(btnX, btnY, btnW, btnH), Color(160, 64, 64, 255));

        // Bottom status bar
        g_graphics.drawFilledRect(Rect(0, windowHeight - 30, windowWidth, 30), Color(8, 16, 32, 255));
        g_graphics.drawFilledRect(Rect(0, windowHeight - 30, windowWidth, 2), Color(64, 96, 128, 255));

        // Version info area (bottom right)
        g_graphics.drawFilledRect(Rect(windowWidth - 200, windowHeight - 25, 190, 20), Color(16, 24, 40, 128));

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
