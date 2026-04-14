//! Local CAS layout, blob fetch coordination, and install / uninstall hooks.

use crate::manifest::{manifest_to_bytes, PluginManifest};
use crate::resolve::ResolvedSet;
use crate::trust::{Signature, TrustStore};
use crate::types::{Blake3Hash, PluginId};
use semver::Version;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Install-time failures (IO, integrity, codegen).
#[derive(Debug, thiserror::Error)]
pub enum InstallError {
    /// Underlying filesystem error.
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    /// Blob bytes did not match the declared BLAKE3 digest.
    #[error("integrity check failed for blob")]
    IntegrityFailure,
    /// Codegen or middleman rebuild failed.
    #[error("codegen failed: {0}")]
    CodegenFailed(String),
    /// Manifest trust or detached signature verification failed.
    #[error("trust verification failed")]
    Trust(#[from] crate::trust::TrustError),
    /// No detached signature was supplied for a resolved plugin version.
    #[error("missing detached signature for resolved plugin")]
    MissingSignature,
}

/// Summary counters for an install pass.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct InstallReport {
    /// Blobs fetched from the network (or injected source), not served from CAS.
    pub fetched_blobs: u32,
}

/// Fetches blob bytes when absent from local CAS.
pub trait BlobSource {
    /// Retrieve raw bytes for `hash` from remote or fixture source.
    fn fetch_blob(&mut self, hash: Blake3Hash) -> Result<Vec<u8>, InstallError>;
}

/// Middleman / codegen rebuild port.
pub trait CodegenPort {
    /// Regenerate artifacts for the currently installed set.
    fn rebuild(&mut self) -> Result<(), InstallError>;
}

/// Filesystem-backed CAS + install layout under a store root.
#[derive(Clone, Debug)]
pub struct InstallPipeline<B, C> {
    /// Store root (e.g. `.harmonius/plugins/`).
    pub root: PathBuf,
    blob_source: B,
    codegen: C,
}

impl<B: BlobSource, C: CodegenPort> InstallPipeline<B, C> {
    /// Create a pipeline rooted at `root`.
    pub fn new(root: PathBuf, blob_source: B, codegen: C) -> Self {
        Self {
            root,
            blob_source,
            codegen,
        }
    }

    /// Install all plugins in `set` using `manifests` and matching detached `signatures`.
    ///
    /// Verifies each manifest's detached signature against `trust` using the canonical
    /// serialized manifest bytes before staging blobs or writing CAS entries.
    pub fn install(
        &mut self,
        set: &ResolvedSet,
        manifests: &[(PluginId, Version, PluginManifest)],
        trust: &TrustStore,
        signatures: &[(PluginId, Version, Signature)],
    ) -> Result<InstallReport, InstallError> {
        let mut report = InstallReport::default();
        let mut staged: Vec<(PluginId, PluginManifest)> = Vec::new();
        for id in &set.order {
            let ver = set
                .plugins
                .iter()
                .find(|(i, _)| i == id)
                .map(|(_, v)| v.clone())
                .ok_or_else(|| {
                    InstallError::CodegenFailed("resolved set missing plugin version".into())
                })?;
            let manifest = manifests
                .iter()
                .find(|(i, v, _)| i == id && v == &ver)
                .map(|(_, _, m)| m)
                .ok_or_else(|| {
                    InstallError::CodegenFailed("missing manifest for resolved plugin".into())
                })?;
            let sig = signatures
                .iter()
                .find(|(i, v, _)| i == id && v == &ver)
                .map(|(_, _, s)| s)
                .ok_or(InstallError::MissingSignature)?;
            let manifest_bytes = manifest_to_bytes(manifest)
                .map_err(|e| InstallError::CodegenFailed(e.to_string()))?;
            trust.verify_with_author(&manifest_bytes, &manifest.author, sig)?;
            self.stage_blobs_and_manifest(id, manifest, &mut report)?;
            staged.push((id.clone(), manifest.clone()));
        }
        self.codegen.rebuild()?;
        for (id, manifest) in staged {
            self.finalize_current_link(&id, &manifest)?;
        }
        Ok(report)
    }

    fn stage_blobs_and_manifest(
        &mut self,
        id: &PluginId,
        manifest: &PluginManifest,
        report: &mut InstallReport,
    ) -> Result<(), InstallError> {
        for file in &manifest.files {
            let hash = Blake3Hash(file.blob.hash);
            let dest = blob_path(&self.root, &hash);
            if dest.exists() {
                let existing = fs::read(&dest)?;
                if *blake3::hash(&existing).as_bytes() != hash.0 {
                    return Err(InstallError::IntegrityFailure);
                }
                continue;
            }
            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent)?;
            }
            let bytes = self.blob_source.fetch_blob(hash)?;
            if *blake3::hash(&bytes).as_bytes() != hash.0 {
                return Err(InstallError::IntegrityFailure);
            }
            report.fetched_blobs = report.fetched_blobs.saturating_add(1);
            fs::write(&dest, &bytes)?;
        }
        let mpath = manifest_path(&self.root, id.as_str(), &manifest.version);
        if let Some(parent) = mpath.parent() {
            fs::create_dir_all(parent)?;
        }
        let manifest_bytes = crate::manifest::manifest_to_bytes(manifest)
            .map_err(|e| InstallError::CodegenFailed(e.to_string()))?;
        fs::write(&mpath, &manifest_bytes)?;
        Ok(())
    }

    fn finalize_current_link(
        &mut self,
        id: &PluginId,
        manifest: &PluginManifest,
    ) -> Result<(), InstallError> {
        let mpath = manifest_path(&self.root, id.as_str(), &manifest.version);
        let current = installed_current_path(&self.root, id.as_str());
        if let Some(parent) = current.parent() {
            fs::create_dir_all(parent)?;
        }
        let tmp = current.with_extension("tmp");
        if tmp.exists() {
            fs::remove_file(&tmp)?;
        }
        symlink(&mpath, &tmp)?;
        if current.exists() {
            fs::remove_file(&current)?;
        }
        fs::rename(&tmp, &current)?;
        Ok(())
    }

    /// Remove `installed/<id>/current` symlink (and empty parent dirs best-effort).
    pub fn uninstall(&mut self, id: &str) -> Result<(), InstallError> {
        let current = installed_current_path(&self.root, id);
        if current.exists() || current.symlink_metadata().is_ok() {
            fs::remove_file(&current)?;
        }
        Ok(())
    }
}

/// Hex encode a BLAKE3 digest for filesystem paths.
#[must_use]
pub fn blake3_hex(hash: &Blake3Hash) -> String {
    hash.0.iter().map(|b| format!("{b:02x}")).collect()
}

/// CAS blob path: `cas/blobs/<2-hex-prefix>/<full-hex>.blob`.
#[must_use]
pub fn blob_path(root: &Path, hash: &Blake3Hash) -> PathBuf {
    let full = blake3_hex(hash);
    let prefix = &full[..2.min(full.len())];
    root.join("cas/blobs")
        .join(prefix)
        .join(format!("{full}.blob"))
}

/// Archived manifest path on disk.
#[must_use]
pub fn manifest_path(root: &Path, id: &str, version: &str) -> PathBuf {
    root.join("cas/manifests")
        .join(format!("{id}-{version}.rkyv"))
}

/// `installed/<id>/current` symlink target.
#[must_use]
pub fn installed_current_path(root: &Path, id: &str) -> PathBuf {
    root.join("installed").join(id).join("current")
}

fn symlink(target: &Path, link: &Path) -> Result<(), InstallError> {
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(target, link)?;
        Ok(())
    }
    #[cfg(not(unix))]
    {
        let _ = (target, link);
        Err(InstallError::CodegenFailed(
            "symlink install not supported on this platform".into(),
        ))
    }
}

/// Candidate emitted by update checks.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateCandidate {
    /// Plugin id.
    pub plugin: PluginId,
    /// Installed semver.
    pub installed: Version,
    /// Highest available semver from the registry view.
    pub available: Version,
    /// Optional changelog snippet.
    pub changelog: Option<String>,
    /// When true, bulk update must skip this entry.
    pub pinned: bool,
}

/// Compute update rows by comparing installed versions to catalog versions.
#[must_use]
pub fn check_updates(
    installed: &[(PluginId, Version, bool)],
    catalog_latest: &dyn Fn(&str) -> Option<Version>,
) -> Vec<UpdateCandidate> {
    let mut out = Vec::new();
    for (id, ver, pinned) in installed {
        if let Some(av) = catalog_latest(id.as_str())
            && av > *ver
        {
            out.push(UpdateCandidate {
                plugin: id.clone(),
                installed: ver.clone(),
                available: av,
                changelog: None,
                pinned: *pinned,
            });
        }
    }
    out.sort_by(|a, b| a.plugin.cmp(&b.plugin));
    out
}

/// Ids eligible for a bulk update pass (not pinned).
#[must_use]
pub fn bulk_update_targets(candidates: &[UpdateCandidate]) -> Vec<PluginId> {
    let mut ids: Vec<PluginId> = candidates
        .iter()
        .filter(|c| !c.pinned)
        .map(|c| c.plugin.clone())
        .collect();
    ids.sort();
    ids
}
