use crate::NodeId;

/// Error returned by graph validation and mutation operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphError {
    /// Adding an edge would introduce a directed cycle.
    CycleDetected(CycleError),
    /// Referenced node is not present in the graph.
    NodeNotFound(NodeId),
    /// A second edge with the same endpoints is not allowed on [`crate::DirectedGraph`].
    DuplicateEdge {
        /// Source endpoint.
        from: NodeId,
        /// Target endpoint.
        to: NodeId,
    },
    /// Self-loop rejected as a degenerate cycle.
    SelfLoop(NodeId),
}

/// Describes one directed cycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CycleError {
    /// Nodes on the cycle, listed in forward edge order starting at the smallest [`NodeId`].
    pub cycle_path: Vec<NodeId>,
}

/// Errors when mutating [`crate::GraphTraversalState`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransitionError<S> {
    /// Unknown node handle.
    InvalidNode(NodeId),
    /// Illegal status change for the domain.
    InvalidTransition {
        /// Affected node.
        node: NodeId,
        /// Previous status.
        from: S,
        /// Requested status.
        to: S,
    },
    /// Graph already marked complete.
    AlreadyComplete,
}
