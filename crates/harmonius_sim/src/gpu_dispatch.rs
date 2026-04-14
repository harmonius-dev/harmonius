//! Heuristics for choosing GPU vs CPU propagation and staging uploads.

use std::time::Instant;

/// Selected execution path for large propagation workloads.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GpuPropagationPath {
    /// CPU synchronous kernel.
    Cpu,
    /// GPU compute dispatch.
    Gpu,
}

/// Decides whether GPU propagation should run based on estimated CPU cost.
pub fn propagation_dispatch_decision(cpu_cost_ms: f32, cpu_budget_ms: f32) -> GpuPropagationPath {
    if cpu_cost_ms > cpu_budget_ms {
        GpuPropagationPath::Gpu
    } else {
        GpuPropagationPath::Cpu
    }
}

/// Measured stats for a staging-buffer style copy.
#[derive(Clone, Debug)]
pub struct StagingUploadStats {
    /// Payload bytes touched.
    pub bytes: usize,
    /// Wall time in milliseconds.
    pub wall_ms: f64,
}

/// Copies `regions` worth of `cell_bytes` each to simulate a staging upload.
pub fn simulate_staging_dirty_upload(
    regions: usize,
    width: u32,
    height: u32,
    cell_bytes: usize,
) -> StagingUploadStats {
    let payload = (width as usize)
        .saturating_mul(height as usize)
        .saturating_mul(cell_bytes);
    let total = payload.saturating_mul(regions);
    let mut buf = vec![0u8; total];
    let start = Instant::now();
    for b in &mut buf {
        *b = (*b).wrapping_add(1);
    }
    StagingUploadStats {
        bytes: total,
        wall_ms: start.elapsed().as_secs_f64() * 1000.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_dirty_upload_1ms() {
        let stats = simulate_staging_dirty_upload(16, 64, 64, 1);
        let limit = if cfg!(debug_assertions) { 50.0 } else { 1.0 };
        assert!(
            stats.wall_ms < limit,
            "upload too slow: {} ms",
            stats.wall_ms
        );
    }

    #[test]
    fn test_gpu_compute_propagation() {
        assert_eq!(
            propagation_dispatch_decision(3.0, 2.0),
            GpuPropagationPath::Gpu
        );
        assert_eq!(
            propagation_dispatch_decision(1.0, 2.0),
            GpuPropagationPath::Cpu
        );
    }
}
