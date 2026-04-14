//! Shared wind field sampling for strand tips.

use crate::math::Vec3;

/// Samples wind at root (m/s) and returns tip displacement bias for a light strand.
pub fn tip_displacement_from_wind(wind_m_s: Vec3, strength: f32) -> Vec3 {
    wind_m_s * (strength * 0.02)
}

/// Turbulent wind sample.
pub fn turbulent_wind(t_s: f32, base: Vec3) -> Vec3 {
    base + Vec3::new((t_s * 3.0).sin() * 0.5, 0.0, 0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_9_5_6_1_wind() {
        let w = Vec3::new(2.0, 0.0, 0.0);
        let d1 = tip_displacement_from_wind(w, 1.0);
        let d2 = tip_displacement_from_wind(w * 2.0, 1.0);
        assert!(d2.x > d1.x);
        assert!(d1.x > 0.0);

        let mut prev = 0.0_f32;
        let mut ok = true;
        for f in 0..120 {
            let t = f as f32 / 60.0;
            let tw = turbulent_wind(t, w);
            ok = ok && tw.x.is_finite();
            prev = tw.x;
        }
        assert!(ok);
        let _ = prev;
    }
}
