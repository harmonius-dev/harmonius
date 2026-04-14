//! Minimal sequencer entry metadata for timeline-driven cameras.

use crate::handle::Handle;
use crate::playback::PlaybackState;

/// One playlist entry evaluated by the camera sequencer.
#[derive(Clone, Debug, PartialEq)]
pub struct SequencerEntry {
    /// Seconds to hold this shot before advancing.
    pub hold_time: f32,
    /// Optional playback component that must start with this entry.
    pub timeline_ref: Option<Handle<PlaybackState>>,
}

impl Default for SequencerEntry {
    fn default() -> Self {
        Self {
            hold_time: 0.0,
            timeline_ref: None,
        }
    }
}
