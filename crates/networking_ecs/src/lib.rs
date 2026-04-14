//! Networking ↔ ECS integration primitives (replication, deltas, interest, snapshots).
//!
//! This crate implements the integration surface described in
//! `docs/design/integration/networking-ecs.md`. It is intentionally self-contained until the
//! core ECS and networking crates wire in.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod delta_tracker;
mod dormancy;
mod filters;
mod ids;
mod interest;
mod replication;
mod snapshot;
mod wire;

pub use delta_tracker::{DeltaError, DeltaTracker};
pub use dormancy::DormancyManager;
pub use filters::ReplicationFilterTable;
pub use ids::{ConnectionId, Entity, SequenceTick};
pub use interest::{InterestManager, NetworkGrid, RelevantSet, ReplicationWorld};
pub use replication::ReplicationSystem;
pub use snapshot::{
    ChunkRange, MmapArena, ResolvedChunkBytes, Snapshot, SnapshotBuffer, SnapshotIndex,
};
pub use wire::{
    AckMessage, Authority, Baseline, DeltaPayload, DeltaRun, Replicated, ReplicationCondition,
    ReplicationFilterId,
};
