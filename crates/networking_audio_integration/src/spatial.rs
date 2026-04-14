//! Game-thread → audio-thread spatial commands with bounded backpressure.

use crate::connection::ConnectionId;
use crate::voice_packet::VoiceChannelId;
use std::collections::VecDeque;

/// Minimal 3D vector for replicated transforms (design uses `Vec3` world space).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    /// X component in world metres.
    pub x: f32,
    /// Y component in world metres.
    pub y: f32,
    /// Z component in world metres.
    pub z: f32,
}

/// Spatialization parameters for a remote speaker.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpatialParams {
    /// World position.
    pub position: Vec3,
    /// Forward unit vector.
    pub forward: Vec3,
    /// Up unit vector.
    pub up: Vec3,
    /// Linear velocity in metres per second.
    pub velocity: Vec3,
}

/// Commands sent from game workers to the audio thread.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VoiceSpatialCommand {
    /// Update spatial position for a remote speaker.
    UpdatePosition {
        /// Connection that moved.
        connection: ConnectionId,
        /// New spatial parameters.
        params: SpatialParams,
    },
    /// Remote speaker joined a voice channel.
    SpeakerJoined {
        /// Connection that joined.
        connection: ConnectionId,
        /// Channel joined.
        channel: VoiceChannelId,
    },
    /// Remote speaker left a voice channel.
    SpeakerLeft {
        /// Connection that left.
        connection: ConnectionId,
        /// Channel left.
        channel: VoiceChannelId,
    },
}

/// Bounded queue (512) with oldest-drop overflow policy for spatial updates.
///
/// Intended for single-threaded staging on the audio thread (or behind a mutex). Multiple
/// concurrent producers require external synchronization or a bounded MPSC queue with the same
/// overflow policy when the transport stack wires game threads to audio.
#[derive(Debug)]
pub struct VoiceSpatialQueue {
    inner: VecDeque<VoiceSpatialCommand>,
    capacity: usize,
}

impl VoiceSpatialQueue {
    /// Builds an empty queue with the design capacity of 512.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: VecDeque::new(),
            capacity: 512,
        }
    }

    /// Pushes `cmd`, dropping the oldest entry when full.
    pub fn push_drop_oldest(&mut self, cmd: VoiceSpatialCommand) {
        if self.inner.len() == self.capacity {
            self.inner.pop_front();
        }
        self.inner.push_back(cmd);
    }

    /// Drains up to `max` commands for the audio thread.
    pub fn drain(&mut self, max: usize) -> impl Iterator<Item = VoiceSpatialCommand> + '_ {
        self.inner.drain(..max.min(self.inner.len()))
    }

    /// Current pending command count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` when no spatial commands are pending.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl Default for VoiceSpatialQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_params() -> SpatialParams {
        SpatialParams {
            position: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            forward: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            up: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            velocity: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }

    /// TC-IR-4.3.4.3 — 513 pushes leave at most 512 pending (oldest dropped).
    #[test]
    fn tc_ir_4_3_4_3_spatial_queue_overflow() {
        let mut q = VoiceSpatialQueue::new();
        for i in 0..513 {
            q.push_drop_oldest(VoiceSpatialCommand::UpdatePosition {
                connection: ConnectionId((i % 65536) as u16),
                params: dummy_params(),
            });
        }
        assert_eq!(q.len(), 512);
    }
}
