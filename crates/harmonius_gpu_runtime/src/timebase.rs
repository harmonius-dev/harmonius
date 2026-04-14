//! Helpers for mapping GPU counter ticks to milliseconds on a shared timeline.

/// Converts raw GPU timestamp ticks to milliseconds using `gpu_ticks_per_ms`.
///
/// This matches the integration test contract `ms == ticks / gpu_ticks_per_ms` where the divisor
/// is **GPU counter ticks per millisecond** (not Hertz of the GPU core clock).
#[must_use]
pub fn gpu_ticks_to_ms(ticks: u64, gpu_ticks_per_ms: f64) -> f64 {
    (ticks as f64) / gpu_ticks_per_ms
}

#[cfg(test)]
mod tests {
    use super::gpu_ticks_to_ms;

    #[test]
    fn tc_ir_5_7_4_u1_timebase_calibration() {
        let ticks = 2_000_000_u64;
        let gpu_ticks_per_ms = 1_000_000.0;
        let ms = gpu_ticks_to_ms(ticks, gpu_ticks_per_ms);
        assert!((ms - (ticks as f64 / gpu_ticks_per_ms)).abs() < f64::EPSILON);
        assert!((ms - 2.0).abs() < f64::EPSILON);
    }
}
