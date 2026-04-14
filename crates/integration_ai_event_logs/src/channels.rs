//! Bounded MPSC-style threshold delivery (IR-2.2.4, FM-5, FM-6).

use crossbeam_channel::{Receiver, Sender, TrySendError};

use crate::threshold::ThresholdFired;

/// Documented per-entity channel capacity from the integration design.
pub const THRESHOLD_CHANNEL_CAP: usize = 64;

/// Thin wrapper around a bounded crossbeam channel (MPSC semantics).
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

    /// Non-blocking send; returns dropped event count on overflow (FM-6).
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
}

impl Default for ThresholdChannel {
    fn default() -> Self {
        Self::new()
    }
}
