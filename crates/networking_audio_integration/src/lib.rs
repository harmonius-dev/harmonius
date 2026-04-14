//! Networking ↔ audio voice integration types and algorithms.
//!
//! Implements the contracts in `docs/design/integration/networking-audio.md`.
//!
//! # Implementation notes (review / integration slice)
//!
//! - Opus encode and decode use the pure-Rust [`opus_rs`](https://docs.rs/opus-rs) crate with a
//!   last-good-frame PLC stand-in on loss; libopus-specific PLC is a follow-up if bit-exact RFC
//!   6716 behavior is required.
//! - [`spatial::VoiceSpatialQueue`] is a single-threaded staging queue with the same capacity and
//!   oldest-drop policy as the design’s bounded bridge; a `crossbeam_channel` MPSC façade for
//!   cross-thread producers is deferred to the transport integration plan.
//! - QUIC loopback, `dhat` allocation proofs, and CI benchmark gates called out in the companion
//!   test-case document remain explicitly out of scope for this crate until those harnesses land.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod acoustic_echo_canceller;
mod auth;
mod channel_manager;
mod connection;
mod jitter_buffer;
mod noise_suppressor;
mod opus_codec;
mod opus_frame_pool;
mod spatial;
mod voice_activity_detector;
mod voice_channel_router;
mod voice_packet;
mod voice_stream_mixer;

pub use acoustic_echo_canceller::AcousticEchoCanceller;
pub use auth::{compute_voice_auth_tag, voice_auth_tag_valid};
pub use channel_manager::{ChannelManager, VoiceRpcSink};
pub use connection::ConnectionId;
pub use jitter_buffer::JitterBuffer;
pub use noise_suppressor::NoiseSuppressor;
pub use opus_codec::{OpusDecoder, OpusEncoder};
pub use opus_frame_pool::{OpusFrameHandle, OpusFramePool};
pub use spatial::{SpatialParams, Vec3, VoiceSpatialCommand, VoiceSpatialQueue};
pub use voice_activity_detector::VoiceActivityDetector;
pub use voice_channel_router::{ConnectionSlot, VoiceChannelRouter};
pub use voice_packet::{VoiceChannelId, VoiceChannelResult, VoiceChannelRpc, VoicePacket};
pub use voice_stream_mixer::mix_voice_streams;
