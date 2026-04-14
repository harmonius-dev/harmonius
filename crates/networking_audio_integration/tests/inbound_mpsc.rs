//! Inbound voice packet queue backpressure (design capacity 1024).

use crossbeam_channel::bounded;
use networking_audio_integration::{ConnectionId, VoiceChannelId, VoicePacket};

fn dummy_packet(seq: u32) -> VoicePacket {
    VoicePacket {
        sequence: seq,
        sender: ConnectionId(1),
        channel: VoiceChannelId::Proximity,
        auth_tag: [0u8; 8],
        opus_data: [0u8; 256],
        opus_len: 0,
    }
}

/// TC-IR-4.3.1.4 — bounded channel rejects the 1025th `try_send`.
#[test]
fn tc_ir_4_3_1_4_inbound_mpsc_full() {
    let (sender, _receiver) = bounded::<VoicePacket>(1024);
    for i in 0u32..1024 {
        sender
            .try_send(dummy_packet(i))
            .expect("queue accepts 1024 voice packets");
    }
    assert!(sender.try_send(dummy_packet(1024)).is_err());
}
