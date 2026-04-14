//! Bounded command queue with priority-aware overflow and underrun accounting.

use std::collections::VecDeque;

use crate::commands::AudioCommand;
use crate::commands::VoicePriority;

fn command_priority(cmd: &AudioCommand) -> VoicePriority {
    match cmd {
        AudioCommand::Play { priority, .. } => *priority,
        AudioCommand::SetParam { .. } => VoicePriority::Medium,
        AudioCommand::Stop { .. } => VoicePriority::High,
        AudioCommand::ActivateReverb { .. } => VoicePriority::Medium,
        AudioCommand::DeactivateReverb { .. } => VoicePriority::Medium,
    }
}

/// Fixed-capacity FIFO queue with lowest-priority eviction when full (design: 1024).
#[derive(Debug)]
pub struct BoundedAudioCommandQueue {
    inner: VecDeque<AudioCommand>,
    capacity: usize,
    underrun_drops: u64,
}

impl BoundedAudioCommandQueue {
    /// Creates a queue with the given capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: VecDeque::with_capacity(capacity.max(1)),
            capacity: capacity.max(1),
            underrun_drops: 0,
        }
    }

    /// Underrun / overload counter incremented when the consumer drops oldest commands.
    pub fn underrun_drops(&self) -> u64 {
        self.underrun_drops
    }

    /// Current queued command count.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` when no commands are queued.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Enqueues a command, evicting the lowest-priority queued command when at capacity.
    pub fn send(&mut self, cmd: AudioCommand) -> Result<(), AudioCommand> {
        if self.inner.len() < self.capacity {
            self.inner.push_back(cmd);
            return Ok(());
        }
        let incoming = command_priority(&cmd);
        let Some(front) = self.inner.front() else {
            return Err(cmd);
        };
        let mut lowest_idx = 0usize;
        let mut lowest_pri = command_priority(front);
        for (i, existing) in self.inner.iter().enumerate().skip(1) {
            let p = command_priority(existing);
            if p < lowest_pri {
                lowest_pri = p;
                lowest_idx = i;
            }
        }
        if incoming <= lowest_pri {
            return Err(cmd);
        }
        self.inner.remove(lowest_idx);
        self.inner.push_back(cmd);
        Ok(())
    }

    /// Drains up to `max_take` commands for the audio callback, counting overload when more remain.
    pub fn drain_processing(&mut self, max_take: usize) -> Vec<AudioCommand> {
        if self.inner.len() > max_take {
            let drop = self.inner.len() - max_take;
            self.underrun_drops = self.underrun_drops.saturating_add(drop as u64);
            for _ in 0..drop {
                self.inner.pop_front();
            }
        }
        let mut out = Vec::new();
        while out.len() < max_take {
            let Some(cmd) = self.inner.pop_front() else {
                break;
            };
            out.push(cmd);
        }
        out
    }

    /// Drains the entire queue (test helper).
    pub fn drain_all(&mut self) -> Vec<AudioCommand> {
        self.inner.drain(..).collect()
    }
}
