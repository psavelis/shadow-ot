/**
 * Shadow OT Client - Input Manager Implementation
 */

#include "inputmanager.h"
#include <algorithm>

namespace shadow {
namespace framework {

InputManager& InputManager::instance() {
    static InputManager instance;
    return instance;
}

void InputManager::init() {
    m_keyPressed.fill(false);
    m_keyDown.fill(false);
    m_keyUp.fill(false);
    m_mousePressed.fill(false);
    m_mouseDown.fill(false);
    m_mouseUp.fill(false);
    m_currentModifiers = KeyModifier::None;
}

void InputManager::terminate() {
    m_hotkeys.clear();
    m_keyCallback = nullptr;
    m_mouseCallback = nullptr;
    m_wheelCallback = nullptr;
    m_textCallback = nullptr;
}

void InputManager::update() {
    // Clear per-frame states
    m_keyDown.fill(false);
    m_keyUp.fill(false);
    m_mouseDown.fill(false);
    m_mouseUp.fill(false);
}

void InputManager::processKeyDown(int keyCode, uint16_t modifiers, bool repeat) {
    if (keyCode < 0 || keyCode >= static_cast<int>(KeyCode::MaxKeys)) return;

    KeyCode key = static_cast<KeyCode>(keyCode);

    // Update modifiers
    m_currentModifiers = static_cast<KeyModifier>(modifiers);

    // Update key state
    if (!m_keyPressed[keyCode]) {
        m_keyDown[keyCode] = true;
    }
    m_keyPressed[keyCode] = true;

    // Create event
    KeyEvent event;
    event.key = key;
    event.modifiers = m_currentModifiers;
    event.pressed = true;
    event.repeat = repeat;
    event.timestamp = 0;  // Would use SDL_GetTicks() or similar

    // Check hotkeys
    if (!repeat) {
        checkHotkeys(event);
    }

    // Notify callback
    if (m_keyCallback) {
        m_keyCallback(event);
    }
}

void InputManager::processKeyUp(int keyCode, uint16_t modifiers) {
    if (keyCode < 0 || keyCode >= static_cast<int>(KeyCode::MaxKeys)) return;

    KeyCode key = static_cast<KeyCode>(keyCode);

    // Update modifiers
    m_currentModifiers = static_cast<KeyModifier>(modifiers);

    // Update key state
    m_keyPressed[keyCode] = false;
    m_keyUp[keyCode] = true;

    // Create event
    KeyEvent event;
    event.key = key;
    event.modifiers = m_currentModifiers;
    event.pressed = false;
    event.repeat = false;
    event.timestamp = 0;

    // Notify callback
    if (m_keyCallback) {
        m_keyCallback(event);
    }
}

void InputManager::processMouseMove(int x, int y, int relX, int relY) {
    m_mouseX = x;
    m_mouseY = y;

    MouseEvent event;
    event.x = x;
    event.y = y;
    event.deltaX = relX;
    event.deltaY = relY;
    event.button = MouseButton::Left;
    event.pressed = false;
    event.clicks = 0;
    event.timestamp = 0;

    if (m_mouseCallback) {
        m_mouseCallback(event);
    }
}

void InputManager::processMouseButtonDown(int button, int x, int y, int clicks) {
    if (button < 1 || button > 7) return;

    m_mouseX = x;
    m_mouseY = y;

    if (!m_mousePressed[button]) {
        m_mouseDown[button] = true;
    }
    m_mousePressed[button] = true;

    MouseEvent event;
    event.x = x;
    event.y = y;
    event.deltaX = 0;
    event.deltaY = 0;
    event.button = static_cast<MouseButton>(button);
    event.pressed = true;
    event.clicks = clicks;
    event.timestamp = 0;

    if (m_mouseCallback) {
        m_mouseCallback(event);
    }
}

void InputManager::processMouseButtonUp(int button, int x, int y) {
    if (button < 1 || button > 7) return;

    m_mouseX = x;
    m_mouseY = y;

    m_mousePressed[button] = false;
    m_mouseUp[button] = true;

    MouseEvent event;
    event.x = x;
    event.y = y;
    event.deltaX = 0;
    event.deltaY = 0;
    event.button = static_cast<MouseButton>(button);
    event.pressed = false;
    event.clicks = 0;
    event.timestamp = 0;

    if (m_mouseCallback) {
        m_mouseCallback(event);
    }
}

void InputManager::processMouseWheel(int x, int y, int scrollX, int scrollY) {
    MouseWheelEvent event;
    event.x = m_mouseX;
    event.y = m_mouseY;
    event.scrollX = scrollX;
    event.scrollY = scrollY;
    event.timestamp = 0;

    if (m_wheelCallback) {
        m_wheelCallback(event);
    }
}

void InputManager::processTextInput(const std::string& text) {
    if (!m_textInputActive) return;

    TextInputEvent event;
    event.text = text;
    event.timestamp = 0;

    if (m_textCallback) {
        m_textCallback(event);
    }
}

bool InputManager::isKeyPressed(KeyCode key) const {
    int code = static_cast<int>(key);
    if (code >= 0 && code < static_cast<int>(KeyCode::MaxKeys)) {
        return m_keyPressed[code];
    }
    return false;
}

bool InputManager::isKeyDown(KeyCode key) const {
    int code = static_cast<int>(key);
    if (code >= 0 && code < static_cast<int>(KeyCode::MaxKeys)) {
        return m_keyDown[code];
    }
    return false;
}

bool InputManager::isKeyUp(KeyCode key) const {
    int code = static_cast<int>(key);
    if (code >= 0 && code < static_cast<int>(KeyCode::MaxKeys)) {
        return m_keyUp[code];
    }
    return false;
}

bool InputManager::isModifierPressed(KeyModifier mod) const {
    return m_currentModifiers & mod;
}

KeyModifier InputManager::getCurrentModifiers() const {
    return m_currentModifiers;
}

bool InputManager::isMouseButtonPressed(MouseButton button) const {
    int btn = static_cast<int>(button);
    if (btn >= 1 && btn < 8) {
        return m_mousePressed[btn];
    }
    return false;
}

bool InputManager::isMouseButtonDown(MouseButton button) const {
    int btn = static_cast<int>(button);
    if (btn >= 1 && btn < 8) {
        return m_mouseDown[btn];
    }
    return false;
}

bool InputManager::isMouseButtonUp(MouseButton button) const {
    int btn = static_cast<int>(button);
    if (btn >= 1 && btn < 8) {
        return m_mouseUp[btn];
    }
    return false;
}

void InputManager::registerHotkey(const std::string& id, KeyCode key, KeyModifier modifiers,
                                  const std::string& action, std::function<void()> callback) {
    Hotkey hotkey;
    hotkey.key = key;
    hotkey.modifiers = modifiers;
    hotkey.action = action;
    hotkey.callback = callback;
    hotkey.enabled = true;

    m_hotkeys[id] = hotkey;
}

void InputManager::unregisterHotkey(const std::string& id) {
    m_hotkeys.erase(id);
}

void InputManager::setHotkeyEnabled(const std::string& id, bool enabled) {
    auto it = m_hotkeys.find(id);
    if (it != m_hotkeys.end()) {
        it->second.enabled = enabled;
    }
}

Hotkey* InputManager::getHotkey(const std::string& id) {
    auto it = m_hotkeys.find(id);
    if (it != m_hotkeys.end()) {
        return &it->second;
    }
    return nullptr;
}

void InputManager::checkHotkeys(const KeyEvent& event) {
    if (!event.pressed) return;

    for (auto& [id, hotkey] : m_hotkeys) {
        if (!hotkey.enabled) continue;
        if (hotkey.key != event.key) continue;

        // Check modifiers match
        bool ctrlMatch = (hotkey.modifiers & KeyModifier::Ctrl) ==
                        (event.modifiers & KeyModifier::Ctrl);
        bool shiftMatch = (hotkey.modifiers & KeyModifier::Shift) ==
                         (event.modifiers & KeyModifier::Shift);
        bool altMatch = (hotkey.modifiers & KeyModifier::Alt) ==
                       (event.modifiers & KeyModifier::Alt);

        if (ctrlMatch && shiftMatch && altMatch && hotkey.callback) {
            hotkey.callback();
        }
    }
}

void InputManager::startTextInput() {
    m_textInputActive = true;
    // Would call SDL_StartTextInput() here
}

void InputManager::stopTextInput() {
    m_textInputActive = false;
    // Would call SDL_StopTextInput() here
}

std::string InputManager::getKeyName(KeyCode key) {
    switch (key) {
        case KeyCode::A: return "A";
        case KeyCode::B: return "B";
        case KeyCode::C: return "C";
        case KeyCode::D: return "D";
        case KeyCode::E: return "E";
        case KeyCode::F: return "F";
        case KeyCode::G: return "G";
        case KeyCode::H: return "H";
        case KeyCode::I: return "I";
        case KeyCode::J: return "J";
        case KeyCode::K: return "K";
        case KeyCode::L: return "L";
        case KeyCode::M: return "M";
        case KeyCode::N: return "N";
        case KeyCode::O: return "O";
        case KeyCode::P: return "P";
        case KeyCode::Q: return "Q";
        case KeyCode::R: return "R";
        case KeyCode::S: return "S";
        case KeyCode::T: return "T";
        case KeyCode::U: return "U";
        case KeyCode::V: return "V";
        case KeyCode::W: return "W";
        case KeyCode::X: return "X";
        case KeyCode::Y: return "Y";
        case KeyCode::Z: return "Z";

        case KeyCode::Num0: return "0";
        case KeyCode::Num1: return "1";
        case KeyCode::Num2: return "2";
        case KeyCode::Num3: return "3";
        case KeyCode::Num4: return "4";
        case KeyCode::Num5: return "5";
        case KeyCode::Num6: return "6";
        case KeyCode::Num7: return "7";
        case KeyCode::Num8: return "8";
        case KeyCode::Num9: return "9";

        case KeyCode::F1: return "F1";
        case KeyCode::F2: return "F2";
        case KeyCode::F3: return "F3";
        case KeyCode::F4: return "F4";
        case KeyCode::F5: return "F5";
        case KeyCode::F6: return "F6";
        case KeyCode::F7: return "F7";
        case KeyCode::F8: return "F8";
        case KeyCode::F9: return "F9";
        case KeyCode::F10: return "F10";
        case KeyCode::F11: return "F11";
        case KeyCode::F12: return "F12";

        case KeyCode::Return: return "Return";
        case KeyCode::Escape: return "Escape";
        case KeyCode::Backspace: return "Backspace";
        case KeyCode::Tab: return "Tab";
        case KeyCode::Space: return "Space";

        case KeyCode::Up: return "Up";
        case KeyCode::Down: return "Down";
        case KeyCode::Left: return "Left";
        case KeyCode::Right: return "Right";

        case KeyCode::LCtrl: return "Left Ctrl";
        case KeyCode::LShift: return "Left Shift";
        case KeyCode::LAlt: return "Left Alt";
        case KeyCode::RCtrl: return "Right Ctrl";
        case KeyCode::RShift: return "Right Shift";
        case KeyCode::RAlt: return "Right Alt";

        default: return "Unknown";
    }
}

KeyCode InputManager::getKeyFromName(const std::string& name) {
    if (name == "A") return KeyCode::A;
    if (name == "B") return KeyCode::B;
    if (name == "C") return KeyCode::C;
    if (name == "D") return KeyCode::D;
    if (name == "E") return KeyCode::E;
    if (name == "F") return KeyCode::F;
    if (name == "G") return KeyCode::G;
    if (name == "H") return KeyCode::H;
    if (name == "I") return KeyCode::I;
    if (name == "J") return KeyCode::J;
    if (name == "K") return KeyCode::K;
    if (name == "L") return KeyCode::L;
    if (name == "M") return KeyCode::M;
    if (name == "N") return KeyCode::N;
    if (name == "O") return KeyCode::O;
    if (name == "P") return KeyCode::P;
    if (name == "Q") return KeyCode::Q;
    if (name == "R") return KeyCode::R;
    if (name == "S") return KeyCode::S;
    if (name == "T") return KeyCode::T;
    if (name == "U") return KeyCode::U;
    if (name == "V") return KeyCode::V;
    if (name == "W") return KeyCode::W;
    if (name == "X") return KeyCode::X;
    if (name == "Y") return KeyCode::Y;
    if (name == "Z") return KeyCode::Z;

    if (name == "F1") return KeyCode::F1;
    if (name == "F2") return KeyCode::F2;
    if (name == "F3") return KeyCode::F3;
    if (name == "F4") return KeyCode::F4;
    if (name == "F5") return KeyCode::F5;
    if (name == "F6") return KeyCode::F6;
    if (name == "F7") return KeyCode::F7;
    if (name == "F8") return KeyCode::F8;
    if (name == "F9") return KeyCode::F9;
    if (name == "F10") return KeyCode::F10;
    if (name == "F11") return KeyCode::F11;
    if (name == "F12") return KeyCode::F12;

    if (name == "Return" || name == "Enter") return KeyCode::Return;
    if (name == "Escape" || name == "Esc") return KeyCode::Escape;
    if (name == "Space") return KeyCode::Space;
    if (name == "Tab") return KeyCode::Tab;
    if (name == "Backspace") return KeyCode::Backspace;

    if (name == "Up") return KeyCode::Up;
    if (name == "Down") return KeyCode::Down;
    if (name == "Left") return KeyCode::Left;
    if (name == "Right") return KeyCode::Right;

    return KeyCode::Unknown;
}

} // namespace framework
} // namespace shadow

// Global accessor
shadow::framework::InputManager& g_input = shadow::framework::InputManager::instance();
