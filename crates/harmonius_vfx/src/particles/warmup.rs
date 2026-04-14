//! Warm-up spawn estimation (`TC-11.1.1.9`).

/// Estimates steady alive count after `duration_s` at constant `spawn_per_second` (no death).
#[must_use]
pub fn warmup_estimated_alive(duration_s: f32, spawn_per_second: f32) -> u32 {
    (duration_s * spawn_per_second).round() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `TC-11.1.1.9` — two seconds at 100/s ≈ 200 particles if none die.
    #[test]
    fn tc_11_1_1_9_warm_up_particle_count() {
        let n = warmup_estimated_alive(2.0, 100.0);
        assert!((n as i32 - 200).abs() <= 1);
    }
}
