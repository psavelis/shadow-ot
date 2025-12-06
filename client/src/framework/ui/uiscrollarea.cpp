/**
 * Shadow OT Client - UI Scroll Area Implementation
 */

#include "uiscrollarea.h"
#include <framework/graphics/graphics.h>
#include <algorithm>

namespace shadow {
namespace framework {

// UIScrollBar Implementation

UIScrollBar::UIScrollBar() : UIWidget() {
    m_backgroundColor = m_trackColor;
}

void UIScrollBar::setRange(int min, int max) {
    m_minimum = min;
    m_maximum = max;
    if (m_value < m_minimum) m_value = m_minimum;
    if (m_value > m_maximum) m_value = m_maximum;
    updateSliderRect();
}

void UIScrollBar::setValue(int value) {
    int newValue = std::clamp(value, m_minimum, m_maximum);
    if (m_value != newValue) {
        m_value = newValue;
        updateSliderRect();

        if (m_onValueChange) {
            m_onValueChange(m_value);
        }
    }
}

void UIScrollBar::updateSliderRect() {
    Rect absRect = getAbsoluteRect();
    int range = m_maximum - m_minimum;

    if (range <= 0) {
        // No scrolling needed - slider fills track
        m_sliderRect = absRect;
        return;
    }

    if (m_orientation == Orientation::Vertical) {
        int trackHeight = absRect.height;
        int sliderHeight = std::max(20, trackHeight * m_pageStep / (range + m_pageStep));
        int scrollableHeight = trackHeight - sliderHeight;
        int sliderY = absRect.y + scrollableHeight * m_value / range;

        m_sliderRect = {absRect.x, sliderY, absRect.width, sliderHeight};
    } else {
        int trackWidth = absRect.width;
        int sliderWidth = std::max(20, trackWidth * m_pageStep / (range + m_pageStep));
        int scrollableWidth = trackWidth - sliderWidth;
        int sliderX = absRect.x + scrollableWidth * m_value / range;

        m_sliderRect = {sliderX, absRect.y, sliderWidth, absRect.height};
    }
}

void UIScrollBar::drawSelf() {
    extern Graphics& g_graphics;
    Rect absRect = getAbsoluteRect();

    // Draw track
    Color trackColor = m_trackColor;
    trackColor.a = static_cast<uint8_t>(trackColor.a * m_opacity);
    g_graphics.drawFilledRect(absRect, trackColor);

    // Draw slider
    updateSliderRect();
    Color sliderColor = m_hovered || m_dragging ? m_sliderHoverColor : m_sliderColor;
    sliderColor.a = static_cast<uint8_t>(sliderColor.a * m_opacity);
    g_graphics.drawFilledRect(m_sliderRect, sliderColor);
}

bool UIScrollBar::onMouseDown(int x, int y, int button) {
    if (!m_enabled || !m_visible) return false;

    Rect absRect = getAbsoluteRect();
    if (!absRect.contains(x, y)) return false;

    if (m_sliderRect.contains(x, y)) {
        // Start dragging slider
        m_dragging = true;
        m_dragOffset = (m_orientation == Orientation::Vertical) ?
                       y - m_sliderRect.y : x - m_sliderRect.x;
    } else {
        // Click on track - page scroll
        int range = m_maximum - m_minimum;
        if (range <= 0) return true;

        if (m_orientation == Orientation::Vertical) {
            if (y < m_sliderRect.y) {
                setValue(m_value - m_pageStep);
            } else {
                setValue(m_value + m_pageStep);
            }
        } else {
            if (x < m_sliderRect.x) {
                setValue(m_value - m_pageStep);
            } else {
                setValue(m_value + m_pageStep);
            }
        }
    }

    return true;
}

bool UIScrollBar::onMouseUp(int x, int y, int button) {
    m_dragging = false;
    return UIWidget::onMouseUp(x, y, button);
}

bool UIScrollBar::onMouseMove(int x, int y) {
    UIWidget::onMouseMove(x, y);

    if (!m_dragging) return false;

    Rect absRect = getAbsoluteRect();
    int range = m_maximum - m_minimum;
    if (range <= 0) return true;

    if (m_orientation == Orientation::Vertical) {
        int sliderHeight = m_sliderRect.height;
        int scrollableHeight = absRect.height - sliderHeight;
        if (scrollableHeight <= 0) return true;

        int newSliderY = y - m_dragOffset - absRect.y;
        int newValue = m_minimum + newSliderY * range / scrollableHeight;
        setValue(newValue);
    } else {
        int sliderWidth = m_sliderRect.width;
        int scrollableWidth = absRect.width - sliderWidth;
        if (scrollableWidth <= 0) return true;

        int newSliderX = x - m_dragOffset - absRect.x;
        int newValue = m_minimum + newSliderX * range / scrollableWidth;
        setValue(newValue);
    }

    return true;
}

bool UIScrollBar::onMouseWheel(int x, int y, int delta) {
    if (!m_enabled || !m_visible) return false;

    Rect absRect = getAbsoluteRect();
    if (!absRect.contains(x, y)) return false;

    setValue(m_value - delta * m_step);
    return true;
}

// UIScrollablePanel Implementation

UIScrollablePanel::UIScrollablePanel() : UIWidget() {
}

void UIScrollablePanel::setVerticalScrollBar(UIScrollBar* scrollBar) {
    m_verticalScrollBar = scrollBar;
    if (scrollBar) {
        scrollBar->setOnValueChange([this](int value) {
            m_scrollY = value;
        });
    }
}

void UIScrollablePanel::setHorizontalScrollBar(UIScrollBar* scrollBar) {
    m_horizontalScrollBar = scrollBar;
    if (scrollBar) {
        scrollBar->setOnValueChange([this](int value) {
            m_scrollX = value;
        });
    }
}

void UIScrollablePanel::setScrollX(int x) {
    m_scrollX = x;
    if (m_horizontalScrollBar) {
        m_horizontalScrollBar->setValue(x);
    }
}

void UIScrollablePanel::setScrollY(int y) {
    m_scrollY = y;
    if (m_verticalScrollBar) {
        m_verticalScrollBar->setValue(y);
    }
}

void UIScrollablePanel::scrollToTop() {
    setScrollY(0);
}

void UIScrollablePanel::scrollToBottom() {
    updateScrollBars();
    if (m_verticalScrollBar) {
        setScrollY(m_verticalScrollBar->getMaximum());
    }
}

void UIScrollablePanel::scrollToWidget(UIWidgetPtr widget) {
    if (!widget) return;

    Rect widgetRect = widget->getRect();
    ensureChildVisible(widget);
}

void UIScrollablePanel::ensureChildVisible(UIWidgetPtr child) {
    if (!child) return;

    Rect childRect = child->getRect();
    int viewHeight = m_rect.height - m_paddingTop - m_paddingBottom;
    int viewWidth = m_rect.width - m_paddingLeft - m_paddingRight;

    // Vertical scroll
    if (childRect.y < m_scrollY) {
        setScrollY(childRect.y);
    } else if (childRect.y + childRect.height > m_scrollY + viewHeight) {
        setScrollY(childRect.y + childRect.height - viewHeight);
    }

    // Horizontal scroll
    if (childRect.x < m_scrollX) {
        setScrollX(childRect.x);
    } else if (childRect.x + childRect.width > m_scrollX + viewWidth) {
        setScrollX(childRect.x + childRect.width - viewWidth);
    }
}

void UIScrollablePanel::draw() {
    if (!m_visible || m_opacity <= 0.0f) return;

    extern Graphics& g_graphics;

    drawSelf();

    // Set up clipping for children
    Rect absRect = getAbsoluteRect();
    Rect clipRect{absRect.x + m_paddingLeft, absRect.y + m_paddingTop,
                  absRect.width - m_paddingLeft - m_paddingRight,
                  absRect.height - m_paddingTop - m_paddingBottom};

    g_graphics.pushClipRect(clipRect);

    // Draw children with scroll offset
    for (auto& child : m_children) {
        if (!child->isVisible()) continue;

        // Temporarily offset child position
        Point originalPos = child->getPosition();
        child->setPosition(originalPos.x - m_scrollX, originalPos.y - m_scrollY);

        child->draw();

        // Restore position
        child->setPosition(originalPos.x, originalPos.y);
    }

    g_graphics.popClipRect();
}

bool UIScrollablePanel::onMouseWheel(int x, int y, int delta) {
    Rect absRect = getAbsoluteRect();
    if (!absRect.contains(x, y)) return false;

    // Scroll vertically
    setScrollY(m_scrollY - delta * 20);

    // Clamp scroll
    if (m_verticalScrollBar) {
        m_scrollY = std::clamp(m_scrollY, 0, m_verticalScrollBar->getMaximum());
        m_verticalScrollBar->setValue(m_scrollY);
    }

    return true;
}

void UIScrollablePanel::updateGeometry() {
    UIWidget::updateGeometry();
    updateScrollBars();
}

void UIScrollablePanel::updateScrollBars() {
    // Calculate content bounds
    int maxY = 0;
    int maxX = 0;

    for (auto& child : m_children) {
        if (!child->isVisible()) continue;

        Rect childRect = child->getRect();
        maxY = std::max(maxY, childRect.y + childRect.height);
        maxX = std::max(maxX, childRect.x + childRect.width);
    }

    int viewHeight = m_rect.height - m_paddingTop - m_paddingBottom;
    int viewWidth = m_rect.width - m_paddingLeft - m_paddingRight;

    if (m_verticalScrollBar) {
        int contentHeight = std::max(maxY, viewHeight);
        m_verticalScrollBar->setRange(0, std::max(0, contentHeight - viewHeight));
        m_verticalScrollBar->setPageStep(viewHeight);
    }

    if (m_horizontalScrollBar) {
        int contentWidth = std::max(maxX, viewWidth);
        m_horizontalScrollBar->setRange(0, std::max(0, contentWidth - viewWidth));
        m_horizontalScrollBar->setPageStep(viewWidth);
    }
}

} // namespace framework
} // namespace shadow
