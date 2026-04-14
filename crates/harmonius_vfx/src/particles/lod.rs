//! Distance-based LOD tiers with hysteresis (`TC-11.1.4.1`, `TC-11.1.4.2`).

/// Distance thresholds for tier transitions (see `docs/design/vfx/particles.md`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LodConfig {
    /// Distance above which we leave [`LodTier::Full`].
    pub full_distance: f32,
    /// Distance above which we leave [`LodTier::Reduced`].
    pub reduced_distance: f32,
    /// Distance above which we leave [`LodTier::Impostor`].
    pub impostor_distance: f32,
    /// Distance above which we enter [`LodTier::Culled`].
    pub cull_distance: f32,
    /// Hysteresis band applied when upgrading to a better tier.
    pub hysteresis: f32,
}

impl Default for LodConfig {
    fn default() -> Self {
        Self {
            full_distance: 30.0,
            reduced_distance: 60.0,
            impostor_distance: 100.0,
            cull_distance: 150.0,
            hysteresis: 5.0,
        }
    }
}

/// Simulation and rendering fidelity tier for an emitter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LodTier {
    /// Full simulation and rendering.
    Full,
    /// Reduced spawn rate, core modules only.
    Reduced,
    /// No new spawns, billboard impostor.
    Impostor,
    /// Completely culled.
    Culled,
}

/// Updates `current` given camera-to-emitter `distance` and hysteresis rules from the design
/// state machine.
#[must_use]
pub fn lod_tier_after_distance(current: LodTier, distance: f32, cfg: &LodConfig) -> LodTier {
    let h = cfg.hysteresis.max(0.0);
    match current {
        LodTier::Full => {
            if distance > cfg.full_distance {
                LodTier::Reduced
            } else {
                LodTier::Full
            }
        }
        LodTier::Reduced => {
            if distance < cfg.full_distance - h {
                LodTier::Full
            } else if distance > cfg.reduced_distance {
                LodTier::Impostor
            } else {
                LodTier::Reduced
            }
        }
        LodTier::Impostor => {
            if distance < cfg.reduced_distance - h {
                LodTier::Reduced
            } else if distance > cfg.cull_distance {
                LodTier::Culled
            } else {
                LodTier::Impostor
            }
        }
        LodTier::Culled => {
            if distance < cfg.cull_distance - h {
                LodTier::Impostor
            } else {
                LodTier::Culled
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `TC-11.1.4.1` — move emitter through four distance bands.
    #[test]
    fn tc_11_1_4_1_lod_tier_transitions() {
        let cfg = LodConfig {
            full_distance: 10.0,
            reduced_distance: 20.0,
            impostor_distance: 30.0,
            cull_distance: 40.0,
            hysteresis: 2.0,
        };
        let mut tier = LodTier::Full;
        tier = lod_tier_after_distance(tier, 11.0, &cfg);
        assert_eq!(tier, LodTier::Reduced);
        tier = lod_tier_after_distance(tier, 21.0, &cfg);
        assert_eq!(tier, LodTier::Impostor);
        tier = lod_tier_after_distance(tier, 41.0, &cfg);
        assert_eq!(tier, LodTier::Culled);
    }

    /// `TC-11.1.4.2` — oscillation at boundary does not flip tiers each step.
    #[test]
    fn tc_11_1_4_2_lod_hysteresis() {
        let cfg = LodConfig {
            full_distance: 30.0,
            reduced_distance: 60.0,
            impostor_distance: 100.0,
            cull_distance: 150.0,
            hysteresis: 5.0,
        };
        let mut tier = LodTier::Reduced;
        let distances = [31.0, 29.0, 31.0, 29.0, 31.0];
        for d in distances {
            tier = lod_tier_after_distance(tier, d, &cfg);
        }
        assert_eq!(
            tier,
            LodTier::Reduced,
            "must stay Reduced until d < full_distance - hysteresis"
        );
        tier = lod_tier_after_distance(tier, 24.0, &cfg);
        assert_eq!(tier, LodTier::Full);
    }
}
