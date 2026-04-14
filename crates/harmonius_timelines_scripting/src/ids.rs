//! Strongly-typed identifiers for the timelines ↔ scripting boundary.

/// ECS entity handle (integration tests use opaque ids).
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    Ord,
    PartialOrd,
)]
pub struct Entity(pub u64);

/// Fixed-tick time primitive (one tick = one fixed simulation step).
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    Ord,
    PartialOrd,
)]
pub struct TickCount(pub u64);

/// Strongly-typed event identifier for `WaitCondition::Event`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct EventId(pub u64);

/// Timeline track id (canonical in timelines subsystem).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TrackId(pub u16);

/// Script variable slot id (canonical in scripting subsystem).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SlotId(pub u16);

/// Keyframe index within a track asset.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct KeyframeId(pub u32);

/// Stable graph identity for diagnostics (FM-1 warnings).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GraphId(pub u64);

/// Sentinel: distinguish fresh instance from node id `0`.
pub const GRAPH_STEP_NOT_STARTED: u32 = u32::MAX;

/// Default wait for `WaitCondition::TimelineComplete` when `timeout` is `None`.
pub const DEFAULT_WAIT_TIMEOUT: TickCount = TickCount(600);
