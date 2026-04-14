//! Plugin dependency graph validation and topological ordering.

use std::any::TypeId;
use std::collections::{HashMap, HashSet, VecDeque};

use super::app::Plugin;

/// A single graph validation issue.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PluginGraphError {
    /// Declared dependency is not registered.
    MissingDependency {
        /// Plugin that is missing a dependency.
        plugin: &'static str,
        /// Missing dependency name (best-effort).
        missing: &'static str,
        /// Dependency chain for diagnostics.
        chain: Vec<&'static str>,
        /// Human-readable remediation hint.
        suggestion: String,
    },
    /// Two plugins declare each other as conflicting.
    Conflict {
        /// First plugin name.
        plugin_a: &'static str,
        /// Second plugin name.
        plugin_b: &'static str,
        /// Human-readable remediation hint.
        suggestion: String,
    },
    /// A dependency cycle was detected.
    CyclicDependency {
        /// Cycle path as plugin names.
        cycle: Vec<&'static str>,
        /// Human-readable remediation hint.
        suggestion: String,
    },
}

/// Directed graph of plugin dependencies and conflicts.
#[derive(Debug, Default)]
pub struct DependencyGraph {
    edges: HashMap<TypeId, Vec<TypeId>>,
    names: HashMap<TypeId, &'static str>,
    dep_labels: HashMap<TypeId, &'static str>,
    conflicts: Vec<(TypeId, TypeId)>,
    nodes: Vec<TypeId>,
}

impl DependencyGraph {
    /// Creates an empty graph.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a plugin node with its declared edges.
    pub fn add_plugin(&mut self, id: TypeId, name: &'static str, plugin: &dyn Plugin) {
        if !self.nodes.contains(&id) {
            self.nodes.push(id);
        }
        self.names.insert(id, name);
        self.edges.insert(id, plugin.dependencies());
        for (tid, label) in plugin.labeled_dependencies() {
            self.dep_labels.insert(tid, label);
        }
        for c in plugin.conflicts() {
            self.conflicts.push((id, c));
        }
    }

    /// Validates dependencies, conflicts, and cycles in one pass.
    pub fn validate(&self) -> Result<(), Vec<PluginGraphError>> {
        let mut errors = Vec::new();
        let known: HashSet<TypeId> = self.nodes.iter().copied().collect();
        for (&plugin_id, deps) in &self.edges {
            let pname = *self.names.get(&plugin_id).unwrap_or(&"?");
            for &dep in deps {
                if !known.contains(&dep) {
                    let missing_name = self
                        .names
                        .get(&dep)
                        .copied()
                        .or_else(|| self.dep_labels.get(&dep).copied())
                        .unwrap_or("<unregistered>");
                    errors.push(PluginGraphError::MissingDependency {
                        plugin: pname,
                        missing: missing_name,
                        chain: vec![pname, missing_name],
                        suggestion: format!("register {missing_name} before {pname}"),
                    });
                }
            }
        }
        for &(a, b) in &self.conflicts {
            if known.contains(&a) && known.contains(&b) {
                errors.push(PluginGraphError::Conflict {
                    plugin_a: self.names.get(&a).copied().unwrap_or("?"),
                    plugin_b: self.names.get(&b).copied().unwrap_or("?"),
                    suggestion: "remove one of the conflicting plugins".to_string(),
                });
            }
        }
        if let Some(cycle) = self.find_cycle_path() {
            errors.push(PluginGraphError::CyclicDependency {
                cycle,
                suggestion: "break the cycle by removing a dependency edge".to_string(),
            });
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Produces a topological order (call [`validate`](Self::validate) first).
    pub fn topological_sort(&self) -> Result<Vec<TypeId>, Vec<PluginGraphError>> {
        let known: HashSet<TypeId> = self.nodes.iter().copied().collect();
        let mut indegree: HashMap<TypeId, usize> = HashMap::new();
        for &n in &self.nodes {
            let deg = self
                .edges
                .get(&n)
                .map(|deps| deps.iter().filter(|d| known.contains(d)).count())
                .unwrap_or(0);
            indegree.insert(n, deg);
        }
        let mut queue: VecDeque<TypeId> = VecDeque::new();
        for &n in &self.nodes {
            if indegree[&n] == 0 {
                queue.push_back(n);
            }
        }
        let mut out = Vec::new();
        while let Some(n) = queue.pop_front() {
            out.push(n);
            for &consumer in &self.nodes {
                let Some(deps) = self.edges.get(&consumer) else {
                    continue;
                };
                if !deps.contains(&n) || !known.contains(&consumer) {
                    continue;
                }
                let Some(e) = indegree.get_mut(&consumer) else {
                    continue;
                };
                *e = e.saturating_sub(1);
                if *e == 0 {
                    queue.push_back(consumer);
                }
            }
        }
        if out.len() != self.nodes.len() {
            let cycle = self
                .find_cycle_path()
                .unwrap_or_else(|| vec!["<unknown>"]);
            return Err(vec![PluginGraphError::CyclicDependency {
                cycle,
                suggestion: "cycle detected during sort".to_string(),
            }]);
        }
        Ok(out)
    }

    fn find_cycle_path(&self) -> Option<Vec<&'static str>> {
        let known: HashSet<TypeId> = self.nodes.iter().copied().collect();
        let mut state: HashMap<TypeId, u8> = HashMap::new();
        let mut stack = Vec::new();
        for &n in &self.nodes {
            if state.get(&n).copied().unwrap_or(0) == 0 {
                if let Some(path) = self.visit_cycle(n, &known, &mut state, &mut stack) {
                    return Some(path);
                }
            }
        }
        None
    }

    fn visit_cycle(
        &self,
        u: TypeId,
        known: &HashSet<TypeId>,
        state: &mut HashMap<TypeId, u8>,
        st: &mut Vec<TypeId>,
    ) -> Option<Vec<&'static str>> {
        match state.get(&u).copied().unwrap_or(0) {
            2 => return None,
            1 => {
                let idx = st.iter().position(|&x| x == u)?;
                let mut cycle: Vec<&'static str> = st[idx..]
                    .iter()
                    .map(|id| *self.names.get(id).unwrap_or(&"?"))
                    .collect();
                cycle.push(*self.names.get(&u).unwrap_or(&"?"));
                return Some(cycle);
            }
            _ => {}
        }
        state.insert(u, 1);
        st.push(u);
        if let Some(deps) = self.edges.get(&u) {
            for &v in deps {
                if known.contains(&v) {
                    if let Some(c) = self.visit_cycle(v, known, state, st) {
                        return Some(c);
                    }
                }
            }
        }
        st.pop();
        state.insert(u, 2);
        None
    }
}
