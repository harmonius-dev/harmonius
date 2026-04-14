//! Quality tiers and effect configuration (TC-2.9.13.1).

/// Target platform quality preset (matches render-effects design).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QualityTier {
    /// Mobile presets: fewer passes, capped custom post.
    Mobile,
    /// Nintendo Switch class handheld/console docked.
    Switch,
    /// Desktop default tier.
    Desktop,
    /// Desktop high-end reference path.
    HighEnd,
}

/// Bloom implementation variant selected for a tier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BloomImplementation {
    /// Dual-Kawase down/up chain.
    DualKawase,
    /// Wide Gaussian mip chain.
    Gaussian,
}

/// Resolved post-process capabilities for a tier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EffectConfig {
    /// Depth-of-field enabled.
    pub dof: bool,
    /// Bloom variant when bloom is enabled.
    pub bloom: BloomImplementation,
    /// Fullscreen motion blur enabled.
    pub motion_blur: bool,
    /// Maximum custom post-process material passes.
    pub max_pp: u8,
}

/// Build [`EffectConfig`] for `tier` (TC-2.9.13.1).
pub fn effect_config_for_tier(tier: QualityTier) -> EffectConfig {
    match tier {
        QualityTier::Mobile => EffectConfig {
            dof: false,
            bloom: BloomImplementation::DualKawase,
            motion_blur: false,
            max_pp: 1,
        },
        QualityTier::Switch | QualityTier::Desktop | QualityTier::HighEnd => EffectConfig {
            dof: true,
            bloom: BloomImplementation::Gaussian,
            motion_blur: true,
            max_pp: 8,
        },
    }
}

