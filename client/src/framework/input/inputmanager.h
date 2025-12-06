/**
 * Shadow OT Client - Input Manager
 *
 * Handles keyboard and mouse input with hotkey support.
 */

#pragma once

#include <cstdint>
#include <functional>
#include <map>
#include <vector>
#include <string>
#include <array>

namespace shadow {
namespace framework {

// Key codes (matching SDL2 scancodes)
enum class KeyCode : int {
    Unknown = 0,

    // Letters
    A = 4, B = 5, C = 6, D = 7, E = 8, F = 9, G = 10, H = 11, I = 12,
    J = 13, K = 14, L = 15, M = 16, N = 17, O = 18, P = 19, Q = 20,
    R = 21, S = 22, T = 23, U = 24, V = 25, W = 26, X = 27, Y = 28, Z = 29,

    // Numbers
    Num1 = 30, Num2 = 31, Num3 = 32, Num4 = 33, Num5 = 34,
    Num6 = 35, Num7 = 36, Num8 = 37, Num9 = 38, Num0 = 39,

    // Function keys
    F1 = 58, F2 = 59, F3 = 60, F4 = 61, F5 = 62, F6 = 63,
    F7 = 64, F8 = 65, F9 = 66, F10 = 67, F11 = 68, F12 = 69,

    // Control keys
    Return = 40,
    Escape = 41,
    Backspace = 42,
    Tab = 43,
    Space = 44,

    // Arrow keys
    Right = 79,
    Left = 80,
    Down = 81,
    Up = 82,

    // Modifiers
    LCtrl = 224,
    LShift = 225,
    LAlt = 226,
    RCtrl = 228,
    RShift = 229,
    RAlt = 230,

    // Numpad
    NumPad0 = 98, NumPad1 = 89, NumPad2 = 90, NumPad3 = 91,
    NumPad4 = 92, NumPad5 = 93, NumPad6 = 94, NumPad7 = 95,
    NumPad8 = 96, NumPad9 = 97,
    NumPadEnter = 88,
    NumPadPlus = 87,
    NumPadMinus = 86,

    // Others
    Insert = 73,
    Delete = 76,
    Home = 74,
    End = 77,
    PageUp = 75,
    PageDown = 78,

    MaxKeys = 512
};

// Mouse buttons
enum class MouseButton : int {
    Left = 1,
    Middle = 2,
    Right = 3,
    X1 = 4,
    X2 = 5
};

// Key modifiers
enum class KeyModifier : uint16_t {
    None = 0,
    Ctrl = 1 << 0,
    Shift = 1 << 1,
    Alt = 1 << 2,
    CtrlShift = Ctrl | Shift,
    CtrlAlt = Ctrl | Alt,
    ShiftAlt = Shift | Alt,
    CtrlShiftAlt = Ctrl | Shift | Alt
};

inline KeyModifier operator|(KeyModifier a, KeyModifier b) {
    return static_cast<KeyModifier>(static_cast<uint16_t>(a) | static_cast<uint16_t>(b));
}

inline bool operator&(KeyModifier a, KeyModifier b) {
    return (static_cast<uint16_t>(a) & static_cast<uint16_t>(b)) != 0;
}

// Input event types
struct KeyEvent {
    KeyCode key;
    KeyModifier modifiers;
    bool pressed;
    bool repeat;
    uint32_t timestamp;
};

struct MouseEvent {
    int x;
    int y;
    int deltaX;
    int deltaY;
    MouseButton button;
    bool pressed;
    int clicks;
    uint32_t timestamp;
};

struct MouseWheelEvent {
    int x;
    int y;
    int scrollX;
    int scrollY;
    uint32_t timestamp;
};

struct TextInputEvent {
    std::string text;
    uint32_t timestamp;
};

// Hotkey binding
struct Hotkey {
    KeyCode key;
    KeyModifier modifiers;
    std::string action;
    std::function<void()> callback;
    bool enabled{true};
};

class InputManager {
public:
    static InputManager& instance();

    void init();
    void terminate();

    // Process SDL events
    void processKeyDown(int keyCode, uint16_t modifiers, bool repeat);
    void processKeyUp(int keyCode, uint16_t modifiers);
    void processMouseMove(int x, int y, int relX, int relY);
    void processMouseButtonDown(int button, int x, int y, int clicks);
    void processMouseButtonUp(int button, int x, int y);
    void processMouseWheel(int x, int y, int scrollX, int scrollY);
    void processTextInput(const std::string& text);

    // Key state
    bool isKeyPressed(KeyCode key) const;
    bool isKeyDown(KeyCode key) const;  // Pressed this frame
    bool isKeyUp(KeyCode key) const;    // Released this frame
    bool isModifierPressed(KeyModifier mod) const;
    KeyModifier getCurrentModifiers() const;

    // Mouse state
    int getMouseX() const { return m_mouseX; }
    int getMouseY() const { return m_mouseY; }
    bool isMouseButtonPressed(MouseButton button) const;
    bool isMouseButtonDown(MouseButton button) const;
    bool isMouseButtonUp(MouseButton button) const;

    // Hotkey system
    void registerHotkey(const std::string& id, KeyCode key, KeyModifier modifiers,
                       const std::string& action, std::function<void()> callback);
    void unregisterHotkey(const std::string& id);
    void setHotkeyEnabled(const std::string& id, bool enabled);
    Hotkey* getHotkey(const std::string& id);
    const std::map<std::string, Hotkey>& getHotkeys() const { return m_hotkeys; }

    // Event callbacks
    using KeyCallback = std::function<void(const KeyEvent&)>;
    using MouseCallback = std::function<void(const MouseEvent&)>;
    using MouseWheelCallback = std::function<void(const MouseWheelEvent&)>;
    using TextInputCallback = std::function<void(const TextInputEvent&)>;

    void setKeyCallback(KeyCallback callback) { m_keyCallback = callback; }
    void setMouseCallback(MouseCallback callback) { m_mouseCallback = callback; }
    void setMouseWheelCallback(MouseWheelCallback callback) { m_wheelCallback = callback; }
    void setTextInputCallback(TextInputCallback callback) { m_textCallback = callback; }

    // Text input mode
    void startTextInput();
    void stopTextInput();
    bool isTextInputActive() const { return m_textInputActive; }

    // Reset per-frame state
    void update();

    // Convert key code to string name
    static std::string getKeyName(KeyCode key);
    static KeyCode getKeyFromName(const std::string& name);

private:
    InputManager() = default;

    void checkHotkeys(const KeyEvent& event);

    // Key states
    std::array<bool, static_cast<size_t>(KeyCode::MaxKeys)> m_keyPressed{};
    std::array<bool, static_cast<size_t>(KeyCode::MaxKeys)> m_keyDown{};
    std::array<bool, static_cast<size_t>(KeyCode::MaxKeys)> m_keyUp{};

    // Mouse state
    int m_mouseX{0};
    int m_mouseY{0};
    std::array<bool, 8> m_mousePressed{};
    std::array<bool, 8> m_mouseDown{};
    std::array<bool, 8> m_mouseUp{};

    // Current modifiers
    KeyModifier m_currentModifiers{KeyModifier::None};

    // Hotkeys
    std::map<std::string, Hotkey> m_hotkeys;

    // Callbacks
    KeyCallback m_keyCallback;
    MouseCallback m_mouseCallback;
    MouseWheelCallback m_wheelCallback;
    TextInputCallback m_textCallback;

    // Text input state
    bool m_textInputActive{false};
};

} // namespace framework
} // namespace shadow

// Global accessor
extern shadow::framework::InputManager& g_input;
