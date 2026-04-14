//! Change-detection markers aligned with `docs/design/simulation/game-loop-phases.md`.
//!
//! The design attaches [`PrimitiveTickCompleted`] as an ECS `Component` on the engine side. This
//! crate defines the portable shape only; runtime wiring lives in the ECS integration layer.

/// Identifies which simulation primitive completed a tick that mutates shared state.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PrimitiveId {
    /// Timeline playback and keyframe emission.
    Timeline,
    /// Grid and volume propagation.
    Grid,
    /// Event log decay and neighbor propagation.
    EventLog,
    /// Shared spatial index rebuild.
    SpatialIndex,
}

/// Marker for a completed primitive tick (see design “Change Detection Contract”).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrimitiveTickCompleted(pub PrimitiveId);
