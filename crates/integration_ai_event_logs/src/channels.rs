//! Bounded MPSC-style threshold delivery (IR-2.2.4, FM-5, FM-6).

use crossbeam_channel::{Receiver, Sender, TrySendError};

use crate::debug_flags::EventLogDebugFlags;
use crate::threshold::ThresholdFired;

/// Documented per-entity channel capacity from the integration design.
pub const THRESHOLD_CHANNEL_CAP: usize = 64;

/// Thin wrapper around a bounded crossbeam channel (MPSC semantics).
///
/// Owns both ends for tests and single-threaded harnesses; production wiring should pass the
/// [`Sender`] to producers and the [`Receiver`] to consumers on their respective threads per
/// `docs/design/constraints.md` channel rules.
#[derive(Debug)]
pub struct ThresholdChannel {
    tx: Sender<ThresholdFired>,
    rx: Receiver<ThresholdFired>,
}

impl ThresholdChannel {
    /// Opens a bounded channel with [`THRESHOLD_CHANNEL_CAP`] slots.
    pub fn new() -> Self {
        let (tx, rx) = crossbeam_channel::bounded(THRESHOLD_CHANNEL_CAP);
        Self { tx, rx }
    }

    /// Non-blocking send. On success returns `Ok(())`; on overflow or disconnect returns
    /// `Err(evt)` with the unconsumed event (FM-6).
    pub fn try_send(&self, evt: ThresholdFired) -> Result<(), ThresholdFired> {
        match self.tx.try_send(evt) {
            Ok(()) => Ok(()),
            Err(TrySendError::Full(v)) | Err(TrySendError::Disconnected(v)) => Err(v),
        }
    }

    /// Drains every pending event (FM-5 unconsumed drain).
    pub fn drain_all(&self) -> Vec<ThresholdFired> {
        let mut out = Vec::new();
        while let Ok(v) = self.rx.try_recv() {
            out.push(v);
        }
        out
    }

    /// Like [`Self::drain_all`], but records a trace signal when drains happen under
    /// [`EventLogDebugFlags::trace_thresholds`].
    pub fn drain_all_traced(&self, flags: &mut EventLogDebugFlags) -> Vec<ThresholdFired> {
        let out = self.drain_all();
        if flags.trace_thresholds && !out.is_empty() {
            flags.threshold_nonempty_drains =
                flags.threshold_nonempty_drains.saturating_add(1);
        }
        out
    }
}

impl Default for ThresholdChannel {
    fn default() -> Self {
        Self::new()
    }
}
