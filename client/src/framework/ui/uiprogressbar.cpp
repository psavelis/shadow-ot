/**
 * Shadow OT Client - UI Progress Bar Implementation
 */

#include "uiprogressbar.h"
#include <framework/graphics/graphics.h>
#include <cstdio>
#include <algorithm>

namespace shadow {
namespace framework {

UIProgressBar::UIProgressBar() : UIWidget() {
    m_backgroundColor = Color{30, 30, 30, 255};
    m_borderColor = Color{60, 60, 60, 255};
    m_borderWidth = 1;
}

void UIProgressBar::setValue(int value) {
    m_value = std::clamp(value, m_minimum, m_maximum);
}

void UIProgressBar::setPercent(float percent) {
    percent = std::clamp(percent, 0.0f, 100.0f);
    m_value = m_minimum + static_cast<int>((m_maximum - m_minimum) * percent / 100.0f);
}

float UIProgressBar::getPercent() const {
    if (m_maximum == m_minimum) return 0.0f;
    return static_cast<float>(m_value - m_minimum) / (m_maximum - m_minimum) * 100.0f;
}

void UIProgressBar::drawSelf() {
    extern Graphics& g_graphics;
    Rect absRect = getAbsoluteRect();

    // Draw background
    Color bgColor = m_backgroundColor;
    bgColor.a = static_cast<uint8_t>(bgColor.a * m_opacity);
    g_graphics.drawFilledRect(absRect, bgColor);

    // Calculate fill width
    float percent = getPercent();
    int fillWidth = static_cast<int>(absRect.width * percent / 100.0f);

    if (fillWidth > 0) {
        // Draw foreground (progress fill)
        Rect fillRect{absRect.x, absRect.y, fillWidth, absRect.height};
        Color fgColor = m_foregroundColor;
        fgColor.a = static_cast<uint8_t>(fgColor.a * m_opacity);
        g_graphics.drawFilledRect(fillRect, fgColor);
    }

    // Draw border
    if (m_borderWidth > 0) {
        Color borderColor = m_borderColor;
        borderColor.a = static_cast<uint8_t>(borderColor.a * m_opacity);
        g_graphics.drawRect(absRect, borderColor);
    }

    // Draw text if enabled
    if (m_showText) {
        char text[64];
        snprintf(text, sizeof(text), m_textFormat.c_str(), static_cast<int>(percent));

        Size textSize = g_graphics.measureText(text, 10);
        int textX = absRect.x + (absRect.width - textSize.width) / 2;
        int textY = absRect.y + (absRect.height - textSize.height) / 2;

        Color textColor{255, 255, 255, 255};
        textColor.a = static_cast<uint8_t>(textColor.a * m_opacity);
        g_graphics.drawText(text, textX, textY, textColor, 10);
    }
}

} // namespace framework
} // namespace shadow
