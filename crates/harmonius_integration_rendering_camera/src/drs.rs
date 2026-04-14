//! Dynamic resolution feedback queue and PI controller step.

use std::collections::VecDeque;
use std::sync::Mutex;

use crate::types::{DynamicResolutionState, Viewport};

/// Clamps a DRS scale factor into `[min_scale, 1.0]` with non-finite guards.
#[must_use]
pub fn clamp_drs_scale(scale: f32, min_scale: f32) -> f32 {
    let min_scale = min_scale.clamp(0.0, 1.0);
    if !scale.is_finite() || scale <= 0.0 {
        return min_scale;
    }
    scale.clamp(min_scale, 1.0)
}

/// Scales a viewport by a DRS factor, keeping at least one pixel per axis.
#[must_use]
pub fn scale_viewport(viewport: Viewport, scale: f32, min_scale: f32) -> Viewport {
    let s = clamp_drs_scale(scale, min_scale);
    Viewport {
        x: viewport.x,
        y: viewport.y,
        width: ((viewport.width as f32) * s).round().max(1.0) as u32,
        height: ((viewport.height as f32) * s).round().max(1.0) as u32,
    }
}

/// Bounded FIFO that drops the oldest entries when full (capacity four).
#[derive(Debug)]
pub struct DrsFeedbackQueue {
    inner: Mutex<VecDeque<DynamicResolutionState>>,
}

impl DrsFeedbackQueue {
    /// Harmonius integration default capacity (design).
    pub const CAPACITY: usize = 4;

    /// Creates an empty queue with the default Harmonius capacity.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(VecDeque::new()),
        }
    }

    /// Pushes a message, dropping oldest entries when at capacity.
    pub fn push(&self, message: DynamicResolutionState) {
        let mut guard = self.inner.lock().expect("DrsFeedbackQueue mutex poisoned");
        while guard.len() >= Self::CAPACITY {
            guard.pop_front();
        }
        guard.push_back(message);
    }

    /// Drains every pending message in FIFO order.
    #[must_use]
    pub fn drain(&self) -> Vec<DynamicResolutionState> {
        let mut guard = self.inner.lock().expect("DrsFeedbackQueue mutex poisoned");
        guard.drain(..).collect()
    }
}

impl Default for DrsFeedbackQueue {
    fn default() -> Self {
        Self::new()
    }
}

/// Single-step PI adjustment for DRS scale (design gains).
#[must_use]
pub fn drs_pi_step(scale: f32, frame_ms: f32, target_ms: f32, min_scale: f32) -> f32 {
    let err = frame_ms - target_ms;
    let p = 0.1 * err;
    let i = 0.02 * err;
    clamp_drs_scale(scale - p - i, min_scale)
}
