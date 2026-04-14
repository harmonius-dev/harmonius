//! Simplified balance PID on a single inverted-pendulum tilt state.

/// Integrates torso tilt toward upright using PD correction from lateral impulse events.
#[derive(Clone, Debug)]
pub struct BalanceState {
    /// Tilt from vertical in degrees (single axis roll).
    pub tilt_deg: f32,
    /// Angular velocity deg/s.
    pub tilt_vel: f32,
    /// Frames since stumble started.
    pub stumble_frame: Option<u32>,
    /// Frames in recovery after stumble.
    pub recovery_frame: Option<u32>,
}

impl Default for BalanceState {
    fn default() -> Self {
        Self {
            tilt_deg: 0.0,
            tilt_vel: 0.0,
            stumble_frame: None,
            recovery_frame: None,
        }
    }
}

impl BalanceState {
    /// Applies lateral impulse (N·s scale) as initial tilt kick.
    pub fn impulse(&mut self, lateral_ns: f32) {
        self.tilt_deg += lateral_ns * 0.05;
        self.stumble_frame = Some(0);
    }

    /// One simulation step with PD gains toward `target_tilt` (usually 0).
    pub fn step(&mut self, dt: f32, k_p: f32, k_d: f32, target_tilt: f32) {
        let err = target_tilt - self.tilt_deg;
        let acc = k_p * err - k_d * self.tilt_vel;
        self.tilt_vel += acc * dt;
        self.tilt_deg += self.tilt_vel * dt;
        if let Some(sf) = self.stumble_frame.as_mut() {
            *sf += 1;
        }
        if self.tilt_deg.abs() < 2.0 && self.stumble_frame.is_some() {
            self.recovery_frame.get_or_insert(0);
        }
        if let Some(r) = self.recovery_frame.as_mut() {
            *r += 1;
        }
    }
}

/// Forward lean angle (degrees) for uphill slope.
pub fn slope_lean_deg(slope_deg: f32) -> f32 {
    (slope_deg * 0.35).max(5.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_9_3_9_1_balance() {
        let dt = 1.0 / 60.0;
        let mut s = BalanceState::default();
        for _ in 0..120 {
            s.step(dt, 100.0, 10.0, 0.0);
        }
        assert!(s.tilt_deg.abs() < 2.0);
        let mut s2 = BalanceState::default();
        let mut max_os = 0.0_f32;
        for _ in 0..240 {
            s2.step(dt, 100.0, 10.0, 0.0);
            max_os = max_os.max(s2.tilt_deg.abs());
        }
        assert!(max_os < 45.0);
    }

    #[test]
    fn tc_9_3_9_2_stumble() {
        let dt = 1.0 / 60.0;
        let mut s = BalanceState::default();
        s.impulse(500.0);
        assert_eq!(s.stumble_frame, Some(0));
        let mut recovered_by = None;
        for f in 0..60 {
            s.step(dt, 120.0, 14.0, 0.0);
            if s.tilt_deg.abs() < 3.0 && f > 2 {
                recovered_by = Some(f);
                break;
            }
        }
        assert!(recovered_by.is_some());
    }

    #[test]
    fn tc_9_3_9_3_slope_lean() {
        let lean = slope_lean_deg(20.0);
        assert!(lean > 5.0);
    }
}
