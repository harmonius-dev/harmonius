//! Level-of-detail tier selection from camera distance (integration test thresholds).

use crate::types::AnimationLodTier;

#[cfg(test)]
use core::sync::atomic::AtomicU64;
#[cfg(test)]
use core::sync::atomic::Ordering;

#[cfg(test)]
static INVALID_LOD_WARNINGS: AtomicU64 = AtomicU64::new(0);

/// Returns how many times [`sanitize_lod_tier`] fell back due to invalid input.
///
/// Outside unit tests this always returns `0` (no process-global counter in release builds).
#[must_use]
pub fn invalid_lod_warning_count() -> u64 {
    #[cfg(test)]
    {
        INVALID_LOD_WARNINGS.load(Ordering::Relaxed)
    }
    #[cfg(not(test))]
    {
        0
    }
}

/// Resets the invalid-LOD warning counter (test hook).
pub fn reset_invalid_lod_warning_count() {
    #[cfg(test)]
    {
        INVALID_LOD_WARNINGS.store(0, Ordering::Relaxed);
    }
}

/// Returns `true` when the tier evaluates the full skeleton bone set (design “all bones”).
#[must_use]
pub fn lod_tier_evaluates_full_skeleton(tier: AnimationLodTier) -> bool {
    matches!(tier, AnimationLodTier::Full)
}

/// Maps authored distance in meters to an evaluation tier using design test thresholds.
///
/// - `< 10` m → [`AnimationLodTier::Full`]
/// - `30..=60` m → [`AnimationLodTier::HalfRate`]
/// - `> 100` m → [`AnimationLodTier::Vat`]
/// - Otherwise → [`AnimationLodTier::ReducedBones`] as the intermediate band.
#[must_use]
pub fn lod_tier_from_distance_m(distance_m: f32) -> AnimationLodTier {
    if distance_m < 10.0 {
        AnimationLodTier::Full
    } else if distance_m > 100.0 {
        AnimationLodTier::Vat
    } else if (30.0..=60.0).contains(&distance_m) {
        AnimationLodTier::HalfRate
    } else {
        AnimationLodTier::ReducedBones
    }
}

/// Repairs corrupted tier discriminants by falling back to [`AnimationLodTier::Full`].
#[must_use]
pub fn sanitize_lod_tier(raw: u8) -> AnimationLodTier {
    match raw {
        0 => AnimationLodTier::Full,
        1 => AnimationLodTier::ReducedBones,
        2 => AnimationLodTier::HalfRate,
        3 => AnimationLodTier::Vat,
        _ => {
            #[cfg(test)]
            {
                INVALID_LOD_WARNINGS.fetch_add(1, Ordering::Relaxed);
            }
            AnimationLodTier::Full
        }
    }
}

/// Tracks consecutive skipped evaluations for [`AnimationLodTier::HalfRate`].
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HalfRateStaleTracker {
    missed_frames: u8,
}

impl HalfRateStaleTracker {
    /// Creates a tracker starting at zero missed frames.
    #[must_use]
    pub const fn new() -> Self {
        Self { missed_frames: 0 }
    }

    /// Records whether this entity evaluated this frame; returns `true` when a full eval is forced.
    pub fn record_frame(&mut self, evaluated_this_frame: bool) -> bool {
        if evaluated_this_frame {
            self.missed_frames = 0;
            false
        } else {
            self.missed_frames = self.missed_frames.saturating_add(1);
            self.missed_frames > 2
        }
    }

    /// Resets missed-frame count after a forced full evaluation.
    pub fn clear_after_forced_full(&mut self) {
        self.missed_frames = 0;
    }
}
