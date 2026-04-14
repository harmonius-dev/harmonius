//! AI behavior ↔ event logs integration primitives.
//!
//! This crate captures the contracts from `docs/design/integration/ai-event-logs.md` as
//! dependency-free, testable logic (bounded logs, queries, thresholds, propagation buffer, and
//! rkyv-backed decision entries).

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod ai_decision;
mod blackboard;
mod bt_memory;
mod channels;
mod debug_flags;
mod event_log;
mod goap;
mod ids;
mod propagation;
mod threshold;
mod utility;

pub use ai_decision::{write_ai_decision, AiDecisionEntry, AiDecisionSource};
pub use blackboard::{Blackboard, BlackboardKey, BlackboardScope, BlackboardValue};
pub use bt_memory::{apply_bt_event_memory_check, BtEventMemoryCheck};
pub use channels::{ThresholdChannel, THRESHOLD_CHANNEL_CAP};
pub use debug_flags::EventLogDebugFlags;
pub use event_log::{
    DecayingEntry, EventLog, EventLogQuery, EventPredicate, PredicateId, PredicateTable,
    QueryContext, QueryWarnings, TimeRange,
};
pub use goap::GoapThreatBits;
pub use ids::{ActionId, Entity};
pub use propagation::PropagationBuffer;
pub use threshold::{evaluate_threshold_trigger, ThresholdFired, ThresholdTrigger};
pub use utility::{score_event_log_consideration, EventLogConsideration, ResponseCurve};

#[cfg(test)]
mod tests_ir_2_2;
