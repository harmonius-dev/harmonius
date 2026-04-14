//! Radial chromatic aberration sample offsets (TC-2.9.6.2).

/// Returns horizontal RGB channel sample offsets in UV space for `uv`.
pub fn chromatic_offsets_x(strength: f32, uv: (f32, f32)) -> (f32, f32, f32) {
    let edge = ((uv.0 - 0.5).abs() * 2.0).clamp(0.0, 1.0);
    let s = strength * edge;
    (s * 1.0, s * 0.55, s * 0.25)
}
