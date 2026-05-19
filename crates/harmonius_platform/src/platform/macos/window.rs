//! macOS windowing via AppKit (`objc2-app-kit`).

use std::ptr::NonNull;

use harmonius_core::EngineError;
use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{class, msg_send, MainThreadOnly};
use objc2_app_kit::{
    NSApplication, NSApplicationActivationPolicy, NSBackingStoreType, NSView, NSWindow,
    NSWindowStyleMask,
};
use objc2_foundation::{MainThreadMarker, NSRect, NSSize, NSString};
use objc2_quartz_core::CAMetalLayer;
use raw_window_handle::{
    AppKitDisplayHandle, AppKitWindowHandle, RawDisplayHandle, RawWindowHandle,
};

use crate::windowing::{PhysicalSize, SurfaceHandle, SurfaceNativeExt, WindowConfig, WindowEvent};

/// AppKit-backed native window.
pub struct NativeWindow {
    #[allow(dead_code)]
    app: Retained<NSApplication>,
    window: Retained<NSWindow>,
    _content_view: Retained<NSView>,
    metal_layer: Retained<CAMetalLayer>,
    ns_view: NonNull<AnyObject>,
    window_id: u64,
    physical_size: PhysicalSize,
    scale_factor: f64,
}

impl NativeWindow {
    /// Creates and shows a new `NSWindow`.
    pub fn create(config: &WindowConfig) -> Result<Self, EngineError> {
        let mtm = MainThreadMarker::new().ok_or_else(|| {
            EngineError::Platform("NSApplication must run on the main thread".into())
        })?;

        let app = NSApplication::sharedApplication(mtm);
        app.setActivationPolicy(NSApplicationActivationPolicy::Regular);
        let _ = mtm;

        let scale = 1.0_f64;
        let physical = config.size.to_physical(scale);
        let content_rect = NSRect::new(
            objc2_foundation::NSPoint::new(100.0, 100.0),
            NSSize::new(f64::from(physical.width), f64::from(physical.height)),
        );

        let style = NSWindowStyleMask::Titled
            | NSWindowStyleMask::Closable
            | NSWindowStyleMask::Miniaturizable
            | NSWindowStyleMask::Resizable;

        let title = NSString::from_str(&config.title);
        let window = unsafe {
            NSWindow::initWithContentRect_styleMask_backing_defer(
                NSWindow::alloc(mtm),
                content_rect,
                style,
                NSBackingStoreType::Buffered,
                false,
            )
        };
        window.setTitle(&title);

        let content_view = window
            .contentView()
            .ok_or_else(|| EngineError::Platform("NSWindow has no content view".into()))?;
        content_view.setWantsLayer(true);
        let metal_layer = unsafe {
            let layer: Retained<CAMetalLayer> = msg_send![class!(CAMetalLayer), new];
            layer.setDrawableSize(NSSize::new(
                f64::from(physical.width),
                f64::from(physical.height),
            ));
            content_view.setLayer(Some(&layer));
            layer
        };
        let ns_view = NonNull::from(content_view.as_ref());

        window.center();
        window.makeKeyAndOrderFront(None);
        app.finishLaunching();

        Ok(Self {
            app,
            window,
            _content_view: content_view,
            metal_layer,
            ns_view,
            window_id: 1,
            physical_size: physical,
            scale_factor: scale,
        })
    }

    /// Stable window id for handles.
    #[must_use]
    pub fn window_id(&self) -> u64 {
        self.window_id
    }

    /// Backing scale factor.
    #[must_use]
    pub fn scale_factor(&self) -> f64 {
        self.scale_factor
    }

    /// Current physical client size.
    #[must_use]
    pub fn physical_size(&self) -> PhysicalSize {
        self.physical_size
    }

    /// Pointer to the `CAMetalLayer` for MoltenVK.
    #[must_use]
    pub fn metal_layer_ptr(&self) -> *mut std::ffi::c_void {
        Retained::as_ptr(&self.metal_layer) as *mut std::ffi::c_void
    }

    /// Builds a [`SurfaceHandle`] for Vulkan (MoltenVK metal surface).
    #[must_use]
    pub fn surface_handle(&self, scale_factor: f64) -> SurfaceHandle {
        let view_ptr = self.ns_view.as_ptr().cast::<std::ffi::c_void>();
        let raw_window = RawWindowHandle::AppKit(AppKitWindowHandle::new(
            NonNull::new(view_ptr).expect("content view pointer"),
        ));
        let raw_display = RawDisplayHandle::AppKit(AppKitDisplayHandle::new());
        SurfaceHandle::new(
            raw_window,
            raw_display,
            self.physical_size,
            scale_factor,
            false,
            Some(SurfaceNativeExt::MetalLayer(self.metal_layer_ptr())),
        )
    }

    /// Updates cached size after a resize event.
    pub fn on_resize(&mut self, size: PhysicalSize) {
        self.physical_size = size;
        self.metal_layer
            .setDrawableSize(NSSize::new(f64::from(size.width), f64::from(size.height)));
    }

    /// Drains AppKit events for this window.
    pub fn pump_events(&mut self) -> Result<Vec<WindowEvent>, EngineError> {
        let mtm = MainThreadMarker::new()
            .ok_or_else(|| EngineError::Platform("event pump requires main thread".into()))?;
        let app = NSApplication::sharedApplication(mtm);
        if let Some(event) = app.nextEventMatchingMask_untilDate_inMode_dequeue(
            objc2_app_kit::NSEventMask::Any,
            None,
            unsafe { objc2_foundation::NSDefaultRunLoopMode },
            true,
        ) {
            app.sendEvent(&event);
        }

        let mut out = Vec::new();
        let bounds = self._content_view.bounds();
        let new_size = PhysicalSize {
            width: bounds.size.width.max(1.0) as u32,
            height: bounds.size.height.max(1.0) as u32,
        };
        if new_size != self.physical_size {
            self.on_resize(new_size);
            out.push(WindowEvent::Resized(new_size));
        }
        Ok(out)
    }
}
