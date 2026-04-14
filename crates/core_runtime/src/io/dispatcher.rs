//! Main-thread dispatcher that drains requests and emits responses.
//!
//! ## Phase-1 limitations
//!
//! - Request and response [`crossbeam_channel`] queues are **unbounded**; subsystems must apply
//!   their own backpressure until a bounded design is integrated.
//! - File operations use **blocking** [`std::fs`] APIs here to exercise the protocol on every host.
//!   This is an intentional bring-up shortcut relative to the engine’s non-blocking I/O goals; treat
//!   latency as non-representative of the final platform backends.

use crossbeam_channel::{Receiver, Sender};
use std::collections::BTreeMap;
use std::fs;
use std::io::Write as _;

use crate::error::{map_std_io_error, IoError};
use crate::primitives::SortedVecMap;

use super::buffer::{IoBuffer, IoBufferPool};
use super::request::{IoRequest, IoRequestKind};
use super::response::IoResponse;
use super::vfs::{VPath, VirtualFileSystem};
use super::{IoRequestId, SocketHandle};

/// Controls how the dispatcher advances work during [`IoDispatcher::poll_completions`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IoBackendKind {
    /// Completes supported operations immediately after ingestion.
    Loopback,
    /// Queues **only** [`IoRequest::ReadFile`] requests in `in_flight` without completing them until
    /// cancellation. Every other request follows the same immediate completion path as
    /// [`IoBackendKind::Loopback`] so buffers are never stranded.
    StallReads,
}

/// Bookkeeping entry for an outstanding request.
#[derive(Debug)]
pub struct Ticket {
    /// Correlation id shared with the originating [`IoRequest`].
    pub request_id: IoRequestId,
    /// Coarse request classification.
    pub kind: IoRequestKind,
    /// Engine tick when the request was observed on the main thread.
    pub submitted_at_tick: u64,
    /// Optional buffer retained while a read is stalled.
    pub buffer: Option<IoBuffer>,
}

/// Channel endpoints owned by subsystems that talk to [`IoDispatcher`].
pub struct IoDispatcherChannels {
    /// Subsystems enqueue [`IoRequest`] values here.
    pub request_tx: Sender<IoRequest>,
    /// Subsystems dequeue [`IoResponse`] values here.
    pub response_rx: Receiver<IoResponse>,
}

/// Configuration bundle for [`IoDispatcher::new`].
pub struct IoDispatcherConfig {
    /// Initial mount table used for path resolution.
    pub vfs: VirtualFileSystem,
    /// Buffer pool shared by dispatcher and subsystems.
    pub buffers: IoBufferPool,
    /// Backend selection for tests and bring-up environments.
    pub backend: IoBackendKind,
}

/// Main-thread-owned dispatcher state.
pub struct IoDispatcher {
    request_rx: Receiver<IoRequest>,
    response_tx: Sender<IoResponse>,
    in_flight: SortedVecMap<IoRequestId, Ticket>,
    buffers: IoBufferPool,
    vfs: VirtualFileSystem,
    backend: IoBackendKind,
    next_socket: u32,
    loopback_payloads: BTreeMap<SocketHandle, Vec<u8>>,
}

impl IoDispatcher {
    /// Creates a dispatcher plus the channel pair used by subsystems.
    ///
    /// Both channels are [`crossbeam_channel::unbounded`]; see the module-level note on
    /// backpressure.
    #[must_use]
    pub fn new(config: IoDispatcherConfig) -> (Self, IoDispatcherChannels) {
        let (request_tx, request_rx) = crossbeam_channel::unbounded();
        let (response_tx, response_rx) = crossbeam_channel::unbounded();
        let dispatcher = Self {
            request_rx,
            response_tx,
            in_flight: SortedVecMap::default(),
            buffers: config.buffers,
            vfs: config.vfs,
            backend: config.backend,
            next_socket: 1,
            loopback_payloads: BTreeMap::new(),
        };
        (
            dispatcher,
            IoDispatcherChannels {
                request_tx,
                response_rx,
            },
        )
    }

    /// Borrows the mount table.
    #[must_use]
    pub fn vfs(&self) -> &VirtualFileSystem {
        &self.vfs
    }

    /// Mutably borrows the mount table.
    pub fn vfs_mut(&mut self) -> &mut VirtualFileSystem {
        &mut self.vfs
    }

    /// Borrows the buffer pool.
    #[must_use]
    pub fn buffers(&self) -> &IoBufferPool {
        &self.buffers
    }

    /// Mutably borrows the buffer pool.
    pub fn buffers_mut(&mut self) -> &mut IoBufferPool {
        &mut self.buffers
    }

    /// Switches backend behavior for deterministic tests.
    pub fn set_backend(&mut self, backend: IoBackendKind) {
        self.backend = backend;
    }

    /// Returns how many requests are currently tracked as in-flight.
    #[must_use]
    pub fn in_flight_len(&self) -> usize {
        self.in_flight.len()
    }

    /// Returns in-flight ids sorted ascending for determinism assertions.
    #[must_use]
    pub fn in_flight_ids(&self) -> Vec<IoRequestId> {
        self.in_flight.keys().collect()
    }

    /// Drains pending requests and advances completions for the current frame.
    ///
    /// `now_tick` is stored on [`Ticket::submitted_at_tick`] when [`IoBackendKind::StallReads`]
    /// queues a [`IoRequest::ReadFile`].
    pub fn poll_completions(&mut self, now_tick: u64) {
        let mut pending = Vec::new();
        while let Ok(r) = self.request_rx.try_recv() {
            pending.push(r);
        }

        for req in pending {
            self.dispatch_one(req, now_tick);
        }
    }

    /// Returns `true` when non-`ReadFile` work should complete immediately for the active backend.
    #[inline]
    fn completes_non_read_file_immediately(&self) -> bool {
        matches!(
            self.backend,
            IoBackendKind::Loopback | IoBackendKind::StallReads
        )
    }

    fn dispatch_one(&mut self, req: IoRequest, tick: u64) {
        match req {
            IoRequest::CancelRequest { target } => {
                if let Some(mut ticket) = self.in_flight.remove(target) {
                    if let Some(buf) = ticket.buffer.take() {
                        self.buffers.release(buf);
                    }
                    let _ = self.response_tx.send(IoResponse::Cancelled { id: target });
                }
                // No response when `target` is unknown or already completed: explicit no-op.
            }
            IoRequest::ReadFile { id, path, buffer } => match self.backend {
                IoBackendKind::StallReads => {
                    debug_assert!(
                        self.in_flight.get(id).is_none(),
                        "duplicate IoRequestId while stalling reads"
                    );
                    self.in_flight.insert(
                        id,
                        Ticket {
                            request_id: id,
                            kind: IoRequestKind::Read,
                            submitted_at_tick: tick,
                            buffer: Some(buffer),
                        },
                    );
                }
                IoBackendKind::Loopback => {
                    self.complete_read_file(id, path, buffer);
                }
            },
            IoRequest::WriteFile { id, path, buffer } => {
                if self.completes_non_read_file_immediately() {
                    self.complete_write_file(id, path, buffer);
                }
            }
            IoRequest::OpenSocket { id, endpoint: _ } => {
                if self.completes_non_read_file_immediately() {
                    let handle = SocketHandle(self.next_socket);
                    self.next_socket = self.next_socket.wrapping_add(1);
                    let _ = self
                        .response_tx
                        .send(IoResponse::OpenSocketOk { id, socket: handle });
                }
            }
            IoRequest::SendPacket { id, socket, buffer } => {
                if self.completes_non_read_file_immediately() {
                    // Loopback stores a copy for deterministic recv; a pooled arena can replace this
                    // if this path becomes hot.
                    let len = buffer.len;
                    let bytes = self.buffers.as_slice(&buffer).to_vec();
                    self.buffers.release(buffer);
                    self.loopback_payloads.insert(socket, bytes);
                    let _ = self.response_tx.send(IoResponse::SendPacketOk {
                        id,
                        bytes_sent: u64::from(len),
                    });
                }
            }
            IoRequest::RecvPacket {
                id,
                socket,
                mut buffer,
            } => {
                if self.completes_non_read_file_immediately() {
                    let payload = self
                        .loopback_payloads
                        .get(&socket)
                        .cloned()
                        .unwrap_or_default();
                    let take = (buffer.capacity() as usize).min(payload.len());
                    let slice = payload.get(..take).unwrap_or(&[]);
                    self.buffers.write_all(&mut buffer, slice);
                    let bytes_recv = u64::from(buffer.len);
                    let _ = self.response_tx.send(IoResponse::RecvPacketOk {
                        id,
                        bytes_recv,
                        buffer,
                    });
                }
            }
            IoRequest::CloseSocket { socket, .. } => {
                if self.completes_non_read_file_immediately() {
                    self.loopback_payloads.remove(&socket);
                }
            }
            IoRequest::SwapChainPresent { id, .. } => {
                if self.completes_non_read_file_immediately() {
                    let _ = self.response_tx.send(IoResponse::PresentOk { id });
                }
            }
            IoRequest::GpuAssetUpload { id, source, .. } => {
                if self.completes_non_read_file_immediately() {
                    self.buffers.release(source);
                    let _ = self.response_tx.send(IoResponse::UploadOk { id });
                }
            }
            IoRequest::SpawnProcess { id, .. } => {
                if self.completes_non_read_file_immediately() {
                    let mut stdout = self.buffers.acquire(1);
                    self.buffers.write_all(&mut stdout, b"\n");
                    let mut stderr = self.buffers.acquire(1);
                    self.buffers.write_all(&mut stderr, b"");
                    let _ = self.response_tx.send(IoResponse::SpawnOk {
                        id,
                        exit_code: 0,
                        stdout,
                        stderr,
                    });
                }
            }
            IoRequest::SignalFile { id, .. } => {
                if self.completes_non_read_file_immediately() {
                    // Loopback has no cross-process watcher; acknowledge with zero bytes touched.
                    let _ = self.response_tx.send(IoResponse::WriteOk {
                        id,
                        bytes_written: 0,
                    });
                }
            }
            IoRequest::AppendFile { id, path, buffer } => {
                if self.completes_non_read_file_immediately() {
                    self.complete_append_file(id, path, buffer);
                }
            }
            IoRequest::ReadRange {
                id,
                path,
                offset,
                len,
                buffer,
            } => {
                if self.completes_non_read_file_immediately() {
                    self.complete_read_range(id, path, offset, len, buffer);
                }
            }
        }
    }

    fn complete_read_file(&mut self, id: IoRequestId, path: VPath, mut buffer: IoBuffer) {
        match self.read_bytes_for_path(&path) {
            Ok(bytes) => {
                let take = (buffer.capacity() as usize).min(bytes.len());
                self.buffers.write_all(&mut buffer, &bytes[..take]);
                let bytes_read = u64::from(buffer.len);
                let _ = self.response_tx.send(IoResponse::ReadOk {
                    id,
                    bytes_read,
                    buffer,
                });
            }
            Err(err) => {
                self.buffers.release(buffer);
                let _ = self.response_tx.send(IoResponse::Failed { id, error: err });
            }
        }
    }

    fn complete_read_range(
        &mut self,
        id: IoRequestId,
        path: VPath,
        offset: u64,
        len: u64,
        mut buffer: IoBuffer,
    ) {
        match self.read_bytes_for_path(&path) {
            Ok(bytes) => {
                let start = usize::try_from(offset).unwrap_or(usize::MAX);
                let want = usize::try_from(len).unwrap_or(usize::MAX);
                let end = start.saturating_add(want).min(bytes.len());
                let slice = bytes.get(start..end).unwrap_or(&[]);
                let take = (buffer.capacity() as usize).min(slice.len());
                self.buffers.write_all(&mut buffer, &slice[..take]);
                let bytes_read = u64::from(buffer.len);
                let _ = self.response_tx.send(IoResponse::ReadOk {
                    id,
                    bytes_read,
                    buffer,
                });
            }
            Err(err) => {
                self.buffers.release(buffer);
                let _ = self.response_tx.send(IoResponse::Failed { id, error: err });
            }
        }
    }

    fn complete_write_file(&mut self, id: IoRequestId, path: VPath, buffer: IoBuffer) {
        let result = self.write_bytes_for_path(&path, buffer, false);
        match result {
            Ok(bytes_written) => {
                let _ = self
                    .response_tx
                    .send(IoResponse::WriteOk { id, bytes_written });
            }
            Err((buf, err)) => {
                self.buffers.release(buf);
                let _ = self.response_tx.send(IoResponse::Failed { id, error: err });
            }
        }
    }

    fn complete_append_file(&mut self, id: IoRequestId, path: VPath, buffer: IoBuffer) {
        let result = self.write_bytes_for_path(&path, buffer, true);
        match result {
            Ok(bytes_written) => {
                let _ = self
                    .response_tx
                    .send(IoResponse::WriteOk { id, bytes_written });
            }
            Err((buf, err)) => {
                self.buffers.release(buf);
                let _ = self.response_tx.send(IoResponse::Failed { id, error: err });
            }
        }
    }

    fn read_bytes_for_path(&self, path: &VPath) -> Result<Vec<u8>, IoError> {
        // Deterministic test fixture: not routed through the mount table.
        if path.0.starts_with("mem://") {
            return Ok(vec![0xAB; 1024]);
        }
        let resolved = self.vfs.resolve(path).ok_or_else(|| IoError::NotFound {
            path: path.0.clone(),
        })?;
        let path_str = resolved.display().to_string();
        fs::read(&resolved).map_err(|e| map_std_io_error(path_str, e))
    }

    fn write_bytes_for_path(
        &mut self,
        path: &VPath,
        buffer: IoBuffer,
        append: bool,
    ) -> Result<u64, (IoBuffer, IoError)> {
        let resolved = match self.vfs.resolve(path) {
            Some(p) => p,
            None => {
                return Err((
                    buffer,
                    IoError::NotFound {
                        path: path.0.clone(),
                    },
                ));
            }
        };
        if let Some(parent) = resolved.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let payload = self.buffers.as_slice(&buffer).to_vec();
        let len = payload.len() as u64;
        let open = if append {
            fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&resolved)
        } else {
            fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&resolved)
        };
        let path_str = resolved.display().to_string();
        let mut file = match open {
            Ok(f) => f,
            Err(e) => {
                return Err((buffer, map_std_io_error(path_str, e)));
            }
        };
        if let Err(e) = file.write_all(&payload) {
            return Err((buffer, map_std_io_error(path_str, e)));
        }
        self.buffers.release(buffer);
        Ok(len)
    }
}
