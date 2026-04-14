//! Fixed timestep accumulator used by simulation-class phases.

use std::time::Duration;

/// Manages fixed-timestep simulation with an accumulator.
#[derive(Debug)]
pub struct FixedTimestep {
    /// Duration of each tick (for example `1/60` second).
    pub tick_duration: Duration,
    /// Maximum ticks per frame to avoid a spiral of death.
    pub max_ticks_per_frame: u32,
    accumulator: Duration,
    /// Total fixed simulation time advanced by [`Self::consume`].
    pub total_simulated: Duration,
}

impl FixedTimestep {
    /// Creates a timestep configuration with zero accumulated time.
    #[must_use]
    pub fn new(tick_duration: Duration, max_ticks_per_frame: u32) -> Self {
        Self {
            tick_duration,
            max_ticks_per_frame,
            accumulator: Duration::ZERO,
            total_simulated: Duration::ZERO,
        }
    }

    /// Adds elapsed wall time since the last call.
    pub fn accumulate(&mut self, dt: Duration) {
        self.accumulator = self.accumulator.saturating_add(dt);
    }

    /// Returns how many ticks to run this frame and drains the accumulator.
    pub fn consume(&mut self) -> u32 {
        if self.tick_duration.is_zero() {
            return 0;
        }

        let mut ticks = 0u32;
        while ticks < self.max_ticks_per_frame && self.accumulator >= self.tick_duration {
            self.accumulator = self.accumulator.saturating_sub(self.tick_duration);
            ticks = ticks.saturating_add(1);
        }

        let advanced = self.tick_duration.saturating_mul(ticks);
        self.total_simulated = self.total_simulated.saturating_add(advanced);
        ticks
    }

    /// Interpolation factor in `[0.0, 1.0)` between the last completed tick and the next.
    #[must_use]
    pub fn alpha(&self) -> f32 {
        if self.tick_duration.is_zero() {
            return 0.0;
        }

        let numer = self.accumulator.as_secs_f64();
        let denom = self.tick_duration.as_secs_f64();
        if denom <= f64::EPSILON {
            return 0.0;
        }

        let a = numer / denom;
        if a >= 1.0 {
            return 0.999_999_94;
        }

        a as f32
    }
}
