//! Surface handles and render-thread surface events.
//!
//! The stub backend does not yet expose a native window pointer, so
//! [`HasWindowHandle::window_handle`] returns [`HandleError::NotSupported`]. Display
//! interop uses a portable empty web display handle until per-OS backends are wired.

use raw_window_handle::{DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle};

use crate::windowing::PhysicalSize;

/// Handle passed to the render thread for swapchain creation.
#[derive(Clone, Debug)]
pub struct SurfaceHandle {
    /// Reserved for future native handle wiring (stub canvas id).
    pub web_canvas_id: u32,
    /// Initial physical size of the client area.
    pub initial_size: PhysicalSize,
    /// Initial DPI scale factor.
    pub scale_factor: f64,
    /// Whether the display supports HDR output.
    pub hdr_capable: bool,
}

impl SurfaceHandle {
    /// Build a portable stub handle for tests and early integration.
    #[must_use]
    pub fn new_stub(
        web_canvas_id: u32,
        initial_size: PhysicalSize,
        scale_factor: f64,
        hdr_capable: bool,
    ) -> Self {
        Self {
            web_canvas_id,
            initial_size,
            scale_factor,
            hdr_capable,
        }
    }
}

impl HasDisplayHandle for SurfaceHandle {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        Ok(DisplayHandle::web())
    }
}

impl HasWindowHandle for SurfaceHandle {
    fn window_handle(&self) -> Result<raw_window_handle::WindowHandle<'_>, HandleError> {
        Err(HandleError::NotSupported)
    }
}

/// Events sent from the main thread to the render thread when the surface changes.
#[derive(Clone, Debug, PartialEq)]
pub enum SurfaceEvent {
    /// Client area resized to a new physical size.
    Resized(PhysicalSize),
    /// DPI scale factor changed.
    ScaleFactorChanged(f64),
    /// HDR capability changed.
    HdrChanged(bool),
    /// Surface invalidated; recreate swapchain.
    SurfaceInvalidated,
}
