//! Lightweight entity identifier for broadphase bookkeeping.

/// Stable entity identifier used in broadphase pairs.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Entity(pub u64);

impl Entity {
    /// Creates an entity from a raw id (tests and harnesses).
    #[must_use]
    pub const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }
}
