//! Surface handle for GPU swapchain creation.

use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle,
    RawWindowHandle, WindowHandle,
};

use super::PhysicalSize;

/// Platform-specific data required for Vulkan surface creation.
#[derive(Clone, Copy, Debug)]
pub enum SurfaceNativeExt {
    /// `CAMetalLayer` pointer for `VK_EXT_metal_surface`.
    #[cfg(target_os = "macos")]
    MetalLayer(*mut std::ffi::c_void),
}

/// One-time handle passed to the render thread for swapchain creation.
pub struct SurfaceHandle {
    raw_window: RawWindowHandle,
    raw_display: RawDisplayHandle,
    /// Initial physical size of the client area.
    pub initial_size: PhysicalSize,
    /// Initial DPI scale factor.
    pub scale_factor: f64,
    /// Whether the display supports HDR output.
    pub hdr_capable: bool,
    /// Optional platform surface extension data.
    pub native_ext: Option<SurfaceNativeExt>,
}

impl SurfaceHandle {
    /// Construct a surface handle from raw platform handles.
    #[must_use]
    pub fn new(
        raw_window: RawWindowHandle,
        raw_display: RawDisplayHandle,
        initial_size: PhysicalSize,
        scale_factor: f64,
        hdr_capable: bool,
        native_ext: Option<SurfaceNativeExt>,
    ) -> Self {
        Self {
            raw_window,
            raw_display,
            initial_size,
            scale_factor,
            hdr_capable,
            native_ext,
        }
    }
}

// Safety: valid while the owning `Window` lives on the main thread.
unsafe impl Send for SurfaceHandle {}
unsafe impl Sync for SurfaceHandle {}

impl HasWindowHandle for SurfaceHandle {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        unsafe { Ok(WindowHandle::borrow_raw(self.raw_window)) }
    }
}

impl HasDisplayHandle for SurfaceHandle {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        unsafe { Ok(DisplayHandle::borrow_raw(self.raw_display)) }
    }
}
