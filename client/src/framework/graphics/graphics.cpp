/**
 * Shadow OT Client - Graphics Implementation
 */

#include "graphics.h"
#include <framework/core/resourcemanager.h>

#include <GL/glew.h>
#include <GLFW/glfw3.h>

#include <vector>
#include <cmath>
#include <cstring>
#include <fstream>

namespace shadow {
namespace framework {

Color Color::fromHex(const std::string& hex) {
    std::string h = hex;
    if (!h.empty() && h[0] == '#') h = h.substr(1);

    if (h.empty()) return Color::white();

    uint32_t value = std::stoul(h, nullptr, 16);

    if (h.length() == 6) {
        return Color(
            (value >> 16) & 0xFF,
            (value >> 8) & 0xFF,
            value & 0xFF,
            255
        );
    } else if (h.length() == 8) {
        return Color(
            (value >> 24) & 0xFF,
            (value >> 16) & 0xFF,
            (value >> 8) & 0xFF,
            value & 0xFF
        );
    }
    return Color::white();
}

class GLTexture : public Texture {
public:
    GLTexture(uint32_t id, int width, int height, bool alpha)
        : m_id(id), m_width(width), m_height(height), m_hasAlpha(alpha) {}

    ~GLTexture() override {
        if (m_id) {
            glDeleteTextures(1, &m_id);
        }
    }

    uint32_t getId() const override { return m_id; }
    int getWidth() const override { return m_width; }
    int getHeight() const override { return m_height; }
    bool hasAlpha() const override { return m_hasAlpha; }

    void bind(int unit) const override {
        glActiveTexture(GL_TEXTURE0 + unit);
        glBindTexture(GL_TEXTURE_2D, m_id);
    }

    void unbind() const override {
        glBindTexture(GL_TEXTURE_2D, 0);
    }

private:
    uint32_t m_id;
    int m_width;
    int m_height;
    bool m_hasAlpha;
};

struct Graphics::Impl {
    GLuint vao{0};
    GLuint vbo{0};
    GLuint shaderProgram{0};

    std::vector<Rect> clipStack;
    Color currentColor{255, 255, 255, 255};
    float opacity{1.0f};

    int viewportWidth{0};
    int viewportHeight{0};

    GLFWwindow* window{nullptr};

    void setWindow(GLFWwindow* win) { window = win; }
};

Graphics& Graphics::instance() {
    static Graphics instance;
    return instance;
}

static const char* vertexShaderSource = R"(
#version 330 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec2 aTexCoord;
layout (location = 2) in vec4 aColor;

out vec2 TexCoord;
out vec4 Color;

uniform mat4 projection;

void main() {
    gl_Position = projection * vec4(aPos, 0.0, 1.0);
    TexCoord = aTexCoord;
    Color = aColor;
}
)";

static const char* fragmentShaderSource = R"(
#version 330 core
in vec2 TexCoord;
in vec4 Color;

out vec4 FragColor;

uniform sampler2D tex;
uniform bool useTexture;

void main() {
    if (useTexture) {
        FragColor = texture(tex, TexCoord) * Color;
    } else {
        FragColor = Color;
    }
}
)";

static GLuint compileShader(GLenum type, const char* source) {
    GLuint shader = glCreateShader(type);
    glShaderSource(shader, 1, &source, nullptr);
    glCompileShader(shader);

    GLint success;
    glGetShaderiv(shader, GL_COMPILE_STATUS, &success);
    if (!success) {
        char log[512];
        glGetShaderInfoLog(shader, 512, nullptr, log);
        // Log error
    }

    return shader;
}

bool Graphics::init() {
    m_impl = std::make_unique<Impl>();

    // Initialize GLEW - requires valid OpenGL context (window must be created first)
    glewExperimental = GL_TRUE;
    GLenum glewErr = glewInit();
    if (glewErr != GLEW_OK) {
        // GLEW initialization failed
        return false;
    }
    // Clear any GL errors from GLEW initialization
    glGetError();

    const char* renderer = reinterpret_cast<const char*>(glGetString(GL_RENDERER));
    const char* vendor = reinterpret_cast<const char*>(glGetString(GL_VENDOR));
    m_renderer = renderer ? renderer : "Unknown";
    m_vendor = vendor ? vendor : "Unknown";
    glGetIntegerv(GL_MAX_TEXTURE_SIZE, &m_maxTextureSize);

    // Create shader program
    GLuint vertexShader = compileShader(GL_VERTEX_SHADER, vertexShaderSource);
    GLuint fragmentShader = compileShader(GL_FRAGMENT_SHADER, fragmentShaderSource);

    m_impl->shaderProgram = glCreateProgram();
    glAttachShader(m_impl->shaderProgram, vertexShader);
    glAttachShader(m_impl->shaderProgram, fragmentShader);
    glLinkProgram(m_impl->shaderProgram);

    glDeleteShader(vertexShader);
    glDeleteShader(fragmentShader);

    // Create VAO/VBO
    glGenVertexArrays(1, &m_impl->vao);
    glGenBuffers(1, &m_impl->vbo);

    glBindVertexArray(m_impl->vao);
    glBindBuffer(GL_ARRAY_BUFFER, m_impl->vbo);

    // Position
    glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE, 8 * sizeof(float), (void*)0);
    glEnableVertexAttribArray(0);
    // TexCoord
    glVertexAttribPointer(1, 2, GL_FLOAT, GL_FALSE, 8 * sizeof(float), (void*)(2 * sizeof(float)));
    glEnableVertexAttribArray(1);
    // Color
    glVertexAttribPointer(2, 4, GL_FLOAT, GL_FALSE, 8 * sizeof(float), (void*)(4 * sizeof(float)));
    glEnableVertexAttribArray(2);

    glEnable(GL_BLEND);
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

    return true;
}

void Graphics::terminate() {
    if (m_impl) {
        if (m_impl->vbo) glDeleteBuffers(1, &m_impl->vbo);
        if (m_impl->vao) glDeleteVertexArrays(1, &m_impl->vao);
        if (m_impl->shaderProgram) glDeleteProgram(m_impl->shaderProgram);
        m_impl.reset();
    }
}

void Graphics::beginFrame() {
    // Nothing specific needed
}

void Graphics::endFrame() {
    // Nothing specific needed
}

void Graphics::render() {
    // Swap buffers to present the frame
    if (m_impl && m_impl->window) {
        glfwSwapBuffers(m_impl->window);
    }
}

void Graphics::setViewport(int x, int y, int width, int height) {
    glViewport(x, y, width, height);
    if (m_impl) {
        m_impl->viewportWidth = width;
        m_impl->viewportHeight = height;
    }
}

void Graphics::setOrtho(int width, int height) {
    if (!m_impl || !m_impl->shaderProgram) return;

    glUseProgram(m_impl->shaderProgram);

    // Create orthographic projection matrix
    float left = 0.0f;
    float right = static_cast<float>(width);
    float bottom = static_cast<float>(height);
    float top = 0.0f;
    float nearPlane = -1.0f;
    float farPlane = 1.0f;

    float projection[16] = {
        2.0f / (right - left), 0.0f, 0.0f, 0.0f,
        0.0f, 2.0f / (top - bottom), 0.0f, 0.0f,
        0.0f, 0.0f, -2.0f / (farPlane - nearPlane), 0.0f,
        -(right + left) / (right - left), -(top + bottom) / (top - bottom), -(farPlane + nearPlane) / (farPlane - nearPlane), 1.0f
    };

    GLint loc = glGetUniformLocation(m_impl->shaderProgram, "projection");
    glUniformMatrix4fv(loc, 1, GL_FALSE, projection);

    m_impl->viewportWidth = width;
    m_impl->viewportHeight = height;
}

void Graphics::drawRect(const Rect& rect, const Color& color) {
    if (!m_impl) return;

    float vertices[] = {
        (float)rect.x, (float)rect.y, 0.0f, 0.0f, color.r/255.0f, color.g/255.0f, color.b/255.0f, color.a/255.0f * m_impl->opacity,
        (float)(rect.x + rect.width), (float)rect.y, 0.0f, 0.0f, color.r/255.0f, color.g/255.0f, color.b/255.0f, color.a/255.0f * m_impl->opacity,
        (float)(rect.x + rect.width), (float)(rect.y + rect.height), 0.0f, 0.0f, color.r/255.0f, color.g/255.0f, color.b/255.0f, color.a/255.0f * m_impl->opacity,
        (float)rect.x, (float)(rect.y + rect.height), 0.0f, 0.0f, color.r/255.0f, color.g/255.0f, color.b/255.0f, color.a/255.0f * m_impl->opacity,
    };

    glUseProgram(m_impl->shaderProgram);
    glUniform1i(glGetUniformLocation(m_impl->shaderProgram, "useTexture"), 0);

    glBindVertexArray(m_impl->vao);
    glBindBuffer(GL_ARRAY_BUFFER, m_impl->vbo);
    glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_DYNAMIC_DRAW);

    glDrawArrays(GL_LINE_LOOP, 0, 4);
}

void Graphics::drawFilledRect(const Rect& rect, const Color& color) {
    if (!m_impl) return;

    float vertices[] = {
        (float)rect.x, (float)rect.y, 0.0f, 0.0f, color.r/255.0f, color.g/255.0f, color.b/255.0f, color.a/255.0f * m_impl->opacity,
        (float)(rect.x + rect.width), (float)rect.y, 1.0f, 0.0f, color.r/255.0f, color.g/255.0f, color.b/255.0f, color.a/255.0f * m_impl->opacity,
        (float)(rect.x + rect.width), (float)(rect.y + rect.height), 1.0f, 1.0f, color.r/255.0f, color.g/255.0f, color.b/255.0f, color.a/255.0f * m_impl->opacity,

        (float)rect.x, (float)rect.y, 0.0f, 0.0f, color.r/255.0f, color.g/255.0f, color.b/255.0f, color.a/255.0f * m_impl->opacity,
        (float)(rect.x + rect.width), (float)(rect.y + rect.height), 1.0f, 1.0f, color.r/255.0f, color.g/255.0f, color.b/255.0f, color.a/255.0f * m_impl->opacity,
        (float)rect.x, (float)(rect.y + rect.height), 0.0f, 1.0f, color.r/255.0f, color.g/255.0f, color.b/255.0f, color.a/255.0f * m_impl->opacity,
    };

    glUseProgram(m_impl->shaderProgram);
    glUniform1i(glGetUniformLocation(m_impl->shaderProgram, "useTexture"), 0);

    glBindVertexArray(m_impl->vao);
    glBindBuffer(GL_ARRAY_BUFFER, m_impl->vbo);
    glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_DYNAMIC_DRAW);

    glDrawArrays(GL_TRIANGLES, 0, 6);
}

void Graphics::drawTexture(const Texture* texture, int x, int y) {
    if (!texture) return;
    drawTexture(texture, Rect(x, y, texture->getWidth(), texture->getHeight()));
}

void Graphics::drawTexture(const Texture* texture, const Rect& dest) {
    if (!texture || !m_impl) return;

    float vertices[] = {
        (float)dest.x, (float)dest.y, 0.0f, 0.0f, 1.0f, 1.0f, 1.0f, m_impl->opacity,
        (float)(dest.x + dest.width), (float)dest.y, 1.0f, 0.0f, 1.0f, 1.0f, 1.0f, m_impl->opacity,
        (float)(dest.x + dest.width), (float)(dest.y + dest.height), 1.0f, 1.0f, 1.0f, 1.0f, 1.0f, m_impl->opacity,

        (float)dest.x, (float)dest.y, 0.0f, 0.0f, 1.0f, 1.0f, 1.0f, m_impl->opacity,
        (float)(dest.x + dest.width), (float)(dest.y + dest.height), 1.0f, 1.0f, 1.0f, 1.0f, 1.0f, m_impl->opacity,
        (float)dest.x, (float)(dest.y + dest.height), 0.0f, 1.0f, 1.0f, 1.0f, 1.0f, m_impl->opacity,
    };

    glUseProgram(m_impl->shaderProgram);
    glUniform1i(glGetUniformLocation(m_impl->shaderProgram, "useTexture"), 1);

    texture->bind(0);

    glBindVertexArray(m_impl->vao);
    glBindBuffer(GL_ARRAY_BUFFER, m_impl->vbo);
    glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_DYNAMIC_DRAW);

    glDrawArrays(GL_TRIANGLES, 0, 6);
}

void Graphics::drawTexture(const Texture* texture, const Rect& src, const Rect& dest) {
    if (!texture || !m_impl) return;

    float tw = static_cast<float>(texture->getWidth());
    float th = static_cast<float>(texture->getHeight());

    float u0 = src.x / tw;
    float v0 = src.y / th;
    float u1 = (src.x + src.width) / tw;
    float v1 = (src.y + src.height) / th;

    float vertices[] = {
        (float)dest.x, (float)dest.y, u0, v0, 1.0f, 1.0f, 1.0f, m_impl->opacity,
        (float)(dest.x + dest.width), (float)dest.y, u1, v0, 1.0f, 1.0f, 1.0f, m_impl->opacity,
        (float)(dest.x + dest.width), (float)(dest.y + dest.height), u1, v1, 1.0f, 1.0f, 1.0f, m_impl->opacity,

        (float)dest.x, (float)dest.y, u0, v0, 1.0f, 1.0f, 1.0f, m_impl->opacity,
        (float)(dest.x + dest.width), (float)(dest.y + dest.height), u1, v1, 1.0f, 1.0f, 1.0f, m_impl->opacity,
        (float)dest.x, (float)(dest.y + dest.height), u0, v1, 1.0f, 1.0f, 1.0f, m_impl->opacity,
    };

    glUseProgram(m_impl->shaderProgram);
    glUniform1i(glGetUniformLocation(m_impl->shaderProgram, "useTexture"), 1);

    texture->bind(0);

    glBindVertexArray(m_impl->vao);
    glBindBuffer(GL_ARRAY_BUFFER, m_impl->vbo);
    glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_DYNAMIC_DRAW);

    glDrawArrays(GL_TRIANGLES, 0, 6);
}

void Graphics::pushClipRect(const Rect& rect) {
    if (!m_impl) return;
    m_impl->clipStack.push_back(rect);
    glEnable(GL_SCISSOR_TEST);
    glScissor(rect.x, m_impl->viewportHeight - rect.y - rect.height, rect.width, rect.height);
}

void Graphics::popClipRect() {
    if (!m_impl) return;

    if (!m_impl->clipStack.empty()) {
        m_impl->clipStack.pop_back();
    }

    if (m_impl->clipStack.empty()) {
        glDisable(GL_SCISSOR_TEST);
    } else {
        const Rect& rect = m_impl->clipStack.back();
        glScissor(rect.x, m_impl->viewportHeight - rect.y - rect.height, rect.width, rect.height);
    }
}

void Graphics::setOpacity(float opacity) {
    if (m_impl) {
        m_impl->opacity = opacity;
    }
}

void Graphics::clear(const Color& color) {
    glClearColor(color.r / 255.0f, color.g / 255.0f, color.b / 255.0f, color.a / 255.0f);
    glClear(GL_COLOR_BUFFER_BIT);
}

void Graphics::present() {
    render();
}

void Graphics::setWindow(void* window) {
    if (m_impl) {
        m_impl->window = static_cast<GLFWwindow*>(window);
    }
}

std::shared_ptr<Texture> Graphics::createTexture(int width, int height, const uint8_t* data, bool hasAlpha) {
    GLuint textureId;
    glGenTextures(1, &textureId);
    glBindTexture(GL_TEXTURE_2D, textureId);

    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);

    GLenum format = hasAlpha ? GL_RGBA : GL_RGB;
    glTexImage2D(GL_TEXTURE_2D, 0, format, width, height, 0, format, GL_UNSIGNED_BYTE, data);

    return std::make_shared<GLTexture>(textureId, width, height, hasAlpha);
}

std::shared_ptr<Texture> Graphics::loadTexture(const std::string& filename) {
    // Simple BMP/PNG loader would go here
    // For now, return nullptr - texture loading requires stb_image or similar
    return nullptr;
}

void Graphics::drawText(const std::string& text, int x, int y, const Color& color, int fontSize) {
    // Text rendering would require a proper font rendering system (FreeType, stb_truetype, etc.)
    // For now, this is a stub - text will not be visible until font rendering is implemented
    // In a full implementation, this would:
    // 1. Get/load font at the specified size
    // 2. Render each character glyph using textures
    // 3. Position characters with proper kerning
    (void)text;
    (void)x;
    (void)y;
    (void)color;
    (void)fontSize;
}

Size Graphics::measureText(const std::string& text, int fontSize) const {
    // Placeholder text measurement
    // In a full implementation, this would use the actual font metrics
    // For now, estimate based on character count
    int charWidth = fontSize * 6 / 10;  // Rough estimate for monospace
    int charHeight = fontSize;
    return Size(static_cast<int>(text.length()) * charWidth, charHeight);
}

// Global instance inside the namespace
Graphics& g_graphics = Graphics::instance();

} // namespace framework
} // namespace shadow
