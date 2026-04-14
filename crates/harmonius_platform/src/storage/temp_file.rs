//! Temporary scratch files with explicit orphan cleanup (`R-14.5.12`).

use std::fs;
use std::path::PathBuf;

use crate::filesystem::{CanonicalPath, FsError};

/// Temp scratch errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TempError {
    /// Filesystem failure.
    IoError(FsError),
    /// Byte budget exceeded.
    BudgetExceeded {
        /// Bytes currently used.
        used: u64,
        /// Maximum allowed bytes.
        max: u64,
    },
}

/// Lease handle; dropping does **not** delete the file (cleanup does).
#[derive(Debug)]
pub struct TempFileHandle {
    path: PathBuf,
}

impl TempFileHandle {
    /// Returns the backing path.
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

/// Manages temporary files under a root directory.
#[derive(Debug)]
pub struct TempFileManager {
    root: CanonicalPath,
    max_bytes: u64,
    used: u64,
}

impl TempFileManager {
    /// Initializes a manager rooted at `root`.
    pub fn init(root: CanonicalPath, max_bytes: u64) -> Result<Self, TempError> {
        fs::create_dir_all(root.to_path_buf()).map_err(|e| TempError::IoError(FsError::Platform {
            code: e.raw_os_error().unwrap_or(-1),
            message: e.to_string(),
        }))?;
        Ok(Self {
            root,
            max_bytes,
            used: 0,
        })
    }

    /// Allocates a new scratch file (1 byte placeholder when size omitted in tests).
    pub fn allocate(&mut self, name: &str) -> Result<TempFileHandle, TempError> {
        self.allocate_sized(name, 1)
    }

    /// Allocates `size` bytes at `name` under the manager root.
    pub fn allocate_sized(&mut self, name: &str, size: u64) -> Result<TempFileHandle, TempError> {
        let new_used = self.used.saturating_add(size);
        if new_used > self.max_bytes {
            return Err(TempError::BudgetExceeded {
                used: new_used,
                max: self.max_bytes,
            });
        }
        let pb = self.root.to_path_buf().join(name);
        fs::write(&pb, vec![0u8; size as usize]).map_err(|e| TempError::IoError(FsError::Platform {
            code: e.raw_os_error().unwrap_or(-1),
            message: e.to_string(),
        }))?;
        self.used = new_used;
        Ok(TempFileHandle { path: pb })
    }

    /// Deletes every regular file under the manager root.
    pub fn cleanup_orphans(&mut self) -> Result<u32, TempError> {
        let mut removed = 0u32;
        let rd = fs::read_dir(self.root.to_path_buf()).map_err(|e| {
            TempError::IoError(FsError::Platform {
                code: e.raw_os_error().unwrap_or(-1),
                message: e.to_string(),
            })
        })?;
        for ent in rd {
            let ent = ent.map_err(|e| TempError::IoError(FsError::Platform {
                code: e.raw_os_error().unwrap_or(-1),
                message: e.to_string(),
            }))?;
            let p = ent.path();
            if p.is_file() {
                fs::remove_file(&p).map_err(|e| TempError::IoError(FsError::Platform {
                    code: e.raw_os_error().unwrap_or(-1),
                    message: e.to_string(),
                }))?;
                removed += 1;
            }
        }
        self.used = 0;
        Ok(removed)
    }
}
