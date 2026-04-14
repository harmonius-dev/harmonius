//! Frame-in-flight slot indexing for ring-buffered per-frame resources (R-2.10.9).

use std::num::NonZeroU32;

/// Maps monotonically increasing frame indices to a small ring of CPU/GPU slots.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FrameSlotRing {
    frames_in_flight: NonZeroU32,
}

impl FrameSlotRing {
    /// Creates a ring with `frames_in_flight` slots.
    #[must_use]
    pub fn new(frames_in_flight: NonZeroU32) -> Self {
        Self { frames_in_flight }
    }

    /// Slot index in `0..frames_in_flight` for this absolute frame counter.
    #[must_use]
    pub fn slot(&self, frame_index: u64) -> u32 {
        let n = u64::from(self.frames_in_flight.get());
        (frame_index % n) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::FrameSlotRing;
    use std::num::NonZeroU32;

    /// TC-2.10.9.1 — three frames in flight; frame 3 reuses slot 0.
    #[test]
    fn test_ring_buffer_3_frames() {
        let ring = FrameSlotRing::new(NonZeroU32::new(3).expect("three"));
        assert_eq!(ring.slot(0), 0);
        assert_eq!(ring.slot(1), 1);
        assert_eq!(ring.slot(2), 2);
        assert_eq!(ring.slot(3), 0);
    }
}
