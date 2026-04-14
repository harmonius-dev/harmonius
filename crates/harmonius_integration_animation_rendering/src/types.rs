//! Transient GPU-upload structs shared between animation evaluation and rendering.

use crate::handle::GpuBuffer;
use crate::handle::Handle;

/// Skinning algorithm selection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SkinningMode {
    /// Linear blend skinning baseline.
    Lbs,
    /// Dual quaternion skinning.
    Dqs,
}

/// Animation evaluation tier selected by level-of-detail policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnimationLodTier {
    /// All bones evaluated, full blend tree.
    Full,
    /// Reduced bone set (spine + limb roots only).
    ReducedBones,
    /// Evaluate every other frame, reuse last result.
    HalfRate,
    /// Vertex animation texture playback, no skinning.
    Vat,
}

/// Clip weights produced by the animation state machine.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BlendDescriptor {
    /// Clip indices for up to eight active clips.
    pub clip_indices: [u16; 8],
    /// Normalized clip weights.
    pub clip_weights: [f32; 8],
    /// Number of active slots in `clip_indices` / `clip_weights`.
    pub active_count: u8,
}

/// Morph target weights for facial or corrective blends.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MorphTargets {
    /// Target indices for up to sixteen morphs.
    pub target_indices: [u16; 16],
    /// Per-target weights.
    pub weights: [f32; 16],
    /// Number of active morph slots.
    pub active_count: u8,
}

/// GPU-side bone palette uploaded during Phase 6 animation evaluation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BonePaletteGpu {
    /// GPU buffer containing bone matrices or dual quaternions.
    pub buffer: Handle<GpuBuffer>,
    /// Number of bones represented in the palette.
    pub bone_count: u16,
}

/// Per-skinned mesh snapshot consumed on the render thread without ECS access.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SkinnedMeshProxy {
    /// Bone palette buffer handle snapshotted into the frame (flattened from [`BonePaletteGpu`]).
    pub bone_palette: Handle<GpuBuffer>,
    /// Bone count for shader binding.
    pub bone_count: u16,
    /// Skinning path selection for this mesh.
    pub skinning_mode: SkinningMode,
    /// Optional morph delta buffer; [`None`] skips morph accumulation.
    pub morph_buffer: Option<Handle<GpuBuffer>>,
    /// Animation evaluation tier carried into render scheduling.
    pub lod_tier: AnimationLodTier,
}

/// GPU compute dispatch descriptor for one instanced skinning batch.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SkinningDispatch {
    /// Arena backing instance transforms or skinning inputs.
    pub arena_buffer: Handle<GpuBuffer>,
    /// Number of instances in this dispatch.
    pub instance_count: u32,
    /// Bones per instance for threadgroup sizing.
    pub bone_count_per_instance: u16,
    /// Skinning algorithm for this batch.
    pub mode: SkinningMode,
}

/// Fixed-capacity GPU arena for instanced skinning.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GpuArenaBuffer {
    /// Backing GPU buffer handle.
    pub buffer: Handle<GpuBuffer>,
    /// Maximum instance slots pre-allocated for this arena.
    pub capacity: u32,
    /// Instance slots currently in use for this frame.
    pub used: u32,
}

/// Stable identifier for a skeleton asset.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SkeletonId(pub u32);

/// Stable identifier for a clip set asset.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ClipSetId(pub u32);

/// Entity identifier used only for deterministic sort order in grouping.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EntityId(pub u32);

/// One row on the instanced skinning hot path before batching.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InstancedSkinningRow {
    /// Skeleton grouping key.
    pub skeleton_id: SkeletonId,
    /// Clip set grouping key.
    pub clip_set_id: ClipSetId,
    /// Entity tie-breaker for stable ordering.
    pub entity: EntityId,
    /// Arena backing this row's instance data.
    pub arena_buffer: Handle<GpuBuffer>,
    /// Bones per instance for this row.
    pub bone_count_per_instance: u16,
    /// Skinning mode for this row.
    pub mode: SkinningMode,
}
