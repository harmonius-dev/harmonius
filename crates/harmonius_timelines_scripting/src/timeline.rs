//! Minimal multi-track timeline asset and sampling helpers.

use smallvec::SmallVec;

use crate::ids::{Entity, KeyframeId, TickCount, TrackId};

/// Sampled or keyframed value carried on a track.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TrackValue {
    F32(f32),
    Bool(bool),
    Vec3([f32; 3]),
    Entity(Entity),
}

/// Single keyframe on a track.
#[derive(Clone, Debug, PartialEq)]
pub struct Keyframe {
    pub id: KeyframeId,
    pub tick: TickCount,
    pub value: TrackValue,
}

/// One named track on a timeline.
#[derive(Clone, Debug, PartialEq)]
pub struct Track {
    pub id: TrackId,
    pub keyframes: Vec<Keyframe>,
}

/// Authoring-time timeline asset.
#[derive(Clone, Debug, PartialEq)]
pub struct MultiTrackTimeline {
    pub duration: TickCount,
    pub tracks: Vec<Track>,
}

impl MultiTrackTimeline {
    /// Returns the constant value from the last keyframe at or before `tick`.
    pub fn sample_track(&self, track_id: TrackId, tick: TickCount) -> Option<TrackValue> {
        let tr = self.tracks.iter().find(|t| t.id == track_id)?;
        let mut current: Option<TrackValue> = None;
        for kf in &tr.keyframes {
            if kf.tick.0 <= tick.0 {
                current = Some(kf.value);
            }
        }
        current
    }

    /// Keyframes strictly after `from` and at or before `to`, ordered by `KeyframeId`.
    pub fn keyframes_crossed(
        &self,
        from: TickCount,
        to: TickCount,
    ) -> SmallVec<[(TrackId, KeyframeId, TrackValue); 8]> {
        let mut out: SmallVec<[(TrackId, KeyframeId, TrackValue); 8]> = SmallVec::new();
        for tr in &self.tracks {
            for kf in &tr.keyframes {
                if kf.tick.0 > from.0 && kf.tick.0 <= to.0 {
                    out.push((tr.id, kf.id, kf.value));
                }
            }
        }
        out.sort_by(|a, b| a.1.cmp(&b.1));
        out
    }
}
