use crate::graph::DirectedGraph;
use crate::id::{EdgeId, NodeId};

/// Boolean guard attached to a conditional edge.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConditionExpr {
    /// Folded constant for compile-time elimination tests.
    Const(bool),
    /// Placeholder for runtime-resolved guards.
    Runtime(u32),
}

/// Edge payload plus its guard expression.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CondEdge<E> {
    /// Guard evaluated during compilation or at runtime.
    pub condition: ConditionExpr,
    /// Underlying edge data shared with the logic graph.
    pub data: E,
}

/// Directed graph whose edges carry `ConditionExpr` guards.
#[derive(Clone, Debug, Default)]
pub struct ConditionalGraph<N, E> {
    inner: DirectedGraph<N, CondEdge<E>>,
}

impl<N, E> ConditionalGraph<N, E> {
    /// Empty conditional graph.
    pub fn new() -> Self {
        Self {
            inner: DirectedGraph::new(),
        }
    }

    /// Add a node; see [`DirectedGraph::add_node`].
    pub fn add_node(&mut self, node: N) -> NodeId {
        self.inner.add_node(node)
    }

    /// Add a guarded edge.
    pub fn add_conditional_edge(
        &mut self,
        from: NodeId,
        to: NodeId,
        edge: CondEdge<E>,
    ) -> EdgeId {
        self.inner.add_edge(from, to, edge)
    }

    /// Borrow the underlying topology.
    pub fn inner(&self) -> &DirectedGraph<N, CondEdge<E>> {
        &self.inner
    }

    /// Mutably borrow the underlying topology.
    pub fn inner_mut(&mut self) -> &mut DirectedGraph<N, CondEdge<E>> {
        &mut self.inner
    }
}
