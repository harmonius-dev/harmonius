//! Shared value types for frame captures, GPU timing, and aggregate stats.

/// One GPU pass interval resolved from timestamp queries.
#[derive(Clone, Debug, PartialEq)]
pub struct GpuPassTiming {
    /// Stable pass identifier.
    pub pass_id: u32,
    /// Pass label (static string from render graph).
    pub pass_name: &'static str,
    /// Pass begin time in milliseconds (CPU timeline aligned).
    pub begin_ms: f64,
    /// Pass end time in milliseconds.
    pub end_ms: f64,
    /// `end_ms - begin_ms`.
    pub duration_ms: f64,
}

/// Aggregate statistics for a single frame.
#[derive(Clone, Debug, PartialEq)]
pub struct FrameStats {
    /// Wall CPU frame time in milliseconds.
    pub cpu_frame_ms: f64,
    /// Wall GPU frame time in milliseconds.
    pub gpu_frame_ms: f64,
    /// Draw call count for the frame.
    pub draw_calls: u32,
    /// Triangle count submitted this frame.
    pub triangles: u32,
    /// Estimated GPU memory footprint in bytes.
    pub gpu_memory_bytes: u64,
    /// Live entity count at frame end.
    pub entity_count: u32,
    /// Network bandwidth in bits per second (rolling estimate).
    pub net_bandwidth_bps: f64,
}

impl Default for FrameStats {
    fn default() -> Self {
        Self {
            cpu_frame_ms: 0.0,
            gpu_frame_ms: 0.0,
            draw_calls: 0,
            triangles: 0,
            gpu_memory_bytes: 0,
            entity_count: 0,
            net_bandwidth_bps: 0.0,
        }
    }
}
