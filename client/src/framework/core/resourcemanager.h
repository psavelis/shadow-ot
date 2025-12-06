/**
 * Shadow OT Client - Resource Manager
 *
 * Handles loading and caching of game resources.
 */

#pragma once

#include <string>
#include <map>
#include <vector>
#include <memory>
#include <functional>

namespace shadow {
namespace framework {

// Forward declarations
class Texture;
class Sound;
class Font;

class ResourceManager {
public:
    static ResourceManager& instance();

    bool init();
    void terminate();

    // Path management
    void addSearchPath(const std::string& path);
    void removeSearchPath(const std::string& path);
    std::string resolvePath(const std::string& filename) const;
    bool fileExists(const std::string& filename) const;

    // File operations
    std::vector<uint8_t> readFile(const std::string& filename) const;
    std::string readFileText(const std::string& filename) const;
    bool writeFile(const std::string& filename, const std::vector<uint8_t>& data);
    bool writeFileText(const std::string& filename, const std::string& text);

    // Directory operations
    std::vector<std::string> listDirectory(const std::string& path) const;
    bool directoryExists(const std::string& path) const;
    bool createDirectory(const std::string& path);

    // Texture loading
    std::shared_ptr<Texture> loadTexture(const std::string& filename);
    std::shared_ptr<Texture> getTexture(const std::string& name) const;
    void unloadTexture(const std::string& name);

    // Sound loading
    std::shared_ptr<Sound> loadSound(const std::string& filename);
    std::shared_ptr<Sound> getSound(const std::string& name) const;
    void unloadSound(const std::string& name);

    // Font loading
    std::shared_ptr<Font> loadFont(const std::string& filename, int size);
    std::shared_ptr<Font> getFont(const std::string& name) const;
    void unloadFont(const std::string& name);

    // Cache management
    void clearCache();
    void preloadResources(const std::vector<std::string>& files);
    size_t getCacheSize() const;
    void setCacheLimit(size_t bytes);

    // Asset pack support
    bool loadAssetPack(const std::string& filename);
    void unloadAssetPack(const std::string& filename);

private:
    ResourceManager() = default;
    ~ResourceManager() = default;
    ResourceManager(const ResourceManager&) = delete;
    ResourceManager& operator=(const ResourceManager&) = delete;

    std::vector<std::string> m_searchPaths;
    std::map<std::string, std::shared_ptr<Texture>> m_textures;
    std::map<std::string, std::shared_ptr<Sound>> m_sounds;
    std::map<std::string, std::shared_ptr<Font>> m_fonts;
    size_t m_cacheLimit{512 * 1024 * 1024}; // 512MB default
};

// Global accessor inside namespace
extern ResourceManager& g_resources;

} // namespace framework
} // namespace shadow
