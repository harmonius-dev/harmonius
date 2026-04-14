//! Bounded command queue with priority-aware overflow and underrun accounting.
//!
//! Uses a [`std::sync::Mutex`] around a [`VecDeque`] so multiple ECS observers can enqueue from
//! different threads (MPSC) while preserving the design’s lowest-priority eviction scan. A future
//! path can swap the inner buffer for `crossbeam_queue::ArrayQueue` once overflow policy maps to
//! pure ring semantics.

use std::collections::VecDeque;
use std::sync::Mutex;

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

#[derive(Debug)]
struct QueueInner {
    deque: VecDeque<AudioCommand>,
    underrun_drops: u64,
}

fn lock_queue<'a>(m: &'a Mutex<QueueInner>) -> std::sync::MutexGuard<'a, QueueInner> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}

/// Fixed-capacity FIFO queue with lowest-priority eviction when full (design: 1024).
#[derive(Debug)]
pub struct BoundedAudioCommandQueue {
    inner: Mutex<QueueInner>,
    capacity: usize,
}

impl BoundedAudioCommandQueue {
    /// Creates a queue with the given capacity.
    pub fn new(capacity: usize) -> Self {
        let cap = capacity.max(1);
        Self {
            inner: Mutex::new(QueueInner {
                deque: VecDeque::with_capacity(cap),
                underrun_drops: 0,
            }),
            capacity: cap,
        }
    }

    /// Underrun / overload counter incremented when the consumer drops oldest commands.
    pub fn underrun_drops(&self) -> u64 {
        lock_queue(&self.inner).underrun_drops
    }

    /// Current queued command count.
    pub fn len(&self) -> usize {
        lock_queue(&self.inner).deque.len()
    }

    /// Returns `true` when no commands are queued.
    pub fn is_empty(&self) -> bool {
        lock_queue(&self.inner).deque.is_empty()
    }

    /// Enqueues a command, evicting the lowest-priority queued command when at capacity.
    pub fn send(&self, cmd: AudioCommand) -> Result<(), AudioCommand> {
        let mut g = lock_queue(&self.inner);
        if g.deque.len() < self.capacity {
            g.deque.push_back(cmd);
            return Ok(());
        }
        let incoming = command_priority(&cmd);
        let Some(front) = g.deque.front() else {
            return Err(cmd);
        };
        let mut lowest_idx = 0usize;
        let mut lowest_pri = command_priority(front);
        for (i, existing) in g.deque.iter().enumerate().skip(1) {
            let p = command_priority(existing);
            if p < lowest_pri {
                lowest_pri = p;
                lowest_idx = i;
            }
        }
        if incoming <= lowest_pri {
            return Err(cmd);
        }
        g.deque.remove(lowest_idx);
        g.deque.push_back(cmd);
        Ok(())
    }

    /// Drains up to `max_take` commands for the audio callback, counting overload when more remain.
    pub fn drain_processing(&self, max_take: usize) -> Vec<AudioCommand> {
        let mut g = lock_queue(&self.inner);
        if g.deque.len() > max_take {
            let drop = g.deque.len() - max_take;
            g.underrun_drops = g.underrun_drops.saturating_add(drop as u64);
            for _ in 0..drop {
                g.deque.pop_front();
            }
        }
        let mut out = Vec::new();
        while out.len() < max_take {
            let Some(cmd) = g.deque.pop_front() else {
                break;
            };
            out.push(cmd);
        }
        out
    }

    /// Drains the entire queue (test helper).
    pub fn drain_all(&self) -> Vec<AudioCommand> {
        let mut g = lock_queue(&self.inner);
        g.deque.drain(..).collect()
    }
}
