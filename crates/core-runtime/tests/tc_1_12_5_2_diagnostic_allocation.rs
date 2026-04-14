//! TC-1.12.5.2 — `Diagnostic` with static message and no path must not heap-allocate (R-1.12.5a).
//!
//! Separate integration-test binary so a counting `GlobalAlloc` does not affect other tests.

use core_runtime::{Diagnostic, Severity};
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

struct CountingAlloc;

static HEAP_ALLOCS: AtomicUsize = AtomicUsize::new(0);

// SAFETY: This type only forwards to `System`; it preserves `GlobalAlloc` invariants from the
// platform default allocator.
unsafe impl GlobalAlloc for CountingAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        HEAP_ALLOCS.fetch_add(1, Ordering::SeqCst);
        // SAFETY: `layout` satisfies `GlobalAlloc::alloc` requirements; behavior matches `System`.
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: `ptr` and `layout` came from this allocator's `alloc`; forwarded to `System`.
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
