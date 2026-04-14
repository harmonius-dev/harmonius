//! Hold-last-velocity extrapolation with smooth correction decay.

/// Extrapolate with constant velocity `v` from `last_pos` for `dt_ms`.
pub fn extrapolate_position(last_pos: f32, vel: f32, dt_ms: f32) -> f32 {
    last_pos + vel * (dt_ms / 1000.0)
}

/// Exponential decay of error `err` over `steps` iterations with factor `alpha`.
pub fn smooth_correct(mut err: f32, alpha: f32, steps: usize) -> f32 {
    for _ in 0..steps {
        err *= alpha;
    }
    err
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-8.4.4.1 — extrapolation then decaying error toward snapshot.
    #[test]
    fn test_extrapolation_with_correction() {
        let v = 1.0f32;
        let mut pos = 10.0f32;
        for _ in 0..50 {
            pos = extrapolate_position(pos, v, 1.0);
        }
        assert!(pos > 10.0);
        let snap = 12.0f32;
        let mut err = snap - pos;
        err = smooth_correct(err, 0.9, 40);
        assert!(err.abs() < 0.5, "err={err}");
    }
}
