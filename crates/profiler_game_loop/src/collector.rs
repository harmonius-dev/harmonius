use std::collections::HashMap;

use crate::arena::FrameArena;
use crate::cpu_event::CpuEvent;
use crate::hash::fnv1a;
use crate::phase::Phase;
use crate::ring_buffer::ProfileRingBuffer;
use crate::scope::set_local_thread_id;
use crate::spike::{SpikeEntry, SpikeRing};
use crate::PhaseBudgetTable;

/// Aggregated frame statistics derived from CPU events.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FrameStats {
    /// Monotonic frame identifier.
    pub frame_number: u64,
    /// Wall-clock CPU frame time in milliseconds.
    pub cpu_frame_ms: f64,
    /// GPU frame time placeholder (not measured here).
    pub gpu_frame_ms: f64,
    /// Draw calls placeholder.
    pub draw_calls: u32,
    /// Triangles placeholder.
    pub triangles: u32,
    /// GPU memory placeholder.
    pub gpu_memory_bytes: u64,
    /// Entity count placeholder for overlays.
    pub entity_count: u32,
    /// Networking placeholder.
    pub net_bandwidth_bps: f64,
    /// Set when the arena could not hold the full event stream.
    pub profiler_truncated: bool,
}

impl FrameStats {
    fn empty(frame_number: u64) -> Self {
        Self {
            frame_number,
            cpu_frame_ms: 0.0,
            gpu_frame_ms: 0.0,
            draw_calls: 0,
            triangles: 0,
            gpu_memory_bytes: 0,
            entity_count: 0,
            net_bandwidth_bps: 0.0,
            profiler_truncated: false,
        }
    }

    fn from_events(frame_number: u64, events: &[CpuEvent], truncated: bool) -> Self {
        let mut cpu_frame_ms = 0.0;
        if let (Some(first), Some(last)) = (events.first(), events.last()) {
            cpu_frame_ms = (last.end_tsc.saturating_sub(first.begin_tsc)) as f64 / 1_000_000.0;
        }
        Self {
            frame_number,
            cpu_frame_ms,
            gpu_frame_ms: 0.0,
            draw_calls: 0,
            triangles: 0,
            gpu_memory_bytes: 0,
            entity_count: 0,
            net_bandwidth_bps: 0.0,
            profiler_truncated: truncated,
        }
    }
}

/// Immutable snapshot of one frame worth of profiling data.
#[derive(Clone, Debug, PartialEq)]
pub struct FrameCapture {
    /// Frame identifier.
    pub frame_number: u64,
    /// Sorted CPU events for the frame.
    pub cpu_events: Vec<CpuEvent>,
    /// Aggregated statistics.
    pub stats: FrameStats,
    /// Spike records collected for this frame.
    pub spikes: Vec<SpikeEntry>,
}

impl FrameCapture {
    fn empty(frame_number: u64) -> Self {
        Self {
            frame_number,
            cpu_events: Vec::new(),
            stats: FrameStats::empty(frame_number),
            spikes: Vec::new(),
        }
    }
}

/// ECS-facing resource published at the end of each frame.
#[derive(Clone, Debug, PartialEq)]
pub struct LatestFrameCapture {
    /// Most recently collected frame.
    pub capture: FrameCapture,
}

/// Collects per-thread ring buffers into a [`FrameCapture`].
#[derive(Debug)]
pub struct FrameCollector {
    /// Monotonic frame counter incremented on each collection.
    pub frame_number: u64,
    /// Runtime toggle for collection hot path.
    pub enabled: bool,
    /// Per-phase millisecond budgets.
    pub phase_budgets: PhaseBudgetTable,
    spike_ring: SpikeRing,
    arena: FrameArena,
    buffers: Vec<ProfileRingBuffer>,
    thread_index: HashMap<u64, usize>,
    thread_depths: Vec<u16>,
    latest: Option<LatestFrameCapture>,
}

impl FrameCollector {
    /// Builds a collector with a 1 MiB arena and disabled collection until enabled.
    #[must_use]
    pub fn new() -> Self {
        Self {
            frame_number: 0,
            enabled: true,
            phase_budgets: PhaseBudgetTable::new(),
            spike_ring: SpikeRing::new(),
            arena: FrameArena::with_capacity(1024 * 1024),
            buffers: Vec::new(),
            thread_index: HashMap::new(),
            thread_depths: Vec::new(),
            latest: None,
        }
    }

    /// Registers the calling thread's ring buffer slot.
    pub fn register_thread(&mut self, thread_id: u64) -> usize {
        if let Some(idx) = self.thread_index.get(&thread_id).copied() {
            set_local_thread_id(thread_id);
            return idx;
        }
        let idx = self.buffers.len();
        self.buffers.push(ProfileRingBuffer::with_capacity(16_384));
        self.thread_depths.push(0);
        self.thread_index.insert(thread_id, idx);
        set_local_thread_id(thread_id);
        idx
    }

    /// Sets a per-phase budget in milliseconds.
    pub fn set_phase_budget(&mut self, phase: Phase, budget_ms: f64) {
        self.phase_budgets.set_phase_budget(phase, budget_ms);
    }

    /// Enables or disables CPU event recording and frame aggregation.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Begins a CPU scope for the active thread (used by [`crate::CpuScopeGuard`]).
    pub fn begin_scope(&mut self, zone: u32, begin: u64, thread_id: u64) {
        if !self.enabled {
            return;
        }
        let Some(&idx) = self.thread_index.get(&thread_id) else {
            return;
        };
        let depth = self.thread_depths[idx];
        let event = CpuEvent::pending(begin, thread_id, zone, depth);
        self.buffers[idx].push_pending(event);
        self.thread_depths[idx] = depth.saturating_add(1);
    }

    /// Ends the innermost CPU scope for the active thread.
    pub fn end_scope(&mut self, zone: u32, end: u64, thread_id: u64) {
        if !self.enabled {
            return;
        }
        let Some(&idx) = self.thread_index.get(&thread_id) else {
            return;
        };
        let _ = zone;
        let _ = self.buffers[idx].complete_pending(end);
        self.thread_depths[idx] = self.thread_depths[idx].saturating_sub(1);
    }

    /// Drains buffers, sorts events, runs spike detection, and publishes [`LatestFrameCapture`].
    pub fn collect_frame(&mut self) -> FrameCapture {
        self.frame_number = self.frame_number.saturating_add(1);
        if !self.enabled {
            let capture = FrameCapture::empty(self.frame_number);
            self.publish_latest(&capture);
            return capture;
        }

        self.arena.reset();
        let total: usize = self.buffers.iter().map(ProfileRingBuffer::len).sum();

        let cpu_events = if total == 0 {
            Vec::new()
        } else {
            let Some(merged) = self.arena.alloc_slice::<CpuEvent>(total) else {
                let capture = FrameCapture {
                    frame_number: self.frame_number,
                    cpu_events: Vec::new(),
                    stats: FrameStats {
                        frame_number: self.frame_number,
                        cpu_frame_ms: 0.0,
                        gpu_frame_ms: 0.0,
                        draw_calls: 0,
                        triangles: 0,
                        gpu_memory_bytes: 0,
                        entity_count: 0,
                        net_bandwidth_bps: 0.0,
                        profiler_truncated: true,
                    },
                    spikes: Vec::new(),
                };
                self.publish_latest(&capture);
                return capture;
            };

            let mut offset = 0usize;
            for buffer in &mut self.buffers {
                let n = buffer.len();
                if n == 0 {
                    continue;
                }
                let written = buffer.copy_completed_to(&mut merged[offset..]);
                buffer.clear_completed();
                offset += written;
            }

            merged.sort_unstable_by_key(|event| event.begin_tsc);
            merged.to_vec()
        };

        let stats = FrameStats::from_events(self.frame_number, &cpu_events, false);
        detect_spikes(
            &mut self.spike_ring,
            &self.phase_budgets,
            &cpu_events,
            self.frame_number,
        );
        let spike_view = self.spike_ring.drain();
        let spikes = spike_view.entries.to_vec();
        let capture = FrameCapture {
            frame_number: self.frame_number,
            cpu_events,
            stats,
            spikes,
        };
        self.publish_latest(&capture);
        capture
    }

    /// Returns the most recently published capture, if any.
    #[must_use]
    pub fn latest_capture(&self) -> Option<&LatestFrameCapture> {
        self.latest.as_ref()
    }

    fn publish_latest(&mut self, capture: &FrameCapture) {
        self.latest = Some(LatestFrameCapture {
            capture: capture.clone(),
        });
    }
}

fn detect_spikes(
    spike_ring: &mut SpikeRing,
    budgets: &PhaseBudgetTable,
    events: &[CpuEvent],
    frame_number: u64,
) {
    for phase in [
        Phase::Input,
        Phase::NetworkReceive,
        Phase::Simulation,
        Phase::AiUpdate,
        Phase::PhysicsStep,
        Phase::AnimationUpdate,
        Phase::FrameSnapshot,
        Phase::FrameEnd,
    ] {
        let budget_ms = budgets.budgets[phase.budget_slot()];
        if budget_ms <= 0.0 {
            continue;
        }
        let zone = zone_hash_for_phase(phase);
        let mut longest_ms = 0.0f64;
        for event in events {
            if event.zone_name_hash != zone {
                continue;
            }
            let dur_ns = event.end_tsc.saturating_sub(event.begin_tsc);
            let dur_ms = dur_ns as f64 / 1_000_000.0;
            if dur_ms > longest_ms {
                longest_ms = dur_ms;
            }
        }
        if longest_ms > budget_ms {
            spike_ring.push(SpikeEntry {
                phase,
                duration_ms: longest_ms,
                budget_ms,
                frame_number,
            });
        }
    }
}

impl Default for FrameCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Stable zone hash for a built-in [`Phase`] scope label.
#[must_use]
pub fn zone_hash_for_phase(phase: Phase) -> u32 {
    match phase {
        Phase::Input => fnv1a("Phase1_Input"),
        Phase::NetworkReceive => fnv1a("Phase2_NetworkReceive"),
        Phase::Simulation => fnv1a("Phase3_Simulation"),
        Phase::AiUpdate => fnv1a("Phase4_AiUpdate"),
        Phase::PhysicsStep => fnv1a("Phase5_PhysicsStep"),
        Phase::AnimationUpdate => fnv1a("Phase6_AnimationUpdate"),
        Phase::FrameSnapshot => fnv1a("Phase7_FrameSnapshot"),
        Phase::FrameEnd => fnv1a("Phase8_FrameEnd"),
        Phase::Custom(id) => fnv1a("Phase_Custom").wrapping_add(id),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scope::{CpuScopeGuard, ProfileBindGuard};

    #[test]
    fn tc_ir_5_6_7_u1_latest_frame_capture_replaced() {
        let mut collector = FrameCollector::new();
        let tid = 1u64;
        collector.register_thread(tid);
        {
            let _bind = ProfileBindGuard::enter(&mut collector);
            let _scope = CpuScopeGuard::new("Phase1_Input");
        }
        collector.collect_frame();
        let first = collector
            .latest_capture()
            .expect("published")
            .capture
            .frame_number;
        {
            let _bind = ProfileBindGuard::enter(&mut collector);
            let _scope = CpuScopeGuard::new("Phase1_Input");
        }
        std::thread::sleep(std::time::Duration::from_micros(10));
        collector.collect_frame();
        let second = collector
            .latest_capture()
            .expect("published")
            .capture
            .frame_number;
        assert_ne!(first, second);
    }

    #[test]
    fn tc_ir_5_6_3_1_spike_on_slow_phase() {
        let mut collector = FrameCollector::new();
        let tid = 2u64;
        collector.register_thread(tid);
        collector.set_phase_budget(Phase::PhysicsStep, 4.0);
        {
            let _bind = ProfileBindGuard::enter(&mut collector);
            let _scope = CpuScopeGuard::new("Phase5_PhysicsStep");
            std::thread::sleep(std::time::Duration::from_millis(12));
        }
        let capture = collector.collect_frame();
        assert!(
            capture.spikes.iter().any(|s| {
                s.phase == Phase::PhysicsStep && s.budget_ms == 4.0 && s.duration_ms > 4.0
            }),
            "expected physics spike, got {:?}",
            capture.spikes
        );
    }
}
