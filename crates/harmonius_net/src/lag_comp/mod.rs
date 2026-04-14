//! Lag compensation (hitbox rewind window).

/// Computes rewind duration capped at `max_ms` from RTT (half RTT model).
pub fn rewind_ms_for_rtt(rtt_ms: f32, max_ms: f32) -> f32 {
    (rtt_ms * 0.5).min(max_ms)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-8.4.5.1 — half-RTT rewind, never exceeding the compensation cap.
    #[test]
    fn test_lag_compensation_rewind() {
        assert!((rewind_ms_for_rtt(100.0, 250.0) - 50.0).abs() < 1.0);
        let r = rewind_ms_for_rtt(300.0, 250.0);
        assert!(r <= 250.0 && (r - 150.0).abs() < 1.0);
    }
}
