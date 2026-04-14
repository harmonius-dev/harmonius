//! Exposure modes and adaptation (`R-2.7.7`).

/// Histogram sample used by automatic exposure modes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HistogramSample {
    /// Mean scene luminance in linear space, normalized so mid-gray is ~0.18.
    pub mean_linear: f32,
}

/// Automatic exposure strategy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExposureMode {
    /// Fixed EV100 output.
    Manual,
    /// Histogram mean chases a mid-gray target.
    AutomaticHistogram,
}

/// Exposure configuration carried by a render camera.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExposureSettings {
    /// Active exposure mode.
    pub mode: ExposureMode,
    /// Manual EV100 when `mode == Manual`.
    pub manual_ev100: f32,
    /// EV bias applied after automatic adaptation.
    pub ev_bias: f32,
    /// Adaptation speed in EV per second toward the target.
    pub adaptation_speed: f32,
    /// Minimum allowed EV100 after clamping.
    pub min_ev: f32,
    /// Maximum allowed EV100 after clamping.
    pub max_ev: f32,
}

impl ExposureSettings {
    /// Returns the EV100 for manual mode (constant across frames).
    #[must_use]
    pub fn manual_output(self) -> f32 {
        (self.manual_ev100 + self.ev_bias).clamp(self.min_ev, self.max_ev)
    }

    /// Advances automatic histogram exposure by `dt_seconds`.
    #[must_use]
    pub fn adapt_histogram(self, current_ev: f32, sample: HistogramSample, dt_seconds: f32) -> f32 {
        const MID_GRAY: f32 = 0.18;
        let target_ev = sample.mean_linear.max(1.0e-6).log2() - MID_GRAY.max(1.0e-6).log2();
        let delta = (target_ev - current_ev) * (self.adaptation_speed * dt_seconds).min(1.0);
        (current_ev + delta + self.ev_bias).clamp(self.min_ev, self.max_ev)
    }
}

#[cfg(test)]
mod tests {
    use super::{ExposureMode, ExposureSettings, HistogramSample};

    /// TC-2.7.7.1 — manual mode is constant for fixed settings.
    #[test]
    fn test_exposure_manual_constant() {
        let s = ExposureSettings {
            mode: ExposureMode::Manual,
            manual_ev100: 5.5,
            ev_bias: 0.25,
            adaptation_speed: 2.0,
            min_ev: 0.0,
            max_ev: 20.0,
        };
        assert!((s.manual_output() - 5.75).abs() < 1.0e-4);
    }

    /// TC-2.7.7.2 — mid-gray histogram converges toward the neutral target.
    #[test]
    fn test_exposure_auto_histogram_midgray() {
        let s = ExposureSettings {
            mode: ExposureMode::AutomaticHistogram,
            manual_ev100: 0.0,
            ev_bias: 0.0,
            adaptation_speed: 5.0,
            min_ev: -10.0,
            max_ev: 10.0,
        };
        let sample = HistogramSample { mean_linear: 0.18 };
        let next = s.adapt_histogram(0.0, sample, 1.0 / 60.0);
        assert!(
            next.abs() < 0.05,
            "expected near zero EV bias at mid-gray, got {next}"
        );
    }

    /// TC-2.7.7.3 — output never leaves `[min_ev, max_ev]`.
    #[test]
    fn test_exposure_clamped_to_range() {
        let s = ExposureSettings {
            mode: ExposureMode::AutomaticHistogram,
            manual_ev100: 0.0,
            ev_bias: 100.0,
            adaptation_speed: 50.0,
            min_ev: 1.0,
            max_ev: 3.0,
        };
        let sample = HistogramSample { mean_linear: 10.0 };
        let next = s.adapt_histogram(0.0, sample, 1.0);
        assert!((next - 3.0).abs() < 1.0e-3);
    }

    /// TC-2.7.7.4 — adaptation scales with `adaptation_speed` and `dt`.
    #[test]
    fn test_exposure_adaptation_rate() {
        let s = ExposureSettings {
            mode: ExposureMode::AutomaticHistogram,
            manual_ev100: 0.0,
            ev_bias: 0.0,
            adaptation_speed: 1.0,
            min_ev: -20.0,
            max_ev: 20.0,
        };
        let sample = HistogramSample { mean_linear: 0.36 };
        let start = 0.0_f32;
        let next = s.adapt_histogram(start, sample, 1.0 / 60.0);
        let target = sample.mean_linear.log2() - 0.18_f32.log2();
        let expected_delta = (target - start) * (s.adaptation_speed * (1.0 / 60.0)).min(1.0);
        assert!((next - (start + expected_delta)).abs() < 0.05);
    }
}
