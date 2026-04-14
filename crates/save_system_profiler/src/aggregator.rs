//! Profiler-side aggregation for save profile streams.

use std::collections::{BTreeMap, HashSet, VecDeque};

use crate::types::{DebugFlags, IoError, ProfileMessage, SaveMetrics, SavePhase, SaveProfileEvent};

/// Phase 8 save-time budget in milliseconds (high-level performance budget).
pub const PHASE8_SAVE_BUDGET_MS: f64 = 0.5;

/// Rolling window capacity for duration statistics (TC-IR-8.1.5.3).
pub const DURATION_WINDOW_CAP: usize = 100;

/// Profiler-owned aggregation state updated while draining CH-24.
#[derive(Debug)]
pub struct SaveProfilerAggregator {
    metrics: SaveMetrics,
    /// `save_id` values that received a successful finalize in order.
    finalized_saves: HashSet<u64>,
    /// Sum of Phase-8-counted save slice durations for the current frame.
    phase8_save_ms_this_frame: f64,
    /// Set when [`Self::end_frame`] sees a Phase 8 over-budget condition.
    pub phase8_budget_breach: bool,
    /// Main-thread write exceeded the HUD slow-write threshold (8 ms).
    pub io_write_slow_warning: bool,
    /// Last write duration observed for assertions (TC-IR-8.1.2.x).
    pub last_io_write_duration_ms: f64,
    /// FM-2: RSS sampler failures falling back to last-known memory.
    pub fm2_rss_failures: u64,
    /// FM-5: schema events arriving after finalize for a save.
    pub fm5_orphan_schema: u64,
    /// Last successfully applied memory snapshot (FM-2 fallback source).
    last_memory: Option<(u64, u64, u64)>,
    /// Rolling window of completed-save total durations for P99 (TC-IR-8.1.5.3).
    duration_window: VecDeque<f64>,
}

impl Default for SaveProfilerAggregator {
    fn default() -> Self {
        Self::new()
    }
}

impl SaveProfilerAggregator {
    /// Builds empty aggregation state.
    pub fn new() -> Self {
        Self {
            metrics: SaveMetrics::default(),
            finalized_saves: HashSet::new(),
            phase8_save_ms_this_frame: 0.0,
            phase8_budget_breach: false,
            io_write_slow_warning: false,
            last_io_write_duration_ms: 0.0,
            fm2_rss_failures: 0,
            fm5_orphan_schema: 0,
            last_memory: None,
            duration_window: VecDeque::new(),
        }
    }

    /// Borrow rolling metrics for assertions.
    pub fn metrics(&self) -> &SaveMetrics {
        &self.metrics
    }

    /// Mutable metrics for tests that set expectations directly.
    pub fn metrics_mut(&mut self) -> &mut SaveMetrics {
        &mut self.metrics
    }

    /// Drains every queued profile message in FIFO order.
    pub fn drain(&mut self, channel: &mut crate::channel::SaveProfileChannel) {
        while let Some(msg) = channel.try_recv() {
            self.apply_message(msg);
        }
    }

    /// Applies a single message without routing through a channel (tests / tooling).
    pub fn ingest(&mut self, msg: ProfileMessage) {
        self.apply_message(msg);
    }

    /// Returns true after a finalize marker was observed for `save_id`.
    #[must_use]
    pub fn is_save_finalized(&self, save_id: u64) -> bool {
        self.finalized_saves.contains(&save_id)
    }

    fn counts_toward_phase8(phase: SavePhase) -> bool {
        !matches!(phase, SavePhase::Write)
    }

    fn apply_message(&mut self, msg: ProfileMessage) {
        match msg {
            ProfileMessage::SaveProfile(ev) => self.apply_save_profile(ev),
            ProfileMessage::IoWrite(ev) => self.apply_io_write(ev),
            ProfileMessage::Memory(ev) => self.apply_memory(ev),
            ProfileMessage::Schema(ev) => self.apply_schema(ev),
        }
    }

    fn apply_save_profile(&mut self, ev: SaveProfileEvent) {
        if Self::counts_toward_phase8(ev.phase) {
            self.phase8_save_ms_this_frame += ev.duration_ms;
        }
        if ev.phase == SavePhase::Finalize {
            self.finalized_saves.insert(ev.save_id);
        }
    }

    fn apply_io_write(&mut self, ev: crate::types::IoWriteEvent) {
        self.last_io_write_duration_ms = ev.duration_ms;
        if ev.duration_ms > 8.0 {
            self.io_write_slow_warning = true;
        }
        if ev.error != IoError::None {
            self.metrics.total_failures += 1;
            self.metrics.last_error_code = io_error_code(ev.error);
        }
    }

    fn apply_memory(&mut self, ev: crate::types::MemorySnapshotEvent) {
        self.last_memory = Some((ev.rss_before, ev.rss_after, ev.peak_during));
    }

    fn apply_schema(&mut self, ev: crate::types::SchemaBreakdownEvent) {
        if self.finalized_saves.contains(&ev.save_id) {
            self.fm5_orphan_schema += 1;
        }
    }

    /// Marks the end of a frame: checks Phase 8 budget and clears per-frame sums.
    pub fn end_frame(&mut self) {
        if self.phase8_save_ms_this_frame > PHASE8_SAVE_BUDGET_MS {
            self.phase8_budget_breach = true;
        }
        self.phase8_save_ms_this_frame = 0.0;
    }

    /// Records a successful save completion with measured total duration.
    pub fn record_save_success(&mut self, total_duration_ms: f64) {
        self.metrics.total_saves += 1;
        self.push_duration_sample(total_duration_ms);
    }

    fn push_duration_sample(&mut self, d: f64) {
        self.duration_window.push_back(d);
        while self.duration_window.len() > DURATION_WINDOW_CAP {
            self.duration_window.pop_front();
        }
        self.recompute_rolling_stats();
    }

    fn recompute_rolling_stats(&mut self) {
        let n = self.duration_window.len();
        if n == 0 {
            self.metrics.avg_duration_ms = 0.0;
            self.metrics.p99_duration_ms = 0.0;
            return;
        }
        let sum: f64 = self.duration_window.iter().sum();
        self.metrics.avg_duration_ms = sum / n as f64;
        let mut sorted: Vec<f64> = self.duration_window.iter().copied().collect();
        sorted.sort_by(|a, b| a.total_cmp(b));
        let idx = ((n as f64) * 0.99).ceil() as usize;
        let idx = idx.saturating_sub(1).min(n - 1);
        self.metrics.p99_duration_ms = sorted[idx];
    }

    /// Increments FM-2 counter when RSS sampling fails in the save harness.
    pub fn record_rss_sample_failure(&mut self) {
        self.fm2_rss_failures += 1;
    }

    /// Applies FM-2 policy: reuse last-known RSS triple when sampling fails.
    pub fn memory_snapshot_or_last_known(
        &mut self,
        save_id: u64,
        sample: Result<(u64, u64, u64), ()>,
    ) -> crate::types::MemorySnapshotEvent {
        match sample {
            Ok((b, a, p)) => {
                let ev = crate::types::MemorySnapshotEvent {
                    save_id,
                    rss_before: b,
                    rss_after: a,
                    peak_during: p,
                };
                self.last_memory = Some((b, a, p));
                ev
            }
            Err(()) => {
                self.record_rss_sample_failure();
                let (b, a, p) = self.last_memory.unwrap_or((0, 0, 0));
                crate::types::MemorySnapshotEvent {
                    save_id,
                    rss_before: b,
                    rss_after: a,
                    peak_during: p,
                }
            }
        }
    }
}

/// Returns true when the HUD should draw the Phase 8 bar red for budget breach (TC-IR-8.1.4.2).
#[must_use]
pub fn hud_phase8_bar_red(flags: &DebugFlags, agg: &SaveProfilerAggregator) -> bool {
    flags.show_profiler_hud && agg.phase8_budget_breach
}

fn io_error_code(e: IoError) -> u64 {
    match e {
        IoError::None => 0,
        IoError::Interrupted => 1,
        IoError::DiskFull => 2,
        IoError::Permission => 3,
        IoError::Unknown => 4,
    }
}

/// Sums schema breakdown bytes and compares to archive bytes (TC-IR-8.1.6.1).
pub fn schema_bytes_sum(events: &[crate::types::SchemaBreakdownEvent]) -> u64 {
    events.iter().map(|e| e.bytes).sum()
}

/// Builds a sorted map of schema node totals (SC-2 style ordering).
pub fn schema_totals_map(
    events: &[crate::types::SchemaBreakdownEvent],
) -> BTreeMap<crate::types::SchemaNodeId, u64> {
    let mut m: BTreeMap<crate::types::SchemaNodeId, u64> = BTreeMap::new();
    for e in events {
        *m.entry(e.node).or_insert(0) += e.bytes;
    }
    m
}
