//! Bounded timeline→audio command queue (design capacity: 4096).

use std::sync::mpsc::{sync_channel, SyncSender, TrySendError};
use std::sync::Arc;

use crate::debug::TimelineAudioDebug;
use crate::types::AudioCommand;

const COMMAND_QUEUE_CAP: usize = 4096;

/// Sender handle cloned by timeline producers.
#[derive(Clone, Debug)]
pub struct AudioCommandSender {
    inner: SyncSender<AudioCommand>,
    debug: Arc<TimelineAudioDebug>,
}

impl AudioCommandSender {
    /// Enqueues a command; returns the command back on overflow.
    pub fn send(&self, cmd: AudioCommand) -> Result<(), AudioCommand> {
        match self.inner.try_send(cmd) {
            Ok(()) => Ok(()),
            Err(TrySendError::Full(cmd)) => {
                self.debug.record_dropped_command();
                Err(cmd)
            }
            Err(TrySendError::Disconnected(cmd)) => Err(cmd),
        }
    }
}

/// Owning queue endpoints for tests and engine wiring.
#[derive(Debug)]
pub struct AudioCommandQueue {
    /// Producer handle.
    pub sender: AudioCommandSender,
    /// Consumer handle.
    pub receiver: std::sync::mpsc::Receiver<AudioCommand>,
}

impl AudioCommandQueue {
    /// Builds a bounded queue with design capacity.
    pub fn new(debug: Arc<TimelineAudioDebug>) -> Self {
        let (tx, rx) = sync_channel(COMMAND_QUEUE_CAP);
        Self {
            sender: AudioCommandSender {
                inner: tx,
                debug: Arc::clone(&debug),
            },
            receiver: rx,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{AudioTimestamp, BusId, VoiceId, VoicePriority};

    /// TC-IR-4.7.3.4 — filling the command queue makes `send` fail and increments drops.
    #[test]
    fn tc_ir_4_7_3_4_command_queue_full() {
        let debug = Arc::new(TimelineAudioDebug::new());
        let queue = AudioCommandQueue::new(Arc::clone(&debug));
        for i in 0..COMMAND_QUEUE_CAP {
            queue
                .sender
                .send(AudioCommand::SetBusParam {
                    bus_id: BusId(i as u16),
                    param: crate::BusParam::Gain,
                    value: 1.0,
                })
                .expect("enqueue");
        }
        let err = queue
            .sender
            .send(AudioCommand::Play {
                voice_id: VoiceId(1),
                clip: crate::AssetHandle::new(1),
                bus: BusId::SFX,
                priority: VoicePriority::Normal,
                timestamp: AudioTimestamp::Immediate,
            })
            .expect_err("overflow");
        assert!(matches!(
            err,
            AudioCommand::Play {
                voice_id: VoiceId(1),
                ..
            }
        ));
        assert_eq!(
            debug
                .dropped_commands
                .load(std::sync::atomic::Ordering::Relaxed),
            1
        );
    }

    /// TC-IR-4.7.N.1 — 4097th send fails and increments `dropped_commands`.
    #[test]
    fn tc_ir_4_7_n_1_command_queue_full_fallback() {
        let debug = Arc::new(TimelineAudioDebug::new());
        let queue = AudioCommandQueue::new(Arc::clone(&debug));
        for i in 0..COMMAND_QUEUE_CAP {
            assert!(queue
                .sender
                .send(AudioCommand::Prefetch {
                    clip: crate::AssetHandle::new(i as u32),
                })
                .is_ok());
        }
        assert!(queue
            .sender
            .send(AudioCommand::Prefetch {
                clip: crate::AssetHandle::new(9999),
            })
            .is_err());
        assert_eq!(
            debug
                .dropped_commands
                .load(std::sync::atomic::Ordering::Relaxed),
            1
        );
    }
}
