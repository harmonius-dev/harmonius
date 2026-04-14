//! Profiler integration with the Harmonius game loop frame pipeline.
//!
//! Implements CPU scope guards, per-thread ring buffers, frame collection, and spike detection
//! per `docs/design/integration/profiler-game-loop.md`.

#![deny(clippy::all)]
#![deny(missing_docs)]

mod arena;
mod collector;
mod cpu_event;
mod hash;
mod phase;
mod ring_buffer;
mod scope;
mod spike;
mod tsc;

pub use arena::FrameArena;
pub use collector::{FrameCapture, FrameCollector, FrameStats, LatestFrameCapture};
pub use cpu_event::CpuEvent;
pub use phase::Phase;
pub use ring_buffer::ProfileRingBuffer;
pub use scope::{CpuScopeGuard, ProfileBindGuard};
pub use spike::{SpikeEntry, SpikeRing, SpikeRingView};
pub use tsc::monotonic_stamp;

/// Per-phase millisecond budgets indexed by [`Phase::budget_slot`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PhaseBudgetTable {
    /// Millisecond budgets for the nine canonical slots.
    pub budgets: [f64; 9],
}

impl PhaseBudgetTable {
    /// Builds a table with every budget unset (`0.0` skips spike checks).
    pub const fn new() -> Self {
        Self { budgets: [0.0; 9] }
    }

    /// Registers a per-phase budget in milliseconds.
    pub fn set_phase_budget(&mut self, phase: Phase, budget_ms: f64) {
        self.budgets[phase.budget_slot()] = budget_ms;
    }
}

impl Default for PhaseBudgetTable {
    fn default() -> Self {
        Self::new()
    }
}
