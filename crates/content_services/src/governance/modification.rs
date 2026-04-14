//! Human modification bitmask tracking for AI-generated assets.

/// Spatial or logical subdivision granularity for bitmask tracking.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrackingGranularity {
    /// Uniform grid cells across the asset.
    UniformGrid,
}

/// Bitmask describing how much of an AI-generated asset was replaced by human edits.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModificationBitmask {
    /// One bit per region; length matches `region_count` rounded up to whole `u64` words.
    words: Vec<u64>,
    /// Number of logical regions tracked (TC-15.7.2 uses 10).
    region_count: u32,
}

impl ModificationBitmask {
    /// Creates an empty bitmask for `region_count` regions.
    pub fn new(_granularity: TrackingGranularity, region_count: u32) -> Self {
        let word_len = (region_count as usize).div_ceil(64);
        Self {
            words: vec![0u64; word_len.max(1)],
            region_count,
        }
    }

    /// Marks a single region as modified (`region` is zero-based). Out-of-range indices are ignored.
    pub fn mark_modified(&mut self, region: u32) {
        if region >= self.region_count {
            return;
        }
        let idx = region as usize / 64;
        let bit = region as usize % 64;
        self.words[idx] |= 1u64 << bit;
    }

    /// Approximate percentage of regions touched (TC-15.7.2.1).
    pub fn modification_pct(&self) -> f32 {
        if self.region_count == 0 {
            return 0.0;
        }
        let set = self.set_count();
        (set as f32 / self.region_count as f32) * 100.0
    }

    /// True when every region has been modified (TC-15.7.2.2).
    pub fn is_fully_replaced(&self) -> bool {
        self.set_count() == self.region_count
    }

    fn set_count(&self) -> u32 {
        let mut count = 0u32;
        for r in 0..self.region_count {
            let idx = r as usize / 64;
            let bit = r as usize % 64;
            if (self.words[idx] & (1u64 << bit)) != 0 {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-15.7.2.1 — bitmask mark single region (~10% for 10 regions).
    #[test]
    fn tc_15_7_2_1_modification_pct_single_region() {
        let mut mask = ModificationBitmask::new(TrackingGranularity::UniformGrid, 10);
        mask.mark_modified(3);
        let pct = mask.modification_pct();
        assert!((pct - 10.0).abs() < 0.01, "expected ~10.0, got {pct}");
    }

    /// TC-15.7.2.2 — all regions modified counts as fully replaced.
    #[test]
    fn tc_15_7_2_2_fully_replaced() {
        let mut mask = ModificationBitmask::new(TrackingGranularity::UniformGrid, 10);
        for r in 0..10 {
            mask.mark_modified(r);
        }
        assert!(mask.is_fully_replaced());
    }
}
