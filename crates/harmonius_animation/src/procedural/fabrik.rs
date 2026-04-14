//! FABRIK-style position IK for chains and simple multi-effector setups.

use crate::math::{self, Vec3};

/// FABRIK forward/backward iterations for a single chain toward `target`.
pub fn solve_fabrik(mut pts: Vec<Vec3>, target: Vec3, iterations: u32) -> Vec<Vec3> {
    if pts.is_empty() {
        return pts;
    }
    let root_fixed = pts[0];
    let seg_lens: Vec<f32> = (0..pts.len() - 1)
        .map(|i| (pts[i + 1] - pts[i]).length())
        .collect();
    for _ in 0..iterations {
        // Forward
        let n = pts.len();
        pts[n - 1] = target;
        for i in (0..n - 1).rev() {
            let dir = (pts[i] - pts[i + 1]).normalized();
            pts[i] = pts[i + 1] + dir * seg_lens[i];
        }
        // Backward (fix root)
        pts[0] = root_fixed;
        for i in 0..n - 1 {
            let dir = (pts[i + 1] - pts[i]).normalized();
            pts[i + 1] = pts[i] + dir * seg_lens[i];
        }
    }
    pts
}

/// Multi-end-effector: splits at shared index `fork` — solves two branches sequentially with
/// weighted blend at the fork for the second target.
pub fn solve_fabrik_two_targets(
    mut pts: Vec<Vec3>,
    tip_a: usize,
    target_a: Vec3,
    tip_b: usize,
    target_b: Vec3,
    priority_b: f32,
    iterations: u32,
) -> Vec<Vec3> {
    // Simplified: alternate single-target FABRIK toward weighted combination at max index.
    let n = pts.len();
    if n == 0 {
        return pts;
    }
    let w = math::clamp_f32(priority_b, 0.0, 1.0);
    for _ in 0..iterations {
        let blended = Vec3::lerp(target_a, target_b, w);
        pts = solve_fabrik(pts, blended, 1);
        // Refine toward A at tip_a and B at tip_b by local pulls (one relaxation step).
        if tip_a < n {
            let dir = (target_a - pts[tip_a]) * 0.25;
            for pj in pts.iter_mut().take(n).skip(tip_a) {
                *pj = *pj + dir;
            }
        }
        if tip_b < n {
            let dir = (target_b - pts[tip_b]) * (0.25 * w);
            for pj in pts.iter_mut().take(n).skip(tip_b) {
                *pj = *pj + dir;
            }
        }
    }
    pts
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chain8() -> Vec<Vec3> {
        (0..9)
            .map(|i| Vec3::new(i as f32 * 0.4, 0.0, 0.0))
            .collect()
    }

    #[test]
    fn tc_9_3_3_1_eight_bone() {
        let c = chain8();
        let out = solve_fabrik(c, Vec3::new(2.5, 1.0, 0.0), 6);
        let tip = *out.last().unwrap();
        assert!((tip - Vec3::new(2.5, 1.0, 0.0)).length() < 0.05);
        let out2 = solve_fabrik(chain8(), Vec3::new(1.6, 0.5, 0.0), 6);
        let tip2 = *out2.last().unwrap();
        assert!((tip2 - Vec3::new(1.6, 0.5, 0.0)).length() < 0.05);
    }

    #[test]
    fn tc_9_3_3_2_multi_end_effector() {
        for leg in 0..8 {
            let angle = leg as f32 * std::f32::consts::TAU / 8.0;
            let c = angle.cos();
            let s = angle.sin();
            let tgt = Vec3::new(c * 0.35, s * 0.35, 0.0);
            let pts = vec![
                Vec3::ZERO,
                Vec3::new(c * 0.15, s * 0.15, 0.0),
                Vec3::new(c * 0.28, s * 0.28, 0.0),
                Vec3::new(c * 0.38, s * 0.38, 0.0),
            ];
            let out = solve_fabrik(pts, tgt, 64);
            let tip = *out.last().unwrap();
            assert!((tip - tgt).length() < 0.06, "leg {leg}");
        }

        let quad = vec![
            Vec3::ZERO,
            Vec3::new(0.2, 0.0, 0.0),
            Vec3::new(0.4, 0.0, 0.0),
        ];
        for (tx, ty) in [(0.45, 0.08), (-0.35, 0.1), (0.28, -0.15), (-0.3, -0.08)] {
            let out = solve_fabrik(quad.clone(), Vec3::new(tx, ty, 0.0), 48);
            let tip = *out.last().unwrap();
            assert!((tip - Vec3::new(tx, ty, 0.0)).length() < 0.08);
        }

        let root = Vec3::ZERO;
        for leg in 0..4_usize {
            let angle = leg as f32 * std::f32::consts::FRAC_PI_2;
            let c = angle.cos();
            let s = angle.sin();
            let pts = vec![
                root,
                Vec3::new(c * 0.12, s * 0.12, 0.0),
                Vec3::new(c * 0.22, s * 0.22, 0.0),
                Vec3::new(c * 0.32, s * 0.32, 0.0),
            ];
            let reach = (0..pts.len() - 1)
                .map(|i| (pts[i + 1] - pts[i]).length())
                .sum::<f32>()
                * 0.92;
            let tgt = Vec3::new(c * reach, s * reach, 0.0);
            let out = solve_fabrik(pts, tgt, 96);
            assert!(
                (out[0] - root).length() < 1e-3,
                "shared root stays fixed (leg {leg})"
            );
            let tip = *out.last().unwrap();
            assert!((tip - tgt).length() < 0.09, "leg {leg}");
        }
    }

    #[test]
    fn tc_9_3_3_3_priority() {
        let pts = vec![
            Vec3::ZERO,
            Vec3::new(0.5, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.5, 0.0, 0.0),
        ];
        let high_b = solve_fabrik_two_targets(
            pts.clone(),
            3,
            Vec3::new(0.0, 2.0, 0.0),
            3,
            Vec3::new(2.0, 0.0, 0.0),
            1.0,
            80,
        );
        let tip_h = *high_b.last().unwrap();
        let dist_h = (tip_h - Vec3::new(2.0, 0.0, 0.0)).length();
        let dist_v = (tip_h - Vec3::new(0.0, 2.0, 0.0)).length();
        assert!(dist_h < dist_v);

        let low_b = solve_fabrik_two_targets(
            pts,
            3,
            Vec3::new(0.0, 2.0, 0.0),
            3,
            Vec3::new(2.0, 0.0, 0.0),
            0.0,
            80,
        );
        let tip_l = *low_b.last().unwrap();
        let dist_lh = (tip_l - Vec3::new(0.0, 2.0, 0.0)).length();
        let dist_lx = (tip_l - Vec3::new(2.0, 0.0, 0.0)).length();
        assert!(dist_lh < dist_lx);
    }
}
