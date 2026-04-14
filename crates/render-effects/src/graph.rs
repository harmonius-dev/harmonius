//! Node post-process graph compilation (TC-2.9.14.1).

/// Identifier for a graph node.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(pub u32);

/// Directed edge `from -> to`.
#[derive(Clone, Copy, Debug)]
pub struct Edge {
    /// Source node.
    pub from: NodeId,
    /// Destination node.
    pub to: NodeId,
}

/// Compiles a pass order via Kahn topological sort with stable ID ordering.
pub fn compile_post_graph(nodes: &[NodeId], edges: &[Edge]) -> Vec<NodeId> {
    let mut indeg: std::collections::HashMap<NodeId, usize> =
        nodes.iter().map(|&n| (n, 0)).collect();
    for e in edges {
        *indeg.entry(e.to).or_insert(0) += 1;
    }
    let mut order_key: Vec<NodeId> = nodes.to_vec();
    order_key.sort_by_key(|n| n.0);
    let mut out = Vec::new();
    let mut done = std::collections::HashSet::new();
    while out.len() < nodes.len() {
        let next = order_key.iter().copied().find(|n| {
            *indeg.get(n).unwrap_or(&1) == 0 && !done.contains(n)
        });
        let Some(n) = next else {
            break;
        };
        out.push(n);
        done.insert(n);
        for e in edges {
            if e.from == n {
                if let Some(v) = indeg.get_mut(&e.to) {
                    *v = v.saturating_sub(1);
                }
            }
        }
    }
    out
}
