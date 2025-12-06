/**
 * Shadow OT Client - UI Window Implementation
 */

#include "uiwindow.h"
#include <framework/graphics/graphics.h>
#include <algorithm>

// Key codes
#define KEY_ESCAPE 27

namespace shadow {
namespace framework {

UIWindow::UIWindow() : UIWidget() {
    m_backgroundColor = Color{35, 35, 35, 245};
    m_borderColor = Color{60, 60, 60, 255};
    m_borderWidth = 1;
    m_paddingTop = m_titleBarHeight + 4;
    m_paddingLeft = 8;
    m_paddingRight = 8;
    m_paddingBottom = 8;
}

void UIWindow::drawSelf() {
    extern Graphics& g_graphics;
    Rect absRect = getAbsoluteRect();

    // Draw window background
    Color bgColor = m_backgroundColor;
    bgColor.a = static_cast<uint8_t>(bgColor.a * m_opacity);
    g_graphics.drawFilledRect(absRect, bgColor);

    // Draw title bar
    Rect titleRect{absRect.x, absRect.y, absRect.width, m_titleBarHeight};
    Color titleBgColor = m_titleBarColor;
    titleBgColor.a = static_cast<uint8_t>(titleBgColor.a * m_opacity);
    g_graphics.drawFilledRect(titleRect, titleBgColor);

    // Draw title text
    if (!m_title.empty()) {
        Color titleColor = m_titleColor;
        titleColor.a = static_cast<uint8_t>(titleColor.a * m_opacity);

        Size textSize = g_graphics.measureText(m_title, 11);
        int textX = absRect.x + 8;
        int textY = absRect.y + (m_titleBarHeight - textSize.height) / 2;

        g_graphics.drawText(m_title, textX, textY, titleColor, 11);
    }

    // Draw close button if closeable
    if (m_closeable) {
        Rect closeRect{absRect.x + absRect.width - m_titleBarHeight, absRect.y,
                       m_titleBarHeight, m_titleBarHeight};

        // Draw X
        Color closeColor = m_hovered ? Color{255, 100, 100, 255} : Color{180, 180, 180, 255};
        closeColor.a = static_cast<uint8_t>(closeColor.a * m_opacity);

        int cx = closeRect.x + closeRect.width / 2;
        int cy = closeRect.y + closeRect.height / 2;
        int s = 5;

        // Draw simple X using filled rects (lines would be better)
        g_graphics.drawText("X", cx - 4, cy - 6, closeColor, 11);
    }

    // Draw border
    if (m_borderWidth > 0) {
        Color borderColor = m_borderColor;
        borderColor.a = static_cast<uint8_t>(borderColor.a * m_opacity);
        g_graphics.drawRect(absRect, borderColor);
    }
}

UIWindow::DragMode UIWindow::getDragMode(int x, int y) const {
    Rect absRect = getAbsoluteRect();

    // Check if in title bar (for moving)
    Rect titleRect{absRect.x, absRect.y, absRect.width - (m_closeable ? m_titleBarHeight : 0), m_titleBarHeight};
    if (m_draggable && titleRect.contains(x, y)) {
        return DragMode::Move;
    }

    if (!m_resizable) return DragMode::None;

    // Check resize edges (8 pixel border)
    const int border = 8;
    bool atLeft = x >= absRect.x && x < absRect.x + border;
    bool atRight = x > absRect.x + absRect.width - border && x <= absRect.x + absRect.width;
    bool atTop = y >= absRect.y && y < absRect.y + border;
    bool atBottom = y > absRect.y + absRect.height - border && y <= absRect.y + absRect.height;

    if (atTop && atLeft) return DragMode::ResizeNW;
    if (atTop && atRight) return DragMode::ResizeNE;
    if (atBottom && atLeft) return DragMode::ResizeSW;
    if (atBottom && atRight) return DragMode::ResizeSE;
    if (atLeft) return DragMode::ResizeW;
    if (atRight) return DragMode::ResizeE;
    if (atTop) return DragMode::ResizeN;
    if (atBottom) return DragMode::ResizeS;

    return DragMode::None;
}

bool UIWindow::onMouseDown(int x, int y, int button) {
    if (!m_enabled || !m_visible) return false;

    Rect absRect = getAbsoluteRect();
    if (!absRect.contains(x, y)) return false;

    // Check close button
    if (m_closeable) {
        Rect closeRect{absRect.x + absRect.width - m_titleBarHeight, absRect.y,
                       m_titleBarHeight, m_titleBarHeight};
        if (closeRect.contains(x, y)) {
            // Close window
            if (!m_onClose || m_onClose()) {
                hide();
            }
            return true;
        }
    }

    // Check drag mode
    m_dragMode = getDragMode(x, y);
    if (m_dragMode != DragMode::None) {
        m_dragStartX = x;
        m_dragStartY = y;
        m_dragStartRect = m_rect;
        raise();
        return true;
    }

    // Pass to children
    return UIWidget::onMouseDown(x, y, button);
}

bool UIWindow::onMouseUp(int x, int y, int button) {
    m_dragMode = DragMode::None;
    return UIWidget::onMouseUp(x, y, button);
}

bool UIWindow::onMouseMove(int x, int y) {
    if (m_dragMode == DragMode::None) {
        return UIWidget::onMouseMove(x, y);
    }

    int dx = x - m_dragStartX;
    int dy = y - m_dragStartY;

    switch (m_dragMode) {
        case DragMode::Move:
            setPosition(m_dragStartRect.x + dx, m_dragStartRect.y + dy);
            break;

        case DragMode::ResizeE:
            setSize(std::clamp(m_dragStartRect.width + dx, m_minWidth, m_maxWidth),
                    m_rect.height);
            break;

        case DragMode::ResizeW:
            {
                int newWidth = std::clamp(m_dragStartRect.width - dx, m_minWidth, m_maxWidth);
                int actualDx = m_dragStartRect.width - newWidth;
                setPosition(m_dragStartRect.x + actualDx, m_rect.y);
                setSize(newWidth, m_rect.height);
            }
            break;

        case DragMode::ResizeS:
            setSize(m_rect.width,
                    std::clamp(m_dragStartRect.height + dy, m_minHeight, m_maxHeight));
            break;

        case DragMode::ResizeN:
            {
                int newHeight = std::clamp(m_dragStartRect.height - dy, m_minHeight, m_maxHeight);
                int actualDy = m_dragStartRect.height - newHeight;
                setPosition(m_rect.x, m_dragStartRect.y + actualDy);
                setSize(m_rect.width, newHeight);
            }
            break;

        case DragMode::ResizeSE:
            setSize(std::clamp(m_dragStartRect.width + dx, m_minWidth, m_maxWidth),
                    std::clamp(m_dragStartRect.height + dy, m_minHeight, m_maxHeight));
            break;

        case DragMode::ResizeNE:
            {
                int newHeight = std::clamp(m_dragStartRect.height - dy, m_minHeight, m_maxHeight);
                int actualDy = m_dragStartRect.height - newHeight;
                setPosition(m_rect.x, m_dragStartRect.y + actualDy);
                setSize(std::clamp(m_dragStartRect.width + dx, m_minWidth, m_maxWidth), newHeight);
            }
            break;

        case DragMode::ResizeSW:
            {
                int newWidth = std::clamp(m_dragStartRect.width - dx, m_minWidth, m_maxWidth);
                int actualDx = m_dragStartRect.width - newWidth;
                setPosition(m_dragStartRect.x + actualDx, m_rect.y);
                setSize(newWidth, std::clamp(m_dragStartRect.height + dy, m_minHeight, m_maxHeight));
            }
            break;

        case DragMode::ResizeNW:
            {
                int newWidth = std::clamp(m_dragStartRect.width - dx, m_minWidth, m_maxWidth);
                int newHeight = std::clamp(m_dragStartRect.height - dy, m_minHeight, m_maxHeight);
                int actualDx = m_dragStartRect.width - newWidth;
                int actualDy = m_dragStartRect.height - newHeight;
                setPosition(m_dragStartRect.x + actualDx, m_dragStartRect.y + actualDy);
                setSize(newWidth, newHeight);
            }
            break;

        default:
            break;
    }

    return true;
}

bool UIWindow::onKeyDown(int keyCode, int modifiers) {
    if (!m_enabled || !m_visible) return false;

    if (keyCode == KEY_ESCAPE && m_closeable) {
        if (!m_onClose || m_onClose()) {
            hide();
        }
        return true;
    }

    return UIWidget::onKeyDown(keyCode, modifiers);
}

// MainWindow implementation
MainWindow::MainWindow() : UIWindow() {
    m_titleBarColor = Color{25, 25, 28, 255};
    m_backgroundColor = Color{18, 18, 21, 250};
    m_titleColor = Color{200, 200, 200, 255};
    m_closeable = false;
    m_resizable = false;
}

} // namespace framework
} // namespace shadow
