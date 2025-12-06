/**
 * Shadow OT Client - UI Scroll Area
 *
 * Scrollable container widget.
 */

#pragma once

#include "uiwidget.h"

namespace shadow {
namespace framework {

class UIScrollBar : public UIWidget {
public:
    UIScrollBar();

    enum class Orientation { Vertical, Horizontal };
    void setOrientation(Orientation orientation) { m_orientation = orientation; }
    Orientation getOrientation() const { return m_orientation; }

    void setRange(int min, int max);
    int getMinimum() const { return m_minimum; }
    int getMaximum() const { return m_maximum; }

    void setValue(int value);
    int getValue() const { return m_value; }

    void setStep(int step) { m_step = step; }
    int getStep() const { return m_step; }

    void setPageStep(int step) { m_pageStep = step; }
    int getPageStep() const { return m_pageStep; }

    void setPixelsScroll(bool pixelsScroll) { m_pixelsScroll = pixelsScroll; }

    using ValueChangeCallback = std::function<void(int)>;
    void setOnValueChange(ValueChangeCallback cb) { m_onValueChange = cb; }

    void drawSelf() override;

    bool onMouseDown(int x, int y, int button) override;
    bool onMouseUp(int x, int y, int button) override;
    bool onMouseMove(int x, int y) override;
    bool onMouseWheel(int x, int y, int delta) override;

private:
    void updateSliderRect();

    Orientation m_orientation{Orientation::Vertical};
    int m_minimum{0};
    int m_maximum{100};
    int m_value{0};
    int m_step{1};
    int m_pageStep{10};
    bool m_pixelsScroll{false};

    Rect m_sliderRect;
    bool m_dragging{false};
    int m_dragOffset{0};

    Color m_trackColor{30, 30, 30};
    Color m_sliderColor{80, 80, 80};
    Color m_sliderHoverColor{100, 100, 100};

    ValueChangeCallback m_onValueChange;
};

class UIScrollablePanel : public UIWidget {
public:
    UIScrollablePanel();

    void setVerticalScrollBar(UIScrollBar* scrollBar);
    void setHorizontalScrollBar(UIScrollBar* scrollBar);

    void setScrollX(int x);
    void setScrollY(int y);
    int getScrollX() const { return m_scrollX; }
    int getScrollY() const { return m_scrollY; }

    void scrollToTop();
    void scrollToBottom();
    void scrollToWidget(UIWidgetPtr widget);

    void ensureChildVisible(UIWidgetPtr child);

    void draw() override;
    bool onMouseWheel(int x, int y, int delta) override;

protected:
    void updateGeometry() override;
    void updateScrollBars();

private:
    UIScrollBar* m_verticalScrollBar{nullptr};
    UIScrollBar* m_horizontalScrollBar{nullptr};
    int m_scrollX{0};
    int m_scrollY{0};
};

using ScrollablePanel = UIScrollablePanel;
using VerticalScrollBar = UIScrollBar;

} // namespace framework
} // namespace shadow
