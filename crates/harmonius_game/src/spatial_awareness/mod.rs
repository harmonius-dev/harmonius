//! Spatial awareness primitives: sense definitions, geometric volumes, and queries.
//!
//! See `docs/design/simulation/spatial-awareness.md` for the full system design.

mod awareness;
mod query;
mod selection;
mod sense;
mod spatial_index;

pub use awareness::{
    apply_noise_pulse, update_awareness, AwarenessConfig, AwarenessEntry, AwarenessState,
    AwarenessTransition, AwarenessTransitionEvent,
};
pub use query::{query_sense, SenseQueryOptions};
pub use selection::{execute_selection, SelectionQuery, SelectionResult};
pub use sense::{
    AssetId, AwarenessLevel, Entity, FalloffCurve, ScoringFunction, SenseCandidate,
    SenseDefinition, SenseDefinitionId, SenseResult, SenseShape, StringId, TagId, TagSet,
};
pub use spatial_index::{SpatialIndex, SpatialTraversalMode};
