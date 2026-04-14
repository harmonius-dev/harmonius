//! Canonical path helpers (`F-14.6.7` / `R-14.6.7`).

use std::fmt;
use std::path::Path;
use std::path::PathBuf;

use super::error::FsError;

/// Normalized logical path used by platform file APIs.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CanonicalPath {
    inner: String,
}

impl CanonicalPath {
    /// Lexically resolves `.` / `..` segments using POSIX-style rules.
    pub fn resolve(path: &str) -> Result<Self, FsError> {
        if path.is_empty() {
            return Err(FsError::Platform {
                code: 22,
                message: "empty path".into(),
            });
        }
        let mut stack: Vec<String> = Vec::new();
        for part in path.split('/') {
            if part.is_empty() || part == "." {
                continue;
            }
            if part == ".." {
                stack.pop();
            } else {
                stack.push(part.into());
            }
        }
        if stack.is_empty() {
            return Ok(Self { inner: ".".into() });
        }
        Ok(Self {
            inner: stack.join("/"),
        })
    }

    /// Borrows the normalized path as UTF-8.
    pub fn as_str(&self) -> &str {
        &self.inner
    }

    /// Returns the final path component when present.
    pub fn file_name(&self) -> Option<&str> {
        self.inner.rsplit_once('/').map(|(_, n)| n).filter(|s| !s.is_empty())
    }

    /// Parent directory, if any.
    pub fn parent(&self) -> Option<Self> {
        if self.inner == "." {
            return None;
        }
        match self.inner.rsplit_once('/') {
            Some(("", _)) => None,
            Some((p, _)) => Some(Self { inner: p.into() }),
            None => Some(Self { inner: ".".into() }),
        }
    }

    /// Appends a single path component.
    pub fn join(&self, component: &str) -> Result<Self, FsError> {
        if component.is_empty() || component.contains('/') {
            return Err(FsError::Platform {
                code: 22,
                message: "invalid path component".into(),
            });
        }
        if component == "." || component == ".." {
            return Err(FsError::Platform {
                code: 22,
                message: "dot components must be resolved via resolve()".into(),
            });
        }
        let combined = if self.inner == "." {
            component.into()
        } else {
            format!("{}/{}", self.inner, component)
        };
        Self::resolve(&combined)
    }

    /// Returns the file extension without the leading dot.
    pub fn extension(&self) -> Option<&str> {
        self.file_name().and_then(|n| n.rsplit_once('.').map(|(_, e)| e))
    }

    /// Builds a [`CanonicalPath`] from an existing on-disk path.
    pub fn from_std_path(path: &Path) -> Result<Self, FsError> {
        let canon = path.canonicalize().map_err(|e| FsError::Platform {
            code: e.raw_os_error().unwrap_or(-1),
            message: e.to_string(),
        })?;
        let s = canon.to_string_lossy().replace('\\', "/");
        Ok(Self { inner: s })
    }

    /// Builds a path string without touching the filesystem (tests + staging paths).
    pub fn from_path_unverified(path: &Path) -> Self {
        Self {
            inner: path.to_string_lossy().replace('\\', "/"),
        }
    }

    /// Converts to a host [`PathBuf`] for `std::fs` operations.
    pub fn to_path_buf(&self) -> PathBuf {
        PathBuf::from(&self.inner)
    }
}

impl fmt::Display for CanonicalPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}
