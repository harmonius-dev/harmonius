//! Window configuration and lifecycle.

use harmonius_core::EngineError;

use super::dpi::{LogicalSize, PhysicalSize};
use super::event::{SurfaceEvent, WindowEvent};
use super::surface::SurfaceHandle;
use crate::platform;

/// Opaque window identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WindowHandle {
    id: u64,
}

impl WindowHandle {
    /// Returns the stable window id.
    #[must_use]
    pub fn id(self) -> u64 {
        self.id
    }
}

/// Configuration for creating a window.
#[derive(Clone, Debug, PartialEq)]
pub struct WindowConfig {
    /// Title bar text.
    pub title: String,
    /// Initial client size in logical pixels.
    pub size: LogicalSize,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: String::from("Harmonius"),
            size: LogicalSize {
                width: 1280.0,
                height: 720.0,
            },
        }
    }
}

/// Native OS window.
pub struct Window {
    inner: platform::NativeWindow,
    handle: WindowHandle,
    scale_factor: f64,
}

impl Window {
    /// Creates a new native window.
    pub fn new(config: WindowConfig) -> Result<Self, EngineError> {
        let inner = platform::NativeWindow::create(&config)?;
        let handle = WindowHandle {
            id: inner.window_id(),
        };
        let scale_factor = inner.scale_factor();
        Ok(Self {
            inner,
            handle,
            scale_factor,
        })
    }

    /// Returns this window's handle.
    #[must_use]
    pub fn handle(&self) -> WindowHandle {
        self.handle
    }

    /// Current DPI scale factor (1.0 = 100%).
    #[must_use]
    pub fn scale_factor(&self) -> f64 {
        self.scale_factor
    }

    /// Client area size in physical pixels.
    #[must_use]
    pub fn physical_size(&self) -> PhysicalSize {
        self.inner.physical_size()
    }

    /// Returns a surface handle for GPU swapchain creation.
    #[must_use]
    pub fn surface_handle(&self) -> SurfaceHandle {
        self.inner.surface_handle(self.scale_factor)
    }

    /// Pumps pending OS events.
    pub fn poll_events(&mut self) -> Result<Vec<WindowEvent>, EngineError> {
        let events = self.inner.pump_events()?;
        for event in &events {
            if let WindowEvent::Resized(size) = event {
                self.inner.on_resize(*size);
            }
        }
        Ok(events)
    }

    /// Maps window events to render-thread surface events.
    #[must_use]
    pub fn surface_events(events: &[WindowEvent]) -> Vec<SurfaceEvent> {
        events
            .iter()
            .filter_map(|event| match event {
                WindowEvent::Resized(size) => Some(SurfaceEvent::Resized(*size)),
                WindowEvent::CloseRequested => Some(SurfaceEvent::Invalidated),
                WindowEvent::FocusChanged(_) => None,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::WindowConfig;

    #[test]
    fn test_window_config_default() {
        let config = WindowConfig::default();
        assert_eq!(config.title, "Harmonius");
        assert!((config.size.width - 1280.0).abs() < f64::EPSILON);
        assert!((config.size.height - 720.0).abs() < f64::EPSILON);
    }
}
