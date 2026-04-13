# Core Primitives Test Cases

Companion test cases for [primitives.md](primitives.md).

## Unit Tests

### TC-1.7.4.1 Handle Type Tagging

| # | Requirement |
|---|-------------|
| 1 | F-1.7.4     |

1. **#1** — `let h: Handle<Mesh> = map.insert(mesh)`; attempt to pass `h` to `HandleMap<Texture>`
   - **Expected:** Compile error (type mismatch on `PhantomData<fn() -> T>`)

### TC-1.7.4.2 HandleMap Insert Get Remove

| # | Requirement |
|---|-------------|
| 1 | F-1.7.4     |
| 2 | F-1.7.4     |
| 3 | F-1.7.4     |

1. **#1** — Insert value 42; call `get(handle)`
   - **Expected:** `Some(&42)`
2. **#2** — Remove handle; call `get(handle)` again
   - **Expected:** `None` (stale generation)
3. **#3** — Insert new value at same index; reuse old handle
   - **Expected:** `get(old_handle) == None`

### TC-1.7.4.3 HandleMap Generation Wrap

| # | Requirement |
|---|-------------|
| 1 | F-1.7.4     |

1. **#1** — Insert/remove until `generation` reaches `u32::MAX`, insert once more
   - **Expected:** `generation` wraps to 0; old handles invalidated

### TC-1.7.5.1 SlotMap Dense Iteration

| # | Requirement |
|---|-------------|
| 1 | F-1.7.5     |

1. **#1** — Insert 1000 values, remove every 3rd, `iter` the dense slice
   - **Expected:** Iterator yields exactly `len()` values contiguously

### TC-1.7.5.2 GenerationalIndex Equality

| # | Requirement |
|---|-------------|
| 1 | F-1.7.5     |

1. **#1** — `GenerationalIndex { index: 1, generation: 0 } == { index: 1, generation: 0 }`
   - **Expected:** `true`
2. **#2** — Different generations
   - **Expected:** `false`

### TC-1.9.2.1 SortedVecMap Ordering

| # | Requirement |
|---|-------------|
| 1 | F-1.9.2     |
| 2 | F-1.9.2     |

1. **#1** — Insert keys `[5, 1, 3, 2, 4]`
   - **Expected:** `iter()` yields keys in order `[1, 2, 3, 4, 5]`
2. **#2** — `insert(3, "new")` on existing key
   - **Expected:** Returns `Some(old_value)`, map unchanged in size

### TC-1.9.2.2 SortedVecMap Binary Search

| # | Requirement |
|---|-------------|
| 1 | F-1.9.2     |

1. **#1** — Map with 1000 entries, `get(&500)`
   - **Expected:** `Some(&value)` in under 20 comparisons

### TC-1.9.3.1 RingBuffer Wrap Around

| # | Requirement |
|---|-------------|
| 1 | F-1.9.3     |

1. **#1** — `RingBuffer<u8, 4>`, push 4, pop 2, push 2, pop 4
   - **Expected:** Pops yield exactly the pushed values in order

### TC-1.9.3.2 RingBuffer Full

| # | Requirement |
|---|-------------|
| 1 | F-1.9.3     |

1. **#1** — `RingBuffer<u8, 4>` full, call `push(5)`
   - **Expected:** `Err(5)`; buffer unchanged

### TC-1.9.4.1 DirtyRegionSet Coalesce Adjacent

| # | Requirement |
|---|-------------|
| 1 | F-1.9.4     |

1. **#1** — Mark `[0,0,0..1,1,1]` and `[1,0,0..2,1,1]`, call `coalesce()`
   - **Expected:** Single region `[0,0,0..2,1,1]`

### TC-1.9.4.2 DirtyRegionSet Drain

| # | Requirement |
|---|-------------|
| 1 | F-1.9.4     |

1. **#1** — Mark 3 regions, call `drain()`
   - **Expected:** Returns 3 regions, set is empty

### TC-1.9.5.1 DispatchTable Register Dispatch

| # | Requirement |
|---|-------------|
| 1 | F-1.9.5     |

1. **#1** — Register closure for id=5, call `get(5)`
   - **Expected:** `Some(&closure)`

### TC-1.7.6.1 BudgetAllocator Reserve Within

| # | Requirement |
|---|-------------|
| 1 | F-1.7.6     |

1. **#1** — Ceiling 1024, reserve 512
   - **Expected:** `Some(())`; `available() == 512`

### TC-1.7.6.2 BudgetAllocator Over Budget

| # | Requirement |
|---|-------------|
| 1 | F-1.7.6     |

1. **#1** — Ceiling 1024, reserve 512 twice, reserve 1
   - **Expected:** Third call returns `None`

### TC-1.9.6.1 DeterministicRng Reproducible

| # | Requirement |
|---|-------------|
| 1 | F-1.9.6     |
| 2 | F-1.9.6     |

1. **#1** — Seed `0xDEADBEEF`, generate 1000 u64 values
   - **Expected:** Byte-identical sequence on every run and every platform
2. **#2** — Two RNGs with same seed
   - **Expected:** Produce identical sequences

### TC-1.9.6.2 DeterministicRng Range

| # | Requirement |
|---|-------------|
| 1 | F-1.9.6     |

1. **#1** — `gen_range(10, 20)` 100 times
   - **Expected:** Every value in `[10, 20)`

### TC-1.9.7.1 SmallVec Inline Storage

| # | Requirement |
|---|-------------|
| 1 | F-1.9.7     |

1. **#1** — `SmallVec<u32, 4>`, push 4 elements
   - **Expected:** No heap allocation

### TC-1.9.7.2 SmallVec Spill

| # | Requirement |
|---|-------------|
| 1 | F-1.9.7     |

1. **#1** — `SmallVec<u32, 4>`, push 5 elements
   - **Expected:** Heap allocation triggered on 5th push

## Integration Tests

### TC-1.7.4.4 ECS Entity Uses HandleMap

| # | Requirement |
|---|-------------|
| 1 | F-1.7.4     |

1. **#1** — ECS world uses `HandleMap<EntityRow>` as entity store
   - **Expected:** Spawn/despawn cycles do not leak handles

### TC-1.7.5.3 Asset Pipeline Uses SlotMap

| # | Requirement |
|---|-------------|
| 1 | F-1.7.5     |

1. **#1** — Asset system inserts 10K meshes into `SlotMap<MeshAsset>`
   - **Expected:** Iteration yields exactly 10K meshes in dense order

### TC-1.9.6.3 DeterministicRng Save Roundtrip

| # | Requirement |
|---|-------------|
| 1 | F-1.9.6     |

1. **#1** — Seed, advance 500 calls, rkyv-serialize, deserialize, advance 500 more
   - **Expected:** Resulting sequence matches a never-serialized RNG at same total step

## Benchmarks

### TC-1.7.4.5 HandleMap 1M Insert Under 50 ms

| # | Requirement |
|---|-------------|
| 1 | R-1.7.4a    |

1. **#1** — Insert 1,000,000 small values into empty `HandleMap`
   - **Expected:** Completes under 50 ms on reference workstation

### TC-1.9.2.3 SortedVecMap 10K Lookup Under 20 us

| # | Requirement |
|---|-------------|
| 1 | R-1.9.2a    |

1. **#1** — 10,000-entry map, `get(&key)` average
   - **Expected:** Under 20 microseconds per lookup

### TC-1.9.6.4 DeterministicRng 1M Throughput

| # | Requirement |
|---|-------------|
| 1 | R-1.9.6a    |

1. **#1** — Generate 1,000,000 `u64` values
   - **Expected:** Under 10 ms on reference workstation

### TC-1.9.4.3 DirtyRegionSet Coalesce 10K

| # | Requirement |
|---|-------------|
| 1 | R-1.9.4a    |

1. **#1** — 10,000 random regions, coalesce
   - **Expected:** Under 5 ms
