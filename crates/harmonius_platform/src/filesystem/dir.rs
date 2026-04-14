//! Directory enumeration (`R-14.6.4`).

use std::fs;

use super::error::FsError;
use super::ops::FileType;
use super::path::CanonicalPath;

/// One entry returned from directory enumeration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirEntry {
    /// File name only.
    pub name: String,
    /// Full canonical path.
    pub path: CanonicalPath,
    /// Detected file type.
    pub file_type: FileType,
    /// Size in bytes when available.
    pub size: u64,
}

/// Options controlling recursive depth and optional glob filter.
#[derive(Clone, Debug)]
pub struct EnumerateOptions {
    /// Maximum directory depth from the root argument; `0` lists immediate
    /// children only (no subdirectory descent).
    pub max_depth: u32,
    /// Optional glob such as `*.txt` (suffix match after `*`).
    pub glob: Option<String>,
}

impl Default for EnumerateOptions {
    fn default() -> Self {
        Self {
            max_depth: u32::MAX,
            glob: None,
        }
    }
}

/// Enumerates a directory tree with optional filtering.
pub fn enumerate_dir(
    path: &CanonicalPath,
    options: &EnumerateOptions,
) -> Result<Vec<DirEntry>, FsError> {
    let mut out = Vec::new();
    let mut stack = vec![(path.clone(), 0u32)];
    while let Some((cur, depth)) = stack.pop() {
        let rd = fs::read_dir(cur.to_path_buf()).map_err(|e| FsError::Platform {
            code: e.raw_os_error().unwrap_or(-1),
            message: e.to_string(),
        })?;
        for ent in rd {
            let ent = ent.map_err(|e| FsError::Platform {
                code: e.raw_os_error().unwrap_or(-1),
                message: e.to_string(),
            })?;
            let name = ent.file_name().to_string_lossy().into_owned();
            let child_path = cur.join(&name)?;
            let meta = ent.metadata().map_err(|e| FsError::Platform {
                code: e.raw_os_error().unwrap_or(-1),
                message: e.to_string(),
            })?;
            let file_type = if meta.is_dir() {
                FileType::Directory
            } else if meta.is_symlink() {
                FileType::Symlink
            } else {
                FileType::File
            };
            let passes_glob = options
                .glob
                .as_ref()
                .map(|g| glob_matches(&name, g))
                .unwrap_or(true);
            if file_type == FileType::File && passes_glob {
                out.push(DirEntry {
                    name: name.clone(),
                    path: child_path.clone(),
                    file_type,
                    size: meta.len(),
                });
            }
            if file_type == FileType::Directory && depth < options.max_depth {
                stack.push((child_path, depth + 1));
            }
        }
    }
    Ok(out)
}

fn glob_matches(name: &str, pattern: &str) -> bool {
    if let Some(suffix) = pattern.strip_prefix('*') {
        name.ends_with(suffix)
    } else {
        name == pattern
    }
}
