//! Render-thread-owned profiling queries and RAII GPU scopes.

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use crate::draw_list::DrawListStats;
use crate::timebase::gpu_ticks_to_ms;

/// Runtime toggle state. Never `cfg`-gated in shipping builds.
#[derive(Debug)]
pub struct GpuProfilerState {
    /// Master switch read on the render thread hot path.
    pub enabled: AtomicBool,
    /// Desired maximum number of begin/end pairs for the backing pool.
    pub pool_capacity: AtomicU32,
}

impl GpuProfilerState {
    /// Creates a new profiler state with profiling disabled by default.
    #[must_use]
    pub fn new(pool_capacity: u32) -> Self {
        Self {
            enabled: AtomicBool::new(false),
            pool_capacity: AtomicU32::new(pool_capacity),
        }
    }
}

/// Record of GPU-side memory accounting used when assembling [`GpuFrameStats`](GpuFrameStats).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GpuAllocatorStats {
    /// Meshlets submitted this frame.
    pub meshlets_submitted: u32,
    /// Meshlets culled before draw.
    pub meshlets_culled: u32,
    /// Total bytes tracked by the allocator.
    pub total: u64,
    /// Bytes attributed to texture resources.
    pub textures: u64,
    /// Bytes attributed to buffer resources.
    pub buffers: u64,
    /// Bytes attributed to render targets.
    pub render_targets: u64,
}

/// Resolved GPU timing for one render pass after readback completes.
#[derive(Clone, Debug, PartialEq)]
pub struct GpuPassTiming {
    /// Stable pass identifier within the frame.
    pub pass_id: u32,
    /// Compile-time literal name for UI display.
    pub pass_name: &'static str,
    /// Begin time in milliseconds on the GPU timebase.
    pub begin_ms: f64,
    /// End time in milliseconds on the GPU timebase.
    pub end_ms: f64,
    /// `end_ms - begin_ms`, clamped to be non-negative for UI stability.
    pub duration_ms: f64,
}

/// Aggregated counters for a single frame.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct GpuFrameStats {
    /// Total draw calls across all views.
    pub draw_calls: u32,
    /// Total triangles submitted across all views.
    pub triangles: u32,
    /// Meshlets submitted for the frame.
    pub meshlets_submitted: u32,
    /// Meshlets culled for the frame.
    pub meshlets_culled: u32,
    /// Total VRAM footprint in bytes.
    pub gpu_memory_bytes: u64,
    /// VRAM used by textures.
    pub vram_textures: u64,
    /// VRAM used by buffers.
    pub vram_buffers: u64,
    /// VRAM used by render targets.
    pub vram_render_targets: u64,
}

/// Assembled capture for one frame published to ECS consumers.
#[derive(Clone, Debug, PartialEq)]
pub struct GpuFrameCapture {
    /// Frame index for correlation with CPU-side captures.
    pub frame_number: u64,
    /// Aggregated GPU statistics.
    pub stats: GpuFrameStats,
    /// Ordered pass timings for timeline rendering.
    pub timings: Vec<GpuPassTiming>,
    /// Per-view draw statistics.
    pub per_view: Vec<DrawListStats>,
}

/// Message posted from the render thread to workers once profiling data is resolved.
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedTimestamps {
    /// Frame index carried through the handoff.
    pub frame_number: u64,
    /// Resolved pass timings for this frame.
    pub timings: Vec<GpuPassTiming>,
    /// Aggregated statistics bundled with the timing payload.
    pub stats: GpuFrameStats,
    /// Per-view draw statistics captured on workers.
    pub per_view: Vec<DrawListStats>,
}

/// Render-thread command recording surface used for deterministic fake backends.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct CommandBuffer {
    /// Timestamp operations emitted in order.
    pub ops: Vec<TimestampOp>,
}

/// Timestamp operations recorded into a [`CommandBuffer`](CommandBuffer).
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TimestampOp {
    /// Begin timestamp for `slot`.
    Begin(u32),
    /// End timestamp for `slot`.
    End(u32),
}

/// Backend-agnostic timestamp pool (headless fake for CI).
#[derive(Debug)]
pub struct QueryPool {
    ticks: Vec<Option<u64>>,
    clock: u64,
}

impl QueryPool {
    fn new_pair_capacity(pair_capacity: u32) -> Self {
        let slots = (pair_capacity as usize).saturating_mul(2);
        Self {
            ticks: vec![None; slots],
            clock: 0,
        }
    }

    fn clear(&mut self) {
        for slot in &mut self.ticks {
            *slot = None;
        }
        self.clock = 0;
    }

    fn resize_pair_capacity(&mut self, pair_capacity: u32) {
        let slots = (pair_capacity as usize).saturating_mul(2);
        self.ticks.resize(slots, None);
    }

    fn write_begin(&mut self, slot: u32) {
        self.clock = self.clock.saturating_add(1);
        self.ticks[slot as usize] = Some(self.clock);
    }

    fn write_end(&mut self, slot: u32) {
        self.clock = self.clock.saturating_add(4);
        self.ticks[slot as usize] = Some(self.clock);
    }

    fn tick(&self, slot: u32) -> Option<u64> {
        self.ticks.get(slot as usize).copied().flatten()
    }
}

/// Render-thread-owned manager over a flat array of query slots.
#[derive(Debug)]
pub struct ProfilingQueries {
    pool: QueryPool,
    capacity: u32,
    /// One static name per allocated pair (`pair_index` maps to `pass_names[pair_index]`).
    pass_names: Vec<Option<&'static str>>,
    /// Next pair index to allocate.
    cursor: u32,
    /// Conversion factor from GPU ticks to milliseconds (`ticks / gpu_ticks_per_ms == ms`).
    gpu_ticks_per_ms: f64,
}

impl ProfilingQueries {
    /// Creates a query manager with the given maximum number of begin/end pairs.
    #[must_use]
    pub fn new(capacity: u32, gpu_ticks_per_ms: f64) -> Self {
        let mut pass_names = Vec::with_capacity(capacity as usize);
        pass_names.resize(capacity as usize, None);
        Self {
            pool: QueryPool::new_pair_capacity(capacity),
            capacity,
            pass_names,
            cursor: 0,
            gpu_ticks_per_ms,
        }
    }

    /// Returns how many begin/end pairs have been allocated since the last [`reset`](Self::reset).
    #[must_use]
    pub fn allocated_pair_count(&self) -> u32 {
        self.cursor
    }

    /// Allocates a begin/end slot pair for `pass_name`.
    pub fn allocate_pair(&mut self, pass_name: &'static str) -> Option<(u32, u32)> {
        if self.cursor >= self.capacity {
            return None;
        }
        let pair = self.cursor;
        self.pass_names[pair as usize] = Some(pass_name);
        let begin = pair.saturating_mul(2);
        let end = begin.saturating_add(1);
        self.cursor = self.cursor.saturating_add(1);
        Some((begin, end))
    }

    fn begin_scope(
        &mut self,
        cmd: &mut CommandBuffer,
        pass_name: &'static str,
    ) -> Option<(u32, u32)> {
        let (begin, end) = self.allocate_pair(pass_name)?;
        self.pool.write_begin(begin);
        cmd.ops.push(TimestampOp::Begin(begin));
        Some((begin, end))
    }

    fn end_scope(&mut self, cmd: &mut CommandBuffer, _begin: u32, end: u32) {
        self.pool.write_end(end);
        cmd.ops.push(TimestampOp::End(end));
    }

    /// Reads resolved timestamps for all allocated pairs in this frame.
    #[must_use]
    pub fn read_resolved(&mut self) -> Vec<GpuPassTiming> {
        let mut out = Vec::new();
        for pair in 0..self.cursor {
            let begin_slot = pair.saturating_mul(2);
            let end_slot = begin_slot.saturating_add(1);
            let Some(name) = self.pass_names[pair as usize] else {
                continue;
            };
            let begin_ticks = self.pool.tick(begin_slot);
            let end_ticks = self.pool.tick(end_slot);
            let (Some(begin_ticks), Some(end_ticks)) = (begin_ticks, end_ticks) else {
                continue;
            };
            let begin_ms = gpu_ticks_to_ms(begin_ticks, self.gpu_ticks_per_ms);
            let end_ms = gpu_ticks_to_ms(end_ticks, self.gpu_ticks_per_ms);
            let duration_ms = (end_ms - begin_ms).max(0.0);
            out.push(GpuPassTiming {
                pass_id: pair,
                pass_name: name,
                begin_ms,
                end_ms,
                duration_ms,
            });
        }
        out
    }

    /// Doubles the number of supported pairs (used after pool exhaustion).
    pub fn grow(&mut self) {
        let new_cap = self.capacity.saturating_mul(2).max(1);
        self.capacity = new_cap;
        self.pass_names.resize(new_cap as usize, None);
        self.pool.resize_pair_capacity(new_cap);
    }

    /// Resets allocation state at frame start.
    pub fn reset(&mut self) {
        self.cursor = 0;
        self.pool.clear();
    }
}

/// RAII guard that records begin/end timestamp queries for a render pass.
pub struct GpuScope<'a> {
    cmd: &'a mut CommandBuffer,
    pool: &'a mut ProfilingQueries,
    pass_name: &'static str,
    begin_query: u32,
    end_query: u32,
    enabled: bool,
}

impl<'a> GpuScope<'a> {
    /// Inserts a begin timestamp when profiling is enabled.
    #[must_use]
    pub fn new(
        cmd: &'a mut CommandBuffer,
        pool: &'a mut ProfilingQueries,
        state: &GpuProfilerState,
        name: &'static str,
    ) -> Self {
        if !state.enabled.load(Ordering::Relaxed) {
            return Self {
                cmd,
                pool,
                pass_name: name,
                begin_query: 0,
                end_query: 0,
                enabled: false,
            };
        }
        match pool.begin_scope(cmd, name) {
            None => Self {
                cmd,
                pool,
                pass_name: name,
                begin_query: 0,
                end_query: 0,
                enabled: false,
            },
            Some((begin, end)) => Self {
                cmd,
                pool,
                pass_name: name,
                begin_query: begin,
                end_query: end,
                enabled: true,
            },
        }
    }

    /// Returns the static pass name associated with this scope.
    #[must_use]
    pub fn pass_name(&self) -> &'static str {
        self.pass_name
    }

    /// Returns `true` when this scope recorded GPU queries.
    #[must_use]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl Drop for GpuScope<'_> {
    fn drop(&mut self) {
        if !self.enabled {
            return;
        }
        self.pool
            .end_scope(self.cmd, self.begin_query, self.end_query);
    }
}

/// Builds [`GpuFrameStats`](GpuFrameStats) from per-view stats and allocator totals.
#[must_use]
pub fn assemble_gpu_frame_stats(
    per_view: &[DrawListStats],
    vram: &GpuAllocatorStats,
) -> GpuFrameStats {
    GpuFrameStats {
        draw_calls: per_view.iter().map(|v| v.draw_calls).sum(),
        triangles: per_view.iter().map(|v| v.triangles).sum(),
        meshlets_submitted: vram.meshlets_submitted,
        meshlets_culled: vram.meshlets_culled,
        gpu_memory_bytes: vram.total,
        vram_textures: vram.textures,
        vram_buffers: vram.buffers,
        vram_render_targets: vram.render_targets,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::catch_unwind;
    use std::sync::Mutex;

    #[test]
    fn tc_ir_5_7_1_u1_gpu_scope_drop_emits_end() {
        let state = GpuProfilerState::new(8);
        state.enabled.store(true, Ordering::Relaxed);
        let mut cmd = CommandBuffer::default();
        let mut pool = ProfilingQueries::new(8, 1.0);
        {
            let _scope = GpuScope::new(&mut cmd, &mut pool, &state, "PassA");
        }
        assert_eq!(cmd.ops, vec![TimestampOp::Begin(0), TimestampOp::End(1)]);
    }

    #[test]
    fn tc_ir_5_7_1_u2_flat_slot_name_lookup() {
        let state = GpuProfilerState::new(128);
        state.enabled.store(true, Ordering::Relaxed);
        let mut cmd = CommandBuffer::default();
        let mut pool = ProfilingQueries::new(128, 1.0);
        for idx in 0u32..128 {
            let name: &'static str = match idx % 4 {
                0 => "A",
                1 => "B",
                2 => "C",
                _ => "D",
            };
            let _scope = GpuScope::new(&mut cmd, &mut pool, &state, name);
        }
        let timings = pool.read_resolved();
        assert_eq!(timings.len(), 128);
        assert_eq!(timings[63].pass_id, 63);
        assert_eq!(timings[63].pass_name, "D");
    }

    #[test]
    fn tc_ir_5_7_7_u1_enabled_false_does_not_allocate() {
        let state = GpuProfilerState::new(8);
        let mut cmd = CommandBuffer::default();
        let mut pool = ProfilingQueries::new(8, 1.0);
        let scope = GpuScope::new(&mut cmd, &mut pool, &state, "PassA");
        assert!(!scope.is_enabled());
        drop(scope);
        assert!(cmd.ops.is_empty());
        assert_eq!(pool.allocated_pair_count(), 0);
    }

    #[test]
    fn tc_ir_5_7_1_n1_pool_exhausted_returns_disabled_scope() {
        let state = GpuProfilerState::new(2);
        state.enabled.store(true, Ordering::Relaxed);
        let mut cmd = CommandBuffer::default();
        let mut pool = ProfilingQueries::new(2, 1.0);
        {
            let _a = GpuScope::new(&mut cmd, &mut pool, &state, "P0");
        }
        {
            let _b = GpuScope::new(&mut cmd, &mut pool, &state, "P1");
        }
        let c = GpuScope::new(&mut cmd, &mut pool, &state, "P2");
        assert!(!c.is_enabled());
        drop(c);
        assert_eq!(pool.allocated_pair_count(), 2);
    }

    #[test]
    fn tc_ir_5_7_1_n2_pass_panic_still_emits_end() {
        let state = GpuProfilerState::new(4);
        state.enabled.store(true, Ordering::Relaxed);
        let stash: Mutex<Option<(CommandBuffer, ProfilingQueries)>> = Mutex::new(Some((
            CommandBuffer::default(),
            ProfilingQueries::new(4, 1.0),
        )));
        let result = catch_unwind(|| {
            let mut guard = stash.lock().expect("lock poisoned");
            let (cmd, pool) = guard.as_mut().expect("stash");
            let _scope = GpuScope::new(cmd, pool, &state, "PanicPass");
            panic!("boom");
        });
        assert!(result.is_err());
        let packed = match stash.into_inner() {
            Ok(value) => value,
            Err(poisoned) => poisoned.into_inner(),
        };
        let (cmd, _) = packed.expect("some");
        assert!(
            cmd.ops.iter().any(|op| matches!(op, TimestampOp::End(_))),
            "expected end timestamp after unwind"
        );
    }

    #[test]
    fn tc_ir_5_7_1_n3_unpaired_query_skipped_on_read() {
        let state = GpuProfilerState::new(4);
        state.enabled.store(true, Ordering::Relaxed);
        let mut cmd = CommandBuffer::default();
        let mut pool = ProfilingQueries::new(4, 1.0);
        let (begin, end) = pool.allocate_pair("Orphan").expect("pair");
        pool.pool.write_begin(begin);
        cmd.ops.push(TimestampOp::Begin(begin));
        assert_eq!(pool.read_resolved().len(), 0);
        pool.pool.write_end(end);
        cmd.ops.push(TimestampOp::End(end));
        assert_eq!(pool.read_resolved().len(), 1);
    }

    #[test]
    fn tc_ir_5_7_1_n4_grow_then_retry_allocation() {
        let mut pool = ProfilingQueries::new(1, 1.0);
        assert!(pool.allocate_pair("A").is_some());
        assert!(pool.allocate_pair("B").is_none());
        pool.grow();
        assert!(pool.allocate_pair("B").is_some());
        assert_eq!(pool.capacity, 2);
    }
}
