//! Plugin discovery, dependency resolution, and editor widget registry (`TC-1.6.*`).

use std::any::TypeId;
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::Path;

/// Semantic version triple for host and plugin compatibility checks.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SemanticVersion {
    /// Major component.
    pub major: u16,
    /// Minor component.
    pub minor: u16,
    /// Patch component.
    pub patch: u16,
}

impl SemanticVersion {
    /// Constructs a version literal.
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

/// Declared dependency edge between plugins.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PluginDependency {
    /// Dependency plugin name.
    pub name: String,
}

/// Metadata discovered before loading a plugin package.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PluginMetadata {
    /// Unique package name.
    pub name: String,
    /// Package semantic version.
    pub version: SemanticVersion,
    /// Minimum compatible editor host version.
    pub min_host_version: SemanticVersion,
    /// Declared edges for ordering.
    pub dependencies: Vec<PluginDependency>,
}

/// Runtime handle for a loaded plugin.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PluginHandle {
    /// Stable id chosen by the host.
    pub id: u64,
    /// Package name.
    pub name: String,
    /// Current lifecycle state.
    pub state: PluginState,
}

/// High-level plugin lifecycle marker.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PluginState {
    /// Middleman compile or dlopen in progress.
    Loading,
    /// Panels and widgets registered.
    Active,
    /// Previous load faulted; host keeps diagnostics only.
    Faulted(String),
    /// Teardown in progress.
    Unloading,
}

/// Factory function type for custom inspector widgets (test stub).
pub type EditorWidgetFactory = fn() -> u32;

/// Maps component [`TypeId`] values to plugin-provided factories.
#[derive(Debug, Default)]
pub struct EditorWidgetRegistry {
    map: HashMap<TypeId, EditorWidgetFactory>,
}

impl EditorWidgetRegistry {
    /// Empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers `factory` for `ty`.
    pub fn register(&mut self, ty: TypeId, factory: EditorWidgetFactory) {
        self.map.insert(ty, factory);
    }

    /// Looks up a factory for `ty`.
    pub fn lookup(&self, ty: TypeId) -> Option<EditorWidgetFactory> {
        self.map.get(&ty).copied()
    }
}

/// Dependency graph resolution failures.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DependencyResolverError {
    /// Unknown dependency name referenced by a package.
    UnknownDependency(String),
    /// Cycle detected while resolving load order.
    CircularDependency,
    /// Host binary is too old for a plugin package.
    IncompatibleHost {
        /// Requesting plugin.
        plugin: String,
        /// Declared minimum host.
        min_host: SemanticVersion,
        /// Running host version.
        host: SemanticVersion,
    },
}

impl fmt::Display for DependencyResolverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DependencyResolverError::UnknownDependency(n) => {
                write!(f, "unknown dependency {n}")
            }
            DependencyResolverError::CircularDependency => write!(f, "circular dependency"),
            DependencyResolverError::IncompatibleHost { plugin, .. } => {
                write!(f, "incompatible host for plugin {plugin}")
            }
        }
    }
}

impl std::error::Error for DependencyResolverError {}

/// Topological ordering for plugin packages.
#[derive(Debug, Default)]
pub struct DependencyResolver {
    host_version: SemanticVersion,
}

impl DependencyResolver {
    /// Uses `host_version` when validating `min_host_version` claims.
    pub fn new(host_version: SemanticVersion) -> Self {
        Self { host_version }
    }

    /// Verifies each plugin can run on this host.
    pub fn check_host_compat(&self, meta: &PluginMetadata) -> Result<(), DependencyResolverError> {
        if self.host_version < meta.min_host_version {
            return Err(DependencyResolverError::IncompatibleHost {
                plugin: meta.name.clone(),
                min_host: meta.min_host_version,
                host: self.host_version,
            });
        }
        Ok(())
    }

    /// Returns load order oldest-first respecting dependency edges (Kahn).
    pub fn resolve(
        &self,
        plugins: &[PluginMetadata],
    ) -> Result<Vec<String>, DependencyResolverError> {
        let mut index: HashMap<String, usize> = HashMap::new();
        for (i, p) in plugins.iter().enumerate() {
            index.insert(p.name.clone(), i);
        }
        let mut indegree: Vec<usize> = vec![0; plugins.len()];
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); plugins.len()];
        for (i, p) in plugins.iter().enumerate() {
            for d in &p.dependencies {
                let Some(&j) = index.get(&d.name) else {
                    return Err(DependencyResolverError::UnknownDependency(d.name.clone()));
                };
                adj[j].push(i);
                indegree[i] += 1;
            }
        }
        let mut q: VecDeque<usize> = VecDeque::new();
        for (i, d) in indegree.iter().enumerate() {
            if *d == 0 {
                q.push_back(i);
            }
        }
        let mut order = Vec::new();
        while let Some(u) = q.pop_front() {
            order.push(plugins[u].name.clone());
            for &v in &adj[u] {
                indegree[v] -= 1;
                if indegree[v] == 0 {
                    q.push_back(v);
                }
            }
        }
        if order.len() != plugins.len() {
            return Err(DependencyResolverError::CircularDependency);
        }
        Ok(order)
    }
}

/// Executes plugin-provided callbacks inside an isolation boundary.
#[derive(Debug, Default)]
pub struct IsolationScope;

impl IsolationScope {
    /// Runs `f`, translating panics into a stable error string.
    pub fn catch_panic<F, R>(&self, f: F) -> Result<R, &'static str>
    where
        F: FnOnce() -> R,
    {
        catch_unwind(AssertUnwindSafe(f)).map_err(|_| "panic")
    }
}

/// Hot reload failures.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HotReloadError {
    /// Handle was not found in the loaded table.
    MissingHandle,
}

impl fmt::Display for HotReloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HotReloadError::MissingHandle => write!(f, "missing plugin handle"),
        }
    }
}

impl std::error::Error for HotReloadError {}

/// Owns discovered metadata and simulated load transitions.
#[derive(Debug)]
pub struct PluginHost {
    resolver: DependencyResolver,
    discovered: Vec<PluginMetadata>,
    loaded: Vec<PluginHandle>,
    next_id: u64,
}

impl PluginHost {
    /// Creates a host with `host_version`.
    pub fn new(host_version: SemanticVersion) -> Self {
        Self {
            resolver: DependencyResolver::new(host_version),
            discovered: Vec::new(),
            loaded: Vec::new(),
            next_id: 1,
        }
    }

    /// Scans `paths` for `.meta` sibling files (test stub: uses in-memory fixtures only).
    pub fn discover(&mut self, paths: &[impl AsRef<Path>]) -> Vec<PluginMetadata> {
        for p in paths {
            let _ = p.as_ref();
        }
        self.discovered.clone()
    }

    /// Seeds discovery results for unit tests.
    pub fn seed_discovered(&mut self, metas: Vec<PluginMetadata>) {
        self.discovered = metas;
    }

    /// Simulates load after dependency resolution.
    pub fn load(&mut self, name: &str) -> Result<PluginHandle, DependencyResolverError> {
        let meta = self
            .discovered
            .iter()
            .find(|m| m.name == name)
            .ok_or_else(|| DependencyResolverError::UnknownDependency(name.to_string()))?;
        self.resolver.check_host_compat(meta)?;
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);
        let h = PluginHandle {
            id,
            name: name.to_string(),
            state: PluginState::Loading,
        };
        self.loaded.push(PluginHandle {
            state: PluginState::Active,
            ..h.clone()
        });
        Ok(PluginHandle {
            state: PluginState::Active,
            ..h
        })
    }

    /// Simulates hot reload by cycling through loading and active states.
    pub fn hot_reload(&mut self, handle: &PluginHandle) -> Result<PluginHandle, HotReloadError> {
        let pos = self
            .loaded
            .iter()
            .position(|h| h.id == handle.id)
            .ok_or(HotReloadError::MissingHandle)?;
        self.loaded[pos].state = PluginState::Loading;
        self.loaded[pos].state = PluginState::Active;
        Ok(self.loaded[pos].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn meta(name: &str, deps: &[&str], min_host: SemanticVersion) -> PluginMetadata {
        PluginMetadata {
            name: name.to_string(),
            version: SemanticVersion::new(1, 0, 0),
            min_host_version: min_host,
            dependencies: deps
                .iter()
                .map(|d| PluginDependency {
                    name: (*d).to_string(),
                })
                .collect(),
        }
    }

    #[test]
    fn tc_1_6_1_1_plugin_discovery() {
        let mut host = PluginHost::new(SemanticVersion::new(1, 0, 0));
        host.seed_discovered(vec![meta("a", &[], SemanticVersion::new(0, 9, 0))]);
        let d = host.discover(&["/tmp/project/plugins/a.meta"]);
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].name, "a");
    }

    #[test]
    fn tc_1_6_2_1_plugin_load_lifecycle() {
        let mut host = PluginHost::new(SemanticVersion::new(1, 0, 0));
        host.seed_discovered(vec![meta("solo", &[], SemanticVersion::new(0, 9, 0))]);
        let h = host.load("solo").unwrap();
        assert_eq!(h.state, PluginState::Active);
    }

    #[test]
    fn tc_1_6_3_1_plugin_hot_reload_state() {
        let mut host = PluginHost::new(SemanticVersion::new(1, 0, 0));
        host.seed_discovered(vec![meta("hot", &[], SemanticVersion::new(0, 9, 0))]);
        let h = host.load("hot").unwrap();
        let h2 = host.hot_reload(&h).unwrap();
        assert_eq!(h2.state, PluginState::Active);
    }

    #[test]
    fn tc_1_6_4_1_plugin_panic_isolation() {
        let scope = IsolationScope;
        let r = scope.catch_panic(|| panic!("boom"));
        assert!(r.is_err());
    }

    #[test]
    fn tc_1_6_5_1_dependency_resolution() {
        let r = DependencyResolver::new(SemanticVersion::new(1, 0, 0));
        let plugins = vec![
            meta("a", &["b"], SemanticVersion::new(0, 9, 0)),
            meta("b", &[], SemanticVersion::new(0, 9, 0)),
        ];
        let order = r.resolve(&plugins).unwrap();
        assert_eq!(order, vec!["b", "a"]);
    }

    #[test]
    fn tc_1_6_5_2_circular_dependency() {
        let r = DependencyResolver::new(SemanticVersion::new(1, 0, 0));
        let plugins = vec![
            meta("a", &["b"], SemanticVersion::new(0, 9, 0)),
            meta("b", &["a"], SemanticVersion::new(0, 9, 0)),
        ];
        assert_eq!(
            r.resolve(&plugins).unwrap_err(),
            DependencyResolverError::CircularDependency
        );
    }

    #[test]
    fn tc_1_6_6_1_abi_version_compat() {
        let r = DependencyResolver::new(SemanticVersion::new(1, 0, 0));
        let p = meta("need_new_host", &[], SemanticVersion::new(1, 1, 0));
        assert_eq!(
            r.check_host_compat(&p).unwrap_err(),
            DependencyResolverError::IncompatibleHost {
                plugin: "need_new_host".to_string(),
                min_host: SemanticVersion::new(1, 1, 0),
                host: SemanticVersion::new(1, 0, 0),
            }
        );
    }

    fn sample_factory() -> u32 {
        42
    }

    #[test]
    fn tc_1_6_7_1_custom_widget_register() {
        let mut reg = EditorWidgetRegistry::new();
        reg.register(TypeId::of::<()>(), sample_factory);
        assert!(reg.lookup(TypeId::of::<()>()).is_some());
    }

    #[test]
    fn tc_1_6_7_2_widget_lookup_dispatch() {
        let mut reg = EditorWidgetRegistry::new();
        reg.register(TypeId::of::<()>(), sample_factory);
        let f = reg.lookup(TypeId::of::<()>()).unwrap();
        assert_eq!(f(), 42);
    }
}
