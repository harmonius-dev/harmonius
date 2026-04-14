//! Deterministic importer that emits stable "rkyv-like" blobs without serde/rkyv.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::contracts::{ImportKind, ImportOptions};

/// Controlled failure surface for parse/process stages (FM-2).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FakeImportError {
    Parse,
}

/// Tracks which source paths should fail parsing.
#[derive(Clone, Debug, Default)]
pub struct FakeImporter {
    parse_failures: HashSet<PathBuf>,
}

impl FakeImporter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Marks `path` to fail during parse.
    pub fn fail_parse_for<P: AsRef<Path>>(&mut self, path: P) {
        self.parse_failures.insert(path.as_ref().to_path_buf());
    }

    /// Produces a deterministic blob from bytes + options.
    pub fn import(
        &self,
        source_path: &Path,
        kind: ImportKind,
        options: &ImportOptions,
        bytes: &[u8],
    ) -> Result<Vec<u8>, FakeImportError> {
        if self.parse_failures.contains(source_path) {
            return Err(FakeImportError::Parse);
        }
        let mut out = Vec::new();
        out.extend_from_slice(b"HAR_FAKE_V1");
        out.push(kind as u8);
        out.push(options.generate_mips as u8);
        out.push(options.force_reimport as u8);
        out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        out.extend_from_slice(bytes);
        Ok(out)
    }
}
