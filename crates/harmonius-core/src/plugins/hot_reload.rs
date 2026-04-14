//! Development-only hot reload manager (simulated for tests).

use std::collections::BTreeMap;

use crate::world::World;

use super::app::PluginDescriptor;

/// Errors surfaced by [`HotReloadManager::reload`].
#[derive(Debug, Eq, PartialEq)]
pub enum HotReloadError {
    /// Codegen failed (simulated).
    CodegenFailed {
        /// Reason string.
        reason: String,
    },
    /// Compile failed (simulated).
    CompileFailed {
        /// Reason string.
        reason: String,
    },
    /// State migration failed (simulated).
    StateMigrationFailed {
        /// Type name.
        type_name: String,
        /// Reason string.
        reason: String,
    },
}

/// Tracks loaded plugin descriptors and reload simulation hooks.
#[derive(Debug, Default)]
pub struct HotReloadManager {
    loaded: BTreeMap<String, LoadedPlugin>,
    pub(crate) migration_should_fail: bool,
    pub(crate) reload_latency_ms: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct LoadedPlugin {
    descriptor: PluginDescriptor,
    state_marker: u64,
}

impl HotReloadManager {
    /// Creates an empty manager.
    pub fn new() -> Self {
        Self::default()
    }

    /// Marks whether the next migration should fail (test hook).
    pub fn set_migration_should_fail(&mut self, fail: bool) {
        self.migration_should_fail = fail;
    }

    /// Simulated reload latency in milliseconds (test hook).
    pub fn set_reload_latency_ms(&mut self, ms: u64) {
        self.reload_latency_ms = ms;
    }

    /// Registers a plugin as loaded.
    pub fn register_loaded(&mut self, descriptor: PluginDescriptor, state_marker: u64) {
        self.loaded.insert(
            descriptor.name.to_string(),
            LoadedPlugin {
                descriptor,
                state_marker,
            },
        );
    }

    /// Simulates a full reload cycle for the named plugin.
    pub fn reload(&mut self, name: &str, world: &mut World) -> Result<(), HotReloadError> {
        #[cfg(test)]
        if self.reload_latency_ms > 0 {
            std::thread::sleep(std::time::Duration::from_millis(self.reload_latency_ms));
        }
        if self.migration_should_fail {
            return Err(HotReloadError::StateMigrationFailed {
                type_name: name.to_string(),
                reason: "simulated migration failure".to_string(),
            });
        }
        let Some(entry) = self.loaded.get_mut(name) else {
            return Err(HotReloadError::CodegenFailed {
                reason: format!("unknown plugin {name}"),
            });
        };
        entry.state_marker = entry.state_marker.saturating_add(1);
        world.insert_resource(HotReloadMarker {
            plugin: name.to_string(),
            tick: entry.state_marker,
        });
        Ok(())
    }

    /// Returns true when the plugin is currently tracked as loaded.
    pub fn is_loaded(&self, name: &str) -> bool {
        self.loaded.contains_key(name)
    }
}

/// Resource written after a successful simulated reload.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HotReloadMarker {
    /// Plugin name that reloaded.
    pub plugin: String,
    /// Monotonic marker bumped each reload.
    pub tick: u64,
}
