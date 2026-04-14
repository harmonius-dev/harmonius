//! TC-1.12.5.2 — constructing [`core_runtime::Diagnostic`] with a static message and no path must
//! not trigger heap allocation (R-1.12.5a).
//!
//! This file is its own integration-test binary so we can install a counting [`GlobalAlloc`] without
//! affecting other tests.

use core_runtime::{Diagnostic, Severity};
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

struct CountingAlloc;

static HEAP_ALLOCS: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for CountingAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        HEAP_ALLOCS.fetch_add(1, Ordering::SeqCst);
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static GLOBAL: CountingAlloc = CountingAlloc;

#[test]
fn tc_1_12_5_2_diagnostic_allocation_avoided() {
    let before = HEAP_ALLOCS.load(Ordering::SeqCst);
    let diagnostic = Diagnostic {
        path: None,
        span: None,
        message: "static",
        severity: Severity::Error,
    };
    let after = HEAP_ALLOCS.load(Ordering::SeqCst);
    assert_eq!(
        after, before,
        "expected zero heap allocations while constructing Diagnostic"
    );
    std::hint::black_box(diagnostic);
}
