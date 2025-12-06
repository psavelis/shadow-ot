/**
 * Shadow OT Client - Platform Abstraction
 *
 * Cross-platform utilities and system functions.
 */

#pragma once

#include <string>
#include <vector>
#include <cstdint>
#include <functional>

// Use 'shadowot' instead of 'shadow' to avoid macOS MacTypes.h 'shadow' enum conflict
namespace shadowot {
namespace framework {

class Platform {
public:
    static Platform& instance();

    // System info
    std::string getOSName() const;
    std::string getOSVersion() const;
    std::string getCPUName() const;
    int getCPUCores() const;
    uint64_t getTotalMemory() const;
    uint64_t getAvailableMemory() const;
    std::string getGPUName() const;

    // Paths
    std::string getAppDataPath() const;
    std::string getDocumentsPath() const;
    std::string getTempPath() const;
    std::string getExecutablePath() const;
    std::string getCurrentDirectory() const;

    // Window management
    void* createWindow(const std::string& title, int width, int height, bool fullscreen = false);
    void destroyWindow(void* window);
    void setWindowTitle(void* window, const std::string& title);
    void setWindowSize(void* window, int width, int height);
    void setWindowPosition(void* window, int x, int y);
    void setWindowFullscreen(void* window, bool fullscreen);
    bool isWindowFocused(void* window) const;
    bool isWindowMinimized(void* window) const;

    // Clipboard
    std::string getClipboardText() const;
    void setClipboardText(const std::string& text);

    // External operations
    void openUrl(const std::string& url);
    void openFile(const std::string& path);
    void showMessage(const std::string& title, const std::string& message, bool isError = false);
    bool showConfirm(const std::string& title, const std::string& message);

    // File dialogs
    std::string showOpenFileDialog(const std::string& title,
                                   const std::vector<std::string>& filters = {});
    std::string showSaveFileDialog(const std::string& title,
                                   const std::string& defaultName = "",
                                   const std::vector<std::string>& filters = {});
    std::string showFolderDialog(const std::string& title);

    // Time
    double getTime() const; // High-resolution time in seconds
    uint64_t getMilliseconds() const;
    uint64_t getMicroseconds() const;
    void sleep(uint32_t milliseconds);

    // Threading
    int getThreadId() const;
    void setThreadPriority(int priority);

    // Input
    bool isKeyPressed(int keyCode) const;
    bool isMouseButtonPressed(int button) const;
    void getMousePosition(int& x, int& y) const;
    void setMousePosition(int x, int y);
    void setCursorVisible(bool visible);
    void setCursorShape(int shape);

    // Gamepad
    int getConnectedGamepads() const;
    bool isGamepadButtonPressed(int gamepad, int button) const;
    float getGamepadAxis(int gamepad, int axis) const;

    // Network
    std::string getLocalIPAddress() const;
    std::string getPublicIPAddress() const;
    bool isNetworkAvailable() const;

    // Power
    int getBatteryLevel() const; // -1 if no battery
    bool isCharging() const;

    // Callbacks
    using QuitCallback = std::function<bool()>;
    using FocusCallback = std::function<void(bool)>;
    using ResizeCallback = std::function<void(int, int)>;
    using FileDropCallback = std::function<void(const std::vector<std::string>&)>;

    void setQuitCallback(QuitCallback callback);
    void setFocusCallback(FocusCallback callback);
    void setResizeCallback(ResizeCallback callback);
    void setFileDropCallback(FileDropCallback callback);

    // Event processing
    void pollEvents();

private:
    Platform() = default;
    ~Platform() = default;
    Platform(const Platform&) = delete;
    Platform& operator=(const Platform&) = delete;

    struct Impl;
    std::unique_ptr<Impl> m_impl;
};

} // namespace framework
} // namespace shadowot

// Global accessor
extern shadowot::framework::Platform& g_platform;
