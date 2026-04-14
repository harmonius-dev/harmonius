//! HMAC-based authentication tags for `VoicePacket`.

use crate::voice_packet::{VoiceChannelId, VoicePacket};
use hmac::Mac;
use sha2::Sha256;

type HmacSha256 = hmac::Hmac<Sha256>;

fn channel_wire_bytes(channel: VoiceChannelId) -> [u8; 5] {
    let mut out = [0u8; 5];
    match channel {
        VoiceChannelId::Proximity => {
            out[0] = 0;
        }
        VoiceChannelId::Party(id) => {
            out[0] = 1;
            out[1..5].copy_from_slice(&id.to_le_bytes());
        }
        VoiceChannelId::Raid(id) => {
            out[0] = 2;
            out[1..5].copy_from_slice(&id.to_le_bytes());
        }
        VoiceChannelId::Custom(id) => {
            out[0] = 3;
            out[1..5].copy_from_slice(&id.to_le_bytes());
        }
    }
    out
}

/// Computes the truncated HMAC tag for a voice packet payload.
#[must_use]
pub fn compute_voice_auth_tag(
    session_key: &[u8; 32],
    sequence: u32,
    channel: VoiceChannelId,
    opus_payload: &[u8],
) -> [u8; 8] {
    let mut mac =
        HmacSha256::new_from_slice(session_key.as_slice()).expect("HMAC accepts 32-byte keys");
    mac.update(&sequence.to_le_bytes());
    mac.update(&channel_wire_bytes(channel));
    mac.update(opus_payload);
    let full = mac.finalize().into_bytes();
    full[..8].try_into().expect("8-byte slice")
}

/// Returns `true` when the packet's `auth_tag` matches the session key and fields.
#[must_use]
pub fn voice_auth_tag_valid(packet: &VoicePacket, session_key: &[u8; 32]) -> bool {
    let payload_len = usize::from(packet.opus_len);
    if payload_len > packet.opus_data.len() {
        return false;
    }
    let expected = compute_voice_auth_tag(
        session_key,
        packet.sequence,
        packet.channel,
        &packet.opus_data[..payload_len],
    );
    expected == packet.auth_tag
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connection::ConnectionId;

    #[test]
    fn tc_ir_4_3_1_3_forged_auth_tag_rejected() {
        let key = [7u8; 32];
        let mut pkt = VoicePacket {
            sequence: 1,
            sender: ConnectionId(1),
            channel: VoiceChannelId::Proximity,
            auth_tag: [0u8; 8],
            opus_data: [0u8; 256],
            opus_len: 4,
        };
        pkt.auth_tag = compute_voice_auth_tag(&key, pkt.sequence, pkt.channel, &pkt.opus_data[..4]);
        assert!(voice_auth_tag_valid(&pkt, &key));
        pkt.auth_tag[0] ^= 0xFF;
        assert!(!voice_auth_tag_valid(&pkt, &key));
    }
}
