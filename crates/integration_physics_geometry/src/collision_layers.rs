/// Collision layer bitmask used by both geometry producers and the physics runtime.
///
/// Contacts are generated when **both** directions agree:
/// `(a.mask & b.filter) != 0 && (b.mask & a.filter) != 0`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CollisionLayers {
    /// Bitmask describing which layers this object occupies.
    pub mask: u32,
    /// Bitmask describing which opposing layers may generate contacts.
    pub filter: u32,
}

impl CollisionLayers {
    /// Returns `true` when `a` and `b` should generate a contact under the layer rule.
    #[must_use]
    pub const fn contact_allowed(a: Self, b: Self) -> bool {
        (a.mask & b.filter) != 0 && (b.mask & a.filter) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_ir_3_8_6_1_layer_filter_blocks_contact() {
        let projectile = CollisionLayers {
            mask: 0x04,
            filter: !0,
        };
        let terrain = CollisionLayers {
            mask: 0x01,
            filter: 0x01,
        };
        assert!(!CollisionLayers::contact_allowed(projectile, terrain));
    }

    #[test]
    fn tc_ir_3_8_6_2_layer_filter_allows_contact() {
        let character = CollisionLayers {
            mask: 0x01,
            filter: !0,
        };
        let terrain = CollisionLayers {
            mask: 0x01,
            filter: 0x01,
        };
        assert!(CollisionLayers::contact_allowed(character, terrain));
    }
}
