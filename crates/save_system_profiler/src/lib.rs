#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

//! Save system ↔ profiler integration contracts (IR-8.1.x, CH-24).
//!
//! Provides bounded profile transport, typed events, and profiler aggregation helpers used by
//! integration tests and future engine wiring.

pub mod aggregator;
pub mod channel;
pub mod harness;
pub mod types;

pub use aggregator::{
    hud_phase8_bar_red, schema_bytes_sum, schema_totals_map, SaveProfilerAggregator,
    DURATION_WINDOW_CAP, PHASE8_SAVE_BUDGET_MS,
};
pub use channel::SaveProfileChannel;
pub use harness::{drain_messages, FakeFileSystem, SavePipeline, NODE_AI, NODE_PLAYER, NODE_WORLD};
pub use types::{
    DebugFlags, IoError, IoWriteEvent, MemorySnapshotEvent, ProfileMessage, SaveMetrics, SavePhase,
    SaveProfileEvent, SchemaBreakdownEvent, SchemaNodeId,
};
