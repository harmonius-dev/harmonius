//! Networking ↔ ECS integration primitives (replication, deltas, interest, snapshots).
//!
//! This crate tracks the repository design at `docs/design/integration/networking-ecs.md`. Phase 1
//! covers dense
//! baselines and XOR/RLE deltas (IR-4.4.2), brute-force interest over a [`NetworkGrid`]
//! (IR-4.4.3), indexed snapshots (IR-4.4.5), dormancy (IR-4.4.6), and bounded ACK plumbing
//! (IR-4.4.8 subset). IR-4.4.1, IR-4.4.4, IR-4.4.7, full reconciliation, and grid-neighbor AOI
//! remain for follow-up once core ECS and transport types integrate.

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
