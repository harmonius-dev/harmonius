//! Camera input integration helpers.

/// Integrates angular velocity using real elapsed time for frame-rate independence.
#[must_use]
pub fn integrate_camera_input_frame_independent(rate_deg_per_sec: f32, dt: f32) -> f32 {
    rate_deg_per_sec * dt
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-13.25.37.1 — same angular rate yields identical travel across frame rates.
    #[test]
    fn tc_13_25_37_1_input_frame_independence() {
        let rate = 90.0;
        let low = integrate_camera_input_frame_independent(rate, 1.0 / 30.0);
        let high = integrate_camera_input_frame_independent(rate, 1.0 / 120.0);
        let low_total = low * 30.0;
        let high_total = high * 120.0;
        assert!((low_total - high_total).abs() < 1e-3);
    }
}
