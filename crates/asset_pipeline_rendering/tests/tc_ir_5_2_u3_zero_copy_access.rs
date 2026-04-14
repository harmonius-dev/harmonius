//! TC-IR-5.2.U3 — `rkyv::access` on a baked texture buffer must not allocate on the heap.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

use asset_pipeline_rendering::{ArchivedBakedTexture, BakedTexture, GpuTextureFormat};
use rkyv::access;
use rkyv::rancor::Error;
use rkyv::to_bytes;

struct CountingAlloc;

static ALLOC_HITS: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for CountingAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOC_HITS.fetch_add(1, Ordering::SeqCst);
        System.alloc(layout)
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        ALLOC_HITS.fetch_add(1, Ordering::SeqCst);
        System.alloc_zeroed(layout)
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        ALLOC_HITS.fetch_add(1, Ordering::SeqCst);
        System.realloc(ptr, layout, new_size)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
    }
}

#[global_allocator]
static GLOBAL: CountingAlloc = CountingAlloc;

#[test]
fn tc_ir_5_2_u3_archived_vec_access_allocates_no_heap_on_access_path() {
    let tex = BakedTexture {
        data: vec![9_u8; 32],
        format: GpuTextureFormat::Bc7Unorm,
        height: 2,
        mip_count: 1,
        mip_offsets: vec![0_u64],
        width: 2,
    };
    let bytes = to_bytes::<Error>(&tex).expect("serialize");
    let before = ALLOC_HITS.load(Ordering::SeqCst);
    let archived = access::<ArchivedBakedTexture, Error>(bytes.as_slice()).expect("access");
    let after = ALLOC_HITS.load(Ordering::SeqCst);
    assert_eq!(
        after, before,
        "validated access must not allocate; hits before={before} after={after}"
    );
    assert_eq!(archived.data.len(), 32);
}
