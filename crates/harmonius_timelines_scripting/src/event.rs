//! Timeline events emitted during advance and consumed by scripting.

use crate::ids::{Entity, KeyframeId, TickCount, TrackId};

/// Discrete timeline event kind (fully enumerated).
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TimelineEventKind {
    /// A keyframe was crossed on `track`.
    KeyframeCrossed {
        track: TrackId,
        keyframe: KeyframeId,
    },
    /// A track finished playing.
    TrackComplete {
        track: TrackId,
    },
    /// The whole timeline finished playing.
    TimelineComplete,
    /// A loop boundary was crossed; `count` is completed loops so far.
    LoopPoint {
        count: u32,
    },
}

/// Timeline event envelope (`time` is tick-typed per integration design).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimelineEvent {
    /// Event payload.
    pub kind: TimelineEventKind,
    /// Tick when the event was produced.
    pub time: TickCount,
    /// Timeline entity that produced the event.
    pub entity: Entity,
    /// Resolved `TrackValue::Entity` target for scripting dispatch (IR-4.9.1).
    pub graph_entity: Option<Entity>,
}
