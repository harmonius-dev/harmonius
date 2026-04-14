//! Dense directed graphs with validation and deterministic topological ordering.
#![deny(clippy::all)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

mod conditional;
mod graph;
mod id;
mod ordered;
mod topology_error;
mod traversal;

pub use conditional::{CondEdge, ConditionExpr, ConditionalGraph};
pub use graph::DirectedGraph;
pub use id::{EdgeId, NodeId};
pub use ordered::OrderedGraph;
pub use topology_error::{CycleError, TopologyError};
pub use traversal::{AssetId, GraphTraversalState, HandleMap};
