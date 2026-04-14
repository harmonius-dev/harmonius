//! Bounded audio→timeline beat event queue (design capacity: 256).

use std::sync::mpsc::{sync_channel, SyncSender, TrySendError};
use std::sync::Arc;

use crate::debug::TimelineAudioDebug;

const BEAT_QUEUE_CAP: usize = 256;

/// Beat boundary marker emitted by the audio thread.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BeatEvent {
    /// Quarter-note boundary tick.
    Beat,
    /// Bar boundary tick.
    Bar,
}

/// Producer handle for beat events.
#[derive(Clone, Debug)]
pub struct BeatEventSender {
    inner: SyncSender<BeatEvent>,
    debug: Arc<TimelineAudioDebug>,
}

impl BeatEventSender {
    /// Sends a beat marker; returns the event back on overflow.
    pub fn send(&self, event: BeatEvent) -> Result<(), BeatEvent> {
        match self.inner.try_send(event) {
            Ok(()) => Ok(()),
            Err(TrySendError::Full(ev)) => {
                self.debug.record_dropped_beat();
                Err(ev)
            }
            Err(TrySendError::Disconnected(ev)) => Err(ev),
        }
    }
}

/// Owning queue endpoints for tests and engine wiring.
#[derive(Debug)]
pub struct BeatEventQueue {
    /// Producer handle.
    pub sender: BeatEventSender,
    /// Consumer handle.
    pub receiver: std::sync::mpsc::Receiver<BeatEvent>,
}

impl BeatEventQueue {
    /// Builds a bounded queue with design capacity.
    pub fn new(debug: Arc<TimelineAudioDebug>) -> Self {
        let (tx, rx) = sync_channel(BEAT_QUEUE_CAP);
        Self {
            sender: BeatEventSender {
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
    use std::sync::atomic::Ordering;

    /// TC-IR-4.7.4.3 — beat MPSC overflow increments `dropped_beats`.
    #[test]
    fn tc_ir_4_7_4_3_beat_queue_full() {
        let debug = Arc::new(TimelineAudioDebug::new());
        let queue = BeatEventQueue::new(Arc::clone(&debug));
        for _ in 0..BEAT_QUEUE_CAP {
            queue.sender.send(BeatEvent::Beat).expect("enqueue");
        }
        assert!(queue.sender.send(BeatEvent::Bar).is_err());
        assert_eq!(debug.dropped_beats.load(Ordering::Relaxed), 1);
    }

    /// TC-IR-4.7.N.2 — 257th beat send fails while the atomic counter can still advance.
    #[test]
    fn tc_ir_4_7_n_2_beat_queue_full_fallback() {
        let debug = Arc::new(TimelineAudioDebug::new());
        let queue = BeatEventQueue::new(Arc::clone(&debug));
        let counter = Arc::new(crate::AtomicBeatCounter::new());
        for _ in 0..BEAT_QUEUE_CAP {
            assert!(queue.sender.send(BeatEvent::Beat).is_ok());
        }
        assert!(queue.sender.send(BeatEvent::Beat).is_err());
        counter.store(crate::BeatSnapshot {
            bar: 1,
            beat: 4,
            phase: 0.0,
            bpm: 120.0,
        });
        assert_eq!(counter.snapshot().bpm, 120.0);
        assert_eq!(debug.dropped_beats.load(Ordering::Relaxed), 1);
    }
}
