//! Platform windowing and threading.

#![deny(clippy::all)]

pub mod platform;
pub mod threading;
pub mod windowing;

pub use windowing::{
    LogicalSize, PhysicalSize, SurfaceEvent, SurfaceHandle, SurfaceNativeExt, Window, WindowConfig,
    WindowEvent, WindowHandle,
};
