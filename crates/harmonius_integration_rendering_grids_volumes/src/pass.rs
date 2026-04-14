//! Render-graph pass registration stubs for fog, voxel GI, and SDF shadows.

use glam::UVec3;

use crate::types::GridFormat;
use crate::GpuTextureHandle;

/// Placeholder command buffer type for pass recording.
#[derive(Debug, Default)]
pub struct RenderCommandBuffer;

/// Placeholder render-graph builder.
#[derive(Debug, Default)]
pub struct RenderGraphBuilder;

/// GPU-side fog-of-war texture descriptor (no world metadata).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FogGpuTexture {
    /// Device texture slot.
    pub handle: GpuTextureHandle,
    /// Width in texels.
    pub width: u32,
    /// Height in texels.
    pub height: u32,
    /// Sample layout on the GPU.
    pub format: GridFormat,
}

/// GPU-side 3D volume descriptor (no world metadata).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VolumeGpuTexture {
    /// Device texture slot.
    pub handle: GpuTextureHandle,
    /// Extent in voxels per axis.
    pub dimensions: UVec3,
    /// Sample layout on the GPU.
    pub format: GridFormat,
}

/// Declares read/write texture handles for a render-graph node.
#[derive(Debug)]
pub struct GridPassRegistration {
    /// Debug pass name.
    pub name: &'static str,
    /// Inputs sampled by the pass.
    pub read_resources: Vec<GpuTextureHandle>,
    /// Outputs written by the pass.
    pub write_resources: Vec<GpuTextureHandle>,
}

impl GridPassRegistration {
    /// Records GPU commands (stub: no-op for CPU-only tests).
    pub fn record(&self, _cmd_buf: &mut RenderCommandBuffer) {}
}

/// Registers the fog overlay pass reading the uploaded fog texture.
#[must_use]
pub fn register_fog_overlay_pass(
    _graph: &mut RenderGraphBuilder,
    fog: FogGpuTexture,
    lit_color: GpuTextureHandle,
) -> GridPassRegistration {
    GridPassRegistration {
        name: "fog_overlay",
        read_resources: vec![fog.handle],
        write_resources: vec![lit_color],
    }
}

/// Registers the voxel GI compute pass.
#[must_use]
pub fn register_voxel_gi_pass(
    _graph: &mut RenderGraphBuilder,
    gi: VolumeGpuTexture,
    indirect_out: GpuTextureHandle,
) -> GridPassRegistration {
    GridPassRegistration {
        name: "voxel_gi",
        read_resources: vec![gi.handle],
        write_resources: vec![indirect_out],
    }
}

/// Registers the SDF shadow ray-march pass.
#[must_use]
pub fn register_sdf_shadow_pass(
    _graph: &mut RenderGraphBuilder,
    sdf: VolumeGpuTexture,
    shadow_mask: GpuTextureHandle,
) -> GridPassRegistration {
    GridPassRegistration {
        name: "sdf_shadow",
        read_resources: vec![sdf.handle],
        write_resources: vec![shadow_mask],
    }
}

#[cfg(test)]
mod tests {
    use super::{
        register_fog_overlay_pass, register_sdf_shadow_pass, register_voxel_gi_pass, FogGpuTexture,
        RenderGraphBuilder, VolumeGpuTexture,
    };
    use crate::types::GridFormat;
    use crate::GpuTextureHandle;

    #[test]
    fn fog_pass_wires_handles() {
        let mut g = RenderGraphBuilder::default();
        let fog = FogGpuTexture {
            handle: GpuTextureHandle {
                index: 1,
                generation: 0,
            },
            width: 256,
            height: 256,
            format: GridFormat::R8Unorm,
        };
        let lit = GpuTextureHandle {
            index: 2,
            generation: 0,
        };
        let reg = register_fog_overlay_pass(&mut g, fog, lit);
        assert_eq!(reg.name, "fog_overlay");
        assert_eq!(reg.read_resources, vec![fog.handle]);
        assert_eq!(reg.write_resources, vec![lit]);
    }

    #[test]
    fn voxel_gi_pass_wires_handles() {
        let mut g = RenderGraphBuilder::default();
        let gi = VolumeGpuTexture {
            handle: GpuTextureHandle {
                index: 3,
                generation: 1,
            },
            dimensions: glam::UVec3::splat(64),
            format: GridFormat::R16Float,
        };
        let out = GpuTextureHandle {
            index: 4,
            generation: 0,
        };
        let reg = register_voxel_gi_pass(&mut g, gi, out);
        assert_eq!(reg.name, "voxel_gi");
        assert_eq!(reg.read_resources, vec![gi.handle]);
        assert_eq!(reg.write_resources, vec![out]);
    }

    #[test]
    fn sdf_shadow_pass_wires_handles() {
        let mut g = RenderGraphBuilder::default();
        let sdf = VolumeGpuTexture {
            handle: GpuTextureHandle {
                index: 5,
                generation: 0,
            },
            dimensions: glam::UVec3::splat(128),
            format: GridFormat::R32Float,
        };
        let mask = GpuTextureHandle {
            index: 6,
            generation: 0,
        };
        let reg = register_sdf_shadow_pass(&mut g, sdf, mask);
        assert_eq!(reg.name, "sdf_shadow");
        assert_eq!(reg.read_resources, vec![sdf.handle]);
        assert_eq!(reg.write_resources, vec![mask]);
    }
}
