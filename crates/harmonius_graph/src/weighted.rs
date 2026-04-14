use std::collections::{HashMap, HashSet};

use crate::error::GraphError;
use crate::{DirectedGraph, NodeId};

/// Directed graph with a parallel `f64` cost per edge.
#[derive(Debug, Clone)]
pub struct WeightedGraph<N, E> {
    graph: DirectedGraph<N, E>,
    weights: Vec<f64>,
}

impl<N, E> Default for WeightedGraph<N, E> {
    fn default() -> Self {
        Self {
            graph: DirectedGraph::new(),
            weights: Vec::new(),
        }
    }
}

impl<N, E> WeightedGraph<N, E> {
    /// Empty weighted graph.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Delegates to the inner [`DirectedGraph`].
    pub fn add_node(&mut self, data: N) -> NodeId {
        self.graph.add_node(data)
    }

    /// Adds a weighted directed edge.
    pub fn add_weighted_edge(
        &mut self,
        from: NodeId,
        to: NodeId,
        data: E,
        weight: f64,
    ) -> Result<(), GraphError> {
        self.graph.add_edge(from, to, data)?;
        self.weights.push(weight);
        Ok(())
    }

    /// Dijkstra shortest path using `f64` costs (linear-scan frontier for determinism).
    #[must_use]
    pub fn shortest_path(&self, from: NodeId, to: NodeId) -> Option<(Vec<NodeId>, f64)> {
        if self.graph.get_node(from).is_none() || self.graph.get_node(to).is_none() {
            return None;
        }
        let n = self.graph.node_count();
        let mut dist: HashMap<NodeId, f64> = HashMap::new();
        let mut prev: HashMap<NodeId, NodeId> = HashMap::new();
        let mut done = HashSet::new();
        dist.insert(from, 0.0);
        for _ in 0..n {
            let mut best: Option<(NodeId, f64)> = None;
            for (id, &d) in &dist {
                if done.contains(id) || !d.is_finite() {
                    continue;
                }
                best = Some(match best {
                    None => (*id, d),
                    Some((bid, bd)) if d < bd || (d == bd && *id < bid) => (*id, d),
                    Some(b) => b,
                });
            }
            let (u, du) = match best {
                Some(b) => b,
                None => break,
            };
            if u == to {
                let mut path = vec![to];
                let mut cur = to;
                while cur != from {
                    cur = prev[&cur];
                    path.push(cur);
                }
                path.reverse();
                return Some((path, du));
            }
            done.insert(u);
            for (i, edge) in self.graph.raw_edges().iter().enumerate() {
                if edge.from != u {
                    continue;
                }
                let w = self.weights.get(i).copied().unwrap_or(0.0);
                let nd = du + w;
                let v = edge.to;
                let entry = dist.entry(v).or_insert(f64::INFINITY);
                if nd < *entry {
                    *entry = nd;
                    prev.insert(v, u);
                }
            }
        }
        None
    }

    /// Nodes reachable from `start` with cumulative cost at most `budget`.
    #[must_use]
    pub fn min_cost_reachable(&self, start: NodeId, budget: f64) -> Vec<NodeId> {
        if self.graph.get_node(start).is_none() {
            return Vec::new();
        }
        let n = self.graph.node_count();
        let mut best: HashMap<NodeId, f64> = HashMap::new();
        best.insert(start, 0.0);
        let mut done = HashSet::new();
        for _ in 0..n {
            let mut pick: Option<(NodeId, f64)> = None;
            for (id, &d) in &best {
                if done.contains(id) || d > budget || !d.is_finite() {
                    continue;
                }
                pick = Some(match pick {
                    None => (*id, d),
                    Some((bid, bd)) if d < bd || (d == bd && *id < bid) => (*id, d),
                    Some(b) => b,
                });
            }
            let (u, du) = match pick {
                Some(p) => p,
                None => break,
            };
            done.insert(u);
            for (i, edge) in self.graph.raw_edges().iter().enumerate() {
                if edge.from != u {
                    continue;
                }
                let w = self.weights.get(i).copied().unwrap_or(0.0);
                let nd = du + w;
                if nd > budget {
                    continue;
                }
                let v = edge.to;
                let entry = best.entry(v).or_insert(f64::INFINITY);
                if nd < *entry {
                    *entry = nd;
                }
            }
        }
        let mut out: Vec<NodeId> = best
            .into_iter()
            .filter(|(_, c)| *c <= budget)
            .map(|(id, _)| id)
            .collect();
        out.sort();
        out
    }

    /// Borrow inner graph.
    #[must_use]
    pub fn inner(&self) -> &DirectedGraph<N, E> {
        &self.graph
    }
}
