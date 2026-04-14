//! Cross-platform windowing configuration, sizing, HDR, and event delivery helpers.

pub mod coordinates;
pub mod display;
pub mod event_channel;
pub mod hdr;
pub mod mode;
pub mod window_config;

pub use coordinates::{LogicalSize, PhysicalSize, Point, Rect};
pub use display::DisplayId;
pub use event_channel::AutoSizingEventChannel;
pub use hdr::{ColorSpace, HdrConfig};
pub use mode::{FrameRateCap, PresentMode, RefreshRate, WindowMode};
pub use window_config::{DpiPolicy, WindowConfig};
