//! Directory creation, deletion, and metadata (`R-14.6.2`, `R-14.6.3`).

use std::fs;
use std::time::UNIX_EPOCH;

use super::error::FsError;
use super::path::CanonicalPath;

/// File kind reported by [`stat`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileType {
    /// Regular file.
    File,
    /// Directory.
    Directory,
    /// Symlink (host-dependent).
    Symlink,
}

/// Metadata bundle returned by [`stat`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileMetadata {
    /// Kind of filesystem object.
    pub file_type: FileType,
    /// Size in bytes for regular files.
    pub size: u64,
    /// Last modified time in seconds since UNIX epoch.
    pub modified: u64,
    /// Creation time when the host exposes it.
    pub created: Option<u64>,
    /// Read-only attribute bit.
    pub read_only: bool,
}

/// Creates a directory tree, including parents.
pub fn create_dir_all(path: &CanonicalPath) -> Result<(), FsError> {
    fs::create_dir_all(path.to_path_buf()).map_err(|e| FsError::Platform {
        code: e.raw_os_error().unwrap_or(-1),
        message: e.to_string(),
    })
}

/// Deletes a single file.
pub fn delete_file(path: &CanonicalPath) -> Result<(), FsError> {
    fs::remove_file(path.to_path_buf()).map_err(|e| map_rm_error(e, path.as_str()))
}

/// Deletes many files, returning one result per input path.
pub fn delete_batch(paths: &[CanonicalPath]) -> Vec<Result<(), FsError>> {
    paths.iter().map(delete_file).collect()
}

/// Returns metadata for `path`.
pub fn stat(path: &CanonicalPath) -> Result<FileMetadata, FsError> {
    let meta = fs::metadata(path.to_path_buf()).map_err(|e| map_stat_error(e, path.as_str()))?;
    Ok(map_meta(meta))
}

/// Returns metadata for many paths.
pub fn stat_batch(paths: &[CanonicalPath]) -> Vec<Result<FileMetadata, FsError>> {
    paths.iter().map(stat).collect()
}

fn map_meta(meta: fs::Metadata) -> FileMetadata {
    let file_type = if meta.is_dir() {
        FileType::Directory
    } else if meta.is_symlink() {
        FileType::Symlink
    } else {
        FileType::File
    };
    let modified = meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let created = meta
        .created()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs());
    #[cfg(unix)]
    let read_only = meta.permissions().readonly();
    #[cfg(not(unix))]
    let read_only = meta.permissions().readonly();
    FileMetadata {
        file_type,
        size: meta.len(),
        modified,
        created,
        read_only,
    }
}

fn map_stat_error(err: std::io::Error, path: &str) -> FsError {
    use std::io::ErrorKind;
    match err.kind() {
        ErrorKind::NotFound => FsError::NotFound {
            path: path.into(),
        },
        _ => FsError::Platform {
            code: err.raw_os_error().unwrap_or(-1),
            message: err.to_string(),
        },
    }
}

fn map_rm_error(err: std::io::Error, path: &str) -> FsError {
    use std::io::ErrorKind;
    match err.kind() {
        ErrorKind::NotFound => FsError::NotFound {
            path: path.into(),
        },
        _ => FsError::Platform {
            code: err.raw_os_error().unwrap_or(-1),
            message: err.to_string(),
        },
    }
}
