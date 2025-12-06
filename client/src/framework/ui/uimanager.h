/**
 * Shadow OT Client - UI Manager
 *
 * Central UI management and OTUI file loading.
 */

#pragma once

#include <string>
#include <memory>
#include <map>
#include <vector>
#include <functional>

#include "uiwidget.h"

namespace shadow {
namespace framework {

class UIManager {
public:
    static UIManager& instance();

    bool init();
    void terminate();

    // Display UI from file
    UIWidgetPtr displayUI(const std::string& name);
    UIWidgetPtr loadUI(const std::string& filename);
    UIWidgetPtr loadUIFromString(const std::string& otui);

    // Widget creation
    UIWidgetPtr createWidget(const std::string& type, UIWidgetPtr parent = nullptr);
    void registerWidgetType(const std::string& type, std::function<UIWidgetPtr()> factory);

    // Root widget
    UIWidgetPtr getRootWidget() const { return m_rootWidget; }

    // Focus management
    UIWidgetPtr getFocusedWidget() const { return m_focusedWidget; }
    void setFocusedWidget(UIWidgetPtr widget);
    void clearFocus();

    // Hover tracking
    UIWidgetPtr getHoveredWidget() const { return m_hoveredWidget; }

    // Modal windows
    void pushModal(UIWidgetPtr widget);
    void popModal();
    UIWidgetPtr getTopModal() const;
    bool hasModal() const { return !m_modalStack.empty(); }

    // Rendering
    void draw();

    // Input handling
    bool onKeyDown(int keyCode, int modifiers);
    bool onKeyUp(int keyCode, int modifiers);
    bool onKeyPress(int keyCode, int modifiers);
    bool onTextInput(const std::string& text);

    bool onMouseDown(int x, int y, int button);
    bool onMouseUp(int x, int y, int button);
    bool onMouseMove(int x, int y);
    bool onMouseWheel(int x, int y, int delta);

    // Style management
    void loadStyle(const std::string& filename);
    void setStyleProperty(const std::string& widgetType,
                         const std::string& property,
                         const std::string& value);

    // Cursor
    void setCursor(const std::string& cursor);
    void resetCursor();

    // Tooltip
    void showTooltip(const std::string& text, int x, int y);
    void hideTooltip();

    // Debug
    void setDebugDraw(bool enabled) { m_debugDraw = enabled; }
    bool isDebugDraw() const { return m_debugDraw; }

    // Animations
    void fadeIn(UIWidgetPtr widget, float duration);
    void fadeOut(UIWidgetPtr widget, float duration, bool destroyAfter = false);
    void moveTo(UIWidgetPtr widget, int x, int y, float duration);

private:
    UIManager() = default;
    ~UIManager() = default;
    UIManager(const UIManager&) = delete;
    UIManager& operator=(const UIManager&) = delete;

    void parseOTUI(const std::string& content, UIWidgetPtr parent);
    void processAnimations(float deltaTime);

    UIWidgetPtr m_rootWidget;
    UIWidgetPtr m_focusedWidget;
    UIWidgetPtr m_hoveredWidget;
    UIWidgetPtr m_pressedWidget;
    UIWidgetPtr m_tooltipWidget;

    std::vector<UIWidgetPtr> m_modalStack;
    std::map<std::string, std::function<UIWidgetPtr()>> m_widgetFactories;
    std::map<std::string, std::map<std::string, std::string>> m_styles;

    bool m_debugDraw{false};

    struct Animation {
        UIWidgetPtr widget;
        enum Type { FadeIn, FadeOut, MoveTo } type;
        float duration;
        float elapsed{0};
        float startValue;
        float endValue;
        int targetX, targetY;
        bool destroyAfter{false};
    };
    std::vector<Animation> m_animations;
};

} // namespace framework
} // namespace shadow

// Global accessor
extern shadow::framework::UIManager& g_ui;
