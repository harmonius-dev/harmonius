//! Editor and diagnostics overlays for NavMesh visualization (TC-7.1.15).

/// Records debug primitives for later rendering.
#[derive(Clone, Debug, Default)]
pub struct NavMeshDebugRecorder {
    /// Number of polygon outline segments submitted.
    pub polygon_edges: u32,
    /// Tile boundary segments.
    pub tile_boundaries: u32,
    /// Off-mesh link segments.
    pub link_segments: u32,
    /// Pending rebuild markers.
    pub pending_zones: u32,
}

impl NavMeshDebugRecorder {
    /// Adds polygon edges for `polygon_count` convex polygons (approximate edge count).
    pub fn record_polygons(&mut self, polygon_count: u32) {
        self.polygon_edges = self
            .polygon_edges
            .saturating_add(polygon_count.saturating_mul(4));
    }

    /// Marks one tile boundary outline.
    pub fn record_tile_boundary(&mut self) {
        self.tile_boundaries = self.tile_boundaries.saturating_add(1);
    }

    /// Marks one off-mesh link visualization.
    pub fn record_link(&mut self) {
        self.link_segments = self.link_segments.saturating_add(1);
    }

    /// Marks a pending rebuild overlay zone.
    pub fn record_pending_zone(&mut self) {
        self.pending_zones = self.pending_zones.saturating_add(1);
    }
}

/// Grouping for editor wiring (engine attaches render backends later).
#[derive(Clone, Debug, Default)]
pub struct NavMeshDebugBundle {
    /// Mutable overlay recorder.
    pub recorder: NavMeshDebugRecorder,
}

/// Shipping builds use a zero-cost stub (TC-7.1.15.2 — no retained overlay state).
#[cfg(not(debug_assertions))]
#[inline]
#[allow(dead_code)] // Public hook for editor/shipping wiring; exercised in unit tests.
pub fn shipping_overlay_enabled() -> bool {
    false
}

/// Debug/editor builds may enable overlays.
#[cfg(debug_assertions)]
#[inline]
#[allow(dead_code)] // Public hook for editor/shipping wiring; exercised in unit tests.
pub fn shipping_overlay_enabled() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-7.1.15.1 — overlay recorder accepts multi-category geometry.
    #[test]
    fn tc_7_1_15_1_overlay_categories() {
        let mut r = NavMeshDebugRecorder::default();
        r.record_polygons(10);
        r.record_tile_boundary();
        r.record_link();
        r.record_pending_zone();
        assert!(r.polygon_edges > 0);
        assert!(r.tile_boundaries > 0);
        assert!(r.link_segments > 0);
        assert!(r.pending_zones > 0);
    }

    /// TC-7.1.15.2 — shipping hook reports disabled overlays outside debug builds.
    #[test]
    fn tc_7_1_15_2_shipping_toggle() {
        if cfg!(debug_assertions) {
            assert!(shipping_overlay_enabled());
        } else {
            assert!(!shipping_overlay_enabled());
        }
    }
}
