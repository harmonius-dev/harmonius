//! Synchronous host-backed file handle (`R-14.6.1`).

use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

use super::error::FsError;
use super::path::CanonicalPath;

/// Open mode flags for [`AsyncFile`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OpenFlags {
    /// Open for reads.
    pub read: bool,
    /// Open for writes.
    pub write: bool,
    /// Create if missing.
    pub create: bool,
    /// Fail create when the file already exists.
    pub exclusive_create: bool,
    /// Truncate to zero length after open.
    pub truncate: bool,
    /// Append-only writes.
    pub append: bool,
}

impl OpenFlags {
    /// Read-only open.
    pub fn read_only() -> Self {
        Self {
            read: true,
            write: false,
            create: false,
            exclusive_create: false,
            truncate: false,
            append: false,
        }
    }

    /// Write-only open.
    pub fn write_only() -> Self {
        Self {
            read: false,
            write: true,
            create: false,
            exclusive_create: false,
            truncate: false,
            append: false,
        }
    }

    /// Read/write open.
    pub fn read_write() -> Self {
        Self {
            read: true,
            write: true,
            create: false,
            exclusive_create: false,
            truncate: false,
            append: false,
        }
    }

    /// Create a new file exclusively (fails if it already exists).
    pub fn create_new() -> Self {
        Self {
            read: true,
            write: true,
            create: true,
            exclusive_create: true,
            truncate: false,
            append: false,
        }
    }
}

/// Host-backed file handle; synchronous completion matches unit test cases.
#[derive(Debug)]
pub struct AsyncFile {
    file: File,
}

impl AsyncFile {
    /// Opens or creates a file according to `flags`.
    pub fn open(path: &CanonicalPath, flags: OpenFlags) -> Result<Self, FsError> {
        let p = path.to_path_buf();
        let mut opts = OpenOptions::new();
        opts.read(flags.read).write(flags.write);
        if flags.create {
            opts.create(true);
        }
        if flags.exclusive_create {
            opts.create_new(true);
        }
        if flags.truncate {
            opts.truncate(true);
        }
        if flags.append {
            opts.append(true);
        }
        let file = opts.open(&p).map_err(|e| map_io_error(e, path.as_str()))?;
        Ok(Self { file })
    }

    /// Reads up to `buf.len()` bytes at `offset`.
    pub fn read(&self, buf: &mut [u8], offset: u64) -> Result<usize, FsError> {
        let mut f = &self.file;
        f.seek(SeekFrom::Start(offset))
            .map_err(|e| FsError::Platform {
                code: e.raw_os_error().unwrap_or(-1),
                message: e.to_string(),
            })?;
        f.read(buf).map_err(|e| FsError::Platform {
            code: e.raw_os_error().unwrap_or(-1),
            message: e.to_string(),
        })
    }

    /// Reads the entire file from the start.
    pub fn read_to_end(&self) -> Result<Vec<u8>, FsError> {
        let mut f = &self.file;
        f.seek(SeekFrom::Start(0))
            .map_err(|e| FsError::Platform {
                code: e.raw_os_error().unwrap_or(-1),
                message: e.to_string(),
            })?;
        let mut v = Vec::new();
        f.read_to_end(&mut v).map_err(|e| FsError::Platform {
            code: e.raw_os_error().unwrap_or(-1),
            message: e.to_string(),
        })?;
        Ok(v)
    }

    /// Writes `data` at `offset`, returning bytes written.
    pub fn write(&self, data: &[u8], offset: u64) -> Result<usize, FsError> {
        let mut f = &self.file;
        f.seek(SeekFrom::Start(offset))
            .map_err(|e| FsError::Platform {
                code: e.raw_os_error().unwrap_or(-1),
                message: e.to_string(),
            })?;
        f.write(data).map_err(|e| FsError::Platform {
            code: e.raw_os_error().unwrap_or(-1),
            message: e.to_string(),
        })
    }

    /// Flushes OS buffers for the open file.
    pub fn flush(&self) -> Result<(), FsError> {
        self.file.sync_all().map_err(|e| FsError::Platform {
            code: e.raw_os_error().unwrap_or(-1),
            message: e.to_string(),
        })
    }

    /// Closes the handle after flushing.
    pub fn close(self) -> Result<(), FsError> {
        self.flush()
    }
}

fn map_io_error(err: std::io::Error, path: &str) -> FsError {
    use std::io::ErrorKind;
    match err.kind() {
        ErrorKind::NotFound => FsError::NotFound {
            path: path.into(),
        },
        ErrorKind::AlreadyExists => FsError::AlreadyExists {
            path: path.into(),
        },
        ErrorKind::PermissionDenied => FsError::PermissionDenied {
            path: path.into(),
        },
        _ => FsError::Platform {
            code: err.raw_os_error().unwrap_or(-1),
            message: err.to_string(),
        },
    }
}
