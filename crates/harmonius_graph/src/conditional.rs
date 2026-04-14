use std::collections::{HashMap, HashSet, VecDeque};

use crate::error::GraphError;
use crate::{DirectedGraph, NodeId};

/// Boolean expression used to guard edges.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConditionExpr {
    /// True when the named flag is set in [`ConditionContext`].
    HasFlag(String),
    /// Logical negation of an inner expression.
    Not(Box<ConditionExpr>),
}

/// Runtime flag bag for condition evaluation.
#[derive(Debug, Clone, Default)]
pub struct ConditionContext {
    /// Named boolean flags.
    pub flags: HashMap<String, bool>,
}

impl ConditionContext {
    /// Creates an empty context.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

/// Placeholder for codegen-backed condition dispatch (see design doc).
#[derive(Debug, Default, Clone)]
pub struct ConditionRegistry;

impl ConditionRegistry {
    /// Empty registry (sufficient for [`ConditionExpr::HasFlag`]).
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

/// Edge payload paired with a guard.
#[derive(Debug, Clone)]
pub struct CondEdge<E> {
    /// Guard evaluated against a [`ConditionContext`].
    pub condition: ConditionExpr,
    /// Inner edge payload.
    pub data: E,
}

/// Directed graph whose edges carry [`ConditionExpr`] guards.
#[derive(Debug, Clone)]
pub struct ConditionalGraph<N, E> {
    graph: DirectedGraph<N, CondEdge<E>>,
}

impl<N, E> Default for ConditionalGraph<N, E> {
    fn default() -> Self {
        Self {
            graph: DirectedGraph::new(),
        }
    }
}

impl<N, E> ConditionalGraph<N, E> {
    /// Empty conditional graph.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a node.
    pub fn add_node(&mut self, data: N) -> NodeId {
        self.graph.add_node(data)
    }

    /// Adds a guarded edge.
    pub fn add_conditional_edge(
        &mut self,
        from: NodeId,
        to: NodeId,
        condition: ConditionExpr,
        data: E,
    ) -> Result<(), GraphError> {
        self.graph
            .add_edge(from, to, CondEdge { condition, data })
    }

    #[allow(clippy::only_used_in_recursion)]
    fn eval(expr: &ConditionExpr, ctx: &ConditionContext, reg: &ConditionRegistry) -> bool {
        match expr {
            ConditionExpr::HasFlag(name) => *ctx.flags.get(name).unwrap_or(&false),
            ConditionExpr::Not(inner) => !Self::eval(inner, ctx, reg),
        }
    }

    /// Immediate successors along edges whose conditions pass.
    #[must_use]
    pub fn successors(
        &self,
        node: NodeId,
        ctx: &ConditionContext,
        registry: &ConditionRegistry,
    ) -> Vec<NodeId> {
        let mut out: Vec<NodeId> = self
            .graph
            .out_edges(node)
            .filter(|(_, ce)| Self::eval(&ce.condition, ctx, registry))
            .map(|(to, _)| to)
            .collect();
        out.sort();
        out
    }

    /// BFS reachability with conditional edges.
    #[must_use]
    pub fn reachable_from(
        &self,
        start: NodeId,
        ctx: &ConditionContext,
        registry: &ConditionRegistry,
    ) -> Vec<NodeId> {
        if self.graph.get_node(start).is_none() {
            return Vec::new();
        }
        let mut out = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        visited.insert(start);
        queue.push_back(start);
        while let Some(n) = queue.pop_front() {
            out.push(n);
            let mut succ = self.successors(n, ctx, registry);
            succ.sort();
            for s in succ {
                if visited.insert(s) {
                    queue.push_back(s);
                }
            }
        }
        out
    }

    /// Read-only view of the inner graph.
    #[must_use]
    pub fn inner(&self) -> &DirectedGraph<N, CondEdge<E>> {
        &self.graph
    }
}

impl<N, E> ConditionalGraph<N, E> {
    /// Same as [`Self::reachable_from`] (alias for design-doc naming).
    #[must_use]
    pub fn traverse_from(
        &self,
        start: NodeId,
        ctx: &ConditionContext,
        registry: &ConditionRegistry,
    ) -> Vec<NodeId> {
        self.reachable_from(start, ctx, registry)
    }
}
