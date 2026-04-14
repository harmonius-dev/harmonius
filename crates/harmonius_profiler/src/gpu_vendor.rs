//! GPU vendor counter stubs (design: `VendorCounters`).

/// Vendor counter data for a specific pass.
#[derive(Clone, Debug, PartialEq)]
pub struct VendorCounterData {
    /// Shader occupancy in `[0, 1]` when available.
    pub shader_occupancy: Option<f32>,
    /// Wave utilization when available.
    pub wave_utilization: Option<f32>,
    /// ALU utilization when available.
    pub alu_utilization: Option<f32>,
}

/// Vendor-specific GPU counter access (compile-time vendor selection for tests).
#[derive(Clone, Debug)]
pub struct VendorCounters {
    is_amd: bool,
}

impl VendorCounters {
    /// Builds counters for an AMD-class GPU (harness / unit tests).
    #[must_use]
    pub fn new_amd() -> Self {
        Self { is_amd: true }
    }

    /// Builds counters for a non-AMD GPU.
    #[must_use]
    pub fn new_non_amd() -> Self {
        Self { is_amd: false }
    }

    /// Shader occupancy for the current frame when the driver exposes it.
    #[must_use]
    pub fn shader_occupancy(&self) -> Option<f32> {
        if self.is_amd {
            Some(0.5)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-15.5.2.3 #1 — AMD occupancy in range.
    #[test]
    fn tc_15_5_2_3_shader_occupancy_amd() {
        let v = VendorCounters::new_amd();
        let occ = v.shader_occupancy().expect("amd occupancy");
        assert!((0.0..=1.0).contains(&occ));
    }

    /// TC-15.5.2.3 #2 — non-AMD returns `None`.
    #[test]
    fn tc_15_5_2_3_shader_occupancy_non_amd() {
        let v = VendorCounters::new_non_amd();
        assert!(v.shader_occupancy().is_none());
    }
}
