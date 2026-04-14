//! Runtime-toggleable bridge diagnostics.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Mutex;

use crate::SegmentId;

/// Runtime-toggleable debug counters for the timelines ↔ audio bridge.
#[derive(Debug)]
pub struct TimelineAudioDebug {
    /// Enables verbose capture paths (still cheap when disabled).
    pub enabled: AtomicBool,
    /// Counts dropped timeline→audio commands when the MPSC is full.
    pub dropped_commands: AtomicU64,
    /// Counts dropped beat events when the reverse MPSC is full.
    pub dropped_beats: AtomicU64,
    /// Last music segment transition target observed by the bridge.
    pub last_transition: Mutex<Option<SegmentId>>,
}

impl TimelineAudioDebug {
    /// Builds a disabled debug resource.
    pub fn new() -> Self {
        Self {
            enabled: AtomicBool::new(false),
            dropped_commands: AtomicU64::new(0),
            dropped_beats: AtomicU64::new(0),
            last_transition: Mutex::new(None),
        }
    }

    /// Increments dropped command counter when `enabled` is true (always counts drops).
    pub fn record_dropped_command(&self) {
        self.dropped_commands.fetch_add(1, Ordering::Relaxed);
    }

    /// Increments dropped beat counter when the beat MPSC overflows.
    pub fn record_dropped_beat(&self) {
        self.dropped_beats.fetch_add(1, Ordering::Relaxed);
    }

    /// Records the most recent music transition target for profiler overlays.
    pub fn record_transition(&self, target: SegmentId) {
        if let Ok(mut slot) = self.last_transition.lock() {
            *slot = Some(target);
        }
    }
}

impl Default for TimelineAudioDebug {
    fn default() -> Self {
        Self::new()
    }
}
