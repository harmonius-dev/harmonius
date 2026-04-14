//! Deterministic frame phase ordering (R-16.5.4).

/// Ordered phases from the composition design.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum FramePhase {
    /// Event log ingest.
    EventLogIngest = 0,
    /// Timeline tick.
    TimelineTick = 1,
    /// Spatial awareness update.
    SpatialAwareness = 2,
    /// Grid propagation.
    GridPropagation = 3,
    /// Container mutation.
    ContainerMutation = 4,
    /// Directed graph advance.
    DirectedGraphAdvance = 5,
    /// Attribute aggregation.
    AttributeAggregation = 6,
    /// Effect tick.
    EffectTick = 7,
    /// Event log commit.
    EventLogCommit = 8,
}

impl FramePhase {
    /// All phases in canonical ascending order.
    pub const ORDER: [FramePhase; 9] = [
        FramePhase::EventLogIngest,
        FramePhase::TimelineTick,
        FramePhase::SpatialAwareness,
        FramePhase::GridPropagation,
        FramePhase::ContainerMutation,
        FramePhase::DirectedGraphAdvance,
        FramePhase::AttributeAggregation,
        FramePhase::EffectTick,
        FramePhase::EventLogCommit,
    ];
}
