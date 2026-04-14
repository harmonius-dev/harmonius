//! Timelines ↔ audio integration primitives from
//! `docs/design/integration/timelines-audio.md`.
//!
//! This crate is CI-friendly: bounded queues use `std::sync::mpsc::sync_channel` with
//! `try_send`, matching the design’s capacity and back-pressure semantics without pulling
//! in extra dependencies.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod audio_command_queue;
mod beat_counter;
mod beat_event_queue;
mod debug;
mod dispatch;
mod music_machine;
mod playback_beats;
mod types;
mod voice_manager;

pub use audio_command_queue::{AudioCommandQueue, AudioCommandSender};
pub use beat_counter::{AtomicBeatCounter, BeatSnapshot};
pub use beat_event_queue::{BeatEvent, BeatEventQueue, BeatEventSender};
pub use debug::TimelineAudioDebug;
pub use dispatch::{
    dispatch_subtitle_hide_for_stop, dispatch_track_target, DispatchContext, DispatchOutcome,
};
pub use music_machine::{MusicStateMachine, TransitionRule};
pub use playback_beats::{PlaybackState, TimeBase};
pub use types::{
    AssetHandle, AudioClip, AudioCommand, AudioTimestamp, AudioTrackTarget, BusId, BusParam,
    DialogueLineId, ParamId, SegmentId, StingerRequest, StringId, SubtitleEvent, VoiceId,
    VoicePriority,
};
pub use voice_manager::VoiceManager;
