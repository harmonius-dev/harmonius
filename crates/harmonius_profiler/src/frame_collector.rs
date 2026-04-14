//! Per-frame aggregation: drains per-thread ring buffers and sorts CPU events.

use std::sync::{Arc, RwLock};

use crate::ring_buffer::{CpuEvent, FrameArena, ProfileRingBuffer};
use crate::types::{FrameStats, GpuPassTiming};

/// GPU timestamp query pool (stubbed until render-graph integration lands).
#[derive(Debug)]
pub struct QueryPool {
    _capacity: u32,
    resolved: RwLock<Vec<GpuPassTiming>>,
}

impl QueryPool {
    /// Allocates a query pool ring of `capacity` slots (must be > 0).
    #[must_use]
    pub fn new(capacity: u32) -> Self {
        assert!(capacity > 0, "query pool capacity must be positive");
        Self {
            _capacity: capacity,
            resolved: RwLock::new(Vec::new()),
        }
    }

    /// Injects a resolved pass for harnesses and unit tests (no GPU present).
    pub fn push_resolved_pass(&self, timing: GpuPassTiming) {
        self.resolved.write().expect("poisoned lock").push(timing);
    }

    /// Clears injected timings after the collector consumes them.
    pub fn clear_resolved(&self) {
        self.resolved.write().expect("poisoned lock").clear();
    }

    /// Returns resolved GPU passes for the current frame boundary.
    #[must_use]
    pub fn read_resolved(&self) -> Vec<GpuPassTiming> {
        self.resolved.read().expect("poisoned lock").clone()
    }
}

/// Aggregates profiling data from registered threads into a [`FrameCapture`].
#[derive(Debug)]
pub struct FrameCollector {
    thread_buffers: Vec<(u32, Arc<ProfileRingBuffer>)>,
    query_pool: QueryPool,
    frame_number: u64,
    scratch: Vec<CpuEvent>,
    recent: Vec<FrameCapture>,
}

/// Complete CPU + GPU capture for one engine frame.
#[derive(Clone, Debug, PartialEq)]
pub struct FrameCapture {
    /// Monotonic frame index.
    pub frame_number: u64,
    /// Wall time for the frame in milliseconds.
    pub frame_time_ms: f64,
    /// CPU zones sorted by [`CpuEvent::begin_tsc`].
    pub cpu_events: Vec<CpuEvent>,
    /// GPU passes resolved for this frame.
    pub gpu_passes: Vec<GpuPassTiming>,
    /// Rolled-up counters for HUD / overlays.
    pub stats: FrameStats,
}

impl FrameCollector {
    /// Builds a collector for `thread_count` logical workers and the shared [`QueryPool`].
    #[must_use]
    pub fn new(_thread_count: u32, query_pool: QueryPool) -> Self {
        Self {
            thread_buffers: Vec::new(),
            query_pool,
            frame_number: 0,
            scratch: Vec::new(),
            recent: Vec::new(),
        }
    }

    /// Registers a thread-local ring buffer for draining at the frame boundary.
    pub fn register_thread(&mut self, thread_id: u32, buffer: Arc<ProfileRingBuffer>) {
        self.thread_buffers.push((thread_id, buffer));
    }

    /// Returns a handle to the GPU query pool owned by this collector.
    #[must_use]
    pub fn query_pool(&self) -> &QueryPool {
        &self.query_pool
    }

    /// Drains all ring buffers, merges GPU results, and returns a sorted [`FrameCapture`].
    pub fn collect_frame(&mut self) -> FrameCapture {
        self.scratch.clear();
        let mut arena = FrameArena::new();
        for (tid, buf) in &self.thread_buffers {
            buf.drain_into(&mut arena);
            for mut ev in arena.as_slice().iter().copied() {
                ev.thread_id = *tid;
                self.scratch.push(ev);
            }
            arena.clear();
        }
        self.scratch.sort_by_key(|e| e.begin_tsc);
        let gpu_passes = self.query_pool.read_resolved();
        self.query_pool.clear_resolved();
        let cpu_events = self.scratch.clone();
        let t_min = cpu_events.first().map(|e| e.begin_tsc).unwrap_or(0);
        let t_max = cpu_events
            .iter()
            .map(|e| e.end_tsc.max(e.begin_tsc))
            .max()
            .unwrap_or(t_min);
        let span_ticks = t_max.saturating_sub(t_min).max(1);
        let frame_time_ms = span_ticks as f64 / 1_000_000.0;
        let gpu_frame_ms: f64 = gpu_passes.iter().map(|g| g.duration_ms).sum();
        let stats = FrameStats {
            cpu_frame_ms: frame_time_ms,
            gpu_frame_ms,
            draw_calls: 0,
            triangles: 0,
            gpu_memory_bytes: 0,
            entity_count: 0,
            net_bandwidth_bps: 0.0,
        };
        self.frame_number += 1;
        let capture = FrameCapture {
            frame_number: self.frame_number,
            frame_time_ms,
            cpu_events,
            gpu_passes,
            stats,
        };
        self.recent.push(capture.clone());
        capture
    }

    /// Most recent `count` captures (newest last); capped in-memory ring of 64 entries.
    #[must_use]
    pub fn recent_frames(&self, count: u32) -> &[FrameCapture] {
        let n = usize::try_from(count).unwrap_or(64).min(64);
        let len = self.recent.len();
        if len <= n {
            return self.recent.as_slice();
        }
        &self.recent[len.saturating_sub(n)..]
    }

    /// Lookup by frame number from the recent ring.
    #[must_use]
    pub fn get_frame(&self, frame_number: u64) -> Option<&FrameCapture> {
        self.recent.iter().find(|f| f.frame_number == frame_number)
    }
}

/// Records nested CPU scopes and assigns [`CpuEvent::depth`] on close (TC-15.5.1.4).
#[derive(Debug, Default)]
pub struct CpuScopeRecorder {
    stack: Vec<u32>,
}

impl CpuScopeRecorder {
    /// Opens a nested scope identified by `zone_name_hash` at `begin_tsc`.
    pub fn begin(&mut self, zone_name_hash: u32, begin_tsc: u64, thread_id: u32) {
        let _ = (zone_name_hash, begin_tsc, thread_id);
        self.stack.push(zone_name_hash);
    }

    /// Closes the innermost scope and appends one [`CpuEvent`] with correct depth.
    pub fn end(&mut self, end_tsc: u64, thread_id: u32, begin_tsc: u64, out: &mut Vec<CpuEvent>) {
        let depth = u16::try_from(self.stack.len().saturating_sub(1)).unwrap_or(u16::MAX);
        let zone_name_hash = self.stack.pop().unwrap_or(0);
        out.push(CpuEvent {
            begin_tsc,
            end_tsc,
            thread_id,
            zone_name_hash,
            depth,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-15.5.1.3 #1 — four threads, interleaved TSC, `collect_frame` sorts ascending.
    #[test]
    fn tc_15_5_1_3_frame_collector_sort() {
        let pool = QueryPool::new(8);
        let mut col = FrameCollector::new(4, pool);
        let b0 = Arc::new(ProfileRingBuffer::new(256));
        let b1 = Arc::new(ProfileRingBuffer::new(256));
        let b2 = Arc::new(ProfileRingBuffer::new(256));
        let b3 = Arc::new(ProfileRingBuffer::new(256));
        col.register_thread(0, Arc::clone(&b0));
        col.register_thread(1, Arc::clone(&b1));
        col.register_thread(2, Arc::clone(&b2));
        col.register_thread(3, Arc::clone(&b3));
        assert!(b0.push(ev(40, 41, 0, 1, 0)));
        assert!(b2.push(ev(10, 11, 2, 3, 0)));
        assert!(b1.push(ev(30, 31, 1, 2, 0)));
        assert!(b3.push(ev(20, 21, 3, 4, 0)));
        let cap = col.collect_frame();
        let ts: Vec<u64> = cap.cpu_events.iter().map(|e| e.begin_tsc).collect();
        assert_eq!(ts, vec![10, 20, 30, 40]);
    }

    /// TC-15.5.1.4 #1 — nested scopes A { B { C } } → depths 0, 1, 2.
    #[test]
    fn tc_15_5_1_4_flame_depth_nested() {
        let mut rec = CpuScopeRecorder::default();
        let mut events = Vec::new();
        rec.begin(1, 100, 0);
        rec.begin(2, 110, 0);
        rec.begin(3, 120, 0);
        rec.end(130, 0, 120, &mut events);
        rec.end(140, 0, 110, &mut events);
        rec.end(150, 0, 100, &mut events);
        assert_eq!(events.len(), 3);
        assert_eq!(events[0].depth, 2);
        assert_eq!(events[1].depth, 1);
        assert_eq!(events[2].depth, 0);
    }

    fn ev(begin: u64, end: u64, tid: u32, zh: u32, depth: u16) -> CpuEvent {
        CpuEvent {
            begin_tsc: begin,
            end_tsc: end,
            thread_id: tid,
            zone_name_hash: zh,
            depth,
        }
    }

    /// TC-15.5.2.1 #1 — resolved GPU pass reports positive duration.
    #[test]
    fn tc_15_5_2_1_gpu_pass_timing() {
        let pool = QueryPool::new(8);
        pool.push_resolved_pass(GpuPassTiming {
            pass_id: 1,
            pass_name: "opaque",
            begin_ms: 0.0,
            end_ms: 2.0,
            duration_ms: 2.0,
        });
        let mut col = FrameCollector::new(1, pool);
        let cap = col.collect_frame();
        assert_eq!(cap.gpu_passes.len(), 1);
        assert!(cap.gpu_passes[0].duration_ms > 0.0);
    }

    /// TC-15.5.2.2 #1 — GPU pass `begin_ms` lies within CPU frame span.
    #[test]
    fn tc_15_5_2_2_gpu_cpu_alignment() {
        let pool = QueryPool::new(8);
        pool.push_resolved_pass(GpuPassTiming {
            pass_id: 2,
            pass_name: "pass",
            begin_ms: 4.0,
            end_ms: 8.0,
            duration_ms: 4.0,
        });
        let mut col = FrameCollector::new(1, pool);
        let b = Arc::new(ProfileRingBuffer::new(256));
        col.register_thread(0, Arc::clone(&b));
        assert!(b.push(ev(0, 10_000_000, 0, 1, 0)));
        let cap = col.collect_frame();
        let gpu_begin = cap.gpu_passes[0].begin_ms;
        assert!(gpu_begin >= 0.0);
        assert!(gpu_begin <= cap.frame_time_ms + f64::EPSILON);
    }
}
