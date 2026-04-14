//! Semver dependency resolution with deterministic ordering and backtracking.

use crate::types::{DependencyReq, InstalledPlugin, PluginId};
use semver::Version;
use std::collections::{BTreeMap, HashMap, VecDeque};

/// Catalog surface required for resolution.
pub trait ManifestSource {
    /// Engine version the resolver assumes for `engine_range` checks.
    fn current_engine(&self) -> Version;

    /// All published versions for `id` (any order; resolver sorts).
    fn list_versions(&self, id: &PluginId) -> Vec<Version>;

    /// Dependency edges and engine requirement for an exact version, if present.
    fn manifest_constraints(
        &self,
        id: &PluginId,
        version: &Version,
    ) -> Option<(semver::VersionReq, Vec<DependencyReq>)>;
}

/// Successful resolution output.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedSet {
    /// Chosen id->version pairs.
    pub plugins: Vec<(PluginId, Version)>,
    /// Topological install order (dependencies first).
    pub order: Vec<PluginId>,
}

/// Resolution failure reasons.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum ResolveError {
    /// No semver satisfied the combined constraints for a plugin.
    #[error("no compatible version for {plugin}")]
    NoCompatibleVersion {
        /// Plugin id.
        plugin: PluginId,
    },
    /// Two hard requirements could not be satisfied together.
    #[error("conflicting ranges for {plugin}")]
    ConflictingRanges {
        /// Plugin id.
        plugin: PluginId,
    },
    /// Dependency graph contains a directed cycle.
    #[error("dependency cycle")]
    Cycle,
    /// Engine compatibility range not met for a plugin.
    #[error("engine range not met for {plugin}")]
    EngineRangeUnmet {
        /// Plugin id.
        plugin: PluginId,
    },
}

/// Stateless resolver entry point.
#[derive(Debug, Default)]
pub struct Resolver;

impl Resolver {
    /// Resolve `wanted` against `installed`, querying `catalog` for manifests.
    pub fn resolve(
        wanted: &[DependencyReq],
        installed: &[InstalledPlugin],
        catalog: &dyn ManifestSource,
    ) -> Result<ResolvedSet, ResolveError> {
        let engine = catalog.current_engine();
        let mut assigned: BTreeMap<PluginId, Version> = BTreeMap::new();
        for p in installed {
            assigned.insert(p.id.clone(), p.version.clone());
        }
        let mut merged: Vec<(PluginId, semver::VersionReq)> = wanted
            .iter()
            .map(|w| (w.id.clone(), w.range.clone()))
            .collect();
        merged.sort_by(|a, b| a.0.cmp(&b.0));
        let mut stack: Vec<PluginId> = Vec::new();
        for (id, range) in merged {
            Self::ensure_plugin(&id, &range, &mut assigned, catalog, &engine, &mut stack)?;
        }
        let order = topo_order(&assigned, catalog)?;
        let plugins: Vec<(PluginId, Version)> = assigned.into_iter().collect();
        Ok(ResolvedSet { plugins, order })
    }

    fn ensure_plugin(
        plugin: &PluginId,
        range: &semver::VersionReq,
        assigned: &mut BTreeMap<PluginId, Version>,
        catalog: &dyn ManifestSource,
        engine: &Version,
        stack: &mut Vec<PluginId>,
    ) -> Result<(), ResolveError> {
        if let Some(existing) = assigned.get(plugin) {
            return if range.matches(existing) {
                Ok(())
            } else {
                Err(ResolveError::ConflictingRanges {
                    plugin: plugin.clone(),
                })
            };
        }
        if stack.iter().any(|s| s == plugin) {
            return Err(ResolveError::Cycle);
        }
        stack.push(plugin.clone());
        let mut candidates = catalog.list_versions(plugin);
        candidates.sort_by(|a, b| b.cmp(a));
        let mut saw_range_match = false;
        let mut saw_engine_fail = false;
        for cand in candidates.iter() {
            if !range.matches(cand) {
                continue;
            }
            saw_range_match = true;
            let Some((eng_range, mut deps)) = catalog.manifest_constraints(plugin, cand) else {
                continue;
            };
            if !eng_range.matches(engine) {
                saw_engine_fail = true;
                continue;
            }
            deps.sort_by(|a, b| a.id.cmp(&b.id));
            assigned.insert(plugin.clone(), cand.clone());
            let mut failed = None;
            for d in &deps {
                match Self::ensure_plugin(&d.id, &d.range, assigned, catalog, engine, stack) {
                    Ok(()) => {}
                    Err(e @ ResolveError::Cycle)
                    | Err(e @ ResolveError::ConflictingRanges { .. }) => {
                        assigned.remove(plugin);
                        stack.pop();
                        return Err(e);
                    }
                    Err(e) => {
                        failed = Some(e);
                        break;
                    }
                }
            }
            if failed.is_none() {
                stack.pop();
                return Ok(());
            }
            assigned.remove(plugin);
        }
        stack.pop();
        if saw_engine_fail && saw_range_match {
            return Err(ResolveError::EngineRangeUnmet {
                plugin: plugin.clone(),
            });
        }
        Err(ResolveError::NoCompatibleVersion {
            plugin: plugin.clone(),
        })
    }
}

fn topo_order(
    assigned: &BTreeMap<PluginId, Version>,
    catalog: &dyn ManifestSource,
) -> Result<Vec<PluginId>, ResolveError> {
    let mut edges: HashMap<PluginId, Vec<PluginId>> = HashMap::new();
    let mut indegree: HashMap<PluginId, usize> = HashMap::new();
    for id in assigned.keys() {
        indegree.entry(id.clone()).or_insert(0);
    }
    for (id, ver) in assigned {
        let Some((_eng, deps)) = catalog.manifest_constraints(id, ver) else {
            continue;
        };
        for d in deps {
            if assigned.contains_key(&d.id) {
                edges.entry(d.id.clone()).or_default().push(id.clone());
                *indegree.entry(id.clone()).or_insert(0) += 1;
            }
        }
    }
    let mut q: VecDeque<PluginId> = VecDeque::new();
    let mut keys: Vec<PluginId> = assigned.keys().cloned().collect();
    keys.sort();
    for k in &keys {
        if *indegree.get(k).unwrap_or(&0) == 0 {
            q.push_back(k.clone());
        }
    }
    let mut out = Vec::new();
    while let Some(n) = q.pop_front() {
        out.push(n.clone());
        if let Some(ch) = edges.get(&n) {
            let mut chs = ch.clone();
            chs.sort();
            for m in chs {
                let e = indegree.entry(m.clone()).or_insert(0);
                *e -= 1;
                if *e == 0 {
                    q.push_back(m);
                }
            }
        }
    }
    if out.len() != assigned.len() {
        return Err(ResolveError::Cycle);
    }
    Ok(out)
}
