//! Phase-7 render snapshot handoff (IR-3.7.1.S1).

use crate::compiler::CompiledPassDesc;

/// Immutable handoff from simulation to render thread (clone for triple-buffer snapshots).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RenderFrameVfxSnapshot {
    /// Flattened compiled passes ready for render-graph registration.
    pub compiled_passes: Vec<CompiledPassDesc>,
}

/// Moves producer-side pass descriptors into an immutable snapshot.
///
/// TC-IR-3.7.1.S1 — render thread consumes `RenderFrameVfxSnapshot` without shared mutation.
pub fn drain_passes_to_snapshot(passes: Vec<CompiledPassDesc>) -> RenderFrameVfxSnapshot {
    RenderFrameVfxSnapshot {
        compiled_passes: passes,
    }
}
