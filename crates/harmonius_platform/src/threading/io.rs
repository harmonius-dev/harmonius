//! Channel-based platform I/O façade (main thread submits, workers observe completions).

use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

use crossbeam_channel::Sender;

/// Monotonic identifier for an I/O request.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct IoRequestId(pub(crate) u64);

/// Request sent from worker threads to the main thread.
#[derive(Debug)]
pub enum IoRequest {
    /// File read request.
    Read {
        /// Client-chosen id echoed in [`IoResult`].
        id: IoRequestId,
        /// Path to read.
        path: std::path::PathBuf,
        /// Destination buffer slot.
        buffer: BufferSlot,
    },
    /// File write request.
    Write {
        /// Client-chosen id echoed in [`IoResult`].
        id: IoRequestId,
        /// Path to write.
        path: std::path::PathBuf,
        /// Source buffer slot.
        buffer: BufferSlot,
    },
}

/// Completion record posted back to workers.
#[derive(Debug)]
pub struct IoResult {
    /// Echo of [`IoRequest`] id.
    pub id: IoRequestId,
    /// Bytes moved on success.
    pub bytes_transferred: u32,
    /// Buffer handed back to the caller.
    pub buffer: BufferSlot,
    /// Transport status.
    pub status: Result<(), IoError>,
}

/// Typed I/O failure modes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IoError {
    /// Path missing.
    NotFound,
    /// Permission denied.
    PermissionDenied,
    /// Cancelled before completion.
    Cancelled,
    /// Device full.
    DeviceFull,
    /// OS-specific error code.
    Platform {
        /// Raw errno-style value.
        code: i32,
    },
}

/// Fixed-size buffer pool for staging reads and writes.
#[derive(Clone, Debug)]
pub struct BufferPool {
    buffer_size: usize,
}

/// Owning view into a pooled buffer.
#[derive(Debug)]
pub struct BufferSlot {
    pub(crate) data: Vec<u8>,
}

impl BufferPool {
    /// Allocates metadata for buffers of `buffer_size` bytes (slots are created on demand).
    pub fn new(buffer_size: usize, _count: u32) -> Self {
        Self { buffer_size }
    }

    /// Acquires a zeroed buffer.
    pub fn acquire(&self) -> Option<BufferSlot> {
        Some(BufferSlot {
            data: vec![0u8; self.buffer_size],
        })
    }

    /// Releases a slot (currently a no-op; buffers are owned by completions).
    pub fn release(&self, _slot: BufferSlot) {}
}

impl BufferSlot {
    /// Immutable view of the bytes.
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    /// Mutable view of the bytes.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data
    }

    /// Byte length of the slot.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` when the slot contains zero bytes.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Platform I/O driver owned by the main thread in the design topology.
pub struct PlatformIo {
    completion_tx: Sender<IoResult>,
    next_id: AtomicU64,
    pending: Vec<IoResult>,
}

fn map_io_error(err: std::io::Error) -> IoError {
    if err.kind() == std::io::ErrorKind::NotFound {
        IoError::NotFound
    } else if err.kind() == std::io::ErrorKind::PermissionDenied {
        IoError::PermissionDenied
    } else {
        IoError::Platform {
            code: err.raw_os_error().unwrap_or(-1),
        }
    }
}

impl PlatformIo {
    /// Creates a driver that posts completions to `completion_tx`.
    pub fn new(completion_tx: Sender<IoResult>) -> Self {
        Self {
            completion_tx,
            next_id: AtomicU64::new(1),
            pending: Vec::new(),
        }
    }

    /// Enqueues a read; completions flush when [`PlatformIo::poll_completions`] runs.
    pub fn submit_read(&mut self, path: &Path, mut buffer: BufferSlot) -> IoRequestId {
        let id = IoRequestId(self.next_id.fetch_add(1, Ordering::Relaxed));
        let io = match std::fs::read(path) {
            Ok(bytes) => {
                let n = bytes.len().min(buffer.data.len());
                buffer.data[..n].copy_from_slice(&bytes[..n]);
                buffer.data.truncate(n);
                IoResult {
                    id,
                    bytes_transferred: n as u32,
                    buffer,
                    status: Ok(()),
                }
            }
            Err(e) => IoResult {
                id,
                bytes_transferred: 0,
                buffer,
                status: Err(map_io_error(e)),
            },
        };
        self.pending.push(io);
        id
    }

    /// Enqueues a write; completions flush when [`PlatformIo::poll_completions`] runs.
    pub fn submit_write(&mut self, path: &Path, buffer: BufferSlot) -> IoRequestId {
        let id = IoRequestId(self.next_id.fetch_add(1, Ordering::Relaxed));
        let status = std::fs::write(path, buffer.as_slice()).map_err(map_io_error);
        let bytes_transferred = buffer.data.len() as u32;
        self.pending.push(IoResult {
            id,
            bytes_transferred,
            buffer,
            status,
        });
        id
    }

    /// Drains locally buffered completions onto the worker channel.
    pub fn poll_completions(&mut self) {
        for item in self.pending.drain(..) {
            let _ = self.completion_tx.send(item);
        }
    }
}
