//! Capability registry for optional engine features.

use std::collections::BTreeMap;

use super::semver::SemVer;

/// Named capability with a semantic version.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Capability {
    /// Stable capability name.
    pub name: &'static str,
    /// Advertised version.
    pub version: SemVer,
}

/// Maps capability names to their advertised versions.
#[derive(Debug, Default)]
pub struct CapabilityRegistry {
    caps: BTreeMap<&'static str, Capability>,
}

impl CapabilityRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts or replaces a capability entry.
    pub fn register(&mut self, cap: Capability) {
        self.caps.insert(cap.name, cap);
    }

    /// Removes a capability by name.
    pub fn unregister(&mut self, name: &'static str) {
        self.caps.remove(name);
    }

    /// Returns true when the capability exists.
    pub fn has(&self, name: &str) -> bool {
        self.caps.contains_key(name)
    }

    /// Borrows a capability by name.
    pub fn get(&self, name: &str) -> Option<&Capability> {
        self.caps.get(name)
    }

    /// True when the capability exists and meets the minimum version.
    pub fn meets_version(&self, name: &str, min_version: SemVer) -> bool {
        self.get(name)
            .map(|c| c.version >= min_version)
            .unwrap_or(false)
    }
}
