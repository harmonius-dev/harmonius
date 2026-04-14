//! Minimal PCG graph validation (cycle + pin typing) used until `GraphRuntime` lands in core.

use std::collections::{HashMap, HashSet};

/// Pin data kinds carried on procedural graph edges.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PcgDataType {
    /// Point cloud stream.
    PointSet,
    /// Scalar parameter stream.
    Scalar,
}

/// Stable node identifier inside an authored graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PcgNodeId(pub u32);

/// Stable edge identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PcgEdgeId(pub u32);

/// Authoring-time node with named pins and types.
#[derive(Debug, Clone)]
pub struct PcgGraphNode {
    /// Node id.
    pub id: PcgNodeId,
    /// Output pin name → type.
    pub outputs: Vec<(&'static str, PcgDataType)>,
    /// Input pin name → type.
    pub inputs: Vec<(&'static str, PcgDataType)>,
}

/// Directed data edge between pins.
#[derive(Debug, Clone)]
pub struct PcgGraphEdge {
    /// Edge id.
    pub id: PcgEdgeId,
    /// Source node and output pin name.
    pub from: (PcgNodeId, &'static str),
    /// Target node and input pin name.
    pub to: (PcgNodeId, &'static str),
}

/// Validation failures surfaced to editors and compilers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PcgGraphError {
    /// Directed cycle breaks DAG requirement.
    CycleDetected,
    /// Pin types disagree across an edge.
    TypeMismatch,
}

fn pin_type(
    nodes: &[PcgGraphNode],
    node: PcgNodeId,
    pin: &str,
    output_side: bool,
) -> Option<PcgDataType> {
    let n = nodes.iter().find(|x| x.id == node)?;
    let table: &[(&str, PcgDataType)] = if output_side { &n.outputs } else { &n.inputs };
    table.iter().find(|(name, _)| *name == pin).map(|(_, t)| *t)
}

/// Validates nodes and edges together (DAG + pin typing).
pub fn validate_full(nodes: &[PcgGraphNode], edges: &[PcgGraphEdge]) -> Result<(), PcgGraphError> {
    let mut adj: HashMap<PcgNodeId, Vec<PcgNodeId>> = HashMap::new();
    for e in edges {
        adj.entry(e.from.0).or_default().push(e.to.0);
    }

    // Kahn topological sort with cycle detection.
    let mut indeg: HashMap<PcgNodeId, u32> = HashMap::new();
    for n in nodes {
        indeg.entry(n.id).or_insert(0);
    }
    for e in edges {
        *indeg.entry(e.to.0).or_insert(0) += 1;
    }
    let mut queue: Vec<PcgNodeId> = indeg
        .iter()
        .filter(|(_, d)| **d == 0)
        .map(|(id, _)| *id)
        .collect();
    queue.sort_by_key(|id| id.0);
    let mut seen = 0u32;
    while let Some(u) = queue.pop() {
        seen += 1;
        if let Some(out) = adj.get(&u) {
            for v in out {
                let e = indeg.entry(*v).or_insert(0);
                *e -= 1;
                if *e == 0 {
                    queue.push(*v);
                }
            }
        }
        queue.sort_by_key(|id| id.0);
    }
    if seen != nodes.len() as u32 {
        return Err(PcgGraphError::CycleDetected);
    }

    for e in edges {
        let out_ty =
            pin_type(nodes, e.from.0, e.from.1, true).ok_or(PcgGraphError::TypeMismatch)?;
        let in_ty = pin_type(nodes, e.to.0, e.to.1, false).ok_or(PcgGraphError::TypeMismatch)?;
        if out_ty != in_ty {
            return Err(PcgGraphError::TypeMismatch);
        }
    }

    Ok(())
}

/// Linearize evaluation order for a DAG (Kahn output order).
pub fn topo_order(nodes: &[PcgGraphNode], edges: &[PcgGraphEdge]) -> Vec<PcgNodeId> {
    let mut adj: HashMap<PcgNodeId, Vec<PcgNodeId>> = HashMap::new();
    for e in edges {
        adj.entry(e.from.0).or_default().push(e.to.0);
    }
    let mut indeg: HashMap<PcgNodeId, u32> = HashMap::new();
    for n in nodes {
        indeg.entry(n.id).or_insert(0);
    }
    for e in edges {
        *indeg.entry(e.to.0).or_insert(0) += 1;
    }
    let mut queue: Vec<PcgNodeId> = indeg
        .iter()
        .filter(|(_, d)| **d == 0)
        .map(|(id, _)| *id)
        .collect();
    let mut order = Vec::new();
    while let Some(u) = queue.pop() {
        order.push(u);
        if let Some(out) = adj.get(&u) {
            for v in out {
                let e = indeg.entry(*v).or_insert(0);
                *e -= 1;
                if *e == 0 {
                    queue.push(*v);
                }
            }
        }
    }
    order
}

/// Multiply every point coordinate by `scale` (demo transform).
pub fn eval_scale_points(input: &[(f32, f32, f32)], scale: f32) -> Vec<(f32, f32, f32)> {
    input
        .iter()
        .map(|(x, y, z)| (x * scale, y * scale, z * scale))
        .collect()
}

/// Evaluate a simple two-node chain: identity → scale, or subgraph with same semantics.
pub fn eval_inline_scale_chain(base: &[(f32, f32, f32)], scale: f32) -> Vec<(f32, f32, f32)> {
    eval_scale_points(base, scale)
}

/// Subgraph exposes one PointSet in and one PointSet out applying `scale`.
pub fn eval_subgraph_scale(base: &[(f32, f32, f32)], scale: f32) -> Vec<(f32, f32, f32)> {
    eval_scale_points(base, scale)
}

/// Checks that a set of directed edges contains a cycle reachable from any node.
pub fn graph_has_cycle(edges: &[(PcgNodeId, PcgNodeId)]) -> bool {
    let mut adj: HashMap<PcgNodeId, Vec<PcgNodeId>> = HashMap::new();
    let mut nodes = HashSet::new();
    for (a, b) in edges {
        nodes.insert(*a);
        nodes.insert(*b);
        adj.entry(*a).or_default().push(*b);
    }
    fn dfs(
        u: PcgNodeId,
        adj: &HashMap<PcgNodeId, Vec<PcgNodeId>>,
        color: &mut HashMap<PcgNodeId, u8>,
    ) -> bool {
        match color.get(&u).copied().unwrap_or(0) {
            2 => return false,
            1 => return true,
            _ => {}
        }
        color.insert(u, 1);
        if let Some(nbrs) = adj.get(&u) {
            for v in nbrs {
                if dfs(*v, adj, color) {
                    return true;
                }
            }
        }
        color.insert(u, 2);
        false
    }
    let mut color = HashMap::new();
    for n in nodes {
        if color.get(&n).copied().unwrap_or(0) == 0 && dfs(n, &adj, &mut color) {
            return true;
        }
    }
    false
}
