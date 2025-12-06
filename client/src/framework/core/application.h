/**
 * Shadow OT Client - Application Core
 *
 * Main application lifecycle management.
 */

#pragma once

#include <string>
#include <vector>
#include <memory>
#include <functional>
#include <atomic>

namespace shadow {
namespace framework {

class Application {
public:
    static Application& instance();

    bool init(const std::vector<std::string>& args);
    void terminate();

    void poll();
    bool shouldClose() const { return m_shouldClose; }
    void requestClose() { m_shouldClose = true; }

    // Window management
    void setWindowTitle(const std::string& title);
    void setWindowSize(int width, int height);
    void setFullscreen(bool fullscreen);
    bool isFullscreen() const { return m_fullscreen; }
    void* getWindow() const;
    int getWindowWidth() const { return m_windowWidth; }
    int getWindowHeight() const { return m_windowHeight; }

    // Frame timing
    double getFrameTime() const { return m_frameTime; }
    double getDeltaTime() const { return m_deltaTime; }
    int getFPS() const { return m_fps; }
    void setTargetFPS(int fps) { m_targetFPS = fps; }
    uint64_t getMilliseconds() const;

    // Application info
    const std::string& getVersion() const { return m_version; }
    const std::string& getPlatform() const { return m_platform; }
    const std::string& getDataPath() const { return m_dataPath; }
    const std::string& getUserPath() const { return m_userPath; }

    // Command line
    bool hasArg(const std::string& arg) const;
    std::string getArgValue(const std::string& arg) const;

    // Event callbacks
    using ResizeCallback = std::function<void(int, int)>;
    using FocusCallback = std::function<void(bool)>;
    void setResizeCallback(ResizeCallback callback) { m_resizeCallback = callback; }
    void setFocusCallback(FocusCallback callback) { m_focusCallback = callback; }

private:
    Application();
    ~Application();
    Application(const Application&) = delete;
    Application& operator=(const Application&) = delete;

    void initPaths();
    void initWindow();
    void processEvents();
    void updateTiming();

    std::atomic<bool> m_shouldClose{false};
    bool m_fullscreen{false};
    int m_windowWidth{1280};
    int m_windowHeight{720};
    int m_targetFPS{60};
    int m_fps{0};
    double m_frameTime{0.0};
    double m_deltaTime{0.0};
    double m_lastFrameTime{0.0};

    std::string m_version{"1.0.0"};
    std::string m_platform;
    std::string m_dataPath;
    std::string m_userPath;
    std::vector<std::string> m_args;

    ResizeCallback m_resizeCallback;
    FocusCallback m_focusCallback;

    struct Impl;
    std::unique_ptr<Impl> m_impl;
};

} // namespace framework
} // namespace shadow

// Global accessor
extern shadow::framework::Application& g_app;
