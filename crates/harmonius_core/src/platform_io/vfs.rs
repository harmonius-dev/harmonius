//! Virtual path helpers layered on [`PlatformIo`](super::PlatformIo).

use super::{IoRequest, IoRequestId, IoResponse, PlatformIo, ReadPriority};

/// Thin wrapper for path-oriented helpers (tests and future mount tables).
#[derive(Default)]
pub struct VirtualFileSystem {
    io: PlatformIo,
}

impl VirtualFileSystem {
    /// Borrows the underlying reactor.
    #[must_use]
    pub fn reactor(&self) -> &PlatformIo {
        &self.io
    }

    /// Mutably borrows the underlying reactor.
    pub fn reactor_mut(&mut self) -> &mut PlatformIo {
        &mut self.io
    }

    /// Submits a read for `path` at `offset` for `len` bytes.
    pub fn submit_read(
        &mut self,
        path: &str,
        offset: u64,
        len: usize,
        priority: ReadPriority,
    ) -> IoRequestId {
        let id = self.io.next_request_id();
        self.io.submit(IoRequest::Read {
            id,
            path: path.to_string(),
            offset,
            len,
            priority,
        });
        id
    }

    /// Submits a write for `path` at `offset`.
    pub fn submit_write(&mut self, path: &str, offset: u64, data: Vec<u8>) -> IoRequestId {
        let id = self.io.next_request_id();
        self.io.submit(IoRequest::Write {
            id,
            path: path.to_string(),
            offset,
            data,
        });
        id
    }

    /// Drains completions from the reactor.
    pub fn poll_completions(&mut self) -> Vec<IoResponse> {
        self.io.poll_completions()
    }
}
