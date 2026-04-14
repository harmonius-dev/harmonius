//! GOAP world-state bits derived from hostile counts (IR-2.2.3).

/// Minimal threat latch used by GOAP integration tests.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GoapThreatBits {
    /// True when hostile events meet or exceed the planner threshold.
    pub has_threat: bool,
}

impl GoapThreatBits {
    /// Updates `has_threat` from a hostile event count inside a window.
    pub fn set_from_hostile_count(hostile_in_window: u32, min_hostiles: u32) -> Self {
        Self {
            has_threat: hostile_in_window >= min_hostiles,
        }
    }
}
