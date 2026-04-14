//! `App` builder and [`Plugin`] trait definitions.

use std::any::TypeId;
use std::collections::HashMap;

use crate::world::{World, WorldId};

use super::capability::{Capability, CapabilityRegistry};
use super::graph::{DependencyGraph, PluginGraphError};
use super::semver::SemVer;

/// Build-time metadata carried by every plugin.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PluginDescriptor {
    /// Human-readable plugin name.
    pub name: &'static str,
    /// Semantic version of the plugin package.
    pub version: SemVer,
    /// Stable fingerprint used for compatibility gates (TC-1.6.6); aligns with test cases.
    pub abi_hash: u64,
}

/// A composable module registered into an [`App`].
pub trait Plugin: Send + Sync + 'static {
    /// Returns the descriptor for diagnostics and compatibility checks.
    fn descriptor(&self) -> PluginDescriptor;

    /// Contributes systems, resources, and events into the app.
    fn build(&self, app: &mut App);

    /// Declares hard dependencies that must initialize first.
    fn dependencies(&self) -> Vec<TypeId> {
        Vec::new()
    }

    /// Declares mutually exclusive plugins.
    fn conflicts(&self) -> Vec<TypeId> {
        Vec::new()
    }

    /// Optional human-readable labels for [`dependencies`](Self::dependencies) edges.
    fn labeled_dependencies(&self) -> Vec<(TypeId, &'static str)> {
        self.dependencies()
            .into_iter()
            .map(|id| (id, "unknown"))
            .collect()
    }

    /// Optional capability advertisements.
    fn capabilities(&self) -> Vec<Capability> {
        Vec::new()
    }
}

/// Top-level plugin composition error.
#[derive(Debug, Eq, PartialEq)]
pub enum PluginError {
    /// Graph validation failed with one or more issues.
    GraphErrors(Vec<PluginGraphError>),
    /// Duplicate plugin type registration.
    DuplicatePlugin {
        /// Conflicting plugin name.
        name: &'static str,
    },
    /// ABI compatibility failure.
    AbiMismatch {
        /// Human-readable reason.
        reason: String,
    },
}

/// Records plugin registration order and contributions.
pub struct App {
    world: World,
    plugins: Vec<(TypeId, Box<dyn Plugin>)>,
    graph: DependencyGraph,
    pub(crate) capability_registry: CapabilityRegistry,
    pub(crate) systems: Vec<&'static str>,
    expected_abi: Option<u64>,
}

impl App {
    /// Creates an app targeting a fresh world.
    pub fn new(world: World) -> Self {
        Self {
            world,
            plugins: Vec::new(),
            graph: DependencyGraph::new(),
            capability_registry: CapabilityRegistry::new(),
            systems: Vec::new(),
            expected_abi: None,
        }
    }

    /// Creates an app with default `WorldId` zero.
    pub fn with_default_world() -> Self {
        Self::new(World::new(WorldId(0)))
    }

    /// Optional ABI gate applied during `add_plugin` / `add_plugins`.
    pub fn with_expected_abi(mut self, abi: u64) -> Self {
        self.expected_abi = Some(abi);
        self
    }

    /// Registers a single plugin instance.
    pub fn add_plugin<P: Plugin>(&mut self, plugin: P) -> Result<&mut Self, PluginError> {
        let id = TypeId::of::<P>();
        if self.plugins.iter().any(|(i, _)| *i == id) {
            return Err(PluginError::DuplicatePlugin {
                name: plugin.descriptor().name,
            });
        }
        if let Some(expected) = self.expected_abi {
            if plugin.descriptor().abi_hash != expected {
                return Err(PluginError::AbiMismatch {
                    reason: format!(
                        "plugin {} abi {} != expected {}",
                        plugin.descriptor().name,
                        plugin.descriptor().abi_hash,
                        expected
                    ),
                });
            }
        }
        self.graph.add_plugin(id, plugin.descriptor().name, &plugin);
        self.plugins.push((id, Box::new(plugin)));
        Ok(self)
    }

    /// Registers boxed plugins produced by [`super::PluginGroupBuilder::finish`].
    pub fn add_plugins(
        &mut self,
        entries: Vec<(TypeId, Box<dyn Plugin>)>,
    ) -> Result<&mut Self, PluginError> {
        for (id, plugin) in entries {
            self.add_plugin_boxed(id, plugin)?;
        }
        Ok(self)
    }

    fn add_plugin_boxed(&mut self, id: TypeId, plugin: Box<dyn Plugin>) -> Result<(), PluginError> {
        if self.plugins.iter().any(|(i, _)| *i == id) {
            return Err(PluginError::DuplicatePlugin {
                name: plugin.descriptor().name,
            });
        }
        if let Some(expected) = self.expected_abi {
            if plugin.descriptor().abi_hash != expected {
                return Err(PluginError::AbiMismatch {
                    reason: format!(
                        "plugin {} abi {} != expected {}",
                        plugin.descriptor().name,
                        plugin.descriptor().abi_hash,
                        expected
                    ),
                });
            }
        }
        self.graph
            .add_plugin(id, plugin.descriptor().name, plugin.as_ref());
        self.plugins.push((id, plugin));
        Ok(())
    }

    /// Records a system name contribution (test hook).
    pub fn register_system(&mut self, name: &'static str) {
        self.systems.push(name);
    }

    /// Mutable world access for plugins.
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    /// Validates the graph, sorts plugins, and runs `build` in order.
    pub fn build(self) -> Result<BuiltApp, PluginError> {
        if let Err(e) = self.graph.validate() {
            return Err(PluginError::GraphErrors(e));
        }
        let order = self
            .graph
            .topological_sort()
            .map_err(PluginError::GraphErrors)?;
        let mut plugins_map: HashMap<TypeId, Box<dyn Plugin>> = self.plugins.into_iter().collect();
        let mut init_order = Vec::new();
        let mut world = self.world;
        let mut capability_registry = self.capability_registry;
        let mut systems = self.systems;
        for id in order {
            let Some(plugin) = plugins_map.remove(&id) else {
                continue;
            };
            init_order.push(plugin.descriptor().name);
            for cap in plugin.capabilities() {
                capability_registry.register(cap);
            }
            let mut nested = App {
                world,
                plugins: Vec::new(),
                graph: DependencyGraph::new(),
                capability_registry,
                systems: systems.clone(),
                expected_abi: None,
            };
            plugin.build(&mut nested);
            world = nested.world;
            capability_registry = nested.capability_registry;
            systems = nested.systems;
        }
        Ok(BuiltApp {
            world,
            capability_registry,
            init_order,
            systems,
        })
    }
}

/// Finalized application state after plugin initialization succeeds.
#[derive(Debug)]
pub struct BuiltApp {
    /// Simulation world owned by the app.
    pub world: World,
    /// Capability advertisements merged from all plugins.
    pub capability_registry: CapabilityRegistry,
    /// Plugin initialization order (names).
    pub init_order: Vec<&'static str>,
    /// Registered system names in insertion order.
    pub systems: Vec<&'static str>,
}

/// Verifies a plugin descriptor ABI hash against an engine expectation.
pub fn verify_plugin_abi(descriptor: &PluginDescriptor, expected: u64) -> Result<(), PluginError> {
    if descriptor.abi_hash != expected {
        return Err(PluginError::AbiMismatch {
            reason: format!(
                "abi mismatch for {}: got {}, expected {}",
                descriptor.name, descriptor.abi_hash, expected
            ),
        });
    }
    Ok(())
}
