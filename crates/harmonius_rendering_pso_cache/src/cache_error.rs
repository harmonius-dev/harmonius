use std::fmt;
use std::io;

/// Errors opening or reading the on-disk PSO cache.
#[derive(Debug)]
pub enum CacheError {
    /// Underlying I/O failure.
    Io(io::Error),
    /// No matching on-disk record for the requested key.
    Missing,
    /// Checksum or structural corruption.
    Corrupted(&'static str),
    /// Active device does not match cached blob metadata.
    DeviceMismatch,
    /// Cache format is newer or incompatible.
    VersionMismatch,
    /// CPU-side pipeline compile failure (surface detail omitted in this crate).
    Compile(&'static str),
}

impl fmt::Display for CacheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CacheError::Io(err) => write!(f, "io error: {err}"),
            CacheError::Missing => write!(f, "missing cache entry"),
            CacheError::Corrupted(reason) => write!(f, "corrupted: {reason}"),
            CacheError::DeviceMismatch => write!(f, "device mismatch"),
            CacheError::VersionMismatch => write!(f, "version mismatch"),
            CacheError::Compile(msg) => write!(f, "compile: {msg}"),
        }
    }
}

impl std::error::Error for CacheError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CacheError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for CacheError {
    fn from(value: io::Error) -> Self {
        CacheError::Io(value)
    }
}

/// Errors returned by [`crate::PsoCache::get_or_build`](super::PsoCache::get_or_build).
#[derive(Debug)]
pub enum PsoError {
    /// Disk or validation failure.
    Cache(CacheError),
    /// Compiler failure.
    Compiler(&'static str),
}

impl fmt::Display for PsoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PsoError::Cache(err) => write!(f, "{err}"),
            PsoError::Compiler(msg) => write!(f, "compiler: {msg}"),
        }
    }
}

impl std::error::Error for PsoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PsoError::Cache(err) => Some(err),
            _ => None,
        }
    }
}

impl From<CacheError> for PsoError {
    fn from(value: CacheError) -> Self {
        PsoError::Cache(value)
    }
}
