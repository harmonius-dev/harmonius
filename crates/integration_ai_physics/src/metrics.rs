//! Fallback-mode counters (FM-*) from the AI ↔ physics integration design.

/// Telemetry counters for documented fallback modes.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FallbackMetrics {
    /// FM-1: shared BVH not yet built (walkability returns `NoSurface`).
    pub fm1_bvh_missing: u64,
    /// FM-2: physics-private cast could not be evaluated (here: zero-length segment surrogate).
    pub fm2_physics_private_raycast_failed: u64,
    /// FM-3: nav query channel overflow (`DropOldest` policy).
    pub fm3_channel_drop_oldest: u64,
    /// FM-4: missing contact list; previous grounded state reused.
    pub fm4_reuse_grounded: u64,
    /// FM-6: avoidance neighbor truncation to the nearest 16 bodies.
    pub fm6_avoidance_truncation: u64,
}
