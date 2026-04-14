//! CPU swimlane / flame graph view model (design: `CpuTimeline`).

use crate::frame_collector::FrameCapture;
use crate::ring_buffer::CpuEvent;

/// Filter for the CPU timeline view.
#[derive(Clone, Debug, Default)]
pub struct TimelineFilter {
    /// When `Some`, only matching thread IDs are shown.
    pub thread_ids: Option<Vec<u32>>,
    /// Reserved for subsystem tagging (not wired to `CpuEvent` yet).
    pub subsystem_names: Option<Vec<String>>,
    /// Minimum zone duration in microseconds.
    pub min_duration_us: Option<f64>,
}

/// View mode for the CPU profiler.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CpuProfileViewMode {
    /// Swimlane chart with one lane per thread.
    Timeline,
    /// Flame graph (call stack depth view).
    FlameGraph,
    /// Flat profile (sorted by total time).
    FlatProfile,
}

/// CPU timeline viewer state (pure data; rendering lives in tools).
#[derive(Debug)]
pub struct CpuTimeline {
    view_mode: CpuProfileViewMode,
    filter: TimelineFilter,
    frame: Option<FrameCapture>,
    baseline: Option<FrameCapture>,
    comparison: Option<FrameCapture>,
    displayed: Vec<CpuEvent>,
}

impl CpuTimeline {
    /// Builds an empty viewer.
    #[must_use]
    pub fn new() -> Self {
        Self {
            view_mode: CpuProfileViewMode::Timeline,
            filter: TimelineFilter::default(),
            frame: None,
            baseline: None,
            comparison: None,
            displayed: Vec::new(),
        }
    }

    /// Sets the view mode.
    pub fn set_view_mode(&mut self, mode: CpuProfileViewMode) {
        self.view_mode = mode;
    }

    /// Applies a filter and refreshes [`CpuTimeline::displayed_events`].
    pub fn set_filter(&mut self, filter: TimelineFilter) {
        self.filter = filter;
        self.refresh_displayed();
    }

    /// Sets the active frame (clears comparison mode).
    pub fn set_frame(&mut self, capture: &FrameCapture) {
        self.frame = Some(capture.clone());
        self.baseline = None;
        self.comparison = None;
        self.refresh_displayed();
    }

    /// Enables frame-to-frame comparison (clears single-frame mode).
    pub fn set_comparison(&mut self, baseline: &FrameCapture, current: &FrameCapture) {
        self.baseline = Some(baseline.clone());
        self.comparison = Some(current.clone());
        self.frame = None;
        self.refresh_displayed();
    }

    /// Placeholder selection hook (no picking in this crate build).
    #[must_use]
    pub fn selected_zone(&self) -> Option<&CpuEvent> {
        None
    }

    /// Events after the active filter for the current [`CpuTimeline::set_frame`] or comparison
    /// frame (uses `current` when comparing).
    #[must_use]
    pub fn displayed_events(&self) -> &[CpuEvent] {
        self.displayed.as_slice()
    }

    /// Delta in milliseconds for one zone hash between comparison frames.
    #[must_use]
    pub fn comparison_delta_ms_for_zone(&self, zone_name_hash: u32) -> Option<f64> {
        let baseline = self.baseline.as_ref()?;
        let current = self.comparison.as_ref()?;
        let base_ms = zone_duration_ms(&baseline.cpu_events, zone_name_hash);
        let cur_ms = zone_duration_ms(&current.cpu_events, zone_name_hash);
        Some(cur_ms - base_ms)
    }

    fn refresh_displayed(&mut self) {
        let source = self
            .comparison
            .as_ref()
            .or(self.frame.as_ref())
            .cloned();
        let Some(cap) = source else {
            self.displayed.clear();
            return;
        };
        let mut events: Vec<CpuEvent> = cap.cpu_events.to_vec();
        if let Some(ref ids) = self.filter.thread_ids {
            events.retain(|e| ids.contains(&e.thread_id));
        }
        if let Some(min_us) = self.filter.min_duration_us {
            events.retain(|e| {
                let ticks = e.end_tsc.saturating_sub(e.begin_tsc);
                let us = ticks_to_us(ticks);
                us + f64::EPSILON >= min_us
            });
        }
        self.displayed = events;
    }
}

impl Default for CpuTimeline {
    fn default() -> Self {
        Self::new()
    }
}

fn ticks_to_us(ticks: u64) -> f64 {
    // Matches `FrameCollector` tick scale used in unit tests (see `frame_collector` tests).
    (ticks as f64) / 1_000.0
}

fn zone_duration_ms(events: &[CpuEvent], zone_name_hash: u32) -> f64 {
    events
        .iter()
        .filter(|e| e.zone_name_hash == zone_name_hash)
        .map(|e| (e.end_tsc.saturating_sub(e.begin_tsc)) as f64 / 1_000_000.0)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame_collector::FrameCapture;
    use crate::types::FrameStats;

    fn ev(tid: u32, begin: u64, end: u64, zone: u32) -> CpuEvent {
        CpuEvent {
            begin_tsc: begin,
            end_tsc: end,
            thread_id: tid,
            zone_name_hash: zone,
            depth: 0,
        }
    }

    /// TC-15.5.1.5 #1 — filter thread 2 only.
    #[test]
    fn tc_15_5_1_5_timeline_filter_thread() {
        let cap = FrameCapture {
            frame_number: 1,
            frame_time_ms: 1.0,
            cpu_events: vec![ev(1, 0, 1, 1), ev(2, 0, 1, 2), ev(3, 0, 1, 3)],
            gpu_passes: Vec::new(),
            stats: FrameStats::default(),
        };
        let mut tl = CpuTimeline::new();
        tl.set_frame(&cap);
        tl.set_filter(TimelineFilter {
            thread_ids: Some(vec![2]),
            subsystem_names: None,
            min_duration_us: None,
        });
        assert_eq!(tl.displayed_events().len(), 1);
        assert_eq!(tl.displayed_events()[0].thread_id, 2);
    }

    /// TC-15.5.1.5 #2 — `thread_ids: None` shows all threads.
    #[test]
    fn tc_15_5_1_5_timeline_filter_none_shows_all() {
        let cap = FrameCapture {
            frame_number: 1,
            frame_time_ms: 1.0,
            cpu_events: vec![ev(1, 0, 1, 1), ev(2, 0, 1, 2)],
            gpu_passes: Vec::new(),
            stats: FrameStats::default(),
        };
        let mut tl = CpuTimeline::new();
        tl.set_frame(&cap);
        tl.set_filter(TimelineFilter {
            thread_ids: None,
            subsystem_names: None,
            min_duration_us: None,
        });
        assert_eq!(tl.displayed_events().len(), 2);
    }

    /// TC-15.5.1.6 #1 — frame comparison delta for one zone.
    #[test]
    fn tc_15_5_1_6_frame_comparison_delta() {
        let zone = 99_u32;
        let frame_a = FrameCapture {
            frame_number: 1,
            frame_time_ms: 10.0,
            cpu_events: vec![ev(0, 0, 5_000_000, zone)],
            gpu_passes: Vec::new(),
            stats: FrameStats::default(),
        };
        let frame_b = FrameCapture {
            frame_number: 2,
            frame_time_ms: 10.0,
            cpu_events: vec![ev(0, 0, 8_000_000, zone)],
            gpu_passes: Vec::new(),
            stats: FrameStats::default(),
        };
        let mut tl = CpuTimeline::new();
        tl.set_comparison(&frame_a, &frame_b);
        let delta = tl.comparison_delta_ms_for_zone(zone).expect("delta");
        assert!((delta - 3.0).abs() < 0.001);
    }
}
