//! Virtual filesystem seam for transactional slot I/O (design: `VirtualFileSystem`).

use std::path::Path;
use std::path::PathBuf;

use crate::error::IoError;

/// Minimal synchronous filesystem surface used by [`crate::slots::SaveSlotManager`] and tests.
pub trait VirtualFileSystem {
    /// Write `data` to `path` atomically using a sibling temp file + rename.
    fn write_atomic(&self, final_path: &Path, data: &[u8]) -> Result<(), IoError>;

    /// Read the entire file.
    fn read_file(&self, path: &Path) -> Result<Vec<u8>, IoError>;

    /// Remove a file if it exists.
    fn remove_file(&self, path: &Path) -> Result<(), IoError>;

    /// Copy `src` to `dst` atomically (write-temp + rename into `dst`).
    fn copy_atomic(&self, src: &Path, dst: &Path) -> Result<(), IoError>;

    /// List file names in a directory (non-recursive).
    fn read_dir_names(&self, dir: &Path) -> Result<Vec<String>, IoError>;
}

/// `std::fs`-backed implementation for integration tests and local saves.
#[derive(Debug, Clone)]
pub struct StdVirtualFileSystem {
    root: PathBuf,
}

impl StdVirtualFileSystem {
    /// Root directory for all relative paths passed to slot helpers.
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    fn resolve(&self, path: &Path) -> PathBuf {
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.root.join(path)
        }
    }
}

impl VirtualFileSystem for StdVirtualFileSystem {
    fn write_atomic(&self, final_path: &Path, data: &[u8]) -> Result<(), IoError> {
        let final_path = self.resolve(final_path);
        if let Some(parent) = final_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| IoError {
                message: e.to_string(),
            })?;
        }
        let tmp = final_path.with_extension("tmp");
        std::fs::write(&tmp, data).map_err(|e| IoError {
            message: e.to_string(),
        })?;
        std::fs::rename(&tmp, &final_path).map_err(|e| IoError {
            message: e.to_string(),
        })
    }

    fn read_file(&self, path: &Path) -> Result<Vec<u8>, IoError> {
        let path = self.resolve(path);
        std::fs::read(&path).map_err(|e| IoError {
            message: e.to_string(),
        })
    }

    fn remove_file(&self, path: &Path) -> Result<(), IoError> {
        let path = self.resolve(path);
        match std::fs::remove_file(&path) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(IoError {
                message: e.to_string(),
            }),
        }
    }

    fn copy_atomic(&self, src: &Path, dst: &Path) -> Result<(), IoError> {
        let bytes = self.read_file(src)?;
        self.write_atomic(dst, &bytes)
    }

    fn read_dir_names(&self, dir: &Path) -> Result<Vec<String>, IoError> {
        let dir = self.resolve(dir);
        let rd = std::fs::read_dir(&dir).map_err(|e| IoError {
            message: e.to_string(),
        })?;
        let mut out = Vec::new();
        for ent in rd {
            let ent = ent.map_err(|e| IoError {
                message: e.to_string(),
            })?;
            if let Some(s) = ent.file_name().to_str() {
                out.push(s.to_string());
            }
        }
        out.sort();
        Ok(out)
    }
}

/// Test double that records read counts (TC-13.3.4.3).
#[derive(Debug)]
pub struct CountingVirtualFileSystem {
    inner: StdVirtualFileSystem,
    pub read_count: std::cell::Cell<u32>,
}

impl CountingVirtualFileSystem {
    /// Wrap a [`StdVirtualFileSystem`].
    pub fn new(inner: StdVirtualFileSystem) -> Self {
        Self {
            inner,
            read_count: std::cell::Cell::new(0),
        }
    }
}

impl VirtualFileSystem for CountingVirtualFileSystem {
    fn write_atomic(&self, final_path: &Path, data: &[u8]) -> Result<(), IoError> {
        self.inner.write_atomic(final_path, data)
    }

    fn read_file(&self, path: &Path) -> Result<Vec<u8>, IoError> {
        self.read_count.set(self.read_count.get().saturating_add(1));
        self.inner.read_file(path)
    }

    fn remove_file(&self, path: &Path) -> Result<(), IoError> {
        self.inner.remove_file(path)
    }

    fn copy_atomic(&self, src: &Path, dst: &Path) -> Result<(), IoError> {
        self.inner.copy_atomic(src, dst)
    }

    fn read_dir_names(&self, dir: &Path) -> Result<Vec<String>, IoError> {
        self.inner.read_dir_names(dir)
    }
}
