//! Morph target normalization and pass scheduling.

use crate::types::MorphTargets;

/// Returns `true` when morph accumulation should run before skinning.
#[must_use]
pub fn morph_pass_required(targets: &MorphTargets) -> bool {
    if targets.active_count == 0 {
        return false;
    }
    let n = targets.active_count as usize;
    targets.weights[..n].iter().any(|w| *w > 0.0)
}

/// Clamps morph activation to the first sixteen targets without panicking on overflow.
#[must_use]
pub fn clamp_morph_targets(mut targets: MorphTargets) -> MorphTargets {
    if targets.active_count <= 16 {
        return targets;
    }
    targets.active_count = 16;
    targets
}
