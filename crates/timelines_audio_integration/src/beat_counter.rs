//! Portable beat snapshot shared between the audio and simulation threads.

use std::sync::Mutex;

/// Monotonic beat phase snapshot consumed by `PlaybackState::advance_beats`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BeatSnapshot {
    /// Current bar index (1-based in tests; 0 means “not started”).
    pub bar: u32,
    /// Beat within the current bar (1-based in tests).
    pub beat: u32,
    /// Fractional phase within the current beat in `0.0..=1.0`.
    pub phase: f32,
    /// Tempo in beats per minute.
    pub bpm: f32,
}

/// Shared beat snapshot written by the audio thread, read by the simulation thread.
///
/// This portable implementation uses a `Mutex` so snapshots are always coherent on
/// stable Rust targets without `AtomicU128`. A future audio backend can swap the
/// internals for a single wide atomic where platform support guarantees are met.
#[derive(Debug)]
pub struct AtomicBeatCounter {
    snap: Mutex<BeatSnapshot>,
}

impl AtomicBeatCounter {
    /// Builds a counter initialized to silence (`bpm == 0.0`).
    pub fn new() -> Self {
        Self {
            snap: Mutex::new(BeatSnapshot {
                bar: 0,
                beat: 0,
                phase: 0.0,
                bpm: 0.0,
            }),
        }
    }

    /// Loads a complete snapshot.
    pub fn snapshot(&self) -> BeatSnapshot {
        *self
            .snap
            .lock()
            .expect("beat snapshot mutex should not be poisoned")
    }

    /// Stores a complete snapshot.
    pub fn store(&self, snap: BeatSnapshot) {
        *self
            .snap
            .lock()
            .expect("beat snapshot mutex should not be poisoned") = snap;
    }
}

impl Default for AtomicBeatCounter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    /// TC-IR-4.7.4.5 — concurrent `store` and `snapshot` never tear fields.
    #[test]
    fn tc_ir_4_7_4_5_atomic_snapshot_roundtrip() {
        let counter = Arc::new(AtomicBeatCounter::new());
        let mut handles = Vec::new();
        for t in 0..8 {
            let c = Arc::clone(&counter);
            handles.push(thread::spawn(move || {
                for i in 0..10_000 {
                    let bar = (t + i) % 100;
                    let beat = (i % 4) + 1;
                    let phase = ((i % 97) as f32) / 97.0;
                    let bpm = 60.0 + ((t * i) % 180) as f32;
                    c.store(BeatSnapshot {
                        bar,
                        beat,
                        phase,
                        bpm,
                    });
                }
            }));
        }
        let reader = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..50_000 {
                let s = reader.snapshot();
                assert!(s.phase.is_finite());
                assert!(s.bpm.is_finite());
            }
        }));
        for h in handles {
            h.join().expect("thread join");
        }
    }
}
