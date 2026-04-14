//! Axis-aligned dirty region tracking.

use crate::aliases::SmallVec;
use glam::UVec3;

/// Inclusive integer axis-aligned box in `UVec3` space.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Region {
    /// Minimum corner (inclusive).
    pub min: UVec3,
    /// Maximum corner (inclusive).
    pub max: UVec3,
}

/// Accumulates dirty regions for frame-coalesced processing.
#[derive(Debug, Default)]
pub struct DirtyRegionSet {
    regions: SmallVec<Region, 8>,
}

impl DirtyRegionSet {
    /// Creates an empty set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            regions: SmallVec::new(),
        }
    }

    /// Marks `region` dirty.
    pub fn mark(&mut self, region: Region) {
        self.regions.push(region);
    }

    /// Drains all regions, leaving the set empty.
    pub fn drain(&mut self) -> SmallVec<Region, 8> {
        core::mem::take(&mut self.regions)
    }

    /// Clears all regions without returning them.
    pub fn clear(&mut self) {
        self.regions.clear();
    }

    /// Merges overlapping or face-adjacent regions (inclusive integer bounds).
    pub fn coalesce(&mut self) {
        if self.regions.is_empty() {
            return;
        }

        let mut changed = true;
        while changed {
            changed = false;
            'outer: for i in 0..self.regions.len() {
                for j in (i + 1)..self.regions.len() {
                    let a = self.regions[i];
                    let b = self.regions[j];
                    if regions_overlap_or_touch(a, b) {
                        let merged = Region {
                            min: a.min.min(b.min),
                            max: a.max.max(b.max),
                        };
                        self.regions[i] = merged;
                        self.regions.remove(j);
                        changed = true;
                        break 'outer;
                    }
                }
            }
        }
    }
}

fn regions_overlap_or_touch(a: Region, b: Region) -> bool {
    fn axis_overlap(a0: u32, a1: u32, b0: u32, b1: u32) -> bool {
        a0 <= b1 && b0 <= a1
    }

    axis_overlap(a.min.x, a.max.x, b.min.x, b.max.x)
        && axis_overlap(a.min.y, a.max.y, b.min.y, b.max.y)
        && axis_overlap(a.min.z, a.max.z, b.min.z, b.max.z)
}
