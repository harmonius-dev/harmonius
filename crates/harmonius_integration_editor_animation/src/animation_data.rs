//! Minimal animation authoring payloads carried on editor undo commands.

use crate::ids::{BoneTrackIndex, Entity};

/// Single keyframe sample on a bone track.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Keyframe {
    /// Normalized or absolute time domain is defined by the owning clip.
    pub time: f32,
}

/// In/out tangents for Hermite or Bezier curve segments.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TangentPair {
    /// Incoming tangent magnitude or control value.
    pub in_tangent: f32,
    /// Outgoing tangent magnitude or control value.
    pub out_tangent: f32,
}

/// Marker emitted from the timeline for gameplay or tooling observers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnimEventMarker {
    /// Normalized timeline position in `[0.0, 1.0]`.
    pub normalized_time: f32,
    /// Stable event type id for routing.
    pub event_kind: u32,
}

/// One sample point inside a 2D blend space diagram.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BlendSample2D {
    /// Horizontal blend axis coordinate.
    pub x: f32,
    /// Vertical blend axis coordinate.
    pub y: f32,
    /// Weight contribution for this sample.
    pub weight: f32,
}

/// Key used to store per-track keyframe sequences in the editor [`World`](crate::World).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TrackKey {
    /// Owning entity for authored data.
    pub entity: Entity,
    /// Track index inside the clip.
    pub track: BoneTrackIndex,
}
