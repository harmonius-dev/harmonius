//! Networking ↔ physics integration primitives.
//!
//! Implements the data contracts from `docs/design/integration/networking-physics.md`:
//! ring buffers for snapshots and hitboxes, reconciliation checks, interpolation,
//! extrapolation, lag-compensation rewind helpers, and client-side prediction scaffolding.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod buffers;
pub mod determinism;
mod entity;
mod error_corrector;
mod history_rewind;
mod input;
mod interpolate;
mod predictor;
mod reconciler;
mod snapshot_types;

pub use buffers::{HitboxBuffer, MAX_REWIND_TICKS, MAX_ROLLBACK_TICKS, SnapshotBuffer};
pub use determinism::{SolverBodyKey, sort_solver_body_keys};
pub use entity::Entity;
pub use error_corrector::{CorrectedState, ErrorCorrector};
pub use history_rewind::HistoryRewinder;
pub use input::InputFrame;
pub use interpolate::{ExtrapolatedState, Extrapolator, InterpolatedState, SnapshotInterpolator};
pub use predictor::ClientPredictor;
pub use reconciler::{RollbackOutcome, ServerReconciler};
pub use snapshot_types::{
    ColliderShape, ContactRange, ConvexMeshHandle, HeightfieldHandle, HitboxSnapshot,
    PhysicsSnapshot, TriMeshHandle,
};
