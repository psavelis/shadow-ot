/**
 * Shadow OT Client - Graphics System
 *
 * OpenGL-based rendering system with modern features.
 */

#pragma once

#include <string>
#include <memory>
#include <vector>
#include <cstdint>

namespace shadow {
namespace framework {

struct Color {
    uint8_t r, g, b, a;

    Color() : r(255), g(255), b(255), a(255) {}
    Color(uint8_t r, uint8_t g, uint8_t b, uint8_t a = 255) : r(r), g(g), b(b), a(a) {}

    static Color fromHex(const std::string& hex);
    static Color white() { return Color(255, 255, 255); }
    static Color black() { return Color(0, 0, 0); }
    static Color red() { return Color(255, 0, 0); }
    static Color green() { return Color(0, 255, 0); }
    static Color blue() { return Color(0, 0, 255); }
    static Color transparent() { return Color(0, 0, 0, 0); }
};

struct Rect {
    int x, y, width, height;

    Rect() : x(0), y(0), width(0), height(0) {}
    Rect(int x, int y, int w, int h) : x(x), y(y), width(w), height(h) {}

    bool contains(int px, int py) const {
        return px >= x && px < x + width && py >= y && py < y + height;
    }

    bool intersects(const Rect& other) const {
        return !(x + width <= other.x || other.x + other.width <= x ||
                 y + height <= other.y || other.y + other.height <= y);
    }
};

struct Point {
    int x, y;
    Point() : x(0), y(0) {}
    Point(int x, int y) : x(x), y(y) {}
};

struct Size {
    int width, height;
    Size() : width(0), height(0) {}
    Size(int w, int h) : width(w), height(h) {}
};

class Texture {
public:
    virtual ~Texture() = default;

    virtual uint32_t getId() const = 0;
    virtual int getWidth() const = 0;
    virtual int getHeight() const = 0;
    virtual bool hasAlpha() const = 0;

    virtual void bind(int unit = 0) const = 0;
    virtual void unbind() const = 0;
};

class Graphics {
public:
    static Graphics& instance();

    bool init();
    void terminate();

    void beginFrame();
    void endFrame();
    void render();

    // Viewport
    void setViewport(int x, int y, int width, int height);
    void setOrtho(int width, int height);

    // Drawing primitives
    void drawRect(const Rect& rect, const Color& color);
    void drawFilledRect(const Rect& rect, const Color& color);
    void drawRoundedRect(const Rect& rect, const Color& color, int radius);

    void drawLine(int x1, int y1, int x2, int y2, const Color& color, int thickness = 1);
    void drawCircle(int cx, int cy, int radius, const Color& color);
    void drawFilledCircle(int cx, int cy, int radius, const Color& color);

    // Texture drawing
    void drawTexture(const Texture* texture, int x, int y);
    void drawTexture(const Texture* texture, const Rect& dest);
    void drawTexture(const Texture* texture, const Rect& src, const Rect& dest);
    void drawTextureColored(const Texture* texture, const Rect& dest, const Color& color);

    // Text drawing (requires font)
    void drawText(const std::string& text, int x, int y, const Color& color, int fontSize = 12);
    Size measureText(const std::string& text, int fontSize = 12) const;

    // Clipping
    void pushClipRect(const Rect& rect);
    void popClipRect();

    // State management
    void setBlendMode(int mode);
    void setColor(const Color& color);
    void setOpacity(float opacity);

    // Framebuffer operations
    void clear(const Color& color = Color::black());
    void present();

    // Window management (for buffer swapping)
    void setWindow(void* window);

    // Info
    const std::string& getRenderer() const { return m_renderer; }
    const std::string& getVendor() const { return m_vendor; }
    int getMaxTextureSize() const { return m_maxTextureSize; }

    // Create resources
    std::shared_ptr<Texture> createTexture(int width, int height, const uint8_t* data, bool hasAlpha = true);
    std::shared_ptr<Texture> loadTexture(const std::string& filename);

private:
    Graphics() = default;
    ~Graphics() = default;
    Graphics(const Graphics&) = delete;
    Graphics& operator=(const Graphics&) = delete;

    std::string m_renderer;
    std::string m_vendor;
    int m_maxTextureSize{4096};

    struct Impl;
    std::unique_ptr<Impl> m_impl;
};

// Global accessor inside namespace
extern Graphics& g_graphics;

} // namespace framework
} // namespace shadow
