//! Bounded handoff for `RenderFrame` (IR-3.4.5 / failure N5).

use std::collections::VecDeque;
use std::sync::Mutex;

use crate::buffer::DebugDrawBuffer;

/// Frame snapshot crossing the worker → render handoff queue.
#[derive(Debug)]
pub struct RenderFrame {
    pub debug_lines: DebugDrawBuffer,
    pub interp_alpha: f32,
    pub frame_index: u64,
}

/// Triple-buffered queue (`capacity = 3`): evicts the oldest frame when full.
pub struct BoundedRenderFrameSender {
    cap: usize,
    queue: Mutex<VecDeque<RenderFrame>>,
}

impl BoundedRenderFrameSender {
    /// Creates an empty bounded queue matching the integration design (capacity 3).
    pub fn new(capacity: usize) -> Self {
        let cap = capacity.max(1);
        Self {
            cap,
            queue: Mutex::new(VecDeque::with_capacity(cap)),
        }
    }

    /// Enqueues `frame`, dropping the oldest queued frame when at capacity.
    pub fn try_send_drop_oldest(&self, frame: RenderFrame) {
        let mut q = self.queue.lock().unwrap();
        while q.len() >= self.cap {
            q.pop_front();
        }
        q.push_back(frame);
    }

    /// Drains all queued frames in FIFO order (for tests / render thread stub).
    pub fn drain(&self) -> Vec<RenderFrame> {
        let mut q = self.queue.lock().unwrap();
        q.drain(..).collect()
    }

    /// Queue capacity (for tests).
    pub fn capacity(&self) -> usize {
        self.cap
    }
}

impl Default for RenderFrame {
    fn default() -> Self {
        Self {
            debug_lines: DebugDrawBuffer::new(16),
            interp_alpha: 0.0,
            frame_index: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{DebugLine, LinearColor};
    use glam::Vec3;

    #[test]
    fn tc_ir_3_4_n5_mpsc_drops_oldest_keeps_newest() {
        let tx = BoundedRenderFrameSender::new(3);
        assert_eq!(tx.capacity(), 3);
        for i in 0..4u64 {
            let mut buf = DebugDrawBuffer::new(16);
            buf.push_line(
                100,
                DebugLine {
                    start: Vec3::ZERO,
                    end: Vec3::new(i as f32, 0.0, 0.0),
                    color: LinearColor::new(1.0, 1.0, 1.0, 1.0),
                },
            );
            tx.try_send_drop_oldest(RenderFrame {
                debug_lines: buf,
                interp_alpha: 0.0,
                frame_index: i,
            });
        }
        let frames: Vec<u64> = tx.drain().into_iter().map(|f| f.frame_index).collect();
        assert_eq!(frames, vec![1, 2, 3]);
    }
}
