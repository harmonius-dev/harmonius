//! Find-usages and rename helpers for graph assets.

use crate::graph_ir::{LogicGraph, NodeKind};

/// Returns node ids whose stub name references `symbol`.
pub fn find_usages(graph: &LogicGraph, symbol: &str) -> Vec<crate::graph_ir::NodeId> {
    graph
        .nodes
        .iter()
        .filter(|n| match &n.kind {
            NodeKind::Stub { name } => name.contains(symbol),
            _ => false,
        })
        .map(|n| n.node_id)
        .collect()
}

/// Renames stub nodes containing `old` substring.
pub fn rename_symbol(graph: &mut LogicGraph, old: &str, new: &str) {
    for n in &mut graph.nodes {
        if let NodeKind::Stub { name } = &mut n.kind {
            if name.contains(old) {
                *name = name.replace(old, new);
            }
        }
    }
    for v in &mut graph.variables {
        if v.name.contains(old) {
            v.name = v.name.replace(old, new);
        }
    }
}
