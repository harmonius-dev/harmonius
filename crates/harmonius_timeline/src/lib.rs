//! Timeline sequencer primitives for Harmonius.
//!
//! Implements the data model and sampling rules from `docs/design/simulation/timelines.md`.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod asset;
mod bezier;
mod event;
mod ids;
mod keyframe;
mod lerp;
mod playback;
mod track;
mod vectors;

pub use asset::{Color, LoopMode, MultiTrackTimeline, TrackValue};
pub use event::{TimelineEvent, TimelineEventKind};
pub use ids::{AssetId, Entity, KeyframeId, TrackId};
pub use keyframe::{Interpolation, Keyframe};
pub use lerp::Lerp;
pub use playback::{PlaybackDirection, PlaybackState};
pub use track::Track;
pub use vectors::{Quat, Vec2, Vec3, Vec4};
