/**
 * Shadow OT Client - UI Text Input
 *
 * Single-line text input widget.
 */

#pragma once

#include "uiwidget.h"
#include <string>

namespace shadow {
namespace framework {

class UITextInput : public UIWidget {
public:
    UITextInput();

    void setText(const std::string& text);
    const std::string& getText() const { return m_text; }

    void setPlaceholder(const std::string& text) { m_placeholder = text; }
    const std::string& getPlaceholder() const { return m_placeholder; }

    void setMaxLength(size_t length) { m_maxLength = length; }
    size_t getMaxLength() const { return m_maxLength; }

    void setPassword(bool password) { m_password = password; }
    bool isPassword() const { return m_password; }

    void setReadOnly(bool readOnly) { m_readOnly = readOnly; }
    bool isReadOnly() const { return m_readOnly; }

    void selectAll();
    void clearSelection();
    std::string getSelectedText() const;

    void setCursorPosition(size_t pos);
    size_t getCursorPosition() const { return m_cursorPos; }

    // Colors
    void setTextColor(const Color& color) { m_textColor = color; }
    void setPlaceholderColor(const Color& color) { m_placeholderColor = color; }
    void setSelectionColor(const Color& color) { m_selectionColor = color; }

    // Callbacks
    using TextChangeCallback = std::function<void(const std::string&)>;
    using SubmitCallback = std::function<void(const std::string&)>;

    void setOnTextChange(TextChangeCallback cb) { m_onTextChange = cb; }
    void setOnSubmit(SubmitCallback cb) { m_onSubmit = cb; }

    void drawSelf() override;

    bool onKeyDown(int keyCode, int modifiers) override;
    bool onTextInput(const std::string& text) override;
    bool onMouseDown(int x, int y, int button) override;
    void onFocusGain() override;
    void onFocusLoss() override;

private:
    void insertText(const std::string& text);
    void deleteSelection();
    void updateDisplayText();

    std::string m_text;
    std::string m_displayText;
    std::string m_placeholder;

    size_t m_cursorPos{0};
    size_t m_selectionStart{0};
    size_t m_selectionEnd{0};
    size_t m_maxLength{1024};
    int m_scrollOffset{0};

    bool m_password{false};
    bool m_readOnly{false};
    bool m_showCursor{true};
    double m_cursorBlinkTime{0};

    Color m_textColor{255, 255, 255};
    Color m_placeholderColor{150, 150, 150};
    Color m_selectionColor{60, 120, 180};
    Color m_cursorColor{255, 255, 255};

    TextChangeCallback m_onTextChange;
    SubmitCallback m_onSubmit;
};

} // namespace framework
} // namespace shadow
