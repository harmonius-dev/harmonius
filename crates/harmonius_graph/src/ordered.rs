use std::collections::{BTreeMap, HashMap, HashSet};

use smallvec::SmallVec;

use crate::error::GraphError;
use crate::{DirectedGraph, NodeId};

/// Tree-friendly directed graph with deterministic child ordering.
#[derive(Debug, Clone)]
pub struct OrderedGraph<N, E> {
    graph: DirectedGraph<N, E>,
    /// For each parent, ordered child ids (must match outgoing edges).
    child_order: BTreeMap<NodeId, SmallVec<[NodeId; 8]>>,
}

impl<N, E> Default for OrderedGraph<N, E> {
    fn default() -> Self {
        Self {
            graph: DirectedGraph::new(),
            child_order: BTreeMap::new(),
        }
    }
}

impl<N, E> OrderedGraph<N, E> {
    /// Empty ordered graph.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a node.
    pub fn add_node(&mut self, data: N) -> NodeId {
        self.graph.add_node(data)
    }

    /// Adds `parent -> child` and appends `child` to the ordered child list.
    pub fn add_child(&mut self, parent: NodeId, child: NodeId, data: E) -> Result<(), GraphError> {
        self.graph.add_edge(parent, child, data)?;
        self.child_order.entry(parent).or_default().push(child);
        Ok(())
    }

    /// Ordered child ids for `parent`.
    #[must_use]
    pub fn ordered_children(&self, parent: NodeId) -> &[NodeId] {
        self.child_order
            .get(&parent)
            .map(SmallVec::as_slice)
            .unwrap_or(&[])
    }

    /// Alias used by tests (`children`).
    #[must_use]
    pub fn children(&self, parent: NodeId) -> &[NodeId] {
        self.ordered_children(parent)
    }

    /// Replaces the child ordering for `parent`.
    pub fn set_order(&mut self, parent: NodeId, order: SmallVec<[NodeId; 8]>) {
        if order.is_empty() {
            self.child_order.remove(&parent);
        } else {
            self.child_order.insert(parent, order);
        }
    }

    /// Test-case name for reordering.
    pub fn reorder_children(&mut self, parent: NodeId, order: SmallVec<[NodeId; 8]>) {
        self.set_order(parent, order);
    }

    /// Reorders children from any slice-like source (integration-test friendly).
    pub fn reorder_children_list(&mut self, parent: NodeId, order: &[NodeId]) {
        self.set_order(parent, SmallVec::from_slice(order));
    }

    /// Unique node with no incoming edges, if exactly one exists.
    #[must_use]
    pub fn root(&self) -> Option<NodeId> {
        let ids = self.graph.node_ids();
        let mut roots: Vec<NodeId> = ids
            .into_iter()
            .filter(|id| self.graph.in_edges(*id).next().is_none())
            .collect();
        roots.sort();
        if roots.len() == 1 {
            Some(roots[0])
        } else {
            None
        }
    }

    /// Nodes with no outgoing edges.
    #[must_use]
    pub fn leaves(&self) -> Vec<NodeId> {
        let mut out: Vec<NodeId> = self
            .graph
            .node_ids()
            .into_iter()
            .filter(|id| self.graph.out_edges(*id).next().is_none())
            .collect();
        out.sort();
        out
    }

    /// Depth (number of edges) from [`Self::root`] to `node`.
    #[must_use]
    pub fn depth(&self, node: NodeId) -> Option<usize> {
        let r = self.root()?;
        let mut dist = HashMap::new();
        dist.insert(r, 0usize);
        let mut frontier = vec![r];
        while let Some(u) = frontier.pop() {
            if u == node {
                return dist.get(&node).copied();
            }
            let mut ch: Vec<NodeId> = self.ordered_children(u).to_vec();
            ch.sort();
            for v in ch {
                if !dist.contains_key(&v) {
                    dist.insert(v, dist[&u] + 1);
                    frontier.push(v);
                }
            }
        }
        None
    }

    /// Extracts the subtree rooted at `node` (descendants only, including the root).
    #[must_use]
    pub fn subtree(&self, node: NodeId) -> OrderedGraph<N, E>
    where
        N: Clone,
        E: Clone,
    {
        let mut keep = HashSet::new();
        let mut stack = vec![node];
        while let Some(u) = stack.pop() {
            if !keep.insert(u) {
                continue;
            }
            for c in self.ordered_children(u) {
                stack.push(*c);
            }
        }
        let mut out = OrderedGraph::new();
        let mut map: HashMap<NodeId, NodeId> = HashMap::new();
        let mut sorted: Vec<_> = keep.iter().copied().collect();
        sorted.sort();
        for old in &sorted {
            let payload = self.graph.get_node(*old).expect("live node").clone();
            let new_id = out.add_node(payload);
            map.insert(*old, new_id);
        }
        for old in &sorted {
            for &och in self.ordered_children(*old) {
                if keep.contains(&och) {
                    let data = self
                        .graph
                        .out_edges(*old)
                        .find(|(t, _)| *t == och)
                        .expect("child edge")
                        .1
                        .clone();
                    out.add_child(map[old], map[&och], data).expect("subtree edge");
                }
            }
        }
        out
    }

    /// Path from `node` up to the root (including both ends, root last).
    #[must_use]
    pub fn ancestors(&self, node: NodeId) -> Vec<NodeId> {
        let mut out = vec![node];
        let mut cur = node;
        while let Some(p) = self.unique_parent(cur) {
            out.push(p);
            cur = p;
        }
        out
    }

    fn unique_parent(&self, node: NodeId) -> Option<NodeId> {
        let mut ps: Vec<NodeId> = self.graph.in_edges(node).map(|(f, _)| f).collect();
        ps.sort();
        if ps.len() == 1 {
            Some(ps[0])
        } else {
            None
        }
    }

    /// Lowest common ancestor in a tree-shaped ordered graph.
    #[must_use]
    pub fn lca(&self, a: NodeId, b: NodeId) -> Option<NodeId> {
        if a == b {
            return Some(a);
        }
        let mut pa = self.ancestors(a);
        let mut pb = self.ancestors(b);
        pa.reverse();
        pb.reverse();
        let mut last = None;
        for (x, y) in pa.iter().zip(pb.iter()) {
            if x == y {
                last = Some(*x);
            } else {
                break;
            }
        }
        last
    }

    /// Borrow inner [`DirectedGraph`].
    #[must_use]
    pub fn inner(&self) -> &DirectedGraph<N, E> {
        &self.graph
    }

    /// Number of live nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }
}
