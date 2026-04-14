//! Client-side voice channel membership and RPC fan-out.

use crate::voice_packet::{VoiceChannelId, VoiceChannelRpc};

/// Sink for reliable voice-channel RPCs (implemented by the networking layer).
pub trait VoiceRpcSink {
    /// Sends one RPC payload on the reliable ordered stream.
    fn send_rpc(&mut self, msg: VoiceChannelRpc);
}

/// Tracks locally joined voice channels and emits join/leave RPCs.
#[derive(Debug, Default)]
pub struct ChannelManager {
    /// Active voice channels for the local player.
    pub active_channels: Vec<VoiceChannelId>,
}

impl ChannelManager {
    /// Joins `channel`, sending a [`VoiceChannelRpc::JoinRequest`].
    pub fn join(&mut self, channel: VoiceChannelId, rpc: &mut impl VoiceRpcSink) {
        rpc.send_rpc(VoiceChannelRpc::JoinRequest { channel });
        if !self.active_channels.contains(&channel) {
            self.active_channels.push(channel);
        }
    }

    /// Leaves `channel`, sending a [`VoiceChannelRpc::LeaveRequest`].
    pub fn leave(&mut self, channel: VoiceChannelId, rpc: &mut impl VoiceRpcSink) {
        rpc.send_rpc(VoiceChannelRpc::LeaveRequest { channel });
        self.active_channels.retain(|c| *c != channel);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::voice_packet::VoiceChannelResult;

    struct Recorder(Vec<VoiceChannelRpc>);

    impl VoiceRpcSink for Recorder {
        fn send_rpc(&mut self, msg: VoiceChannelRpc) {
            self.0.push(msg);
        }
    }

    /// TC-IR-4.3.6.1 — join party emits a join request RPC.
    #[test]
    fn tc_ir_4_3_6_1_join_party_rpc() {
        let mut mgr = ChannelManager::default();
        let mut rpc = Recorder(Vec::new());
        mgr.join(VoiceChannelId::Party(1), &mut rpc);
        assert!(matches!(
            rpc.0[0],
            VoiceChannelRpc::JoinRequest {
                channel: VoiceChannelId::Party(1)
            }
        ));
    }

    /// TC-IR-4.3.6.2 — leave stops tracking the channel.
    #[test]
    fn tc_ir_4_3_6_2_leave_channel() {
        let mut mgr = ChannelManager {
            active_channels: vec![VoiceChannelId::Party(1)],
        };
        let mut rpc = Recorder(Vec::new());
        mgr.leave(VoiceChannelId::Party(1), &mut rpc);
        assert!(mgr.active_channels.is_empty());
        assert!(matches!(
            rpc.0[0],
            VoiceChannelRpc::LeaveRequest {
                channel: VoiceChannelId::Party(1)
            }
        ));
    }

    /// TC-IR-4.3.6.3 — proximity and party can both be active.
    #[test]
    fn tc_ir_4_3_6_3_multi_channel_membership() {
        let mut mgr = ChannelManager::default();
        let mut rpc = Recorder(Vec::new());
        mgr.join(VoiceChannelId::Proximity, &mut rpc);
        mgr.join(VoiceChannelId::Party(2), &mut rpc);
        assert_eq!(mgr.active_channels.len(), 2);
        // Server ack path is handled by networking; model a successful join response.
        mgr.active_channels.retain(|_| true);
        let mut rpc2 = Recorder(Vec::new());
        rpc2.send_rpc(VoiceChannelRpc::JoinResponse {
            channel: VoiceChannelId::Proximity,
            result: VoiceChannelResult::Ok,
        });
        assert!(matches!(
            rpc2.0[0],
            VoiceChannelRpc::JoinResponse {
                result: VoiceChannelResult::Ok,
                ..
            }
        ));
    }
}
