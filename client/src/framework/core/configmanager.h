/**
 * Shadow OT Client - Configuration Manager
 *
 * Handles loading and saving of configuration settings.
 */

#pragma once

#include <string>
#include <map>
#include <variant>
#include <optional>

namespace shadow {
namespace framework {

class ConfigManager {
public:
    static ConfigManager& instance();

    using ConfigValue = std::variant<bool, int, double, std::string>;

    // Load/save configuration
    bool load(const std::string& filename);
    bool save(const std::string& filename);
    void clear();

    // Get values
    bool getBool(const std::string& key, bool defaultValue = false) const;
    int getInt(const std::string& key, int defaultValue = 0) const;
    double getDouble(const std::string& key, double defaultValue = 0.0) const;
    std::string getString(const std::string& key, const std::string& defaultValue = "") const;

    template<typename T>
    T get(const std::string& key) const;

    // Set values
    void setBool(const std::string& key, bool value);
    void setInt(const std::string& key, int value);
    void setDouble(const std::string& key, double value);
    void setString(const std::string& key, const std::string& value);

    template<typename T>
    void set(const std::string& key, T value);

    // Check existence
    bool hasKey(const std::string& key) const;
    void remove(const std::string& key);

    // Section management
    std::map<std::string, ConfigValue> getSection(const std::string& prefix) const;
    void setSection(const std::string& prefix, const std::map<std::string, ConfigValue>& values);

private:
    ConfigManager() = default;
    ~ConfigManager() = default;
    ConfigManager(const ConfigManager&) = delete;
    ConfigManager& operator=(const ConfigManager&) = delete;

    std::map<std::string, ConfigValue> m_values;
    std::string m_loadedFile;
};

} // namespace framework
} // namespace shadow

// Global accessor
extern shadow::framework::ConfigManager& g_configs;
