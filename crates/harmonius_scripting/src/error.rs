//! Graph and compilation errors for the scripting ↔ directed-graph boundary.

pub use harmonius_directed_graphs::CycleError;

use harmonius_directed_graphs::{EdgeId, NodeId};

use crate::types::ScriptTypeId;

/// Integration-level error surface (IR-2.7.x).
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GraphError {
    /// A cycle was detected during validation.
    CycleDetected(CycleError),
    /// A directed self-edge on one node.
    SelfLoop(NodeId),
    /// A referenced node handle is not present.
    NodeNotFound(NodeId),
    /// Pin types disagree across an edge.
    EdgeTypeMismatch {
        /// Offending edge.
        edge: EdgeId,
        /// Source output type.
        source: ScriptTypeId,
        /// Target input type.
        target: ScriptTypeId,
    },
    /// Hot-reload left traversal state on an older program version.
    StaleProgram {
        /// Version the instance expected.
        expected: u32,
        /// Version found on the program.
        found: u32,
    },
}
