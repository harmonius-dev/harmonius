//! Timelines ↔ camera integration contracts (IR-4.8.*).
//!
//! This crate hosts deterministic binding logic that mirrors
//! `docs/design/integration/timelines-camera.md` without pulling in the full
//! ECS runtime.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod assets;
mod binding;
mod handle;
mod interp;
mod playback;
mod recomposer;
mod sequencer;
mod timeline;

pub use assets::AssetStore;
pub use binding::{
    apply_timeline_camera_binding, clamp_unit_f32, TimelineCameraBinding,
    TimelineCameraDebug,
};
pub use handle::Handle;
pub use interp::interpolate_recomposer_overrides;
pub use playback::{PlaybackDirection, PlaybackState};
pub use recomposer::{DollyRig, Recomposer};
pub use sequencer::SequencerEntry;
pub use timeline::{
    Interpolation, Keyframe, LoopMode, MultiTrackTimeline, Track, TrackId, TrackValue,
};
