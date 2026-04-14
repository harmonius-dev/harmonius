//! Parsing of `git status --porcelain` output into structured [`FileStatus`] values.
//!
//! This module implements **TC-15.10.1.1** (R-15.10.1): deterministic parsing of Git's short
//! porcelain format without invoking libgit2.

use std::path::{Path, PathBuf};

/// High-level classification for index and worktree slots, aligned with Git's first/second column.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StatusKind {
    /// No change (` ` in porcelain).
    Unmodified,
    /// Ordinary modification (`M`).
    Modified,
    /// New file in index (`A`).
    Added,
    /// Removal (`D`).
    Deleted,
    /// Rename (`R`).
    Renamed,
    /// Copy (`C`).
    Copied,
    /// Unmerged / conflict (`U`).
    Unmerged,
    /// Untracked (`?`).
    Untracked,
    /// Ignored (`!`).
    Ignored,
    /// Type change (`T`).
    TypeChange,
    /// Unknown non-space status character from Git.
    Other(char),
}

/// Per-path status row produced from porcelain parsing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileStatus {
    /// Repository-relative path (normalized separators on Unix hosts).
    pub path: PathBuf,
    /// Index (first column) status.
    pub index_status: StatusKind,
    /// Worktree (second column) status.
    pub worktree_status: StatusKind,
    /// Whether the path is treated as Git LFS managed for UI purposes.
    pub is_lfs: bool,
}

/// Recoverable parse failures for porcelain lines and version-control operations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VcError {
    /// libgit2 reported an error during repository access or mutation.
    Git {
        /// Human-readable message from libgit2.
        message: String,
    },
    /// Input line was empty or too short to be valid short porcelain.
    InvalidLine {
        /// Original line (lossy for diagnostics only).
        line: String,
    },
    /// Path segment could not be decoded.
    InvalidPath,
    /// Another principal holds the Git LFS lock for this path.
    LfsLockHeld {
        /// Repository-relative locked path.
        path: PathBuf,
        /// Display name of the lock holder.
        holder: String,
    },
    /// Unlock requested for a path with no lock or a different holder.
    LfsLockMismatch {
        /// Path that could not be unlocked.
        path: PathBuf,
    },
}

impl From<git2::Error> for VcError {
    fn from(value: git2::Error) -> Self {
        VcError::Git {
            message: value.message().to_string(),
        }
    }
}

fn status_from_column(ch: char) -> StatusKind {
    match ch {
        ' ' => StatusKind::Unmodified,
        'M' => StatusKind::Modified,
        'A' => StatusKind::Added,
        'D' => StatusKind::Deleted,
        'R' => StatusKind::Renamed,
        'C' => StatusKind::Copied,
        'U' => StatusKind::Unmerged,
        'T' => StatusKind::TypeChange,
        '?' => StatusKind::Untracked,
        '!' => StatusKind::Ignored,
        other => StatusKind::Other(other),
    }
}

fn extract_path_after_status(line: &str) -> Result<&str, VcError> {
    let mut chars = line.chars();
    let _c0 = chars.next().ok_or(VcError::InvalidLine {
        line: line.to_string(),
    })?;
    let _c1 = chars.next().ok_or(VcError::InvalidLine {
        line: line.to_string(),
    })?;
    let rest = chars.as_str();
    let trimmed = rest.trim_start();
    if trimmed.is_empty() {
        return Err(VcError::InvalidLine {
            line: line.to_string(),
        });
    }
    Ok(trimmed)
}

/// Parse a single line of `git status --porcelain` (short format, without `-z`).
///
/// Returns `Ok(None)` for purely informational lines that do not describe a path (currently none
/// are emitted, but the variant keeps the API stable for future extensions).
pub fn parse_short_porcelain_line(line: &str, is_lfs_path: impl Fn(&Path) -> bool) -> Result<Option<FileStatus>, VcError> {
    let line = line.strip_suffix('\r').unwrap_or(line);
    if line.is_empty() {
        return Err(VcError::InvalidLine {
            line: line.to_string(),
        });
    }

    let mut chars = line.chars();
    let c0 = chars.next().ok_or(VcError::InvalidLine {
        line: line.to_string(),
    })?;
    let c1 = chars.next().ok_or(VcError::InvalidLine {
        line: line.to_string(),
    })?;

    if c0 == '?' && c1 == '?' {
        let path_tail = chars.as_str().trim_start();
        if path_tail.is_empty() {
            return Err(VcError::InvalidLine {
                line: line.to_string(),
            });
        }
        let path = PathBuf::from(path_tail);
        if path.as_os_str().is_empty() {
            return Err(VcError::InvalidPath);
        }
        return Ok(Some(FileStatus {
            is_lfs: is_lfs_path(&path),
            index_status: StatusKind::Untracked,
            path,
            worktree_status: StatusKind::Untracked,
        }));
    }

    if c0 == '!' && c1 == '!' {
        let path_tail = chars.as_str().trim_start();
        if path_tail.is_empty() {
            return Err(VcError::InvalidLine {
                line: line.to_string(),
            });
        }
        let path = PathBuf::from(path_tail);
        if path.as_os_str().is_empty() {
            return Err(VcError::InvalidPath);
        }
        return Ok(Some(FileStatus {
            is_lfs: is_lfs_path(&path),
            index_status: StatusKind::Ignored,
            path,
            worktree_status: StatusKind::Ignored,
        }));
    }

    let tail = extract_path_after_status(line)?;
    let (path_str, index_status, worktree_status) = if let Some((_, new_path)) = tail.rsplit_once(" -> ") {
        let new_path = new_path.trim();
        if new_path.is_empty() {
            return Err(VcError::InvalidLine {
                line: line.to_string(),
            });
        }
        (new_path, status_from_column(c0), status_from_column(c1))
    } else {
        (tail.trim_end(), status_from_column(c0), status_from_column(c1))
    };

    if path_str.is_empty() {
        return Err(VcError::InvalidLine {
            line: line.to_string(),
        });
    }

    let path = PathBuf::from(path_str);
    if path.as_os_str().is_empty() {
        return Err(VcError::InvalidPath);
    }

    Ok(Some(FileStatus {
        index_status,
        is_lfs: is_lfs_path(&path),
        path,
        worktree_status,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **TC-15.10.1.1** — Git status parsing returns correct file states.
    #[test]
    fn tc_15_10_1_1_parses_short_porcelain_rows() {
        let rows = vec![
            (" M src/lib.rs", "src/lib.rs", StatusKind::Unmodified, StatusKind::Modified, false),
            ("M  src/lib.rs", "src/lib.rs", StatusKind::Modified, StatusKind::Unmodified, false),
            ("A  new.txt", "new.txt", StatusKind::Added, StatusKind::Unmodified, false),
            (" D gone.txt", "gone.txt", StatusKind::Unmodified, StatusKind::Deleted, false),
            ("?? untracked.log", "untracked.log", StatusKind::Untracked, StatusKind::Untracked, false),
            (
                "R  old.rs -> new.rs",
                "new.rs",
                StatusKind::Renamed,
                StatusKind::Unmodified,
                false,
            ),
        ];

        for (line, expect_path, idx, wt, lfs) in rows {
            let got = parse_short_porcelain_line(line, |_| false)
                .expect("parse")
                .expect("row");
            assert_eq!(got.path, Path::new(expect_path));
            assert_eq!(got.index_status, idx);
            assert_eq!(got.worktree_status, wt);
            assert_eq!(got.is_lfs, lfs);
        }
    }

    #[test]
    fn marks_lfs_paths_when_predicate_matches() {
        let got = parse_short_porcelain_line(" M assets/huge.bin", |p| p.extension().is_some_and(|e| e == "bin"))
            .expect("parse")
            .expect("row");
        assert_eq!(got.path, Path::new("assets/huge.bin"));
        assert!(got.is_lfs);
    }
}
