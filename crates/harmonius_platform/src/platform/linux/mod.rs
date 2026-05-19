//! Linux platform backend (X11).

mod window;

pub use window::NativeWindow;

/// Wayland support is planned; see `docs/design/platform/windowing.md`.
pub mod wayland {}
