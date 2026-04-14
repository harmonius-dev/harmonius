//! Tick-based playback state shared by timelines and scripting.

use crate::ids::TickCount;

/// Playback controls for a timeline bound to an entity (tick axis).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlaybackState {
    /// Current playhead position.
    pub current_tick: TickCount,
    /// When false, advance integration skips time integration.
    pub playing: bool,
    /// Looping flag (reserved for timeline integration).
    pub looping: bool,
}

impl PlaybackState {
    /// Rewinds to tick zero with playback stopped.
    pub fn new_stopped() -> Self {
        Self {
            current_tick: TickCount(0),
            playing: false,
            looping: false,
        }
    }

    /// Starts or resumes playback integration.
    pub fn play(&mut self) {
        self.playing = true;
    }

    /// Freezes playback without moving the cursor.
    pub fn pause(&mut self) {
        self.playing = false;
    }

    /// Stops playback and rewinds the cursor to tick zero.
    pub fn stop(&mut self) {
        self.playing = false;
        self.current_tick = TickCount(0);
    }

    /// Seeks to `target` clamped to `[TickCount(0), duration]`.
    pub fn seek(&mut self, target: TickCount, duration: TickCount) {
        let max = duration.0;
        let t = target.0.min(max);
        self.current_tick = TickCount(t);
    }

    /// Advances the playhead by `delta` ticks when playing (no events here).
    pub fn advance_ticks(&mut self, delta: u64, duration: TickCount) {
        if !self.playing || delta == 0 {
            return;
        }
        let max = duration.0;
        let next = self.current_tick.0.saturating_add(delta).min(max);
        self.current_tick = TickCount(next);
    }
}

#[cfg(test)]
mod tests {
    use super::PlaybackState;
    use crate::ids::TickCount;

    #[test]
    fn tc_ir_4_9_2_1_play_sets_playing() {
        let mut p = PlaybackState::new_stopped();
        p.play();
        assert!(p.playing);
    }

    #[test]
    fn tc_ir_4_9_2_2_pause_freezes_cursor() {
        let mut p = PlaybackState::new_stopped();
        p.play();
        p.seek(TickCount(120), TickCount(600));
        p.pause();
        assert!(!p.playing);
        assert_eq!(p.current_tick, TickCount(120));
    }

    #[test]
    fn tc_ir_4_9_2_3_seek_moves_cursor() {
        let mut p = PlaybackState::new_stopped();
        p.play();
        p.seek(TickCount(180), TickCount(600));
        assert_eq!(p.current_tick, TickCount(180));
        assert!(p.playing);
    }

    #[test]
    fn tc_ir_4_9_2_4_stop_resets_cursor() {
        let mut p = PlaybackState::new_stopped();
        p.play();
        p.seek(TickCount(200), TickCount(600));
        p.stop();
        assert!(!p.playing);
        assert_eq!(p.current_tick, TickCount(0));
    }

    #[test]
    fn tc_ir_4_9_2_5_pause_vs_stop_cursor() {
        let mut p = PlaybackState::new_stopped();
        p.play();
        p.seek(TickCount(120), TickCount(600));
        p.pause();
        assert_eq!(p.current_tick, TickCount(120));
        p.play();
        p.stop();
        assert_eq!(p.current_tick, TickCount(0));
    }

    #[test]
    fn tc_ir_4_9_6_2_seek_clamps_to_duration() {
        let mut p = PlaybackState::new_stopped();
        p.seek(TickCount(99999), TickCount(600));
        assert_eq!(p.current_tick, TickCount(600));
    }
}
