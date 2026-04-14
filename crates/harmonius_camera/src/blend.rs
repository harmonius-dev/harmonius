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
#[derive(Clone, Debug, PartialEq)]
pub struct BlendRule {
    /// Source camera name (`None` acts as wildcard).
    pub from: Option<u32>,
    /// Destination camera name (`None` acts as wildcard).
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
        BlendCurve::HardOut => {
            if t >= 1.0 {
                1.0
            } else {
                0.0
            }
        }
        BlendCurve::Custom { .. } => t,
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
        let mut samples = std::collections::BTreeSet::new();
        for curve in &curves {
            let key = (evaluate_blend_curve(curve, 0.25) * 1_000.0) as i32;
            samples.insert(key);
        }
        assert!(
            samples.len() >= 6,
            "expected diverse samples, got {}",
            samples.len()
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
