//! Plugin registration, dependency graphs, capabilities, and app build.

mod app;
mod capability;
mod graph;
mod group;
mod hot_reload;
mod semver;

pub use app::{verify_plugin_abi, App, BuiltApp, Plugin, PluginDescriptor, PluginError};
pub use capability::{Capability, CapabilityRegistry};
pub use graph::{DependencyGraph, PluginGraphError};
pub use group::{PluginGroup, PluginGroupBuilder};
pub use hot_reload::{HotReloadError, HotReloadManager, HotReloadMarker};
pub use semver::SemVer;
