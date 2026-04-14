//! Worker-side drain and assembly for GPU profiling payloads.

use crate::drop_queue::{GpuTimestampQueue, GpuTimestampSender};
use crate::gpu_profiler::{GpuFrameCapture, ResolvedTimestamps};

/// Worker-side consumer that drains GPU timestamp messages once per frame.
#[derive(Debug)]
pub struct FrameCollector {
    /// Monotonic frame counter owned by the worker assembling captures.
    pub frame_number: u64,
    queue: GpuTimestampQueue<ResolvedTimestamps>,
}

impl FrameCollector {
    /// Creates a collector with a bounded `crossbeam_channel` queue of `capacity` slots.
    ///
    /// The integration design uses `capacity == 4` for the profiler ↔ rendering handoff.
    #[must_use]
    pub fn new(frame_number: u64, capacity: usize) -> Self {
        Self {
            frame_number,
            queue: GpuTimestampQueue::new(capacity),
        }
    }

    /// Handle used from the render thread to enqueue GPU payloads.
    #[must_use]
    pub fn sender(&self) -> GpuTimestampSender<ResolvedTimestamps> {
        self.queue.sender()
    }

    /// Drains the mailbox and returns the most recently queued message, if any.
    pub fn drain_gpu(&mut self) -> Option<ResolvedTimestamps> {
        self.queue.drain_take_last()
    }

    /// Assembles the published capture from a drained GPU message.
    #[must_use]
    pub fn assemble_gpu(&self, gpu: ResolvedTimestamps) -> GpuFrameCapture {
        GpuFrameCapture {
            frame_number: gpu.frame_number,
            stats: gpu.stats,
            timings: gpu.timings,
            per_view: gpu.per_view,
        }
    }
}
