//! Memory allocation tracking (design: `MemAllocTracker`, `MemorySnapshot`).

use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, RwLock};

/// One live allocation sample.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Allocation {
    /// Stable allocation id (synthetic address).
    pub address: u64,
    /// Allocation size in bytes.
    pub size_bytes: u32,
    /// Subsystem tag id for treemap rollups.
    pub subsystem_id: u32,
    /// Asset type tag id for grouping.
    pub asset_type_id: u32,
    /// Return addresses (best-effort; see [`StackCapture::capture`]).
    pub call_stack: Vec<u64>,
}

/// Point-in-time memory snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemorySnapshot {
    /// Monotonic snapshot id.
    pub timestamp: u64,
    /// Live allocations at capture time.
    pub allocations: Vec<Allocation>,
    /// Sum of [`Allocation::size_bytes`] for `allocations`.
    pub total_bytes: u64,
}

/// One grouped leak bucket.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LeakGroup {
    /// Shared return-address prefix for the bucket.
    pub call_stack: Vec<u64>,
    /// Asset type id shared by the bucket.
    pub asset_type_id: u32,
    /// Number of leaked allocations in the bucket.
    pub count: u32,
    /// Sum of leaked bytes in the bucket.
    pub total_bytes: u64,
}

/// Result of comparing two snapshots for leaks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LeakReport {
    /// Grouped leak buckets keyed by call site.
    pub groups: Vec<LeakGroup>,
    /// Sum of leaked bytes across all buckets.
    pub total_leaked_bytes: u64,
    /// Total leaked allocation count.
    pub total_leaked_count: u32,
}

impl MemorySnapshot {
    /// Allocations present in `later` but not in `self` (by address).
    #[must_use]
    pub fn diff(&self, later: &MemorySnapshot) -> LeakReport {
        let earlier: HashSet<u64> = self.allocations.iter().map(|a| a.address).collect();
        let leaked: Vec<Allocation> = later
            .allocations
            .iter()
            .filter(|a| !earlier.contains(&a.address))
            .cloned()
            .collect();
        let total_leaked_count = u32::try_from(leaked.len()).unwrap_or(u32::MAX);
        let total_leaked_bytes = leaked.iter().map(|a| u64::from(a.size_bytes)).sum();
        let mut groups: HashMap<(Vec<u64>, u32), LeakGroup> = HashMap::new();
        for a in leaked {
            let key = (a.call_stack.clone(), a.asset_type_id);
            groups
                .entry(key)
                .and_modify(|g| {
                    g.count += 1;
                    g.total_bytes += u64::from(a.size_bytes);
                })
                .or_insert(LeakGroup {
                    call_stack: a.call_stack.clone(),
                    asset_type_id: a.asset_type_id,
                    count: 1,
                    total_bytes: u64::from(a.size_bytes),
                });
        }
        let mut group_vec: Vec<LeakGroup> = groups.into_values().collect();
        group_vec.sort_by_key(|g| (g.asset_type_id, g.count));
        LeakReport {
            groups: group_vec,
            total_leaked_bytes,
            total_leaked_count,
        }
    }
}

/// Tracks live allocations for memory profiling.
#[derive(Debug)]
pub struct MemAllocTracker {
    live: RwLock<HashMap<u64, Allocation>>,
    next_id: Mutex<u64>,
    clock: Mutex<u64>,
    frame_allocs: AtomicU32,
}

impl MemAllocTracker {
    /// Empty tracker.
    #[must_use]
    pub fn new() -> Self {
        Self {
            live: RwLock::new(HashMap::new()),
            next_id: Mutex::new(1),
            clock: Mutex::new(0),
            frame_allocs: AtomicU32::new(0),
        }
    }

    fn bump_clock(&self) -> u64 {
        let mut c = self.clock.lock().expect("poisoned lock");
        *c += 1;
        *c
    }

    /// Marks the start of a new frame for [`MemAllocTracker::per_frame_alloc_rate`].
    pub fn begin_frame(&self) {
        self.frame_allocs.store(0, Ordering::Relaxed);
    }

    /// Records a new allocation (synthetic address assigned when `address == 0`).
    pub fn record_alloc(&self, address: u64, size: u32, subsystem: u32, asset_type: u32) {
        let addr = if address == 0 {
            let mut n = self.next_id.lock().expect("poisoned lock");
            let id = *n;
            *n += 1;
            id
        } else {
            address
        };
        let alloc = Allocation {
            address: addr,
            size_bytes: size,
            subsystem_id: subsystem,
            asset_type_id: asset_type,
            call_stack: Vec::new(),
        };
        self.live
            .write()
            .expect("poisoned lock")
            .insert(addr, alloc);
        self.frame_allocs.fetch_add(1, Ordering::Relaxed);
    }

    /// Records a deallocation of `address`.
    pub fn record_dealloc(&self, address: u64) {
        self.live.write().expect("poisoned lock").remove(&address);
    }

    /// Captures all live allocations.
    #[must_use]
    pub fn take_snapshot(&self) -> MemorySnapshot {
        let live = self.live.read().expect("poisoned lock");
        let allocations: Vec<Allocation> = live.values().cloned().collect();
        let total_bytes = allocations
            .iter()
            .map(|a| u64::from(a.size_bytes))
            .sum();
        MemorySnapshot {
            timestamp: self.bump_clock(),
            allocations,
            total_bytes,
        }
    }

    /// Allocations recorded since the last [`MemAllocTracker::begin_frame`].
    #[must_use]
    pub fn per_frame_alloc_rate(&self) -> u32 {
        self.frame_allocs.load(Ordering::Relaxed)
    }

    /// Rolls up live bytes by `subsystem_id`.
    #[must_use]
    pub fn memory_by_subsystem(&self) -> Vec<(u32, u64)> {
        let live = self.live.read().expect("poisoned lock");
        let mut sums: HashMap<u32, u64> = HashMap::new();
        for a in live.values() {
            *sums.entry(a.subsystem_id).or_insert(0) += u64::from(a.size_bytes);
        }
        let mut out: Vec<(u32, u64)> = sums.into_iter().collect();
        out.sort_by_key(|(k, _)| *k);
        out
    }

    /// Records an allocation including a captured stack.
    pub fn record_alloc_with_stack(
        &self,
        address: u64,
        size: u32,
        subsystem: u32,
        asset_type: u32,
        stack: Vec<u64>,
    ) {
        let addr = if address == 0 {
            let mut n = self.next_id.lock().expect("poisoned lock");
            let id = *n;
            *n += 1;
            id
        } else {
            address
        };
        let alloc = Allocation {
            address: addr,
            size_bytes: size,
            subsystem_id: subsystem,
            asset_type_id: asset_type,
            call_stack: stack,
        };
        self.live
            .write()
            .expect("poisoned lock")
            .insert(addr, alloc);
        self.frame_allocs.fetch_add(1, Ordering::Relaxed);
    }
}

impl Default for MemAllocTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Platform call-stack capture (design: `StackCapture`).
pub struct StackCapture;

#[cfg(unix)]
extern "C" {
    fn backtrace(buffer: *mut *mut std::ffi::c_void, size: std::os::raw::c_int)
        -> std::os::raw::c_int;
}

#[cfg(windows)]
extern "system" {
    fn CaptureStackBackTrace(
        frames_to_skip: u32,
        frames_to_capture: u32,
        back_trace: *mut *mut std::ffi::c_void,
        back_trace_hash: *mut u32,
    ) -> u16;
}

impl StackCapture {
    /// Captures up to `max_frames` return addresses for the current thread.
    #[must_use]
    pub fn capture(max_frames: u32) -> Vec<u64> {
        let cap = usize::try_from(max_frames).unwrap_or(64).clamp(1, 64);
        #[cfg(unix)]
        {
            let mut buf = vec![std::ptr::null_mut::<std::ffi::c_void>(); cap];
            let n = unsafe { backtrace(buf.as_mut_ptr(), buf.len() as std::os::raw::c_int) };
            let n = usize::try_from(n).unwrap_or(0).min(buf.len());
            buf.into_iter()
                .take(n)
                .filter(|p| !p.is_null())
                .map(|p| p as usize as u64)
                .collect()
        }
        #[cfg(all(not(unix), windows))]
        {
            let mut buf = vec![std::ptr::null_mut::<std::ffi::c_void>(); cap];
            let mut hash = 0_u32;
            let n = unsafe {
                CaptureStackBackTrace(0, cap as u32, buf.as_mut_ptr(), &mut hash)
            };
            let n = usize::from(n).min(buf.len());
            buf.into_iter()
                .take(n)
                .filter(|p| !p.is_null())
                .map(|p| p as usize as u64)
                .collect()
        }
        #[cfg(all(not(unix), not(windows)))]
        {
            let _ = cap;
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-15.5.3.1 — record 100 varying allocations.
    #[test]
    fn tc_15_5_3_1_allocation_tracking() {
        let t = MemAllocTracker::new();
        t.begin_frame();
        for i in 0..100_u32 {
            t.record_alloc(1000 + u64::from(i), 10 + (i % 7), 1, 2);
        }
        let mut snap = t.take_snapshot();
        assert_eq!(snap.allocations.len(), 100);
        snap.allocations.sort_by_key(|a| a.address);
        for (i, a) in snap.allocations.iter().enumerate() {
            let i = u32::try_from(i).unwrap();
            assert_eq!(a.address, 1000 + u64::from(i));
            assert_eq!(a.size_bytes, 10 + (i % 7));
        }
    }

    /// TC-15.5.3.2 — dealloc removes live rows.
    #[test]
    fn tc_15_5_3_2_deallocation_tracking() {
        let t = MemAllocTracker::new();
        for i in 0..50_u32 {
            t.record_alloc(2000 + u64::from(i), 16, 0, 0);
        }
        for i in 0..20_u32 {
            t.record_dealloc(2000 + u64::from(i));
        }
        let snap = t.take_snapshot();
        assert_eq!(snap.allocations.len(), 30);
    }

    /// TC-15.5.3.3 — subsystem totals match `total_bytes`.
    #[test]
    fn tc_15_5_3_3_treemap_by_subsystem() {
        let t = MemAllocTracker::new();
        t.record_alloc(1, 100, 10, 0);
        t.record_alloc(2, 200, 20, 0);
        t.record_alloc(3, 50, 10, 0);
        let snap = t.take_snapshot();
        let by = t.memory_by_subsystem();
        let sum: u64 = by.iter().map(|(_, b)| b).sum();
        assert_eq!(sum, snap.total_bytes);
    }

    /// TC-15.5.3.4 — stack capture returns a short chain.
    #[test]
    fn tc_15_5_3_4_stack_capture() {
        fn d() -> Vec<u64> {
            StackCapture::capture(16)
        }
        fn c() -> Vec<u64> {
            d()
        }
        fn b() -> Vec<u64> {
            c()
        }
        fn a() -> Vec<u64> {
            b()
        }
        let stack = a();
        assert!(stack.len() >= 3, "stack={stack:?}");
    }

    /// TC-15.5.3.5 — per-frame allocation counter.
    #[test]
    fn tc_15_5_3_5_per_frame_alloc_rate() {
        let t = MemAllocTracker::new();
        t.begin_frame();
        for _ in 0..500 {
            t.record_alloc(0, 8, 0, 0);
        }
        assert_eq!(t.per_frame_alloc_rate(), 500);
    }
}
