//! Virtual path resolution and mount table management.

use std::path::{Path, PathBuf};

/// Virtual file system path using forward slashes and a scheme prefix (for example `asset://`).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VPath(pub String);

/// Identifier returned by [`VirtualFileSystem::mount`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MountId(pub u32);

/// A single mount entry tracked by [`VirtualFileSystem`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Mount {
    /// Scheme prefix including trailing components as defined by the mount table.
    pub prefix: &'static str,
    /// Physical storage backend for this prefix.
    pub backend: MountBackend,
}

/// Physical backend bound to a virtual prefix.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MountBackend {
    /// Read-only packed archive rooted at `path`.
    PackedArchive {
        /// Root directory on the host filesystem.
        path: PathBuf,
    },
    /// Loose files rooted at `path`.
    LooseFiles {
        /// Root directory on the host filesystem.
        path: PathBuf,
    },
    /// Build output overlay rooted at `path`.
    BuildOverlay {
        /// Root directory on the host filesystem.
        path: PathBuf,
    },
    /// Network-backed mount described by `endpoint`.
    Network {
        /// Stable textual endpoint identifier.
        endpoint: String,
    },
}

#[derive(Debug)]
struct MountRecord {
    mount: Mount,
    id: MountId,
}

/// Owns mount ordering and deterministic prefix resolution.
#[derive(Debug, Default)]
pub struct VirtualFileSystem {
    mounts: Vec<MountRecord>,
    next_id: u32,
}

impl VirtualFileSystem {
    /// Constructs an empty mount table.
    #[must_use]
    pub fn new() -> Self {
        Self {
            mounts: Vec::new(),
            next_id: 0,
        }
    }

    /// Registers `prefix` with `backend`, returning a stable [`MountId`].
    pub fn mount(&mut self, prefix: &'static str, backend: MountBackend) -> MountId {
        let id = MountId(self.next_id);
        self.next_id += 1;
        self.mounts.push(MountRecord {
            mount: Mount { prefix, backend },
            id,
        });
        id
    }

    /// Removes a mount by id when present.
    pub fn unmount(&mut self, id: MountId) {
        if let Some(pos) = self.mounts.iter().position(|m| m.id == id) {
            self.mounts.remove(pos);
        }
    }

    /// Resolves `vpath` against mounts from newest to oldest (later mounts shadow earlier ones).
    #[must_use]
    pub fn resolve(&self, vpath: &VPath) -> Option<PathBuf> {
        for record in self.mounts.iter().rev() {
            let prefix = record.mount.prefix;
            if vpath.0.starts_with(prefix) {
                let tail = &vpath.0[prefix.len()..];
                let tail = tail.trim_start_matches('/');
                let base: PathBuf = match &record.mount.backend {
                    MountBackend::PackedArchive { path }
                    | MountBackend::LooseFiles { path }
                    | MountBackend::BuildOverlay { path } => path.clone(),
                    MountBackend::Network { .. } => {
                        // Network mounts do not map to a local [`PathBuf`] in this layer.
                        return None;
                    }
                };
                return Some(join_under(&base, tail));
            }
        }
        None
    }

    /// Returns how many mounts are currently registered.
    #[must_use]
    pub fn mount_count(&self) -> usize {
        self.mounts.len()
    }
}

fn join_under(base: &Path, tail: &str) -> PathBuf {
    if tail.is_empty() {
        base.to_path_buf()
    } else {
        base.join(Path::new(tail))
    }
}
