//! Lightweight identifiers used by camera evaluation helpers.

/// Stable entity identifier for deterministic tests.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Entity(pub u32);

/// Bitmask used for collision and visibility routing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LayerMask(pub u32);

impl LayerMask {
    /// Returns true when any shared bit is set.
    pub fn overlaps(self, other: LayerMask) -> bool {
        (self.0 & other.0) != 0
    }
}
