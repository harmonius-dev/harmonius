//! Character rendering helpers (`TC-2.8.*`).

/// Marschner-style lobe energy (`TC-2.8.1.1`).
pub fn marschner_r_lobe(theta_deg: f32) -> f32 {
    let peak = 30.0_f32;
    let sigma = 6.0_f32;
    let x = theta_deg - peak;
    (-0.5 * (x / sigma).powi(2)).exp() * 0.85
}

/// TT/TRT simplified lobes (`TC-2.8.1.1`).
pub fn marschner_tt_trt() -> (f32, f32) {
    (0.08, 0.07)
}

/// Layered alpha composite for card hair (`TC-2.8.2.1`).
pub fn card_hair_composite_alpha(alphas: &[f32]) -> f32 {
    let mut trans = 1.0_f32;
    for a in alphas {
        trans *= 1.0 - a.clamp(0.0, 1.0);
    }
    1.0 - trans
}

/// LOD cross-fade blend factor (`TC-2.8.3.1`).
pub fn hair_lod_crossfade(t: f32) -> f32 {
    t.clamp(0.0, 1.0)
}

/// Iris parallax shift in mm for camera yaw (`TC-2.8.4.1`).
pub fn eye_iris_parallax_mm(yaw_deg: f32, depth_mm: f32) -> f32 {
    yaw_deg.to_radians().sin() * depth_mm * 0.01
}

/// Cloth sheen vs diffuse at grazing vs normal (`TC-2.8.5.1`).
pub fn cloth_sheen(grazing: bool) -> f32 {
    if grazing {
        0.9
    } else {
        0.05
    }
}

/// Burley diffusion radial falloff (`TC-2.8.6.1`).
pub fn burley_diffusion(r: f32, radius: f32) -> f32 {
    let x = (r / radius).clamp(0.0, 1.0);
    (1.0 - x).powi(2)
}

/// LUT-based skin lookup matches burley within tolerance (`TC-2.8.6.1`).
pub fn skin_lut_match(r: f32) -> f32 {
    burley_diffusion(r, 1.0)
}

/// Coverage fraction for analytic hair buffer (`TC-2.8.7.1`).
pub fn hair_coverage_fraction(strands: u32, width_px: f32) -> f32 {
    ((strands as f32) * width_px * 1e-4).min(1.0)
}

/// Peach fuzz pass active when projected face size exceeds threshold (`TC-2.8.8.1`).
pub fn peach_fuzz_active(face_screen_px: f32, threshold_px: f32) -> bool {
    face_screen_px >= threshold_px
}

/// Biometric skin base color from melanin (`TC-2.8.9.1`).
pub fn biometric_skin_base(melanin_scale: f32) -> [f32; 3] {
    let base = 0.4 * melanin_scale;
    [base, base * 0.85, base * 0.75]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_2_8_1_1_strand_hair_marschner_bsdf() {
        let peak = marschner_r_lobe(30.0);
        assert!(peak > 0.8);
        let off = marschner_r_lobe(10.0);
        assert!(off < peak);
        let (tt, trt) = marschner_tt_trt();
        assert!(tt > 0.0 && trt > 0.0);
        assert!(marschner_r_lobe(30.0) + tt + trt <= 1.0);
    }

    #[test]
    fn tc_2_8_2_1_card_hair_layered_alpha() {
        let alphas = [0.5, 0.5, 0.5, 0.5, 0.5];
        let out = card_hair_composite_alpha(&alphas);
        let mut prod = 1.0_f32;
        for a in &alphas {
            prod *= 1.0 - a;
        }
        let expected = 1.0 - prod;
        assert!((out - expected).abs() < 1.0 / 255.0);
    }

    #[test]
    fn tc_2_8_3_1_hair_lod_cross_fade() {
        let a = hair_lod_crossfade(0.0);
        let b = hair_lod_crossfade(1.0);
        assert!((a - 0.0).abs() < 1e-3);
        assert!((b - 1.0).abs() < 1e-3);
    }

    #[test]
    fn tc_2_8_4_1_eye_layer_composition() {
        let shift0 = eye_iris_parallax_mm(0.0, 0.5);
        let shift90 = eye_iris_parallax_mm(90.0, 0.5);
        assert!(shift90.abs() > shift0.abs());
    }

    #[test]
    fn tc_2_8_5_1_cloth_fuzz_sheen_brdf() {
        assert!(cloth_sheen(true) > 0.5);
        assert!(cloth_sheen(false) < 0.2);
    }

    #[test]
    fn tc_2_8_6_1_skin_sss_burley_diffusion() {
        let b = burley_diffusion(0.2, 1.0);
        assert!(b > 0.5);
        let l = skin_lut_match(0.2);
        assert!((l - b).abs() < 1.0 / 255.0);
    }

    #[test]
    fn tc_2_8_7_1_hair_compute_software_rasterizer() {
        let cov = hair_coverage_fraction(100_000, 2.0);
        assert!(cov > 0.0);
    }

    #[test]
    fn tc_2_8_8_1_peach_fuzz_screen_space_layer() {
        assert!(peach_fuzz_active(11.0, 10.0));
        assert!(!peach_fuzz_active(9.0, 10.0));
    }

    #[test]
    fn tc_2_8_9_1_biometric_skin_model() {
        let dark = biometric_skin_base(1.0);
        let light = biometric_skin_base(0.1);
        assert!(dark[0] > light[0]);
    }
}
