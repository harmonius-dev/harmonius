//! Weighted and priority steering force blending.

use glam::Vec3;

/// How multiple steering forces are combined.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlendMode {
    /// Weighted sum of all behavior forces.
    Weighted,
    /// Priority pipeline: lower numeric priority values run first and consume force budget.
    Priority,
}

/// One active steering behavior slot used by the blender.
#[derive(Clone, Debug, PartialEq)]
pub struct ActiveBehavior {
    /// Linear blend weight.
    pub weight: f32,
    /// Lower values are higher priority when using [`BlendMode::Priority`].
    pub priority: u8,
    /// When false, this slot is ignored.
    pub enabled: bool,
}

fn truncate(v: Vec3, max_len: f32) -> Vec3 {
    let len = v.length();
    if len > max_len && len > f32::EPSILON {
        v * (max_len / len)
    } else {
        v
    }
}

/// Blend parallel steering forces using a weighted sum, then clamp to `max_force`.
pub fn blend_weighted(behaviors: &[ActiveBehavior], forces: &[Vec3], max_force: f32) -> Vec3 {
    let mut total = Vec3::ZERO;
    for (b, f) in behaviors.iter().zip(forces) {
        if b.enabled {
            total += *f * b.weight;
        }
    }
    truncate(total, max_force)
}

/// Blend using a priority pipeline: earlier priorities consume `max_force` magnitude first.
pub fn blend_priority(behaviors: &mut [(ActiveBehavior, Vec3)], max_force: f32) -> Vec3 {
    behaviors.sort_by_key(|(b, _)| b.priority);
    let mut total = Vec3::ZERO;
    let mut remaining = max_force;
    for (b, f) in behaviors.iter() {
        if !b.enabled || remaining <= 0.0 {
            continue;
        }
        let scaled = truncate(*f * b.weight, remaining);
        remaining -= scaled.length();
        total += scaled;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_7_2_4_1_weighted_blend_sum() {
        let bs = vec![
            ActiveBehavior {
                weight: 0.6,
                priority: 0,
                enabled: true,
            },
            ActiveBehavior {
                weight: 0.4,
                priority: 0,
                enabled: true,
            },
        ];
        let fs = vec![Vec3::X, Vec3::Y];
        let out = blend_weighted(&bs, &fs, 100.0);
        assert!((out - Vec3::new(0.6, 0.4, 0.0)).length() < 1e-5);
    }

    #[test]
    fn tc_7_2_4_1_weighted_cancellation() {
        let bs = vec![
            ActiveBehavior {
                weight: 0.5,
                priority: 0,
                enabled: true,
            },
            ActiveBehavior {
                weight: 0.5,
                priority: 0,
                enabled: true,
            },
        ];
        let fs = vec![Vec3::new(2.0, 0.0, 0.0), Vec3::new(-2.0, 0.0, 0.0)];
        let out = blend_weighted(&bs, &fs, 100.0);
        assert!(out.length_squared() < 1e-8);
    }

    #[test]
    fn tc_7_2_4_2_priority_avoidance_first() {
        let mut pairs = vec![
            (
                ActiveBehavior {
                    weight: 1.0,
                    priority: 0,
                    enabled: true,
                },
                Vec3::X,
            ),
            (
                ActiveBehavior {
                    weight: 1.0,
                    priority: 1,
                    enabled: true,
                },
                Vec3::Y,
            ),
        ];
        let out = blend_priority(&mut pairs, 1.0);
        assert!((out - Vec3::X).length() < 1e-5);
    }

    #[test]
    fn tc_7_2_4_3_priority_non_zero() {
        let mut pairs = vec![
            (
                ActiveBehavior {
                    weight: 1.0,
                    priority: 0,
                    enabled: true,
                },
                Vec3::X,
            ),
            (
                ActiveBehavior {
                    weight: 1.0,
                    priority: 1,
                    enabled: true,
                },
                -Vec3::X,
            ),
        ];
        let out = blend_priority(&mut pairs, 1.0);
        assert!(out.length() > 0.0);
    }
}
