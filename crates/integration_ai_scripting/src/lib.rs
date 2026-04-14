//! AI behavior ↔ scripting integration primitives.
//!
//! Implements the interface-level contracts from
//! `docs/design/integration/ai-scripting.md` (IR-2.4.x), including
//! `StepResult` → `NodeStatus` adaptation, blackboard ↔ variable bridge, and
//! pointer tables for graph dispatch.
//!
//! ECS system wiring, graph compiler emission, and `.dylib` reload integration
//! stay out of this crate on purpose; see the plan progress file for deferred
//! companion test rows.
#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod adapter;
mod bridge;
mod leaf;
mod runtime;

pub use adapter::{step_to_status, NodeStatus, RuntimeError, StepResult, WaitCondition};
pub use bridge::{BbVarMapping, Blackboard, BlackboardBridge, BlackboardKey, VariableStore};
pub use leaf::{tick_bt_graph_leaf, BtGraphLeaf};
pub use runtime::{
    utility_score, ExecutionContext, FnPtrTable, GoapGraphAction, GraphFn, GraphInstanceState,
    UtilityScoreFn, UtilityScoreTable,
};
