use std::borrow::Cow;
use std::collections::BTreeMap;

use crate::graph::DirectedGraph;
use crate::id::{EdgeId, NodeId};

/// Directed graph with explicit per-parent child ordering.
#[derive(Clone, Debug)]
pub struct OrderedGraph<N, E> {
    inner: DirectedGraph<N, E>,
    order: BTreeMap<NodeId, Vec<NodeId>>,
}

impl<N, E> Default for OrderedGraph<N, E> {
    fn default() -> Self {
        Self {
            inner: DirectedGraph::new(),
            order: BTreeMap::new(),
        }
    }
}

impl<N, E> OrderedGraph<N, E> {
    /// Empty ordered graph.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a node; see [`DirectedGraph::add_node`].
    pub fn add_node(&mut self, node: N) -> NodeId {
        self.inner.add_node(node)
    }

    /// Add an edge; see [`DirectedGraph::add_edge`].
    pub fn add_edge(&mut self, from: NodeId, to: NodeId, data: E) -> EdgeId {
        self.inner.add_edge(from, to, data)
    }

    /// Replace the ordered child list for `parent`.
    pub fn set_order(&mut self, parent: NodeId, children: Vec<NodeId>) {
        self.order.insert(parent, children);
    }

    /// Children of `parent` in evaluation order.
    pub fn ordered_children<'a>(&'a self, parent: NodeId) -> Cow<'a, [NodeId]> {
        if let Some(v) = self.order.get(&parent) {
            Cow::Borrowed(v.as_slice())
        } else {
            let mut v: Vec<NodeId> = self.inner.out_edges(parent).map(|(_, t, _)| t).collect();
            v.sort_unstable();
            Cow::Owned(v)
        }
    }

    /// Borrow the underlying topology.
    pub fn inner(&self) -> &DirectedGraph<N, E> {
        &self.inner
    }

    /// Mutably borrow the underlying topology.
    pub fn inner_mut(&mut self) -> &mut DirectedGraph<N, E> {
        &mut self.inner
    }
}
