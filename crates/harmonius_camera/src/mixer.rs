//! Weighted multi-camera mixing.

use glam::Vec3;

/// Mixes camera positions using normalized weights.
#[must_use]
pub fn mix_camera_positions(inputs: &[(Vec3, f32)]) -> Option<Vec3> {
    let mut sum = Vec3::ZERO;
    let mut weight = 0.0;
    for (pos, w) in inputs {
        if *w <= 0.0 {
            continue;
        }
        sum += *pos * *w;
        weight += *w;
    }
    if weight <= 0.0 {
        None
    } else {
        Some(sum / weight)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-13.25.17.1 — weighted average of three cameras.
    #[test]
    fn tc_13_25_17_1_mixer_weighted_average() {
        let mix = mix_camera_positions(&[
            (Vec3::new(0.0, 0.0, 0.0), 1.0),
            (Vec3::new(2.0, 0.0, 0.0), 2.0),
            (Vec3::new(4.0, 0.0, 0.0), 1.0),
        ])
        .expect("mixed");
        assert!((mix - Vec3::new(2.0, 0.0, 0.0)).length() < 1e-3);
    }

    /// TC-13.25.17.2 — zero weight cameras are ignored.
    #[test]
    fn tc_13_25_17_2_mixer_zero_weight() {
        let mix = mix_camera_positions(&[
            (Vec3::new(1.0, 0.0, 0.0), 0.0),
            (Vec3::new(3.0, 0.0, 0.0), 1.0),
        ])
        .expect("mixed");
        assert!((mix - Vec3::new(3.0, 0.0, 0.0)).length() < 1e-3);
    }
}
