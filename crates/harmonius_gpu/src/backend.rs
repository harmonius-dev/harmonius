//! Backend trait surface (`GpuBackend`). Implementations use static dispatch via generics.

/// Marker trait for GPU device backends (Metal, D3D12, Vulkan). No object safety — callers use
/// `B: GpuBackend` monomorphization per [`crate::backend::GpuBackend`].
pub trait GpuBackend {
    /// Human-readable backend label for diagnostics.
    fn label(&self) -> &'static str;
}

/// Stub backend used in generic dispatch tests (TC-2.1.1.4).
#[derive(Debug, Default)]
pub struct MetalStubBackend;

impl GpuBackend for MetalStubBackend {
    fn label(&self) -> &'static str {
        "metal-stub"
    }
}

/// Dispatches through a concrete backend type parameter — no trait objects on this path.
pub fn backend_label<B: GpuBackend>(backend: &B) -> &'static str {
    backend.label()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-2.1.1.4 — static dispatch through `B: GpuBackend` without `dyn GpuBackend`.
    #[test]
    fn test_static_dispatch_no_vtable() {
        let b = MetalStubBackend;
        assert_eq!(backend_label(&b), "metal-stub");
    }
}
