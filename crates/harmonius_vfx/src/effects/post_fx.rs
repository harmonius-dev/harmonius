//! Screen-space post helpers (chromatic aberration sampling model).

/// Returns radial RGB channel offsets in pixels for `uv` in \[0,1\] (**TC-11.3.4.1**).
pub fn chromatic_radial_offsets(uv: [f32; 2], intensity: f32, pixel_delta: f32) -> ([f32; 2], [f32; 2]) {
    let c = [0.5_f32, 0.5];
    let dx = uv[0] - c[0];
    let dy = uv[1] - c[1];
    let r = (dx * dx + dy * dy).sqrt();
    if r < 1e-4 {
        return ([0.0, 0.0], [0.0, 0.0]);
    }
    let nx = dx / r;
    let ny = dy / r;
    let s = intensity * pixel_delta;
    let red = [nx * s, ny * s];
    let blue = [-nx * s, -ny * s];
    (red, blue)
}

#[cfg(test)]
mod tests {
    use super::chromatic_radial_offsets;

    /// **TC-11.3.4.1** — radial offsets at edge vs center.
    #[test]
    fn tc_11_3_4_1_chromatic_aberration_radial_offset() {
        let (r, b) = chromatic_radial_offsets([1.0, 0.5], 1.0, 4.0);
        assert!(r[0].abs() > 0.1);
        assert!(b[0].abs() > 0.1);

        let (r2, b2) = chromatic_radial_offsets([0.5, 0.5], 1.0, 4.0);
        assert_eq!(r2, [0.0, 0.0]);
        assert_eq!(b2, [0.0, 0.0]);
    }
}
