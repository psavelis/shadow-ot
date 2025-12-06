/**
 * Shadow OT Client - Client Core
 *
 * Main client initialization and coordination.
 */

#pragma once

#include <string>
#include <memory>

namespace shadow {
namespace client {

class Client {
public:
    static Client& instance();

    bool init();
    void terminate();

    void poll();
    void render();

    // Version info
    const std::string& getVersion() const { return m_version; }
    const std::string& getBuildDate() const { return m_buildDate; }
    int getProtocolVersion() const { return m_protocolVersion; }

    // Window
    void setWindowTitle(const std::string& title);

    // State
    bool isRunning() const { return m_running; }
    void requestExit() { m_running = false; }

private:
    Client() = default;
    Client(const Client&) = delete;
    Client& operator=(const Client&) = delete;

    bool initGraphics();
    bool initSound();
    bool initLua();
    bool initUI();
    bool loadData();

    bool m_running{true};
    std::string m_version{"1.0.0"};
    std::string m_buildDate{__DATE__};
    int m_protocolVersion{1098};
};

} // namespace client
} // namespace shadow

// Global accessor
extern shadow::client::Client& g_client;

// Version macros
#define SHADOW_VERSION "1.0.0"
#define SHADOW_PLATFORM "Desktop"
