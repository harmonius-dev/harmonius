//! Utility AI curves, compensation, selection, dual-axis, and contexts.

use rand::rngs::SmallRng;
use rand::Rng;

/// Response curve shapes from the design doc.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ResponseCurve {
    /// `output = clamp01(slope * input + intercept)`.
    Linear {
        /// Multiplier on normalized input.
        slope: f32,
        /// Added before clamping.
        intercept: f32,
    },
    /// `a*input^2 + b*input + c`, then clamped.
    Quadratic {
        /// Quadratic coefficient.
        a: f32,
        /// Linear coefficient.
        b: f32,
        /// Constant term.
        c: f32,
    },
    /// Logistic with steepness `k` and midpoint.
    Logistic {
        /// Steepness.
        k: f32,
        /// Inflection input.
        midpoint: f32,
    },
    /// Step at threshold.
    Step {
        /// Threshold in normalized input space.
        threshold: f32,
    },
}

impl ResponseCurve {
    /// Evaluates the curve on `input` (output clamped to `[0,1]`).
    pub fn evaluate(self, input: f32) -> f32 {
        let raw = match self {
            ResponseCurve::Linear { slope, intercept } => slope * input + intercept,
            ResponseCurve::Quadratic { a, b, c } => a * input * input + b * input + c,
            ResponseCurve::Logistic { k, midpoint } => {
                let x = k * (input - midpoint);
                1.0 / (1.0 + (-x).exp())
            }
            ResponseCurve::Step { threshold } => {
                if input >= threshold {
                    1.0
                } else {
                    0.0
                }
            }
        };
        clamp01(raw)
    }
}

fn clamp01(v: f32) -> f32 {
    v.clamp(0.0, 1.0)
}

/// Designer-authored input channel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputAxis {
    /// Normalized health.
    HealthRatio,
    /// Normalized distance.
    DistanceToTarget,
    /// Abstract threat.
    ThreatLevel,
    /// Seconds since last action.
    TimeSinceAction,
    /// Normalized ammo.
    AmmoCount,
    /// Line of sight boolean as float.
    LineOfSight,
    /// Custom slot (index into parallel table).
    Custom(u32),
}

/// One consideration: axis + curve.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Consideration {
    /// Source channel.
    pub input_axis: InputAxis,
    /// Shaping curve.
    pub curve: ResponseCurve,
}

/// Registered utility action.
#[derive(Clone, Debug, PartialEq)]
pub struct UtilityAction {
    /// Stable action id.
    pub id: u32,
    /// Indices into a shared [`Consideration`] table.
    pub considerations: Vec<usize>,
    /// Owning context id.
    pub context_id: u32,
    /// Category bucket for dual-axis.
    pub category_id: u32,
}

/// Category metadata for dual-axis ranking.
#[derive(Clone, Debug, PartialEq)]
pub struct CategoryDef {
    /// Category id matching [`UtilityAction::category_id`].
    pub id: u32,
    /// Larger wins across categories before intra-category score.
    pub priority: u32,
    /// Debug label.
    pub name: String,
}

/// Declares which actions belong to a context and its hysteresis thresholds.
#[derive(Clone, Debug, PartialEq)]
pub struct ContextSet {
    /// Context id.
    pub id: u32,
    /// Member actions.
    pub actions: Vec<u32>,
    /// Minimum aggregate score to enter.
    pub enter_threshold: f32,
    /// Score required to leave once active.
    pub exit_threshold: f32,
}

/// Post-scoring selection mode.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SelectionStrategy {
    /// Pick max score.
    Highest,
    /// Weighted draw among top `n`.
    WeightedRandomTopN {
        /// How many top actions participate.
        n: u32,
    },
    /// Drop actions below `min_score`.
    ThresholdFilter {
        /// Cutoff.
        min_score: f32,
    },
}

/// Scoring output for one action.
#[derive(Clone, Debug, PartialEq)]
pub struct ScoredAction {
    /// Action id.
    pub action_id: u32,
    /// After compensation.
    pub final_score: f32,
    /// Raw product before compensation (optional diagnostics).
    pub category_score: f32,
}

/// Bundle of actions + contexts + strategy.
#[derive(Clone, Debug, PartialEq)]
pub struct UtilityActionSet {
    /// Actions.
    pub actions: Vec<UtilityAction>,
    /// Context definitions.
    pub contexts: Vec<ContextSet>,
    /// Winner selection strategy.
    pub selection_strategy: SelectionStrategy,
}

/// Minimal per-agent utility state used by tests.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct UtilityAgentState {
    /// Active context id.
    pub active_context: u32,
    /// Cached scores from last evaluation.
    pub action_scores: Vec<ScoredAction>,
}

/// Reads a normalized sample input for built-in axes.
pub fn evaluate_builtins(axis: InputAxis, custom_table: &[f32; 6]) -> f32 {
    let v = match axis {
        InputAxis::HealthRatio => custom_table[0],
        InputAxis::DistanceToTarget => custom_table[1],
        InputAxis::ThreatLevel => custom_table[2],
        InputAxis::TimeSinceAction => custom_table[3],
        InputAxis::AmmoCount => custom_table[4],
        InputAxis::LineOfSight => custom_table[5],
        InputAxis::Custom(i) => {
            if (i as usize) < custom_table.len() {
                custom_table[i as usize]
            } else {
                0.0
            }
        }
    };
    clamp01(v)
}

/// Trait for data-driven custom considerations (no `dyn` in hot path; monomorphize at call).
pub trait CustomConsideration {
    /// Returns a raw score in roughly `[0,1]` before global clamping.
    fn score(&self) -> f32;
}

/// Invokes a custom consideration and clamps.
pub fn score_custom<C: CustomConsideration>(c: &C) -> f32 {
    clamp01(c.score())
}

/// Multiplies curved inputs and applies compensation (`product^(1/n)`).
pub fn score_action_with_inputs(
    action: &UtilityAction,
    considerations: &[Consideration],
    axis_values: impl Fn(InputAxis) -> f32,
) -> f32 {
    let count = action.considerations.len();
    if count == 0 {
        return 0.0;
    }
    let mut product = 1.0f32;
    for &idx in &action.considerations {
        let c = considerations[idx];
        let input = axis_values(c.input_axis);
        product *= c.curve.evaluate(input);
    }
    compensate_product(product, count)
}

/// Raises `product` to `1 / n` (compensation).
pub fn compensate_product(product: f32, n: usize) -> f32 {
    if n == 0 {
        return 0.0;
    }
    product.powf(1.0 / n as f32)
}

/// Highest-score index; ties pick first max.
pub fn select_highest(scores: &[f32]) -> Option<usize> {
    let mut best_i = None;
    let mut best_v = f32::NEG_INFINITY;
    for (i, &s) in scores.iter().enumerate() {
        if s > best_v {
            best_v = s;
            best_i = Some(i);
        }
    }
    best_i
}

/// Weighted random index using scores as weights.
pub fn weighted_random_index(scores: &[f32], rng: &mut SmallRng) -> usize {
    let sum: f32 = scores.iter().sum();
    if sum <= 0.0 {
        return 0;
    }
    let pick = rng.gen::<f32>() * sum;
    let mut acc = 0.0f32;
    for (i, &s) in scores.iter().enumerate() {
        acc += s;
        if pick <= acc {
            return i;
        }
    }
    scores.len().saturating_sub(1)
}

/// `(action_id, category_id, final_score)` plus `(category_id, category_priority)`.
#[must_use]
pub fn dual_axis_pick(actions: &[(u32, u32, f32)], cat_priority: &[(u32, u32)]) -> Option<u32> {
    if actions.is_empty() {
        return None;
    }
    let mut best_cat_prio = 0u32;
    for &(_, cid, _) in actions {
        let p = cat_priority
            .iter()
            .find_map(|(id, pr)| (*id == cid).then_some(*pr))
            .unwrap_or(0);
        best_cat_prio = best_cat_prio.max(p);
    }
    let mut best: Option<(f32, u32)> = None;
    for &(aid, cid, sc) in actions {
        let p = cat_priority
            .iter()
            .find_map(|(id, pr)| (*id == cid).then_some(*pr))
            .unwrap_or(0);
        if p != best_cat_prio {
            continue;
        }
        match best {
            None => best = Some((sc, aid)),
            Some((bs, _)) if sc > bs => best = Some((sc, aid)),
            _ => {}
        }
    }
    best.map(|(_, aid)| aid)
}

/// Hysteresis gate for context switches: `true` when `candidate` clearly beats `active_score`.
#[must_use]
pub fn hysteresis_should_switch(active_score: f32, candidate_score: f32, margin: f32) -> bool {
    candidate_score >= active_score + margin
}
