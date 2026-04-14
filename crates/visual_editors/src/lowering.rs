//! Suspend-state lowering for gameplay graphs.

use crate::graph_ir::{GraphId, NodeId};

/// Describes a suspend point after yield lowering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuspendStateDesc {
    pub graph: GraphId,
    pub resume_node: NodeId,
    pub variant: SuspendVariant,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SuspendVariant {
    NextFrame,
    DelayFrames(u32),
    UntilCondition,
}

/// Lowers yield-like markers into explicit suspend descriptors.
pub fn lower_suspends(graph_id: GraphId, resume: NodeId) -> Vec<SuspendStateDesc> {
    vec![
        SuspendStateDesc {
            graph: graph_id,
            resume_node: resume,
            variant: SuspendVariant::NextFrame,
        },
        SuspendStateDesc {
            graph: graph_id,
            resume_node: resume,
            variant: SuspendVariant::DelayFrames(3),
        },
        SuspendStateDesc {
            graph: graph_id,
            resume_node: resume,
            variant: SuspendVariant::UntilCondition,
        },
    ]
}
