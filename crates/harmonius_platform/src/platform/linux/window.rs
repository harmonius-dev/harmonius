//! Linux X11 windowing via `x11rb`.

use std::num::NonZeroU32;
use std::ptr::NonNull;

use harmonius_core::EngineError;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle, XlibDisplayHandle, XlibWindowHandle};
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{AtomEnum, ConnectionExt, Event, Window as XWindow};
use x11rb::rust_connection::RustConnection;

use crate::windowing::event::WindowEvent;
use crate::windowing::surface::SurfaceHandle;
use crate::windowing::window::{PhysicalSize, WindowConfig};

/// X11-backed native window.
pub struct NativeWindow {
    conn: RustConnection,
    window: XWindow,
    screen: usize,
    wm_protocols: u32,
    wm_delete: u32,
    window_id: u64,
    physical_size: PhysicalSize,
    scale_factor: f64,
}

impl NativeWindow {
    /// Opens a display connection and creates an X11 window.
    pub fn create(config: &WindowConfig) -> Result<Self, EngineError> {
        let (conn, screen_idx) = RustConnection::connect(None)
            .map_err(|e| EngineError::Platform(format!("X11 connect failed: {e}")))?;
        let screen = &conn.setup().roots[screen_idx];
        let scale = 1.0_f64;
        let physical = config.size.to_physical(scale);
        let window_id = conn
            .generate_id()
            .map_err(|e| EngineError::Platform(format!("X11 generate_id failed: {e}")))?;

        let win = window_id;
        conn.create_window(
            win,
            screen.root,
            0,
            0,
            physical.width as u16,
            physical.height as u16,
            0,
            x11rb::protocol::xproto::WindowClass::INPUT_OUTPUT,
            screen.root_visual,
            &x11rb::protocol::xproto::CreateWindowAux::new()
                .event_mask(
                    x11rb::protocol::xproto::EventMask::STRUCTURE_NOTIFY
                        | x11rb::protocol::xproto::EventMask::EXPOSURE
                        | x11rb::protocol::xproto::EventMask::KEY_PRESS,
                )
                .background_pixel(screen.white_pixel),
        )
        .map_err(|e| EngineError::Platform(format!("X11 create_window failed: {e}")))?;

        let title = config.title.as_str();
        conn.change_property(
            win,
            AtomEnum::WM_NAME.into(),
            AtomEnum::STRING.into(),
            8,
            title.len() as u32,
            title.as_bytes(),
        )
        .map_err(|e| EngineError::Platform(format!("X11 WM_NAME failed: {e}")))?;

        let wm_protocols = conn
            .intern_atom(false, b"WM_PROTOCOLS")
            .map_err(|e| EngineError::Platform(format!("X11 intern_atom WM_PROTOCOLS: {e}")))?
            .reply()
            .map_err(|e| EngineError::Platform(format!("X11 WM_PROTOCOLS reply: {e}")))?
            .atom;
        let wm_delete = conn
            .intern_atom(false, b"WM_DELETE_WINDOW")
            .map_err(|e| EngineError::Platform(format!("X11 intern_atom WM_DELETE_WINDOW: {e}")))?
            .reply()
            .map_err(|e| EngineError::Platform(format!("X11 WM_DELETE_WINDOW reply: {e}")))?
            .atom;

        conn.change_property32(win, wm_protocols, AtomEnum::ATOM.into(), &[wm_delete])
            .map_err(|e| EngineError::Platform(format!("X11 WM_PROTOCOLS property: {e}")))?;

        conn.map_window(win)
            .map_err(|e| EngineError::Platform(format!("X11 map_window failed: {e}")))?;
        conn.flush()
            .map_err(|e| EngineError::Platform(format!("X11 flush failed: {e}")))?;

        Ok(Self {
            conn,
            window: win,
            screen: screen_idx,
            wm_protocols,
            wm_delete,
            window_id: u64::from(win),
            physical_size: physical,
            scale_factor: scale,
        })
    }

    /// Stable window id.
    pub fn window_id(&self) -> u64 {
        self.window_id
    }

    /// DPI scale factor.
    pub fn scale_factor(&self) -> f64 {
        self.scale_factor
    }

    /// Physical client size.
    pub fn physical_size(&self) -> PhysicalSize {
        self.physical_size
    }

    /// Vulkan `VK_KHR_xlib_surface` interop handle.
    pub fn surface_handle(&self, scale_factor: f64) -> SurfaceHandle {
        let display = self.conn.display().as_ptr() as *mut _;
        let raw_window = RawWindowHandle::Xlib(XlibWindowHandle::new({
            NonZeroU32::new(self.window).expect("X11 window id is non-zero")
        }));
        let raw_display = RawDisplayHandle::Xlib(XlibDisplayHandle::new({
            // SAFETY: display pointer valid for connection lifetime.
            unsafe { NonNull::new_unchecked(display) }
        }));
        SurfaceHandle::new(
            raw_window,
            raw_display,
            self.physical_size,
            scale_factor,
            false,
            None,
        )
    }

    /// Updates cached size after resize.
    pub fn on_resize(&mut self, size: PhysicalSize) {
        self.physical_size = size;
    }

    /// Polls pending X11 events.
    pub fn pump_events(&mut self) -> Result<Vec<WindowEvent>, EngineError> {
        let mut out = Vec::new();
        while let Some(event) = self
            .conn
            .poll_for_event()
            .map_err(|e| EngineError::Platform(format!("X11 poll_for_event: {e}")))?
        {
            match event {
                Event::ConfigureNotify(cfg) if cfg.window == self.window => {
                    let size = PhysicalSize {
                        width: cfg.width.max(1) as u32,
                        height: cfg.height.max(1) as u32,
                    };
                    if size != self.physical_size {
                        self.physical_size = size;
                        out.push(WindowEvent::Resized(size));
                    }
                }
                Event::ClientMessage(msg)
                    if msg.window == self.window
                        && msg.format == 32
                        && msg.type_ == self.wm_protocols
                        && msg.data.as_data32()[0] == self.wm_delete =>
                {
                    out.push(WindowEvent::CloseRequested);
                }
                _ => {}
            }
        }
        Ok(out)
    }
}
