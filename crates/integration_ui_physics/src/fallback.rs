//! Telemetry counters for documented fallback modes (FM-*).

/// Incremented when integration code applies a documented fallback policy.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FallbackMetrics {
    /// FM-1 cursor outside valid NDC window.
    pub fm1_cursor_outside: u32,
    /// FM-2 physics BVH / pick acceleration not ready.
    pub fm2_bvh_not_built: u32,
    /// FM-3 depth readback older than the current render frame.
    pub fm3_depth_stale: u32,
    /// FM-4 CH-27 overflow drops (each dropped request increments once).
    pub fm4_ch27_drop: u32,
    /// FM-6 unknown camera id for projection.
    pub fm6_unknown_camera: u32,
}
