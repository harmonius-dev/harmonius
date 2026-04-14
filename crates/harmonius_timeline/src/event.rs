//! Runtime events emitted while advancing playback.

use crate::ids::{Entity, KeyframeId, TrackId};

/// High-level lifecycle signal for timeline consumers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TimelineEventKind {
    /// Playback crossed a keyframe time (inclusive boundary in the integration direction).
    KeyframeCrossed {
        /// Track that owns the keyframe.
        track: TrackId,
        /// Keyframe that was crossed.
        keyframe: KeyframeId,
    },
    /// A track reached its terminal keyframe.
    TrackComplete {
        /// Track that finished.
        track: TrackId,
    },
    /// The entire timeline finished in `LoopMode::Once`.
    TimelineComplete,
    /// A loop boundary was crossed.
    LoopPoint {
        /// Monotonic counter of loop wraps.
        count: u32,
    },
}

/// Concrete event instance tied to an entity for ECS routing.
#[derive(Clone, Debug, PartialEq)]
pub struct TimelineEvent {
    /// Kind-specific payload.
    pub kind: TimelineEventKind,
    /// Timeline time associated with the event.
    pub time: f64,
    /// Entity that owns the playback state.
    pub entity: Entity,
}
