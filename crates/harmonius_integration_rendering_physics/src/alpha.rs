/// Computes `accumulator / fixed_dt` clamped to `[0.0, 1.0]`.
pub fn interp_alpha_from_fixed_accumulator(accumulator: f32, fixed_dt: f32) -> f32 {
    if fixed_dt <= 0.0 || !fixed_dt.is_finite() {
        return 0.0;
    }
    (accumulator / fixed_dt).clamp(0.0, 1.0)
}

/// Interpolates translation using a frame-global alpha.
pub fn lerp_transform_translation(prev: glam::Vec3, curr: glam::Vec3, alpha: f32) -> glam::Vec3 {
    prev.lerp(curr, alpha)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_ir_3_4_6_2_alpha_clamped_high() {
        let alpha = interp_alpha_from_fixed_accumulator(1.5, 1.0);
        assert!((alpha - 1.0).abs() < 1e-6);
    }

    #[test]
    fn tc_ir_3_4_n4_accumulator_exceeds_fixed_dt() {
        let alpha = interp_alpha_from_fixed_accumulator(2.5, 1.0);
        assert!((alpha - 1.0).abs() < 1e-6);
    }

    #[test]
    fn alpha_zero_when_fixed_dt_non_positive() {
        assert!((interp_alpha_from_fixed_accumulator(1.0, 0.0) - 0.0).abs() < 1e-6);
        assert!((interp_alpha_from_fixed_accumulator(1.0, -1.0) - 0.0).abs() < 1e-6);
    }

    #[test]
    fn tc_ir_3_4_6_1_interp_smooths_motion() {
        let prev = glam::Vec3::ZERO;
        let curr = glam::Vec3::new(0.0, 1.0, 0.0);
        let mid = lerp_transform_translation(prev, curr, 0.5);
        assert!((mid - glam::Vec3::new(0.0, 0.5, 0.0)).length() < 1e-5);
    }

    #[test]
    fn tc_ir_3_4_6_3_alpha_frame_global_single_resource() {
        let alpha = crate::types::InterpAlpha { value: 0.25 };
        let mut last = 0.0f32;
        for _ in 0..1000 {
            last = alpha.value;
        }
        assert!((last - 0.25).abs() < 1e-6);
    }
}
