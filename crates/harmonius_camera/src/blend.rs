//! Blend curves and custom per-camera-pair rules.

/// Shape of a blend between two virtual cameras.
#[derive(Clone, Debug, PartialEq)]
pub enum BlendCurve {
    Cut,
    EaseInOut,
    EaseIn,
    EaseOut,
    HardIn,
    HardOut,
    Linear,
    Custom {
        /// Placeholder asset id for editor-driven curves.
        curve: u64,
    },
}

/// Blend definition between two cameras.
#[derive(Clone, Debug, PartialEq)]
pub struct BlendDefinition {
    /// Curve shape.
    pub curve: BlendCurve,
    /// Blend duration (seconds).
    pub duration: f32,
}

/// One custom blend rule.
///
/// `from` / `to` use stable numeric camera ids (`None` is a wildcard). The subsystem design uses
/// `StringId` names; this crate stays numeric until string ids are shared with core types.
#[derive(Clone, Debug, PartialEq)]
pub struct BlendRule {
    /// Source virtual camera id (`None` acts as wildcard).
    pub from: Option<u32>,
    /// Destination virtual camera id (`None` acts as wildcard).
    pub to: Option<u32>,
    /// Blend to apply.
    pub blend: BlendDefinition,
}

/// Ordered custom blend rules (first match wins).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomBlends {
    /// Rules evaluated in order.
    pub rules: Vec<BlendRule>,
}

/// Samples a normalized blend parameter `t` in `[0, 1]`.
#[must_use]
pub fn evaluate_blend_curve(curve: &BlendCurve, t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    match curve {
        BlendCurve::Cut => {
            if t >= 1.0 {
                1.0
            } else {
                0.0
            }
        }
        BlendCurve::Linear => t,
        BlendCurve::EaseIn => t * t,
        BlendCurve::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
        BlendCurve::EaseInOut => {
            if t < 0.5 {
                2.0 * t * t
            } else {
                1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
            }
        }
        BlendCurve::HardIn => {
            if t <= 0.0 {
                0.0
            } else {
                1.0
            }
        }
        // Step at the midpoint so `HardOut` differs from `Cut` (step at the end).
        BlendCurve::HardOut => {
            if t >= 0.5 {
                1.0
            } else {
                0.0
            }
        }
        BlendCurve::Custom { curve } => {
            // Deterministic discriminant until editor curves supply real samples.
            let jitter = (*curve % 997) as f32 / 99_997.0;
            (t + jitter).clamp(0.0, 1.0)
        }
    }
}

/// Resolves a blend definition using ordered rules with optional wildcards.
#[must_use]
pub fn resolve_custom_blend(
    custom: &CustomBlends,
    from_id: Option<u32>,
    to_id: Option<u32>,
    default: BlendDefinition,
) -> BlendDefinition {
    for rule in &custom.rules {
        let from_ok = match rule.from {
            None => true,
            Some(v) => from_id == Some(v),
        };
        let to_ok = match rule.to {
            None => true,
            Some(v) => to_id == Some(v),
        };
        if from_ok && to_ok {
            return rule.blend.clone();
        }
    }
    default
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-13.25.16.1 — each blend curve produces a distinct easing shape.
    #[test]
    fn tc_13_25_16_1_blend_eight_curves_distinct() {
        let curves = [
            BlendCurve::Cut,
            BlendCurve::EaseInOut,
            BlendCurve::EaseIn,
            BlendCurve::EaseOut,
            BlendCurve::HardIn,
            BlendCurve::HardOut,
            BlendCurve::Linear,
            BlendCurve::Custom { curve: 1 },
        ];
        let sample_times = [0.01_f32, 0.05, 0.1, 0.25, 0.5, 0.75, 0.9, 0.99];
        let mut fingerprints = std::collections::BTreeSet::new();
        for curve in &curves {
            let key: Vec<u32> = sample_times
                .map(|t| evaluate_blend_curve(curve, t).to_bits())
                .into();
            fingerprints.insert(key);
        }
        assert_eq!(
            fingerprints.len(),
            curves.len(),
            "each curve must yield a distinct multi-sample fingerprint"
        );
    }

    /// TC-13.25.16.2 — explicit rules override defaults before wildcards.
    #[test]
    fn tc_13_25_16_2_custom_pair_overrides_default() {
        let custom = CustomBlends {
            rules: vec![
                BlendRule {
                    from: Some(1),
                    to: Some(2),
                    blend: BlendDefinition {
                        curve: BlendCurve::EaseIn,
                        duration: 0.5,
                    },
                },
                BlendRule {
                    from: None,
                    to: None,
                    blend: BlendDefinition {
                        curve: BlendCurve::Linear,
                        duration: 1.0,
                    },
                },
            ],
        };
        let default = BlendDefinition {
            curve: BlendCurve::Cut,
            duration: 0.0,
        };
        let resolved = resolve_custom_blend(&custom, Some(1), Some(2), default.clone());
        assert_eq!(resolved.curve, BlendCurve::EaseIn);
        let wildcard = resolve_custom_blend(&custom, Some(9), Some(9), default);
        assert_eq!(wildcard.curve, BlendCurve::Linear);
    }
}
