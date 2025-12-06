/**
 * Shadow OT Client - UI Button
 *
 * Interactive button widget.
 */

#pragma once

#include "uiwidget.h"
#include <string>

namespace shadow {
namespace framework {

class UIButton : public UIWidget {
public:
    UIButton();

    void setText(const std::string& text) { m_text = text; }
    const std::string& getText() const { return m_text; }

    void setIcon(const std::string& iconPath);

    // States
    void setNormalColor(const Color& color) { m_normalColor = color; }
    void setHoverColor(const Color& color) { m_hoverColor = color; }
    void setPressedColor(const Color& color) { m_pressedColor = color; }
    void setDisabledColor(const Color& color) { m_disabledColor = color; }

    void setTextColor(const Color& color) { m_textColor = color; }
    void setTextColorHover(const Color& color) { m_textColorHover = color; }
    void setTextColorPressed(const Color& color) { m_textColorPressed = color; }
    void setTextColorDisabled(const Color& color) { m_textColorDisabled = color; }

    void drawSelf() override;

    bool onMouseDown(int x, int y, int button) override;
    bool onMouseUp(int x, int y, int button) override;
    void onMouseEnter() override;
    void onMouseLeave() override;

private:
    std::string m_text;
    std::shared_ptr<Texture> m_icon;

    Color m_normalColor{50, 50, 50};
    Color m_hoverColor{70, 70, 70};
    Color m_pressedColor{40, 40, 40};
    Color m_disabledColor{30, 30, 30};

    Color m_textColor{255, 255, 255};
    Color m_textColorHover{255, 255, 255};
    Color m_textColorPressed{200, 200, 200};
    Color m_textColorDisabled{100, 100, 100};
};

} // namespace framework
} // namespace shadow
