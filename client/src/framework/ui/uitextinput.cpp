/**
 * Shadow OT Client - UI Text Input Implementation
 */

#include "uitextinput.h"
#include <framework/graphics/graphics.h>
#include <framework/platform/platform.h>
#include <algorithm>

// Key codes (SDL-compatible)
#define KEY_BACKSPACE 8
#define KEY_TAB 9
#define KEY_RETURN 13
#define KEY_ESCAPE 27
#define KEY_DELETE 127
#define KEY_LEFT 1073741904
#define KEY_RIGHT 1073741903
#define KEY_HOME 1073741898
#define KEY_END 1073741901

// Modifier flags
#define MOD_CTRL 0x40
#define MOD_SHIFT 0x01

namespace shadow {
namespace framework {

UITextInput::UITextInput() : UIWidget() {
    m_backgroundColor = Color{30, 30, 30, 255};
    m_borderColor = Color{80, 80, 80, 255};
    m_borderWidth = 1;
}

void UITextInput::setText(const std::string& text) {
    m_text = text;
    if (m_text.length() > m_maxLength) {
        m_text = m_text.substr(0, m_maxLength);
    }
    m_cursorPos = m_text.length();
    m_selectionStart = m_selectionEnd = m_cursorPos;
    updateDisplayText();

    if (m_onTextChange) {
        m_onTextChange(m_text);
    }
}

void UITextInput::selectAll() {
    m_selectionStart = 0;
    m_selectionEnd = m_text.length();
    m_cursorPos = m_selectionEnd;
}

void UITextInput::clearSelection() {
    m_selectionStart = m_selectionEnd = m_cursorPos;
}

std::string UITextInput::getSelectedText() const {
    if (m_selectionStart == m_selectionEnd) return "";

    size_t start = std::min(m_selectionStart, m_selectionEnd);
    size_t end = std::max(m_selectionStart, m_selectionEnd);
    return m_text.substr(start, end - start);
}

void UITextInput::setCursorPosition(size_t pos) {
    m_cursorPos = std::min(pos, m_text.length());
}

void UITextInput::insertText(const std::string& text) {
    if (m_readOnly) return;

    // Delete selection first
    if (m_selectionStart != m_selectionEnd) {
        deleteSelection();
    }

    // Check max length
    if (m_text.length() + text.length() > m_maxLength) {
        return;
    }

    // Insert text at cursor
    m_text.insert(m_cursorPos, text);
    m_cursorPos += text.length();
    m_selectionStart = m_selectionEnd = m_cursorPos;

    updateDisplayText();

    if (m_onTextChange) {
        m_onTextChange(m_text);
    }
}

void UITextInput::deleteSelection() {
    if (m_selectionStart == m_selectionEnd) return;

    size_t start = std::min(m_selectionStart, m_selectionEnd);
    size_t end = std::max(m_selectionStart, m_selectionEnd);

    m_text.erase(start, end - start);
    m_cursorPos = start;
    m_selectionStart = m_selectionEnd = m_cursorPos;

    updateDisplayText();

    if (m_onTextChange) {
        m_onTextChange(m_text);
    }
}

void UITextInput::updateDisplayText() {
    if (m_password) {
        m_displayText = std::string(m_text.length(), '*');
    } else {
        m_displayText = m_text;
    }
}

void UITextInput::drawSelf() {
    extern Graphics& g_graphics;
    Rect absRect = getAbsoluteRect();

    // Draw background
    UIWidget::drawSelf();

    int textX = absRect.x + m_paddingLeft + 4;
    int textY = absRect.y + m_paddingTop + (absRect.height - m_paddingTop - m_paddingBottom - 14) / 2;
    int contentWidth = absRect.width - m_paddingLeft - m_paddingRight - 8;

    // Draw placeholder or text
    if (m_text.empty() && !m_placeholder.empty() && !m_focused) {
        Color phColor = m_placeholderColor;
        phColor.a = static_cast<uint8_t>(phColor.a * m_opacity);
        g_graphics.drawText(m_placeholder, textX, textY, phColor, 11);
    } else if (!m_displayText.empty()) {
        // Draw selection background
        if (m_focused && m_selectionStart != m_selectionEnd) {
            size_t start = std::min(m_selectionStart, m_selectionEnd);
            size_t end = std::max(m_selectionStart, m_selectionEnd);

            std::string beforeSel = m_displayText.substr(0, start);
            std::string selection = m_displayText.substr(start, end - start);

            Size beforeSize = g_graphics.measureText(beforeSel, 11);
            Size selSize = g_graphics.measureText(selection, 11);

            Rect selRect{textX + beforeSize.width - m_scrollOffset, textY,
                        selSize.width, 14};
            g_graphics.drawFilledRect(selRect, m_selectionColor);
        }

        // Draw text
        Color txtColor = m_textColor;
        txtColor.a = static_cast<uint8_t>(txtColor.a * m_opacity);
        g_graphics.drawText(m_displayText, textX - m_scrollOffset, textY, txtColor, 11);

        // Draw cursor
        if (m_focused && m_showCursor) {
            std::string beforeCursor = m_displayText.substr(0, m_cursorPos);
            Size beforeSize = g_graphics.measureText(beforeCursor, 11);

            int cursorX = textX + beforeSize.width - m_scrollOffset;
            Rect cursorRect{cursorX, textY, 1, 14};
            g_graphics.drawFilledRect(cursorRect, m_cursorColor);
        }
    } else if (m_focused && m_showCursor) {
        // Draw cursor at start for empty text
        Rect cursorRect{textX, textY, 1, 14};
        g_graphics.drawFilledRect(cursorRect, m_cursorColor);
    }
}

bool UITextInput::onKeyDown(int keyCode, int modifiers) {
    if (!m_enabled || !m_visible || !m_focused) return false;

    bool ctrl = (modifiers & MOD_CTRL) != 0;
    bool shift = (modifiers & MOD_SHIFT) != 0;

    switch (keyCode) {
        case KEY_BACKSPACE:
            if (m_readOnly) return true;
            if (m_selectionStart != m_selectionEnd) {
                deleteSelection();
            } else if (m_cursorPos > 0) {
                m_text.erase(m_cursorPos - 1, 1);
                m_cursorPos--;
                m_selectionStart = m_selectionEnd = m_cursorPos;
                updateDisplayText();
                if (m_onTextChange) m_onTextChange(m_text);
            }
            return true;

        case KEY_DELETE:
            if (m_readOnly) return true;
            if (m_selectionStart != m_selectionEnd) {
                deleteSelection();
            } else if (m_cursorPos < m_text.length()) {
                m_text.erase(m_cursorPos, 1);
                updateDisplayText();
                if (m_onTextChange) m_onTextChange(m_text);
            }
            return true;

        case KEY_LEFT:
            if (ctrl) {
                // Move to previous word
                while (m_cursorPos > 0 && m_text[m_cursorPos - 1] == ' ') m_cursorPos--;
                while (m_cursorPos > 0 && m_text[m_cursorPos - 1] != ' ') m_cursorPos--;
            } else if (m_cursorPos > 0) {
                m_cursorPos--;
            }
            if (!shift) {
                m_selectionStart = m_selectionEnd = m_cursorPos;
            } else {
                m_selectionEnd = m_cursorPos;
            }
            return true;

        case KEY_RIGHT:
            if (ctrl) {
                // Move to next word
                while (m_cursorPos < m_text.length() && m_text[m_cursorPos] != ' ') m_cursorPos++;
                while (m_cursorPos < m_text.length() && m_text[m_cursorPos] == ' ') m_cursorPos++;
            } else if (m_cursorPos < m_text.length()) {
                m_cursorPos++;
            }
            if (!shift) {
                m_selectionStart = m_selectionEnd = m_cursorPos;
            } else {
                m_selectionEnd = m_cursorPos;
            }
            return true;

        case KEY_HOME:
            m_cursorPos = 0;
            if (!shift) {
                m_selectionStart = m_selectionEnd = m_cursorPos;
            } else {
                m_selectionEnd = m_cursorPos;
            }
            return true;

        case KEY_END:
            m_cursorPos = m_text.length();
            if (!shift) {
                m_selectionStart = m_selectionEnd = m_cursorPos;
            } else {
                m_selectionEnd = m_cursorPos;
            }
            return true;

        case KEY_RETURN:
            if (m_onSubmit) {
                m_onSubmit(m_text);
            }
            return true;

        case KEY_ESCAPE:
            clearFocus();
            return true;

        case 'a':
        case 'A':
            if (ctrl) {
                selectAll();
                return true;
            }
            break;

        case 'c':
        case 'C':
            if (ctrl) {
                // Copy to clipboard
                g_platform.setClipboardText(getSelectedText());
                return true;
            }
            break;

        case 'v':
        case 'V':
            if (ctrl && !m_readOnly) {
                // Paste from clipboard
                insertText(g_platform.getClipboardText());
                return true;
            }
            break;

        case 'x':
        case 'X':
            if (ctrl && !m_readOnly) {
                // Cut
                g_platform.setClipboardText(getSelectedText());
                deleteSelection();
                return true;
            }
            break;
    }

    return false;
}

bool UITextInput::onTextInput(const std::string& text) {
    if (!m_enabled || !m_visible || !m_focused || m_readOnly) return false;

    insertText(text);
    return true;
}

bool UITextInput::onMouseDown(int x, int y, int button) {
    if (!m_enabled || !m_visible) return false;

    Rect absRect = getAbsoluteRect();
    if (absRect.contains(x, y)) {
        focus();

        // Calculate cursor position from mouse
        extern Graphics& g_graphics;
        int relX = x - absRect.x - m_paddingLeft - 4 + m_scrollOffset;

        size_t pos = 0;
        int charX = 0;
        for (size_t i = 0; i < m_displayText.length(); i++) {
            Size charSize = g_graphics.measureText(m_displayText.substr(i, 1), 11);
            if (relX < charX + charSize.width / 2) {
                break;
            }
            charX += charSize.width;
            pos = i + 1;
        }

        m_cursorPos = pos;
        m_selectionStart = m_selectionEnd = m_cursorPos;

        return true;
    }

    return false;
}

void UITextInput::onFocusGain() {
    m_showCursor = true;
    m_cursorBlinkTime = 0;
}

void UITextInput::onFocusLoss() {
    m_showCursor = false;
    clearSelection();
}

} // namespace framework
} // namespace shadow
