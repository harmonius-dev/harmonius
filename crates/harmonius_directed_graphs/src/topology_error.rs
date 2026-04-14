use crate::NodeId;

/// Nodes forming a directed cycle, including the closing repeat.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CycleError {
    /// Forward walk through the cycle, ending where it began.
    pub path: Vec<NodeId>,
}

/// Structural problems detected before scripting compilation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TopologyError {
    /// At least one directed cycle exists.
    CycleDetected(CycleError),
    /// An edge references a node id outside the graph.
    UnknownNode(NodeId),
    /// A directed edge starts and ends on the same node.
    SelfLoop(NodeId),
}
