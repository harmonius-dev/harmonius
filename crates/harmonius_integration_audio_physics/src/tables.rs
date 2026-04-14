//! Material-pair lookup tables for impacts and friction.

use crate::ids::{AssetHandle, AudioClip};
use crate::surface::{SurfaceType, SURFACE_TYPE_COUNT};

/// Up to eight clip variants per material pair.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ImpactSoundSet {
    /// Randomized variants to avoid repetition.
    pub clips: [Option<AssetHandle<AudioClip>>; 8],
    /// Minimum impulse required before playing.
    pub threshold: f32,
    /// Cooldown window in seconds between repeats for the same pair.
    pub cooldown_sec: f32,
}

impl ImpactSoundSet {
    /// Picks the first populated clip starting at `salt % 8` and scanning cyclically.
    ///
    /// Deterministic stand-in for the design’s `pick_random()` so tests and replays stay stable.
    pub fn pick_clip(self, salt: u32) -> Option<AssetHandle<AudioClip>> {
        let start = (salt as usize) % self.clips.len();
        for i in 0..self.clips.len() {
            let idx = (start + i) % self.clips.len();
            if let Some(handle) = self.clips[idx] {
                return Some(handle);
            }
        }
        None
    }

    /// Returns whether the first clip slot is populated (used for defaulting logic).
    pub fn has_primary_clip(self) -> bool {
        self.clips[0].is_some()
    }
}

/// Material-pair impact table with symmetric lookup.
#[derive(Clone, Debug, PartialEq)]
pub struct ImpactSoundTable {
    /// Dense symmetric pair entries.
    pub entries: [[ImpactSoundSet; SURFACE_TYPE_COUNT]; SURFACE_TYPE_COUNT],
    /// Fallback when a cell has no primary clip.
    pub default: ImpactSoundSet,
}

impl ImpactSoundTable {
    /// Symmetric lookup with fallback to [`Self::default`] when the cell has no primary clip.
    pub fn get(&self, a: SurfaceType, b: SurfaceType) -> &ImpactSoundSet {
        let (lo, hi) = if a.index() <= b.index() {
            (a.index(), b.index())
        } else {
            (b.index(), a.index())
        };
        let set = &self.entries[lo][hi];
        if set.has_primary_clip() {
            set
        } else {
            &self.default
        }
    }
}

/// Friction clip lookup table.
#[derive(Clone, Debug, PartialEq)]
pub struct FrictionSoundTable {
    /// Symmetric pair entries.
    pub entries: [[Option<AssetHandle<AudioClip>>; SURFACE_TYPE_COUNT]; SURFACE_TYPE_COUNT],
    /// Fallback clip when a pair is missing.
    pub default: AssetHandle<AudioClip>,
}

impl FrictionSoundTable {
    /// Symmetric clip lookup with default fallback.
    pub fn pick(&self, a: SurfaceType, b: SurfaceType) -> AssetHandle<AudioClip> {
        let (lo, hi) = if a.index() <= b.index() {
            (a.index(), b.index())
        } else {
            (b.index(), a.index())
        };
        self.entries[lo][hi].unwrap_or(self.default)
    }
}
