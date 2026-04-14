//! Leak detection by snapshot diff (design: `LeakDetector`).

use crate::memory::{LeakReport, MemAllocTracker, MemorySnapshot};

/// Compares baseline vs current allocator snapshots.
#[derive(Debug)]
pub struct LeakDetector<'a> {
    tracker: &'a MemAllocTracker,
    baseline: Option<MemorySnapshot>,
}

impl<'a> LeakDetector<'a> {
    /// Builds a detector bound to `tracker`.
    #[must_use]
    pub fn new(tracker: &'a MemAllocTracker) -> Self {
        Self {
            tracker,
            baseline: None,
        }
    }

    /// Stores the baseline snapshot.
    pub fn take_baseline(&mut self) {
        self.baseline = Some(self.tracker.take_snapshot());
    }

    /// Diffs `baseline` vs a fresh snapshot from `tracker`.
    #[must_use]
    pub fn check(&self) -> LeakReport {
        let later = self.tracker.take_snapshot();
        let base = self
            .baseline
            .as_ref()
            .expect("LeakDetector::take_baseline must run before check");
        base.diff(&later)
    }

    /// Returns `Ok(())` when no leaks are detected.
    pub fn assert_no_leaks(&self) -> Result<(), LeakReport> {
        let report = self.check();
        if report.total_leaked_count == 0 {
            Ok(())
        } else {
            Err(report)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-15.5.4.1 — ten leaked allocations.
    #[test]
    fn tc_15_5_4_1_leak_detection() {
        let t = MemAllocTracker::new();
        let mut det = LeakDetector::new(&t);
        det.take_baseline();
        for i in 0..10_u32 {
            t.record_alloc(10_000 + u64::from(i), 32, 0, 0);
        }
        let r = det.check();
        assert_eq!(r.total_leaked_count, 10);
    }

    /// TC-15.5.4.2 — frees remove leaks.
    #[test]
    fn tc_15_5_4_2_no_false_leak() {
        let t = MemAllocTracker::new();
        let mut det = LeakDetector::new(&t);
        det.take_baseline();
        for i in 0..10_u32 {
            t.record_alloc(20_000 + u64::from(i), 8, 0, 0);
        }
        for i in 0..10_u32 {
            t.record_dealloc(20_000 + u64::from(i));
        }
        let r = det.check();
        assert_eq!(r.total_leaked_count, 0);
    }

    /// TC-15.5.4.3 — grouping by call site + asset type.
    #[test]
    fn tc_15_5_4_3_leak_grouping() {
        let t = MemAllocTracker::new();
        let mut det = LeakDetector::new(&t);
        det.take_baseline();
        let stack_a = vec![0xA1, 0xA2, 0xA3];
        let stack_b = vec![0xB1, 0xB2];
        for _ in 0..5 {
            t.record_alloc_with_stack(0, 4, 0, 1, stack_a.clone());
        }
        for _ in 0..3 {
            t.record_alloc_with_stack(0, 8, 0, 2, stack_b.clone());
        }
        let r = det.check();
        assert_eq!(r.groups.len(), 2);
        let mut counts: Vec<u32> = r.groups.iter().map(|g| g.count).collect();
        counts.sort_unstable();
        assert_eq!(counts, vec![3, 5]);
    }
}
