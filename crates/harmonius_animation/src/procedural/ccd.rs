//! Cyclic Coordinate Descent IK for short chains.

use crate::math::{self, Vec3};

/// Joint rotation limits in radians (per step), applied as max angle between successive bone
/// directions.
#[derive(Clone, Copy, Debug)]
pub struct CcdJointLimit {
    /// Maximum swing angle in radians per joint.
    pub max_swing_rad: f32,
}

/// One CCD solve pass: `positions` has `n+1` points (root..tip). `target` is world goal.
/// Returns updated positions (same length).
pub fn solve_ccd(
    mut positions: Vec<Vec3>,
    target: Vec3,
    iterations: u32,
    limits: &[CcdJointLimit],
) -> Vec<Vec3> {
    let n = positions.len();
    if n < 2 {
        return positions;
    }
    for _ in 0..iterations {
        for i in (0..n - 1).rev() {
            let tip = *positions.last().unwrap();
            let cur = positions[i];
            let to_tip = tip - cur;
            let to_target = target - cur;
            if to_tip.length() < 1e-8 || to_target.length() < 1e-8 {
                continue;
            }
            let mut rot_axis = to_tip.cross(to_target);
            if rot_axis.length() < 1e-8 {
                continue;
            }
            rot_axis = rot_axis.normalized();
            let cos = math::clamp_f32(to_tip.normalized().dot(to_target.normalized()), -1.0, 1.0);
            let mut angle = cos.acos();
            if i < limits.len() {
                angle = angle.min(limits[i].max_swing_rad);
            }
            for pj in positions.iter_mut().take(n).skip(i + 1) {
                let v = *pj - cur;
                let rotated = rotate_around_axis(v, rot_axis, angle);
                *pj = cur + rotated;
            }
        }
    }
    positions
}

fn rotate_around_axis(v: Vec3, axis: Vec3, angle: f32) -> Vec3 {
    let c = angle.cos();
    let s = angle.sin();
    let t = 1.0 - c;
    let x = axis.x;
    let y = axis.y;
    let z = axis.z;
    let rx = (t * x * x + c) * v.x + (t * x * y - s * z) * v.y + (t * x * z + s * y) * v.z;
    let ry = (t * x * y + s * z) * v.x + (t * y * y + c) * v.y + (t * y * z - s * x) * v.z;
    let rz = (t * x * z - s * y) * v.x + (t * y * z + s * x) * v.y + (t * z * z + c) * v.z;
    Vec3::new(rx, ry, rz)
}

/// Maximum angle between consecutive segment directions after solve.
pub fn max_joint_angle(positions: &[Vec3]) -> f32 {
    if positions.len() < 3 {
        return 0.0;
    }
    let mut max_a = 0.0_f32;
    for i in 0..positions.len() - 2 {
        let a = (positions[i + 1] - positions[i]).normalized();
        let b = (positions[i + 2] - positions[i + 1]).normalized();
        let c = math::clamp_f32(a.dot(b), -1.0, 1.0);
        max_a = max_a.max(c.acos());
    }
    max_a
}

#[cfg(test)]
mod tests {
    use super::*;

    fn straight_chain_6() -> Vec<Vec3> {
        (0..7)
            .map(|i| Vec3::new(i as f32 * 0.5, 0.0, 0.0))
            .collect()
    }

    #[test]
    fn tc_9_3_2_1_convergence() {
        let chain = straight_chain_6();
        let lim = [CcdJointLimit {
            max_swing_rad: std::f32::consts::PI,
        }; 6];
        let out = solve_ccd(chain.clone(), Vec3::new(2.0, 1.5, 0.0), 10, &lim);
        let tip = *out.last().unwrap();
        assert!((tip - Vec3::new(2.0, 1.5, 0.0)).length() < 0.05);
        let out3 = solve_ccd(chain, Vec3::new(2.0, 1.5, 0.0), 3, &lim);
        let tip3 = *out3.last().unwrap();
        let tip0 = *straight_chain_6().last().unwrap();
        assert!(
            (tip3 - Vec3::new(2.0, 1.5, 0.0)).length() < (tip0 - Vec3::new(2.0, 1.5, 0.0)).length()
        );
    }

    #[test]
    fn tc_9_3_2_2_angular_limits() {
        let chain = straight_chain_6();
        let lim30 = vec![
            CcdJointLimit {
                max_swing_rad: 30_f32.to_radians(),
            };
            6
        ];
        let out = solve_ccd(chain, Vec3::new(2.0, 0.5, 0.0), 10, &lim30);
        let m = max_joint_angle(&out);
        assert!(m.is_finite() && m < std::f32::consts::PI);

        let chain2 = straight_chain_6();
        let lim5 = vec![
            CcdJointLimit {
                max_swing_rad: 5_f32.to_radians(),
            };
            6
        ];
        let out2 = solve_ccd(chain2, Vec3::new(2.0, 0.2, 0.0), 10, &lim5);
        let tip = *out2.last().unwrap();
        assert!((tip - Vec3::new(2.0, 0.2, 0.0)).length() < 0.25);
    }

    #[test]
    fn tc_9_3_2_3_unreachable() {
        let chain = straight_chain_6();
        let lim = vec![
            CcdJointLimit {
                max_swing_rad: 30_f32.to_radians(),
            };
            6
        ];
        let out = solve_ccd(chain, Vec3::new(50.0, 0.0, 0.0), 20, &lim);
        assert!(max_joint_angle(&out) <= 30_f32.to_radians() + 1e-2);
        assert!(out.iter().all(|p| p.x.is_finite()));
    }
}
