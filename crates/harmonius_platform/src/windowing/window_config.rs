//! Window creation configuration shared across platforms.

use crate::windowing::{DisplayId, HdrConfig, LogicalSize, PresentMode, WindowMode};

/// Controls how the window responds to DPI changes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DpiPolicy {
    /// Scale the window content and let the OS resize the window to the suggested rectangle.
    SystemScaled,
    /// Keep the window size fixed in physical pixels; the application scales internally.
    ApplicationScaled,
}

/// Configuration for creating a new window.
#[derive(Clone, Debug, PartialEq)]
pub struct WindowConfig {
    /// Window title displayed in the title bar and taskbar.
    pub title: String,
    /// Initial logical size of the client area.
    pub size: LogicalSize,
    /// Initial window mode.
    pub mode: WindowMode,
    /// Preferred swapchain presentation mode for the render pipeline.
    pub present_mode: PresentMode,
    /// DPI handling policy.
    pub dpi_policy: DpiPolicy,
    /// Initial HDR configuration.
    pub hdr: HdrConfig,
    /// Whether the window is resizable.
    pub resizable: bool,
    /// Whether to show window decorations.
    pub decorations: bool,
    /// Whether the window supports transparency.
    pub transparent: bool,
    /// Target display for initial placement (`None` = primary).
    pub target_display: Option<DisplayId>,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: String::from("Harmonius"),
            size: LogicalSize {
                width: 1280.0,
                height: 720.0,
            },
            mode: WindowMode::Windowed,
            present_mode: PresentMode::Fifo,
            dpi_policy: DpiPolicy::SystemScaled,
            hdr: HdrConfig::disabled(),
            resizable: true,
            decorations: true,
            transparent: false,
            target_display: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_14_1_1_1_window_config_default() {
        let cfg = WindowConfig::default();
        assert_eq!(cfg.size.width, 1280.0);
        assert_eq!(cfg.size.height, 720.0);
        assert_eq!(cfg.mode, WindowMode::Windowed);
        assert_eq!(cfg.present_mode, PresentMode::Fifo);
        assert_eq!(cfg.dpi_policy, DpiPolicy::SystemScaled);
        assert_eq!(cfg.hdr, HdrConfig::disabled());
        assert!(!cfg.hdr.enabled);
    }
}
