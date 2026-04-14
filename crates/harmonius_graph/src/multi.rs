use std::collections::{HashMap, HashSet, VecDeque};

use crate::error::{CycleError, GraphError};
use crate::graph::DirectedEdge;
use crate::{HandleMap, NodeId};

/// Stable index of an edge inside a [`MultiGraph`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EdgeHandle(pub usize);

/// Directed multigraph allowing parallel edges between the same endpoints.
#[derive(Debug, Clone)]
pub struct MultiGraph<N, E> {
    nodes: HandleMap<N>,
    edges: Vec<DirectedEdge<E>>,
}

impl<N, E> Default for MultiGraph<N, E> {
    fn default() -> Self {
        Self {
            nodes: HandleMap::new(),
            edges: Vec::new(),
        }
    }
}

impl<N, E> MultiGraph<N, E> {
    /// Empty multigraph.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a node.
    pub fn add_node(&mut self, data: N) -> NodeId {
        self.nodes.insert(data)
    }

    fn ensure_node(&self, id: NodeId) -> Result<(), GraphError> {
        self.nodes
            .get(id)
            .ok_or(GraphError::NodeNotFound(id))
            .map(|_| ())
    }

    fn reachable_on_graph(&self, start: NodeId, goal: NodeId) -> bool {
        if self.nodes.get(start).is_none() {
            return false;
        }
        let mut q = VecDeque::new();
        let mut seen = HashSet::new();
        q.push_back(start);
        seen.insert(start);
        while let Some(u) = q.pop_front() {
            if u == goal {
                return true;
            }
            for e in &self.edges {
                if e.from == u && seen.insert(e.to) {
                    q.push_back(e.to);
                }
            }
        }
        false
    }

    /// Adds an edge, allowing parallel endpoints; still rejects cycles and self-loops.
    pub fn add_edge(&mut self, from: NodeId, to: NodeId, data: E) -> Result<(), GraphError> {
        self.ensure_node(from)?;
        self.ensure_node(to)?;
        if from == to {
            return Err(GraphError::CycleDetected(CycleError {
                cycle_path: crate::graph::normalize_cycle(vec![from]),
            }));
        }
        if self.reachable_on_graph(to, from) {
            let path = self.path_on_graph(to, from).unwrap_or_default();
            return Err(GraphError::CycleDetected(CycleError {
                cycle_path: crate::graph::normalize_cycle(path),
            }));
        }
        self.edges.push(DirectedEdge { from, to, data });
        Ok(())
    }

    fn path_on_graph(&self, start: NodeId, goal: NodeId) -> Option<Vec<NodeId>> {
        if start == goal {
            return Some(vec![start]);
        }
        let mut q = VecDeque::new();
        let mut parent = HashMap::new();
        let mut seen = HashSet::new();
        q.push_back(start);
        seen.insert(start);
        while let Some(u) = q.pop_front() {
            for e in &self.edges {
                if e.from != u {
                    continue;
                }
                let v = e.to;
                if seen.insert(v) {
                    parent.insert(v, u);
                    if v == goal {
                        let mut path = vec![goal];
                        let mut cur = goal;
                        while cur != start {
                            cur = parent[&cur];
                            path.push(cur);
                        }
                        path.reverse();
                        return Some(path);
                    }
                    q.push_back(v);
                }
            }
        }
        None
    }

    /// Outgoing edges with stable handles.
    pub fn out_edges_with_handles(
        &self,
        node: NodeId,
    ) -> impl Iterator<Item = (EdgeHandle, NodeId, &E)> + '_ {
        self.edges
            .iter()
            .enumerate()
            .filter(move |(_, e)| e.from == node)
            .map(|(i, e)| (EdgeHandle(i), e.to, &e.data))
    }

    /// Node count.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Edge count.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}
