//! Visual logic graph scripting and directed-graph integration surface.
#![deny(clippy::all)]
#![forbid(unsafe_code)]

pub mod types;
pub mod error;
mod compiler;
mod runtime;

pub use compiler::GraphCompiler;
pub use error::{CycleError, GraphError};
pub use harmonius_directed_graphs::{
    CondEdge, ConditionExpr, ConditionalGraph, DirectedGraph, EdgeId, GraphTraversalState, NodeId,
    OrderedGraph, TopologyError,
};
pub use runtime::{
    step_graph, traversal_to_step, GraphExecutionSystem, GraphInstance, GraphProgram, StepOutcome,
    GRAPH_STEP_NOT_STARTED,
};
pub use types::{EdgePayload, LogicGraph, NodeOpId, NodePayload, ScriptTypeId};
