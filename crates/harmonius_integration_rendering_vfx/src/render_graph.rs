//! Deterministic render-graph pass ordering for the Rendering ↔ VFX seam.

use crate::types::DispatchQueue;
use crate::types::RenderMode;

/// Recorded pass kinds for validation tests (IR-3.7.1 / IR-3.7.2 ordering).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecordedPass {
    /// GPU particle simulation compute.
    ParticleSimCompute,
    /// Barrier between simulation writes and draw reads of the same buffers.
    BarrierSimToDraw,
    /// Optional radix sort compute for transparent particles.
    RadixSortParticles,
    /// Deferred decal G-buffer modification.
    DecalGbufferPass,
    /// Froxel density injection from VFX volumes.
    FroxelInjectionPass,
    /// Cluster grid update consuming injected lights.
    ClusterLightGridPass,
    /// Indirect particle draw on the graphics queue.
    ParticleDrawIndirect(RenderMode),
    /// Post-process screen effect before tonemapping.
    ScreenEffectPass(crate::types::ScreenEffectKind),
    /// HDR tonemap (marker for ordering assertions only).
    TonemapPass,
}

/// Returns the canonical pass slice for GPU particle sim + sorted transparent draw.
///
/// TC-IR-3.7.1.1: compute registration precedes draw.
/// TC-IR-3.7.1.2: explicit barrier between sim write and draw read.
pub fn particle_sim_then_draw_pipeline(render_mode: RenderMode) -> Vec<RecordedPass> {
    vec![
        RecordedPass::ParticleSimCompute,
        RecordedPass::BarrierSimToDraw,
        RecordedPass::RadixSortParticles,
        RecordedPass::ParticleDrawIndirect(render_mode),
    ]
}

/// Full-frame slice including decals, froxel, lights, screen effects, tonemap.
pub fn frame_passes_with_vfx_tail(render_mode: RenderMode) -> Vec<RecordedPass> {
    let mut v = particle_sim_then_draw_pipeline(render_mode);
    v.push(RecordedPass::DecalGbufferPass);
    v.push(RecordedPass::FroxelInjectionPass);
    v.push(RecordedPass::ClusterLightGridPass);
    v.push(RecordedPass::ScreenEffectPass(crate::types::ScreenEffectKind::HeatHaze));
    v.push(RecordedPass::TonemapPass);
    v
}

/// TC-IR-3.7.1.F1 — fence timeout on async compute forces graphics reissue.
pub fn recover_particle_sim_dispatch(async_compute_stalled: bool) -> DispatchQueue {
    if async_compute_stalled {
        DispatchQueue::Graphics
    } else {
        DispatchQueue::AsyncCompute
    }
}
