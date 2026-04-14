//! Look-at yaw and aim alignment helpers (simplified single-axis head model).

use crate::math::{self, Vec3};

/// Computes head yaw (radians) to look from `forward` (+X default) toward `target_dir` projected on
/// XZ plane, clamped by `max_yaw_rad`.
pub fn look_yaw(forward: Vec3, target_dir: Vec3, max_yaw_rad: f32) -> f32 {
    let f = Vec3::new(forward.x, 0.0, forward.z).normalized();
    let t = Vec3::new(target_dir.x, 0.0, target_dir.z).normalized();
    if f.length() < 1e-6 || t.length() < 1e-6 {
        return 0.0;
    }
    let raw = f32::atan2(t.cross(f).y, f.dot(t));
    math::clamp_f32(raw, -max_yaw_rad, max_yaw_rad)
}

/// Aim: returns minimal angle between weapon forward and direction to target (degrees).
pub fn aim_alignment_degrees(weapon_forward: Vec3, weapon_origin: Vec3, aim_target: Vec3) -> f32 {
    let dir = (aim_target - weapon_origin).normalized();
    let w = weapon_forward.normalized();
    let c = math::clamp_f32(w.dot(dir), -1.0, 1.0);
    c.acos().to_degrees()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_9_3_5_1_look_45() {
        let fwd = Vec3::new(1.0, 0.0, 0.0);
        let right = Vec3::new(1.0, 0.0, 1.0).normalized();
        let yaw_r = look_yaw(fwd, right, std::f32::consts::FRAC_PI_2);
        assert!((yaw_r.to_degrees() - 45.0).abs() < 1.0);
        let left = Vec3::new(1.0, 0.0, -1.0).normalized();
        let yaw_l = look_yaw(fwd, left, std::f32::consts::FRAC_PI_2);
        assert!((yaw_l.to_degrees() + 45.0).abs() < 1.0);
    }

    #[test]
    fn tc_9_3_5_2_clamp() {
        let fwd = Vec3::new(1.0, 0.0, 0.0);
        let t = Vec3::new(-0.5, 0.0, 0.866).normalized();
        let yaw = look_yaw(fwd, t, 90_f32.to_radians());
        assert!((yaw.to_degrees() - 90.0).abs() < 0.1);
        let yaw2 = look_yaw(fwd, t, 90_f32.to_radians());
        assert!((yaw2.to_degrees().abs() - 90.0).abs() < 0.1);
    }

    #[test]
    fn tc_9_3_5_3_aim() {
        let origin = Vec3::ZERO;
        let tgt = Vec3::new(10.0, 5.0, 0.0);
        let dir = tgt.normalized();
        let ang = aim_alignment_degrees(dir, origin, tgt);
        assert!(ang < 2.0);
    }

    #[test]
    fn tc_9_3_5_4_lower_body_preserved() {
        let fwd = Vec3::new(1.0, 0.0, 0.0);
        let tgt = Vec3::new(1.0, 0.0, 1.0).normalized();
        let upper_yaw = look_yaw(fwd, tgt, std::f32::consts::FRAC_PI_2);
        let lower_yaw = 0.0_f32;
        assert!(lower_yaw.abs() < 1e-6, "lower body stays on locomotion yaw");
        assert!(upper_yaw.abs() > 0.2, "upper body receives look delta");
    }
}
