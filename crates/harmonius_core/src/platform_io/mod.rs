//! Synchronous platform I/O facade and in-memory test doubles.

mod types;
mod vfs;

pub use types::{IoError, IoRequest, IoRequestId, IoResponse, PlatformIo, ReadPriority};
pub use vfs::VirtualFileSystem;
