//! Analytical two-bone IK (law of cosines) with optional pole vector and blend weight.

use crate::math::{self, Vec3};

/// Rest-style joint positions: root, mid (elbow), tip (wrist).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TwoBonePose {
    /// Root joint position.
    pub root: Vec3,
    /// Mid joint position.
    pub mid: Vec3,
    /// Tip (end effector) position.
    pub tip: Vec3,
}

/// Solves two-bone IK toward `target`, optionally biased by `pole`.
///
/// `len_upper` / `len_lower` are bone lengths. `weight` blends between the input pose (`1` = full
/// IK, `0` = exact input pose).
pub fn solve_two_bone(
    input: TwoBonePose,
    target: Vec3,
    len_upper: f32,
    len_lower: f32,
    pole: Option<Vec3>,
    weight: f32,
) -> TwoBonePose {
    let w = math::clamp_f32(weight, 0.0, 1.0);
    if w == 0.0 {
        return input;
    }
    let full = solve_two_bone_inner(input.root, target, len_upper, len_lower, pole, input.mid);
    if w == 1.0 {
        return full;
    }
    TwoBonePose {
        root: input.root,
        mid: Vec3::lerp(input.mid, full.mid, w),
        tip: Vec3::lerp(input.tip, full.tip, w),
    }
}

fn solve_two_bone_inner(
    root: Vec3,
    goal: Vec3,
    len_upper: f32,
    len_lower: f32,
    pole: Option<Vec3>,
    mid_hint: Vec3,
) -> TwoBonePose {
    let mut v = goal - root;
    let mut dist = v.length();
    let sum = (len_upper + len_lower).max(1e-6);
    if dist < 1e-6 {
        dist = 1e-6;
        v = Vec3::new(1e-6, 0.0, 0.0);
    }
    dist = dist
        .min(sum - 1e-5)
        .max((len_upper - len_lower).abs() + 1e-5);
    let dir = v.normalized();

    let cos_a = math::clamp_f32(
        (len_upper * len_upper + dist * dist - len_lower * len_lower) / (2.0 * len_upper * dist),
        -1.0,
        1.0,
    );
    let sin_a = (1.0_f32 - cos_a * cos_a).max(0.0).sqrt();

    let pole_pt = pole.unwrap_or(mid_hint);
    let mut n = dir.cross(pole_pt - root);
    if n.length() < 1e-6 {
        n = dir.cross(Vec3::new(0.0, 1.0, 0.0));
    }
    if n.length() < 1e-6 {
        n = dir.cross(Vec3::new(0.0, 0.0, 1.0));
    }
    n = n.normalized();
    let perp = n.cross(dir).normalized();

    let upper_dir = (dir * cos_a + perp * sin_a).normalized();
    let mid = root + upper_dir * len_upper;
    let tip_dir = (goal - mid).normalized();
    let tip = mid + tip_dir * len_lower;
    TwoBonePose { root, mid, tip }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn straight_chain() -> TwoBonePose {
        TwoBonePose {
            root: Vec3::ZERO,
            mid: Vec3::new(1.0, 0.0, 0.0),
            tip: Vec3::new(2.0, 0.0, 0.0),
        }
    }

    #[test]
    fn tc_9_3_1_1_reach_target() {
        let p = solve_two_bone(
            straight_chain(),
            Vec3::new(1.5, 0.0, 0.0),
            1.0,
            1.0,
            Some(Vec3::new(0.0, 0.0, 1.0)),
            1.0,
        );
        assert!((p.tip - Vec3::new(1.5, 0.0, 0.0)).length() < 0.01);
        let p2 = solve_two_bone(
            straight_chain(),
            Vec3::new(2.0, 0.0, 0.0),
            1.0,
            1.0,
            Some(Vec3::new(0.0, 0.0, 1.0)),
            1.0,
        );
        assert!((p2.tip - Vec3::new(2.0, 0.0, 0.0)).length() < 0.01);
    }

    #[test]
    fn tc_9_3_1_2_pole_vector() {
        let p_pos = solve_two_bone(
            straight_chain(),
            Vec3::new(1.5, 0.0, 0.0),
            1.0,
            1.0,
            Some(Vec3::new(0.0, 0.0, 1.0)),
            1.0,
        );
        let p_neg = solve_two_bone(
            straight_chain(),
            Vec3::new(1.5, 0.0, 0.0),
            1.0,
            1.0,
            Some(Vec3::new(0.0, 0.0, -1.0)),
            1.0,
        );
        assert!(p_pos.mid.z > 0.0);
        assert!(p_neg.mid.z < 0.0);
    }

    #[test]
    fn tc_9_3_1_3_unreachable() {
        let p_far = solve_two_bone(
            straight_chain(),
            Vec3::new(5.0, 0.0, 0.0),
            1.0,
            1.0,
            Some(Vec3::new(0.0, 0.0, 1.0)),
            1.0,
        );
        let reach = (p_far.tip - p_far.root).length();
        assert!((reach - 2.0).abs() < 0.02);
        assert!(p_far.mid.x.is_finite() && p_far.tip.x.is_finite());

        let p_close = solve_two_bone(
            straight_chain(),
            Vec3::new(0.01, 0.0, 0.0),
            1.0,
            1.0,
            Some(Vec3::new(0.0, 0.0, 1.0)),
            1.0,
        );
        assert!(p_close.mid.x.is_finite() && p_close.tip.x.is_finite());
    }

    #[test]
    fn tc_9_3_1_4_weight_zero_and_half() {
        let input = straight_chain();
        let out0 = solve_two_bone(input, Vec3::new(5.0, 5.0, 5.0), 1.0, 1.0, None, 0.0);
        assert_eq!(out0, input);
        let full = solve_two_bone(input, Vec3::new(1.5, 0.0, 0.0), 1.0, 1.0, None, 1.0);
        let half = solve_two_bone(input, Vec3::new(1.5, 0.0, 0.0), 1.0, 1.0, None, 0.5);
        let expect_mid = Vec3::lerp(input.mid, full.mid, 0.5);
        let expect_tip = Vec3::lerp(input.tip, full.tip, 0.5);
        assert!(math::approx_eq_vec3(half.mid, expect_mid, 1e-5));
        assert!(math::approx_eq_vec3(half.tip, expect_tip, 1e-5));
    }
}
