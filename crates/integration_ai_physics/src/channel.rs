//! CH-26 nav query channel (`cap = 256`, `DropOldest`).
//!
//! This module is a **single-threaded surrogate** for the runtime crossbeam MPSC channel described
//! in the integration design.

use std::collections::VecDeque;

use crate::jump_arc::{JumpArcQuery, JumpArcResult, JumpArcScene};
use crate::metrics::FallbackMetrics;
use crate::walkability::{WalkabilityQuery, WalkabilityResult, WalkabilityScene};

/// Nav requests multiplexed on CH-26 in the full runtime.
#[derive(Clone, Debug, PartialEq)]
pub enum NavRequest {
    /// Walkability query envelope.
    Walkability(WalkabilityQuery),
    /// Jump arc query envelope.
    JumpArc(JumpArcQuery),
}

/// Replies paired with processed requests (test harness only).
#[derive(Clone, Debug, PartialEq)]
pub enum NavReply {
    /// Walkability resolution payload.
    Walkability(WalkabilityResult),
    /// Jump arc trace payload.
    JumpArc(JumpArcResult),
}

/// Bounded channel surrogate with `DropOldest` overflow policy (`FM-3`).
#[derive(Clone, Debug)]
pub struct AiNavQueryChannel {
    cap: usize,
    queue: VecDeque<NavRequest>,
}

impl AiNavQueryChannel {
    /// CH-26 capacity from the integration design.
    pub const CH26_CAP: usize = 256;

    /// Opens a new channel at the design capacity.
    #[must_use]
    pub fn new() -> Self {
        Self {
            cap: Self::CH26_CAP,
            queue: VecDeque::new(),
        }
    }

    /// Enqueues `request`, dropping oldest entries until the backlog fits (`FM-3`).
    pub fn send(&mut self, request: NavRequest, metrics: &mut FallbackMetrics) {
        while self.queue.len() >= self.cap {
            self.queue.pop_front();
            metrics.fm3_channel_drop_oldest += 1;
        }
        self.queue.push_back(request);
    }

    /// Pops the next request if any are pending.
    #[must_use]
    pub fn recv(&mut self) -> Option<NavRequest> {
        self.queue.pop_front()
    }

    /// Returns the number of queued requests (test helper).
    #[must_use]
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Returns `true` when no requests are queued.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn prepend_requests(&mut self, mut requests: Vec<NavRequest>) {
        while let Some(req) = requests.pop() {
            self.queue.push_front(req);
        }
    }
}

impl Default for AiNavQueryChannel {
    fn default() -> Self {
        Self::new()
    }
}

/// Drains `channel` and resolves walkability against `scene`.
#[must_use]
pub fn drain_walkability(
    channel: &mut AiNavQueryChannel,
    scene: &WalkabilityScene,
    metrics: &mut FallbackMetrics,
) -> Vec<WalkabilityResult> {
    let mut pending = Vec::new();
    while let Some(req) = channel.recv() {
        pending.push(req);
    }
    let mut out = Vec::new();
    let mut restore = Vec::new();
    for req in pending {
        match req {
            NavRequest::Walkability(q) => out.push(scene.resolve(q, metrics)),
            other => restore.push(other),
        }
    }
    channel.prepend_requests(restore);
    out
}

/// Drains `channel` and resolves jump arcs against `scene`.
#[must_use]
pub fn drain_jump_arcs(
    channel: &mut AiNavQueryChannel,
    scene: &JumpArcScene,
) -> Vec<JumpArcResult> {
    let mut pending = Vec::new();
    while let Some(req) = channel.recv() {
        pending.push(req);
    }
    let mut out = Vec::new();
    let mut restore = Vec::new();
    for req in pending {
        match req {
            NavRequest::JumpArc(q) => out.push(scene.trace(q)),
            other => restore.push(other),
        }
    }
    channel.prepend_requests(restore);
    out
}
