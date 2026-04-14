//! Minimal logic-graph compiler for directed-graph integration tests.

use std::collections::BTreeSet;

use harmonius_directed_graphs::{
    CondEdge, ConditionExpr, DirectedGraph, EdgeId, NodeId, OrderedGraph, TopologyError,
};

use crate::error::GraphError;
use crate::types::{EdgePayload, NodePayload, ScriptTypeId};

/// Emits deterministic Rust source for a logic graph.
#[derive(Debug, Default)]
pub struct GraphCompiler;

impl GraphCompiler {
    /// Compile a plain `LogicGraph` using optional sibling ordering.
    pub fn compile(
        graph: &DirectedGraph<NodePayload, EdgePayload>,
        ordered: Option<&OrderedGraph<NodePayload, EdgePayload>>,
    ) -> Result<String, GraphError> {
        graph.validate().map_err(GraphError::from)?;
        Self::typecheck_plain(graph)?;
        let order = Self::emit_order(graph, ordered)?;
        Ok(Self::emit_plain_body(&order))
    }

    /// Compile a graph whose edges carry conditional guards.
    pub fn compile_conditional(
        graph: &DirectedGraph<NodePayload, CondEdge<EdgePayload>>,
        ordered: Option<&OrderedGraph<NodePayload, CondEdge<EdgePayload>>>,
    ) -> Result<String, GraphError> {
        graph.validate().map_err(GraphError::from)?;
        Self::typecheck_conditional(graph)?;
        let order = Self::emit_order_conditional(graph, ordered)?;
        Ok(Self::emit_conditional_body(graph, &order))
    }

    fn typecheck_plain(graph: &DirectedGraph<NodePayload, EdgePayload>) -> Result<(), GraphError> {
        for (eid, from, to, edge) in Self::for_each_edge(graph) {
            let from_node = graph.get_node(from).ok_or(GraphError::NodeNotFound(from))?;
            let to_node = graph.get_node(to).ok_or(GraphError::NodeNotFound(to))?;
            Self::check_edge_payload(eid, from_node, to_node, edge)?;
        }
        Ok(())
    }

    fn typecheck_conditional(
        graph: &DirectedGraph<NodePayload, CondEdge<EdgePayload>>,
    ) -> Result<(), GraphError> {
        for (eid, from, to, cedge) in Self::for_each_edge_conditional(graph) {
            let from_node = graph.get_node(from).ok_or(GraphError::NodeNotFound(from))?;
            let to_node = graph.get_node(to).ok_or(GraphError::NodeNotFound(to))?;
            Self::check_edge_payload(eid, from_node, to_node, &cedge.data)?;
        }
        Ok(())
    }

    fn check_edge_payload(
        eid: EdgeId,
        from_node: &NodePayload,
        to_node: &NodePayload,
        edge: &EdgePayload,
    ) -> Result<(), GraphError> {
        let src_ty = *from_node
            .output_types
            .get(edge.source_pin as usize)
            .unwrap_or(&ScriptTypeId::UNKNOWN);
        let dst_ty = *to_node
            .input_types
            .get(edge.target_pin as usize)
            .unwrap_or(&ScriptTypeId::UNKNOWN);
        if src_ty != dst_ty {
            return Err(GraphError::EdgeTypeMismatch {
                edge: eid,
                source: src_ty,
                target: dst_ty,
            });
        }
        if edge.data_type != src_ty {
            return Err(GraphError::EdgeTypeMismatch {
                edge: eid,
                source: src_ty,
                target: edge.data_type,
            });
        }
        Ok(())
    }

    fn for_each_edge(
        graph: &DirectedGraph<NodePayload, EdgePayload>,
    ) -> impl Iterator<Item = (EdgeId, NodeId, NodeId, &EdgePayload)> + '_ {
        (0..graph.node_count()).flat_map(move |i| {
            let u = NodeId(i as u32);
            graph
                .out_edges(u)
                .map(move |(eid, t, ed)| (eid, u, t, ed))
        })
    }

    fn for_each_edge_conditional(
        graph: &DirectedGraph<NodePayload, CondEdge<EdgePayload>>,
    ) -> impl Iterator<Item = (EdgeId, NodeId, NodeId, &CondEdge<EdgePayload>)> + '_ {
        (0..graph.node_count()).flat_map(move |i| {
            let u = NodeId(i as u32);
            graph
                .out_edges(u)
                .map(move |(eid, t, ed)| (eid, u, t, ed))
        })
    }

    fn children_ordered_plain(
        graph: &DirectedGraph<NodePayload, EdgePayload>,
        ordered: Option<&OrderedGraph<NodePayload, EdgePayload>>,
        parent: NodeId,
    ) -> Vec<NodeId> {
        if let Some(o) = ordered {
            o.ordered_children(parent).into_owned()
        } else {
            let mut v: Vec<NodeId> = graph.out_edges(parent).map(|(_, t, _)| t).collect();
            v.sort_unstable();
            v
        }
    }

    fn children_ordered_conditional(
        graph: &DirectedGraph<NodePayload, CondEdge<EdgePayload>>,
        ordered: Option<&OrderedGraph<NodePayload, CondEdge<EdgePayload>>>,
        parent: NodeId,
    ) -> Vec<NodeId> {
        if let Some(o) = ordered {
            o.ordered_children(parent).into_owned()
        } else {
            let mut v: Vec<NodeId> = graph.out_edges(parent).map(|(_, t, _)| t).collect();
            v.sort_unstable();
            v
        }
    }

    fn emit_order(
        graph: &DirectedGraph<NodePayload, EdgePayload>,
        ordered: Option<&OrderedGraph<NodePayload, EdgePayload>>,
    ) -> Result<Vec<NodeId>, GraphError> {
        if graph.node_count() == 0 {
            return Ok(Vec::new());
        }
        let topo = graph.topological_sort().map_err(GraphError::from)?;
        let entry = *topo.first().unwrap_or(&NodeId(0));
        let mut out = Vec::new();
        let mut seen = BTreeSet::new();
        Self::dfs_plain(graph, ordered, entry, &mut seen, &mut out);
        Ok(out)
    }

    fn dfs_plain(
        graph: &DirectedGraph<NodePayload, EdgePayload>,
        ordered: Option<&OrderedGraph<NodePayload, EdgePayload>>,
        u: NodeId,
        seen: &mut BTreeSet<NodeId>,
        out: &mut Vec<NodeId>,
    ) {
        if !seen.insert(u) {
            return;
        }
        out.push(u);
        let kids = Self::children_ordered_plain(graph, ordered, u);
        for v in kids {
            Self::dfs_plain(graph, ordered, v, seen, out);
        }
    }

    fn emit_order_conditional(
        graph: &DirectedGraph<NodePayload, CondEdge<EdgePayload>>,
        ordered: Option<&OrderedGraph<NodePayload, CondEdge<EdgePayload>>>,
    ) -> Result<Vec<NodeId>, GraphError> {
        if graph.node_count() == 0 {
            return Ok(Vec::new());
        }
        let topo = graph.topological_sort().map_err(GraphError::from)?;
        let entry = *topo.first().unwrap_or(&NodeId(0));
        let mut out = Vec::new();
        let mut seen = BTreeSet::new();
        Self::dfs_conditional(graph, ordered, entry, &mut seen, &mut out);
        Ok(out)
    }

    fn dfs_conditional(
        graph: &DirectedGraph<NodePayload, CondEdge<EdgePayload>>,
        ordered: Option<&OrderedGraph<NodePayload, CondEdge<EdgePayload>>>,
        u: NodeId,
        seen: &mut BTreeSet<NodeId>,
        out: &mut Vec<NodeId>,
    ) {
        if !seen.insert(u) {
            return;
        }
        out.push(u);
        let kids = Self::children_ordered_conditional(graph, ordered, u);
        for v in kids {
            Self::dfs_conditional(graph, ordered, v, seen, out);
        }
    }

    fn emit_plain_body(order: &[NodeId]) -> String {
        let mut s = String::new();
        for id in order {
            s.push_str("#[allow(dead_code)]\n");
            s.push_str(&format!("fn __node_{}() {{}}\n", id.0));
        }
        s.push_str("fn main() {\n");
        for id in order {
            s.push_str(&format!("  __node_{}();\n", id.0));
        }
        s.push_str("}\n");
        s
    }

    fn emit_conditional_body(
        graph: &DirectedGraph<NodePayload, CondEdge<EdgePayload>>,
        order: &[NodeId],
    ) -> String {
        let mut s = String::new();
        for id in order {
            s.push_str("#[allow(dead_code)]\n");
            s.push_str(&format!("fn __node_{}() {{}}\n", id.0));
        }
        let mut guards = BTreeSet::new();
        for (_, _, _, c) in Self::for_each_edge_conditional(graph) {
            if let ConditionExpr::Runtime(g) = &c.condition {
                guards.insert(*g);
            }
        }
        for g in &guards {
            s.push_str(&format!("fn __guard_{g}() -> bool {{ true }}\n"));
        }
        s.push_str("fn main() {\n");
        for win in order.windows(2) {
            let u = win[0];
            let v = win[1];
            let cond = Self::find_cond_edge(graph, u, v);
            let line = match cond {
                Some(ConditionExpr::Const(true)) => format!("  __node_{}();\n", v.0),
                Some(ConditionExpr::Const(false)) => String::new(),
                Some(ConditionExpr::Runtime(g)) => {
                    format!("  if __guard_{g}() {{ __node_{}(); }}\n", v.0)
                }
                None => format!("  __node_{}();\n", v.0),
            };
            s.push_str(&line);
        }
        s.push_str("}\n");
        s
    }

    fn find_cond_edge(
        graph: &DirectedGraph<NodePayload, CondEdge<EdgePayload>>,
        from: NodeId,
        to: NodeId,
    ) -> Option<ConditionExpr> {
        for (_, t, c) in graph.out_edges(from) {
            if t == to {
                return Some(c.condition.clone());
            }
        }
        None
    }
}

impl From<TopologyError> for GraphError {
    fn from(value: TopologyError) -> Self {
        match value {
            TopologyError::CycleDetected(c) => GraphError::CycleDetected(c),
            TopologyError::SelfLoop(n) => GraphError::SelfLoop(n),
            TopologyError::UnknownNode(n) => GraphError::NodeNotFound(n),
        }
    }
}
