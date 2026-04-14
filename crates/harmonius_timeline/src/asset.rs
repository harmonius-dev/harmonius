//! Immutable multi-track assets.

use crate::ids::{AssetId, Entity, TrackId};
use crate::lerp::Lerp;
use crate::track::Track;
use crate::vectors::{Quat, Vec2, Vec3, Vec4};

/// Looping behavior applied when playback reaches the timeline ends.
#[derive(
    Clone, Copy, Debug, Eq, Hash, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize,
)]
pub enum LoopMode {
    /// Play once and stop at the end.
    Once,
    /// Wrap time modulo `duration`.
    Loop,
    /// Reflect time at the ends while reversing direction.
    PingPong,
    /// Clamp to `[0, duration]` forever without completing.
    ClampForever,
}

/// HDR linear color sample for authored gradients.
#[derive(Clone, Copy, Debug, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct Color {
    /// Linear RGBA in physical color space.
    pub linear: Vec4,
}

impl Lerp for Color {
    fn lerp(&self, other: &Self, t: f64) -> Self {
        Color {
            linear: self.linear.lerp(other.linear, t as f32),
        }
    }
}

impl Lerp for TrackValue {
    fn lerp(&self, other: &Self, t: f64) -> Self {
        use TrackValue::*;
        match (self, other) {
            (F32(a), F32(b)) => F32(a.lerp(b, t)),
            (Vec2(a), Vec2(b)) => Vec2(a.lerp(b, t)),
            (Vec3(a), Vec3(b)) => Vec3(a.lerp(b, t)),
            (Quat(a), Quat(b)) => Quat(a.lerp(b, t)),
            (Color(a), Color(b)) => Color(a.lerp(b, t)),
            (Bool(a), Bool(b)) => Bool(a.lerp(b, t)),
            (Entity(a), Entity(b)) => Entity(a.lerp(b, t)),
            _ => panic!("TrackValue::lerp requires matching variants"),
        }
    }
}

/// Concrete value type for mixed tracks inside a timeline asset.
#[derive(Clone, Copy, Debug, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub enum TrackValue {
    /// Scalar float channel.
    F32(f32),
    /// Two-component vector channel.
    Vec2(Vec2),
    /// Three-component vector channel.
    Vec3(Vec3),
    /// Unit quaternion rotation channel.
    Quat(Quat),
    /// HDR color channel.
    Color(Color),
    /// Boolean gate channel.
    Bool(bool),
    /// Entity reference channel.
    Entity(crate::Entity),
}

/// Immutable multi-track timeline definition.
#[derive(Clone, Debug, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct MultiTrackTimeline {
    /// Asset id for save files and lookups.
    pub id: AssetId,
    /// All authored tracks, addressable by dense `TrackId` indices.
    pub tracks: Vec<Track<TrackValue>>,
    /// Cached duration (typically the max track end time).
    pub duration: f64,
    /// Looping semantics for playback.
    pub loop_mode: LoopMode,
}

impl MultiTrackTimeline {
    /// Linear scan by display name.
    pub fn track_by_name(&self, name: &str) -> Option<&Track<TrackValue>> {
        self.tracks.iter().find(|track| track.name == name)
    }

    /// O(1) lookup when `TrackId` matches the vector index.
    pub fn track_by_id(&self, id: TrackId) -> Option<&Track<TrackValue>> {
        self.tracks
            .get(id.0 as usize)
            .filter(|track| track.id == id)
    }

    /// Counts tracks in the asset.
    pub fn track_count(&self) -> usize {
        self.tracks.len()
    }

    /// Samples a single typed track at `time`.
    pub fn sample_track(&self, track: TrackId, time: f64) -> Option<TrackValue> {
        self.track_by_id(track).map(|t| t.sample(time))
    }

    /// Placeholder for ECS binding evaluation; wiring arrives with codegen integration.
    pub fn evaluate_all(&self, _time: f64, _entity: Entity, _world: &mut (), _bindings: &()) {}
}
