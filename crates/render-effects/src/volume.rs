//! Post-process volume blending (TC-2.9.12.1).

/// Numeric parameter contributed by a volume layer.
#[derive(Clone, Copy, Debug)]
pub struct VolumeLayer {
    /// Higher wins when overlapping.
    pub priority: i32,
    /// Parameter value in `[0, 1]`.
    pub bloom: f32,
}

/// Blends `bloom` by strongly priority-weighted average (deterministic).
pub fn blend_bloom_by_priority(layers: &[VolumeLayer]) -> f32 {
    let mut num = 0.0_f32;
    let mut den = 0.0_f32;
    for l in layers {
        let w = 10_f32.powi(l.priority);
        num += l.bloom * w;
        den += w;
    }
    if den <= 0.0 {
        0.0
    } else {
        num / den
    }
}
