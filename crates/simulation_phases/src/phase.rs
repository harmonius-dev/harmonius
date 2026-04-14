//! Top-level game loop phase labels from the core runtime design.

/// Built-in game loop phases.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Phase {
    /// Drain input, map raw input to actions.
    Input,
    /// Process incoming network packets.
    NetworkReceive,
    /// Fixed-timestep simulation tick.
    Simulation,
    /// AI evaluation and navigation.
    AiUpdate,
    /// Fixed-timestep physics substeps.
    PhysicsStep,
    /// Animation state machines and procedural work.
    AnimationUpdate,
    /// Build immutable render snapshot.
    FrameSnapshot,
    /// Save hooks, stats, bookkeeping.
    FrameEnd,
    /// User-defined phase with explicit ordering relative to built-ins.
    ///
    /// [`crate::runner::run_scheduled_systems`] follows [`Self::default_pipeline`] only, so systems
    /// registered for [`Phase::Custom`] are not invoked by that test helper until scheduling APIs
    /// accept a custom phase list.
    Custom(u32),
}

impl Phase {
    /// Canonical single-frame ordering for the default eight-phase pipeline.
    #[must_use]
    pub const fn default_pipeline() -> &'static [Phase] {
        &[
            Phase::Input,
            Phase::NetworkReceive,
            Phase::Simulation,
            Phase::AiUpdate,
            Phase::PhysicsStep,
            Phase::AnimationUpdate,
            Phase::FrameSnapshot,
            Phase::FrameEnd,
        ]
    }
}
