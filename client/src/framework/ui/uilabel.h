/**
 * Shadow OT Client - UI Label
 *
 * Text display widget.
 */

#pragma once

#include "uiwidget.h"
#include <string>

namespace shadow {
namespace framework {

class UILabel : public UIWidget {
public:
    UILabel();

    void setText(const std::string& text);
    const std::string& getText() const { return m_text; }

    void setFont(const std::string& fontName);
    void setFontSize(int size) { m_fontSize = size; }
    void setColor(const Color& color) { m_textColor = color; }
    void setColor(const std::string& hex) { m_textColor = Color::fromHex(hex); }

    enum class Align { Left, Center, Right };
    enum class VAlign { Top, Middle, Bottom };
    void setTextAlign(Align align) { m_textAlign = align; }
    void setTextVAlign(VAlign align) { m_textVAlign = align; }

    void setTextWrap(bool wrap) { m_textWrap = wrap; }
    bool isTextWrap() const { return m_textWrap; }

    void setTextAutoResize(bool autoResize) { m_autoResize = autoResize; }
    bool isTextAutoResize() const { return m_autoResize; }

    void drawSelf() override;

private:
    std::string m_text;
    std::string m_fontName{"verdana-11px"};
    int m_fontSize{11};
    Color m_textColor{255, 255, 255};
    Align m_textAlign{Align::Left};
    VAlign m_textVAlign{VAlign::Top};
    bool m_textWrap{false};
    bool m_autoResize{false};
};

} // namespace framework
} // namespace shadow
