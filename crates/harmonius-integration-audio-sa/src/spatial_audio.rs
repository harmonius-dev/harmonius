//! Spatial audio configuration consumed by the propagation tracer.

/// Per-source spatial audio configuration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpatialAudio {
    /// Number of occlusion rays cast toward the listener.
    pub occlusion_rays: u32,
    /// Maximum distance for proximity / reverb probes.
    pub max_distance: f32,
}

impl SpatialAudio {
    /// Builds a configuration with the given ray count.
    #[must_use]
    pub const fn new(occlusion_rays: u32, max_distance: f32) -> Self {
        Self {
            occlusion_rays,
            max_distance,
        }
    }
}
