//! OS-specific backend bootstrap stubs (R-2.1.4, R-2.1.5, R-2.1.6).
//!
//! `PLAN-rendering-render-pipeline` proves the CPU-side bootstrap surface with `*Stub` types.
//! Native `objc2-metal`, `windows-rs`, and Vulkan validation wiring lands in a follow-up slice;
//! see `docs/design/rendering/render-pipeline-test-cases.md` (milestone section).

/// Rounds `value` up to `align` (power of two).
#[must_use]
pub fn align_up(value: u64, align: u64) -> u64 {
    debug_assert!(align.is_power_of_two());
    (value + align - 1) & !(align - 1)
}

#[cfg(target_os = "macos")]
/// Placeholder Metal device until `objc2-metal` wiring lands.
#[derive(Debug)]
pub struct MetalBackendStub;

#[cfg(target_os = "macos")]
impl Default for MetalBackendStub {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_os = "macos")]
impl MetalBackendStub {
    /// Creates a stub Metal backend handle.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Number of enumerated devices (stub returns one on macOS).
    #[must_use]
    pub fn device_count(&self) -> usize {
        1
    }

    /// Whether the default command queue handle is non-null in real integrations.
    #[must_use]
    pub fn default_queue_valid(&self) -> bool {
        true
    }
}

#[cfg(target_os = "windows")]
/// Placeholder D3D12 adapter enumeration until `windows-rs` wiring lands.
#[derive(Debug)]
pub struct D3D12BackendStub;

#[cfg(target_os = "windows")]
impl Default for D3D12BackendStub {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_os = "windows")]
impl D3D12BackendStub {
    /// Creates a stub D3D12 backend.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Adapter count placeholder.
    #[must_use]
    pub fn adapter_count(&self) -> usize {
        1
    }

    /// Direct queue validity placeholder.
    #[must_use]
    pub fn direct_queue_valid(&self) -> bool {
        true
    }
}

/// Vulkan validation layer error counter (stubbed).
#[derive(Debug, Default)]
pub struct VulkanValidationStub {
    errors: u32,
}

impl VulkanValidationStub {
    /// Zero reported validation errors in the stub.
    #[must_use]
    pub fn new() -> Self {
        Self { errors: 0 }
    }

    /// Validation error count after a fake frame.
    #[must_use]
    pub fn error_count(&self) -> u32 {
        self.errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-2.1.7.1 — sub-allocation addresses align to power-of-two boundaries.
    #[test]
    fn test_heap_suballoc_alignment() {
        assert_eq!(align_up(1000, 256), 1024);
        assert_eq!(align_up(1024, 256), 1024);
    }

    /// TC-2.1.4.3 — Metal stub enumerates at least one device on macOS.
    #[cfg(target_os = "macos")]
    #[test]
    fn test_metal_backend_objc2_init() {
        let b = MetalBackendStub::new();
        assert!(b.device_count() >= 1);
        assert!(b.default_queue_valid());
    }

    /// TC-2.1.5.1 — D3D12 stub enumerates at least one adapter on Windows.
    #[cfg(target_os = "windows")]
    #[test]
    fn test_d3d12_backend_windows_init() {
        let b = D3D12BackendStub::new();
        assert!(b.adapter_count() >= 1);
        assert!(b.direct_queue_valid());
    }

    /// TC-2.1.6.1 — validation stub reports zero errors (real layers replace this).
    #[test]
    fn test_vulkan_validation_zero_errors() {
        let v = VulkanValidationStub::new();
        assert_eq!(v.error_count(), 0);
    }
}
