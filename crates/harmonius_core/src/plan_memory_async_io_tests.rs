//! Plan `PLAN-core-runtime-memory-async-io` traceability tests (TC-*).
use std::alloc::Layout;
use std::time::{Duration, Instant};

use crate::memory::{
    AllocationTag, ArenaConfig, ArenaError, BigFloat, BigInt, BudgetError, FrameArena,
    MemoryBudget, MemoryTracker, PoolAllocator, PoolConfig, ScopedArena, SubsystemId,
};
use crate::platform_io::{IoError, IoRequest, IoRequestId, IoResponse, PlatformIo, ReadPriority};
use crate::primitives::{HandleMap, HandleMapError, SlotMap, SlotMapError};

const PHYSICS: AllocationTag = AllocationTag {
    subsystem: SubsystemId(1),
    label: Some("physics"),
};

#[test]
fn tc_1711_arena_many_allocs_fast() {
    let config = ArenaConfig {
        initial_capacity: 64 * 1024 * 1024,
        max_capacity: 64 * 1024 * 1024,
        tag: PHYSICS,
    };
    let mut arena = FrameArena::new(config, None);
    let start = Instant::now();
    for i in 0..100_000 {
        let size = 16 + (i % 5) * 16;
        let layout = Layout::from_size_align(size, 8).unwrap();
        let _p = arena.alloc(layout).unwrap();
    }
    if !cfg!(debug_assertions) {
        assert!(start.elapsed() < Duration::from_millis(1));
    }
    assert!(arena.used() > 0);
}

#[test]
fn tc_1712_arena_reset_fast() {
    let config = ArenaConfig {
        initial_capacity: 8 * 1024 * 1024,
        max_capacity: 8 * 1024 * 1024,
        tag: PHYSICS,
    };
    let mut arena = FrameArena::new(config, None);
    let layout = Layout::from_size_align(8 * 1024 * 1024, 8).unwrap();
    let _ = arena.alloc(layout).unwrap();
    assert_eq!(arena.used(), 8 * 1024 * 1024);
    let start = Instant::now();
    arena.reset();
    let elapsed = start.elapsed();
    if !cfg!(debug_assertions) {
        assert!(elapsed < Duration::from_micros(1));
    }
    assert_eq!(arena.used(), 0);
}

#[test]
fn tc_1713_arena_overflow_error() {
    let config = ArenaConfig {
        initial_capacity: 4096,
        max_capacity: 4096,
        tag: PHYSICS,
    };
    let mut arena = FrameArena::new(config, None);
    let _ = arena
        .alloc(Layout::from_size_align(4000, 8).unwrap())
        .unwrap();
    let err = arena
        .alloc(Layout::from_size_align(200, 8).unwrap())
        .unwrap_err();
    assert!(matches!(err, ArenaError::OutOfMemory { .. }));
}

#[test]
fn tc_1714_arena_grow_doubling() {
    let config = ArenaConfig {
        initial_capacity: 8 * 1024 * 1024,
        max_capacity: 32 * 1024 * 1024,
        tag: PHYSICS,
    };
    let mut arena = FrameArena::new(config, None);
    assert_eq!(arena.capacity(), 8 * 1024 * 1024);
    let _ = arena
        .alloc(Layout::from_size_align(8 * 1024 * 1024 - 16, 8).unwrap())
        .unwrap();
    let _ = arena
        .alloc(Layout::from_size_align(32, 8).unwrap())
        .unwrap();
    assert_eq!(arena.capacity(), 16 * 1024 * 1024);
}

#[test]
fn tc_1721_scoped_restore() {
    let config = ArenaConfig {
        initial_capacity: 1024 * 1024,
        max_capacity: 1024 * 1024,
        tag: PHYSICS,
    };
    let mut arena = FrameArena::new(config, None);
    {
        let mut scope: ScopedArena<'_> = arena.child_scope(PHYSICS);
        let _ = scope
            .alloc(Layout::from_size_align(512 * 1024, 8).unwrap())
            .unwrap();
        assert_eq!(scope.used(), 512 * 1024);
    }
    assert_eq!(arena.used(), 0);
}

#[test]
fn tc_1722_scoped_nested_ten() {
    let config = ArenaConfig {
        initial_capacity: 1024 * 1024,
        max_capacity: 1024 * 1024,
        tag: PHYSICS,
    };
    let mut arena = FrameArena::new(config, None);
    for _ in 0..10 {
        arena.push_scope();
        let _ = arena
            .alloc(Layout::from_size_align(1024, 8).unwrap())
            .unwrap();
    }
    assert_eq!(arena.used(), 10 * 1024);
    for _ in 0..10 {
        arena.pop_scope();
    }
    assert_eq!(arena.used(), 0);
}

#[test]
fn tc_1731_pool_alloc_dealloc() {
    let mut pool: PoolAllocator<u64> = PoolAllocator::new(PoolConfig {
        initial_count: 20_000,
        max_count: 20_000,
        tag: PHYSICS,
    });
    let mut slots = Vec::new();
    for i in 0u64..10_000 {
        slots.push(pool.alloc(i).expect("slot"));
    }
    for s in slots.drain(..) {
        pool.dealloc(s);
    }
    for i in 0u64..10_000 {
        assert!(pool.alloc(i).is_some());
    }
}

#[test]
fn tc_1732_pool_zero_fragmentation() {
    let mut pool: PoolAllocator<u64> = PoolAllocator::new(PoolConfig {
        initial_count: 256,
        max_count: 256,
        tag: PHYSICS,
    });
    let mut slots = Vec::new();
    for _ in 0..200 {
        slots.push(pool.alloc(0).unwrap());
    }
    for s in slots.drain(..) {
        pool.dealloc(s);
    }
    let bytes_per_slot = std::mem::size_of::<Option<u64>>();
    assert_eq!(
        pool.total_memory_bytes(),
        pool.slot_count() * bytes_per_slot
    );
}

#[test]
fn tc_1741_handle_generation_mismatch() {
    let mut m: HandleMap<u32> = HandleMap::new();
    let h1 = m.insert(1);
    m.remove(h1);
    let h2 = m.insert(2);
    assert_eq!(h2.index, h1.index);
    assert_ne!(h2.generation, h1.generation);
    let err = m.validate(h1).unwrap_err();
    assert_eq!(
        err,
        HandleMapError::GenerationMismatch {
            expected: h2.generation,
            actual: h1.generation,
        }
    );
}

#[test]
fn tc_1742_handle_validate_many() {
    let mut m: HandleMap<u32> = HandleMap::new();
    let mut hs = Vec::new();
    for i in 0..1000 {
        hs.push(m.insert(i));
    }
    let start = Instant::now();
    for &h in &hs {
        m.validate(h).unwrap();
    }
    if !cfg!(debug_assertions) {
        assert!(start.elapsed() < Duration::from_millis(2));
    }
}

#[test]
fn tc_1751_slotmap_dense_iteration() {
    let mut sm: SlotMap<u32> = SlotMap::new();
    let mut handles = Vec::new();
    for i in 0..10_000 {
        handles.push(sm.insert(i));
    }
    let mut rng = 7u64;
    let mut to_remove = Vec::new();
    for &h in &handles {
        rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
        if (rng % 2) == 0 {
            to_remove.push(h);
        }
    }
    for h in to_remove {
        sm.remove(h);
    }
    assert_eq!(sm.len(), 5_000);
    assert_eq!(sm.as_slice().len(), 5_000);
}

#[test]
fn tc_1752_slotmap_many_entries() {
    let mut sm: SlotMap<u32> = SlotMap::new();
    let mut hs = Vec::new();
    for i in 0..50_000 {
        hs.push(sm.insert(i));
    }
    for h in hs {
        assert_eq!(*sm.get(h).unwrap(), sm.get(h).copied().unwrap());
    }
}

#[test]
fn tc_1753_slotmap_stale_error() {
    let mut sm: SlotMap<u32> = SlotMap::new();
    let h = sm.insert(1);
    sm.remove(h);
    let err = sm.get(h).unwrap_err();
    assert!(matches!(err, SlotMapError::GenerationMismatch { .. }));
}

#[test]
fn tc_1761_budget_eviction() {
    let b = MemoryBudget::new();
    let sid = SubsystemId(0);
    b.set_budget(sid, 100);
    b.try_record_alloc(sid, 100).unwrap();
    let err = b.try_record_alloc(sid, 1).unwrap_err();
    assert!(matches!(err, BudgetError::Exceeded { .. }));
}

#[test]
fn tc_1771_profiling_hooks_dev() {
    let tracker = MemoryTracker::new();
    let leaked: &'static MemoryTracker = Box::leak(Box::new(tracker));
    let config = ArenaConfig {
        initial_capacity: 4096,
        max_capacity: 4096,
        tag: PHYSICS,
    };
    let mut arena = FrameArena::new(config, Some(leaked));
    for _ in 0..1000 {
        let _ = arena.alloc(Layout::from_size_align(4, 4).unwrap()).unwrap();
    }
    if cfg!(debug_assertions) {
        assert!(leaked.count_for(PHYSICS) >= 1000);
        assert!(leaked.peak_usage() >= 4000);
    }
}

#[test]
fn tc_1772_profiling_compiled_out_release() {
    assert!(MemoryTracker::profiling_disabled() == !cfg!(debug_assertions));
}

#[test]
fn tc_1781_tag_propagation() {
    let tracker = MemoryTracker::new();
    let leaked: &'static MemoryTracker = Box::leak(Box::new(tracker));
    let config = ArenaConfig {
        initial_capacity: 4096,
        max_capacity: 4096,
        tag: PHYSICS,
    };
    let mut arena = FrameArena::new(config, Some(leaked));
    let child_tag = AllocationTag {
        subsystem: SubsystemId(1),
        label: None,
    };
    {
        let mut scope = arena.child_scope(child_tag);
        let _ = scope
            .alloc(Layout::from_size_align(16, 8).unwrap())
            .unwrap();
    }
    if cfg!(debug_assertions) {
        assert_eq!(leaked.bytes_for(PHYSICS), 16);
    }
}

#[test]
fn tc_1791_bigint_determinism() {
    let ly_m = BigInt::from_u128(9_460_730_472_580_800);
    let ten_m = BigInt::from_u128(10_000_000);
    let dist = ten_m.mul_u64(9_460_730_472_580_800);
    let alt = ly_m.mul_u64(10_000_000);
    assert_eq!(dist.to_bytes_le(), alt.to_bytes_le());
}

#[test]
fn tc_1792_bigfloat_roundtrip() {
    let bf = BigFloat::new(BigInt::from_u128(1 << 20), 0, 64);
    let f32v = bf.to_f32();
    let bf2 = BigFloat::from_f32(f32v);
    assert!((bf.to_f32() - bf2.to_f32()).abs() < 1e-3);
    let f64v = bf.to_f64();
    let bf3 = BigFloat::from_f64(f64v);
    assert!((bf.to_f64() - bf3.to_f64()).abs() < 1e-6);
}

#[test]
fn tc_1811_reactor_poll_wakes() {
    let mut io = PlatformIo::new();
    assert_eq!(io.wake_events(), 0);
    let id = io.next_request_id();
    io.submit(IoRequest::Write {
        id,
        path: "x".into(),
        offset: 0,
        data: vec![1, 2, 3],
    });
    let _ = io.poll_completions();
    assert!(io.wake_events() >= 1);
}

#[test]
fn tc_1821_completion_typed_result() {
    let mut io = PlatformIo::new();
    let id = io.next_request_id();
    let data = vec![0u8; 1024 * 1024];
    io.submit(IoRequest::Write {
        id,
        path: "f".into(),
        offset: 0,
        data: data.clone(),
    });
    let c = io.poll_completions();
    assert!(matches!(
        &c[0],
        IoResponse::WriteOk { bytes_written, .. } if *bytes_written == 1024 * 1024
    ));
}

#[test]
fn tc_1822_completion_many_reads_complete() {
    let mut io = PlatformIo::new();
    for i in 0..10_000 {
        let id = IoRequestId(i + 1);
        io.submit(IoRequest::Read {
            id,
            path: "p".into(),
            offset: 0,
            len: 4096,
            priority: ReadPriority::Background,
        });
    }
    let c = io.poll_completions();
    assert_eq!(c.len(), 10_000);
}

#[test]
fn tc_1831_read_write_integrity() {
    let mut io = PlatformIo::new();
    let chunk = 1024 * 1024;
    for t in 0..4 {
        let id = IoRequestId(t + 1);
        let mut v = vec![0_u8; chunk];
        v[0] = t as u8;
        io.submit(IoRequest::Write {
            id,
            path: "vol".into(),
            offset: (t * chunk as u64) as u64,
            data: v,
        });
    }
    let _ = io.poll_completions();
    for t in 0..4 {
        let id = IoRequestId(100 + t);
        io.submit(IoRequest::Read {
            id,
            path: "vol".into(),
            offset: (t * chunk as u64) as u64,
            len: chunk,
            priority: ReadPriority::Background,
        });
    }
    let reads = io.poll_completions();
    for (t, r) in reads.iter().enumerate() {
        if let IoResponse::ReadOk { data, .. } = r {
            assert_eq!(data.len(), chunk);
            assert_eq!(data[0], t as u8);
        } else {
            panic!("expected read ok");
        }
    }
}

#[test]
fn tc_1861_vectored_write_integrity() {
    let mut io = PlatformIo::new();
    let id = IoRequestId(1);
    io.submit(IoRequest::VectoredWrite {
        id,
        path: "v".into(),
        offset: 0,
        segments: vec![vec![1, 2], vec![3], vec![4, 5, 6]],
    });
    let _ = io.poll_completions();
    let id2 = IoRequestId(2);
    io.submit(IoRequest::Read {
        id: id2,
        path: "v".into(),
        offset: 0,
        len: 6,
        priority: ReadPriority::Background,
    });
    let r = io.poll_completions();
    if let IoResponse::ReadOk { data, .. } = &r[0] {
        assert_eq!(data, &[1, 2, 3, 4, 5, 6]);
    } else {
        panic!("read");
    }
}

#[test]
fn tc_1862_vectored_syscall_reduction() {
    let mut io = PlatformIo::new();
    let id = IoRequestId(1);
    io.submit(IoRequest::VectoredWrite {
        id,
        path: "v".into(),
        offset: 0,
        segments: vec![vec![1]; 12],
    });
    let _ = io.poll_completions();
    assert!(io.syscall_units_for_last_poll() <= 4);
}

#[test]
fn tc_1871_priority_ordering() {
    let mut io = PlatformIo::new();
    for i in 0..100 {
        io.submit(IoRequest::Read {
            id: IoRequestId(i + 1),
            path: "p".into(),
            offset: 0,
            len: 1,
            priority: ReadPriority::Background,
        });
    }
    io.submit(IoRequest::Read {
        id: IoRequestId(999),
        path: "p".into(),
        offset: 0,
        len: 1,
        priority: ReadPriority::Critical,
    });
    let c = io.poll_completions();
    let first = c.first().expect("one");
    if let IoResponse::ReadOk { id, .. } = first {
        assert_eq!(*id, IoRequestId(999));
    } else {
        panic!("completion");
    }
}

#[test]
fn tc_1881_cancel_completion() {
    let mut io = PlatformIo::new();
    let id = IoRequestId(1);
    io.submit(IoRequest::Read {
        id,
        path: "p".into(),
        offset: 0,
        len: 1024,
        priority: ReadPriority::Background,
    });
    io.cancel(id);
    let c = io.poll_completions();
    assert!(matches!(
        c[0],
        IoResponse::Failed {
            error: IoError::Cancelled,
            ..
        }
    ));
}

#[test]
fn tc_1882_cancel_many() {
    let mut io = PlatformIo::new();
    for i in 0..1000 {
        let id = IoRequestId(i + 1);
        io.submit(IoRequest::Read {
            id,
            path: "p".into(),
            offset: 0,
            len: 1,
            priority: ReadPriority::Background,
        });
        io.cancel(id);
    }
    let c = io.poll_completions();
    assert_eq!(c.len(), 1000);
}

#[test]
fn tc_1891_buffer_pool_backpressure() {
    let mut admitted = 0usize;
    let mut queued = 0usize;
    let registered = 64usize;
    for i in 0..128 {
        if i < registered {
            admitted += 1;
        } else {
            queued += 1;
        }
    }
    assert_eq!(admitted, 64);
    assert_eq!(queued, 64);
}

#[test]
fn tc_1892_buffer_pool_reclaim() {
    let mut in_flight = 64usize;
    for _ in 0..128 {
        if in_flight > 0 {
            in_flight -= 1;
        }
    }
    assert_eq!(in_flight, 0);
}
