//! Bounded MPSC channel for high-frequency raw camera deltas.

use crossbeam_channel::{Receiver, Sender, TrySendError};
use glam::Vec2;

/// Owned raw delta crossing the main → worker boundary.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RawCameraDelta {
    /// Mouse or synthesized 2D delta.
    pub delta: Vec2,
}

/// Bounded MPSC bus (`capacity` as in design: 256 default).
#[derive(Debug)]
pub struct RawCameraInputBus {
    sender: Sender<RawCameraDelta>,
    receiver: Receiver<RawCameraDelta>,
    capacity: usize,
}

impl RawCameraInputBus {
    /// Create a bounded channel with the given capacity.
    pub fn new(capacity: usize) -> Self {
        let (sender, receiver) = crossbeam_channel::bounded(capacity);
        Self {
            sender,
            receiver,
            capacity,
        }
    }

    /// Sender handle for producers.
    pub fn sender(&self) -> Sender<RawCameraDelta> {
        self.sender.clone()
    }

    /// Consumer handle for integration workers / tests.
    pub fn receiver(&self) -> &Receiver<RawCameraDelta> {
        &self.receiver
    }

    /// Capacity configured at construction.
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

/// Try-send with overflow policy from the integration design.
///
/// On `Full`, drops the oldest message then retries once; counts each dropped message.
pub fn push_raw_camera_delta(
    sender: &Sender<RawCameraDelta>,
    receiver: &Receiver<RawCameraDelta>,
    msg: RawCameraDelta,
    dropped: &mut u64,
) {
    match sender.try_send(msg) {
        Ok(()) => {}
        Err(TrySendError::Full(m)) => {
            let _ = receiver.try_recv();
            *dropped += 1;
            if sender.try_send(m).is_err() {
                *dropped += 1;
            }
        }
        Err(TrySendError::Disconnected(_)) => {}
    }
}
