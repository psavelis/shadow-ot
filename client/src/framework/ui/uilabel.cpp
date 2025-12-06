/**
 * Shadow OT Client - UI Label Implementation
 */

#include "uilabel.h"
#include <framework/graphics/graphics.h>

namespace shadow {
namespace framework {

UILabel::UILabel() : UIWidget() {
}

void UILabel::setText(const std::string& text) {
    m_text = text;

    if (m_autoResize) {
        // Calculate text size and resize widget
        extern Graphics& g_graphics;
        Size textSize = g_graphics.measureText(m_text, m_fontSize);
        setSize(textSize.width + m_paddingLeft + m_paddingRight,
                textSize.height + m_paddingTop + m_paddingBottom);
    }
}

void UILabel::setFont(const std::string& fontName) {
    m_fontName = fontName;
}

void UILabel::drawSelf() {
    // Draw background first
    UIWidget::drawSelf();

    if (m_text.empty()) return;

    extern Graphics& g_graphics;
    Rect absRect = getAbsoluteRect();

    // Calculate text position based on alignment
    Size textSize = g_graphics.measureText(m_text, m_fontSize);

    int textX = absRect.x + m_paddingLeft;
    int textY = absRect.y + m_paddingTop;

    int contentWidth = absRect.width - m_paddingLeft - m_paddingRight;
    int contentHeight = absRect.height - m_paddingTop - m_paddingBottom;

    // Horizontal alignment
    switch (m_textAlign) {
        case Align::Center:
            textX = absRect.x + m_paddingLeft + (contentWidth - textSize.width) / 2;
            break;
        case Align::Right:
            textX = absRect.x + absRect.width - m_paddingRight - textSize.width;
            break;
        default:
            break;
    }

    // Vertical alignment
    switch (m_textVAlign) {
        case VAlign::Middle:
            textY = absRect.y + m_paddingTop + (contentHeight - textSize.height) / 2;
            break;
        case VAlign::Bottom:
            textY = absRect.y + absRect.height - m_paddingBottom - textSize.height;
            break;
        default:
            break;
    }

    // Apply opacity to text color
    Color drawColor = m_textColor;
    drawColor.a = static_cast<uint8_t>(drawColor.a * m_opacity);

    g_graphics.drawText(m_text, textX, textY, drawColor, m_fontSize);
}

} // namespace framework
} // namespace shadow
