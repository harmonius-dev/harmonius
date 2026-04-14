//! Core identifiers and shared types for the plugin marketplace protocol.

use semver::Version;

/// Content-addressed blob digest (BLAKE3, 32 bytes).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Blake3Hash(pub [u8; 32]);

/// Plugin identifier (stable string id from the registry).
pub type PluginId = String;

/// Publisher identifier string.
pub type PublisherId = String;

/// A semver dependency edge requested by a manifest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DependencyReq {
    /// Target plugin id.
    pub id: PluginId,
    /// Accepted version range.
    pub range: semver::VersionReq,
}

/// Plugin already present in the local store.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstalledPlugin {
    /// Installed plugin id.
    pub id: PluginId,
    /// Installed semver.
    pub version: Version,
}
