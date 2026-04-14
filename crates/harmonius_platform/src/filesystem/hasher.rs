//! Content hashing (`R-14.6.6`).

use std::collections::HashMap;
use std::fs;

use super::error::FsError;
use super::path::CanonicalPath;

/// BLAKE3 digest for cached file bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Blake3Hash(pub [u8; 32]);

/// Computes and caches BLAKE3 hashes for file paths.
#[derive(Debug, Default)]
pub struct ContentHasher {
    cache: HashMap<String, Blake3Hash>,
}

impl ContentHasher {
    /// Creates an empty hasher.
    pub fn new() -> Self {
        Self::default()
    }

    /// Hashes the entire file at `path`.
    pub fn hash_file(&self, path: &CanonicalPath) -> Result<Blake3Hash, FsError> {
        let bytes = fs::read(path.to_path_buf()).map_err(|e| FsError::Platform {
            code: e.raw_os_error().unwrap_or(-1),
            message: e.to_string(),
        })?;
        Ok(Blake3Hash(*blake3::hash(&bytes).as_bytes()))
    }

    /// Returns whether bytes on disk differ from `old_hash`.
    pub fn has_content_changed(
        &self,
        path: &CanonicalPath,
        old_hash: &Blake3Hash,
    ) -> Result<bool, FsError> {
        let now = self.hash_file(path)?;
        Ok(now != *old_hash)
    }

    /// Stores a known-good digest for a path.
    pub fn cache_hash(&mut self, path: CanonicalPath, hash: Blake3Hash) {
        self.cache.insert(path.as_str().into(), hash);
    }
}
