//! On-disk `PluginManifest` layout and helpers (rkyv archive).

use crate::types::Blake3Hash;

/// Reference to a blob stored in CAS.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    ::rkyv_derive::Archive,
    ::rkyv_derive::Deserialize,
    ::rkyv_derive::Serialize,
)]
pub struct BlobRef {
    /// BLAKE3 digest of blob bytes.
    pub hash: [u8; 32],
    /// Byte length of the blob.
    pub size: u64,
}

/// Role of a file entry in a plugin package.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    ::rkyv_derive::Archive,
    ::rkyv_derive::Deserialize,
    ::rkyv_derive::Serialize,
)]
pub enum FileRole {
    /// Codegen input consumed by middleman.
    CodegenSource,
    /// Arbitrary asset payload.
    Asset,
    /// License text.
    License,
    /// Store preview asset.
    Preview,
    /// Extra documentation.
    Documentation,
}

/// One logical file inside a package pointing at a CAS blob.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ::rkyv_derive::Archive,
    ::rkyv_derive::Deserialize,
    ::rkyv_derive::Serialize,
)]
pub struct FileEntry {
    /// Relative path inside the package.
    pub path: String,
    /// CAS reference.
    pub blob: BlobRef,
    /// Classification for install and UI.
    pub role: FileRole,
}

/// Middleman codegen entry point description.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ::rkyv_derive::Archive,
    ::rkyv_derive::Deserialize,
    ::rkyv_derive::Serialize,
)]
pub enum CodegenEntry {
    /// Rust module path for generated glue.
    MiddlemanModule {
        /// Module name.
        module: String,
    },
}

/// Bitmask of supported target platforms (design uses a bitset; u32 is sufficient for tests).
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    ::rkyv_derive::Archive,
    ::rkyv_derive::Deserialize,
    ::rkyv_derive::Serialize,
)]
pub struct PlatformSet(pub u32);

/// Serialized dependency requirement inside an archived manifest.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ::rkyv_derive::Archive,
    ::rkyv_derive::Deserialize,
    ::rkyv_derive::Serialize,
)]
pub struct ManifestDependency {
    /// Target plugin id.
    pub id: String,
    /// Semver range text (e.g. `^1.0`).
    pub range: String,
}

/// Root manifest for a plugin package (rkyv-serialized on wire / disk).
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    ::rkyv_derive::Archive,
    ::rkyv_derive::Deserialize,
    ::rkyv_derive::Serialize,
)]
pub struct PluginManifest {
    /// Stable plugin id.
    pub id: String,
    /// This package version.
    pub version: String,
    /// Declared author / publisher id.
    pub author: String,
    /// Human-readable name.
    pub display_name: String,
    /// Longer description.
    pub description: String,
    /// Codegen entry.
    pub entry_point: CodegenEntry,
    /// Dependency edges.
    pub dependencies: Vec<ManifestDependency>,
    /// Packaged files.
    pub files: Vec<FileEntry>,
    /// Compatible engine semver range text.
    pub engine_range: String,
    /// Supported platforms.
    pub platforms: PlatformSet,
}

/// Count files grouped by [`FileRole`].
#[must_use]
pub fn file_role_counts(manifest: &PluginManifest) -> [usize; 5] {
    let mut counts = [0usize; 5];
    for f in &manifest.files {
        let idx = match f.role {
            FileRole::CodegenSource => 0,
            FileRole::Asset => 1,
            FileRole::License => 2,
            FileRole::Preview => 3,
            FileRole::Documentation => 4,
        };
        counts[idx] += 1;
    }
    counts
}

/// Serialize a manifest to an owned rkyv byte buffer (aligned for access).
pub fn manifest_to_bytes(manifest: &PluginManifest) -> Result<Vec<u8>, rkyv::rancor::Error> {
    Ok(rkyv::to_bytes(manifest)?.into_vec())
}

/// Deserialize a manifest from an rkyv buffer, checking bounds and alignment.
pub fn manifest_from_bytes(bytes: &[u8]) -> Result<PluginManifest, rkyv::rancor::Error> {
    let archived = rkyv::access::<ArchivedPluginManifest, rkyv::rancor::Error>(bytes)?;
    rkyv::deserialize::<PluginManifest, rkyv::rancor::Error>(archived)
}

/// Round-trip through rkyv; returns true if re-serialized bytes match exactly.
#[must_use]
pub fn manifest_rkyv_roundtrip_bitidentical(manifest: &PluginManifest) -> bool {
    let Ok(a) = manifest_to_bytes(manifest) else {
        return false;
    };
    let Ok(round) = manifest_from_bytes(&a) else {
        return false;
    };
    if &round != manifest {
        return false;
    }
    let Ok(b) = manifest_to_bytes(&round) else {
        return false;
    };
    a == b
}

/// Hash manifest bytes for signing (BLAKE3 digest over the canonical archive bytes).
#[must_use]
pub fn manifest_signing_digest(manifest_bytes: &[u8]) -> Blake3Hash {
    Blake3Hash(*blake3::hash(manifest_bytes).as_bytes())
}
