//! Forward and reverse dependency edges.

use std::collections::{HashMap, HashSet, VecDeque};

use crate::AssetId;

/// Dependency graph with forward `from -> to` edges.
#[derive(Debug, Default)]
pub struct DependencyGraph {
    forward: HashMap<AssetId, Vec<AssetId>>,
    reverse: HashMap<AssetId, Vec<AssetId>>,
}

impl DependencyGraph {
    /// New empty graph.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record that `from` depends on `to`.
    pub fn add_dependency(&mut self, from: AssetId, to: AssetId) {
        self.forward.entry(from).or_default().push(to);
        self.reverse.entry(to).or_default().push(from);
    }

    /// Direct dependents (reverse edges) of `id`.
    pub fn dependents_of(&self, id: AssetId) -> Vec<AssetId> {
        self.reverse.get(&id).cloned().unwrap_or_default()
    }

    /// Transitive dependents in breadth-first order (test checks containment + topo).
    pub fn transitive_dependents(&self, id: AssetId) -> Vec<AssetId> {
        let mut out = Vec::new();
        let mut seen = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back(id);
        while let Some(n) = q.pop_front() {
            for d in self.dependents_of(n) {
                if seen.insert(d) {
                    out.push(d);
                    q.push_back(d);
                }
            }
        }
        out
    }

    /// Topological sort of `subset` restricted to edges between nodes in `subset`.
    pub fn topological_sort(&self, subset: &[AssetId]) -> Vec<AssetId> {
        let set: HashSet<AssetId> = subset.iter().copied().collect();
        let mut indeg: HashMap<AssetId, u32> = HashMap::new();
        for &n in &set {
            indeg.entry(n).or_insert(0);
        }
        for (&from, tos) in &self.forward {
            if !set.contains(&from) {
                continue;
            }
            for &to in tos {
                if set.contains(&to) {
                    *indeg.entry(to).or_insert(0) += 1;
                }
            }
        }
        let mut q: VecDeque<AssetId> = indeg
            .iter()
            .filter_map(|(&n, &d)| if d == 0 { Some(n) } else { None })
            .collect();
        let mut order = Vec::new();
        while let Some(n) = q.pop_front() {
            order.push(n);
            if let Some(tos) = self.forward.get(&n) {
                for &to in tos {
                    if !set.contains(&to) {
                        continue;
                    }
                    let e = indeg.get_mut(&to).expect("indeg");
                    *e -= 1;
                    if *e == 0 {
                        q.push_back(to);
                    }
                }
            }
        }
        order
    }

    /// Dirty-set closure: roots plus all transitive dependents, then forward topological order.
    pub fn invalidate(&self, roots: &[AssetId]) -> Vec<AssetId> {
        let mut dirty = HashSet::new();
        let mut q = VecDeque::new();
        for &r in roots {
            if dirty.insert(r) {
                q.push_back(r);
            }
        }
        while let Some(n) = q.pop_front() {
            for d in self.dependents_of(n) {
                if dirty.insert(d) {
                    q.push_back(d);
                }
            }
        }
        let v: Vec<AssetId> = dirty.into_iter().collect();
        self.topological_sort(&v)
    }
}
