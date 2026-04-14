//! Motion blur sampling from screen-space velocity.

/// Motion blur tuning parameters.
#[derive(Clone, Copy, Debug)]
pub struct MotionBlurParams {
    /// Number of temporal samples along the velocity vector.
    pub sample_count: u8,
}

/// Normalized blur direction and sample positions along it.
#[derive(Clone, Debug, PartialEq)]
pub struct MotionBlurSamplePlan {
    /// Unit direction of blur in screen space.
    pub direction: (f32, f32),
    /// Offsets along `direction` from the center pixel, in normalized velocity units.
    pub offsets: Vec<f32>,
}

/// Builds a motion blur sample plan from a velocity vector (TC-2.9.3.1).
pub fn motion_blur_sample_plan(velocity: (f32, f32), params: &MotionBlurParams) -> MotionBlurSamplePlan {
    let len = (velocity.0 * velocity.0 + velocity.1 * velocity.1).sqrt().max(1e-6);
    let direction = (velocity.0 / len, velocity.1 / len);
    let n = params.sample_count as usize;
    let mut offsets = Vec::with_capacity(n);
    for i in 0..n {
        let t = (i as f32 + 0.5) / n as f32 - 0.5;
        offsets.push(t * len);
    }
    MotionBlurSamplePlan { direction, offsets }
}
