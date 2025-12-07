/**
 * Shadow OT Client - Thing Type Implementation
 */

#include "thingtype.h"
#include <framework/core/resourcemanager.h>
#include <framework/graphics/graphics.h>
#include <fstream>
#include <cstring>

// Globals are declared in shadow::framework namespace
using shadow::framework::g_graphics;
using shadow::framework::g_resources;

namespace shadow {
namespace client {

ThingType::ThingType() = default;

bool ThingType::load(const uint8_t* data, size_t size) {
    // Parse DAT format for this thing type - complete attribute parsing
    size_t pos = 0;

    // Read flags until we hit 0xFF (end of attributes marker)
    while (pos < size) {
        uint8_t attr = data[pos++];
        if (attr == 0xFF) break;

        ThingAttr thingAttr = static_cast<ThingAttr>(attr);
        m_attrs[thingAttr] = true;

        // Parse attribute data based on type
        switch (static_cast<int>(attr)) {
            case 0: // Ground - has speed
                if (pos + 2 <= size) {
                    m_speed = data[pos] | (data[pos + 1] << 8);
                    pos += 2;
                }
                break;

            case 1: // TopOrder1 - on bottom (no data)
            case 2: // TopOrder2 - on top (no data)
            case 3: // TopOrder3 - above top (no data)
            case 4: // Container (no data)
            case 5: // Stackable (no data)
            case 6: // ForceUse (no data)
            case 7: // MultiUse (no data)
                break;

            case 8: // Writable - has max text length
                if (pos + 2 <= size) {
                    m_maxTextLength = data[pos] | (data[pos + 1] << 8);
                    pos += 2;
                }
                break;

            case 9: // WritableOnce - has max text length
                if (pos + 2 <= size) {
                    m_maxTextLength = data[pos] | (data[pos + 1] << 8);
                    pos += 2;
                }
                break;

            case 10: // FluidContainer (no data)
            case 11: // Splash (no data)
            case 12: // NotWalkable (no data)
            case 13: // NotMoveable (no data)
            case 14: // BlockProjectile (no data)
            case 15: // NotPathable (no data)
            case 16: // NoMoveAnimation (no data)
            case 17: // Pickupable (no data)
            case 18: // Hangable (no data)
            case 19: // HookSouth (no data)
            case 20: // HookEast (no data)
            case 21: // Rotateable (no data)
                break;

            case 22: // Light - has intensity and color
                if (pos + 2 <= size) {
                    m_lightIntensity = data[pos++];
                    m_lightColor = data[pos++];
                }
                break;

            case 23: // DontHide (no data)
            case 24: // Translucent (no data)
                break;

            case 25: // Displacement - has x,y offset
                if (pos + 4 <= size) {
                    m_displacementX = data[pos] | (data[pos + 1] << 8);
                    m_displacementY = data[pos + 2] | (data[pos + 3] << 8);
                    pos += 4;
                }
                break;

            case 26: // Elevation - has height
                if (pos + 2 <= size) {
                    m_elevation = data[pos] | (data[pos + 1] << 8);
                    pos += 2;
                }
                break;

            case 27: // LyingCorpse (no data)
            case 28: // AnimateAlways (no data)
                break;

            case 29: // Minimap - has color
                if (pos + 2 <= size) {
                    m_minimapColor = data[pos] | (data[pos + 1] << 8);
                    pos += 2;
                }
                break;

            case 30: // LensHelp - has value
                if (pos + 2 <= size) {
                    m_lensHelp = data[pos] | (data[pos + 1] << 8);
                    pos += 2;
                }
                break;

            case 31: // FullGround (no data)
            case 32: // IgnoreLook (no data)
                break;

            case 33: // Cloth - has slot
                if (pos + 2 <= size) {
                    m_clothSlot = data[pos] | (data[pos + 1] << 8);
                    pos += 2;
                }
                break;

            case 34: // Market - has category, trade info
                if (pos + 6 <= size) {
                    m_marketCategory = data[pos] | (data[pos + 1] << 8);
                    pos += 2;
                    m_marketTradeAs = data[pos] | (data[pos + 1] << 8);
                    pos += 2;
                    m_marketShowAs = data[pos] | (data[pos + 1] << 8);
                    pos += 2;
                    // Read name string
                    if (pos + 2 <= size) {
                        uint16_t nameLen = data[pos] | (data[pos + 1] << 8);
                        pos += 2;
                        if (pos + nameLen <= size) {
                            m_marketName = std::string(reinterpret_cast<const char*>(&data[pos]), nameLen);
                            pos += nameLen;
                        }
                    }
                    // Skip profession and level
                    if (pos + 4 <= size) {
                        pos += 4; // 2 bytes profession + 2 bytes level
                    }
                }
                break;

            case 35: // DefaultAction - has action type
                if (pos + 2 <= size) {
                    m_defaultAction = data[pos] | (data[pos + 1] << 8);
                    pos += 2;
                }
                break;

            case 36: // Wrappable (no data)
            case 37: // Unwrappable (no data)
            case 38: // TopEffect (no data)
                break;

            case 39: // NPCSaleData (newer clients)
                // Skip 2 bytes
                if (pos + 2 <= size) pos += 2;
                break;

            case 40: // ChangedToExpire (no data) - newer clients
            case 41: // Corpse (no data)
            case 42: // PlayerCorpse (no data)
                break;

            case 43: // CyclopediaItem - has cyclopedia type
                if (pos + 2 <= size) {
                    m_cyclopediaType = data[pos] | (data[pos + 1] << 8);
                    pos += 2;
                }
                break;

            case 44: // Ammo (no data)
            case 45: // ShowOffSocket (no data)
            case 46: // Reportable (no data)
                break;

            case 47: // UpgradeClassification
                if (pos + 1 <= size) {
                    m_upgradeClassification = data[pos++];
                }
                break;

            case 48: // Podium (no data)
            case 49: // Wearout (no data)
            case 50: // ClockExpire (no data)
            case 51: // Expire (no data)
            case 52: // ExpireStop (no data)
                break;

            default:
                // Unknown attribute - try to skip safely
                // Most unknown attributes have no data
                break;
        }
    }

    // Read dimensions
    if (pos < size) m_width = data[pos++];
    if (pos < size) m_height = data[pos++];

    if (m_width > 1 || m_height > 1) {
        if (pos < size) m_exactSize = data[pos++];
    } else {
        m_exactSize = 32;
    }

    if (pos < size) m_layers = data[pos++];
    if (pos < size) m_patternX = data[pos++];
    if (pos < size) m_patternY = data[pos++];
    if (pos < size) m_patternZ = data[pos++];
    if (pos < size) m_animPhases = data[pos++];

    // Read animation info if present (newer formats)
    if (m_animPhases > 1 && pos < size) {
        // Animation type: 0 = async, 1 = sync
        if (pos < size) {
            m_animationType = data[pos++];
        }
        // Loop count (-1 = infinite)
        if (pos + 4 <= size) {
            m_animationLoopCount = static_cast<int32_t>(data[pos] | (data[pos + 1] << 8) |
                                                        (data[pos + 2] << 16) | (data[pos + 3] << 24));
            pos += 4;
        }
        // Start phase
        if (pos < size) {
            m_animationStartPhase = data[pos++];
        }
        // Per-phase timing
        for (int i = 0; i < m_animPhases && pos + 8 <= size; i++) {
            uint32_t minDuration = data[pos] | (data[pos + 1] << 8) |
                                   (data[pos + 2] << 16) | (data[pos + 3] << 24);
            pos += 4;
            uint32_t maxDuration = data[pos] | (data[pos + 1] << 8) |
                                   (data[pos + 2] << 16) | (data[pos + 3] << 24);
            pos += 4;
            m_animationDurations.push_back({minDuration, maxDuration});
        }
    }

    // Read sprite IDs
    int spriteCount = m_width * m_height * m_layers * m_patternX * m_patternY * m_patternZ * m_animPhases;
    m_spriteIds.resize(spriteCount);
    for (int i = 0; i < spriteCount && pos + 4 <= size; i++) {
        m_spriteIds[i] = data[pos] | (data[pos + 1] << 8) |
                         (data[pos + 2] << 16) | (data[pos + 3] << 24);
        pos += 4;
    }

    return true;
}

bool ThingType::hasAttr(ThingAttr attr) const {
    auto it = m_attrs.find(attr);
    return it != m_attrs.end() && it->second;
}

void ThingType::draw(int x, int y, float scale, int patternX, int patternY, int patternZ, int animationPhase) {
    // g_graphics is declared in framework/graphics/graphics.h

    // Calculate sprite index
    int totalPatterns = m_patternX * m_patternY * m_patternZ * m_animPhases * m_layers;

    // Clamp patterns
    patternX = patternX % m_patternX;
    patternY = patternY % m_patternY;
    patternZ = patternZ % m_patternZ;
    animationPhase = animationPhase % m_animPhases;

    // Draw each layer, each tile
    for (int layer = 0; layer < m_layers; layer++) {
        for (int py = 0; py < m_height; py++) {
            for (int px = 0; px < m_width; px++) {
                // Calculate sprite index
                int idx = ((((animationPhase * m_patternZ + patternZ) * m_patternY + patternY) *
                           m_patternX + patternX) * m_layers + layer) * m_width * m_height +
                          py * m_width + px;

                if (idx >= 0 && idx < static_cast<int>(m_sprites.size())) {
                    auto sprite = m_sprites[idx];
                    if (sprite) {
                        int drawX = x - (m_width - 1 - px) * 32 * static_cast<int>(scale);
                        int drawY = y - (m_height - 1 - py) * 32 * static_cast<int>(scale);

                        // Apply displacement
                        drawX -= m_displacementX;
                        drawY -= m_displacementY;

                        framework::Rect destRect{drawX, drawY,
                                                  static_cast<int>(32 * scale),
                                                  static_cast<int>(32 * scale)};
                        g_graphics.drawTexture(sprite.get(), destRect);
                    }
                }
            }
        }
    }
}

void ThingType::setSprite(int index, std::shared_ptr<framework::Texture> sprite) {
    if (index >= static_cast<int>(m_sprites.size())) {
        m_sprites.resize(index + 1);
    }
    m_sprites[index] = sprite;
}

std::shared_ptr<framework::Texture> ThingType::getSprite(int index) const {
    if (index >= 0 && index < static_cast<int>(m_sprites.size())) {
        return m_sprites[index];
    }
    return nullptr;
}

// ThingTypeManager implementation

ThingTypeManager& ThingTypeManager::instance() {
    static ThingTypeManager instance;
    return instance;
}

bool ThingTypeManager::loadDat(const std::string& filename) {
    // g_resources already available from using declaration

    std::vector<uint8_t> data = g_resources.readFile(filename);
    if (data.empty()) {
        return false;
    }

    // DAT file format:
    // 4 bytes: signature
    // 2 bytes: item count
    // 2 bytes: creature count
    // 2 bytes: effect count
    // 2 bytes: missile count
    // Then item data, creature data, etc.

    if (data.size() < 12) return false;

    size_t pos = 0;

    // Signature (not validated here)
    uint32_t signature = data[pos] | (data[pos + 1] << 8) | (data[pos + 2] << 16) | (data[pos + 3] << 24);
    pos += 4;
    (void)signature;

    uint16_t itemCount = data[pos] | (data[pos + 1] << 8);
    pos += 2;

    uint16_t creatureCount = data[pos] | (data[pos + 1] << 8);
    pos += 2;

    uint16_t effectCount = data[pos] | (data[pos + 1] << 8);
    pos += 2;

    uint16_t missileCount = data[pos] | (data[pos + 1] << 8);
    pos += 2;

    // Resize containers
    m_items.resize(itemCount + 1);
    m_creatures.resize(creatureCount + 1);
    m_effects.resize(effectCount + 1);
    m_missiles.resize(missileCount + 1);

    // Load items (ID starts at 100)
    for (uint16_t id = 100; id <= itemCount; id++) {
        auto type = std::make_unique<ThingType>();
        type->setId(id);
        type->setCategory(ThingCategory::Item);

        // Find next 0xFF boundary for this item's data
        size_t start = pos;
        while (pos < data.size() && data[pos] != 0xFF) {
            // Skip attribute data
            uint8_t attr = data[pos++];
            switch (attr) {
                case 0: // Ground - 2 bytes speed
                case 23: // Light - 2 bytes
                case 27: // Elevation - 2 bytes
                case 30: // Minimap - 2 bytes
                    if (pos + 2 <= data.size()) pos += 2;
                    break;
                case 26: // Displaced - 4 bytes
                    if (pos + 4 <= data.size()) pos += 4;
                    break;
                case 35: // Market - variable
                case 36: // Default action - 2 bytes
                    if (pos + 2 <= data.size()) pos += 2;
                    break;
                default:
                    break;
            }
        }
        if (pos < data.size()) pos++; // Skip 0xFF

        // Read dimensions
        if (pos + 1 <= data.size()) {
            // Simplified - just read sprite count info
            // Full implementation would parse all dimension data
            pos += 8; // Skip dimension bytes

            // Read sprite count
            if (pos + 4 <= data.size()) {
                uint32_t spriteCount = data[pos] | (data[pos + 1] << 8) |
                                       (data[pos + 2] << 16) | (data[pos + 3] << 24);
                pos += 4;
                pos += spriteCount * 4; // Skip sprite IDs
            }
        }

        if (id < m_items.size()) {
            m_items[id] = std::move(type);
        }
    }

    // Similar loading for creatures, effects, missiles...
    // Simplified for now

    return true;
}

bool ThingTypeManager::loadSpr(const std::string& filename) {
    // g_resources already available from using declaration

    m_sprData = g_resources.readFile(filename);
    if (m_sprData.empty()) {
        return false;
    }

    // SPR file format:
    // 4 bytes: signature
    // 2/4 bytes: sprite count
    // Then sprite offsets
    // Then sprite data

    if (m_sprData.size() < 8) return false;

    size_t pos = 4; // Skip signature

    // Sprite count (could be 2 or 4 bytes depending on version)
    uint32_t spriteCount = m_sprData[pos] | (m_sprData[pos + 1] << 8);
    if (spriteCount < 0xFFFF) {
        pos += 2;
    } else {
        spriteCount = m_sprData[pos] | (m_sprData[pos + 1] << 8) |
                      (m_sprData[pos + 2] << 16) | (m_sprData[pos + 3] << 24);
        pos += 4;
    }

    // Read offsets
    m_spriteOffsets.resize(spriteCount + 1);
    for (uint32_t i = 1; i <= spriteCount && pos + 4 <= m_sprData.size(); i++) {
        m_spriteOffsets[i] = m_sprData[pos] | (m_sprData[pos + 1] << 8) |
                             (m_sprData[pos + 2] << 16) | (m_sprData[pos + 3] << 24);
        pos += 4;
    }

    return true;
}

ThingType* ThingTypeManager::getItemType(uint16_t id) {
    if (id < m_items.size() && m_items[id]) {
        return m_items[id].get();
    }
    return nullptr;
}

ThingType* ThingTypeManager::getCreatureType(uint16_t id) {
    if (id < m_creatures.size() && m_creatures[id]) {
        return m_creatures[id].get();
    }
    return nullptr;
}

ThingType* ThingTypeManager::getEffectType(uint16_t id) {
    if (id < m_effects.size() && m_effects[id]) {
        return m_effects[id].get();
    }
    return nullptr;
}

ThingType* ThingTypeManager::getMissileType(uint16_t id) {
    if (id < m_missiles.size() && m_missiles[id]) {
        return m_missiles[id].get();
    }
    return nullptr;
}

std::shared_ptr<framework::Texture> ThingTypeManager::loadSprite(uint32_t spriteId) {
    // Check cache
    auto it = m_spriteCache.find(spriteId);
    if (it != m_spriteCache.end()) {
        return it->second;
    }

    if (spriteId == 0 || spriteId >= m_spriteOffsets.size()) {
        return nullptr;
    }

    uint32_t offset = m_spriteOffsets[spriteId];
    if (offset == 0 || offset >= m_sprData.size()) {
        return nullptr;
    }

    // Parse sprite data
    // Format: 3 bytes transparent color, 2 bytes data size, then pixel data
    size_t pos = offset;

    if (pos + 5 > m_sprData.size()) return nullptr;

    // Skip transparent color (3 bytes)
    pos += 3;

    // Pixel data size
    uint16_t pixelDataSize = m_sprData[pos] | (m_sprData[pos + 1] << 8);
    pos += 2;

    if (pos + pixelDataSize > m_sprData.size()) return nullptr;

    // Decode RLE sprite into 32x32 RGBA
    std::vector<uint8_t> pixels(32 * 32 * 4, 0);

    size_t writePos = 0;
    size_t endPos = pos + pixelDataSize;

    while (pos < endPos && writePos < 32 * 32) {
        // Transparent pixels
        uint16_t transparentPixels = m_sprData[pos] | (m_sprData[pos + 1] << 8);
        pos += 2;
        writePos += transparentPixels;

        if (pos >= endPos) break;

        // Colored pixels
        uint16_t coloredPixels = m_sprData[pos] | (m_sprData[pos + 1] << 8);
        pos += 2;

        for (uint16_t i = 0; i < coloredPixels && writePos < 32 * 32; i++) {
            if (pos + 3 > m_sprData.size()) break;

            size_t pixelOffset = writePos * 4;
            pixels[pixelOffset + 0] = m_sprData[pos++]; // R
            pixels[pixelOffset + 1] = m_sprData[pos++]; // G
            pixels[pixelOffset + 2] = m_sprData[pos++]; // B
            pixels[pixelOffset + 3] = 255;               // A

            writePos++;
        }
    }

    // Create texture
    // g_graphics is declared in framework/graphics/graphics.h
    auto texture = g_graphics.createTexture(32, 32, pixels.data());

    m_spriteCache[spriteId] = texture;
    return texture;
}

} // namespace client
} // namespace shadow
