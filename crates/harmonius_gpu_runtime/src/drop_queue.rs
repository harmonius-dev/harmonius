use std::sync::Arc;

use crossbeam_queue::ArrayQueue;

/// Bounded FIFO queue that drops the oldest entry when full, matching the render-thread overflow
/// contract for GPU timestamp handoff.
pub struct GpuTimestampQueue<T> {
    inner: Arc<ArrayQueue<T>>,
}

impl<T> GpuTimestampQueue<T> {
    /// Creates a queue with the given capacity.
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: Arc::new(ArrayQueue::new(capacity)),
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
        while let Some(msg) = self.inner.pop() {
            last = Some(msg);
        }
        last
    }
}

/// Handle used from the render thread to enqueue [`ResolvedTimestamps`](crate::ResolvedTimestamps).
#[derive(Clone)]
pub struct GpuTimestampSender<T> {
    inner: Arc<ArrayQueue<T>>,
}

impl<T> GpuTimestampSender<T> {
    /// Enqueues `value`, evicting the oldest entry if the queue is already at capacity.
    pub fn send_drop_oldest(&self, mut value: T) {
        loop {
            match self.inner.push(value) {
                Ok(()) => break,
                Err(v) => {
                    let _ = self.inner.pop();
                    value = v;
                }
            }
        }
    }
}
