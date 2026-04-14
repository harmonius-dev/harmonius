//! Minimal adaptive music transition state used by integration tests.

use crate::BeatSnapshot;
use crate::SegmentId;

/// Edge rule applied when crossing a music cue keyframe.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TransitionRule {
    /// Instantly swap segments.
    ImmediateCut,
    /// Equal-power crossfade over `seconds`.
    TimedCrossfade {
        /// Fade duration in seconds.
        seconds: f32,
    },
    /// Defer segment swap until the next beat boundary.
    BeatSyncCrossfade,
}

/// Simplified music graph driver for deterministic tests (not a full mixer).
#[derive(Clone, Debug, PartialEq)]
pub struct MusicStateMachine {
    current: Option<SegmentId>,
    pending_target: Option<SegmentId>,
    pending_rule: Option<TransitionRule>,
    crossfade_remaining: Option<f32>,
}

impl MusicStateMachine {
    /// Starts in silence.
    pub fn new() -> Self {
        Self {
            current: None,
            pending_target: None,
            pending_rule: None,
            crossfade_remaining: None,
        }
    }

    /// Requests a transition to `target` using `rule` and the latest beat snapshot.
    pub fn transition(&mut self, target: SegmentId, rule: TransitionRule, snap: BeatSnapshot) {
        if self.pending_target.is_some() || self.crossfade_remaining.is_some() {
            self.pending_target = None;
            self.pending_rule = None;
            self.crossfade_remaining = None;
        }
        self.begin_transition(target, rule, snap);
    }

    /// Advances internal timers (call once per audio buffer or test step).
    pub fn tick(&mut self, dt_seconds: f32, snap: BeatSnapshot) {
        if let Some(ref mut remaining) = self.crossfade_remaining {
            *remaining = (*remaining - dt_seconds).max(0.0);
            if *remaining <= f32::EPSILON {
                self.current = self.pending_target.take();
                self.pending_rule = None;
                self.crossfade_remaining = None;
            }
        }

        if matches!(self.pending_rule, Some(TransitionRule::BeatSyncCrossfade))
            && (snap.phase <= 1.0e-3 || snap.phase >= 1.0 - 1.0e-3)
        {
            if let Some(target) = self.pending_target.take() {
                self.current = Some(target);
                self.pending_rule = None;
            }
        }
    }

    /// Remaining timed crossfade duration if active.
    pub fn crossfade_remaining_seconds(&self) -> Option<f32> {
        self.crossfade_remaining
    }

    /// Current audible segment after transitions resolve.
    pub fn current_segment(&self) -> Option<SegmentId> {
        self.current
    }

    fn begin_transition(&mut self, target: SegmentId, rule: TransitionRule, snap: BeatSnapshot) {
        match rule {
            TransitionRule::ImmediateCut => {
                self.current = Some(target);
                self.pending_target = None;
                self.pending_rule = None;
                self.crossfade_remaining = None;
            }
            TransitionRule::TimedCrossfade { seconds } => {
                self.pending_target = Some(target);
                self.pending_rule = Some(rule);
                self.crossfade_remaining = Some(seconds);
            }
            TransitionRule::BeatSyncCrossfade => {
                if snap.phase <= 1.0e-3 {
                    self.current = Some(target);
                    self.pending_target = None;
                    self.pending_rule = None;
                } else {
                    self.pending_target = Some(target);
                    self.pending_rule = Some(TransitionRule::BeatSyncCrossfade);
                }
            }
        }
    }
}

impl Default for MusicStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-IR-4.7.1.2 — timed crossfade runs for one second.
    #[test]
    fn tc_ir_4_7_1_2_timed_crossfade_seconds() {
        let mut msm = MusicStateMachine::new();
        msm.transition(
            SegmentId(10),
            TransitionRule::TimedCrossfade { seconds: 1.0 },
            BeatSnapshot {
                bar: 1,
                beat: 1,
                phase: 0.5,
                bpm: 120.0,
            },
        );
        assert_eq!(msm.crossfade_remaining_seconds(), Some(1.0));
        msm.tick(
            0.5,
            BeatSnapshot {
                bar: 1,
                beat: 2,
                phase: 0.0,
                bpm: 120.0,
            },
        );
        assert_eq!(msm.crossfade_remaining_seconds(), Some(0.5));
        msm.tick(
            0.6,
            BeatSnapshot {
                bar: 1,
                beat: 3,
                phase: 0.0,
                bpm: 120.0,
            },
        );
        assert_eq!(msm.crossfade_remaining_seconds(), None);
        assert_eq!(msm.current_segment(), Some(SegmentId(10)));
    }

    /// TC-IR-4.7.1.3 — beat-sync transition commits on boundary-ish phase.
    #[test]
    fn tc_ir_4_7_1_3_beat_sync_transition() {
        let mut msm = MusicStateMachine::new();
        msm.transition(
            SegmentId(2),
            TransitionRule::BeatSyncCrossfade,
            BeatSnapshot {
                bar: 1,
                beat: 1,
                phase: 0.5,
                bpm: 120.0,
            },
        );
        assert_eq!(msm.current_segment(), None);
        msm.tick(
            0.0,
            BeatSnapshot {
                bar: 1,
                beat: 2,
                phase: 0.0,
                bpm: 120.0,
            },
        );
        assert_eq!(msm.current_segment(), Some(SegmentId(2)));
    }

    /// TC-IR-4.7.1.5 — rapid second transition supersedes the first pending transition.
    #[test]
    fn tc_ir_4_7_1_5_transition_overlap_resolves() {
        let mut msm = MusicStateMachine::new();
        msm.transition(
            SegmentId(1),
            TransitionRule::BeatSyncCrossfade,
            BeatSnapshot {
                bar: 1,
                beat: 1,
                phase: 0.5,
                bpm: 120.0,
            },
        );
        msm.transition(
            SegmentId(2),
            TransitionRule::BeatSyncCrossfade,
            BeatSnapshot {
                bar: 1,
                beat: 1,
                phase: 0.9,
                bpm: 120.0,
            },
        );
        msm.tick(
            0.0,
            BeatSnapshot {
                bar: 1,
                beat: 2,
                phase: 0.0,
                bpm: 120.0,
            },
        );
        assert_eq!(msm.current_segment(), Some(SegmentId(2)));
    }

    /// TC-IR-4.7.N.4 — in-progress fade cancelled by a new transition request.
    #[test]
    fn tc_ir_4_7_n_4_music_overlap_fallback() {
        let mut msm = MusicStateMachine::new();
        msm.transition(
            SegmentId(1),
            TransitionRule::TimedCrossfade { seconds: 2.0 },
            BeatSnapshot {
                bar: 1,
                beat: 1,
                phase: 0.1,
                bpm: 120.0,
            },
        );
        msm.transition(
            SegmentId(9),
            TransitionRule::ImmediateCut,
            BeatSnapshot {
                bar: 1,
                beat: 1,
                phase: 0.2,
                bpm: 120.0,
            },
        );
        assert_eq!(msm.current_segment(), Some(SegmentId(9)));
        assert_eq!(msm.crossfade_remaining_seconds(), None);
    }
}
