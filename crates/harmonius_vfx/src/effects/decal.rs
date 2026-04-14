//! Decal lifecycle and projection types (`docs/design/vfx/effects.md`).

/// Phase of a decal or overlay time curve.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LifecyclePhase {
    /// Opacity ramping from 0 to 1.
    FadeIn,
    /// Full opacity held for `sustain_secs`.
    Sustain,
    /// Opacity ramping from 1 to 0.
    Dissolve,
    /// Curve finished; instance should despawn.
    Expired,
}

/// Time-based fade / sustain / dissolve curve for a single decal instance.
#[derive(Clone, Debug, PartialEq)]
pub struct DecalLifecycle {
    /// Seconds to ramp opacity 0 → 1.
    pub fade_in_secs: f32,
    /// Seconds to hold at full opacity after fade-in.
    pub sustain_secs: f32,
    /// Seconds to ramp opacity 1 → 0 after sustain.
    pub dissolve_secs: f32,
    /// Seconds since spawn along this curve.
    pub elapsed: f32,
    /// Cached phase; kept in sync by [`DecalLifecycle::rebuild`].
    pub phase: LifecyclePhase,
}

impl DecalLifecycle {
    /// Builds a lifecycle with phase derived from `elapsed` and the three durations.
    pub fn new(fade_in_secs: f32, sustain_secs: f32, dissolve_secs: f32, elapsed: f32) -> Self {
        let mut s = Self {
            fade_in_secs,
            sustain_secs,
            dissolve_secs,
            elapsed,
            phase: LifecyclePhase::FadeIn,
        };
        s.rebuild();
        s
    }

    /// Recomputes [`LifecyclePhase`] from [`DecalLifecycle::elapsed`] and durations.
    pub fn rebuild(&mut self) {
        let t = self.elapsed;
        let fi = self.fade_in_secs.max(f32::EPSILON);
        let sus = self.sustain_secs.max(0.0);
        let dis = self.dissolve_secs.max(f32::EPSILON);
        let t_fi = fi;
        let t_sus = t_fi + sus;
        let t_dis = t_sus + dis;

        self.phase = if t < t_fi {
            LifecyclePhase::FadeIn
        } else if t < t_sus {
            LifecyclePhase::Sustain
        } else if t < t_dis {
            LifecyclePhase::Dissolve
        } else {
            LifecyclePhase::Expired
        };
    }

    /// Linear opacity in \[0, 1\] for the current `elapsed` time.
    ///
    /// Matches **TC-11.2.4.1**: 1s fade-in, 2s sustain, 1s dissolve.
    pub fn opacity(&self) -> f32 {
        // Red-phase stub (TC-11.2.4.1): implementation lands in the follow-up green commit.
        let _ = self.elapsed;
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::{DecalLifecycle, LifecyclePhase};

    /// **TC-11.2.4.1** — Decal: 1s fade-in, 2s sustain, 1s dissolve.
    #[test]
    fn tc_11_2_4_1_decal_lifecycle_phases() {
        let lc = |elapsed: f32| DecalLifecycle::new(1.0, 2.0, 1.0, elapsed);

        let a = lc(0.5);
        assert_eq!(a.phase, LifecyclePhase::FadeIn);
        assert!((a.opacity() - 0.5).abs() < 1e-5);

        let b = lc(1.5);
        assert_eq!(b.phase, LifecyclePhase::Sustain);
        assert!((b.opacity() - 1.0).abs() < 1e-5);

        let c = lc(3.5);
        assert_eq!(c.phase, LifecyclePhase::Dissolve);
        assert!((c.opacity() - 0.5).abs() < 1e-5);
    }
}
