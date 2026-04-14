//! Deterministic in-memory filesystem used by the headless harness.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Synthetic I/O failure modes for negative tests (FM-1).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IoError {
    NotFound,
    PermissionDenied,
}

/// In-memory files plus optional per-path read faults.
#[derive(Clone, Debug, Default)]
pub struct FakeFileSystem {
    faults: HashMap<PathBuf, IoError>,
    files: HashMap<PathBuf, Vec<u8>>,
}

impl FakeFileSystem {
    /// Registers readable bytes for `path`.
    pub fn insert<P: Into<PathBuf>>(&mut self, path: P, bytes: Vec<u8>) {
        self.files.insert(path.into(), bytes);
    }

    /// Forces `read` to return `err` for `path`.
    pub fn fault_on_read<P: Into<PathBuf>>(&mut self, path: P, err: IoError) {
        self.faults.insert(path.into(), err);
    }

    /// Reads a file exactly like a blocking platform read for the harness.
    pub fn read(&self, path: &Path) -> Result<Vec<u8>, IoError> {
        if let Some(err) = self.faults.get(path) {
            return Err(*err);
        }
        self.files.get(path).cloned().ok_or(IoError::NotFound)
    }
}
