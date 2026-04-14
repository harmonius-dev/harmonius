use std::cell::Cell;
use std::marker::PhantomData;

use crate::collector::FrameCollector;
use crate::hash::fnv1a;

thread_local! {
    static ACTIVE: Cell<Option<*mut FrameCollector>> = const { Cell::new(None) };
    static LOCAL_THREAD_ID: Cell<Option<u64>> = const { Cell::new(None) };
}

/// Binds the active [`FrameCollector`] for [`CpuScopeGuard`] on this thread.
pub struct ProfileBindGuard<'a> {
    _marker: PhantomData<&'a mut FrameCollector>,
}

impl<'a> ProfileBindGuard<'a> {
    /// Installs `collector` as the TLS target for nested [`CpuScopeGuard`] values.
    pub fn enter(collector: &'a mut FrameCollector) -> Self {
        let ptr = collector as *mut FrameCollector;
        ACTIVE.with(|slot| {
            slot.set(Some(ptr));
        });
        Self {
            _marker: PhantomData,
        }
    }
}

impl Drop for ProfileBindGuard<'_> {
    fn drop(&mut self) {
        ACTIVE.with(|slot| {
            slot.set(None);
        });
    }
}

/// RAII guard that records a CPU scope around a static zone label.
pub struct CpuScopeGuard {
    zone: u32,
    _begin: u64,
    noop: bool,
    thread_id: u64,
}

impl CpuScopeGuard {
    /// Opens a scope for `name` using the TLS [`LOCAL_THREAD_ID`] set by
    /// [`FrameCollector::register_thread`].
    pub fn new(name: &'static str) -> Self {
        let zone = fnv1a(name);
        let begin = crate::monotonic_stamp();
        let thread_id = LOCAL_THREAD_ID.with(|slot| slot.get());
        let Some(thread_id) = thread_id else {
            return Self {
                zone,
                _begin: begin,
                noop: true,
                thread_id: 0,
            };
        };
        let mut noop = true;
        ACTIVE.with(|cell| {
            if let Some(ptr) = cell.get() {
                // SAFETY: `ptr` originates from [`ProfileBindGuard::enter`] and remains valid for
                // the lifetime of nested guards.
                let opened = unsafe { (*ptr).begin_scope(zone, begin, thread_id) };
                noop = !opened;
            }
        });
        Self {
            zone,
            _begin: begin,
            noop,
            thread_id,
        }
    }
}

impl Drop for CpuScopeGuard {
    fn drop(&mut self) {
        if self.noop {
            return;
        }
        let end = crate::monotonic_stamp();
        ACTIVE.with(|cell| {
            if let Some(ptr) = cell.get() {
                // SAFETY: same invariants as [`CpuScopeGuard::new`].
                unsafe {
                    (*ptr).end_scope(self.zone, end, self.thread_id);
                }
            }
        });
    }
}

/// Records the active thread id used by [`CpuScopeGuard::new`].
pub(crate) fn set_local_thread_id(thread_id: u64) {
    LOCAL_THREAD_ID.with(|slot| {
        slot.set(Some(thread_id));
    });
}
