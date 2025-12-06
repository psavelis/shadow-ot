/**
 * Shadow OT Client - UI ComboBox Implementation
 */

#include "uicombobox.h"
#include <framework/graphics/graphics.h>
#include <algorithm>

// Key codes
#define KEY_UP 1073741906
#define KEY_DOWN 1073741905
#define KEY_RETURN 13
#define KEY_ESCAPE 27

namespace shadow {
namespace framework {

UIComboBox::UIComboBox() : UIWidget() {
    m_backgroundColor = Color{40, 40, 40, 255};
    m_borderColor = Color{80, 80, 80, 255};
    m_borderWidth = 1;
}

void UIComboBox::addOption(const std::string& text, std::any data) {
    m_options.push_back({text, data});
    if (m_currentIndex < 0) {
        m_currentIndex = 0;
    }
}

void UIComboBox::removeOption(int index) {
    if (index >= 0 && index < static_cast<int>(m_options.size())) {
        m_options.erase(m_options.begin() + index);
        if (m_currentIndex >= static_cast<int>(m_options.size())) {
            m_currentIndex = static_cast<int>(m_options.size()) - 1;
        }
    }
}

void UIComboBox::clearOptions() {
    m_options.clear();
    m_currentIndex = -1;
}

void UIComboBox::setCurrentIndex(int index) {
    if (index >= -1 && index < static_cast<int>(m_options.size())) {
        if (m_currentIndex != index) {
            m_currentIndex = index;
            if (m_onOptionChange && m_currentIndex >= 0) {
                m_onOptionChange(m_currentIndex, m_options[m_currentIndex]);
            }
        }
    }
}

void UIComboBox::setCurrentOption(const std::string& text) {
    for (int i = 0; i < static_cast<int>(m_options.size()); i++) {
        if (m_options[i].text == text) {
            setCurrentIndex(i);
            return;
        }
    }
}

const UIComboBox::Option& UIComboBox::getCurrentOption() const {
    static Option emptyOption{};
    if (m_currentIndex >= 0 && m_currentIndex < static_cast<int>(m_options.size())) {
        return m_options[m_currentIndex];
    }
    return emptyOption;
}

void UIComboBox::openDropdown() {
    if (m_options.empty()) return;
    m_dropdownOpen = true;
    m_hoveredOption = m_currentIndex;
}

void UIComboBox::closeDropdown() {
    m_dropdownOpen = false;
    m_hoveredOption = -1;
}

void UIComboBox::drawSelf() {
    extern Graphics& g_graphics;
    Rect absRect = getAbsoluteRect();

    // Draw main button
    Color bgColor = m_backgroundColor;
    bgColor.a = static_cast<uint8_t>(bgColor.a * m_opacity);
    g_graphics.drawFilledRect(absRect, bgColor);

    // Draw current selection text
    if (m_currentIndex >= 0 && m_currentIndex < static_cast<int>(m_options.size())) {
        Color textColor{255, 255, 255, 255};
        textColor.a = static_cast<uint8_t>(textColor.a * m_opacity);

        int textX = absRect.x + m_paddingLeft + 4;
        int textY = absRect.y + (absRect.height - 12) / 2;
        g_graphics.drawText(m_options[m_currentIndex].text, textX, textY, textColor, 11);
    }

    // Draw dropdown arrow
    {
        int arrowX = absRect.x + absRect.width - 16;
        int arrowY = absRect.y + (absRect.height - 10) / 2;
        Color arrowColor{180, 180, 180, 255};
        arrowColor.a = static_cast<uint8_t>(arrowColor.a * m_opacity);
        g_graphics.drawText(m_dropdownOpen ? "^" : "v", arrowX, arrowY, arrowColor, 10);
    }

    // Draw border
    if (m_borderWidth > 0) {
        Color borderColor = m_borderColor;
        borderColor.a = static_cast<uint8_t>(borderColor.a * m_opacity);
        g_graphics.drawRect(absRect, borderColor);
    }

    // Draw dropdown if open
    if (m_dropdownOpen && !m_options.empty()) {
        int visibleOptions = std::min(m_maxVisibleOptions, static_cast<int>(m_options.size()));
        int optionHeight = 22;
        int dropdownHeight = visibleOptions * optionHeight;

        Rect dropdownRect{absRect.x, absRect.y + absRect.height,
                          absRect.width, dropdownHeight};

        // Draw dropdown background
        Color dropdownBg = m_dropdownColor;
        dropdownBg.a = static_cast<uint8_t>(dropdownBg.a * m_opacity);
        g_graphics.drawFilledRect(dropdownRect, dropdownBg);

        // Draw options
        for (int i = 0; i < visibleOptions && i < static_cast<int>(m_options.size()); i++) {
            Rect optionRect{dropdownRect.x, dropdownRect.y + i * optionHeight,
                            dropdownRect.width, optionHeight};

            // Highlight hovered/selected option
            if (i == m_hoveredOption) {
                Color hoverColor = m_optionHoverColor;
                hoverColor.a = static_cast<uint8_t>(hoverColor.a * m_opacity);
                g_graphics.drawFilledRect(optionRect, hoverColor);
            }

            // Draw option text
            Color textColor{255, 255, 255, 255};
            if (i == m_currentIndex) {
                textColor = Color{100, 200, 255, 255}; // Highlight current
            }
            textColor.a = static_cast<uint8_t>(textColor.a * m_opacity);

            int textX = optionRect.x + 4;
            int textY = optionRect.y + (optionHeight - 12) / 2;
            g_graphics.drawText(m_options[i].text, textX, textY, textColor, 11);
        }

        // Draw dropdown border
        Color dropdownBorder = m_borderColor;
        dropdownBorder.a = static_cast<uint8_t>(dropdownBorder.a * m_opacity);
        g_graphics.drawRect(dropdownRect, dropdownBorder);
    }
}

bool UIComboBox::onMouseDown(int x, int y, int button) {
    if (!m_enabled || !m_visible) return false;

    Rect absRect = getAbsoluteRect();

    // Check dropdown options
    if (m_dropdownOpen) {
        int visibleOptions = std::min(m_maxVisibleOptions, static_cast<int>(m_options.size()));
        int optionHeight = 22;

        Rect dropdownRect{absRect.x, absRect.y + absRect.height,
                          absRect.width, visibleOptions * optionHeight};

        if (dropdownRect.contains(x, y)) {
            int optionIndex = (y - dropdownRect.y) / optionHeight;
            if (optionIndex >= 0 && optionIndex < static_cast<int>(m_options.size())) {
                setCurrentIndex(optionIndex);
                closeDropdown();
                return true;
            }
        }

        // Click outside dropdown closes it
        closeDropdown();
    }

    // Check main button
    if (absRect.contains(x, y)) {
        focus();
        if (m_dropdownOpen) {
            closeDropdown();
        } else {
            openDropdown();
        }
        return true;
    }

    return false;
}

bool UIComboBox::onKeyDown(int keyCode, int modifiers) {
    if (!m_enabled || !m_visible || !m_focused) return false;

    switch (keyCode) {
        case KEY_DOWN:
            if (m_dropdownOpen) {
                m_hoveredOption = std::min(m_hoveredOption + 1,
                                           static_cast<int>(m_options.size()) - 1);
            } else {
                setCurrentIndex(std::min(m_currentIndex + 1,
                                        static_cast<int>(m_options.size()) - 1));
            }
            return true;

        case KEY_UP:
            if (m_dropdownOpen) {
                m_hoveredOption = std::max(m_hoveredOption - 1, 0);
            } else {
                setCurrentIndex(std::max(m_currentIndex - 1, 0));
            }
            return true;

        case KEY_RETURN:
            if (m_dropdownOpen) {
                if (m_hoveredOption >= 0) {
                    setCurrentIndex(m_hoveredOption);
                }
                closeDropdown();
            } else {
                openDropdown();
            }
            return true;

        case KEY_ESCAPE:
            if (m_dropdownOpen) {
                closeDropdown();
                return true;
            }
            break;
    }

    return false;
}

} // namespace framework
} // namespace shadow
