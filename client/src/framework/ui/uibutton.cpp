/**
 * Shadow OT Client - UI Button Implementation
 */

#include "uibutton.h"
#include <framework/graphics/graphics.h>
#include <framework/core/resourcemanager.h>

namespace shadow {
namespace framework {

UIButton::UIButton() : UIWidget() {
    m_backgroundColor = m_normalColor;
}

void UIButton::setIcon(const std::string& iconPath) {
    extern ResourceManager& g_resources;
    m_icon = g_resources.loadTexture(iconPath);
}

void UIButton::drawSelf() {
    extern Graphics& g_graphics;
    Rect absRect = getAbsoluteRect();

    // Determine colors based on state
    Color bgColor;
    Color textColor;

    if (!m_enabled) {
        bgColor = m_disabledColor;
        textColor = m_textColorDisabled;
    } else if (m_pressed) {
        bgColor = m_pressedColor;
        textColor = m_textColorPressed;
    } else if (m_hovered) {
        bgColor = m_hoverColor;
        textColor = m_textColorHover;
    } else {
        bgColor = m_normalColor;
        textColor = m_textColor;
    }

    // Apply opacity
    bgColor.a = static_cast<uint8_t>(bgColor.a * m_opacity);
    textColor.a = static_cast<uint8_t>(textColor.a * m_opacity);

    // Draw background
    g_graphics.drawFilledRect(absRect, bgColor);

    // Draw border
    if (m_borderWidth > 0 && m_borderColor.a > 0) {
        Color border = m_borderColor;
        border.a = static_cast<uint8_t>(border.a * m_opacity);
        g_graphics.drawRect(absRect, border);
    }

    // Calculate content layout
    int contentX = absRect.x + m_paddingLeft;
    int contentY = absRect.y + m_paddingTop;
    int contentWidth = absRect.width - m_paddingLeft - m_paddingRight;
    int contentHeight = absRect.height - m_paddingTop - m_paddingBottom;

    int iconWidth = 0;
    int spacing = 4;

    // Draw icon if present
    if (m_icon) {
        iconWidth = m_icon->getHeight(); // Assume square icon
        int iconX = contentX;
        int iconY = contentY + (contentHeight - iconWidth) / 2;

        // Center icon if no text
        if (m_text.empty()) {
            iconX = absRect.x + (absRect.width - iconWidth) / 2;
        }

        g_graphics.drawTexture(m_icon.get(), Rect{iconX, iconY, iconWidth, iconWidth});
    }

    // Draw text
    if (!m_text.empty()) {
        Size textSize = g_graphics.measureText(m_text, 11);

        int textX = contentX;
        int textY = contentY + (contentHeight - textSize.height) / 2;

        // Adjust for icon
        if (m_icon) {
            textX += iconWidth + spacing;
            contentWidth -= iconWidth + spacing;
        }

        // Center text horizontally
        textX += (contentWidth - textSize.width) / 2;

        g_graphics.drawText(m_text, textX, textY, textColor, 11);
    }
}

bool UIButton::onMouseDown(int x, int y, int button) {
    if (!m_enabled || !m_visible) return false;

    Rect absRect = getAbsoluteRect();
    if (absRect.contains(x, y)) {
        m_pressed = true;
        focus();
        return true;
    }

    return false;
}

bool UIButton::onMouseUp(int x, int y, int button) {
    if (!m_enabled || !m_visible) return false;

    bool wasPressed = m_pressed;
    m_pressed = false;

    Rect absRect = getAbsoluteRect();
    if (absRect.contains(x, y) && wasPressed) {
        // Trigger click
        if (m_onClick) {
            m_onClick(shared_from_this());
        }
        if (onClick) {
            onClick(shared_from_this());
        }
        return true;
    }

    return false;
}

void UIButton::onMouseEnter() {
    m_hovered = true;
}

void UIButton::onMouseLeave() {
    m_hovered = false;
    m_pressed = false;
}

} // namespace framework
} // namespace shadow
