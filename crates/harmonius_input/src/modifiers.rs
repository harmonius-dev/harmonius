//! Modifier chains: dead zones, response curves, scalars.

use crate::value::ActionValue;
use glam::{Vec2, Vec3};

/// Individual modifier stage.
#[derive(Clone, Debug, PartialEq)]
pub enum InputModifier {
    /// Per-axis dead zone for 1D/2D inputs.
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
    /// Reorder axes (advanced flight / twin-stick).
    Swizzle {
        /// Component permutation.
        order: SwizzleOrder,
    },
    /// Flip axis sign.
    Negate {
        /// Negate X output.
        negate_x: bool,
        /// Negate Y output.
        negate_y: bool,
        /// Negate Z output.
        negate_z: bool,
    },
    /// Linear gain.
    Scalar {
        /// Multiplier applied after dead zone and curve.
        multiplier: f32,
    },
    /// Low-pass style smoothing (stateful).
    Smoothing {
        /// Time constant in seconds.
        time_constant: f32,
    },
    /// Ramp-up acceleration curve (stateful).
    Acceleration {
        /// Seconds to reach full multiplier.
        ramp_up_time: f32,
        /// Maximum extra gain.
        max_multiplier: f32,
        /// Decay per second toward baseline when input drops.
        decay_rate: f32,
    },
}

/// Response curve families.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ResponseCurveType {
    /// `output = input.pow(exp)` with `exp` stored alongside in first ResponseCurve use — tests
    /// use dedicated `exponential(exp)` path via chain construction.
    Exponential {
        /// Exponent > 0.
        exp: f32,
    },
}

/// Axis swizzle order (subset used in tests).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SwizzleOrder {
    /// Identity mapping.
    Xyz,
}

/// Ordered modifier list applied to a binding.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ModifierChain {
    mods: smallvec::SmallVec<[InputModifier; 4]>,
}

impl ModifierChain {
    /// Empty chain (identity).
    pub fn new() -> Self {
        Self {
            mods: smallvec::SmallVec::new(),
        }
    }

    /// Build from explicit stages.
    pub fn from_slice(mods: &[InputModifier]) -> Self {
        Self {
            mods: mods.iter().cloned().collect(),
        }
    }

    /// Apply all modifiers in order.
    pub fn apply(
        &self,
        value: ActionValue,
        dt: f32,
        state: &mut ModifierState,
    ) -> ActionValue {
        let mut v = value;
        for m in &self.mods {
            v = apply_one(m, v, dt, state);
        }
        v
    }
}

/// Stateful modifier scratch (smoothing / acceleration).
#[derive(Clone, Debug, PartialEq)]
pub struct ModifierState {
    /// Last smoothed sample.
    pub smoothed_value: ActionValue,
    /// Acceleration envelope state.
    pub acceleration_velocity: f32,
}

impl Default for ModifierState {
    fn default() -> Self {
        Self {
            smoothed_value: ActionValue::Axis1D(0.0),
            acceleration_velocity: 1.0,
        }
    }
}

fn apply_one(
    m: &InputModifier,
    value: ActionValue,
    dt: f32,
    state: &mut ModifierState,
) -> ActionValue {
    match m {
        InputModifier::DeadZoneAxial {
            threshold_x,
            threshold_y,
        } => match value {
            ActionValue::Axis2D(v) => {
                let mut o = v;
                if o.x.abs() < *threshold_x {
                    o.x = 0.0;
                }
                if o.y.abs() < *threshold_y {
                    o.y = 0.0;
                }
                ActionValue::Axis2D(o)
            }
            ActionValue::Axis1D(x) => {
                if x.abs() < *threshold_x {
                    ActionValue::Axis1D(0.0)
                } else {
                    ActionValue::Axis1D(x)
                }
            }
            _ => value,
        },
        InputModifier::DeadZoneRadial { threshold } => match value {
            ActionValue::Axis2D(v) => {
                let mag = v.length();
                if mag <= *threshold {
                    ActionValue::Axis2D(Vec2::ZERO)
                } else {
                    let remapped = (mag - threshold) / (1.0 - threshold).max(1e-6);
                    ActionValue::Axis2D(v.normalize() * remapped)
                }
            }
            ActionValue::Axis1D(x) => {
                let ax = x.abs();
                if ax <= *threshold {
                    ActionValue::Axis1D(0.0)
                } else {
                    let m = (ax - threshold) / (1.0 - threshold).max(1e-6);
                    ActionValue::Axis1D(m * x.signum())
                }
            }
            _ => value,
        },
        InputModifier::ResponseCurve { curve_type } => match curve_type {
            ResponseCurveType::Exponential { exp } => match value {
                ActionValue::Axis1D(x) => {
                    let s = x.signum();
                    ActionValue::Axis1D(s * x.abs().powf(*exp))
                }
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
        InputModifier::Swizzle { order: SwizzleOrder::Xyz } => value,
        InputModifier::Negate {
            negate_x,
            negate_y,
            negate_z,
        } => match value {
            ActionValue::Axis2D(v) => ActionValue::Axis2D(Vec2::new(
                if *negate_x { -v.x } else { v.x },
                if *negate_y { -v.y } else { v.y },
            )),
            ActionValue::Axis3D(v) => ActionValue::Axis3D(Vec3::new(
                if *negate_x { -v.x } else { v.x },
                if *negate_y { -v.y } else { v.y },
                if *negate_z { -v.z } else { v.z },
            )),
            ActionValue::Axis1D(x) => ActionValue::Axis1D(if *negate_x { -x } else { x }),
            _ => value,
        },
        InputModifier::Scalar { multiplier } => match value {
            ActionValue::Axis1D(x) => ActionValue::Axis1D(x * multiplier),
            ActionValue::Axis2D(v) => ActionValue::Axis2D(v * *multiplier),
            ActionValue::Axis3D(v) => ActionValue::Axis3D(v * *multiplier),
            _ => value,
        },
        InputModifier::Smoothing { time_constant } => {
            let a = (-dt / time_constant.max(1e-6)).exp();
            let prev = state.smoothed_value;
            let blended = match (value, prev) {
                (ActionValue::Axis1D(n), ActionValue::Axis1D(p)) => {
                    ActionValue::Axis1D(p * a + n * (1.0 - a))
                }
                (ActionValue::Axis2D(n), ActionValue::Axis2D(p)) => {
                    ActionValue::Axis2D(p * a + n * (1.0 - a))
                }
                _ => value,
            };
            state.smoothed_value = blended;
            blended
        }
        InputModifier::Acceleration {
            ramp_up_time,
            max_multiplier,
            decay_rate,
        } => {
            let target = match value {
                ActionValue::Axis1D(x) => x.abs(),
                ActionValue::Axis2D(v) => v.length(),
                _ => 0.0,
            };
            if target > 1e-4 {
                let inc = dt / ramp_up_time.max(1e-6);
                state.acceleration_velocity =
                    (state.acceleration_velocity + inc).min(*max_multiplier);
            } else {
                state.acceleration_velocity =
                    (state.acceleration_velocity - decay_rate * dt).max(1.0);
            }
            match value {
                ActionValue::Axis1D(x) => ActionValue::Axis1D(x * state.acceleration_velocity),
                ActionValue::Axis2D(v) => ActionValue::Axis2D(v * state.acceleration_velocity),
                _ => value,
            }
        }
    }
}
