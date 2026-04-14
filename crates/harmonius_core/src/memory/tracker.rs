//! Development-only allocation profiling hooks.

use super::tag::AllocationTag;
use std::collections::BTreeMap;
use std::sync::Mutex;

#[derive(Default)]
struct TrackerInner {
    counts: BTreeMap<(u16, &'static str), usize>,
    bytes: BTreeMap<(u16, &'static str), usize>,
    peak: usize,
    current: usize,
}

/// Records per-tag allocation statistics (compiled out in release builds).
pub struct MemoryTracker {
    #[cfg(debug_assertions)]
    inner: Mutex<TrackerInner>,
}

impl Default for MemoryTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryTracker {
    /// Creates a tracker instance.
    #[must_use]
    pub fn new() -> Self {
        Self {
            #[cfg(debug_assertions)]
            inner: Mutex::new(TrackerInner::default()),
        }
    }

    /// Returns `true` when profiling hooks are compiled to no-ops.
    #[must_use]
    pub const fn profiling_disabled() -> bool {
        !cfg!(debug_assertions)
    }

    /// Records an allocation tagged with `tag`.
    pub fn record_alloc(&self, tag: AllocationTag, bytes: usize, _site: &'static str) {
        #[cfg(debug_assertions)]
        {
            let mut g = self.inner.lock().expect("tracker lock");
            let key_label = tag.label.unwrap_or("");
            let key = (tag.subsystem.0, key_label);
            *g.counts.entry(key).or_insert(0) += 1;
            *g.bytes.entry(key).or_insert(0) += bytes;
            g.current += bytes;
            g.peak = g.peak.max(g.current);
        }
    }

    /// Records a deallocation tagged with `tag`.
    pub fn record_dealloc(&self, tag: AllocationTag, bytes: usize) {
        #[cfg(debug_assertions)]
        {
            let mut g = self.inner.lock().expect("tracker lock");
            let key_label = tag.label.unwrap_or("");
            let key = (tag.subsystem.0, key_label);
            *g.bytes.entry(key).or_insert(0) = g.bytes[&key].saturating_sub(bytes);
            g.current = g.current.saturating_sub(bytes);
        }
    }

    /// Total bytes recorded for `tag` (debug builds only).
    #[must_use]
    pub fn bytes_for(&self, tag: AllocationTag) -> usize {
        #[cfg(debug_assertions)]
        {
            let g = self.inner.lock().expect("tracker lock");
            let key_label = tag.label.unwrap_or("");
            let key = (tag.subsystem.0, key_label);
            *g.bytes.get(&key).unwrap_or(&0)
        }
        #[cfg(not(debug_assertions))]
        {
            let _ = tag;
            0
        }
    }

    /// Peak bytes across all tags (debug builds only).
    #[must_use]
    pub fn peak_usage(&self) -> usize {
        #[cfg(debug_assertions)]
        {
            self.inner.lock().expect("tracker lock").peak
        }
        #[cfg(not(debug_assertions))]
        {
            0
        }
    }

    /// Allocation count for `tag` (debug builds only).
    #[must_use]
    pub fn count_for(&self, tag: AllocationTag) -> usize {
        #[cfg(debug_assertions)]
        {
            let g = self.inner.lock().expect("tracker lock");
            let key_label = tag.label.unwrap_or("");
            let key = (tag.subsystem.0, key_label);
            *g.counts.get(&key).unwrap_or(&0)
        }
        #[cfg(not(debug_assertions))]
        {
            let _ = tag;
            0
        }
    }
}
