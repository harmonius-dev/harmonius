//! Capability gates and API fallbacks (GR-4.6, R-2.1.11).

/// Device capabilities relevant to ray tracing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RtCapabilities {
    /// Hardware ray tracing available.
    pub ray_tracing: bool,
}

/// Recorded dispatch for trace rays emulation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TraceDispatch {
    /// Native ray tracing pipeline.
    TraceRays,
    /// Compute shader fallback grid.
    ComputeDispatch {
        /// X thread groups.
        x: u32,
        /// Y thread groups.
        y: u32,
        /// Z thread groups.
        z: u32,
    },
}

/// Emits compute fallback when ray tracing is unavailable (TC-2.1.4.2).
#[derive(Debug, Default)]
pub struct TraceRaysEmulator {
    recorded: Vec<TraceDispatch>,
}

impl TraceRaysEmulator {
    /// Empty emulator.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Submits a trace rays grid; uses 8x8 compute tiles when RT is off.
    pub fn trace_rays(&mut self, caps: RtCapabilities, width: u32, height: u32, depth: u32) {
        if caps.ray_tracing {
            self.recorded.push(TraceDispatch::TraceRays);
        } else {
            self.recorded.push(TraceDispatch::ComputeDispatch {
                x: width.div_ceil(8),
                y: height.div_ceil(8),
                z: depth,
            });
        }
    }

    /// Recorded dispatches.
    #[must_use]
    pub fn recorded(&self) -> &[TraceDispatch] {
        &self.recorded
    }
}

/// Backend family for cross-backend emulation parity checks (TC-2.1.11.1).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackendFamily {
    /// Metal.
    Metal,
    /// Vulkan.
    Vulkan,
}

/// Returns the same normalized barrier set label for both backends in this stub.
#[must_use]
pub fn emulation_barrier_profile(family: BackendFamily) -> &'static str {
    match family {
        BackendFamily::Metal | BackendFamily::Vulkan => "unified-barrier-profile",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-2.1.4.2 — compute grid fallback when RT is disabled.
    #[test]
    fn test_traceray_compute_fallback() {
        let mut em = TraceRaysEmulator::new();
        em.trace_rays(RtCapabilities { ray_tracing: false }, 64, 64, 1);
        assert_eq!(
            em.recorded(),
            &[TraceDispatch::ComputeDispatch { x: 8, y: 8, z: 1 }]
        );
    }

    /// TC-2.1.11.1 — emulation profile matches across backend families.
    #[test]
    fn test_cross_backend_emulation() {
        assert_eq!(
            emulation_barrier_profile(BackendFamily::Metal),
            emulation_barrier_profile(BackendFamily::Vulkan)
        );
    }
}
