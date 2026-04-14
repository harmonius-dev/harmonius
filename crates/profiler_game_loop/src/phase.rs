/// Identifies a game-loop phase for CPU profiling and budgeting.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Phase {
    /// User input sampling.
    Input,
    /// Network receive.
    NetworkReceive,
    /// Deterministic simulation.
    Simulation,
    /// AI evaluation.
    AiUpdate,
    /// Fixed-timestep physics.
    PhysicsStep,
    /// Animation blending.
    AnimationUpdate,
    /// Render snapshot preparation.
    FrameSnapshot,
    /// Frame teardown and profiler collection.
    FrameEnd,
    /// Extension phase identified by an editor-defined id.
    Custom(u32),
}

impl Phase {
    /// Returns the fixed index used by [`crate::PhaseBudgetTable`].
    pub const fn budget_slot(self) -> usize {
        match self {
            Phase::Input => 0,
            Phase::NetworkReceive => 1,
            Phase::Simulation => 2,
            Phase::AiUpdate => 3,
            Phase::PhysicsStep => 4,
            Phase::AnimationUpdate => 5,
            Phase::FrameSnapshot => 6,
            Phase::FrameEnd => 7,
            Phase::Custom(_) => 8,
        }
    }
}
