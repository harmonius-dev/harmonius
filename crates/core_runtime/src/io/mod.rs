//! Main-thread I/O request and response protocol.
//!
//! ## Bring-up limitations
//!
//! The dispatcher uses **synchronous** [`std::fs`] calls on the main thread in this slice to
//! validate the request/response protocol. A later milestone swaps in platform-native non-blocking
//! I/O while preserving these channel boundaries.

mod buffer;
mod dispatcher;
mod handles;
mod request;
mod response;
mod vfs;

pub use buffer::{IoBuffer, IoBufferPool, IoBufferSlot};
pub use dispatcher::{
    IoBackendKind, IoDispatcher, IoDispatcherChannels, IoDispatcherConfig, Ticket,
};
pub use handles::{GpuBufferHandle, SocketEndpoint, SocketHandle, SwapchainHandle};
pub use request::{IoRequest, IoRequestKind};
pub use response::IoResponse;
pub use vfs::{Mount, MountBackend, MountId, VPath, VirtualFileSystem};

pub use crate::error::IoError;

/// Stable identifier assigned by the caller for correlating requests and responses.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IoRequestId(pub u64);
