use crate::phase::Phase;

/// One spike record emitted when a phase exceeds its budget.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpikeEntry {
    /// Phase that exceeded budget.
    pub phase: Phase,
    /// Observed duration in milliseconds.
    pub duration_ms: f64,
    /// Registered budget in milliseconds.
    pub budget_ms: f64,
    /// Frame where the spike was detected.
    pub frame_number: u64,
}

/// Fixed-capacity ring storing the most recent spike entries.
#[derive(Debug)]
pub struct SpikeRing {
    slots: [SpikeEntry; 32],
    write: u16,
    full: bool,
    scratch: [SpikeEntry; 32],
}

/// Borrowed view of spike entries for a single frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpikeRingView<'a> {
    /// Contiguous spike entries in insertion order (oldest first).
    pub entries: &'a [SpikeEntry],
    /// Number of valid entries in `entries`.
    pub count: u16,
}

impl SpikeRing {
    /// Builds an empty spike ring.
    #[must_use]
    pub fn new() -> Self {
        Self {
            slots: [SpikeEntry {
                phase: Phase::Input,
                duration_ms: 0.0,
                budget_ms: 0.0,
                frame_number: 0,
            }; 32],
            write: 0,
            full: false,
            scratch: [SpikeEntry {
                phase: Phase::Input,
                duration_ms: 0.0,
                budget_ms: 0.0,
                frame_number: 0,
            }; 32],
        }
    }

    /// Pushes a spike entry, overwriting the oldest slot when full.
    pub fn push(&mut self, entry: SpikeEntry) {
        self.slots[usize::from(self.write)] = entry;
        self.write = ((usize::from(self.write) + 1) % 32) as u16;
        if self.write == 0 {
            self.full = true;
        }
    }

    /// Copies chronological entries into scratch storage and clears the ring.
    pub fn drain(&mut self) -> SpikeRingView<'_> {
        let count = if self.full {
            32usize
        } else {
            usize::from(self.write)
        };
        if count == 0 {
            return SpikeRingView {
                entries: &[],
                count: 0,
            };
        }
        let start = if self.full {
            usize::from(self.write)
        } else {
            0
        };
        for i in 0..count {
            self.scratch[i] = self.slots[(start + i) % 32];
        }
        self.write = 0;
        self.full = false;
        SpikeRingView {
            entries: &self.scratch[..count],
            count: count as u16,
        }
    }
}

impl Default for SpikeRing {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(frame: u64, duration: f64) -> SpikeEntry {
        SpikeEntry {
            phase: Phase::PhysicsStep,
            duration_ms: duration,
            budget_ms: 4.0,
            frame_number: frame,
        }
    }

    #[test]
    fn tc_ir_5_6_3_u1_push_wraps_at_capacity_32() {
        let mut ring = SpikeRing::new();
        for i in 0..40 {
            ring.push(entry(i, i as f64));
        }
        let view = ring.drain();
        assert_eq!(usize::from(view.count), 32);
        for (idx, e) in view.entries.iter().enumerate() {
            let expected = 8 + idx;
            assert_eq!(e.frame_number, expected as u64);
            assert_eq!(e.duration_ms, expected as f64);
        }
    }
}
