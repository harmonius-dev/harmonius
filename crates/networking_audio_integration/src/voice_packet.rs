//! Wire-level voice packets and channel RPC payloads.

use crate::connection::ConnectionId;

/// Identifier for a proximity, party, raid, or custom voice channel.
#[derive(
    Clone, Copy, Debug, Eq, Hash, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize,
)]
#[rkyv(compare(PartialEq), derive(Debug, Eq, Hash, PartialEq))]
pub enum VoiceChannelId {
    /// Single global proximity channel instance.
    Proximity,
    /// Party-scoped channel.
    Party(u32),
    /// Raid-scoped channel.
    Raid(u32),
    /// User-defined channel bucket.
    Custom(u32),
}

/// Server response status for a join attempt.
#[derive(
    Clone, Copy, Debug, Eq, Hash, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize,
)]
#[rkyv(compare(PartialEq), derive(Debug, Eq, Hash, PartialEq))]
pub enum VoiceChannelResult {
    /// Join succeeded.
    Ok,
    /// Client lacks authorization.
    NotAuthorized,
    /// Channel already at capacity.
    ChannelFull,
    /// Channel id does not exist.
    InvalidChannel,
}

/// Reliable RPC payloads for voice channel membership.
#[derive(Clone, Debug, Eq, Hash, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[rkyv(compare(PartialEq), derive(Debug, Eq, PartialEq))]
pub enum VoiceChannelRpc {
    /// Client asks to join a channel.
    JoinRequest {
        /// Target channel.
        channel: VoiceChannelId,
    },
    /// Server accepts or rejects the join.
    JoinResponse {
        /// Channel that was joined.
        channel: VoiceChannelId,
        /// Outcome.
        result: VoiceChannelResult,
    },
    /// Client asks to leave a channel.
    LeaveRequest {
        /// Channel to leave.
        channel: VoiceChannelId,
    },
    /// Server acknowledges the leave.
    LeaveAck {
        /// Channel that was left.
        channel: VoiceChannelId,
    },
}

/// One Opus frame carried on the unreliable voice datagram path.
#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[rkyv(compare(PartialEq), derive(Debug, Eq, PartialEq))]
pub struct VoicePacket {
    /// Monotonic sequence for jitter buffer ordering.
    pub sequence: u32,
    /// Sender connection for demuxing.
    pub sender: ConnectionId,
    /// Voice channel (proximity, party, raid).
    pub channel: VoiceChannelId,
    /// Truncated HMAC over authenticated fields.
    pub auth_tag: [u8; 8],
    /// Opus-encoded audio frame (fixed buffer).
    pub opus_data: [u8; 256],
    /// Active length of the Opus payload in `opus_data`.
    pub opus_len: u8,
}
