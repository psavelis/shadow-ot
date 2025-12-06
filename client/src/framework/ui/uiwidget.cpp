/**
 * Shadow OT Client - UI Widget Implementation
 */

#include "uiwidget.h"
#include <framework/core/resourcemanager.h>
#include <algorithm>

namespace shadow {
namespace framework {

UIWidget::UIWidget() = default;

UIWidget::~UIWidget() {
    destroy();
}

void UIWidget::destroy() {
    if (m_destroyed) return;
    m_destroyed = true;

    // Destroy children first
    destroyChildren();

    // Remove from parent
    if (auto parent = m_parent.lock()) {
        parent->removeChild(shared_from_this());
    }
}

void UIWidget::addChild(UIWidgetPtr child) {
    if (!child || child.get() == this) return;

    // Remove from old parent
    if (auto oldParent = child->m_parent.lock()) {
        oldParent->removeChild(child);
    }

    child->m_parent = shared_from_this();
    m_children.push_back(child);
    child->setup();
    updateLayout();
}

void UIWidget::insertChild(int index, UIWidgetPtr child) {
    if (!child || child.get() == this) return;

    if (auto oldParent = child->m_parent.lock()) {
        oldParent->removeChild(child);
    }

    child->m_parent = shared_from_this();

    if (index < 0 || index >= static_cast<int>(m_children.size())) {
        m_children.push_back(child);
    } else {
        m_children.insert(m_children.begin() + index, child);
    }

    child->setup();
    updateLayout();
}

void UIWidget::removeChild(UIWidgetPtr child) {
    if (!child) return;

    auto it = std::find(m_children.begin(), m_children.end(), child);
    if (it != m_children.end()) {
        child->m_parent.reset();
        m_children.erase(it);
        updateLayout();
    }
}

UIWidgetPtr UIWidget::getChildById(const std::string& id) {
    for (auto& child : m_children) {
        if (child->getId() == id) {
            return child;
        }
        // Recursive search
        if (auto found = child->getChildById(id)) {
            return found;
        }
    }
    return nullptr;
}

UIWidgetPtr UIWidget::getChildByIndex(int index) {
    if (index >= 0 && index < static_cast<int>(m_children.size())) {
        return m_children[index];
    }
    return nullptr;
}

UIWidgetPtr UIWidget::getChildByPos(int x, int y) {
    // Search from top (last) to bottom (first)
    for (auto it = m_children.rbegin(); it != m_children.rend(); ++it) {
        auto& child = *it;
        if (!child->isVisible()) continue;

        Rect absRect = child->getAbsoluteRect();
        if (absRect.contains(x, y)) {
            // Check children of this child
            if (auto found = child->getChildByPos(x, y)) {
                return found;
            }
            return child;
        }
    }
    return nullptr;
}

void UIWidget::destroyChildren() {
    auto children = m_children; // Copy to avoid iterator invalidation
    for (auto& child : children) {
        child->destroy();
    }
    m_children.clear();
}

void UIWidget::moveChildToTop(UIWidgetPtr child) {
    auto it = std::find(m_children.begin(), m_children.end(), child);
    if (it != m_children.end()) {
        m_children.erase(it);
        m_children.push_back(child);
    }
}

void UIWidget::moveChildToBottom(UIWidgetPtr child) {
    auto it = std::find(m_children.begin(), m_children.end(), child);
    if (it != m_children.end()) {
        m_children.erase(it);
        m_children.insert(m_children.begin(), child);
    }
}

void UIWidget::setRect(const Rect& rect) {
    m_rect = rect;
    updateGeometry();
}

void UIWidget::setPosition(int x, int y) {
    m_rect.x = x;
    m_rect.y = y;
    updateGeometry();
}

void UIWidget::setSize(int width, int height) {
    m_rect.width = width;
    m_rect.height = height;
    updateGeometry();
}

Rect UIWidget::getAbsoluteRect() const {
    Rect result = m_rect;

    if (auto parent = m_parent.lock()) {
        Point parentPos = parent->getAbsolutePosition();
        result.x += parentPos.x + parent->m_paddingLeft;
        result.y += parentPos.y + parent->m_paddingTop;
    }

    return result;
}

Point UIWidget::getAbsolutePosition() const {
    Rect absRect = getAbsoluteRect();
    return {absRect.x, absRect.y};
}

void UIWidget::setMargin(int top, int right, int bottom, int left) {
    m_marginTop = top;
    m_marginRight = right;
    m_marginBottom = bottom;
    m_marginLeft = left;

    if (auto parent = m_parent.lock()) {
        parent->updateLayout();
    }
}

void UIWidget::setPadding(int top, int right, int bottom, int left) {
    m_paddingTop = top;
    m_paddingRight = right;
    m_paddingBottom = bottom;
    m_paddingLeft = left;
    updateLayout();
}

void UIWidget::setVisible(bool visible) {
    if (m_visible == visible) return;
    m_visible = visible;

    if (!visible && m_focused) {
        clearFocus();
    }
}

void UIWidget::raise() {
    if (auto parent = m_parent.lock()) {
        parent->moveChildToTop(shared_from_this());
    }
}

void UIWidget::lower() {
    if (auto parent = m_parent.lock()) {
        parent->moveChildToBottom(shared_from_this());
    }
}

void UIWidget::focus() {
    if (m_focused || !m_enabled || !m_visible) return;

    // Clear focus from other widgets in the tree
    if (auto parent = m_parent.lock()) {
        for (auto& sibling : parent->getChildren()) {
            if (sibling.get() != this && sibling->hasFocus()) {
                sibling->clearFocus();
            }
        }
    }

    m_focused = true;
    onFocusGain();
}

void UIWidget::clearFocus() {
    if (!m_focused) return;
    m_focused = false;
    onFocusLoss();
}

void UIWidget::setEnabled(bool enabled) {
    if (m_enabled == enabled) return;
    m_enabled = enabled;

    if (!enabled && m_focused) {
        clearFocus();
    }
}

void UIWidget::setImageSource(const std::string& path) {
    extern ResourceManager& g_resources;
    m_backgroundImage = g_resources.loadTexture(path);
}

void UIWidget::draw() {
    if (!m_visible || m_opacity <= 0.0f) return;

    drawSelf();
    drawChildren();
}

void UIWidget::drawSelf() {
    extern Graphics& g_graphics;

    Rect absRect = getAbsoluteRect();

    // Draw background
    if (m_backgroundColor.a > 0) {
        Color bg = m_backgroundColor;
        bg.a = static_cast<uint8_t>(bg.a * m_opacity);
        g_graphics.drawFilledRect(absRect, bg);
    }

    // Draw background image
    if (m_backgroundImage) {
        g_graphics.drawTexture(m_backgroundImage.get(), absRect);
    }

    // Draw border
    if (m_borderWidth > 0 && m_borderColor.a > 0) {
        Color border = m_borderColor;
        border.a = static_cast<uint8_t>(border.a * m_opacity);
        g_graphics.drawRect(absRect, border);
    }
}

void UIWidget::drawChildren() {
    for (auto& child : m_children) {
        child->draw();
    }
}

bool UIWidget::onKeyDown(int keyCode, int modifiers) {
    if (!m_enabled || !m_visible) return false;

    // Propagate to focused child
    for (auto& child : m_children) {
        if (child->hasFocus() && child->onKeyDown(keyCode, modifiers)) {
            return true;
        }
    }
    return false;
}

bool UIWidget::onKeyUp(int keyCode, int modifiers) {
    if (!m_enabled || !m_visible) return false;

    for (auto& child : m_children) {
        if (child->hasFocus() && child->onKeyUp(keyCode, modifiers)) {
            return true;
        }
    }
    return false;
}

bool UIWidget::onKeyPress(int keyCode, int modifiers) {
    if (!m_enabled || !m_visible) return false;

    for (auto& child : m_children) {
        if (child->hasFocus() && child->onKeyPress(keyCode, modifiers)) {
            return true;
        }
    }
    return false;
}

bool UIWidget::onTextInput(const std::string& text) {
    if (!m_enabled || !m_visible) return false;

    for (auto& child : m_children) {
        if (child->hasFocus() && child->onTextInput(text)) {
            return true;
        }
    }
    return false;
}

bool UIWidget::onMouseDown(int x, int y, int button) {
    if (!m_enabled || !m_visible) return false;

    // Find child at position
    if (auto child = getChildByPos(x, y)) {
        return child->onMouseDown(x, y, button);
    }

    // Handle self
    Rect absRect = getAbsoluteRect();
    if (absRect.contains(x, y)) {
        m_pressed = true;
        focus();

        if (m_onMouseDown && m_onMouseDown(shared_from_this(), x, y, button)) {
            return true;
        }
        return true;
    }

    return false;
}

bool UIWidget::onMouseUp(int x, int y, int button) {
    if (!m_enabled || !m_visible) return false;

    bool wasPressed = m_pressed;
    m_pressed = false;

    // Find child at position
    if (auto child = getChildByPos(x, y)) {
        return child->onMouseUp(x, y, button);
    }

    Rect absRect = getAbsoluteRect();
    if (absRect.contains(x, y)) {
        if (m_onMouseUp && m_onMouseUp(shared_from_this(), x, y, button)) {
            return true;
        }

        // Click event
        if (wasPressed && m_onClick) {
            m_onClick(shared_from_this());
        }
        return true;
    }

    return false;
}

bool UIWidget::onMouseMove(int x, int y) {
    if (!m_visible) return false;

    Rect absRect = getAbsoluteRect();
    bool isInside = absRect.contains(x, y);

    if (isInside && !m_hovered) {
        m_hovered = true;
        onMouseEnter();
    } else if (!isInside && m_hovered) {
        m_hovered = false;
        onMouseLeave();
    }

    // Propagate to children
    for (auto& child : m_children) {
        child->onMouseMove(x, y);
    }

    return isInside;
}

bool UIWidget::onMouseWheel(int x, int y, int delta) {
    if (!m_enabled || !m_visible) return false;

    if (auto child = getChildByPos(x, y)) {
        return child->onMouseWheel(x, y, delta);
    }

    return false;
}

void UIWidget::onMouseEnter() {
    // Override in subclasses
}

void UIWidget::onMouseLeave() {
    m_hovered = false;
    m_pressed = false;
}

void UIWidget::onFocusGain() {
    // Override in subclasses
}

void UIWidget::onFocusLoss() {
    // Override in subclasses
}

void UIWidget::updateLayout() {
    if (m_layout == Layout::None || m_children.empty()) return;

    int x = m_paddingLeft;
    int y = m_paddingTop;
    int availableWidth = m_rect.width - m_paddingLeft - m_paddingRight;
    int availableHeight = m_rect.height - m_paddingTop - m_paddingBottom;

    switch (m_layout) {
        case Layout::Vertical: {
            for (auto& child : m_children) {
                if (!child->isVisible()) continue;

                child->setPosition(x + child->m_marginLeft, y + child->m_marginTop);
                y += child->getHeight() + child->m_marginTop + child->m_marginBottom;
            }
            break;
        }

        case Layout::Horizontal: {
            for (auto& child : m_children) {
                if (!child->isVisible()) continue;

                child->setPosition(x + child->m_marginLeft, y + child->m_marginTop);
                x += child->getWidth() + child->m_marginLeft + child->m_marginRight;
            }
            break;
        }

        case Layout::Grid: {
            // Simple grid with auto-wrap
            int maxRowHeight = 0;
            for (auto& child : m_children) {
                if (!child->isVisible()) continue;

                int childWidth = child->getWidth() + child->m_marginLeft + child->m_marginRight;
                int childHeight = child->getHeight() + child->m_marginTop + child->m_marginBottom;

                // Check if we need to wrap
                if (x + childWidth > m_paddingLeft + availableWidth && x > m_paddingLeft) {
                    x = m_paddingLeft;
                    y += maxRowHeight;
                    maxRowHeight = 0;
                }

                child->setPosition(x + child->m_marginLeft, y + child->m_marginTop);
                x += childWidth;
                maxRowHeight = std::max(maxRowHeight, childHeight);
            }
            break;
        }

        case Layout::Anchored: {
            // Anchor-based layout
            for (auto& child : m_children) {
                if (!child->isVisible()) continue;
                // Anchors are processed in updateGeometry
            }
            break;
        }

        default:
            break;
    }
}

void UIWidget::addAnchor(const Anchor& anchor) {
    m_anchors.push_back(anchor);
    updateGeometry();
}

void UIWidget::clearAnchors() {
    m_anchors.clear();
}

void UIWidget::updateGeometry() {
    // Process anchors
    if (!m_anchors.empty() && !m_parent.expired()) {
        auto parent = m_parent.lock();

        for (const auto& anchor : m_anchors) {
            UIWidgetPtr target;

            if (anchor.targetId == "parent") {
                target = parent;
            } else {
                target = parent->getChildById(anchor.targetId);
            }

            if (!target) continue;

            Rect targetRect = (target == parent) ?
                Rect{0, 0, target->getWidth(), target->getHeight()} :
                target->getRect();

            int targetValue = 0;
            switch (anchor.targetEdge) {
                case Anchor::Left: targetValue = targetRect.x; break;
                case Anchor::Right: targetValue = targetRect.x + targetRect.width; break;
                case Anchor::Top: targetValue = targetRect.y; break;
                case Anchor::Bottom: targetValue = targetRect.y + targetRect.height; break;
                case Anchor::HCenter: targetValue = targetRect.x + targetRect.width / 2; break;
                case Anchor::VCenter: targetValue = targetRect.y + targetRect.height / 2; break;
            }

            switch (anchor.sourceEdge) {
                case Anchor::Left:
                    m_rect.x = targetValue + anchor.offset;
                    break;
                case Anchor::Right:
                    m_rect.x = targetValue - m_rect.width + anchor.offset;
                    break;
                case Anchor::Top:
                    m_rect.y = targetValue + anchor.offset;
                    break;
                case Anchor::Bottom:
                    m_rect.y = targetValue - m_rect.height + anchor.offset;
                    break;
                case Anchor::HCenter:
                    m_rect.x = targetValue - m_rect.width / 2 + anchor.offset;
                    break;
                case Anchor::VCenter:
                    m_rect.y = targetValue - m_rect.height / 2 + anchor.offset;
                    break;
            }
        }
    }

    // Update children layout
    updateLayout();
}

} // namespace framework
} // namespace shadow
