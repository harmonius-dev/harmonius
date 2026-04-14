//! Accuracy decay configuration.

use rkyv::{Archive, Deserialize, Serialize};

/// Shape of the accuracy decay curve.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
pub enum DecayCurveType {
    /// Subtract `rate` per elapsed tick from accuracy.
    Linear,
    /// Multiply accuracy by `(1.0 - rate)` once per elapsed tick.
    Exponential,
    /// Hold accuracy until `rate` ticks elapse, then snap to `min_accuracy`.
    Step,
}

/// Immutable decay configuration baked with assets.
#[derive(Clone, Debug, PartialEq, Archive, Serialize, Deserialize)]
pub struct DecayCurve {
    /// Interpretation depends on [`DecayCurveType`].
    pub rate: f32,
    /// Floor accuracy; entries never decay below this value through the curve.
    pub min_accuracy: f32,
    /// Shape selector.
    pub curve_type: DecayCurveType,
}

/// Computes accuracy after `elapsed` ticks from an initial value of `1.0`.
pub(crate) fn accuracy_at_tick(curve: &DecayCurve, elapsed: u64) -> f32 {
    let elapsed_f = elapsed as f32;
    let mut acc = match curve.curve_type {
        DecayCurveType::Linear => 1.0_f32 - curve.rate * elapsed_f,
        DecayCurveType::Exponential => {
            let base = (1.0_f32 - curve.rate).max(0.0);
            base.powf(elapsed_f)
        }
        DecayCurveType::Step => {
            let threshold = curve.rate.max(0.0) as u64;
            if elapsed < threshold {
                1.0
            } else {
                curve.min_accuracy
            }
        }
    };
    if curve.curve_type != DecayCurveType::Step {
        acc = acc.max(curve.min_accuracy);
    }
    acc.clamp(0.0, 1.0_f32)
}
