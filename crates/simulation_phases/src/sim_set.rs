//! Simulation sub-phase ordering inside [`crate::Phase::Simulation`].

use crate::Phase;

/// Ordered system sets for simulation primitives.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SimSet {
    /// Timeline playback and keyframe emission.
    TimelinesAdvance,
    /// Grid and volume propagation.
    GridsPropagate,
    /// Event log decay and neighbor propagation.
    EventLogsUpdate,
    /// Shared spatial index rebuild.
    SpatialIndexRebuild,
    /// Threshold evaluation after all prior simulation work completes.
    ThresholdTriggers,
}

impl SimSet {
    /// Stable index for ordering comparisons inside [`Phase::Simulation`].
    #[must_use]
    pub const fn ordinal(self) -> u8 {
        match self {
            SimSet::TimelinesAdvance => 0,
            SimSet::GridsPropagate => 1,
            SimSet::EventLogsUpdate => 2,
            SimSet::SpatialIndexRebuild => 3,
            SimSet::ThresholdTriggers => 4,
        }
    }
}

/// Returns the canonical chained order for simulation primitive sets.
#[must_use]
pub const fn simulation_set_chain() -> [SimSet; 5] {
    [
        SimSet::TimelinesAdvance,
        SimSet::GridsPropagate,
        SimSet::EventLogsUpdate,
        SimSet::SpatialIndexRebuild,
        SimSet::ThresholdTriggers,
    ]
}

/// Phase where spatial awareness systems are registered per simulation design.
#[must_use]
pub const fn spatial_awareness_phase() -> Phase {
    Phase::AiUpdate
}
