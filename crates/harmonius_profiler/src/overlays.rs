//! HUD stat overlays (design: `StatOverlays`).

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use crate::frame_collector::FrameCapture;

/// One overlay channel that can be toggled.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StatOverlay {
    /// Frames per second text.
    Fps,
    /// Frame time in milliseconds.
    FrameTime,
    /// Draw call counter.
    DrawCalls,
    /// Triangle counter.
    TriangleCount,
    /// GPU memory readout.
    GpuMemory,
    /// CPU thread utilization bars.
    CpuThreadUtilization,
    /// Network bandwidth readout.
    NetworkBandwidth,
    /// ECS entity count.
    EntityCount,
}

/// HUD overlay configuration.
#[derive(Clone, Debug, Default)]
pub struct OverlayConfig {
    /// Enabled overlays (stable order).
    pub enabled: Vec<StatOverlay>,
    /// Compact layout for small screens.
    pub compact_mode: bool,
    /// Corner placement.
    pub position: OverlayPosition,
}

/// Corner placement for the overlay cluster.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum OverlayPosition {
    #[default]
    /// Upper-left HUD anchor.
    TopLeft,
    /// Upper-right HUD anchor.
    TopRight,
    /// Lower-left HUD anchor.
    BottomLeft,
    /// Lower-right HUD anchor.
    BottomRight,
}

/// Viewport stat overlay state (minimal CPU-side model).
#[derive(Debug)]
pub struct StatOverlays {
    config: OverlayConfig,
    last_fps: f64,
    csv_path: Option<String>,
    csv_rows: Vec<String>,
}

impl StatOverlays {
    /// Default overlays with FPS enabled.
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: OverlayConfig {
                enabled: vec![StatOverlay::Fps],
                compact_mode: false,
                position: OverlayPosition::TopLeft,
            },
            last_fps: 0.0,
            csv_path: None,
            csv_rows: Vec::new(),
        }
    }

    /// Enables or disables a single overlay channel.
    pub fn set_enabled(&mut self, overlay: StatOverlay, enabled: bool) {
        if enabled {
            if !self.config.enabled.contains(&overlay) {
                self.config.enabled.push(overlay);
            }
        } else {
            self.config.enabled.retain(|&o| o != overlay);
        }
    }

    /// Sets compact layout mode.
    pub fn set_compact(&mut self, compact: bool) {
        self.config.compact_mode = compact;
    }

    /// Starts CSV capture to `path` (writes header immediately).
    pub fn start_csv_recording(&mut self, path: &'static str) {
        self.csv_path = Some(path.to_owned());
        self.csv_rows.clear();
        self.csv_rows.push("frame,fps".to_string());
    }

    /// Stops CSV capture and flushes rows to disk when a path is set.
    pub fn stop_csv_recording(&mut self) {
        if let Some(ref p) = self.csv_path.take() {
            let _ = flush_csv(p, &self.csv_rows);
        }
    }

    /// Updates cached overlay metrics from `capture`.
    pub fn update(&mut self, capture: &FrameCapture) {
        if self.config.enabled.contains(&StatOverlay::Fps) && capture.frame_time_ms > f64::EPSILON
        {
            self.last_fps = 1000.0 / capture.frame_time_ms;
        } else {
            self.last_fps = 0.0;
        }
        if self.csv_path.is_some() {
            self.csv_rows
                .push(format!("{},{}", capture.frame_number, self.last_fps));
        }
    }

    /// Last computed FPS after [`StatOverlays::update`].
    #[must_use]
    pub fn fps_value(&self) -> f64 {
        self.last_fps
    }

    /// Whether `overlay` is enabled in [`StatOverlays::update`] output.
    #[must_use]
    pub fn is_overlay_enabled(&self, overlay: StatOverlay) -> bool {
        self.config.enabled.contains(&overlay)
    }

    /// Whether FPS would be emitted (enabled + positive fps).
    #[must_use]
    pub fn fps_overlay_active(&self) -> bool {
        self.config.enabled.contains(&StatOverlay::Fps) && self.last_fps > f64::EPSILON
    }
}

impl Default for StatOverlays {
    fn default() -> Self {
        Self::new()
    }
}

fn flush_csv(path: &str, rows: &[String]) -> std::io::Result<()> {
    let p = Path::new(path);
    if let Some(dir) = p.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let mut f = OpenOptions::new().create(true).write(true).truncate(true).open(p)?;
    for line in rows {
        writeln!(f, "{line}")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame_collector::FrameCapture;
    use crate::types::FrameStats;

    /// TC-15.5.6.1 — FPS derived from frame time.
    #[test]
    fn tc_15_5_6_1_overlay_fps_nonzero() {
        let mut o = StatOverlays::new();
        let cap = FrameCapture {
            frame_number: 1,
            frame_time_ms: 16.0,
            cpu_events: Vec::new(),
            gpu_passes: Vec::new(),
            stats: FrameStats::default(),
        };
        o.update(&cap);
        assert!(o.fps_value() > 0.0);
    }

    /// TC-15.5.6.2 — toggle FPS overlay.
    #[test]
    fn tc_15_5_6_2_overlay_toggle() {
        let mut o = StatOverlays::new();
        o.set_enabled(StatOverlay::Fps, false);
        assert!(!o.is_overlay_enabled(StatOverlay::Fps));
        o.set_enabled(StatOverlay::Fps, true);
        assert!(o.is_overlay_enabled(StatOverlay::Fps));
    }

    /// TC-15.5.6.3 — CSV rows include one line per recorded frame.
    #[test]
    fn tc_15_5_6_3_csv_export() {
        let dir = std::env::temp_dir();
        let path = dir.join("harmonius_profiler_overlay_test.csv");
        let path_str = path.to_string_lossy().to_string();
        let leaked: &'static str = Box::leak(path_str.into_boxed_str());
        let mut o = StatOverlays::new();
        o.start_csv_recording(leaked);
        for i in 1..=10_u64 {
            let cap = FrameCapture {
                frame_number: i,
                frame_time_ms: 10.0,
                cpu_events: Vec::new(),
                gpu_passes: Vec::new(),
                stats: FrameStats::default(),
            };
            o.update(&cap);
        }
        o.stop_csv_recording();
        let text = std::fs::read_to_string(&path).expect("read csv");
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines.len(), 11);
        let _ = std::fs::remove_file(&path);
    }
}
