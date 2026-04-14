//! Lens distortion and chromatic aberration helpers (`R-2.7.5`).

/// Radial lens distortion parameters in normalized device coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DistortionSettings {
    /// Primary radial coefficient.
    pub k1: f32,
    /// Secondary radial coefficient.
    pub k2: f32,
    /// Chromatic aberration strength in UV space.
    pub chromatic_aberration: f32,
}

impl DistortionSettings {
    /// Applies radial distortion to a **normalized** coordinate centered at `(0,0)`.
    #[must_use]
    pub fn apply_radial(self, uv: [f32; 2]) -> [f32; 2] {
        let r2 = uv[0] * uv[0] + uv[1] * uv[1];
        let factor = 1.0 + self.k1 * r2 + self.k2 * r2 * r2;
        [uv[0] * factor, uv[1] * factor]
    }

    /// Offsets red/blue channels relative to green for chromatic aberration.
    #[must_use]
    pub fn chromatic_offsets(self, uv: [f32; 2]) -> ([f32; 2], [f32; 2], [f32; 2]) {
        let base = self.apply_radial(uv);
        let dir = [uv[0].signum(), uv[1].signum()];
        let mag = self.chromatic_aberration * (uv[0].abs() + uv[1].abs()).max(1.0e-4);
        let r = [base[0] + dir[0] * mag, base[1] + dir[1] * mag];
        let b = [base[0] - dir[0] * mag, base[1] - dir[1] * mag];
        (r, base, b)
    }
}

#[cfg(test)]
mod tests {
    use super::DistortionSettings;

    /// TC-2.7.5.1 — zero coefficients are identity.
    #[test]
    fn test_distortion_zero_is_identity() {
        let d = DistortionSettings {
            k1: 0.0,
            k2: 0.0,
            chromatic_aberration: 0.0,
        };
        let uv = [0.25, -0.4];
        assert_eq!(d.apply_radial(uv), uv);
    }

    /// TC-2.7.5.2 — positive `k1` pushes outer pixels outward along the radial direction.
    #[test]
    fn test_distortion_barrel_outside_center() {
        let d = DistortionSettings {
            k1: 0.35,
            k2: 0.0,
            chromatic_aberration: 0.0,
        };
        let uv = [0.5, 0.0];
        let out = d.apply_radial(uv);
        assert!(out[0] > uv[0]);
    }

    /// TC-2.7.5.3 — red and blue channels diverge from green under chromatic aberration.
    #[test]
    fn test_chromatic_aberration_rgb_offset() {
        let d = DistortionSettings {
            k1: 0.0,
            k2: 0.0,
            chromatic_aberration: 0.01,
        };
        let uv = [0.4, 0.2];
        let (r, g, b) = d.chromatic_offsets(uv);
        assert_ne!(r, g);
        assert_ne!(b, g);
    }
}
