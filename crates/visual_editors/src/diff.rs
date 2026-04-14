//! Two-way diff and three-way merge for graph assets.

use std::collections::{HashMap, HashSet};

use crate::graph_ir::{LogicGraph, NodeId};

/// Node-level change list between two revisions.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct GraphDiff {
    pub added: Vec<NodeId>,
    pub removed: Vec<NodeId>,
}

/// Result of a three-way merge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MergeResult {
    Merged,
    Conflict {
        node: NodeId,
        reason: String,
    },
}

/// Computes structural diffs keyed by [`NodeId`].
#[derive(Debug, Default)]
pub struct GraphDiffer;

impl GraphDiffer {
    /// Two-way structural diff.
    pub fn diff(base: &LogicGraph, head: &LogicGraph) -> GraphDiff {
        let base_ids: HashMap<NodeId, ()> = base.nodes.iter().map(|n| (n.node_id, ())).collect();
        let head_ids: HashMap<NodeId, ()> = head.nodes.iter().map(|n| (n.node_id, ())).collect();
        let added: Vec<NodeId> = head_ids
            .keys()
            .filter(|id| !base_ids.contains_key(id))
            .copied()
            .collect();
        let removed: Vec<NodeId> = base_ids
            .keys()
            .filter(|id| !head_ids.contains_key(id))
            .copied()
            .collect();
        GraphDiff { added, removed }
    }

    /// Khanna-style merge: prefer ours on direct node collision.
    pub fn merge_three_way(
        _base: &LogicGraph,
        ours: &LogicGraph,
        theirs: &LogicGraph,
    ) -> MergeResult {
        let ours_ids: HashSet<_> = ours.nodes.iter().map(|n| n.node_id).collect();
        let theirs_ids: HashSet<_> = theirs.nodes.iter().map(|n| n.node_id).collect();
        for id in ours_ids.intersection(&theirs_ids) {
            let on = ours.nodes.iter().find(|n| n.node_id == *id);
            let tn = theirs.nodes.iter().find(|n| n.node_id == *id);
            if on.zip(tn).is_some_and(|(a, b)| a.kind != b.kind) {
                return MergeResult::Conflict {
                    node: *id,
                    reason: "kind mismatch".to_string(),
                };
            }
        }
        MergeResult::Merged
    }
}
