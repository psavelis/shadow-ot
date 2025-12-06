/**
 * Shadow OT Client - UI ComboBox
 *
 * Dropdown selection widget.
 */

#pragma once

#include "uiwidget.h"
#include <string>
#include <vector>
#include <any>

namespace shadow {
namespace framework {

class UIComboBox : public UIWidget {
public:
    UIComboBox();

    struct Option {
        std::string text;
        std::any data;
    };

    void addOption(const std::string& text, std::any data = {});
    void removeOption(int index);
    void clearOptions();

    void setCurrentIndex(int index);
    int getCurrentIndex() const { return m_currentIndex; }

    void setCurrentOption(const std::string& text);
    const Option& getCurrentOption() const;

    const std::vector<Option>& getOptions() const { return m_options; }
    int getOptionCount() const { return static_cast<int>(m_options.size()); }

    // Dropdown control
    void openDropdown();
    void closeDropdown();
    bool isDropdownOpen() const { return m_dropdownOpen; }

    // Callbacks
    using OptionChangeCallback = std::function<void(int, const Option&)>;
    void setOnOptionChange(OptionChangeCallback cb) { m_onOptionChange = cb; }

    void drawSelf() override;

    bool onMouseDown(int x, int y, int button) override;
    bool onKeyDown(int keyCode, int modifiers) override;

private:
    std::vector<Option> m_options;
    int m_currentIndex{-1};
    bool m_dropdownOpen{false};
    int m_hoveredOption{-1};
    int m_maxVisibleOptions{8};

    Color m_dropdownColor{45, 45, 45};
    Color m_optionHoverColor{70, 70, 70};

    OptionChangeCallback m_onOptionChange;
};

// Alias for OTUI compatibility
using ComboBox = UIComboBox;

} // namespace framework
} // namespace shadow
