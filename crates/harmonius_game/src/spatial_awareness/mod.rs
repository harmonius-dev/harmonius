//! Spatial awareness primitives: sense definitions, geometric volumes, and queries.
//!
//! See `docs/design/simulation/spatial-awareness.md` for the full system design.

mod query;
mod sense;

pub use query::query_sense;
pub use sense::{
    AssetId, AwarenessLevel, Entity, FalloffCurve, ScoringFunction, SenseCandidate,
    SenseDefinition, SenseDefinitionId, SenseResult, SenseShape, StringId, TagId, TagSet,
};
