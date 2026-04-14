use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

use crate::error::{CycleError, GraphError};
use crate::{HandleMap, NodeId};

/// A single directed edge with a typed payload.
#[derive(Debug, Clone)]
pub struct DirectedEdge<E> {
    /// Source node.
    pub from: NodeId,
    /// Destination node.
    pub to: NodeId,
    /// User edge data.
    pub data: E,
}

/// Generic directed graph with adjacency stored as a flat edge list (insertion order).
#[derive(Debug, Clone)]
pub struct DirectedGraph<N, E> {
    nodes: HandleMap<N>,
    pub(crate) edges: Vec<DirectedEdge<E>>,
}

impl<N, E> Default for DirectedGraph<N, E> {
    fn default() -> Self {
        Self {
            nodes: HandleMap::new(),
            edges: Vec::new(),
        }
    }
}

impl<N, E> DirectedGraph<N, E> {
    /// Empty graph.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a node and returns its stable id.
    pub fn add_node(&mut self, data: N) -> NodeId {
        self.nodes.insert(data)
    }

    fn ensure_node(&self, id: NodeId) -> Result<(), GraphError> {
        self.nodes
            .get(id)
            .ok_or(GraphError::NodeNotFound(id))
            .map(|_| ())
    }

    fn has_duplicate_edge(&self, from: NodeId, to: NodeId) -> bool {
        self.edges
            .iter()
            .any(|edge| edge.from == from && edge.to == to)
    }

    fn path_without_edge(&self, start: NodeId, goal: NodeId, skip: usize) -> Option<Vec<NodeId>> {
        if start == goal {
            return Some(vec![start]);
        }
        let mut queue = VecDeque::new();
        let mut parent: HashMap<NodeId, NodeId> = HashMap::new();
        let mut visited = HashSet::new();
        queue.push_back(start);
        visited.insert(start);
        while let Some(current) = queue.pop_front() {
            for (idx, edge) in self.edges.iter().enumerate() {
                if idx == skip {
                    continue;
                }
                if edge.from != current {
                    continue;
                }
                let next = edge.to;
                if visited.insert(next) {
                    parent.insert(next, current);
                    if next == goal {
                        let mut path = vec![goal];
                        let mut walk = goal;
                        while walk != start {
                            walk = parent[&walk];
                            path.push(walk);
                        }
                        path.reverse();
                        return Some(path);
                    }
                    queue.push_back(next);
                }
            }
        }
        None
    }

    fn find_path_on_graph(&self, start: NodeId, goal: NodeId) -> Option<Vec<NodeId>> {
        self.path_without_edge(start, goal, usize::MAX)
    }

    /// Returns whether `goal` is reachable from `start` using existing edges.
    #[must_use]
    pub fn reachable_on_graph(&self, start: NodeId, goal: NodeId) -> bool {
        self.find_path_on_graph(start, goal).is_some()
    }

    /// Adds a directed edge, rejecting self-loops, cycles, and duplicate endpoints.
    pub fn add_edge(&mut self, from: NodeId, to: NodeId, data: E) -> Result<(), GraphError> {
        self.ensure_node(from)?;
        self.ensure_node(to)?;
        if from == to {
            return Err(GraphError::CycleDetected(CycleError {
                cycle_path: normalize_cycle(vec![from]),
            }));
        }
        if self.has_duplicate_edge(from, to) {
            return Err(GraphError::DuplicateEdge { from, to });
        }
        if self.reachable_on_graph(to, from) {
            let path = self
                .find_path_on_graph(to, from)
                .unwrap_or_default();
            return Err(GraphError::CycleDetected(CycleError {
                cycle_path: normalize_cycle(path),
            }));
        }
        self.edges.push(DirectedEdge { from, to, data });
        Ok(())
    }

    /// Removes a node and all incident edges.
    pub fn remove_node(&mut self, id: NodeId) -> Option<N> {
        let removed = self.nodes.remove(id)?;
        self.edges
            .retain(|edge| edge.from != id && edge.to != id);
        Some(removed)
    }

    /// Borrows node payload.
    #[must_use]
    pub fn get_node(&self, id: NodeId) -> Option<&N> {
        self.nodes.get(id)
    }

    /// Outgoing neighbor ids in edge insertion order (no deduplication).
    pub fn out_neighbors(&self, node: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        self.edges
            .iter()
            .filter(move |e| e.from == node)
            .map(|e| e.to)
    }

    /// Outgoing edges from `node` in insertion order.
    pub fn out_edges(&self, node: NodeId) -> impl Iterator<Item = (NodeId, &E)> + '_ {
        self.edges
            .iter()
            .filter(move |e| e.from == node)
            .map(|e| (e.to, &e.data))
    }

    /// Incoming edges to `node` in insertion order.
    pub fn in_edges(&self, node: NodeId) -> impl Iterator<Item = (NodeId, &E)> + '_ {
        self.edges
            .iter()
            .filter(move |e| e.to == node)
            .map(|e| (e.from, &e.data))
    }

    /// Kahn topological ordering with ascending [`NodeId`] tie-break in the ready set.
    pub fn topological_sort(&self) -> Result<Vec<NodeId>, CycleError> {
        let mut indegree: HashMap<NodeId, usize> = HashMap::new();
        for (id, _) in self.nodes.iter() {
            indegree.insert(id, 0);
        }
        for edge in &self.edges {
            *indegree.entry(edge.to).or_insert(0) += 1;
        }
        let mut ready: BTreeSet<NodeId> = indegree
            .iter()
            .filter(|(_, d)| **d == 0)
            .map(|(id, _)| *id)
            .collect();
        let mut out = Vec::new();
        while let Some(&node) = ready.first() {
            ready.remove(&node);
            out.push(node);
            for edge in self.edges.iter().filter(|e| e.from == node) {
                let to = edge.to;
                if let Some(d) = indegree.get_mut(&to) {
                    *d -= 1;
                    if *d == 0 {
                        ready.insert(to);
                    }
                }
            }
        }
        if out.len() != self.nodes.len() {
            return Err(extract_cycle(self));
        }
        Ok(out)
    }

    /// `true` when the graph has no directed cycles.
    #[must_use]
    pub fn is_acyclic(&self) -> bool {
        self.topological_sort().is_ok()
    }

    /// Breadth-first traversal from `start`, following edges whose payloads pass `filter`.
    pub fn bfs<F>(&self, start: NodeId, filter: F) -> Vec<NodeId>
    where
        F: Fn(&E) -> bool,
    {
        self.bfs_filtered(start, |_, _, e| filter(e))
    }

    /// BFS with `(from, to, edge)` filtering.
    pub fn bfs_filtered<F>(&self, start: NodeId, filter: F) -> Vec<NodeId>
    where
        F: Fn(NodeId, NodeId, &E) -> bool,
    {
        if self.nodes.get(start).is_none() {
            return Vec::new();
        }
        let mut out = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        visited.insert(start);
        queue.push_back(start);
        while let Some(node) = queue.pop_front() {
            out.push(node);
            let mut outs: Vec<(NodeId, &E)> = self.out_edges(node).collect();
            outs.sort_by_key(|(to, _)| *to);
            for (to, data) in outs {
                if !filter(node, to, data) {
                    continue;
                }
                if visited.insert(to) {
                    queue.push_back(to);
                }
            }
        }
        out
    }

    /// DFS pre-order from `start`.
    pub fn dfs_pre<F>(&self, start: NodeId, filter: F) -> Vec<NodeId>
    where
        F: Fn(&E) -> bool,
    {
        self.dfs_pre_filtered(start, |_, _, e| filter(e))
    }

    fn dfs_pre_filtered<F>(&self, start: NodeId, filter: F) -> Vec<NodeId>
    where
        F: Fn(NodeId, NodeId, &E) -> bool,
    {
        if self.nodes.get(start).is_none() {
            return Vec::new();
        }
        let mut visited = HashSet::new();
        let mut order = Vec::new();
        let mut stack = vec![start];
        while let Some(node) = stack.pop() {
            if !visited.insert(node) {
                continue;
            }
            order.push(node);
            let mut outs: Vec<(NodeId, &E)> = self.out_edges(node).collect();
            outs.sort_by_key(|(to, _)| *to);
            for (to, data) in outs.into_iter().rev() {
                if filter(node, to, data) {
                    stack.push(to);
                }
            }
        }
        order
    }

    /// DFS post-order from `start`.
    pub fn dfs_post<F>(&self, start: NodeId, filter: F) -> Vec<NodeId>
    where
        F: Fn(&E) -> bool,
    {
        self.dfs_post_filtered(start, |_, _, e| filter(e))
    }

    fn dfs_post_filtered<F>(&self, start: NodeId, filter: F) -> Vec<NodeId>
    where
        F: Fn(NodeId, NodeId, &E) -> bool,
    {
        if self.nodes.get(start).is_none() {
            return Vec::new();
        }
        #[derive(Debug)]
        enum Frame {
            Enter(NodeId),
            Exit(NodeId),
        }
        let mut visited = HashSet::new();
        let mut order = Vec::new();
        let mut stack = vec![Frame::Enter(start)];
        while let Some(frame) = stack.pop() {
            match frame {
                Frame::Enter(node) => {
                    if !visited.insert(node) {
                        continue;
                    }
                    stack.push(Frame::Exit(node));
                    let mut outs: Vec<(NodeId, &E)> = self.out_edges(node).collect();
                    outs.sort_by_key(|(to, _)| *to);
                    for (to, data) in outs.into_iter().rev() {
                        if filter(node, to, data) {
                            stack.push(Frame::Enter(to));
                        }
                    }
                }
                Frame::Exit(node) => {
                    order.push(node);
                }
            }
        }
        order
    }

    /// Same as [`Self::bfs`], alternate name from the design doc.
    pub fn reachable_from<F>(&self, start: NodeId, filter: F) -> Vec<NodeId>
    where
        F: Fn(&E) -> bool,
    {
        self.bfs(start, filter)
    }

    /// Nodes not reachable from any root along outgoing edges.
    pub fn dead_node_elimination(&self, live_roots: &[NodeId]) -> Vec<NodeId> {
        let mut live = HashSet::new();
        let mut queue = VecDeque::new();
        for &r in live_roots {
            if self.nodes.get(r).is_some() && live.insert(r) {
                queue.push_back(r);
            }
        }
        while let Some(n) = queue.pop_front() {
            for to in self.out_neighbors(n) {
                if live.insert(to) {
                    queue.push_back(to);
                }
            }
        }
        let mut dead: Vec<NodeId> = self
            .nodes
            .iter()
            .map(|(id, _)| id)
            .filter(|id| !live.contains(id))
            .collect();
        dead.sort();
        dead
    }

    /// Removes every node reported by [`Self::dead_node_elimination`].
    pub fn prune_unreachable(&mut self, live_roots: &[NodeId]) {
        let dead = self.dead_node_elimination(live_roots);
        for id in dead {
            let _ = self.remove_node(id);
        }
    }

    /// Builds a new graph without transitively redundant edges.
    pub fn transitive_reduction(&self) -> DirectedGraph<N, E>
    where
        N: Clone,
        E: Clone,
    {
        let mut out = DirectedGraph::new();
        let id_map: HashMap<NodeId, NodeId> = self
            .nodes
            .iter()
            .map(|(old, data)| (old, out.add_node(data.clone())))
            .collect();
        for edge in &self.edges {
            let from = id_map[&edge.from];
            let to = id_map[&edge.to];
            let mut skip = None;
            for (i, e) in self.edges.iter().enumerate() {
                if e.from == edge.from && e.to == edge.to {
                    skip = Some(i);
                    break;
                }
            }
            let skip = skip.expect("edge exists");
            let redundant = self
                .path_without_edge(edge.from, edge.to, skip)
                .map(|p| p.len() > 1)
                .unwrap_or(false);
            if redundant {
                continue;
            }
            out.edges.push(DirectedEdge {
                from,
                to,
                data: edge.data.clone(),
            });
        }
        out
    }

    /// Validates acyclicity, self-loops, and duplicate endpoints.
    pub fn validate(&self) -> Result<(), GraphError> {
        for edge in &self.edges {
            if edge.from == edge.to {
                return Err(GraphError::SelfLoop(edge.from));
            }
        }
        self.topological_sort()
            .map(|_| ())
            .map_err(GraphError::CycleDetected)
    }

    /// Number of live nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Number of stored edges.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub(crate) fn raw_edges(&self) -> &[DirectedEdge<E>] {
        &self.edges
    }

    /// All live node ids in arbitrary but deterministic order (sorted by id).
    pub fn node_ids(&self) -> Vec<NodeId> {
        let mut ids: Vec<_> = self.nodes.iter().map(|(id, _)| id).collect();
        ids.sort();
        ids
    }
}

pub(crate) fn normalize_cycle(mut path: Vec<NodeId>) -> Vec<NodeId> {
    if path.is_empty() {
        return path;
    }
    let min_pos = path
        .iter()
        .enumerate()
        .min_by_key(|(_, id)| *id)
        .map(|(i, _)| i)
        .unwrap_or(0);
    path.rotate_left(min_pos);
    path
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum DfsColor {
    White,
    Gray,
    Black,
}

fn extract_cycle<N, E>(graph: &DirectedGraph<N, E>) -> CycleError {
    let mut color: HashMap<NodeId, DfsColor> = HashMap::new();
    let mut parent: HashMap<NodeId, NodeId> = HashMap::new();
    for (id, _) in graph.nodes.iter() {
        color.insert(id, DfsColor::White);
    }

    fn dfs<N, E>(
        graph: &DirectedGraph<N, E>,
        u: NodeId,
        color: &mut HashMap<NodeId, DfsColor>,
        parent: &mut HashMap<NodeId, NodeId>,
    ) -> Option<Vec<NodeId>> {
        *color.get_mut(&u).unwrap() = DfsColor::Gray;
        let mut outs: Vec<NodeId> = graph.out_neighbors(u).collect();
        outs.sort();
        for v in outs {
            match *color.get(&v).unwrap_or(&DfsColor::White) {
                DfsColor::Gray => {
                    let mut path = Vec::new();
                    let mut cur = u;
                    loop {
                        path.push(cur);
                        if cur == v {
                            break;
                        }
                        cur = parent[&cur];
                    }
                    path.reverse();
                    return Some(normalize_cycle(path));
                }
                DfsColor::White => {
                    parent.insert(v, u);
                    if let Some(c) = dfs(graph, v, color, parent) {
                        return Some(c);
                    }
                }
                DfsColor::Black => {}
            }
        }
        *color.get_mut(&u).unwrap() = DfsColor::Black;
        None
    }

    for (start, _) in graph.nodes.iter() {
        if matches!(color.get(&start), Some(DfsColor::White))
            && let Some(c) = dfs(graph, start, &mut color, &mut parent)
        {
            return CycleError { cycle_path: c };
        }
    }
    CycleError { cycle_path: Vec::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_graph_construct_nodes_edges() {
        let mut g = DirectedGraph::<u32, u32>::new();
        let mut ids = Vec::new();
        for i in 0..10 {
            ids.push(g.add_node(i));
        }
        let pairs = [
            (0, 1),
            (0, 2),
            (0, 3),
            (0, 4),
            (1, 5),
            (2, 5),
            (3, 6),
            (4, 7),
            (5, 8),
            (6, 8),
            (7, 9),
            (8, 9),
            (3, 7),
            (1, 8),
            (2, 6),
        ];
        for (a, b) in pairs {
            g.add_edge(ids[a], ids[b], 0).unwrap();
        }
        assert_eq!(g.node_count(), 10);
        assert_eq!(g.edge_count(), 15);
    }

    #[test]
    fn test_graph_neighbors_outgoing() {
        let mut g = DirectedGraph::<(), ()>::new();
        let a = g.add_node(());
        let b = g.add_node(());
        let c = g.add_node(());
        let d = g.add_node(());
        g.add_edge(a, b, ()).unwrap();
        g.add_edge(a, c, ()).unwrap();
        g.add_edge(a, d, ()).unwrap();
        let hs: HashSet<_> = g.out_neighbors(a).collect();
        assert_eq!(hs, HashSet::from([b, c, d]));
    }

    #[test]
    fn test_graph_handle_generation() {
        let mut g = DirectedGraph::<&str, ()>::new();
        let h1 = g.add_node("A");
        assert_eq!(g.get_node(h1), Some(&"A"));
        assert_eq!(g.remove_node(h1), Some("A"));
        let h2 = g.add_node("B");
        assert!(g.get_node(h1).is_none());
        assert_eq!(g.get_node(h2), Some(&"B"));
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_graph_remove_node_invalidates() {
        let mut g = DirectedGraph::<char, ()>::new();
        let a = g.add_node('A');
        let b = g.add_node('B');
        let c = g.add_node('C');
        g.add_edge(a, b, ()).unwrap();
        g.add_edge(b, c, ()).unwrap();
        g.add_edge(a, c, ()).unwrap();
        g.remove_node(b);
        assert_eq!(g.edge_count(), 1);
        let hs: HashSet<_> = g.out_neighbors(a).collect();
        assert_eq!(hs, HashSet::from([c]));
    }
}
