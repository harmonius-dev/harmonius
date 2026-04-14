//! Record-only [`GpuSkinner`] used to validate compute ordering contracts.

use crate::lod::HalfRateStaleTracker;
use crate::morph::morph_pass_required;
use crate::types::AnimationLodTier;
use crate::types::BlendDescriptor;
use crate::types::MorphTargets;
use crate::types::SkinnedMeshProxy;
use crate::types::SkinningDispatch;
use crate::types::SkinningMode;

/// Recorded compute ordering for assertions (morph accumulation before skinning).
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ComputePass {
    /// Sparse morph delta accumulation stage.
    MorphAccum,
    /// Linear or dual-quaternion skinning dispatch.
    Skinning {
        /// Skinning mode selected for the dispatch.
        mode: SkinningMode,
    },
    /// Zero blend weight path binds bind-pose geometry without skinning compute.
    BindPose,
    /// Half-rate tier skipped skinning on an off-frame.
    HalfRateSkip,
}

/// Test double recording compute passes in invocation order.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct GpuSkinner {
    /// Ordered list of passes recorded by the render thread double.
    pub log: Vec<ComputePass>,
}

impl GpuSkinner {
    /// Creates an empty logger.
    #[must_use]
    pub fn new() -> Self {
        Self { log: Vec::new() }
    }

    /// Records morph accumulation when morph weights require work.
    pub fn dispatch_morph(&mut self, targets: &MorphTargets) {
        if morph_pass_required(targets) {
            self.log.push(ComputePass::MorphAccum);
        }
    }

    /// Records a skinning compute dispatch for the given mode.
    pub fn dispatch_skinning(&mut self, mode: SkinningMode) {
        self.log.push(ComputePass::Skinning { mode });
    }

    /// Records one instanced skinning batch from a [`SkinningDispatch`] contract value.
    pub fn dispatch_skinning_dispatch(&mut self, dispatch: &SkinningDispatch) {
        self.log.push(ComputePass::Skinning {
            mode: dispatch.mode,
        });
    }

    /// Records bind-pose fallback when blend weights are all zero.
    pub fn bind_bind_pose_only(&mut self) {
        self.log.push(ComputePass::BindPose);
    }

    /// Records that half-rate reuse skipped skinning on this frame.
    pub fn record_half_rate_skip(&mut self) {
        self.log.push(ComputePass::HalfRateSkip);
    }
}

/// Returns `true` when every active blend weight is `0.0` (bind-pose path).
///
/// When `active_count == 0`, upstream must treat the descriptor as “no active clips” and this
/// crate selects bind pose; callers with explicit all-zero weights must set `active_count > 0`.
#[must_use]
pub fn zero_blend_weights(blend: &BlendDescriptor) -> bool {
    if blend.active_count == 0 {
        return true;
    }
    let n = blend.active_count as usize;
    blend.clip_weights[..n].iter().all(|w| *w == 0.0)
}

/// Returns the next instance batch size after a GPU timeout recovery step.
#[must_use]
pub fn next_batch_size_after_gpu_timeout(current: u32) -> u32 {
    (current / 2).max(1)
}

/// Records morph accumulation (when required) then one instanced batch dispatch.
///
/// Ordering matches TC-IR-1.4.2.1 for batched work: morph deltas before skinning compute.
pub fn dispatch_morph_then_skinning_dispatch(
    skinner: &mut GpuSkinner,
    morph: &MorphTargets,
    morph_buffer_bound: bool,
    dispatch: &SkinningDispatch,
) {
    if morph_buffer_bound && morph_pass_required(morph) {
        skinner.dispatch_morph(morph);
    }
    skinner.dispatch_skinning_dispatch(dispatch);
}

/// Issues morph-then-skin passes for one skinned mesh proxy (render-thread contract).
///
/// When `half_rate_tracker` is [`Some`], [`HalfRateStaleTracker`] participates in half-rate skip vs
/// forced-full scheduling (IR-1.4.3.N2). When [`None`], half-rate off-frames always skip skinning.
pub fn dispatch_skinning_passes_for_proxy(
    skinner: &mut GpuSkinner,
    blend: &BlendDescriptor,
    morph: &MorphTargets,
    mesh: &SkinnedMeshProxy,
    evaluate_this_frame: bool,
    half_rate_tracker: Option<&mut HalfRateStaleTracker>,
) {
    if mesh.morph_buffer.is_some() && morph_pass_required(morph) {
        skinner.dispatch_morph(morph);
    }
    if mesh.lod_tier == AnimationLodTier::Vat {
        return;
    }
    if mesh.lod_tier == AnimationLodTier::HalfRate {
        if let Some(tracker) = half_rate_tracker {
            if evaluate_this_frame {
                let _ = half_rate_force_full_after_stale(tracker, true);
            } else if !half_rate_force_full_after_stale(tracker, false) {
                skinner.record_half_rate_skip();
                return;
            }
        } else if !evaluate_this_frame {
            skinner.record_half_rate_skip();
            return;
        }
    }
    if zero_blend_weights(blend) {
        skinner.bind_bind_pose_only();
        return;
    }
    skinner.dispatch_skinning(mesh.skinning_mode);
}

/// Drives [`HalfRateStaleTracker`] from per-frame evaluation flags (IR-1.4.3.N2).
#[must_use]
pub fn half_rate_force_full_after_stale(
    tracker: &mut HalfRateStaleTracker,
    evaluated: bool,
) -> bool {
    let force = tracker.record_frame(evaluated);
    if force {
        tracker.clear_after_forced_full();
    }
    force
}
