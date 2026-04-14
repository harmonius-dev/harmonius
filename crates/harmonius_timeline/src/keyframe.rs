//! Keyframe envelopes and interpolation metadata.

use crate::vectors::Vec2;

use crate::KeyframeId;

/// Interpolation mode between adjacent keyframes.
#[derive(Clone, Copy, Debug, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub enum Interpolation {
    /// Hold the previous value until the next keyframe time.
    Step,
    /// Linear blend between keyframe values.
    Linear,
    /// CSS cubic-bezier easing between keyframe values.
    CubicBezier {
        /// First control point in normalized easing space.
        c1: Vec2,
        /// Second control point in normalized easing space.
        c2: Vec2,
    },
    /// Hold the previous value for the remainder of the track.
    Constant,
}

/// Timestamped value on a track.
#[derive(Clone, Debug, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct Keyframe<T> {
    /// Stable id for event correlation.
    pub id: KeyframeId,
    /// Seconds on the parent timeline.
    pub time: f64,
    /// Sampled value at `time`.
    pub value: T,
    /// How this keyframe blends toward the next.
    pub interpolation: Interpolation,
    /// Authoring marker for tooling; all keyframe crossings emit `KeyframeCrossed` during advance.
    pub trigger: bool,
}
