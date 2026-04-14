//! Portable window stub and cross-platform `Window` API surface.
//!
//! Native Win32 / AppKit / Linux backends replace the internal queue logic later;
//! this type is safe on all targets and supports deterministic lifecycle tests.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

use crate::windowing::{
    CursorIcon, DisplayId, DisplayInfo, HdrConfig, HdrError, LogicalSize, PhysicalSize, Point,
    SurfaceEvent, SurfaceHandle, WindowConfig, WindowError, WindowEvent, WindowMode,
};

static NEXT_WINDOW_HANDLE: AtomicU64 = AtomicU64::new(1);
static NEXT_WEB_STUB_ID: AtomicU32 = AtomicU32::new(1);

/// Opaque handle identifying a window instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WindowHandle {
    id: u64,
}

impl WindowHandle {
    /// Returns the stable numeric id for this window.
    #[must_use]
    pub fn id(self) -> u64 {
        self.id
    }
}

/// Non-blocking iterator over pending [`WindowEvent`] values.
pub struct EventIterator {
    sink: Rc<RefCell<VecDeque<WindowEvent>>>,
}

impl Iterator for EventIterator {
    type Item = WindowEvent;

    fn next(&mut self) -> Option<Self::Item> {
        self.sink.borrow_mut().pop_front()
    }
}

/// Iterator over pending [`SurfaceEvent`] values for the render thread.
pub struct SurfaceEventIterator {
    sink: Rc<RefCell<VecDeque<SurfaceEvent>>>,
}

impl Iterator for SurfaceEventIterator {
    type Item = SurfaceEvent;

    fn next(&mut self) -> Option<Self::Item> {
        self.sink.borrow_mut().pop_front()
    }
}

/// Native OS window (portable stub until real platform backends land).
#[allow(dead_code)]
pub struct Window {
    handle: WindowHandle,
    web_stub_id: u32,
    event_sink: Rc<RefCell<VecDeque<WindowEvent>>>,
    surface_sink: Rc<RefCell<VecDeque<SurfaceEvent>>>,
    config: WindowConfig,
    logical_size: LogicalSize,
    physical_size: PhysicalSize,
    position: Point,
    mode: WindowMode,
    scale_factor: f64,
    minimized: bool,
    maximized: bool,
    hdr_active: bool,
    cursor: CursorIcon,
    cursor_visible: bool,
    cursor_confined: bool,
    cursor_captured: bool,
}

impl Window {
    /// Create a new window. The stub accepts only the primary [`DisplayId`] (0).
    pub fn new(config: WindowConfig) -> Result<Self, WindowError> {
        if let Some(id) = config.target_display {
            if id != DisplayId(0) {
                return Err(WindowError::DisplayNotFound(id));
            }
        }

        let handle_id = NEXT_WINDOW_HANDLE.fetch_add(1, Ordering::Relaxed);
        let handle = WindowHandle { id: handle_id };
        let web_stub_id = NEXT_WEB_STUB_ID.fetch_add(1, Ordering::Relaxed);
        let event_sink = Rc::new(RefCell::new(VecDeque::new()));
        let surface_sink = Rc::new(RefCell::new(VecDeque::new()));

        let scale_factor = 1.0;
        let logical_size = config.size;
        let physical_size = logical_size.to_physical(scale_factor);
        let mode = config.mode;

        let mut window = Self {
            handle,
            web_stub_id,
            event_sink: Rc::clone(&event_sink),
            surface_sink: Rc::clone(&surface_sink),
            config,
            logical_size,
            physical_size,
            position: Point { x: 0.0, y: 0.0 },
            mode,
            scale_factor,
            minimized: false,
            maximized: false,
            hdr_active: false,
            cursor: CursorIcon::Default,
            cursor_visible: true,
            cursor_confined: false,
            cursor_captured: false,
        };

        window.push_resize();
        Ok(window)
    }

    /// Opaque handle for this window.
    #[must_use]
    pub fn handle(&self) -> WindowHandle {
        self.handle
    }

    /// Change fullscreen or windowed mode.
    pub fn set_mode(&mut self, mode: WindowMode) -> Result<(), WindowError> {
        self.mode = mode;
        self.push_resize();
        self.queue_window(WindowEvent::ModeChanged(mode));
        Ok(())
    }

    /// Returns the active [`WindowMode`].
    #[must_use]
    pub fn current_mode(&self) -> WindowMode {
        self.mode
    }

    /// Updates the window title string tracked by the stub.
    pub fn set_title(&mut self, title: &str) {
        self.config.title.clear();
        self.config.title.push_str(title);
    }

    /// Resizes the client area using logical units.
    pub fn set_size(&mut self, size: LogicalSize) {
        self.config.size = size;
        self.logical_size = size;
        self.push_resize();
    }

    /// Returns logical and physical client sizes.
    #[must_use]
    pub fn size(&self) -> (LogicalSize, PhysicalSize) {
        (self.logical_size, self.physical_size)
    }

    /// Minimizes the window and queues [`WindowEvent::Minimized`].
    pub fn minimize(&mut self) {
        if !self.minimized {
            self.minimized = true;
            self.maximized = false;
            self.queue_window(WindowEvent::Minimized);
        }
    }

    /// Maximizes the window and queues [`WindowEvent::Maximized`].
    pub fn maximize(&mut self) {
        if !self.maximized {
            self.maximized = true;
            self.minimized = false;
            self.queue_window(WindowEvent::Maximized);
        }
    }

    /// Restores from minimized or maximized chrome state.
    pub fn restore(&mut self) {
        if self.minimized || self.maximized {
            self.minimized = false;
            self.maximized = false;
            self.queue_window(WindowEvent::Restored);
        }
    }

    /// Moves the window origin in logical coordinates.
    pub fn set_position(&mut self, position: Point) {
        self.position = position;
        self.queue_window(WindowEvent::Moved(position));
    }

    /// Returns the window origin in logical coordinates.
    #[must_use]
    pub fn position(&self) -> Point {
        self.position
    }

    /// Selects the cursor icon for this window.
    pub fn set_cursor(&mut self, cursor: CursorIcon) {
        self.cursor = cursor;
    }

    /// Toggles cursor visibility while over this window.
    pub fn set_cursor_visible(&mut self, visible: bool) {
        self.cursor_visible = visible;
    }

    /// Enables or disables cursor confinement to the client area.
    pub fn set_cursor_confined(&mut self, confined: bool) {
        self.cursor_confined = confined;
    }

    /// Enables or disables relative cursor capture mode.
    pub fn set_cursor_captured(&mut self, captured: bool) {
        self.cursor_captured = captured;
    }

    /// Warps the cursor to a logical client position (stub always succeeds).
    pub fn set_cursor_position(&mut self, _pos: Point) -> Result<(), WindowError> {
        Ok(())
    }

    /// Enables HDR using the given configuration when the display supports it.
    pub fn enable_hdr(&mut self, config: HdrConfig) -> Result<(), HdrError> {
        if !config.enabled {
            self.hdr_active = false;
            return Ok(());
        }
        if !stub_display().hdr_capable {
            return Err(HdrError::DisplayNotHdrCapable);
        }
        self.hdr_active = true;
        self.queue_surface(SurfaceEvent::HdrChanged(true));
        Ok(())
    }

    /// Disables HDR and emits [`SurfaceEvent::HdrChanged`] when state changes.
    pub fn disable_hdr(&mut self) {
        if self.hdr_active {
            self.queue_surface(SurfaceEvent::HdrChanged(false));
        }
        self.hdr_active = false;
    }

    /// Whether HDR is active.
    #[must_use]
    pub fn is_hdr_active(&self) -> bool {
        self.hdr_active
    }

    /// Current DPI scale factor.
    #[must_use]
    pub fn current_dpi(&self) -> f64 {
        self.scale_factor
    }

    /// Enumerate connected displays (stub returns one primary display).
    #[must_use]
    pub fn displays(&self) -> Vec<DisplayInfo> {
        vec![stub_display()]
    }

    /// Display the window currently occupies.
    #[must_use]
    pub fn current_display(&self) -> Option<DisplayInfo> {
        Some(stub_display())
    }

    /// Surface handle for swapchain creation.
    #[must_use]
    pub fn surface_handle(&self) -> SurfaceHandle {
        SurfaceHandle::new_stub(
            self.web_stub_id,
            self.physical_size,
            self.scale_factor,
            stub_display().hdr_capable,
        )
    }

    /// Pending surface events for the render thread.
    pub fn surface_events(&mut self) -> SurfaceEventIterator {
        SurfaceEventIterator {
            sink: Rc::clone(&self.surface_sink),
        }
    }

    /// Pending window events.
    pub fn events(&mut self) -> EventIterator {
        EventIterator {
            sink: Rc::clone(&self.event_sink),
        }
    }

    fn queue_window(&self, event: WindowEvent) {
        self.event_sink.borrow_mut().push_back(event);
    }

    fn queue_surface(&self, event: SurfaceEvent) {
        self.surface_sink.borrow_mut().push_back(event);
    }

    fn push_resize(&mut self) {
        self.physical_size = self.logical_size.to_physical(self.scale_factor);
        let logical = self.logical_size;
        let physical = self.physical_size;
        self.queue_window(WindowEvent::Resized { logical, physical });
        self.queue_surface(SurfaceEvent::Resized(physical));
    }

    #[cfg(test)]
    pub(crate) fn test_event_sink(&self) -> Rc<RefCell<VecDeque<WindowEvent>>> {
        Rc::clone(&self.event_sink)
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.event_sink
            .borrow_mut()
            .push_back(WindowEvent::Destroyed);
    }
}

fn stub_display() -> DisplayInfo {
    DisplayInfo {
        id: DisplayId(0),
        name: String::from("StubDisplay"),
        resolution: PhysicalSize {
            width: 1920,
            height: 1080,
        },
        refresh_rate_mhz: 60_000,
        color_depth: 8,
        hdr_capable: false,
        position: Point { x: 0.0, y: 0.0 },
        dpi: 1.0,
        primary: true,
        available_refresh_rates: vec![60_000],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use raw_window_handle::{HandleError, HasDisplayHandle, HasWindowHandle};

    fn drain_all(w: &mut Window) -> Vec<WindowEvent> {
        w.events().collect()
    }

    #[test]
    fn tc_14_1_1_i1_window_lifecycle() {
        let mut window = Window::new(WindowConfig::default()).expect("window");
        let _initial = drain_all(&mut window);

        window.set_size(LogicalSize {
            width: 1920.0,
            height: 1080.0,
        });
        let after_resize: Vec<_> = drain_all(&mut window);
        assert!(
            after_resize.iter().any(|e| {
                matches!(
                    e,
                    WindowEvent::Resized { physical, .. }
                        if physical.width == 1920 && physical.height == 1080
                )
            }),
            "expected Resized 1920x1080, got {after_resize:?}"
        );

        window.minimize();
        assert_eq!(drain_all(&mut window), vec![WindowEvent::Minimized]);

        window.maximize();
        assert_eq!(drain_all(&mut window), vec![WindowEvent::Maximized]);

        window.restore();
        assert_eq!(drain_all(&mut window), vec![WindowEvent::Restored]);

        let sink = window.test_event_sink();
        drop(window);
        let after_drop = sink.borrow();
        assert!(
            after_drop.iter().any(|e| matches!(e, WindowEvent::Destroyed)),
            "expected Destroyed after drop, got {after_drop:?}"
        );
    }

    #[test]
    fn surface_handle_display_stub_window_not_yet_wired() {
        let window = Window::new(WindowConfig::default()).expect("window");
        let surface = window.surface_handle();
        assert!(matches!(
            surface.window_handle(),
            Err(HandleError::NotSupported)
        ));
        assert!(surface.display_handle().is_ok());
    }
}
