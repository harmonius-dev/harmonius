//! Typed keyframe tracks and sampling.

use smallvec::SmallVec;
use smol_str::SmolStr;

use crate::bezier::ease_t;
use crate::ids::TrackId;
use crate::keyframe::{Interpolation, Keyframe};
use crate::Lerp;

/// Named channel of keyframes sorted by time.
#[derive(Clone, Debug, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct Track<T> {
    /// Stable id for O(1) lookup in timelines.
    pub id: TrackId,
    /// Human-readable channel name from authoring tools.
    pub name: SmolStr,
    /// Keyframes sorted ascending by `time`.
    pub keyframes: SmallVec<[Keyframe<T>; 8]>,
    /// Value used when the track has no keyframes.
    pub default_value: T,
}

impl<T: Lerp + Clone> Track<T> {
    /// Samples the track at `time` using the interpolation rules between keyframes.
    pub fn sample(&self, time: f64) -> T {
        if self.keyframes.is_empty() {
            return self.default_value.clone();
        }

        if time < self.keyframes[0].time {
            return self.keyframes[0].value.clone();
        }

        let last = self
            .keyframes
            .last()
            .expect("keyframes non-empty implies last exists");
        if time >= last.time {
            return last.value.clone();
        }

        let right_idx = self.keyframes.partition_point(|kf| kf.time <= time);
        let left_idx = right_idx - 1;
        let left = &self.keyframes[left_idx];
        let right = &self.keyframes[right_idx];

        match &left.interpolation {
            Interpolation::Constant => left.value.clone(),
            Interpolation::Step => {
                if time < right.time {
                    left.value.clone()
                } else {
                    right.value.clone()
                }
            }
            Interpolation::Linear => {
                let span = right.time - left.time;
                let u = if span <= f64::EPSILON {
                    0.0
                } else {
                    (time - left.time) / span
                };
                left.value.lerp(&right.value, u)
            }
            Interpolation::CubicBezier { c1, c2 } => {
                let span = right.time - left.time;
                let u = if span <= f64::EPSILON {
                    0.0
                } else {
                    (time - left.time) / span
                };
                let eased = ease_t(u, *c1, *c2);
                left.value.lerp(&right.value, eased)
            }
        }
    }

    /// Returns the duration implied by the last keyframe time, or `0.0` when empty.
    pub fn duration(&self) -> f64 {
        self.keyframes.last().map(|kf| kf.time).unwrap_or(0.0)
    }

    /// Returns the last keyframe with `time <= query`, if any.
    pub fn keyframe_at_or_before(&self, time: f64) -> Option<&Keyframe<T>> {
        if self.keyframes.is_empty() {
            return None;
        }

        let p = self.keyframes.partition_point(|kf| kf.time <= time);
        if p == 0 {
            return None;
        }

        Some(&self.keyframes[p - 1])
    }

    /// Counts keyframes stored on the track.
    pub fn keyframe_count(&self) -> usize {
        self.keyframes.len()
    }
}
