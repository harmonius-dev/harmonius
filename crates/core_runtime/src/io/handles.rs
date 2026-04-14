//! Lightweight opaque handles referenced by [`super::IoRequest`] and [`super::IoResponse`].

/// Logical socket endpoint description (host + port).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SocketEndpoint {
    /// Hostname or textual address.
    pub host: String,
    /// Port number in host byte order.
    pub port: u16,
}

/// Stable handle for an opened socket.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SocketHandle(pub u32);

/// Stable handle for a swap chain object.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SwapchainHandle(pub u32);

/// Stable handle for a GPU buffer resource.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GpuBufferHandle(pub u32);
