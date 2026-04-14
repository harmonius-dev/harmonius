//! Integration tests for `harmonius_primitives` mapped to `TC-*` identifiers in
//! `docs/design/core-runtime/primitives-test-cases.md`.

use core::cmp::Ordering;
use core::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};

use glam::UVec3;
use harmonius_primitives::{
    BudgetAllocator, DeterministicRng, DirtyRegionSet, DispatchTable, FixedBitSet,
    GenerationalIndex, Handle, HandleMap, Region, RingBuffer, SlotMap, SmallVec, SortedVecMap,
};
use rkyv::rancor::Error;

#[test]
fn tc_1_7_4_2_handle_map_insert_get_remove() {
    let mut map = HandleMap::new();
    let h = map.insert(42);
    assert_eq!(map.get(h), Some(&42));
    assert_eq!(map.remove(h), Some(42));
    assert_eq!(map.get(h), None);
}

#[test]
fn tc_1_7_4_2_handle_map_reuse_invalidates_stale_handle() {
    let mut map = HandleMap::new();
    let h0 = map.insert(1);
    map.remove(h0).unwrap();
    let h1 = map.insert(2);
    assert_eq!(map.get(h0), None);
    assert_eq!(map.get(h1), Some(&2));
}

#[test]
fn tc_1_7_4_3_handle_map_generation_wrap() {
    let mut map = HandleMap::new();
    let h = map.insert(());
    map.remove(h).unwrap();
    map.set_slot_generation_for_test(0, u32::MAX);
    let h_max = map.insert(());
    assert_eq!(h_max.generation, u32::MAX);
    map.remove(h_max).unwrap();
    let h0 = map.insert(());
    assert_eq!(h0.generation, 0);
    assert_eq!(map.get(h_max), None);
}

#[test]
fn tc_1_7_5_1_slot_map_dense_iteration() {
    let mut map = SlotMap::new();
    let mut handles = Vec::new();
    for i in 0..1000 {
        handles.push(map.insert(i));
    }
    for (idx, h) in handles.iter().enumerate() {
        if idx % 3 == 0 {
            map.remove(*h).unwrap();
        }
    }
    let sum: i32 = map.as_slice().iter().sum();
    let expected: i32 = (0..1000).filter(|i| i % 3 != 0).sum();
    assert_eq!(sum, expected);
    let removed = handles.len().div_ceil(3);
    assert_eq!(map.as_slice().len(), handles.len() - removed);
}

#[test]
fn tc_1_7_5_2_generational_index_equality() {
    let a = GenerationalIndex {
        index: 1,
        generation: 0,
    };
    let b = GenerationalIndex {
        index: 1,
        generation: 0,
    };
    assert_eq!(a, b);
    assert_ne!(
        a,
        GenerationalIndex {
            index: 1,
            generation: 1,
        }
    );
}

#[test]
fn tc_1_9_2_1_sorted_vec_map_ordering_and_replace() {
    let mut map = SortedVecMap::new();
    for k in [5, 1, 3, 2, 4] {
        map.insert(k, k);
    }
    let keys: Vec<i32> = map.iter().map(|(k, _)| *k).collect();
    assert_eq!(keys, vec![1, 2, 3, 4, 5]);
    assert_eq!(map.insert(3, 99), Some(3));
    assert_eq!(map.len(), 5);
    assert_eq!(map.get(&3), Some(&99));
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CountingKey(i32);

static CMP_COUNT: AtomicUsize = AtomicUsize::new(0);

impl PartialOrd for CountingKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CountingKey {
    fn cmp(&self, other: &Self) -> Ordering {
        CMP_COUNT.fetch_add(1, AtomicOrdering::Relaxed);
        self.0.cmp(&other.0)
    }
}

#[test]
fn tc_1_9_2_2_sorted_vec_map_binary_search_comparison_budget() {
    let mut map = SortedVecMap::<CountingKey, i32>::new();
    for i in 0..1000 {
        map.insert(CountingKey(i), i);
    }
    CMP_COUNT.store(0, AtomicOrdering::Relaxed);
    assert_eq!(map.get(&CountingKey(500)), Some(&500));
    let comparisons = CMP_COUNT.load(AtomicOrdering::Relaxed);
    assert!(
        comparisons < 20,
        "expected <20 Ord comparisons, got {comparisons}"
    );
}

#[test]
fn tc_1_9_3_1_ring_buffer_wraparound() {
    let mut buf = RingBuffer::<u8, 4>::new();
    buf.push(1).unwrap();
    buf.push(2).unwrap();
    buf.push(3).unwrap();
    buf.push(4).unwrap();
    assert_eq!(buf.pop(), Some(1));
    assert_eq!(buf.pop(), Some(2));
    buf.push(5).unwrap();
    buf.push(6).unwrap();
    assert_eq!(buf.pop(), Some(3));
    assert_eq!(buf.pop(), Some(4));
    assert_eq!(buf.pop(), Some(5));
    assert_eq!(buf.pop(), Some(6));
    assert_eq!(buf.pop(), None);
}

#[test]
fn tc_1_9_3_2_ring_buffer_full_push_returns_err() {
    let mut buf = RingBuffer::<u8, 4>::new();
    buf.push(1).unwrap();
    buf.push(2).unwrap();
    buf.push(3).unwrap();
    buf.push(4).unwrap();
    assert_eq!(buf.push(5), Err(5));
    assert_eq!(buf.pop(), Some(1));
}

#[test]
fn tc_1_9_4_1_dirty_region_set_coalesce_adjacent() {
    let mut set = DirtyRegionSet::new();
    set.mark(Region {
        min: UVec3::new(0, 0, 0),
        max: UVec3::new(1, 1, 1),
    });
    set.mark(Region {
        min: UVec3::new(1, 0, 0),
        max: UVec3::new(2, 1, 1),
    });
    set.coalesce();
    let mut drained = set.drain();
    assert_eq!(drained.len(), 1);
    let merged = drained.pop().unwrap();
    assert_eq!(merged.min, UVec3::new(0, 0, 0));
    assert_eq!(merged.max, UVec3::new(2, 1, 1));
}

#[test]
fn tc_1_9_4_2_dirty_region_set_drain_clears() {
    let mut set = DirtyRegionSet::new();
    set.mark(Region {
        min: UVec3::ZERO,
        max: UVec3::ONE,
    });
    set.mark(Region {
        min: UVec3::ONE,
        max: UVec3::splat(2),
    });
    set.mark(Region {
        min: UVec3::splat(2),
        max: UVec3::splat(3),
    });
    assert_eq!(set.drain().len(), 3);
    assert!(set.drain().is_empty());
}

#[test]
fn tc_1_9_5_1_dispatch_table_register_get() {
    let mut table = DispatchTable::new();
    table.register(5, || 42);
    assert_eq!(table.get(5).unwrap()(), 42);
}

#[test]
fn tc_1_7_6_1_budget_allocator_reserve_within() {
    let mut budget = BudgetAllocator::new(1024);
    assert_eq!(budget.reserve(512), Some(()));
    assert_eq!(budget.available(), 512);
}

#[test]
fn tc_1_7_6_2_budget_allocator_over_budget() {
    let mut budget = BudgetAllocator::new(1024);
    assert_eq!(budget.reserve(512), Some(()));
    assert_eq!(budget.reserve(512), Some(()));
    assert_eq!(budget.reserve(1), None);
}

#[test]
fn tc_1_9_6_1_deterministic_rng_reproducible_sequence() {
    const GOLDEN: [u64; 8] = [
        14219364052333592195,
        7332719151195188792,
        6122488799882574371,
        4799409443904522999,
        18090429560773761838,
        11343726250536552999,
        17589260921017250467,
        6105855439640220682,
    ];
    let mut a = DeterministicRng::seed(0xDEADBEEF);
    let mut b = DeterministicRng::seed(0xDEADBEEF);
    for (i, expected) in GOLDEN.iter().enumerate() {
        let v = a.next_u64();
        assert_eq!(v, *expected, "golden mismatch at index {i}");
        assert_eq!(b.next_u64(), v);
    }
    for _ in GOLDEN.len()..1000 {
        assert_eq!(a.next_u64(), b.next_u64());
    }
}

#[test]
fn tc_1_9_6_2_deterministic_rng_gen_range_bounds() {
    let mut rng = DeterministicRng::seed(1);
    for _ in 0..100 {
        let v = rng.gen_range(10, 20);
        assert!((10..20).contains(&v));
    }
}

#[test]
fn tc_1_9_6_3_deterministic_rng_rkyv_roundtrip() {
    let mut baseline = DeterministicRng::seed(0xC0FFEE);
    for _ in 0..500 {
        baseline.next_u64();
    }

    let mut mid = DeterministicRng::seed(0xC0FFEE);
    for _ in 0..500 {
        mid.next_u64();
    }

    let bytes = rkyv::to_bytes::<Error>(&mid).unwrap();
    let mut restored: DeterministicRng =
        rkyv::from_bytes::<DeterministicRng, Error>(&bytes).unwrap();

    for _ in 0..500 {
        baseline.next_u64();
        restored.next_u64();
    }

    for _ in 0..200 {
        assert_eq!(baseline.next_u64(), restored.next_u64());
    }
}

#[test]
fn tc_1_9_7_1_smallvec_inline_no_spill() {
    let mut v: SmallVec<u32, 4> = SmallVec::new();
    for i in 0..4 {
        v.push(i);
    }
    assert!(!v.spilled());
}

#[test]
fn tc_1_9_7_2_smallvec_spills_on_fifth_push() {
    let mut v: SmallVec<u32, 4> = SmallVec::new();
    for i in 0..5 {
        v.push(i);
    }
    assert!(v.spilled());
}

#[test]
fn tc_1_7_4_4_ecs_style_handle_map_spawn_despawn_no_leak() {
    #[derive(Debug)]
    struct EntityRow {
        _generation: u32,
    }

    let mut store = HandleMap::new();
    let mut live = Vec::new();
    for gen in 0..512 {
        let h = store.insert(EntityRow { _generation: gen });
        live.push(h);
    }
    for h in live.drain(..) {
        store.remove(h).unwrap();
    }
    assert_eq!(store.len(), 0);
}

#[test]
fn tc_1_7_5_3_asset_style_slot_map_dense_iteration_10k() {
    #[derive(Debug, PartialEq)]
    struct MeshAsset {
        id: u32,
    }

    let mut map = SlotMap::new();
    let mut handles = Vec::new();
    for i in 0..10_000 {
        handles.push(map.insert(MeshAsset { id: i }));
    }
    assert_eq!(map.as_slice().len(), 10_000);
    let mut ids: Vec<u32> = map.as_slice().iter().map(|m| m.id).collect();
    ids.sort_unstable();
    for expect in 0..10_000 {
        assert!(ids.binary_search(&expect).is_ok());
    }
    drop(handles);
}

#[test]
#[ignore]
fn tc_1_7_4_5_handle_map_1m_insert_performance() {
    let mut map = HandleMap::<u32>::new();
    let start = std::time::Instant::now();
    let mut last = Handle::NULL;
    for i in 0..1_000_000 {
        last = map.insert(i);
    }
    assert!(!last.is_null());
    let elapsed = start.elapsed();
    assert!(elapsed.as_millis() < 50, "expected <50ms, got {elapsed:?}");
}

#[test]
#[ignore]
fn tc_1_9_2_3_sorted_vec_map_10k_lookup_performance() {
    let mut map = SortedVecMap::new();
    for i in 0..10_000 {
        map.insert(i, i);
    }
    let start = std::time::Instant::now();
    for _ in 0..10_000 {
        let _ = map.get(&5000);
    }
    let elapsed = start.elapsed() / 10_000;
    assert!(
        elapsed.as_micros() < 20,
        "expected <20µs average, got {elapsed:?}"
    );
}

#[test]
#[ignore]
fn tc_1_9_6_4_deterministic_rng_1m_u64_performance() {
    let mut rng = DeterministicRng::seed(123);
    let start = std::time::Instant::now();
    for _ in 0..1_000_000 {
        rng.next_u64();
    }
    let elapsed = start.elapsed();
    assert!(elapsed.as_millis() < 10, "expected <10ms, got {elapsed:?}");
}

#[test]
#[ignore]
fn tc_1_9_4_3_dirty_region_set_coalesce_10k_performance() {
    let mut set = DirtyRegionSet::new();
    for i in 0..10_000 {
        let base = i % 256;
        set.mark(Region {
            min: UVec3::new(base, base, base),
            max: UVec3::new(base + 1, base + 1, base + 1),
        });
    }
    let start = std::time::Instant::now();
    set.coalesce();
    let elapsed = start.elapsed();
    assert!(elapsed.as_millis() < 5, "expected <5ms, got {elapsed:?}");
}

#[test]
fn compile_fail_handle_type_mismatch() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/handle_type_mismatch.rs");
}

#[test]
fn fixed_bit_set_alias_smoke() {
    let mut bits = FixedBitSet::with_capacity(128);
    bits.insert(7);
    assert!(bits.contains(7));
}
