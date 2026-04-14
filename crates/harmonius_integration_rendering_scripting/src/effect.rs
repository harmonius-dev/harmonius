//! VFX `CompiledEffect` / `CompiledKernel` payloads (IR-3.5.6).

use rkyv_derive::{Archive, Deserialize, Serialize};

use crate::shader_types::ShaderProfile;

/// Packed vertex attribute layout for GPU dispatch (IR-3.5.6).
#[derive(Archive, Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[rkyv(bytecheck(), derive(Copy, Clone, Debug, Eq, PartialEq))]
pub struct AttributeLayout(pub u32);

/// Root signature / parameter buffer packing for one kernel (IR-3.5.6).
#[derive(Archive, Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[rkyv(bytecheck(), derive(Copy, Clone, Debug, Eq, PartialEq))]
pub struct ParamBufferLayout(pub u32);

/// Billboard, mesh, ribbon, or decal output (IR-3.5.6).
#[derive(
    Archive, Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
#[rkyv(
    bytecheck(),
    derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)
)]
#[repr(u8)]
pub enum OutputMode {
    /// Screen-facing quads.
    Billboard = 0,
    /// Meshed particles.
    Mesh = 1,
    /// Trail ribbons.
    Ribbon = 2,
    /// Projected decals.
    Decal = 3,
}

/// One compute kernel inside a [`CompiledEffect`].
#[derive(Archive, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[rkyv(bytecheck())]
#[repr(C)]
pub struct CompiledKernel {
    /// Placeholder bytecode blob (real engine stores DXIL/SPIR-V slices).
    pub bytecode: [u8; 64],
    /// `numthreads` product for dispatch sizing.
    pub thread_group_size: u32,
    /// Root parameter layout for this kernel.
    pub param_layout: ParamBufferLayout,
}

/// Full compiled effect with four lifecycle kernels (IR-3.5.6).
#[derive(Archive, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[rkyv(bytecheck())]
#[repr(C, align(16))]
pub struct CompiledEffect {
    /// Fingerprint of authored graph source.
    pub source_hash: u64,
    /// Spawn kernel.
    pub spawn_kernel: CompiledKernel,
    /// Init kernel.
    pub init_kernel: CompiledKernel,
    /// Simulation update kernel.
    pub update_kernel: CompiledKernel,
    /// Output / rasterization kernel.
    pub output_kernel: CompiledKernel,
    /// Attribute packing for instances.
    pub attribute_layout: AttributeLayout,
    /// Output primitive mode.
    pub output_mode: OutputMode,
}

impl CompiledEffect {
    /// Builds a deterministic fixture for rkyv alignment tests (IR-3.5.U2).
    #[must_use]
    pub fn test_fixture() -> Self {
        let mk = |tag: u8| CompiledKernel {
            bytecode: {
                let mut b = [0u8; 64];
                b[0] = tag;
                b
            },
            thread_group_size: 64,
            param_layout: ParamBufferLayout(1),
        };
        Self {
            source_hash: 0xDEAD_BEEF_0000_0001,
            spawn_kernel: mk(1),
            init_kernel: mk(2),
            update_kernel: mk(3),
            output_kernel: mk(4),
            attribute_layout: AttributeLayout(8),
            output_mode: OutputMode::Billboard,
        }
    }
}

/// Documents the intended `dxc` profile for effect kernels (compile-time helper).
#[must_use]
#[cfg_attr(not(test), allow(dead_code))]
pub const fn effect_kernel_profile() -> ShaderProfile {
    ShaderProfile::Compute6_6
}

#[cfg(test)]
mod profile_tests {
    use super::*;

    /// Smoke: effect kernels target CS 6.6 per integration design (IR-3.5.6).
    #[test]
    fn effect_kernel_profile_is_compute() {
        assert!(matches!(effect_kernel_profile(), ShaderProfile::Compute6_6));
    }
}
