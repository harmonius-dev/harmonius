//! Playback state consumed by the binding system.

/// Playback direction for authored timelines.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlaybackDirection {
    /// Normal forward evaluation.
    Forward,
    /// Reverse evaluation.
    Reverse,
}

/// Mutable playback cursor for one bound entity.
#[derive(Clone, Debug, PartialEq)]
pub struct PlaybackState {
    /// Seconds along the authored timeline.
    pub current_time: f64,
    /// Scalar speed multiplier.
    pub speed: f32,
    /// Whether time advances with the simulation clock.
    pub playing: bool,
    /// Direction multiplier for authored evaluation.
    pub direction: PlaybackDirection,
}

impl PlaybackState {
    /// Default paused state at the timeline origin.
    #[must_use]
    pub fn new() -> Self {
        Self {
            current_time: 0.0,
            speed: 1.0,
            playing: false,
            direction: PlaybackDirection::Forward,
        }
    }

    /// Marks playback as running.
    pub fn play(&mut self) {
        self.playing = true;
    }

    /// Marks playback as paused.
    pub fn pause(&mut self) {
        self.playing = false;
    }
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self::new()
    }
}
