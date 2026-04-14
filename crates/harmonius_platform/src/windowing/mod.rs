//! Cross-platform windowing configuration, sizing, HDR, and event delivery helpers.

pub mod coordinates;
pub mod cursor;
pub mod display;
pub mod error;
pub mod event;
pub mod event_channel;
pub mod hdr;
pub mod mode;
pub mod surface;
pub mod window;
pub mod window_config;

pub use coordinates::{LogicalSize, PhysicalSize, Point, Rect};
pub use cursor::CursorIcon;
pub use display::{DisplayId, DisplayInfo};
pub use error::WindowError;
pub use event::WindowEvent;
pub use event_channel::AutoSizingEventChannel;
pub use hdr::{ColorSpace, HdrConfig, HdrError};
pub use mode::{FrameRateCap, PresentMode, RefreshRate, WindowMode};
pub use surface::{SurfaceEvent, SurfaceHandle};
pub use window::{EventIterator, SurfaceEventIterator, Window, WindowHandle};
pub use window_config::{DpiPolicy, WindowConfig};
