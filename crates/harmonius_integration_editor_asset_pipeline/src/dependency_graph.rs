//! Deterministic dependency edges for IR-9.2.2 cascade and cycle detection.

use std::collections::BTreeMap;
use std::path::PathBuf;

/// Maps each upstream source path to sorted dependents that consume it.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DependencyGraph {
    downstream: BTreeMap<PathBuf, Vec<PathBuf>>,
}

impl DependencyGraph {
    /// Registers `dependent` as consuming `upstream` (re-importing `upstream` may cascade here).
    pub fn add_edge(&mut self, dependent: PathBuf, upstream: PathBuf) {
        let entry = self.downstream.entry(upstream).or_default();
        if !entry.contains(&dependent) {
            entry.push(dependent);
            entry.sort();
        }
    }

    /// Removes a single `dependent` → `upstream` edge (stale edge cleanup).
    pub fn remove_edge(&mut self, dependent: &PathBuf, upstream: &PathBuf) {
        let Some(entry) = self.downstream.get_mut(upstream) else {
            return;
        };
        entry.retain(|p| p != dependent);
        if entry.is_empty() {
            self.downstream.remove(upstream);
        }
    }

    /// Dependents that should be considered after `upstream` finishes importing.
    pub fn dependents(&self, upstream: &PathBuf) -> &[PathBuf] {
        self.downstream
            .get(upstream)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }
}
