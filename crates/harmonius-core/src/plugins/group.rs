//! Plugin groups and preset bundles.

use std::any::TypeId;
use std::collections::BTreeSet;

use super::app::Plugin;

/// Builds a list of boxed plugins for group registration.
pub struct PluginGroupBuilder {
    entries: Vec<(TypeId, Box<dyn Plugin>)>,
    disabled: BTreeSet<TypeId>,
}

impl PluginGroupBuilder {
    /// Creates an empty builder.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            disabled: BTreeSet::new(),
        }
    }

    /// Adds a plugin instance to the group.
    #[allow(clippy::should_implement_trait)]
    pub fn add<P: Plugin>(mut self, plugin: P) -> Self {
        self.entries.push((TypeId::of::<P>(), Box::new(plugin)));
        self
    }

    /// Disables a plugin type before finalizing the group.
    pub fn disable<P: Plugin>(mut self) -> Self {
        self.disabled.insert(TypeId::of::<P>());
        self
    }

    /// Returns active plugins with their concrete `TypeId` keys.
    pub fn finish(self) -> Vec<(TypeId, Box<dyn Plugin>)> {
        self.entries
            .into_iter()
            .filter(|(id, _)| !self.disabled.contains(id))
            .collect()
    }
}

impl Default for PluginGroupBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait implemented by named plugin bundles.
pub trait PluginGroup {
    /// Populates a [`PluginGroupBuilder`].
    fn build(self) -> PluginGroupBuilder;
}
