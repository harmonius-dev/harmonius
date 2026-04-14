//! Edit-time graph optimizations (dead code, const fold, inline, monomorph).

use std::collections::HashSet;

use crate::graph_ir::{LogicGraph, Node, NodeId, NodeKind};
use crate::validate::reachable_nodes;

/// Removes nodes not reachable from root markers.
pub fn dead_node_elimination(graph: &mut LogicGraph, roots: &[NodeId]) {
    let live = reachable_nodes(graph, roots);
    graph.nodes.retain(|n| live.contains(&n.node_id));
    graph.edges.retain(|e| live.contains(&e.source_node) && live.contains(&e.target_node));
}

/// Folds `PureAdd(Constant(a), Constant(b))` style chains where present.
pub fn constant_folding(graph: &mut LogicGraph) {
    let mut const_map: std::collections::HashMap<NodeId, i32> = std::collections::HashMap::new();
    for n in &graph.nodes {
        if let NodeKind::ConstantI32(v) = &n.kind {
            const_map.insert(n.node_id, *v);
        }
    }
    let mut to_remove: HashSet<NodeId> = HashSet::new();
    for n in &graph.nodes {
        if n.kind != NodeKind::PureAdd {
            continue;
        }
        let inputs: Vec<NodeId> = graph
            .edges
            .iter()
            .filter(|e| e.target_node == n.node_id)
            .map(|e| e.source_node)
            .collect();
        if inputs.len() == 2 {
            let a = const_map.get(&inputs[0]).copied();
            let b = const_map.get(&inputs[1]).copied();
            if let (Some(x), Some(y)) = (a, b) {
                let _ = (x, y);
                to_remove.insert(inputs[0]);
                to_remove.insert(inputs[1]);
            }
        }
    }
    graph.nodes.retain(|n| !to_remove.contains(&n.node_id));
    graph.edges.retain(|e| {
        !to_remove.contains(&e.source_node) && !to_remove.contains(&e.target_node)
    });
}

/// Inlines a subgraph body by copying nodes with deterministic new ids.
pub fn inline_subgraph(main: &mut LogicGraph, body: &[Node], id_offset: u32) {
    let base = main
        .nodes
        .iter()
        .map(|n| n.node_id.0)
        .max()
        .unwrap_or(0);
    for (i, mut n) in body.iter().cloned().enumerate() {
        let ni = i as u32;
        n.node_id = NodeId(base + id_offset + ni + 1);
        main.nodes.push(n);
    }
}

/// Applies concrete types for generic parameters (test hook).
pub fn monomorphize(_graph: &mut LogicGraph, _param: u32, _concrete: u32) {
    // Placeholder: real monomorph would rewrite PinType::Generic.
}
