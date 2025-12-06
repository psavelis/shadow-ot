/**
 * Shadow OT Client - Platform Implementation (GLFW-based)
 */

#include "platform.h"
#include <chrono>
#include <thread>
#include <cstdlib>

#ifdef _WIN32
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <shellapi.h>
#include <shlobj.h>
#elif defined(__APPLE__)
#include <mach-o/dyld.h>
#include <sys/sysctl.h>
// Don't include CoreFoundation to avoid 'shadow' enum conflict
#elif defined(__linux__)
#include <unistd.h>
#include <sys/sysinfo.h>
#include <sys/utsname.h>
#endif

#ifndef _WIN32
#include <unistd.h>  // For getcwd on Unix
#endif

#include <GLFW/glfw3.h>

namespace shadowot {
namespace framework {

struct Platform::Impl {
    GLFWwindow* window{nullptr};
    QuitCallback quitCallback;
    FocusCallback focusCallback;
    ResizeCallback resizeCallback;
    FileDropCallback fileDropCallback;
    bool initialized{false};
    std::chrono::high_resolution_clock::time_point startTime;
};

// Forward declare to avoid private access issues - use raw pointer for callbacks
static void* s_implPtr = nullptr;

// GLFW callbacks
static void glfwErrorCallback(int error, const char* description) {
    // Log error
}

// Callback storage for GLFW (can't access private Impl from static functions)
static std::function<bool()> s_quitCallback;
static std::function<void(bool)> s_focusCallback;
static std::function<void(int, int)> s_resizeCallback;
static std::function<void(const std::vector<std::string>&)> s_fileDropCallback;

static void glfwWindowCloseCallback(GLFWwindow* window) {
    if (s_quitCallback) {
        s_quitCallback();
    }
}

static void glfwWindowFocusCallback(GLFWwindow* window, int focused) {
    if (s_focusCallback) {
        s_focusCallback(focused == GLFW_TRUE);
    }
}

static void glfwWindowSizeCallback(GLFWwindow* window, int width, int height) {
    if (s_resizeCallback) {
        s_resizeCallback(width, height);
    }
}

static void glfwDropCallback(GLFWwindow* window, int count, const char** paths) {
    if (s_fileDropCallback) {
        std::vector<std::string> files;
        for (int i = 0; i < count; ++i) {
            files.push_back(paths[i]);
        }
        s_fileDropCallback(files);
    }
}

Platform& Platform::instance() {
    static Platform instance;
    return instance;
}

std::string Platform::getOSName() const {
#ifdef _WIN32
    return "Windows";
#elif defined(__APPLE__)
    return "macOS";
#elif defined(__linux__)
    return "Linux";
#elif defined(__EMSCRIPTEN__)
    return "Web";
#else
    return "Unknown";
#endif
}

std::string Platform::getOSVersion() const {
#ifdef _WIN32
    OSVERSIONINFOW info;
    ZeroMemory(&info, sizeof(info));
    info.dwOSVersionInfoSize = sizeof(info);
    return std::to_string(info.dwMajorVersion) + "." + std::to_string(info.dwMinorVersion);
#elif defined(__APPLE__)
    char str[256];
    size_t size = sizeof(str);
    sysctlbyname("kern.osrelease", str, &size, nullptr, 0);
    return str;
#elif defined(__linux__)
    struct utsname uts;
    if (uname(&uts) == 0) {
        return uts.release;
    }
    return "Unknown";
#else
    return "Unknown";
#endif
}

std::string Platform::getCPUName() const {
#ifdef _WIN32
    char cpuBrand[0x40];
    int cpuInfo[4] = {0};
    __cpuid(cpuInfo, 0x80000002);
    memcpy(cpuBrand, cpuInfo, sizeof(cpuInfo));
    __cpuid(cpuInfo, 0x80000003);
    memcpy(cpuBrand + 16, cpuInfo, sizeof(cpuInfo));
    __cpuid(cpuInfo, 0x80000004);
    memcpy(cpuBrand + 32, cpuInfo, sizeof(cpuInfo));
    return cpuBrand;
#elif defined(__APPLE__)
    char str[256];
    size_t size = sizeof(str);
    sysctlbyname("machdep.cpu.brand_string", str, &size, nullptr, 0);
    return str;
#else
    return "Unknown";
#endif
}

int Platform::getCPUCores() const {
    return std::thread::hardware_concurrency();
}

uint64_t Platform::getTotalMemory() const {
#ifdef _WIN32
    MEMORYSTATUSEX status;
    status.dwLength = sizeof(status);
    GlobalMemoryStatusEx(&status);
    return status.ullTotalPhys;
#elif defined(__APPLE__)
    int64_t mem = 0;
    size_t size = sizeof(mem);
    sysctlbyname("hw.memsize", &mem, &size, nullptr, 0);
    return static_cast<uint64_t>(mem);
#elif defined(__linux__)
    struct sysinfo info;
    sysinfo(&info);
    return info.totalram * info.mem_unit;
#else
    return 0;
#endif
}

uint64_t Platform::getAvailableMemory() const {
#ifdef _WIN32
    MEMORYSTATUSEX status;
    status.dwLength = sizeof(status);
    GlobalMemoryStatusEx(&status);
    return status.ullAvailPhys;
#elif defined(__linux__)
    struct sysinfo info;
    sysinfo(&info);
    return info.freeram * info.mem_unit;
#else
    return getTotalMemory() / 2; // Estimate
#endif
}

std::string Platform::getGPUName() const {
    return "Unknown"; // Would be obtained from OpenGL context
}

std::string Platform::getAppDataPath() const {
#ifdef _WIN32
    char path[MAX_PATH];
    SHGetFolderPathA(nullptr, CSIDL_APPDATA, nullptr, 0, path);
    return path;
#elif defined(__APPLE__)
    const char* home = getenv("HOME");
    return std::string(home) + "/Library/Application Support";
#else
    const char* home = getenv("HOME");
    const char* xdg = getenv("XDG_DATA_HOME");
    if (xdg) return xdg;
    return std::string(home) + "/.local/share";
#endif
}

std::string Platform::getDocumentsPath() const {
#ifdef _WIN32
    char path[MAX_PATH];
    SHGetFolderPathA(nullptr, CSIDL_PERSONAL, nullptr, 0, path);
    return path;
#else
    const char* home = getenv("HOME");
    return std::string(home) + "/Documents";
#endif
}

std::string Platform::getTempPath() const {
#ifdef _WIN32
    char path[MAX_PATH];
    GetTempPathA(MAX_PATH, path);
    return path;
#else
    return "/tmp";
#endif
}

std::string Platform::getExecutablePath() const {
#ifdef _WIN32
    char path[MAX_PATH];
    GetModuleFileNameA(nullptr, path, MAX_PATH);
    std::string str(path);
    return str.substr(0, str.find_last_of("\\/"));
#elif defined(__APPLE__)
    char path[1024];
    uint32_t size = sizeof(path);
    _NSGetExecutablePath(path, &size);
    std::string str(path);
    return str.substr(0, str.find_last_of("/"));
#else
    char path[1024];
    ssize_t len = readlink("/proc/self/exe", path, sizeof(path) - 1);
    if (len != -1) {
        path[len] = '\0';
        std::string str(path);
        return str.substr(0, str.find_last_of("/"));
    }
    return ".";
#endif
}

std::string Platform::getCurrentDirectory() const {
    char buffer[1024];
#ifdef _WIN32
    GetCurrentDirectoryA(sizeof(buffer), buffer);
#else
    if (getcwd(buffer, sizeof(buffer)) == nullptr) {
        return ".";
    }
#endif
    return buffer;
}

void* Platform::createWindow(const std::string& title, int width, int height, bool fullscreen) {
    if (!m_impl) {
        m_impl = std::make_unique<Impl>();
        m_impl->startTime = std::chrono::high_resolution_clock::now();
    }

    if (!m_impl->initialized) {
        glfwSetErrorCallback(glfwErrorCallback);
        if (!glfwInit()) {
            return nullptr;
        }
        m_impl->initialized = true;
    }

    // OpenGL 3.3 Core
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
#ifdef __APPLE__
    glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GL_TRUE);
#endif
    glfwWindowHint(GLFW_RESIZABLE, GLFW_TRUE);

    GLFWmonitor* monitor = fullscreen ? glfwGetPrimaryMonitor() : nullptr;
    GLFWwindow* window = glfwCreateWindow(width, height, title.c_str(), monitor, nullptr);

    if (window) {
        glfwMakeContextCurrent(window);
        glfwSwapInterval(1); // VSync

        // Set callbacks
        glfwSetWindowCloseCallback(window, glfwWindowCloseCallback);
        glfwSetWindowFocusCallback(window, glfwWindowFocusCallback);
        glfwSetWindowSizeCallback(window, glfwWindowSizeCallback);
        glfwSetDropCallback(window, glfwDropCallback);

        m_impl->window = window;
    }

    return window;
}

void Platform::destroyWindow(void* window) {
    if (window) {
        glfwDestroyWindow(static_cast<GLFWwindow*>(window));
    }
    if (m_impl && m_impl->initialized) {
        glfwTerminate();
        m_impl->initialized = false;
    }
}

void Platform::setWindowTitle(void* window, const std::string& title) {
    glfwSetWindowTitle(static_cast<GLFWwindow*>(window), title.c_str());
}

void Platform::setWindowSize(void* window, int width, int height) {
    glfwSetWindowSize(static_cast<GLFWwindow*>(window), width, height);
}

void Platform::setWindowPosition(void* window, int x, int y) {
    glfwSetWindowPos(static_cast<GLFWwindow*>(window), x, y);
}

void Platform::setWindowFullscreen(void* window, bool fullscreen) {
    GLFWwindow* win = static_cast<GLFWwindow*>(window);
    if (fullscreen) {
        GLFWmonitor* monitor = glfwGetPrimaryMonitor();
        const GLFWvidmode* mode = glfwGetVideoMode(monitor);
        glfwSetWindowMonitor(win, monitor, 0, 0, mode->width, mode->height, mode->refreshRate);
    } else {
        glfwSetWindowMonitor(win, nullptr, 100, 100, 1280, 720, 0);
    }
}

bool Platform::isWindowFocused(void* window) const {
    return glfwGetWindowAttrib(static_cast<GLFWwindow*>(window), GLFW_FOCUSED) != 0;
}

bool Platform::isWindowMinimized(void* window) const {
    return glfwGetWindowAttrib(static_cast<GLFWwindow*>(window), GLFW_ICONIFIED) != 0;
}

std::string Platform::getClipboardText() const {
    if (m_impl && m_impl->window) {
        const char* text = glfwGetClipboardString(m_impl->window);
        return text ? text : "";
    }
    return "";
}

void Platform::setClipboardText(const std::string& text) {
    if (m_impl && m_impl->window) {
        glfwSetClipboardString(m_impl->window, text.c_str());
    }
}

void Platform::openUrl(const std::string& url) {
#ifdef _WIN32
    ShellExecuteA(nullptr, "open", url.c_str(), nullptr, nullptr, SW_SHOWNORMAL);
#elif defined(__APPLE__)
    std::string cmd = "open \"" + url + "\"";
    system(cmd.c_str());
#else
    std::string cmd = "xdg-open \"" + url + "\"";
    system(cmd.c_str());
#endif
}

void Platform::openFile(const std::string& path) {
    openUrl(path);
}

void Platform::showMessage(const std::string& title, const std::string& message, bool isError) {
    // GLFW doesn't have message boxes - use system calls
#ifdef _WIN32
    MessageBoxA(nullptr, message.c_str(), title.c_str(),
                isError ? MB_ICONERROR : MB_ICONINFORMATION);
#elif defined(__APPLE__)
    std::string script = "osascript -e 'display dialog \"" + message +
                         "\" with title \"" + title + "\" buttons {\"OK\"}'";
    system(script.c_str());
#else
    std::string cmd = "zenity --" + std::string(isError ? "error" : "info") +
                      " --title=\"" + title + "\" --text=\"" + message + "\"";
    system(cmd.c_str());
#endif
}

bool Platform::showConfirm(const std::string& title, const std::string& message) {
#ifdef _WIN32
    return MessageBoxA(nullptr, message.c_str(), title.c_str(),
                       MB_YESNO | MB_ICONQUESTION) == IDYES;
#elif defined(__APPLE__)
    std::string script = "osascript -e 'display dialog \"" + message +
                         "\" with title \"" + title + "\" buttons {\"No\", \"Yes\"}'";
    return system(script.c_str()) == 0;
#else
    std::string cmd = "zenity --question --title=\"" + title +
                      "\" --text=\"" + message + "\"";
    return system(cmd.c_str()) == 0;
#endif
}

double Platform::getTime() const {
    return glfwGetTime();
}

uint64_t Platform::getMilliseconds() const {
    if (m_impl) {
        auto now = std::chrono::high_resolution_clock::now();
        return std::chrono::duration_cast<std::chrono::milliseconds>(
            now - m_impl->startTime).count();
    }
    return static_cast<uint64_t>(glfwGetTime() * 1000.0);
}

uint64_t Platform::getMicroseconds() const {
    return std::chrono::duration_cast<std::chrono::microseconds>(
        std::chrono::high_resolution_clock::now().time_since_epoch()
    ).count();
}

void Platform::sleep(uint32_t milliseconds) {
    std::this_thread::sleep_for(std::chrono::milliseconds(milliseconds));
}

bool Platform::isKeyPressed(int keyCode) const {
    if (m_impl && m_impl->window) {
        return glfwGetKey(m_impl->window, keyCode) == GLFW_PRESS;
    }
    return false;
}

bool Platform::isMouseButtonPressed(int button) const {
    if (m_impl && m_impl->window) {
        return glfwGetMouseButton(m_impl->window, button) == GLFW_PRESS;
    }
    return false;
}

void Platform::getMousePosition(int& x, int& y) const {
    if (m_impl && m_impl->window) {
        double xpos, ypos;
        glfwGetCursorPos(m_impl->window, &xpos, &ypos);
        x = static_cast<int>(xpos);
        y = static_cast<int>(ypos);
    } else {
        x = y = 0;
    }
}

void Platform::setMousePosition(int x, int y) {
    if (m_impl && m_impl->window) {
        glfwSetCursorPos(m_impl->window, x, y);
    }
}

void Platform::setCursorVisible(bool visible) {
    if (m_impl && m_impl->window) {
        glfwSetInputMode(m_impl->window, GLFW_CURSOR,
                         visible ? GLFW_CURSOR_NORMAL : GLFW_CURSOR_HIDDEN);
    }
}

void Platform::setQuitCallback(QuitCallback callback) {
    s_quitCallback = callback;
}

void Platform::setFocusCallback(FocusCallback callback) {
    s_focusCallback = callback;
}

void Platform::setResizeCallback(ResizeCallback callback) {
    s_resizeCallback = callback;
}

void Platform::setFileDropCallback(FileDropCallback callback) {
    s_fileDropCallback = callback;
}

void Platform::pollEvents() {
    glfwPollEvents();
}

} // namespace framework
} // namespace shadowot

// Global instance - use shadowot namespace to avoid macOS 'shadow' conflict
shadowot::framework::Platform& g_platform = shadowot::framework::Platform::instance();
