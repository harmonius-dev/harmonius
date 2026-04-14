use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::id::{EdgeId, NodeId};
use crate::topology_error::{CycleError, TopologyError};

#[derive(Clone, Debug)]
struct StoredEdge<E> {
    from: NodeId,
    to: NodeId,
    data: E,
}

/// Adjacency-list directed multigraph with dense `NodeId` handles.
#[derive(Clone, Debug)]
pub struct DirectedGraph<N, E> {
    nodes: Vec<N>,
    edges: Vec<StoredEdge<E>>,
}

impl<N, E> Default for DirectedGraph<N, E> {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
}

impl<N, E> DirectedGraph<N, E> {
    /// Empty graph.
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of live nodes.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Number of edges.
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Append a node and return its id.
    pub fn add_node(&mut self, node: N) -> NodeId {
        let id = NodeId(self.nodes.len() as u32);
        self.nodes.push(node);
        id
    }

    /// Borrow a node payload.
    pub fn get_node(&self, id: NodeId) -> Option<&N> {
        self.nodes.get(id.0 as usize)
    }

    /// Append an edge; `from` and `to` must exist.
    pub fn add_edge(&mut self, from: NodeId, to: NodeId, data: E) -> EdgeId {
        let id = EdgeId(self.edges.len() as u32);
        self.edges.push(StoredEdge { from, to, data });
        id
    }

    /// Outgoing edges for `node`.
    pub fn out_edges(&self, node: NodeId) -> impl Iterator<Item = (EdgeId, NodeId, &E)> + '_ {
        self.edges.iter().enumerate().filter_map(move |(i, e)| {
            if e.from == node {
                Some((EdgeId(i as u32), e.to, &e.data))
            } else {
                None
            }
        })
    }

    /// Kahn topological ordering with stable tie-breaking by `NodeId`.
    pub fn topological_sort(&self) -> Result<Vec<NodeId>, TopologyError> {
        self.validate()?;
        let n = self.nodes.len();
        if n == 0 {
            return Ok(Vec::new());
        }
        let mut indeg: BTreeMap<NodeId, u32> = BTreeMap::new();
        for id in 0..n {
            let nid = NodeId(id as u32);
            indeg.insert(nid, 0);
        }
        for e in &self.edges {
            *indeg.entry(e.to).or_insert(0) += 1;
        }
        let mut ready: BTreeSet<NodeId> = indeg
            .iter()
            .filter_map(|(&id, &d)| if d == 0 { Some(id) } else { None })
            .collect();
        let mut out = Vec::with_capacity(n);
        let mut q: VecDeque<NodeId> = ready.iter().copied().collect();
        ready.clear();
        while let Some(u) = q.pop_front() {
            out.push(u);
            for (_, v, _) in self.out_edges(u) {
                let entry = indeg.get_mut(&v).expect("indegree map covers all nodes");
                *entry -= 1;
                if *entry == 0 {
                    q.push_back(v);
                }
            }
        }
        if out.len() == n {
            Ok(out)
        } else {
            Err(TopologyError::CycleDetected(
                self.find_cycle_path().unwrap_or_else(|| CycleError {
                    path: Vec::new(),
                }),
            ))
        }
    }

    /// Reject self loops and obvious cycles (via failed topo sort).
    pub fn validate(&self) -> Result<(), TopologyError> {
        let n = self.nodes.len();
        for e in &self.edges {
            if e.from == e.to {
                return Err(TopologyError::SelfLoop(e.from));
            }
            if e.from.0 as usize >= n {
                return Err(TopologyError::UnknownNode(e.from));
            }
            if e.to.0 as usize >= n {
                return Err(TopologyError::UnknownNode(e.to));
            }
        }
        let mut indeg: BTreeMap<NodeId, u32> = BTreeMap::new();
        for id in 0..n {
            indeg.insert(NodeId(id as u32), 0);
        }
        for e in &self.edges {
            *indeg.entry(e.to).or_insert(0) += 1;
        }
        let mut q: VecDeque<NodeId> = indeg
            .iter()
            .filter_map(|(&id, &d)| if d == 0 { Some(id) } else { None })
            .collect();
        let mut seen = 0usize;
        while let Some(u) = q.pop_front() {
            seen += 1;
            for (_, v, _) in self.out_edges(u) {
                let entry = indeg.get_mut(&v).expect("indegree map covers all nodes");
                *entry -= 1;
                if *entry == 0 {
                    q.push_back(v);
                }
            }
        }
        if seen == n {
            Ok(())
        } else {
            Err(TopologyError::CycleDetected(
                self.find_cycle_path().unwrap_or_else(|| CycleError {
                    path: Vec::new(),
                }),
            ))
        }
    }

    fn find_cycle_path(&self) -> Option<CycleError> {
        let n = self.nodes.len();
        if n == 0 {
            return None;
        }
        let mut visited = vec![false; n];
        let mut stack = Vec::new();
        let mut onstack = vec![false; n];
        for start in 0..n {
            let sid = NodeId(start as u32);
            if visited[start] {
                continue;
            }
            if let Some(path) = self.dfs_cycle(sid, &mut visited, &mut stack, &mut onstack) {
                return Some(CycleError { path });
            }
        }
        None
    }

    fn dfs_cycle(
        &self,
        u: NodeId,
        visited: &mut [bool],
        stack: &mut Vec<NodeId>,
        onstack: &mut [bool],
    ) -> Option<Vec<NodeId>> {
        let ui = u.0 as usize;
        visited[ui] = true;
        onstack[ui] = true;
        stack.push(u);
        for (_, v, _) in self.out_edges(u) {
            let vi = v.0 as usize;
            if !visited[vi] {
                if let Some(p) = self.dfs_cycle(v, visited, stack, onstack) {
                    return Some(p);
                }
            } else if onstack[vi] {
                let pos = stack.iter().position(|&x| x == v)?;
                let mut path = stack[pos..].to_vec();
                path.push(v);
                return Some(path);
            }
        }
        stack.pop();
        onstack[ui] = false;
        None
    }
}
