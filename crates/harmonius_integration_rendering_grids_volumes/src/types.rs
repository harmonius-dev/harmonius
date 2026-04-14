//! Shared enums and ECS-facing grid metadata.

use glam::{Quat, Vec3};

/// GPU sample format for grid/volume textures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum GridFormat {
    /// Single-channel normalized unsigned.
    R8Unorm = 0,
    /// Half-float scalar.
    R16Float = 1,
    /// Four-channel normalized unsigned.
    Rgba8Unorm = 2,
    /// Full-float scalar.
    R32Float = 3,
}

/// Logical grid dimensionality.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum GridDimension {
    /// Heightfield or 2D tactical map.
    Grid2d = 0,
    /// Fog, GI, or SDF volume.
    Grid3d = 1,
}

/// Upload priority for the worker→render queue.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum GridUploadPriority {
    /// Must arrive this frame.
    Critical = 0,
    /// Standard scheduling.
    Normal = 1,
    /// May be dropped under pressure.
    Background = 2,
}

/// Status of an upload command after enqueue.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum GridUploadStatus {
    /// Accepted into the outbound channel or pending queue.
    Queued = 0,
    /// GPU transfer in flight (reserved for future telemetry).
    Uploading = 1,
    /// Completed on the GPU (reserved).
    Ready = 2,
    /// Channel full or disconnected.
    Failed = 3,
}

/// SDF NaN / range handling before GPU upload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum SdfClampMode {
    /// Clamp invalid samples to `max_distance`.
    ClampToMaxDistance = 0,
    /// Fail the upload when any NaN is present.
    RejectNaN = 1,
}

/// Worker-side handle into the grid asset arena (`Copy`, no `Arc`).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GridAssetHandle {
    /// Slot index.
    pub index: u32,
    /// Generation after slot reuse.
    pub generation: u32,
}

/// ECS component: world placement for a grid or volume entity.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GridTransform {
    /// World-space origin of grid cell (0,0,0).
    pub world_origin: Vec3,
    /// Edge length of one voxel or cell in world units.
    pub voxel_size: f32,
    /// Orientation of the grid volume.
    pub rotation: Quat,
}

#[cfg(test)]
mod tests {
    use super::{GridTransform, Quat, Vec3};
    use crate::pass::FogGpuTexture;
    use crate::types::GridFormat;
    use crate::GpuTextureHandle;

    /// TC-IR-3.3.1.4 — spatial metadata lives on ECS, not the GPU struct.
    #[test]
    fn fog_gpu_texture_has_no_world_origin_component() {
        let fog = FogGpuTexture {
            handle: GpuTextureHandle {
                index: 0,
                generation: 0,
            },
            width: 4,
            height: 4,
            format: GridFormat::R8Unorm,
        };
        let xf = GridTransform {
            world_origin: Vec3::new(10.0, 0.0, -3.0),
            voxel_size: 2.0,
            rotation: Quat::IDENTITY,
        };
        assert_eq!(xf.world_origin.x, 10.0);
        assert_eq!(fog.width, 4);
    }
}
