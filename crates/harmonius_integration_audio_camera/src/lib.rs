//! Camera ↔ audio listener bridge from `docs/design/integration/audio-camera.md`.
//!
//! Provides deterministic velocity derivation, previous-position tracking, and a frame sync helper
//! that matches the integration design. Engine wiring (ECS queries, `GameTime`, resources) can
//! call [`camera_listener_sync_frame`] with slices built each tick.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod bridge;
mod command;
mod prev;

pub use bridge::{
    camera_listener_sync_frame, listener_velocity, CameraListenerInput, ListenerSyncMetrics,
    LISTENER_FORWARD, MAX_LISTENER_VELOCITY,
};
pub use command::{AudioCommand, AudioCommandSink, ListenerId};
pub use prev::{ListenerPrevPositions, MAX_LOCAL_PLAYERS};
