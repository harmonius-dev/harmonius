//! Camera shake and screen overlay helpers.

/// Single shake contributor.
#[derive(Clone, Copy, Debug)]
pub struct ShakeSource {
    /// Base amplitude before decay.
    pub amplitude: f32,
    /// Exponential decay rate per second.
    pub decay_rate: f32,
    /// Seconds since this source began contributing.
    pub elapsed_secs: f32,
}

impl ShakeSource {
    /// Current amplitude after exponential decay.
    pub fn current_amplitude(&self) -> f32 {
        self.amplitude * (-self.decay_rate * self.elapsed_secs).exp()
    }
}

/// Sums shake sources with a hard amplitude cap (**TC-11.3.1.2**).
pub fn combine_shake_sources(sources: &[ShakeSource], max_amp: f32) -> f32 {
    let sum: f32 = sources.iter().map(|s| s.current_amplitude()).sum();
    sum.min(max_amp)
}

/// Applies accessibility attenuation (**TC-11.3.1.3**).
pub fn apply_reduced_motion(offset: [f32; 3], reduced_motion: bool, attenuation: f32) -> [f32; 3] {
    if reduced_motion {
        [
            offset[0] * attenuation,
            offset[1] * attenuation,
            offset[2] * attenuation,
        ]
    } else {
        offset
    }
}

/// Overlay fade curve (mirrors decal lifecycle semantics).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverlayPhase {
    /// Fading in.
    FadeIn,
    /// Holding.
    Sustain,
    /// Fading out.
    FadeOut,
    /// Done.
    Expired,
}

/// Full-screen overlay opacity curve (**TC-11.3.6.1**).
#[derive(Clone, Debug)]
pub struct OverlayLifecycle {
    /// Fade-in duration in seconds.
    pub fade_in_secs: f32,
    /// Sustain duration in seconds.
    pub sustain_secs: f32,
    /// Fade-out duration in seconds.
    pub fade_out_secs: f32,
    /// Elapsed time since spawn in seconds.
    pub elapsed_secs: f32,
}

impl OverlayLifecycle {
    /// Computes phase for the current elapsed time.
    pub fn phase(&self) -> OverlayPhase {
        let fi = self.fade_in_secs.max(f32::EPSILON);
        let sus = self.sustain_secs.max(0.0);
        let fo = self.fade_out_secs.max(f32::EPSILON);
        let t = self.elapsed_secs;
        if t < fi {
            OverlayPhase::FadeIn
        } else if t < fi + sus {
            OverlayPhase::Sustain
        } else if t < fi + sus + fo {
            OverlayPhase::FadeOut
        } else {
            OverlayPhase::Expired
        }
    }

    /// Linear opacity in \[0,1\] for `elapsed_secs`.
    pub fn opacity(&self) -> f32 {
        let fi = self.fade_in_secs.max(f32::EPSILON);
        let sus = self.sustain_secs.max(0.0);
        let fo = self.fade_out_secs.max(f32::EPSILON);
        let t = self.elapsed_secs;
        if t <= 0.0 {
            return 0.0;
        }
        if t < fi {
            return (t / fi).clamp(0.0, 1.0);
        }
        if t < fi + sus {
            return 1.0;
        }
        if t < fi + sus + fo {
            let u = (t - fi - sus) / fo;
            return (1.0 - u).clamp(0.0, 1.0);
        }
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::{OverlayLifecycle, OverlayPhase, ShakeSource, apply_reduced_motion, combine_shake_sources};

    /// **TC-11.3.1.1** — exponential decay drives amplitude below 0.01 by t=3s.
    #[test]
    fn tc_11_3_1_1_shake_decay() {
        let s = ShakeSource {
            amplitude: 1.0,
            decay_rate: 2.0,
            elapsed_secs: 3.0,
        };
        assert!(s.current_amplitude() < 0.01);
    }

    /// **TC-11.3.1.2** — many sources are clamped to `max_amp`.
    #[test]
    fn tc_11_3_1_2_shake_additive_clamping() {
        let sources = [ShakeSource {
            amplitude: 1.0,
            decay_rate: 0.0,
            elapsed_secs: 0.0,
        }; 10];
        assert!((combine_shake_sources(&sources, 1.0) - 1.0).abs() < 1e-5);
    }

    /// **TC-11.3.1.3** — reduced motion with zero attenuation zeroes the offset.
    #[test]
    fn tc_11_3_1_3_shake_reduced_motion() {
        let v = [1.0_f32, 2.0, 3.0];
        let out = apply_reduced_motion(v, true, 0.0);
        assert_eq!(out, [0.0, 0.0, 0.0]);
    }

    /// **TC-11.3.6.1** — overlay fade-in and sustain sampling.
    #[test]
    fn tc_11_3_6_1_overlay_lifecycle() {
        let o = OverlayLifecycle {
            fade_in_secs: 0.5,
            sustain_secs: 1.0,
            fade_out_secs: 0.5,
            elapsed_secs: 0.25,
        };
        assert_eq!(o.phase(), OverlayPhase::FadeIn);
        assert!((o.opacity() - 0.5).abs() < 1e-5);

        let o2 = OverlayLifecycle {
            fade_in_secs: 0.5,
            sustain_secs: 1.0,
            fade_out_secs: 0.5,
            elapsed_secs: 1.0,
        };
        assert_eq!(o2.phase(), OverlayPhase::Sustain);
        assert!((o2.opacity() - 1.0).abs() < 1e-5);
    }
}
