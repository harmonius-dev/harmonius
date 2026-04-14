//! Generic directed graph primitives for Harmonius data systems.
//!
//! Implements the API described in `docs/design/data-systems/directed-graphs.md`
//! (storage, traversals, conditional and ordered variants, weighted paths, multi-graph).

#![deny(clippy::all)]
#![deny(unsafe_code)]
#![allow(missing_docs)]

mod conditional;
mod error;
mod graph;
mod handle_map;
mod multi;
mod ordered;
mod weighted;

pub use conditional::{
    CondEdge, ConditionContext, ConditionExpr, ConditionRegistry, ConditionalGraph,
};
pub use error::{CycleError, GraphError, TransitionError};
pub use graph::{DirectedEdge, DirectedGraph};
pub use handle_map::{HandleMap, NodeId};
pub use multi::{EdgeHandle, MultiGraph};
pub use ordered::OrderedGraph;
pub use weighted::WeightedGraph;
