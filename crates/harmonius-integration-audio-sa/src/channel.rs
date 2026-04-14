//! Bounded MPSC channel pair sized for propagation updates.

use crossbeam_channel::{Receiver, Sender};

use crate::PropagationResult;

/// Buffer length from the integration design (256 entries).
pub const MPSC_CAPACITY: usize = 256;

/// Sender handle for propagation results (integration design name).
pub type AudioPropagationSender = Sender<PropagationResult>;

/// Receiver handle for propagation results (integration design name).
pub type AudioPropagationReceiver = Receiver<PropagationResult>;

/// Creates a bounded propagation channel pair.
#[must_use]
pub fn propagation_channel_pair() -> (AudioPropagationSender, AudioPropagationReceiver) {
    crossbeam_channel::bounded(MPSC_CAPACITY)
}
