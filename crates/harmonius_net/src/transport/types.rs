//! Core transport identifiers and errors (subset of the design API).

use std::fmt;
use std::net::SocketAddr;

/// Opaque connection identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ConnectionId(pub u16);

/// QUIC stream / logical channel id.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StreamId(pub u32);

/// 16-bit wrapping sequence number.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SequenceNumber(pub u16);

impl SequenceNumber {
    /// Returns true when `self` is strictly newer than `other` in the 16-bit ring.
    pub fn is_newer_than(self, other: SequenceNumber) -> bool {
        let diff = self.0.wrapping_sub(other.0);
        diff > 0 && diff < 32768
    }

    /// Next sequence in the ring.
    pub fn next(self) -> SequenceNumber {
        SequenceNumber(self.0.wrapping_add(1))
    }
}

/// Channel delivery mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChannelMode {
    /// Reliable ordered (QUIC bidi stream semantics at the logical layer).
    ReliableOrdered,
    /// Reliable unordered.
    ReliableUnordered,
    /// Unreliable with sequence discard of stale arrivals.
    UnreliableSequenced,
    /// Unreliable unordered datagrams.
    UnreliableUnordered,
}

/// Canonical networking error for synchronous transport APIs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkError {
    /// Bind to local address failed.
    BindFailed {
        /// Target address.
        addr: SocketAddr,
        /// Platform errno-style code (test doubles use `0`).
        code: i32,
    },
    /// Anti-replay rejected a duplicated client nonce.
    ReplayDetected,
    /// Authentication failed with a reason string.
    AuthRejected {
        /// Human-readable reason.
        reason: String,
    },
    /// Malformed or truncated packet.
    InvalidPacket {
        /// Detail for logs/tests.
        detail: String,
    },
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for NetworkError {}
