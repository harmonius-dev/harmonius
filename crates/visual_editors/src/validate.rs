//! Structural validation passes for logic graphs.

use std::collections::{HashMap, HashSet, VecDeque};

use crate::graph_ir::{
    Edge, LogicGraph, NodeId, PinDirection, PinId, PinType,
};

/// Registry placeholder for future domain validators.
#[derive(Debug, Default)]
pub struct NodeRegistry;

/// Aggregate validation output.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ValidationResult {
    pub errors: Vec<String>,
}

/// Validates graphs prior to compilation.
#[derive(Debug, Default)]
pub struct GraphValidator;

impl GraphValidator {
    /// Runs cycle, connectivity, and required-input checks.
    pub fn validate(graph: &LogicGraph, _registry: &NodeRegistry) -> ValidationResult {
        let mut errors = Vec::new();
        if let Some(msg) = detect_data_cycle(graph) {
            errors.push(msg);
        }
        if let Some(msg) = detect_dangling_pin(graph) {
            errors.push(msg);
        }
        if let Some(msg) = detect_missing_required_input(graph) {
            errors.push(msg);
        }
        ValidationResult { errors }
    }
}

fn detect_data_cycle(graph: &LogicGraph) -> Option<String> {
    let mut adj: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
    let mut nodes: HashSet<NodeId> = HashSet::new();
    for e in &graph.edges {
        if !is_data_edge(graph, e) {
            continue;
        }
        nodes.insert(e.source_node);
        nodes.insert(e.target_node);
        adj.entry(e.source_node).or_default().push(e.target_node);
    }
    let mut indeg: HashMap<NodeId, u32> = HashMap::new();
    for n in &nodes {
        indeg.insert(*n, 0);
    }
    for vs in adj.values() {
        for v in vs {
            *indeg.entry(*v).or_insert(0) += 1;
        }
    }
    let mut q: VecDeque<NodeId> = indeg
        .iter()
        .filter(|(_, d)| **d == 0)
        .map(|(n, _)| *n)
        .collect();
    let mut seen = 0u32;
    while let Some(u) = q.pop_front() {
        seen += 1;
        for v in adj.get(&u).into_iter().flatten() {
            if let Some(e) = indeg.get_mut(v) {
                *e -= 1;
                if *e == 0 {
                    q.push_back(*v);
                }
            }
        }
    }
    let total = nodes.len() as u32;
    if seen < total {
        Some("data dependency cycle".to_string())
    } else {
        None
    }
}

fn is_data_edge(graph: &LogicGraph, e: &Edge) -> bool {
    let Some(sp) = find_pin_type(graph, e.source_node, e.source_pin) else {
        return false;
    };
    let Some(tp) = find_pin_type(graph, e.target_node, e.target_pin) else {
        return false;
    };
    matches!(sp, PinType::Data(_) | PinType::Generic(_) | PinType::Wildcard)
        && matches!(tp, PinType::Data(_) | PinType::Generic(_) | PinType::Wildcard)
}

fn find_pin_type(graph: &LogicGraph, node: NodeId, pin: PinId) -> Option<PinType> {
    let n = graph.nodes.iter().find(|n| n.node_id == node)?;
    let p = n.pins.iter().find(|p| p.pin_id == pin)?;
    Some(p.pin_type.clone())
}

fn detect_dangling_pin(graph: &LogicGraph) -> Option<String> {
    for n in &graph.nodes {
        for p in n.pins.iter().filter(|p| p.direction == PinDirection::Out) {
            if matches!(p.pin_type, PinType::Data(_)) {
                let connected = graph.edges.iter().any(|e| {
                    e.source_node == n.node_id && e.source_pin == p.pin_id
                });
                if !connected {
                    return Some("dangling output data pin".to_string());
                }
            }
        }
    }
    None
}

fn detect_missing_required_input(graph: &LogicGraph) -> Option<String> {
    for n in &graph.nodes {
        for p in n.pins.iter().filter(|p| p.direction == PinDirection::In) {
            if matches!(p.pin_type, PinType::Data(_)) {
                let incoming = graph.edges.iter().any(|e| {
                    e.target_node == n.node_id && e.target_pin == p.pin_id
                });
                if !incoming {
                    return Some("missing required data input".to_string());
                }
            }
        }
    }
    None
}

/// Builds an adjacency set for quick reachability queries.
pub fn reachable_nodes(graph: &LogicGraph, roots: &[NodeId]) -> HashSet<NodeId> {
    let mut adj: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
    for e in &graph.edges {
        adj.entry(e.source_node).or_default().push(e.target_node);
        adj.entry(e.target_node).or_default().push(e.source_node);
    }
    let mut out = HashSet::new();
    let mut stack: Vec<NodeId> = roots.to_vec();
    while let Some(u) = stack.pop() {
        if out.insert(u) {
            for v in adj.get(&u).into_iter().flatten() {
                stack.push(*v);
            }
        }
    }
    out
}
