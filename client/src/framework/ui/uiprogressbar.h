/**
 * Shadow OT Client - UI Progress Bar
 *
 * Progress/loading indicator widget.
 */

#pragma once

#include "uiwidget.h"

namespace shadow {
namespace framework {

class UIProgressBar : public UIWidget {
public:
    UIProgressBar();

    void setMinimum(int min) { m_minimum = min; }
    void setMaximum(int max) { m_maximum = max; }
    void setRange(int min, int max) { m_minimum = min; m_maximum = max; }

    int getMinimum() const { return m_minimum; }
    int getMaximum() const { return m_maximum; }

    void setValue(int value);
    int getValue() const { return m_value; }

    void setPercent(float percent);
    float getPercent() const;

    // Appearance
    void setForegroundColor(const Color& color) { m_foregroundColor = color; }
    const Color& getForegroundColor() const { return m_foregroundColor; }

    void setShowText(bool show) { m_showText = show; }
    bool isShowText() const { return m_showText; }

    void setTextFormat(const std::string& format) { m_textFormat = format; }

    void drawSelf() override;

private:
    int m_minimum{0};
    int m_maximum{100};
    int m_value{0};

    Color m_foregroundColor{46, 204, 113};
    bool m_showText{false};
    std::string m_textFormat{"%d%%"};
};

// Alias for OTUI compatibility
using ProgressBar = UIProgressBar;

} // namespace framework
} // namespace shadow
