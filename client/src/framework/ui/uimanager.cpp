/**
 * Shadow OT Client - UI Manager Implementation
 */

#include "uimanager.h"
#include "uilabel.h"
#include "uibutton.h"
#include "uitextinput.h"
#include "uiwindow.h"
#include "uiscrollarea.h"
#include "uiprogressbar.h"
#include "uicombobox.h"
#include <framework/core/resourcemanager.h>
#include <framework/graphics/graphics.h>
#include <framework/platform/platform.h>
#include <sstream>
#include <algorithm>

namespace shadow {
namespace framework {

UIManager& UIManager::instance() {
    static UIManager instance;
    return instance;
}

bool UIManager::init() {
    // Create root widget
    m_rootWidget = std::make_shared<UIWidget>();
    m_rootWidget->setId("root");

    // Register built-in widget types
    registerWidgetType("UIWidget", []() { return std::make_shared<UIWidget>(); });
    registerWidgetType("UILabel", []() { return std::make_shared<UILabel>(); });
    registerWidgetType("UIButton", []() { return std::make_shared<UIButton>(); });
    registerWidgetType("UITextInput", []() { return std::make_shared<UITextInput>(); });
    registerWidgetType("UIWindow", []() { return std::make_shared<UIWindow>(); });
    registerWidgetType("MainWindow", []() { return std::make_shared<MainWindow>(); });
    registerWidgetType("UIScrollBar", []() { return std::make_shared<UIScrollBar>(); });
    registerWidgetType("UIScrollablePanel", []() { return std::make_shared<UIScrollablePanel>(); });
    registerWidgetType("UIProgressBar", []() { return std::make_shared<UIProgressBar>(); });
    registerWidgetType("UIComboBox", []() { return std::make_shared<UIComboBox>(); });

    // Aliases for OTUI compatibility
    registerWidgetType("Label", []() { return std::make_shared<UILabel>(); });
    registerWidgetType("Button", []() { return std::make_shared<UIButton>(); });
    registerWidgetType("TextEdit", []() { return std::make_shared<UITextInput>(); });
    registerWidgetType("Window", []() { return std::make_shared<UIWindow>(); });
    registerWidgetType("ScrollablePanel", []() { return std::make_shared<UIScrollablePanel>(); });
    registerWidgetType("ProgressBar", []() { return std::make_shared<UIProgressBar>(); });
    registerWidgetType("ComboBox", []() { return std::make_shared<UIComboBox>(); });

    return true;
}

void UIManager::terminate() {
    m_animations.clear();
    m_modalStack.clear();
    m_focusedWidget = nullptr;
    m_hoveredWidget = nullptr;
    m_pressedWidget = nullptr;

    if (m_rootWidget) {
        m_rootWidget->destroyChildren();
        m_rootWidget = nullptr;
    }
}

UIWidgetPtr UIManager::displayUI(const std::string& name) {
    return loadUI("data/ui/" + name + ".otui");
}

UIWidgetPtr UIManager::loadUI(const std::string& filename) {
    extern ResourceManager& g_resources;
    std::string content = g_resources.readFileText(filename);

    if (content.empty()) {
        return nullptr;
    }

    return loadUIFromString(content);
}

UIWidgetPtr UIManager::loadUIFromString(const std::string& otui) {
    auto container = std::make_shared<UIWidget>();
    parseOTUI(otui, container);

    if (!container->getChildren().empty()) {
        auto widget = container->getChildByIndex(0);
        container->removeChild(widget);
        m_rootWidget->addChild(widget);
        return widget;
    }

    return nullptr;
}

UIWidgetPtr UIManager::createWidget(const std::string& type, UIWidgetPtr parent) {
    auto it = m_widgetFactories.find(type);
    if (it == m_widgetFactories.end()) {
        return nullptr;
    }

    UIWidgetPtr widget = it->second();

    if (parent) {
        parent->addChild(widget);
    } else if (m_rootWidget) {
        m_rootWidget->addChild(widget);
    }

    return widget;
}

void UIManager::registerWidgetType(const std::string& type, std::function<UIWidgetPtr()> factory) {
    m_widgetFactories[type] = factory;
}

void UIManager::setFocusedWidget(UIWidgetPtr widget) {
    if (m_focusedWidget == widget) return;

    if (m_focusedWidget) {
        m_focusedWidget->clearFocus();
    }

    m_focusedWidget = widget;

    if (m_focusedWidget) {
        m_focusedWidget->focus();
    }
}

void UIManager::clearFocus() {
    setFocusedWidget(nullptr);
}

void UIManager::pushModal(UIWidgetPtr widget) {
    m_modalStack.push_back(widget);
    widget->raise();
    setFocusedWidget(widget);
}

void UIManager::popModal() {
    if (!m_modalStack.empty()) {
        m_modalStack.pop_back();

        if (!m_modalStack.empty()) {
            setFocusedWidget(m_modalStack.back());
        } else {
            clearFocus();
        }
    }
}

UIWidgetPtr UIManager::getTopModal() const {
    return m_modalStack.empty() ? nullptr : m_modalStack.back();
}

void UIManager::draw() {
    if (!m_rootWidget) return;

    // Update animations
    static double lastTime = 0;
    double currentTime = g_platform.getTime();
    float deltaTime = static_cast<float>(currentTime - lastTime);
    lastTime = currentTime;

    processAnimations(deltaTime);

    // Draw all widgets
    m_rootWidget->draw();

    // Draw tooltip
    if (m_tooltipWidget && m_tooltipWidget->isVisible()) {
        m_tooltipWidget->draw();
    }

    // Debug draw
    if (m_debugDraw && m_hoveredWidget) {
        Rect rect = m_hoveredWidget->getAbsoluteRect();
        g_graphics.drawRect(rect, Color{255, 0, 0, 255});
    }
}

bool UIManager::onKeyDown(int keyCode, int modifiers) {
    if (m_focusedWidget) {
        return m_focusedWidget->onKeyDown(keyCode, modifiers);
    }
    return false;
}

bool UIManager::onKeyUp(int keyCode, int modifiers) {
    if (m_focusedWidget) {
        return m_focusedWidget->onKeyUp(keyCode, modifiers);
    }
    return false;
}

bool UIManager::onKeyPress(int keyCode, int modifiers) {
    if (m_focusedWidget) {
        return m_focusedWidget->onKeyPress(keyCode, modifiers);
    }
    return false;
}

bool UIManager::onTextInput(const std::string& text) {
    if (m_focusedWidget) {
        return m_focusedWidget->onTextInput(text);
    }
    return false;
}

bool UIManager::onMouseDown(int x, int y, int button) {
    // Check modals first
    if (!m_modalStack.empty()) {
        auto modal = m_modalStack.back();
        Rect rect = modal->getAbsoluteRect();
        if (!rect.contains(x, y)) {
            // Click outside modal - ignore or close based on settings
            return true;
        }
        return modal->onMouseDown(x, y, button);
    }

    // Find widget at position
    if (m_rootWidget) {
        if (auto widget = m_rootWidget->getChildByPos(x, y)) {
            m_pressedWidget = widget;
            return widget->onMouseDown(x, y, button);
        }
    }

    clearFocus();
    return false;
}

bool UIManager::onMouseUp(int x, int y, int button) {
    if (m_pressedWidget) {
        bool result = m_pressedWidget->onMouseUp(x, y, button);
        m_pressedWidget = nullptr;
        return result;
    }

    if (m_rootWidget) {
        if (auto widget = m_rootWidget->getChildByPos(x, y)) {
            return widget->onMouseUp(x, y, button);
        }
    }

    return false;
}

bool UIManager::onMouseMove(int x, int y) {
    // Update hovered widget
    UIWidgetPtr newHovered = nullptr;

    if (m_rootWidget) {
        newHovered = m_rootWidget->getChildByPos(x, y);
    }

    if (m_hoveredWidget != newHovered) {
        if (m_hoveredWidget) {
            m_hoveredWidget->onMouseLeave();
        }
        m_hoveredWidget = newHovered;
        if (m_hoveredWidget) {
            m_hoveredWidget->onMouseEnter();
        }
    }

    // Propagate to all widgets for hover state updates
    if (m_rootWidget) {
        m_rootWidget->onMouseMove(x, y);
    }

    return m_hoveredWidget != nullptr;
}

bool UIManager::onMouseWheel(int x, int y, int delta) {
    if (m_rootWidget) {
        if (auto widget = m_rootWidget->getChildByPos(x, y)) {
            return widget->onMouseWheel(x, y, delta);
        }
    }
    return false;
}

void UIManager::loadStyle(const std::string& filename) {
    extern ResourceManager& g_resources;
    std::string content = g_resources.readFileText(filename);

    if (content.empty()) return;

    // Parse OTUI style file
    // Format: WidgetType { property: value; ... }
    // Simplified parsing - real implementation would be more robust

    std::istringstream stream(content);
    std::string line;
    std::string currentType;

    while (std::getline(stream, line)) {
        // Trim whitespace
        size_t start = line.find_first_not_of(" \t\r\n");
        if (start == std::string::npos) continue;
        size_t end = line.find_last_not_of(" \t\r\n");
        line = line.substr(start, end - start + 1);

        // Skip comments
        if (line.empty() || line[0] == '/' || line[0] == '#') continue;

        // Check for type declaration
        size_t bracePos = line.find('{');
        if (bracePos != std::string::npos) {
            currentType = line.substr(0, bracePos);
            // Trim
            size_t typeEnd = currentType.find_last_not_of(" \t");
            if (typeEnd != std::string::npos) {
                currentType = currentType.substr(0, typeEnd + 1);
            }
            continue;
        }

        if (line == "}") {
            currentType.clear();
            continue;
        }

        // Parse property
        if (!currentType.empty()) {
            size_t colonPos = line.find(':');
            if (colonPos != std::string::npos) {
                std::string property = line.substr(0, colonPos);
                std::string value = line.substr(colonPos + 1);

                // Trim
                property.erase(0, property.find_first_not_of(" \t"));
                property.erase(property.find_last_not_of(" \t;") + 1);
                value.erase(0, value.find_first_not_of(" \t"));
                value.erase(value.find_last_not_of(" \t;") + 1);

                setStyleProperty(currentType, property, value);
            }
        }
    }
}

void UIManager::setStyleProperty(const std::string& widgetType,
                                  const std::string& property,
                                  const std::string& value) {
    m_styles[widgetType][property] = value;
}

void UIManager::setCursor(const std::string& cursor) {
    // Cursor changing would be implemented via SDL_SetCursor
    // For now, just store the cursor name
}

void UIManager::resetCursor() {
    setCursor("default");
}

void UIManager::showTooltip(const std::string& text, int x, int y) {
    if (!m_tooltipWidget) {
        m_tooltipWidget = std::make_shared<UILabel>();
        auto label = std::dynamic_pointer_cast<UILabel>(m_tooltipWidget);
        if (label) {
            label->setBackgroundColor(Color{40, 40, 40, 230});
            label->setPadding(4, 8, 4, 8);
        }
    }

    auto label = std::dynamic_pointer_cast<UILabel>(m_tooltipWidget);
    if (label) {
        label->setText(text);
        label->setTextAutoResize(true);
    }

    m_tooltipWidget->setPosition(x + 10, y + 10);
    m_tooltipWidget->show();
}

void UIManager::hideTooltip() {
    if (m_tooltipWidget) {
        m_tooltipWidget->hide();
    }
}

void UIManager::fadeIn(UIWidgetPtr widget, float duration) {
    if (!widget) return;

    widget->setOpacity(0.0f);
    widget->show();

    Animation anim;
    anim.widget = widget;
    anim.type = Animation::FadeIn;
    anim.duration = duration;
    anim.elapsed = 0;
    anim.startValue = 0.0f;
    anim.endValue = 1.0f;

    m_animations.push_back(anim);
}

void UIManager::fadeOut(UIWidgetPtr widget, float duration, bool destroyAfter) {
    if (!widget) return;

    Animation anim;
    anim.widget = widget;
    anim.type = Animation::FadeOut;
    anim.duration = duration;
    anim.elapsed = 0;
    anim.startValue = widget->getOpacity();
    anim.endValue = 0.0f;
    anim.destroyAfter = destroyAfter;

    m_animations.push_back(anim);
}

void UIManager::moveTo(UIWidgetPtr widget, int x, int y, float duration) {
    if (!widget) return;

    Animation anim;
    anim.widget = widget;
    anim.type = Animation::MoveTo;
    anim.duration = duration;
    anim.elapsed = 0;
    anim.startValue = static_cast<float>(widget->getX());
    anim.endValue = static_cast<float>(widget->getY());
    anim.targetX = x;
    anim.targetY = y;

    m_animations.push_back(anim);
}

void UIManager::processAnimations(float deltaTime) {
    for (auto it = m_animations.begin(); it != m_animations.end();) {
        auto& anim = *it;
        anim.elapsed += deltaTime;

        if (!anim.widget || anim.widget->isDestroyed()) {
            it = m_animations.erase(it);
            continue;
        }

        float t = std::min(anim.elapsed / anim.duration, 1.0f);
        // Ease out cubic
        t = 1.0f - (1.0f - t) * (1.0f - t) * (1.0f - t);

        switch (anim.type) {
            case Animation::FadeIn:
            case Animation::FadeOut:
                anim.widget->setOpacity(anim.startValue + (anim.endValue - anim.startValue) * t);
                break;

            case Animation::MoveTo:
                {
                    int startX = static_cast<int>(anim.startValue);
                    int startY = static_cast<int>(anim.endValue);
                    int x = startX + static_cast<int>((anim.targetX - startX) * t);
                    int y = startY + static_cast<int>((anim.targetY - startY) * t);
                    anim.widget->setPosition(x, y);
                }
                break;
        }

        // Check if animation complete
        if (anim.elapsed >= anim.duration) {
            if (anim.type == Animation::FadeOut) {
                anim.widget->hide();
                if (anim.destroyAfter) {
                    anim.widget->destroy();
                }
            }
            it = m_animations.erase(it);
        } else {
            ++it;
        }
    }
}

void UIManager::parseOTUI(const std::string& content, UIWidgetPtr parent) {
    // Simplified OTUI parser
    // Real implementation would handle full OTUI syntax

    std::istringstream stream(content);
    std::string line;
    UIWidgetPtr currentWidget = parent;
    std::vector<UIWidgetPtr> widgetStack;
    widgetStack.push_back(parent);

    int indentLevel = 0;

    while (std::getline(stream, line)) {
        // Skip empty lines and comments
        size_t firstNonSpace = line.find_first_not_of(" \t");
        if (firstNonSpace == std::string::npos) continue;
        if (line[firstNonSpace] == '/' || line[firstNonSpace] == '#') continue;

        // Calculate indent
        int currentIndent = static_cast<int>(firstNonSpace) / 2;

        // Trim line
        line = line.substr(firstNonSpace);
        size_t lastNonSpace = line.find_last_not_of(" \t\r\n");
        if (lastNonSpace != std::string::npos) {
            line = line.substr(0, lastNonSpace + 1);
        }

        // Handle indent changes
        while (currentIndent < static_cast<int>(widgetStack.size()) - 1 && widgetStack.size() > 1) {
            widgetStack.pop_back();
        }

        // Check for widget type declaration
        size_t colonPos = line.find(':');
        if (colonPos == std::string::npos && line.find(' ') == std::string::npos) {
            // Widget type (e.g., "Button")
            auto widget = createWidget(line, nullptr);
            if (widget) {
                widgetStack.back()->addChild(widget);
                widgetStack.push_back(widget);
            }
        } else if (colonPos != std::string::npos) {
            // Property assignment
            std::string property = line.substr(0, colonPos);
            std::string value = line.substr(colonPos + 1);

            // Trim
            property.erase(property.find_last_not_of(" \t") + 1);
            value.erase(0, value.find_first_not_of(" \t"));

            // Apply property to current widget
            if (!widgetStack.empty() && widgetStack.back()) {
                auto widget = widgetStack.back();

                // Common properties
                if (property == "id") {
                    widget->setId(value);
                } else if (property == "visible") {
                    widget->setVisible(value == "true");
                } else if (property == "enabled") {
                    widget->setEnabled(value == "true");
                } else if (property == "width") {
                    widget->setWidth(std::stoi(value));
                } else if (property == "height") {
                    widget->setHeight(std::stoi(value));
                } else if (property == "x") {
                    widget->setX(std::stoi(value));
                } else if (property == "y") {
                    widget->setY(std::stoi(value));
                } else if (property == "background-color" || property == "background") {
                    widget->setBackgroundColor(value);
                } else if (property == "border-color") {
                    widget->setBorderColor(Color::fromHex(value));
                } else if (property == "border-width") {
                    widget->setBorderWidth(std::stoi(value));
                } else if (property == "opacity") {
                    widget->setOpacity(std::stof(value));
                } else if (property == "margin") {
                    int m = std::stoi(value);
                    widget->setMargin(m, m, m, m);
                } else if (property == "padding") {
                    int p = std::stoi(value);
                    widget->setPadding(p, p, p, p);
                }
                // Label-specific
                else if (auto label = std::dynamic_pointer_cast<UILabel>(widget)) {
                    if (property == "text") {
                        label->setText(value);
                    } else if (property == "color" || property == "text-color") {
                        label->setColor(value);
                    } else if (property == "font") {
                        label->setFont(value);
                    }
                }
                // Button-specific
                else if (auto button = std::dynamic_pointer_cast<UIButton>(widget)) {
                    if (property == "text") {
                        button->setText(value);
                    }
                }
                // Window-specific
                else if (auto window = std::dynamic_pointer_cast<UIWindow>(widget)) {
                    if (property == "title") {
                        window->setTitle(value);
                    } else if (property == "draggable") {
                        window->setDraggable(value == "true");
                    } else if (property == "resizable") {
                        window->setResizable(value == "true");
                    } else if (property == "closeable") {
                        window->setCloseable(value == "true");
                    }
                }
            }
        }
    }
}

} // namespace framework
} // namespace shadow

// Global accessor
shadow::framework::UIManager& g_ui = shadow::framework::UIManager::instance();
