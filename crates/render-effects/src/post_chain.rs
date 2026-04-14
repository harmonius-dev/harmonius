//! Simple post-process material chain (TC-2.9.7.1, TC-2.9.7.2).

/// Linear RGB in `[0, 1]`.
pub type Rgb = (f32, f32, f32);

/// Desaturate toward luminance with `amount` in `[0, 1]`.
pub fn desaturate(rgb: Rgb, amount: f32) -> Rgb {
    let l = 0.2126 * rgb.0 + 0.7152 * rgb.1 + 0.0722 * rgb.2;
    let t = amount.clamp(0.0, 1.0);
    (
        rgb.0 * (1.0 - t) + l * t,
        rgb.1 * (1.0 - t) + l * t,
        rgb.2 * (1.0 - t) + l * t,
    )
}

/// Negate color (HDR-safe clamp for tests).
pub fn negate(rgb: Rgb) -> Rgb {
    (1.0 - rgb.0, 1.0 - rgb.1, 1.0 - rgb.2)
}

/// Saturate channels toward extremes (piecewise for tests).
pub fn saturate(rgb: Rgb) -> Rgb {
    let f = |x: f32| if x > 0.5 { 0.5 + (x - 0.5) * 1.4 } else { x * 0.8 };
    (f(rgb.0).clamp(0.0, 1.0), f(rgb.1).clamp(0.0, 1.0), f(rgb.2).clamp(0.0, 1.0))
}
