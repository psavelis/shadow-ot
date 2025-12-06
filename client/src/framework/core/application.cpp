/**
 * Shadow OT Client - Application Implementation
 */

#include "application.h"
#include <framework/graphics/graphics.h>
#include <framework/platform/platform.h>
#include <chrono>
#include <thread>

// Platform is in shadowot namespace to avoid macOS MacTypes.h conflicts
// g_platform is in global namespace
using ::g_platform;

#ifdef SHADOW_PLATFORM_WEB
#include <emscripten.h>
#endif

namespace shadow {
namespace framework {

struct Application::Impl {
    void* window{nullptr};
    std::chrono::high_resolution_clock::time_point startTime;
    std::chrono::high_resolution_clock::time_point lastUpdateTime;
    int frameCount{0};
    double fpsAccumulator{0};
};

Application& Application::instance() {
    static Application instance;
    return instance;
}

Application::Application() : m_impl(std::make_unique<Impl>()) {
    m_impl->startTime = std::chrono::high_resolution_clock::now();
    m_impl->lastUpdateTime = m_impl->startTime;
}

Application::~Application() = default;

bool Application::init(const std::vector<std::string>& args) {
    m_args = args;

    // Detect platform
#if defined(_WIN32)
    m_platform = "Windows";
#elif defined(__APPLE__)
    m_platform = "macOS";
#elif defined(__linux__)
    m_platform = "Linux";
#elif defined(__EMSCRIPTEN__)
    m_platform = "Web";
#else
    m_platform = "Unknown";
#endif

    initPaths();
    initWindow();

    return true;
}

void Application::terminate() {
    if (m_impl->window) {
        g_platform.destroyWindow(m_impl->window);
        m_impl->window = nullptr;
    }
}

void Application::initPaths() {
    m_dataPath = g_platform.getExecutablePath() + "/data";
    m_userPath = g_platform.getAppDataPath() + "/ShadowOT";

    // Create user directory if needed
    if (!g_platform.getCurrentDirectory().empty()) {
        // Directory operations handled by platform
    }
}

void* Application::getWindow() const {
    return m_impl ? m_impl->window : nullptr;
}

void Application::initWindow() {
    m_impl->window = g_platform.createWindow("Shadow OT", m_windowWidth, m_windowHeight, m_fullscreen);

    g_platform.setResizeCallback([this](int w, int h) {
        m_windowWidth = w;
        m_windowHeight = h;
        if (m_resizeCallback) {
            m_resizeCallback(w, h);
        }
    });

    g_platform.setFocusCallback([this](bool focused) {
        if (m_focusCallback) {
            m_focusCallback(focused);
        }
    });

    g_platform.setQuitCallback([this]() -> bool {
        m_shouldClose = true;
        return true;
    });
}

void Application::poll() {
    processEvents();
    updateTiming();
}

void Application::processEvents() {
    g_platform.pollEvents();
}

void Application::updateTiming() {
    auto now = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> elapsed = now - m_impl->lastUpdateTime;
    m_deltaTime = elapsed.count();
    m_impl->lastUpdateTime = now;

    // Calculate FPS
    m_impl->fpsAccumulator += m_deltaTime;
    m_impl->frameCount++;

    if (m_impl->fpsAccumulator >= 1.0) {
        m_fps = m_impl->frameCount;
        m_impl->frameCount = 0;
        m_impl->fpsAccumulator = 0;
    }

    // Frame time since start
    std::chrono::duration<double> totalElapsed = now - m_impl->startTime;
    m_frameTime = totalElapsed.count();

    // Frame limiting
    if (m_targetFPS > 0) {
        double targetFrameTime = 1.0 / m_targetFPS;
        if (m_deltaTime < targetFrameTime) {
            double sleepTime = targetFrameTime - m_deltaTime;
            std::this_thread::sleep_for(std::chrono::duration<double>(sleepTime));
        }
    }
}

void Application::setWindowTitle(const std::string& title) {
    if (m_impl->window) {
        g_platform.setWindowTitle(m_impl->window, title);
    }
}

void Application::setWindowSize(int width, int height) {
    m_windowWidth = width;
    m_windowHeight = height;
    if (m_impl->window) {
        g_platform.setWindowSize(m_impl->window, width, height);
    }
}

void Application::setFullscreen(bool fullscreen) {
    m_fullscreen = fullscreen;
    if (m_impl->window) {
        g_platform.setWindowFullscreen(m_impl->window, fullscreen);
    }
}

bool Application::hasArg(const std::string& arg) const {
    for (const auto& a : m_args) {
        if (a == arg || a.find(arg + "=") == 0) {
            return true;
        }
    }
    return false;
}

std::string Application::getArgValue(const std::string& arg) const {
    std::string prefix = arg + "=";
    for (const auto& a : m_args) {
        if (a.find(prefix) == 0) {
            return a.substr(prefix.length());
        }
    }
    return "";
}

uint64_t Application::getMilliseconds() const {
    auto now = std::chrono::high_resolution_clock::now();
    return std::chrono::duration_cast<std::chrono::milliseconds>(
        now - m_impl->startTime).count();
}

} // namespace framework
} // namespace shadow

// Global instance
shadow::framework::Application& g_app = shadow::framework::Application::instance();
