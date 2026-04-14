//! Minimal directed graph used for formula assets and dependency analysis.

use crate::types::FormulaId;

/// Edge identifier between nodes (dense `u32`).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EdgeId(pub u32);

/// Node identifier inside a graph (dense `u32`).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NodeId(pub u32);

/// Directed edge with payloads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Edge<EdgePayload> {
    /// Source node.
    pub from: NodeId,
    /// Destination node.
    pub to: NodeId,
    /// Authoring payload carried on the edge.
    pub payload: EdgePayload,
}

/// Directed graph stored as explicit node and edge lists.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectedGraph<NodePayload, EdgePayload> {
    /// Parallel node payloads indexed by `NodeId.0`.
    pub nodes: Vec<NodePayload>,
    /// All directed edges.
    pub edges: Vec<Edge<EdgePayload>>,
}

impl<NodePayload, EdgePayload> DirectedGraph<NodePayload, EdgePayload> {
    /// Returns a new empty graph.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
}

impl Default for DirectedGraph<(), ()> {
    fn default() -> Self {
        Self::new()
    }
}

/// Maps a `NodeId` to a `FormulaId` for dependency extraction in tests.
pub fn node_to_formula_id(node: NodeId) -> FormulaId {
    FormulaId(node.0)
}
