//! Window display modes and presentation preferences.

use crate::windowing::DisplayId;

/// Refresh rate in millihertz for sub-Hz precision (e.g., 59940 = 59.94 Hz).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RefreshRate(pub u32);

/// Window display mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowMode {
    /// Standard windowed mode with title bar and borders.
    Windowed,
    /// Borderless window covering the full display area.
    BorderlessFullscreen(DisplayId),
    /// Exclusive fullscreen with direct scanout.
    ExclusiveFullscreen(DisplayId, RefreshRate),
}

/// Swapchain presentation mode requested by the application.
///
/// Actual present scheduling is owned by the render pipeline; this value
/// records the engine-level preference carried with window configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PresentMode {
    /// Present immediately without waiting for vertical blank.
    Immediate,
    /// VSync: wait for vertical blank (FIFO).
    Fifo,
    /// Low-latency triple-buffered presentation when supported.
    Mailbox,
}

/// Optional cap on the number of frames presented per second.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrameRateCap {
    /// Cap at the given maximum FPS.
    Capped(u32),
    /// No artificial frame rate cap.
    Uncapped,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_14_1_5_1_present_mode_variants() {
        let modes = [
            PresentMode::Immediate,
            PresentMode::Fifo,
            PresentMode::Mailbox,
        ];
        assert_eq!(modes.len(), 3);
        assert_ne!(modes[0], modes[1]);
        assert_ne!(modes[1], modes[2]);
        assert_ne!(modes[0], modes[2]);
    }

    #[test]
    fn tc_14_1_5_2_frame_rate_cap_distinct() {
        assert_ne!(FrameRateCap::Capped(30), FrameRateCap::Uncapped);
    }
}
