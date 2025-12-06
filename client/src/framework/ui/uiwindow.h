/**
 * Shadow OT Client - UI Window
 *
 * Draggable, resizable window widget.
 */

#pragma once

#include "uiwidget.h"
#include <string>

namespace shadow {
namespace framework {

class UIWindow : public UIWidget {
public:
    UIWindow();

    void setTitle(const std::string& title) { m_title = title; }
    const std::string& getTitle() const { return m_title; }

    void setDraggable(bool draggable) { m_draggable = draggable; }
    bool isDraggable() const { return m_draggable; }

    void setResizable(bool resizable) { m_resizable = resizable; }
    bool isResizable() const { return m_resizable; }

    void setCloseable(bool closeable) { m_closeable = closeable; }
    bool isCloseable() const { return m_closeable; }

    void setMinSize(int width, int height) { m_minWidth = width; m_minHeight = height; }
    void setMaxSize(int width, int height) { m_maxWidth = width; m_maxHeight = height; }

    // Header appearance
    void setTitleBarHeight(int height) { m_titleBarHeight = height; }
    void setTitleBarColor(const Color& color) { m_titleBarColor = color; }
    void setTitleColor(const Color& color) { m_titleColor = color; }

    // Close callback
    using CloseCallback = std::function<bool()>;
    void setOnClose(CloseCallback cb) { m_onClose = cb; }

    void drawSelf() override;

    bool onMouseDown(int x, int y, int button) override;
    bool onMouseUp(int x, int y, int button) override;
    bool onMouseMove(int x, int y) override;
    bool onKeyDown(int keyCode, int modifiers) override;

protected:
    std::string m_title;
    bool m_draggable{true};
    bool m_resizable{false};
    bool m_closeable{true};

    int m_titleBarHeight{24};
    int m_minWidth{100};
    int m_minHeight{50};
    int m_maxWidth{4096};
    int m_maxHeight{4096};

    Color m_titleBarColor{40, 40, 40};
    Color m_titleColor{255, 255, 255};

    CloseCallback m_onClose;

private:
    enum class DragMode { None, Move, ResizeN, ResizeS, ResizeE, ResizeW,
                          ResizeNE, ResizeNW, ResizeSE, ResizeSW };

    DragMode getDragMode(int x, int y) const;

    DragMode m_dragMode{DragMode::None};
    int m_dragStartX{0};
    int m_dragStartY{0};
    Rect m_dragStartRect;
};

// Main window with integrated title bar styling
class MainWindow : public UIWindow {
public:
    MainWindow();
};

} // namespace framework
} // namespace shadow
