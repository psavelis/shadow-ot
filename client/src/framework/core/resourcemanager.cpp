/**
 * Shadow OT Client - Resource Manager Implementation
 */

#include "resourcemanager.h"
#include <framework/graphics/graphics.h>
#include <fstream>
#include <sstream>
#include <algorithm>
#include <filesystem>

namespace fs = std::filesystem;

namespace shadow {
namespace framework {

ResourceManager& ResourceManager::instance() {
    static ResourceManager instance;
    return instance;
}

bool ResourceManager::init() {
    // Add default search paths
    addSearchPath("data");
    addSearchPath(".");
    return true;
}

void ResourceManager::terminate() {
    clearCache();
    m_searchPaths.clear();
}

void ResourceManager::addSearchPath(const std::string& path) {
    if (std::find(m_searchPaths.begin(), m_searchPaths.end(), path) == m_searchPaths.end()) {
        m_searchPaths.push_back(path);
    }
}

void ResourceManager::removeSearchPath(const std::string& path) {
    m_searchPaths.erase(
        std::remove(m_searchPaths.begin(), m_searchPaths.end(), path),
        m_searchPaths.end()
    );
}

std::string ResourceManager::resolvePath(const std::string& filename) const {
    // Check if absolute path
    if (fs::path(filename).is_absolute()) {
        if (fs::exists(filename)) {
            return filename;
        }
        return "";
    }

    // Search in registered paths
    for (const auto& searchPath : m_searchPaths) {
        fs::path fullPath = fs::path(searchPath) / filename;
        if (fs::exists(fullPath)) {
            return fullPath.string();
        }
    }

    return "";
}

bool ResourceManager::fileExists(const std::string& filename) const {
    return !resolvePath(filename).empty();
}

std::vector<uint8_t> ResourceManager::readFile(const std::string& filename) const {
    std::string path = resolvePath(filename);
    if (path.empty()) {
        return {};
    }

    std::ifstream file(path, std::ios::binary | std::ios::ate);
    if (!file.is_open()) {
        return {};
    }

    std::streamsize size = file.tellg();
    file.seekg(0, std::ios::beg);

    std::vector<uint8_t> buffer(size);
    if (!file.read(reinterpret_cast<char*>(buffer.data()), size)) {
        return {};
    }

    return buffer;
}

std::string ResourceManager::readFileText(const std::string& filename) const {
    std::string path = resolvePath(filename);
    if (path.empty()) {
        return "";
    }

    std::ifstream file(path);
    if (!file.is_open()) {
        return "";
    }

    std::stringstream buffer;
    buffer << file.rdbuf();
    return buffer.str();
}

bool ResourceManager::writeFile(const std::string& filename, const std::vector<uint8_t>& data) {
    std::ofstream file(filename, std::ios::binary);
    if (!file.is_open()) {
        return false;
    }

    file.write(reinterpret_cast<const char*>(data.data()), data.size());
    return file.good();
}

bool ResourceManager::writeFileText(const std::string& filename, const std::string& text) {
    std::ofstream file(filename);
    if (!file.is_open()) {
        return false;
    }

    file << text;
    return file.good();
}

std::vector<std::string> ResourceManager::listDirectory(const std::string& path) const {
    std::vector<std::string> result;
    std::string resolvedPath = resolvePath(path);

    if (resolvedPath.empty() || !fs::is_directory(resolvedPath)) {
        return result;
    }

    for (const auto& entry : fs::directory_iterator(resolvedPath)) {
        result.push_back(entry.path().filename().string());
    }

    return result;
}

bool ResourceManager::directoryExists(const std::string& path) const {
    std::string resolved = resolvePath(path);
    return !resolved.empty() && fs::is_directory(resolved);
}

bool ResourceManager::createDirectory(const std::string& path) {
    return fs::create_directories(path);
}

std::shared_ptr<Texture> ResourceManager::loadTexture(const std::string& filename) {
    // Check cache
    auto it = m_textures.find(filename);
    if (it != m_textures.end()) {
        return it->second;
    }

    // Load from file
    auto texture = g_graphics.loadTexture(resolvePath(filename));
    if (texture) {
        m_textures[filename] = texture;
    }

    return texture;
}

std::shared_ptr<Texture> ResourceManager::getTexture(const std::string& name) const {
    auto it = m_textures.find(name);
    if (it != m_textures.end()) {
        return it->second;
    }
    return nullptr;
}

void ResourceManager::unloadTexture(const std::string& name) {
    m_textures.erase(name);
}

std::shared_ptr<Sound> ResourceManager::loadSound(const std::string& filename) {
    // Check cache
    auto it = m_sounds.find(filename);
    if (it != m_sounds.end()) {
        return it->second;
    }

    // Sound loading would be implemented with OpenAL
    // For now, return nullptr as placeholder
    return nullptr;
}

std::shared_ptr<Sound> ResourceManager::getSound(const std::string& name) const {
    auto it = m_sounds.find(name);
    if (it != m_sounds.end()) {
        return it->second;
    }
    return nullptr;
}

void ResourceManager::unloadSound(const std::string& name) {
    m_sounds.erase(name);
}

std::shared_ptr<Font> ResourceManager::loadFont(const std::string& filename, int size) {
    std::string key = filename + ":" + std::to_string(size);

    auto it = m_fonts.find(key);
    if (it != m_fonts.end()) {
        return it->second;
    }

    // Font loading would be implemented with FreeType
    return nullptr;
}

std::shared_ptr<Font> ResourceManager::getFont(const std::string& name) const {
    auto it = m_fonts.find(name);
    if (it != m_fonts.end()) {
        return it->second;
    }
    return nullptr;
}

void ResourceManager::unloadFont(const std::string& name) {
    m_fonts.erase(name);
}

void ResourceManager::clearCache() {
    m_textures.clear();
    m_sounds.clear();
    m_fonts.clear();
}

void ResourceManager::preloadResources(const std::vector<std::string>& files) {
    for (const auto& file : files) {
        // Determine type by extension
        fs::path path(file);
        std::string ext = path.extension().string();

        if (ext == ".png" || ext == ".bmp" || ext == ".jpg") {
            loadTexture(file);
        } else if (ext == ".ogg" || ext == ".wav") {
            loadSound(file);
        } else if (ext == ".ttf" || ext == ".otf") {
            loadFont(file, 12);
        }
    }
}

size_t ResourceManager::getCacheSize() const {
    // Estimate cache size (simplified)
    size_t size = 0;

    for (const auto& [name, texture] : m_textures) {
        if (texture) {
            size += texture->getWidth() * texture->getHeight() * 4; // RGBA
        }
    }

    return size;
}

void ResourceManager::setCacheLimit(size_t bytes) {
    m_cacheLimit = bytes;
}

bool ResourceManager::loadAssetPack(const std::string& filename) {
    // Asset pack loading (ZIP/custom format) would be implemented here
    // For now, just add the path
    addSearchPath(filename);
    return true;
}

void ResourceManager::unloadAssetPack(const std::string& filename) {
    removeSearchPath(filename);
}

// Global instance inside namespace
ResourceManager& g_resources = ResourceManager::instance();

} // namespace framework
} // namespace shadow
