/**
 * Shadow OT Client - Configuration Manager Implementation
 */

#include "configmanager.h"
#include <fstream>
#include <sstream>
#include <algorithm>

namespace shadow {
namespace framework {

ConfigManager& ConfigManager::instance() {
    static ConfigManager instance;
    return instance;
}

bool ConfigManager::load(const std::string& filename) {
    std::ifstream file(filename);
    if (!file.is_open()) {
        return false;
    }

    m_loadedFile = filename;
    m_values.clear();

    std::string line;
    while (std::getline(file, line)) {
        // Skip comments and empty lines
        if (line.empty() || line[0] == '#' || line[0] == '-') {
            continue;
        }

        // Find key=value separator
        auto pos = line.find('=');
        if (pos == std::string::npos) {
            pos = line.find(':');
        }
        if (pos == std::string::npos) {
            continue;
        }

        std::string key = line.substr(0, pos);
        std::string value = line.substr(pos + 1);

        // Trim whitespace
        auto trimStart = key.find_first_not_of(" \t");
        auto trimEnd = key.find_last_not_of(" \t");
        if (trimStart != std::string::npos) {
            key = key.substr(trimStart, trimEnd - trimStart + 1);
        }

        trimStart = value.find_first_not_of(" \t\"'");
        trimEnd = value.find_last_not_of(" \t\"'");
        if (trimStart != std::string::npos) {
            value = value.substr(trimStart, trimEnd - trimStart + 1);
        }

        // Parse value type
        if (value == "true" || value == "yes" || value == "on") {
            m_values[key] = true;
        } else if (value == "false" || value == "no" || value == "off") {
            m_values[key] = false;
        } else {
            // Try to parse as number
            try {
                if (value.find('.') != std::string::npos) {
                    m_values[key] = std::stod(value);
                } else {
                    m_values[key] = std::stoi(value);
                }
            } catch (...) {
                m_values[key] = value;
            }
        }
    }

    return true;
}

bool ConfigManager::save(const std::string& filename) {
    std::string saveFile = filename.empty() ? m_loadedFile : filename;
    if (saveFile.empty()) {
        return false;
    }

    std::ofstream file(saveFile);
    if (!file.is_open()) {
        return false;
    }

    file << "# Shadow OT Configuration\n\n";

    for (const auto& [key, value] : m_values) {
        file << key << " = ";

        std::visit([&file](auto&& v) {
            using T = std::decay_t<decltype(v)>;
            if constexpr (std::is_same_v<T, bool>) {
                file << (v ? "true" : "false");
            } else if constexpr (std::is_same_v<T, int>) {
                file << v;
            } else if constexpr (std::is_same_v<T, double>) {
                file << v;
            } else if constexpr (std::is_same_v<T, std::string>) {
                file << "\"" << v << "\"";
            }
        }, value);

        file << "\n";
    }

    return true;
}

void ConfigManager::clear() {
    m_values.clear();
}

bool ConfigManager::getBool(const std::string& key, bool defaultValue) const {
    auto it = m_values.find(key);
    if (it == m_values.end()) {
        return defaultValue;
    }

    if (auto* v = std::get_if<bool>(&it->second)) {
        return *v;
    }
    return defaultValue;
}

int ConfigManager::getInt(const std::string& key, int defaultValue) const {
    auto it = m_values.find(key);
    if (it == m_values.end()) {
        return defaultValue;
    }

    if (auto* v = std::get_if<int>(&it->second)) {
        return *v;
    }
    if (auto* v = std::get_if<double>(&it->second)) {
        return static_cast<int>(*v);
    }
    return defaultValue;
}

double ConfigManager::getDouble(const std::string& key, double defaultValue) const {
    auto it = m_values.find(key);
    if (it == m_values.end()) {
        return defaultValue;
    }

    if (auto* v = std::get_if<double>(&it->second)) {
        return *v;
    }
    if (auto* v = std::get_if<int>(&it->second)) {
        return static_cast<double>(*v);
    }
    return defaultValue;
}

std::string ConfigManager::getString(const std::string& key, const std::string& defaultValue) const {
    auto it = m_values.find(key);
    if (it == m_values.end()) {
        return defaultValue;
    }

    if (auto* v = std::get_if<std::string>(&it->second)) {
        return *v;
    }
    return defaultValue;
}

void ConfigManager::setBool(const std::string& key, bool value) {
    m_values[key] = value;
}

void ConfigManager::setInt(const std::string& key, int value) {
    m_values[key] = value;
}

void ConfigManager::setDouble(const std::string& key, double value) {
    m_values[key] = value;
}

void ConfigManager::setString(const std::string& key, const std::string& value) {
    m_values[key] = value;
}

bool ConfigManager::hasKey(const std::string& key) const {
    return m_values.find(key) != m_values.end();
}

void ConfigManager::remove(const std::string& key) {
    m_values.erase(key);
}

std::map<std::string, ConfigManager::ConfigValue> ConfigManager::getSection(const std::string& prefix) const {
    std::map<std::string, ConfigValue> result;
    std::string searchPrefix = prefix + ".";

    for (const auto& [key, value] : m_values) {
        if (key.find(searchPrefix) == 0) {
            result[key.substr(searchPrefix.length())] = value;
        }
    }

    return result;
}

void ConfigManager::setSection(const std::string& prefix, const std::map<std::string, ConfigValue>& values) {
    for (const auto& [key, value] : values) {
        m_values[prefix + "." + key] = value;
    }
}

} // namespace framework
} // namespace shadow

// Global instance
shadow::framework::ConfigManager& g_configs = shadow::framework::ConfigManager::instance();
