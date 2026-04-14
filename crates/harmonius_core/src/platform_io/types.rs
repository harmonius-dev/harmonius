//! I/O request identifiers, enums, and the in-memory [`PlatformIo`] reactor.

use std::collections::{BTreeMap, VecDeque};
use std::sync::atomic::{AtomicU64, Ordering};

/// Monotonic opaque request identifier.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IoRequestId(pub u64);

/// Relative scheduling priority for mock completions.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ReadPriority {
    /// Low-priority background traffic.
    Background = 0,
    /// High-priority latency-sensitive traffic.
    Critical = 1,
}

/// Top-level synchronous I/O operations accepted by [`PlatformIo`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IoRequest {
    /// Read bytes from a logical path at `offset`.
    Read {
        id: IoRequestId,
        path: String,
        offset: u64,
        len: usize,
        priority: ReadPriority,
    },
    /// Write bytes to a logical path at `offset`.
    Write {
        id: IoRequestId,
        path: String,
        offset: u64,
        data: Vec<u8>,
    },
    /// Vectored write preserving segment order.
    VectoredWrite {
        id: IoRequestId,
        path: String,
        offset: u64,
        segments: Vec<Vec<u8>>,
    },
    /// Cooperative cancellation keyed by `target`.
    Cancel {
        id: IoRequestId,
        target: IoRequestId,
    },
}

/// Completion payloads returned from [`PlatformIo::poll_completions`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IoResponse {
    /// Read succeeded with bytes.
    ReadOk { id: IoRequestId, data: Vec<u8> },
    /// Write succeeded with `bytes_written`.
    WriteOk {
        id: IoRequestId,
        bytes_written: usize,
    },
    /// Operation failed.
    Failed { id: IoRequestId, error: IoError },
}

/// Typed I/O failures surfaced to engine subsystems.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IoError {
    /// Request was cancelled before completion.
    Cancelled,
    /// Unknown request identifier for cancellation.
    UnknownRequest,
}

struct Pending {
    seq: u64,
    request: IoRequest,
    /// Synthetic syscall count (vectored writes amortize segments).
    syscall_cost: u32,
}

/// Deterministic in-memory reactor used by unit tests (no host OS I/O).
pub struct PlatformIo {
    next_id: AtomicU64,
    next_seq: u64,
    pending: VecDeque<Pending>,
    cancelled: BTreeMap<u64, ()>,
    files: BTreeMap<String, Vec<u8>>,
    poll_calls: u64,
    wake_events: u64,
    last_poll_syscalls: u32,
}

impl Default for PlatformIo {
    fn default() -> Self {
        Self::new()
    }
}

impl PlatformIo {
    /// Creates an empty reactor.
    #[must_use]
    pub fn new() -> Self {
        Self {
            next_id: AtomicU64::new(1),
            next_seq: 0,
            pending: VecDeque::new(),
            cancelled: BTreeMap::new(),
            files: BTreeMap::new(),
            poll_calls: 0,
            wake_events: 0,
            last_poll_syscalls: 0,
        }
    }

    /// Allocates a fresh [`IoRequestId`].
    #[must_use]
    pub fn next_request_id(&self) -> IoRequestId {
        let v = self.next_id.fetch_add(1, Ordering::Relaxed);
        IoRequestId(v)
    }

    /// Enqueues `request` for a later [`PlatformIo::poll_completions`] drain.
    pub fn submit(&mut self, request: IoRequest) {
        let syscall_cost = match &request {
            IoRequest::VectoredWrite { segments, .. } => {
                1u32.saturating_add((segments.len() as u32).saturating_sub(1) / 4)
            }
            _ => 1,
        };
        self.next_seq = self.next_seq.saturating_add(1);
        self.pending.push_back(Pending {
            seq: self.next_seq,
            request,
            syscall_cost,
        });
    }

    /// Marks `target` as cancelled; a completion will be emitted on the next poll.
    pub fn cancel(&mut self, target: IoRequestId) {
        self.cancelled.insert(target.0, ());
    }

    /// Returns how many logical wake/completion events have been delivered.
    #[must_use]
    pub fn wake_events(&self) -> u64 {
        self.wake_events
    }

    /// Returns how many times [`PlatformIo::poll_completions`] was invoked.
    #[must_use]
    pub fn poll_calls(&self) -> u64 {
        self.poll_calls
    }

    /// Total synthetic syscalls attributed to the last [`PlatformIo::poll_completions`] call.
    #[must_use]
    pub fn syscall_units_for_last_poll(&self) -> u32 {
        self.last_poll_syscalls
    }

    /// Drains ready completions, honoring cancellation and read priority.
    pub fn poll_completions(&mut self) -> Vec<IoResponse> {
        self.poll_calls = self.poll_calls.saturating_add(1);
        self.last_poll_syscalls = 0;
        self.reorder_reads();
        let mut out = Vec::new();
        while let Some(p) = self.pending.pop_front() {
            if let IoRequest::Cancel { id, target } = &p.request {
                self.cancelled.insert(target.0, ());
                out.push(IoResponse::WriteOk {
                    id: *id,
                    bytes_written: 0,
                });
                self.wake_events = self.wake_events.saturating_add(1);
                continue;
            }
            let id = match &p.request {
                IoRequest::Read { id, .. }
                | IoRequest::Write { id, .. }
                | IoRequest::VectoredWrite { id, .. } => *id,
                IoRequest::Cancel { .. } => unreachable!(),
            };
            if self.cancelled.remove(&id.0).is_some() {
                out.push(IoResponse::Failed {
                    id,
                    error: IoError::Cancelled,
                });
                self.wake_events = self.wake_events.saturating_add(1);
                continue;
            }
            self.last_poll_syscalls = self.last_poll_syscalls.saturating_add(p.syscall_cost);
            match p.request {
                IoRequest::Read {
                    id,
                    path,
                    offset,
                    len,
                    ..
                } => {
                    let data = read_file_range(&mut self.files, &path, offset, len);
                    out.push(IoResponse::ReadOk { id, data });
                }
                IoRequest::Write {
                    id,
                    path,
                    offset,
                    data,
                } => {
                    write_file_range(&mut self.files, &path, offset, &data);
                    let bytes_written = data.len();
                    out.push(IoResponse::WriteOk { id, bytes_written });
                }
                IoRequest::VectoredWrite {
                    id,
                    path,
                    offset,
                    segments,
                } => {
                    let mut cursor = offset;
                    let mut total = 0usize;
                    for seg in &segments {
                        write_file_range(&mut self.files, &path, cursor, seg);
                        cursor = cursor.saturating_add(seg.len() as u64);
                        total += seg.len();
                    }
                    out.push(IoResponse::WriteOk {
                        id,
                        bytes_written: total,
                    });
                }
                IoRequest::Cancel { .. } => {}
            }
            self.wake_events = self.wake_events.saturating_add(1);
        }
        out
    }

    fn reorder_reads(&mut self) {
        let mut tmp: Vec<Pending> = self.pending.drain(..).collect();
        tmp.sort_by_key(pending_sort_key);
        self.pending = tmp.into();
    }
}

fn pending_sort_key(p: &Pending) -> (u8, u64) {
    match &p.request {
        IoRequest::Read { priority, .. } => (
            match priority {
                ReadPriority::Critical => 0,
                ReadPriority::Background => 1,
            },
            p.seq,
        ),
        _ => (0, p.seq),
    }
}

fn read_file_range(
    files: &mut BTreeMap<String, Vec<u8>>,
    path: &str,
    offset: u64,
    len: usize,
) -> Vec<u8> {
    let mut out = vec![0_u8; len];
    let Some(buf) = files.get(path) else {
        return out;
    };
    let start = usize::try_from(offset).unwrap_or(usize::MAX);
    if start >= buf.len() {
        return out;
    }
    let take = len.min(buf.len().saturating_sub(start));
    out[..take].copy_from_slice(&buf[start..start + take]);
    out
}

fn write_file_range(files: &mut BTreeMap<String, Vec<u8>>, path: &str, offset: u64, data: &[u8]) {
    let entry = files.entry(path.to_string()).or_default();
    let start = usize::try_from(offset).unwrap_or(usize::MAX);
    let end = start.saturating_add(data.len());
    if end > entry.len() {
        entry.resize(end, 0);
    }
    entry[start..end].copy_from_slice(data);
}
