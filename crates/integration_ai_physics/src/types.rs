//! Shared identifiers and masks referenced by AI ↔ physics queries.

/// Stable entity identifier used across AI and physics contracts.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Entity(pub u64);

impl Entity {
    /// Sentinel entity used when no geometry participates in a result.
    pub const NONE: Self = Self(0);
}

/// Authoring-time material token resolved by physics for walkability.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct MaterialId(pub u32);

/// Bitmask of materials an agent may traverse (`1 << (material_id % 64)` when material_id < 64).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AgentMask(pub u64);

impl AgentMask {
    /// Returns `true` when `material_id` is permitted by this mask.
    #[must_use]
    pub const fn allows(self, material_id: MaterialId) -> bool {
        let bit = 1u64 << (material_id.0 % 64);
        (self.0 & bit) != 0
    }

    /// Marks `material_id` as permitted.
    pub const fn with_material(mut self, material_id: MaterialId) -> Self {
        self.0 |= 1u64 << (material_id.0 % 64);
        self
    }
}
