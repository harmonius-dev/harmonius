//! Bounded MPSC channel pair sized for propagation updates.

use crossbeam_channel::{Receiver, Sender};

use crate::PropagationResult;

/// Buffer length from the integration design (256 entries).
pub const MPSC_CAPACITY: usize = 256;

/// Creates a bounded propagation channel pair.
#[must_use]
pub fn propagation_channel_pair() -> (Sender<PropagationResult>, Receiver<PropagationResult>) {
    crossbeam_channel::bounded(MPSC_CAPACITY)
}
