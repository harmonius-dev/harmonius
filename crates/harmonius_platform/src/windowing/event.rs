//! Cross-platform window lifecycle and chrome events.

use crate::windowing::{DisplayId, LogicalSize, PhysicalSize, Point, Rect, WindowMode};

/// Events emitted by the windowing subsystem for a single window.
#[derive(Clone, Debug, PartialEq)]
pub enum WindowEvent {
    /// Client area resized.
    Resized {
        /// New logical client size.
        logical: LogicalSize,
        /// New physical client size.
        physical: PhysicalSize,
    },
    /// Window moved in logical coordinates.
    Moved(Point),
    /// Window was minimized.
    Minimized,
    /// Window was maximized.
    Maximized,
    /// Window was restored from minimized or maximized state.
    Restored,
    /// Keyboard focus gained or lost.
    FocusChanged {
        /// True when the window gained focus.
        focused: bool,
    },
    /// User requested close (title bar, shortcut, etc.).
    CloseRequested,
    /// Native window destroyed; terminal event.
    Destroyed,
    /// DPI scale factor changed.
    DpiChanged {
        /// Previous scale factor.
        old_scale_factor: f64,
        /// New scale factor.
        new_scale_factor: f64,
        /// OS-suggested window geometry at the new scale.
        suggested_rect: Rect,
    },
    /// Window mode changed.
    ModeChanged(WindowMode),
    /// Window moved to a different display.
    DisplayChanged(DisplayId),
}
