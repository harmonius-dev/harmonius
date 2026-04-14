#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

//! Bounded, decaying event memory primitives for Harmonius simulation.

mod decay;
mod entry;
mod log;
mod propagation;
mod query;
mod threshold;
mod types;

#[cfg(test)]
#[allow(unsafe_code)]
mod tests;

pub use decay::{DecayCurve, DecayCurveType};
pub use entry::DecayingEntry;
pub use log::EventLog;
pub use propagation::{propagate_entries, PropagationRule};
pub use query::{query_entries, EventLogQuery, LogEventMetadata, TimeRange};
pub use threshold::{check_thresholds, ThresholdAction, ThresholdTrigger};
pub use types::{
    AssetId, Entity, EntryId, EventLogId, EventTypeId, GameplayTag, PredicateId, SpatialDim, TagSet,
};
