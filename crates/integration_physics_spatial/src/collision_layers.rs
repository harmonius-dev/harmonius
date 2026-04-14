//! Bidirectional collision layer filtering.

use std::collections::HashSet;
use std::sync::{LazyLock, Mutex};

use crate::entity::Entity;

static ZERO_LAYER_WARNINGS: LazyLock<Mutex<HashSet<u64>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

/// Bitmasks describing collision layer membership and interaction filters.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CollisionLayers {
    /// Layers this body belongs to.
    pub membership: u32,
    /// Layers this body interacts with.
    pub mask: u32,
}

impl CollisionLayers {
    /// Constructs collision layers, rejecting the all-zero configuration.
    ///
    /// # Panics
    ///
    /// Panics when both `membership` and `mask` are zero.
    #[must_use]
    pub fn new(membership: u32, mask: u32) -> Self {
        assert!(
            !(membership == 0 && mask == 0),
            "CollisionLayers::new rejected membership=0 and mask=0"
        );
        Self { membership, mask }
    }

    /// Returns `true` when the bidirectional bitmask test passes.
    #[must_use]
    pub fn interacts_with(self, other: Self) -> bool {
        (self.membership & other.mask) != 0 && (other.membership & self.mask) != 0
    }

    /// Emits at most one warning per entity when both sides are all-zero layers.
    pub(crate) fn warn_if_zero_layers(entity: Entity, a: Self, b: Self) {
        if a.membership == 0 && a.mask == 0 && b.membership == 0 && b.mask == 0 {
            let mut guard = ZERO_LAYER_WARNINGS
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            let _ = guard.insert(entity.0);
        }
    }
}

impl Default for CollisionLayers {
    fn default() -> Self {
        Self {
            membership: 1,
            mask: u32::MAX,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_ir_3_9_3_u1_layer_interacts_with_pass() {
        let a = CollisionLayers {
            membership: 0x01,
            mask: 0x02,
        };
        let b = CollisionLayers {
            membership: 0x02,
            mask: 0x01,
        };
        assert!(a.interacts_with(b));
    }

    #[test]
    fn tc_ir_3_9_3_u2_layer_interacts_with_fail() {
        let a = CollisionLayers {
            membership: 0x01,
            mask: 0x01,
        };
        let b = CollisionLayers {
            membership: 0x02,
            mask: 0x02,
        };
        assert!(!a.interacts_with(b));
    }

    #[test]
    fn tc_ir_3_9_3_u3_layer_zero_on_both_sides() {
        let a = CollisionLayers {
            membership: 0,
            mask: 0,
        };
        let b = CollisionLayers {
            membership: 0,
            mask: 0,
        };
        assert!(!a.interacts_with(b));
        CollisionLayers::warn_if_zero_layers(Entity::from_raw(1), a, b);
        CollisionLayers::warn_if_zero_layers(Entity::from_raw(1), a, b);
    }

    #[test]
    #[should_panic(expected = "CollisionLayers::new rejected")]
    fn tc_ir_3_9_3_n2_construction_assert_zero() {
        let _ = CollisionLayers::new(0, 0);
    }
}
