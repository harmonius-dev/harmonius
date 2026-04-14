use std::fmt;
use std::sync::Arc;

use crossbeam_channel::{bounded, Receiver, Sender, TrySendError};
use tracing::warn;

struct Inner<T> {
    receiver: Receiver<T>,
    sender: Sender<T>,
}

/// Bounded FIFO queue backed by `crossbeam_channel` that drops the oldest entry when full,
/// matching the render-thread overflow contract for GPU timestamp handoff.
pub struct GpuTimestampQueue<T> {
    inner: Arc<Inner<T>>,
}

impl<T: Send + 'static> GpuTimestampQueue<T> {
    /// Creates a queue with the given capacity (minimum 1).
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        let capacity = capacity.max(1);
        let (sender, receiver) = bounded(capacity);
        Self {
            inner: Arc::new(Inner { receiver, sender }),
        }
    }

    /// Sender handle that can be shared with the render thread.
    #[must_use]
    pub fn sender(&self) -> GpuTimestampSender<T> {
        GpuTimestampSender {
            inner: Arc::clone(&self.inner),
        }
    }

    /// Drops all but the last message in arrival order (matches `FrameCollector::drain_gpu`).
    pub fn drain_take_last(&self) -> Option<T> {
        let mut last = None;
        while let Ok(msg) = self.inner.receiver.try_recv() {
            last = Some(msg);
        }
        last
    }
}

impl<T> fmt::Debug for GpuTimestampQueue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GpuTimestampQueue").finish_non_exhaustive()
    }
}

/// Handle used from the render thread to enqueue [`ResolvedTimestamps`](crate::ResolvedTimestamps).
#[derive(Clone)]
pub struct GpuTimestampSender<T> {
    inner: Arc<Inner<T>>,
}

impl<T: Send + 'static> GpuTimestampSender<T> {
    /// Enqueues `value`, evicting the oldest entry if the queue is already at capacity.
    pub fn send_drop_oldest(&self, mut value: T) {
        loop {
            match self.inner.sender.try_send(value) {
                Ok(()) => break,
                Err(TrySendError::Full(v)) => {
                    if self.inner.receiver.try_recv().is_ok() {
                        warn!(
                            target: "harmonius_gpu_runtime::gpu_timestamp_queue",
                            "dropped oldest ResolvedTimestamps message (bounded channel full; worker \
                             stalled)"
                        );
                    }
                    value = v;
                }
                Err(TrySendError::Disconnected(v)) => {
                    warn!(
                        target: "harmonius_gpu_runtime::gpu_timestamp_queue",
                        "ResolvedTimestamps send failed (receiver disconnected)"
                    );
                    let _ = v;
                    break;
                }
            }
        }
    }
}

impl<T> fmt::Debug for GpuTimestampSender<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GpuTimestampSender").finish_non_exhaustive()
    }
}
