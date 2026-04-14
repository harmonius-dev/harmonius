//! Render layer bitmask helpers (`R-2.7.8`).

/// Bitmask selecting which render layers participate in visibility tests.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RenderLayerMask(pub u32);

impl RenderLayerMask {
    /// Returns `true` when any layer bit overlaps between camera and entity.
    #[must_use]
    pub fn intersects(self, entity_mask: RenderLayerMask) -> bool {
        (self.0 & entity_mask.0) != 0
    }

    /// Returns `true` when the entity is visible to the camera.
    #[must_use]
    pub fn is_entity_visible(self, entity_mask: RenderLayerMask) -> bool {
        self.intersects(entity_mask)
    }
}

#[cfg(test)]
mod tests {
    use super::RenderLayerMask;

    /// TC-2.7.8.1 — disjoint masks never intersect.
    #[test]
    fn test_layer_mask_excludes_entity() {
        let camera = RenderLayerMask(1 << 2);
        let entity = RenderLayerMask(1 << 1);
        assert!(!camera.is_entity_visible(entity));
    }

    /// TC-2.7.8.2 — non-zero intersection renders.
    #[test]
    fn test_layer_mask_intersection_logic() {
        let camera = RenderLayerMask(0b1010);
        let entity = RenderLayerMask(0b0010);
        assert!(camera.is_entity_visible(entity));
    }
}
