//! Radial vignette (TC-2.9.6.3, TC-2.9.9.1).

/// Vignette response curve kind.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VignetteFalloff {
    /// Polynomial power on radial distance.
    Power(f32),
    /// Legacy linear-ish radial scale (unused in TC-2.9.9.1).
    Radial(f32),
}

/// Vignette parameters for image-space evaluation.
#[derive(Clone, Copy, Debug)]
pub struct VignetteParams {
    /// Overall vignette strength in `[0, 1]`.
    pub strength: f32,
    /// Falloff shaping.
    pub falloff: VignetteFalloff,
}

fn radial_t(uv: (f32, f32)) -> f32 {
    let dx = uv.0 - 0.5;
    let dy = uv.1 - 0.5;
    (4.0 * (dx * dx + dy * dy)).sqrt().min(1.0)
}

/// Evaluates vignette multiplier at normalized `uv` in `[0, 1]^2`.
pub fn vignette_at(uv: (f32, f32), p: &VignetteParams) -> f32 {
    let t = radial_t(uv);
    match p.falloff {
        VignetteFalloff::Power(exp) => {
            let one_minus = (1.0 - t).max(0.0);
            let shaped = one_minus.powf(exp);
            (1.0 - p.strength * (1.0 - shaped)).clamp(0.0, 1.0)
        }
        VignetteFalloff::Radial(k) => (1.0 - p.strength * (t * k).min(1.0)).clamp(0.0, 1.0),
    }
}
