//! Helpers for mapping GPU counter ticks to milliseconds on a shared timeline.

/// Converts raw GPU timestamp ticks to milliseconds using `gpu_hz` ticks per millisecond.
///
/// This matches the integration test contract `ms == ticks / gpu_hz` where `gpu_hz` is expressed
/// as **ticks per millisecond** (not Hertz of the GPU core clock).
#[must_use]
pub fn gpu_ticks_to_ms(ticks: u64, gpu_hz: f64) -> f64 {
    (ticks as f64) / gpu_hz
}

#[cfg(test)]
mod tests {
    use super::gpu_ticks_to_ms;

    #[test]
    fn tc_ir_5_7_4_u1_timebase_calibration() {
        let ticks = 2_000_000_u64;
        let gpu_hz = 1_000_000.0;
        let ms = gpu_ticks_to_ms(ticks, gpu_hz);
        assert!((ms - (ticks as f64 / gpu_hz)).abs() < f64::EPSILON);
        assert!((ms - 2.0).abs() < f64::EPSILON);
    }
}
