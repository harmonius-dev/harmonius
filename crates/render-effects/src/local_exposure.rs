//! Per-tile local exposure helpers (TC-2.9.8.1, TC-2.9.11.1).

/// Mean luminance for tile `(tx, ty)` in a `width`×`height` row-major `luminance` field.
pub fn tile_mean_luminance(
    width: usize,
    height: usize,
    tile_count: usize,
    tx: usize,
    ty: usize,
    luminance: &[f32],
) -> f32 {
    let tw = width / tile_count;
    let th = height / tile_count;
    let x0 = tx * tw;
    let y0 = ty * th;
    let mut sum = 0.0_f32;
    let mut n = 0u32;
    for y in y0..(y0 + th).min(height) {
        for x in x0..(x0 + tw).min(width) {
            sum += luminance[y * width + x];
            n += 1;
        }
    }
    if n == 0 {
        0.0
    } else {
        sum / n as f32
    }
}

/// EV bias from mean luminance: bright scenes get negative bias (TC-2.9.11.1).
pub fn tile_ev_bias(mean_lum: f32) -> f32 {
    let log_l = mean_lum.max(1e-4).log2();
    let ref_l = 0.18_f32.log2();
    (ref_l - log_l).clamp(-2.0, 2.0)
}
