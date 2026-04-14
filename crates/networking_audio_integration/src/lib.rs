//! Networking ↔ audio voice integration types and algorithms.
//!
//! Implements the contracts in `docs/design/integration/networking-audio.md`.

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
