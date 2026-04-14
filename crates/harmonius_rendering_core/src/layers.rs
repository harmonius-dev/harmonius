//! Render-layer bitmask visibility (`RF-14` in the rendering-core design).

/// `u32` bitmask for up to 32 render layers. Shared by cameras, lights, and visibility.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RenderLayerMask(pub u32);

impl RenderLayerMask {
    /// Default world layer (`bit 0`).
    pub const DEFAULT: Self = Self(1);
    /// Matches every layer.
    pub const ALL: Self = Self(u32::MAX);

    /// Returns a mask with exactly layer `n` set (`n` must be `< 32`).
    #[must_use]
    pub fn layer(n: u32) -> Self {
        debug_assert!(n < 32);
        Self(1 << n)
    }

    /// Adds layer `n` to this mask.
    #[must_use]
    pub fn with(self, n: u32) -> Self {
        debug_assert!(n < 32);
        Self(self.0 | (1 << n))
    }

    /// Clears layer `n` on this mask.
    #[must_use]
    pub fn without(self, n: u32) -> Self {
        debug_assert!(n < 32);
        Self(self.0 & !(1 << n))
    }

    /// `true` when any layer bit overlaps between `self` and `other`.
    #[must_use]
    pub fn intersects(self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::RenderLayerMask;

    /// TC-2.10.7.1 — entity layer bit 2, camera mask bit 2 → visible; mask bit 3 → invisible.
    #[test]
    fn test_render_layer_bitmask_filter() {
        let entity = RenderLayerMask::layer(2);
        let cam_match = RenderLayerMask::layer(2);
        let cam_mismatch = RenderLayerMask::layer(3);
        assert!(entity.intersects(cam_match));
        assert!(!entity.intersects(cam_mismatch));
    }
}
