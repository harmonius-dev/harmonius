//! Audio–physics integration: collision impact, sliding friction, and trigger-zone commands.
//!
//! Implements the contracts in `docs/design/integration/audio-physics.md` as deterministic,
//! queue-testable logic without ECS wiring (phase-1 logic crate; ECS observers land upstream).
//!
//! The command queue uses a [`std::sync::Mutex`] so multiple producers can call [`send`](
//! crate::BoundedAudioCommandQueue::send) concurrently; it preserves the design’s
//! lowest-priority eviction policy. A future swap to `crossbeam_queue::ArrayQueue` can follow
//! once overflow handling maps cleanly to a pure ring. Persistent asset tables match the design’s
//! fixed-array layout; `rkyv` derives are expected when this crate shares the workspace’s baked
//! asset handle types.

#![deny(clippy::all)]
#![deny(unsafe_code)]
#![warn(missing_docs)]

mod audio_thread;
mod bridge;
mod command_queue;
mod commands;
mod cooldown;
mod events;
mod friction_track;
mod ids;
mod math;
mod surface;
mod tables;
mod zones;

#[cfg(test)]
mod integration_tests;

pub use audio_thread::apply_set_param_wire;
pub use bridge::{
    handle_collision_impact, handle_sliding_friction, handle_trigger_zone, AmbientVoiceMap,
};
pub use command_queue::BoundedAudioCommandQueue;
pub use commands::{AudioCommand, AudioTimestamp, BusId, VoiceParam, VoicePriority};
pub use cooldown::{CooldownSlot, ImpactCooldownTracker};
pub use events::{
    CollisionEnded, CollisionPersisted, CollisionStarted, ContactPoint, TriggerEnter, TriggerExit,
};
pub use friction_track::{ActiveFrictionSounds, FrictionSlot};
pub use ids::{AssetHandle, AudioClip, Entity, VoiceId, VoiceIdAllocator};
pub use math::Vec3;
pub use surface::{SurfaceType, SURFACE_TYPE_COUNT};
pub use tables::{FrictionSoundTable, ImpactSoundSet, ImpactSoundTable};
pub use zones::{AmbientLoop, ReverbParams, ReverbZone, ReverbZoneId, TriggerZoneSnapshot};
