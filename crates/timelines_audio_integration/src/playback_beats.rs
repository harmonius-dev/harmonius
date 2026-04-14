//! Beat-based timeline advancement helpers.

use crate::BeatSnapshot;

/// Minimal playback clock for tests (wall vs beat time bases).
#[derive(Clone, Debug, PartialEq)]
pub struct PlaybackState {
    /// Seconds along the timeline when using wall-clock advancement.
    pub time_seconds: f64,
    /// Fixed-point beat ticks accumulated when using `TimeBase::Beats`.
    pub beat_ticks: u64,
}

impl PlaybackState {
    /// Starts at the origin.
    pub fn new() -> Self {
        Self {
            time_seconds: 0.0,
            beat_ticks: 0,
        }
    }

    /// Advances beat ticks from a BPM snapshot and fixed timestep.
    pub fn advance_beats(
        &mut self,
        snap: BeatSnapshot,
        time_base: TimeBase,
        dt_seconds: f64,
    ) -> u64 {
        let TimeBase::Beats { ticks_per_beat } = time_base else {
            return self.beat_ticks;
        };
        let beats_per_second = f64::from(snap.bpm) / 60.0;
        let ticks_per_second = beats_per_second * f64::from(ticks_per_beat);
        let delta_ticks = (ticks_per_second * dt_seconds).round() as u64;
        self.beat_ticks = self.beat_ticks.saturating_add(delta_ticks);
        self.beat_ticks
    }

    /// Updates wall-clock timeline seconds.
    pub fn advance_wall_seconds(&mut self, dt_seconds: f64) {
        self.time_seconds += dt_seconds;
    }
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self::new()
    }
}

/// Timeline time base selection from the integration design.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeBase {
    /// Real-time seconds.
    Wall,
    /// Musical ticks per quarter note.
    Beats {
        /// Fixed-point ticks per beat for sequencing resolution.
        ticks_per_beat: u32,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-IR-4.7.4.1 — 120 BPM yields two beats per second at `ticks_per_beat = 1`.
    #[test]
    fn tc_ir_4_7_4_1_beat_time_timeline_advances() {
        let mut pb = PlaybackState::new();
        let snap = BeatSnapshot {
            bar: 1,
            beat: 1,
            phase: 0.0,
            bpm: 120.0,
        };
        let ticks = pb.advance_beats(snap, TimeBase::Beats { ticks_per_beat: 1 }, 1.0);
        assert_eq!(ticks, 2);
    }

    /// TC-IR-4.7.4.2 — BPM changes are picked up on the next advance call.
    #[test]
    fn tc_ir_4_7_4_2_tempo_change_updates_rate() {
        let mut pb = PlaybackState::new();
        let mut snap = BeatSnapshot {
            bar: 1,
            beat: 1,
            phase: 0.0,
            bpm: 120.0,
        };
        pb.advance_beats(snap, TimeBase::Beats { ticks_per_beat: 1 }, 1.0);
        snap.bpm = 140.0;
        let ticks = pb.advance_beats(snap, TimeBase::Beats { ticks_per_beat: 1 }, 1.0);
        assert_eq!(ticks, pb.beat_ticks);
        assert!(pb.beat_ticks > 2);
    }
}
