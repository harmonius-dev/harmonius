use super::{
    GpuBufferHandle, IoBuffer, IoRequestId, SocketEndpoint, SocketHandle, SwapchainHandle, VPath,
};

/// Coarse classification of [`IoRequest`] variants for telemetry and dispatch routing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IoRequestKind {
    /// File read operations.
    Read,
    /// File write or append operations.
    Write,
    /// Socket lifecycle and framed I/O.
    Socket,
    /// Swap chain presentation.
    Present,
    /// GPU asset uploads.
    Upload,
    /// Subprocess spawning.
    Spawn,
    /// Control-plane operations such as cancellation.
    Control,
}

/// Every I/O path from every subsystem flows through this enum.
#[derive(Debug)]
pub enum IoRequest {
    /// Read an entire virtual file into `buffer`.
    ReadFile {
        /// Caller-assigned correlation id.
        id: IoRequestId,
        /// Virtual path to resolve through the [`super::VirtualFileSystem`].
        path: VPath,
        /// Destination buffer acquired from the pool.
        buffer: IoBuffer,
    },
    /// Overwrite a virtual file with `buffer` contents.
    WriteFile {
        /// Caller-assigned correlation id.
        id: IoRequestId,
        /// Virtual path to resolve through the [`super::VirtualFileSystem`].
        path: VPath,
        /// Source buffer containing bytes to write.
        buffer: IoBuffer,
    },
    /// Append to a virtual file.
    AppendFile {
        /// Caller-assigned correlation id.
        id: IoRequestId,
        /// Virtual path to resolve through the [`super::VirtualFileSystem`].
        path: VPath,
        /// Source buffer containing bytes to append.
        buffer: IoBuffer,
    },
    /// Read a byte range from a virtual file.
    ReadRange {
        /// Caller-assigned correlation id.
        id: IoRequestId,
        /// Virtual path to resolve through the [`super::VirtualFileSystem`].
        path: VPath,
        /// File offset in bytes.
        offset: u64,
        /// Number of bytes to read.
        len: u64,
        /// Destination buffer acquired from the pool.
        buffer: IoBuffer,
    },
    /// Open a socket to `endpoint`.
    OpenSocket {
        /// Caller-assigned correlation id.
        id: IoRequestId,
        /// Remote endpoint to connect.
        endpoint: SocketEndpoint,
    },
    /// Send a datagram or framed payload on an open socket.
    SendPacket {
        /// Caller-assigned correlation id.
        id: IoRequestId,
        /// Socket allocated by a prior `OpenSocketOk` response.
        socket: SocketHandle,
        /// Payload buffer.
        buffer: IoBuffer,
    },
    /// Receive into `buffer` from `socket`.
    RecvPacket {
        /// Caller-assigned correlation id.
        id: IoRequestId,
        /// Socket allocated by a prior `OpenSocketOk` response.
        socket: SocketHandle,
        /// Destination buffer for received bytes.
        buffer: IoBuffer,
    },
    /// Close a previously opened socket.
    CloseSocket {
        /// Caller-assigned correlation id.
        id: IoRequestId,
        /// Socket to close.
        socket: SocketHandle,
    },
    /// Present `image_index` from `swapchain`.
    SwapChainPresent {
        /// Caller-assigned correlation id.
        id: IoRequestId,
        /// Swap chain to present on.
        swapchain: SwapchainHandle,
        /// Image index chosen by the frame scheduler.
        image_index: u32,
    },
    /// Upload `source` bytes into `destination`.
    GpuAssetUpload {
        /// Caller-assigned correlation id.
        id: IoRequestId,
        /// Destination GPU buffer.
        destination: GpuBufferHandle,
        /// Staging buffer containing upload bytes.
        source: IoBuffer,
    },
    /// Spawn a child process with argv/env.
    SpawnProcess {
        /// Caller-assigned correlation id.
        id: IoRequestId,
        /// Argument vector including executable path in argv[0].
        argv: Vec<String>,
        /// Environment pairs inherited by the child.
        env: Vec<(String, String)>,
    },
    /// Touch a virtual file for cross-process signalling.
    SignalFile {
        /// Caller-assigned correlation id.
        id: IoRequestId,
        /// Virtual path to resolve through the [`super::VirtualFileSystem`].
        path: VPath,
    },
    /// Request cancellation of an in-flight request.
    CancelRequest {
        /// Request id to cancel.
        target: IoRequestId,
    },
}

impl IoRequest {
    /// Returns the coarse [`IoRequestKind`] for this request.
    #[must_use]
    pub fn kind(&self) -> IoRequestKind {
        match self {
            Self::ReadFile { .. } | Self::ReadRange { .. } => IoRequestKind::Read,
            Self::WriteFile { .. } | Self::AppendFile { .. } => IoRequestKind::Write,
            Self::OpenSocket { .. }
            | Self::SendPacket { .. }
            | Self::RecvPacket { .. }
            | Self::CloseSocket { .. } => IoRequestKind::Socket,
            Self::SwapChainPresent { .. } => IoRequestKind::Present,
            Self::GpuAssetUpload { .. } => IoRequestKind::Upload,
            Self::SpawnProcess { .. } => IoRequestKind::Spawn,
            Self::SignalFile { .. } | Self::CancelRequest { .. } => IoRequestKind::Control,
        }
    }

    /// Returns the caller correlation id when this variant participates in in-flight tracking.
    ///
    /// # Panics
    ///
    /// Panics for [`IoRequest::CancelRequest`] which carries a `target` instead.
    #[must_use]
    pub fn id(&self) -> IoRequestId {
        match self {
            Self::ReadFile { id, .. }
            | Self::WriteFile { id, .. }
            | Self::AppendFile { id, .. }
            | Self::ReadRange { id, .. }
            | Self::OpenSocket { id, .. }
            | Self::SendPacket { id, .. }
            | Self::RecvPacket { id, .. }
            | Self::CloseSocket { id, .. }
            | Self::SwapChainPresent { id, .. }
            | Self::GpuAssetUpload { id, .. }
            | Self::SpawnProcess { id, .. }
            | Self::SignalFile { id, .. } => *id,
            Self::CancelRequest { .. } => {
                panic!("CancelRequest does not have a request id; inspect `CancelRequest::target`")
            }
        }
    }

    /// Returns the cancellation target when this is [`IoRequest::CancelRequest`].
    #[must_use]
    pub fn cancel_target(&self) -> Option<IoRequestId> {
        match self {
            Self::CancelRequest { target } => Some(*target),
            _ => None,
        }
    }
}
