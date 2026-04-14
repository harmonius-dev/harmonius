//! [`AnimationRenderBridge`] helpers bridging animation staging to [`RenderFrame`].
//!
//! [`AnimationRenderBridge::snapshot_render_frame`] clones proxy vectors for tests and cold
//! triple-buffer handoff; hot paths should move buffers once a move-based API exists upstream.

use crate::handle::GpuBuffer;
use crate::handle::Handle;
use crate::render_frame::RenderFrame;
use crate::resource::GpuBufferTable;
use crate::types::InstancedSkinningRow;
use crate::types::SkinnedMeshProxy;
use crate::types::SkinningDispatch;

/// Animation-side bridge that packages proxies and dispatches for Phase 7 snapshot.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AnimationRenderBridge {
    skinned_meshes: Vec<SkinnedMeshProxy>,
    skinning_rows: Vec<InstancedSkinningRow>,
}

impl AnimationRenderBridge {
    /// Creates a bridge from animation staging vectors.
    #[must_use]
    pub fn new(
        skinned_meshes: Vec<SkinnedMeshProxy>,
        skinning_rows: Vec<InstancedSkinningRow>,
    ) -> Self {
        Self {
            skinned_meshes,
            skinning_rows,
        }
    }

    /// Returns a cloned proxy list for snapshotting into [`RenderFrame`].
    #[must_use]
    pub fn snapshot_proxies(&self) -> Vec<SkinnedMeshProxy> {
        self.skinned_meshes.clone()
    }

    /// Builds instanced skinning dispatches using sort-only grouping.
    #[must_use]
    pub fn build_dispatches(&self) -> Vec<SkinningDispatch> {
        let mut rows = self.skinning_rows.clone();
        crate::grouping::build_skinning_dispatches_sorted(&mut rows)
    }

    /// Materializes a full [`RenderFrame`] for triple-buffer publish in tests.
    #[must_use]
    pub fn snapshot_render_frame(&self, frame_index: u64) -> RenderFrame {
        RenderFrame::new(
            frame_index,
            self.snapshot_proxies(),
            self.build_dispatches(),
        )
    }
}

/// Draw command derived only from [`RenderFrame`] snapshot data (no ECS types).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SkinnedDrawCommand {
    /// Bone palette handle bound for this draw.
    pub bone_palette: Handle<GpuBuffer>,
}

/// Plans skinned draws from a render snapshot (render-thread tick contract).
///
/// When `buffer_table` is [`Some`], proxies whose bone palette handles fail validation are skipped
/// (TC-IR-1.4.4.N1). When [`None`], every proxy becomes a draw command without handle checks.
#[must_use]
pub fn plan_skinned_draws_from_render_frame(
    frame: &RenderFrame,
    buffer_table: Option<&GpuBufferTable>,
) -> Vec<SkinnedDrawCommand> {
    match buffer_table {
        Some(table) => frame
            .skinned_meshes
            .iter()
            .filter_map(|proxy| {
                table.validate(proxy.bone_palette).ok()?;
                Some(SkinnedDrawCommand {
                    bone_palette: proxy.bone_palette,
                })
            })
            .collect(),
        None => frame
            .skinned_meshes
            .iter()
            .map(|proxy| SkinnedDrawCommand {
                bone_palette: proxy.bone_palette,
            })
            .collect(),
    }
}
