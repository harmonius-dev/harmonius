/// Bitmask describing spatial layers for queries and membership.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SpatialLayerMask(pub u32);

impl SpatialLayerMask {
    /// Every layer enabled.
    pub const ALL: Self = Self(u32::MAX);
    /// No layers enabled.
    pub const NONE: Self = Self(0);
    /// Default gameplay layer bit.
    pub const PHYSICS: Self = Self(1 << 0);
    /// Rendering layer bit.
    pub const RENDERING: Self = Self(1 << 1);
    /// Navigation layer bit.
    pub const NAVIGATION: Self = Self(1 << 2);
    /// Networking layer bit.
    pub const NETWORK: Self = Self(1 << 3);
    /// AI perception layer bit.
    pub const AI_PERCEPTION: Self = Self(1 << 4);
    /// Audio layer bit.
    pub const AUDIO: Self = Self(1 << 5);
    /// Gameplay layer bit.
    pub const GAMEPLAY: Self = Self(1 << 6);
    /// Trigger layer bit.
    pub const TRIGGER: Self = Self(1 << 7);

    /// User-defined layer bit in the documented range.
    #[must_use]
    pub const fn custom(bit: u32) -> Self {
        debug_assert!(bit >= 8 && bit < 32);
        Self(1 << bit)
    }

    /// Returns true when any shared bit is set.
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }

    /// Bitwise union of two masks.
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Bitwise intersection of two masks.
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
