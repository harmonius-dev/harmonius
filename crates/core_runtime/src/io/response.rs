use super::{IoBuffer, IoRequestId, SocketHandle};
use crate::error::IoError;

/// Completion records emitted by the main-thread dispatcher.
#[derive(Debug)]
pub enum IoResponse {
    /// Successful file read completion.
    ReadOk {
        /// Correlation id from the original [`super::IoRequest`].
        id: IoRequestId,
        /// Number of bytes read into `buffer`.
        bytes_read: u64,
        /// Buffer returned to the caller for pool release.
        buffer: IoBuffer,
    },
    /// Successful file write completion.
    WriteOk {
        /// Correlation id from the original [`super::IoRequest`].
        id: IoRequestId,
        /// Number of bytes written.
        bytes_written: u64,
    },
    /// Socket opened successfully.
    OpenSocketOk {
        /// Correlation id from the original [`super::IoRequest`].
        id: IoRequestId,
        /// Handle chosen by the platform layer.
        socket: SocketHandle,
    },
    /// Datagram or framed payload sent.
    SendPacketOk {
        /// Correlation id from the original [`super::IoRequest`].
        id: IoRequestId,
        /// Number of bytes sent.
        bytes_sent: u64,
    },
    /// Datagram or framed payload received.
    RecvPacketOk {
        /// Correlation id from the original [`super::IoRequest`].
        id: IoRequestId,
        /// Number of bytes received into `buffer`.
        bytes_recv: u64,
        /// Buffer containing received bytes.
        buffer: IoBuffer,
    },
    /// Swap chain present completed.
    PresentOk {
        /// Correlation id from the original [`super::IoRequest`].
        id: IoRequestId,
    },
    /// GPU upload completed.
    UploadOk {
        /// Correlation id from the original [`super::IoRequest`].
        id: IoRequestId,
    },
    /// Child process exited.
    SpawnOk {
        /// Correlation id from the original [`super::IoRequest`].
        id: IoRequestId,
        /// Process exit code.
        exit_code: i32,
        /// Captured stdout bytes.
        stdout: IoBuffer,
        /// Captured stderr bytes.
        stderr: IoBuffer,
    },
    /// Operation failed with a structured error.
    Failed {
        /// Correlation id from the original [`super::IoRequest`].
        id: IoRequestId,
        /// Failure reason.
        error: IoError,
    },
    /// Operation was cancelled before completion.
    Cancelled {
        /// Correlation id from the original [`super::IoRequest`].
        id: IoRequestId,
    },
}
