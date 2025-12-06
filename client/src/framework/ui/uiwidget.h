/**
 * Shadow OT Client - UI Widget System
 *
 * Flexible widget-based UI framework.
 */

#pragma once

#include <string>
#include <vector>
#include <memory>
#include <functional>
#include <map>
#include <framework/graphics/graphics.h>

namespace shadow {
namespace framework {

class UIWidget;
using UIWidgetPtr = std::shared_ptr<UIWidget>;
using UIWidgetList = std::vector<UIWidgetPtr>;

class UIWidget : public std::enable_shared_from_this<UIWidget> {
public:
    UIWidget();
    virtual ~UIWidget();

    // Lifecycle
    virtual void setup() {}
    virtual void destroy();
    bool isDestroyed() const { return m_destroyed; }

    // Identification
    void setId(const std::string& id) { m_id = id; }
    const std::string& getId() const { return m_id; }

    // Hierarchy
    void addChild(UIWidgetPtr child);
    void insertChild(int index, UIWidgetPtr child);
    void removeChild(UIWidgetPtr child);
    UIWidgetPtr getChildById(const std::string& id);
    UIWidgetPtr getChildByIndex(int index);
    UIWidgetPtr getChildByPos(int x, int y);
    UIWidgetPtr getParent() const { return m_parent.lock(); }
    const UIWidgetList& getChildren() const { return m_children; }
    void destroyChildren();
    void moveChildToTop(UIWidgetPtr child);
    void moveChildToBottom(UIWidgetPtr child);

    // Geometry
    void setRect(const Rect& rect);
    void setPosition(int x, int y);
    void setSize(int width, int height);
    void setX(int x) { m_rect.x = x; updateGeometry(); }
    void setY(int y) { m_rect.y = y; updateGeometry(); }
    void setWidth(int w) { m_rect.width = w; updateGeometry(); }
    void setHeight(int h) { m_rect.height = h; updateGeometry(); }

    const Rect& getRect() const { return m_rect; }
    Point getPosition() const { return {m_rect.x, m_rect.y}; }
    Size getSize() const { return {m_rect.width, m_rect.height}; }
    int getX() const { return m_rect.x; }
    int getY() const { return m_rect.y; }
    int getWidth() const { return m_rect.width; }
    int getHeight() const { return m_rect.height; }

    // Absolute position (relative to root)
    Rect getAbsoluteRect() const;
    Point getAbsolutePosition() const;

    // Margins and padding
    void setMargin(int top, int right, int bottom, int left);
    void setMarginTop(int m) { m_marginTop = m; }
    void setMarginRight(int m) { m_marginRight = m; }
    void setMarginBottom(int m) { m_marginBottom = m; }
    void setMarginLeft(int m) { m_marginLeft = m; }
    void setPadding(int top, int right, int bottom, int left);
    void setPaddingTop(int p) { m_paddingTop = p; }
    void setPaddingRight(int p) { m_paddingRight = p; }
    void setPaddingBottom(int p) { m_paddingBottom = p; }
    void setPaddingLeft(int p) { m_paddingLeft = p; }

    // Visibility
    void setVisible(bool visible);
    bool isVisible() const { return m_visible; }
    void show() { setVisible(true); }
    void hide() { setVisible(false); }
    void raise();
    void lower();
    void focus();
    void clearFocus();
    bool hasFocus() const { return m_focused; }

    // Enabled state
    void setEnabled(bool enabled);
    bool isEnabled() const { return m_enabled; }

    // Appearance
    void setBackgroundColor(const Color& color) { m_backgroundColor = color; }
    void setBackgroundColor(const std::string& hex) { m_backgroundColor = Color::fromHex(hex); }
    const Color& getBackgroundColor() const { return m_backgroundColor; }
    void setBorderColor(const Color& color) { m_borderColor = color; }
    void setBorderWidth(int width) { m_borderWidth = width; }
    void setBorderRadius(int radius) { m_borderRadius = radius; }
    void setOpacity(float opacity) { m_opacity = opacity; }
    float getOpacity() const { return m_opacity; }

    // Image background
    void setImageSource(const std::string& path);

    // Rendering
    virtual void draw();
    virtual void drawSelf();
    virtual void drawChildren();

    // Input handling
    virtual bool onKeyDown(int keyCode, int modifiers);
    virtual bool onKeyUp(int keyCode, int modifiers);
    virtual bool onKeyPress(int keyCode, int modifiers);
    virtual bool onTextInput(const std::string& text);

    virtual bool onMouseDown(int x, int y, int button);
    virtual bool onMouseUp(int x, int y, int button);
    virtual bool onMouseMove(int x, int y);
    virtual bool onMouseWheel(int x, int y, int delta);
    virtual void onMouseEnter();
    virtual void onMouseLeave();

    virtual void onFocusGain();
    virtual void onFocusLoss();

    // Event callbacks (Lua-style)
    using EventCallback = std::function<void(UIWidgetPtr)>;
    using MouseCallback = std::function<bool(UIWidgetPtr, int, int, int)>;

    void setOnClick(EventCallback cb) { m_onClick = cb; }
    void setOnDoubleClick(EventCallback cb) { m_onDoubleClick = cb; }
    void setOnMouseDown(MouseCallback cb) { m_onMouseDown = cb; }
    void setOnMouseUp(MouseCallback cb) { m_onMouseUp = cb; }

    // Lua binding support
    EventCallback onClick;
    EventCallback onDoubleClick;

    // Tooltip
    void setTooltip(const std::string& text) { m_tooltip = text; }
    const std::string& getTooltip() const { return m_tooltip; }

    // Layout
    enum class Layout { None, Vertical, Horizontal, Grid, Anchored };
    void setLayout(Layout layout) { m_layout = layout; }
    void updateLayout();

    // Anchor system
    struct Anchor {
        std::string targetId;
        enum Edge { Left, Right, Top, Bottom, HCenter, VCenter };
        Edge sourceEdge;
        Edge targetEdge;
        int offset{0};
    };
    void addAnchor(const Anchor& anchor);
    void clearAnchors();

protected:
    virtual void updateGeometry();

    std::string m_id;
    std::weak_ptr<UIWidget> m_parent;
    UIWidgetList m_children;
    Rect m_rect{0, 0, 0, 0};

    int m_marginTop{0}, m_marginRight{0}, m_marginBottom{0}, m_marginLeft{0};
    int m_paddingTop{0}, m_paddingRight{0}, m_paddingBottom{0}, m_paddingLeft{0};

    bool m_visible{true};
    bool m_enabled{true};
    bool m_focused{false};
    bool m_destroyed{false};
    bool m_hovered{false};
    bool m_pressed{false};

    Color m_backgroundColor{0, 0, 0, 0};
    Color m_borderColor{100, 100, 100, 255};
    int m_borderWidth{0};
    int m_borderRadius{0};
    float m_opacity{1.0f};

    std::shared_ptr<Texture> m_backgroundImage;
    std::string m_tooltip;

    Layout m_layout{Layout::None};
    std::vector<Anchor> m_anchors;

    EventCallback m_onClick;
    EventCallback m_onDoubleClick;
    MouseCallback m_onMouseDown;
    MouseCallback m_onMouseUp;
};

} // namespace framework
} // namespace shadow
