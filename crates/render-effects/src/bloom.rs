//! Bloom thresholding and pass-list construction.

use crate::quality::{BloomImplementation, QualityTier};

/// User-facing bloom parameters (subset of design `Bloom`).
#[derive(Clone, Copy, Debug)]
pub struct BloomParams {
    /// Luminance threshold before bloom contribution.
    pub threshold: f32,
}

/// Single bloom compute pass classification.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BloomPassKind {
    /// Dual Kawase downsample step.
    DualKawaseDown,
    /// Dual Kawase upsample step.
    DualKawaseUp,
    /// Gaussian downsample for a mip level.
    GaussianDown {
        /// Zero-based mip index.
        mip: u8,
    },
}

/// Planned bloom pass for registration into a render graph.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BloomPass {
    /// Pass classification.
    pub kind: BloomPassKind,
}

/// Prefilter luminance for bloom using a hard knee at `threshold` (TC-2.9.1.1).
pub fn bloom_threshold_prefilter(luminance: f32, params: &BloomParams) -> f32 {
    if luminance > params.threshold {
        luminance - params.threshold
    } else {
        0.0
    }
}

/// Builds the bloom pass list for `tier` when bloom is enabled.
pub fn build_bloom_passes(tier: QualityTier, bloom_enabled: bool) -> Vec<BloomPass> {
    if !bloom_enabled {
        return Vec::new();
    }
    match tier {
        QualityTier::Mobile => (0..4)
            .flat_map(|_| {
                [
                    BloomPass {
                        kind: BloomPassKind::DualKawaseDown,
                    },
                    BloomPass {
                        kind: BloomPassKind::DualKawaseUp,
                    },
                ]
            })
            .collect(),
        QualityTier::HighEnd => (0..6)
            .map(|m| BloomPass {
                kind: BloomPassKind::GaussianDown { mip: m },
            })
            .collect(),
        QualityTier::Switch | QualityTier::Desktop => (0..4)
            .flat_map(|_| {
                [
                    BloomPass {
                        kind: BloomPassKind::DualKawaseDown,
                    },
                    BloomPass {
                        kind: BloomPassKind::DualKawaseUp,
                    },
                ]
            })
            .collect(),
    }
}

/// Counts Gaussian mip levels in `passes` (TC-2.9.1.3).
pub fn gaussian_mip_count(passes: &[BloomPass]) -> u8 {
    passes
        .iter()
        .filter(|p| matches!(p.kind, BloomPassKind::GaussianDown { .. }))
        .count() as u8
}

/// True when every pass is Gaussian downsample (TC-2.9.1.3).
pub fn all_passes_gaussian(passes: &[BloomPass]) -> bool {
    !passes.is_empty()
        && passes.iter().all(|p| matches!(p.kind, BloomPassKind::GaussianDown { .. }))
}

/// Bloom blur method implied by `passes`.
pub fn bloom_blur_method(passes: &[BloomPass]) -> Option<BloomImplementation> {
    if passes.is_empty() {
        return None;
    }
    if passes.iter().all(|p| {
        matches!(
            p.kind,
            BloomPassKind::DualKawaseDown | BloomPassKind::DualKawaseUp
        )
    }) {
        Some(BloomImplementation::DualKawase)
    } else if all_passes_gaussian(passes) {
        Some(BloomImplementation::Gaussian)
    } else {
        None
    }
}
