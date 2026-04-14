//! Minimal multi-track timeline sampling for integration tests.

use glam::Vec3;

/// Stable identifier for a authored track.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TrackId(pub u32);

/// Payload types supported by the integration binding layer.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TrackValue {
    /// Scalar channel (FOV delta, dolly parameter, blend weight).
    F32(f32),
    /// Euler degrees or camera-local offsets in world units.
    Vec3(Vec3),
}

/// Looping behavior for authored timelines.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LoopMode {
    /// Play once and clamp at the end.
    Once,
    /// Repeat from zero after `duration`.
    Loop,
    /// Reverse direction at ends.
    PingPong,
    /// Hold the final sample forever.
    ClampForever,
}

/// How values blend between two keyframes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Interpolation {
    /// Linear interpolation in value space.
    Linear,
    /// Hold left keyframe until the right keyframe time.
    Step,
}

/// Single authored keyframe.
#[derive(Clone, Debug, PartialEq)]
pub struct Keyframe {
    /// Seconds from timeline start.
    pub time: f64,
    /// Sample payload.
    pub value: TrackValue,
    /// Interpolation segment into the next keyframe.
    pub interpolation: Interpolation,
}

/// One named channel of keyframes.
#[derive(Clone, Debug, PartialEq)]
pub struct Track {
    /// Track identifier used by bindings.
    pub id: TrackId,
    /// Sorted by `time` ascending.
    pub keyframes: Vec<Keyframe>,
}

/// Immutable multi-track timeline asset (subset of the engine asset).
#[derive(Clone, Debug, PartialEq)]
pub struct MultiTrackTimeline {
    /// Stable asset key for debugging.
    pub id: u64,
    /// Owned track list.
    pub tracks: Vec<Track>,
    /// Authoring duration in seconds.
    pub duration: f64,
    /// Looping policy.
    pub loop_mode: LoopMode,
}

impl MultiTrackTimeline {
    /// Looks up a track by id (hot path).
    #[must_use]
    pub fn track_by_id(&self, id: TrackId) -> Option<&Track> {
        self.tracks.iter().find(|track| track.id == id)
    }

    /// Samples `id` at `time` using the timeline loop rules.
    #[must_use]
    pub fn sample_track(&self, id: TrackId, time: f64) -> Option<TrackValue> {
        let track = self.track_by_id(id)?;
        let mapped = map_time(time, self.duration, self.loop_mode);
        sample_track_inner(track, mapped)
    }
}

fn map_time(time: f64, duration: f64, mode: LoopMode) -> f64 {
    if duration <= 0.0 {
        return 0.0;
    }
    match mode {
        LoopMode::Once | LoopMode::ClampForever => time.clamp(0.0, duration),
        LoopMode::Loop => {
            let mut t = time.rem_euclid(duration);
            if t == duration && time > 0.0 {
                t = 0.0;
            }
            t
        }
        LoopMode::PingPong => {
            let span = duration * 2.0;
            let wrapped = time.rem_euclid(span);
            if wrapped <= duration {
                wrapped
            } else {
                span - wrapped
            }
        }
    }
}

fn sample_track_inner(track: &Track, time: f64) -> Option<TrackValue> {
    let kfs = &track.keyframes;
    if kfs.is_empty() {
        return None;
    }
    if time <= kfs[0].time {
        return Some(kfs[0].value);
    }
    if time >= kfs[kfs.len() - 1].time {
        return Some(kfs[kfs.len() - 1].value);
    }
    let idx = kfs.partition_point(|keyframe| keyframe.time <= time);
    let right_idx = idx.min(kfs.len() - 1);
    let left_idx = right_idx.saturating_sub(1);
    let left = &kfs[left_idx];
    let right = &kfs[right_idx];
    if left_idx == right_idx {
        return Some(left.value);
    }
    match right.interpolation {
        Interpolation::Step => Some(left.value),
        Interpolation::Linear => {
            let span = (right.time - left.time).max(f64::EPSILON);
            let alpha = ((time - left.time) / span).clamp(0.0, 1.0) as f32;
            lerp_track_value(left.value, right.value, alpha)
        }
    }
}

fn lerp_track_value(a: TrackValue, b: TrackValue, alpha: f32) -> Option<TrackValue> {
    match (a, b) {
        (TrackValue::F32(x), TrackValue::F32(y)) => {
            Some(TrackValue::F32(x + (y - x) * alpha))
        }
        (TrackValue::Vec3(x), TrackValue::Vec3(y)) => {
            Some(TrackValue::Vec3(x.lerp(y, alpha)))
        }
        _ => None,
    }
}
