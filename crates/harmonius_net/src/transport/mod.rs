//! Sans-io transport primitives (handshake, channels, fragmentation, congestion views).
//!
//! This crate ships algorithm stubs and types for deterministic tests. QUIC drivers,
//! `TransportSocket`, `ConnectionManager`, and main-thread I/O drains from the design land in
//! follow-up plans that wire these primitives to platform stacks.

pub mod congestion;
pub mod connection_state;
pub mod crypto_envelope;
pub mod diagnostics;
pub mod fragment;
pub mod handshake;
pub mod reliable_ordered;
pub mod types;
pub mod unreliable;

pub use connection_state::{ConnEvent, ConnectionStateMachine};
pub use crypto_envelope::{epoch_for_message_index, KeyEpoch};
#[cfg(test)]
pub use crypto_envelope::{test_transport_decrypt, test_transport_encrypt};
pub use diagnostics::Diagnostics;
pub use fragment::{effective_mtu, fragment_payload, reassemble_fragments};
pub use handshake::{Handshake, HandshakeError, HandshakeServer, SessionToken};
pub use reliable_ordered::ReliableOrderedChannel;
pub use types::{ChannelMode, ConnectionId, NetworkError, SequenceNumber, StreamId};
pub use unreliable::{UnreliableSequencedRx, UnreliableUnorderedChannel};
