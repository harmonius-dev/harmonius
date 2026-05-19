//! Window and surface events.

use super::dpi::PhysicalSize;

/// Events delivered from the main thread event pump.
#[derive(Clone, Debug, PartialEq)]
pub enum WindowEvent {
    /// The user requested window close.
    CloseRequested,
    /// Client area size changed in physical pixels.
    Resized(PhysicalSize),
    /// Window gained or lost keyboard focus.
    FocusChanged(bool),
}

/// Events forwarded to the render thread for swapchain recreation.
#[derive(Clone, Debug, PartialEq)]
pub enum SurfaceEvent {
    /// Surface dimensions changed.
    Resized(PhysicalSize),
    /// Surface is no longer valid (display reconfigured).
    Invalidated,
}
