//! Modifier chain stages used before CIAC sensitivity scaling.

use crate::action::ActionValue;
use glam::Vec2;

/// Response curve families.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ResponseCurveType {
    /// `output = sign(input) * |input|^exp` per axis / magnitude.
    Exponential {
        /// Exponent > 0.
        exp: f32,
    },
}

/// Individual modifier stage.
#[derive(Clone, Debug, PartialEq)]
pub enum InputModifier {
    /// Per-axis dead zone for 2D inputs.
    DeadZoneAxial {
        /// X threshold 0..1.
        threshold_x: f32,
        /// Y threshold 0..1.
        threshold_y: f32,
    },
    /// Magnitude-based dead zone for 2D sticks.
    DeadZoneRadial {
        /// Inner radius before output rises from zero.
        threshold: f32,
    },
    /// Non-linear shaping.
    ResponseCurve {
        /// Curve family.
        curve_type: ResponseCurveType,
    },
}

/// Stateful modifier scratch (reserved for future smoothing stages).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ModifierState {
    /// Placeholder for future smoothing state.
    pub _reserved: (),
}

/// Ordered modifier list applied to a binding.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ModifierChain {
    mods: Vec<InputModifier>,
}

impl ModifierChain {
    /// Empty chain (identity).
    pub fn new() -> Self {
        Self { mods: Vec::new() }
    }

    /// Build from explicit stages.
    pub fn from_slice(mods: &[InputModifier]) -> Self {
        Self {
            mods: mods.to_vec(),
        }
    }

    /// Apply all modifiers in order (stateless stages only).
    pub fn apply(&self, value: ActionValue, _state: &mut ModifierState) -> ActionValue {
        let mut v = value;
        for m in &self.mods {
            v = apply_one(m, v);
        }
        v
    }
}

fn apply_one(m: &InputModifier, value: ActionValue) -> ActionValue {
    match m {
        InputModifier::DeadZoneAxial {
            threshold_x,
            threshold_y,
        } => match value {
            ActionValue::Axis2D(mut v) => {
                if v.x.abs() < *threshold_x {
                    v.x = 0.0;
                }
                if v.y.abs() < *threshold_y {
                    v.y = 0.0;
                }
                ActionValue::Axis2D(v)
            }
            _ => value,
        },
        InputModifier::DeadZoneRadial { threshold } => match value {
            ActionValue::Axis2D(vec) => {
                let mag = vec.length();
                if mag <= *threshold {
                    ActionValue::Axis2D(Vec2::ZERO)
                } else {
                    let denom = (1.0 - threshold).max(1e-6);
                    let remapped = (mag - threshold) / denom;
                    ActionValue::Axis2D(vec.normalize() * remapped)
                }
            }
            _ => value,
        },
        InputModifier::ResponseCurve { curve_type } => match curve_type {
            ResponseCurveType::Exponential { exp } => match value {
                ActionValue::Axis2D(v) => {
                    let mag = v.length();
                    if mag <= 1e-8 {
                        ActionValue::Axis2D(Vec2::ZERO)
                    } else {
                        let nm = mag.powf(*exp);
                        ActionValue::Axis2D(v.normalize() * nm)
                    }
                }
                _ => value,
            },
        },
    }
}
