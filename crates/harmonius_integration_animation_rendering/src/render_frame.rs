//! [`RenderFrame`] snapshot exchanged at the animation → rendering boundary.
//!
//! The integration design places the canonical render snapshot type on the rendering path once
//! those crates land. This crate hosts a **contract-shaped** snapshot today so animation and
//! rendering tests share one struct; expect the type to move beside the triple-buffer publisher
//! without changing field names.

use crate::types::SkinnedMeshProxy;
use crate::types::SkinningDispatch;

/// Immutable per-frame snapshot handed to the render thread via the triple buffer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RenderFrame {
    /// Skinned mesh draw proxies snapshotted from animation.
    pub skinned_meshes: Vec<SkinnedMeshProxy>,
    /// Instanced skinning compute dispatches for this frame.
    pub skinning_dispatches: Vec<SkinningDispatch>,
    /// Monotonic frame index used for triple-buffer slot selection.
    pub frame_index: u64,
}

impl RenderFrame {
    /// Builds a render frame snapshot for tests and snapshot systems.
    #[must_use]
    pub fn new(
        frame_index: u64,
        skinned_meshes: Vec<SkinnedMeshProxy>,
        skinning_dispatches: Vec<SkinningDispatch>,
    ) -> Self {
        Self {
            skinned_meshes,
            skinning_dispatches,
            frame_index,
        }
    }
}
