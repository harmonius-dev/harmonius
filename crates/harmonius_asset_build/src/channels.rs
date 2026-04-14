//! Bounded MPSC channels for build ↔ packaging integration (design buffer sizes).

use crossbeam_channel::{Receiver, Sender};

/// Buffer sizes from the integration design (`build_requests` / `cook_completions` / `bundle_completions`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChannelSizes {
    pub build_requests: usize,
    pub cook_completions: usize,
    pub bundle_completions: usize,
}

impl Default for ChannelSizes {
    fn default() -> Self {
        Self {
            build_requests: 64,
            cook_completions: 256,
            bundle_completions: 64,
        }
    }
}

/// `(sender, receiver)` for build requests (buffer 64).
pub fn build_requests<T: Send + 'static>() -> (Sender<T>, Receiver<T>) {
    crossbeam_channel::bounded(ChannelSizes::default().build_requests)
}

/// `(sender, receiver)` for cook completions (buffer 256).
pub fn cook_completions<T: Send + 'static>() -> (Sender<T>, Receiver<T>) {
    crossbeam_channel::bounded(ChannelSizes::default().cook_completions)
}

/// `(sender, receiver)` for bundle completions (buffer 64).
pub fn bundle_completions<T: Send + 'static>() -> (Sender<T>, Receiver<T>) {
    crossbeam_channel::bounded(ChannelSizes::default().bundle_completions)
}
