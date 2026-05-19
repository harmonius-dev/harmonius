# Core Algorithms Test Cases

Companion test cases for [algorithms.md](algorithms.md).

## Unit Tests

### TC-1.7.4.1 Handle Insert Get Remove

| # | Requirement |
|---|-------------|
| 1 | F-1.1.11    |
| 2 | F-1.7.4     |
| 3 | F-1.7.4     |
| 4 | F-1.7.4     |

1. **#1** — `map.insert(42)`
   - **Expected:** Returns Handle(idx=0, gen=0)
2. **#2** — `map.get(handle)`
   - **Expected:** Returns `Some(&42)`
3. **#3** — `map.remove(handle)`
   - **Expected:** Returns `Some(42)`
4. **#4** — `map.get(handle)` after remove
   - **Expected:** Returns `None` (stale gen)

### TC-1.7.4.2 Handle Generation Overflow

| # | Requirement |
|---|-------------|
| 1 | F-1.7.4     |
| 2 | F-1.7.4     |

1. **#1** — Insert/remove at same slot until gen = u32::MAX
   - **Expected:** Gen wraps to 0
2. **#2** — Insert new value at wrapped slot
   - **Expected:** Handle(idx, gen=0) valid

### TC-1.7.4.3 Handle Iteration Skips Free

| # | Requirement |
|---|-------------|
| 1 | F-1.7.5     |
| 2 | F-1.7.5     |

1. **#1** — Insert 10, remove indices 2, 5, 8
   - **Expected:** 7 entries remain
2. **#2** — `map.iter()`
   - **Expected:** Visits exactly 7 entries

### TC-1.7.4.4 Handle Type Safety

| # | Requirement |
|---|-------------|
| 1 | F-1.7.4     |

1. **#1** — `Handle<Mesh>` used in `HandleMap<Texture>`
   - **Expected:** Compile error (type mismatch)

### TC-7.6.1 UniformGrid World-to-Cell Mapping

| # | Requirement |
|---|-------------|
| 1 | F-7.6       |
| 2 | F-7.6       |
| 3 | F-7.6       |

1. **#1** — Grid cell_size=1.0, origin=(0,0,0), pos=(0.5,0.5,0.5)
   - **Expected:** CellCoord(0, 0, 0)
2. **#2** — pos at cell boundary (1.0, 0, 0)
   - **Expected:** CellCoord(1, 0, 0)
3. **#3** — pos out of bounds (−1, 0, 0)
   - **Expected:** Returns `None`

### TC-7.6.2 UniformGrid Neighbor Iteration

| # | Requirement |
|---|-------------|
| 1 | F-7.6       |
| 2 | F-7.6       |

1. **#1** — 3D grid, VonNeumann neighbors of interior cell
   - **Expected:** 6 neighbors returned
2. **#2** — 3D grid, Moore neighbors of interior cell
   - **Expected:** 26 neighbors returned

### TC-7.6.3 UniformGrid Clear

| # | Requirement |
|---|-------------|
| 1 | F-7.6       |
| 2 | F-7.6       |

1. **#1** — Set 100 cells to non-default values
   - **Expected:** 100 cells modified
2. **#2** — `grid.clear()`
   - **Expected:** All cells == `T::default()`

### TC-13.7.1 Curve Linear Evaluate

| # | Requirement |
|---|-------------|
| 1 | F-13.7      |

1. **#1** — Linear keys [(0.0, 0.0), (1.0, 10.0)], evaluate(0.5)
   - **Expected:** 5.0

### TC-13.7.2 Curve CatmullRom Through Points

| # | Requirement |
|---|-------------|
| 1 | F-13.7      |

1. **#1** — CatmullRom keys at t=0.0, 0.33, 0.66, 1.0
   - **Expected:** evaluate(0.33) == key value (passes through)

### TC-13.7.3 Curve Bezier Endpoint

| # | Requirement |
|---|-------------|
| 1 | F-13.7      |

1. **#1** — Bezier key at t=0.0 value=0, t=1.0 value=10
   - **Expected:** evaluate(0.0) == 0, evaluate(1.0) == 10

### TC-13.7.4 Curve Step Hold

| # | Requirement |
|---|-------------|
| 1 | F-13.7      |
| 2 | F-13.7      |

1. **#1** — Step keys [(0.0, 5), (0.5, 10)], evaluate(0.25)
   - **Expected:** 5 (holds until next key)
2. **#2** — evaluate(0.75)
   - **Expected:** 10

### TC-13.7.5 Curve Derivative

| # | Requirement |
|---|-------------|
| 1 | F-13.7      |

1. **#1** — Linear keys [(0.0, 0.0), (1.0, 10.0)], derivative(0.5)
   - **Expected:** 10.0 (constant slope)

### TC-9.6.1 SpringDamper Critically Damped

| # | Requirement |
|---|-------------|
| 1 | F-9.6       |

1. **#1** — damping=1.0, target=10, current=0, 100 ticks
   - **Expected:** Converges to 10 without overshoot

### TC-9.6.2 SpringDamper Underdamped

| # | Requirement |
|---|-------------|
| 1 | F-9.6       |

1. **#1** — damping=0.3, target=10, current=0, 200 ticks
   - **Expected:** Oscillates past 10 before settling

### TC-9.6.3 SpringDamper Overdamped

| # | Requirement |
|---|-------------|
| 1 | F-9.6       |

1. **#1** — damping=3.0, target=10, current=0, 200 ticks
   - **Expected:** Approaches 10 without overshoot (sluggish)

### TC-9.6.4 SpringDamper Zero Dt

| # | Requirement |
|---|-------------|
| 1 | F-9.6       |

1. **#1** — tick(dt=0.0, target=10, current=5, velocity=1)
   - **Expected:** position=5, velocity=1 (no change)

### TC-9.6.5 SpringDamper Vec3 and Quat

| # | Requirement |
|---|-------------|
| 1 | F-9.6       |
| 2 | F-9.6       |

1. **#1** — SpringDamper<Vec3> with target=(10,0,0)
   - **Expected:** Converges to (10,0,0)
2. **#2** — SpringDamper<Quat> with target=90 deg Y
   - **Expected:** Converges to 90 deg Y rotation

<!-- Review feedback: StatModifier tests move to
     data-systems/attributes-effects-test-cases.md -->

### TC-13.10.1 StatModifier Flat Only

| # | Requirement |
|---|-------------|
| 1 | F-13.10     |

1. **#1** — Base=100, Flat(+10), Flat(+5)
   - **Expected:** evaluate(100, 0, 999) = 115

### TC-13.10.2 StatModifier Percent Only

| # | Requirement |
|---|-------------|
| 1 | F-13.10     |

1. **#1** — Base=100, Percent(0.5)
   - **Expected:** evaluate(100, 0, 999) = 150

### TC-13.10.3 StatModifier Override

| # | Requirement |
|---|-------------|
| 1 | F-13.10     |

1. **#1** — Base=100, Flat(+50), Override(200)
   - **Expected:** evaluate(100, 0, 999) = 200

### TC-13.10.4 StatModifier Combined Ordering

| # | Requirement |
|---|-------------|
| 1 | F-13.10     |

1. **#1** — Base=100, Flat(+10), Percent(0.5)
   - **Expected:** 100 + 10 = 110, then 110 * 1.5 = 165

### TC-13.10.5 StatModifier Remove By Source

| # | Requirement |
|---|-------------|
| 1 | F-13.10     |
| 2 | F-13.10     |

1. **#1** — 3 modifiers from source A, 2 from source B
   - **Expected:** 5 total
2. **#2** — `remove_by_source(A)`
   - **Expected:** 2 remaining (source B only)

### TC-13.10.6 StatModifier Clamp

| # | Requirement |
|---|-------------|
| 1 | F-13.10     |

1. **#1** — Base=100, Flat(+999), min=0, max=200
   - **Expected:** evaluate = 200 (clamped)

### TC-13.6.1 ConditionExpr And

| # | Requirement |
|---|-------------|
| 1 | F-13.6      |
| 2 | F-13.6      |

1. **#1** — And([true, true, true])
   - **Expected:** evaluate = true
2. **#2** — And([true, false, true])
   - **Expected:** evaluate = false

### TC-13.6.2 ConditionExpr Or

| # | Requirement |
|---|-------------|
| 1 | F-13.6      |
| 2 | F-13.6      |

1. **#1** — Or([false, true, false])
   - **Expected:** evaluate = true
2. **#2** — Or([false, false, false])
   - **Expected:** evaluate = false

### TC-13.6.3 ConditionExpr Not

| # | Requirement |
|---|-------------|
| 1 | F-13.6      |

1. **#1** — Not(Leaf(always_true))
   - **Expected:** evaluate = false

### TC-13.6.4 ConditionExpr Nested

| # | Requirement |
|---|-------------|
| 1 | F-13.6      |

1. **#1** — And([Or([true, false]), Not(false)])
   - **Expected:** evaluate = true

### TC-13.6.5 ConditionExpr Leaf Count

| # | Requirement |
|---|-------------|
| 1 | F-13.6      |

1. **#1** — And([Leaf(1), Or([Leaf(2), Leaf(3)])])
   - **Expected:** leaf_count() = 3

### TC-7.3.1 FrameBudget Priority Execution

| # | Requirement |
|---|-------------|
| 1 | F-7.3       |

1. **#1** — Enqueue low(priority=0) and high(priority=10), budget=100us
   - **Expected:** High executes first

### TC-7.3.2 FrameBudget Exhaustion

| # | Requirement |
|---|-------------|
| 1 | F-7.3       |
| 2 | F-7.3       |

1. **#1** — Budget=50us, 3 items each 30us
   - **Expected:** 1 executes, 2 deferred
2. **#2** — `deferred_count()`
   - **Expected:** Returns 2

### TC-7.3.3 FrameBudget Reset

| # | Requirement |
|---|-------------|
| 1 | F-7.3       |

1. **#1** — Execute partial, reset
   - **Expected:** elapsed_us = 0, deferred items remain

### TC-7.3.4 FrameBudget Zero Budget

| # | Requirement |
|---|-------------|
| 1 | F-7.3       |

1. **#1** — Budget=0, enqueue 5 items
   - **Expected:** 0 executed, 5 deferred

### TC-15.2.1 FalloffCurve Linear

| # | Requirement |
|---|-------------|
| 1 | F-15.2      |
| 2 | F-15.2      |
| 3 | F-15.2      |

1. **#1** — Linear max_range=100, evaluate(0)
   - **Expected:** 1.0
2. **#2** — Linear max_range=100, evaluate(50)
   - **Expected:** 0.5
3. **#3** — Linear max_range=100, evaluate(100)
   - **Expected:** 0.0

### TC-15.2.2 FalloffCurve Quadratic

| # | Requirement |
|---|-------------|
| 1 | F-15.2      |
| 2 | F-15.2      |

1. **#1** — Quadratic max_range=100, evaluate(0)
   - **Expected:** 1.0
2. **#2** — Quadratic: verify inverse-square law
   - **Expected:** 1/(1 + k*d^2) matches expected

### TC-15.2.3 FalloffCurve Exponential

| # | Requirement |
|---|-------------|
| 1 | F-15.2      |

1. **#1** — Exponential decay_rate=0.1, evaluate(10)
   - **Expected:** e^(−1.0) ~= 0.368

### TC-15.2.4 FalloffCurve Custom

| # | Requirement |
|---|-------------|
| 1 | F-15.2      |

1. **#1** — Custom with inner linear Curve, evaluate(0.5 * max_range)
   - **Expected:** Delegates to curve, returns curve result

### TC-15.2.5 FalloffCurve Beyond Max Range

| # | Requirement |
|---|-------------|
| 1 | F-15.2      |

1. **#1** — Any variant, evaluate(max_range + 1)
   - **Expected:** 0.0

### TC-2.9.1 PlatformTier Ordering

| # | Requirement |
|---|-------------|
| 1 | F-2.9       |

1. **#1** — Compare Mobile < Switch < Desktop < HighEnd
   - **Expected:** All comparisons true

### TC-2.9.2 PlatformTier Meets

| # | Requirement |
|---|-------------|
| 1 | F-2.9       |
| 2 | F-2.9       |
| 3 | F-2.9       |

1. **#1** — Desktop.meets(Desktop)
   - **Expected:** true
2. **#2** — Desktop.meets(HighEnd)
   - **Expected:** false
3. **#3** — HighEnd.meets(Mobile)
   - **Expected:** true

### TC-2.9.3 PlatformTier Recommended Values

| # | Requirement |
|---|-------------|
| 1 | F-2.9       |

1. **#1** — max_draw_distance for each tier
   - **Expected:** Monotonically increasing Mobile < Switch < Desktop < HighEnd

### TC-12.5.1 CompressionCodec None

| # | Requirement |
|---|-------------|
| 1 | F-12.5      |

1. **#1** — Compress 1 KiB data with None
   - **Expected:** Output equals input

### TC-12.5.2 CompressionCodec Lz4 Roundtrip

| # | Requirement |
|---|-------------|
| 1 | F-12.5      |
| 2 | F-12.5      |

1. **#1** — Compress 1 KiB data with Lz4
   - **Expected:** Compressed bytes produced
2. **#2** — Decompress
   - **Expected:** Output equals original input

### TC-12.5.3 CompressionCodec Zstd Roundtrip

| # | Requirement |
|---|-------------|
| 1 | F-12.5      |
| 2 | F-12.5      |

1. **#1** — Compress 1 KiB data with Zstd
   - **Expected:** Compressed bytes produced
2. **#2** — Decompress
   - **Expected:** Output equals original input

### TC-12.5.4 CompressionCodec Empty Input

| # | Requirement |
|---|-------------|
| 1 | F-12.5      |

1. **#1** — Compress empty slice with each codec
   - **Expected:** 0 bytes written (or valid empty frame)

### TC-12.5.5 CompressionCodec Large Input

| # | Requirement |
|---|-------------|
| 1 | F-12.5      |
| 2 | F-12.5      |

1. **#1** — Compress 1 MiB data with Lz4, decompress
   - **Expected:** Round-trip fidelity
2. **#2** — Compress 1 MiB data with Zstd, decompress
   - **Expected:** Round-trip fidelity

### TC-13.6.6 ConditionalGraph Add and Query

| # | Requirement |
|---|-------------|
| 1 | F-13.6      |
| 2 | F-13.6      |

1. **#1** — Add nodes A, B, C; add edges A->B, B->C
   - **Expected:** Graph has 3 nodes, 2 edges
2. **#2** — `topological_iter()`
   - **Expected:** Order: A, B, C

### TC-13.6.7 ConditionalGraph Reachable Filtering

| # | Requirement |
|---|-------------|
| 1 | F-13.6      |

1. **#1** — Edge A->B condition=true, edge A->C condition=false
   - **Expected:** `reachable_from(A)` returns [B] only

### TC-13.6.8 ConditionalGraph Cycle Rejection

| # | Requirement |
|---|-------------|
| 1 | F-13.6      |

1. **#1** — Attempt to add edge creating cycle A->B->A
   - **Expected:** Error (cycle detected)

### TC-7.6.4 DecayingStore Insert and Tick

| # | Requirement |
|---|-------------|
| 1 | F-7.6       |

1. **#1** — Insert entry at strength 1.0, decay_rate=0.1, tick dt=5.0
   - **Expected:** strength = 0.5

### TC-7.6.5 DecayingStore Below Threshold Removal

| # | Requirement |
|---|-------------|
| 1 | F-7.6       |

1. **#1** — Threshold=0.1, entry at strength 0.05 after tick
   - **Expected:** Entry removed, tick returns 1

### TC-7.6.6 DecayingStore Refresh

| # | Requirement |
|---|-------------|
| 1 | F-7.6       |

1. **#1** — Entry at strength 0.3, refresh matching predicate
   - **Expected:** strength restored to 1.0

### TC-7.6.7 DecayingStore Strongest

| # | Requirement |
|---|-------------|
| 1 | F-7.6       |

1. **#1** — 3 entries at strength 0.2, 0.8, 0.5
   - **Expected:** `strongest()` returns entry with 0.8

### TC-4.6.1 ConnectivityAnalyzer Fully Connected

| # | Requirement |
|---|-------------|
| 1 | F-4.6       |

1. **#1** — 10 entities all connected, 1 anchor
   - **Expected:** connected = 10, disconnected = []

### TC-4.6.2 ConnectivityAnalyzer Anchor Removed

| # | Requirement |
|---|-------------|
| 1 | F-4.6       |
| 2 | F-4.6       |

1. **#1** — 10 entities, 1 anchor, remove anchor's connection
   - **Expected:** Cluster detaches
2. **#2** — Analyze
   - **Expected:** disconnected contains detached cluster

### TC-4.6.3 ConnectivityAnalyzer Island Detection

| # | Requirement |
|---|-------------|
| 1 | F-4.6       |

1. **#1** — 2 disconnected groups, anchor in group 1
   - **Expected:** Group 2 in disconnected list

### TC-4.6.4 ConnectivityAnalyzer Empty Input

| # | Requirement |
|---|-------------|
| 1 | F-4.6       |

1. **#1** — Empty entity list, empty anchors
   - **Expected:** connected=[], disconnected=[]

### TC-13.16.1 TaggedLookupTable Insert Get

| # | Requirement |
|---|-------------|
| 1 | F-13.16     |
| 2 | F-13.16     |

1. **#1** — Insert ("fire", "burn", weight=1.0)
   - **Expected:** Inserted
2. **#2** — `get("fire")`
   - **Expected:** Returns [WeightedEntry("burn", 1.0)]

### TC-13.16.2 TaggedLookupTable Weighted Selection

| # | Requirement |
|---|-------------|
| 1 | F-13.16     |
| 2 | F-13.16     |

1. **#1** — Insert ("loot", "gold", 9.0) and ("loot", "gem", 1.0)
   - **Expected:** Both inserted
2. **#2** — `select_weighted("loot")` 10,000 times
   - **Expected:** ~90% gold, ~10% gem (within 5%)

### TC-13.16.3 TaggedLookupTable Missing Key

| # | Requirement |
|---|-------------|
| 1 | F-13.16     |

1. **#1** — `get("nonexistent")`
   - **Expected:** Returns `None`

<!-- Review feedback: LiveOpsResource tests move to
     game-framework or networking design -->

### TC-13.23.1 LiveOpsResource UpToDate

| # | Requirement |
|---|-------------|
| 1 | F-13.23     |

1. **#1** — Local version=5, remote version=5
   - **Expected:** `poll()` returns `UpToDate`

### TC-13.23.2 LiveOpsResource Updated

| # | Requirement |
|---|-------------|
| 1 | F-13.23     |

1. **#1** — Local version=5, remote version=6
   - **Expected:** `poll()` returns `Updated(Versioned{version:6, ..})`

### TC-13.23.3 LiveOpsResource Error

| # | Requirement |
|---|-------------|
| 1 | F-13.23     |

1. **#1** — Network failure during poll
   - **Expected:** `poll()` returns `Error(message)`

### TC-13.23.4 LiveOpsResource Force Refresh

| # | Requirement |
|---|-------------|
| 1 | F-13.23     |

1. **#1** — `force_refresh()` within poll interval
   - **Expected:** Fetches immediately (bypasses interval)

### TC-15.3.1 GraphCompiler Topological Sort

| # | Requirement |
|---|-------------|
| 1 | F-15.3      |

1. **#1** — Nodes A->B->C (linear chain)
   - **Expected:** Compile output in topological order

### TC-15.3.2 GraphCompiler Type Mismatch

| # | Requirement |
|---|-------------|
| 1 | F-15.3      |

1. **#1** — Connect Float output to Texture2D input
   - **Expected:** `CompileError` with node_id and message

### TC-15.3.3 GraphCompiler Dead Code Elimination

| # | Requirement |
|---|-------------|
| 1 | F-15.3      |

1. **#1** — Graph with unreachable node D
   - **Expected:** GLSL output excludes D's code

### TC-15.3.4 GraphCompiler GLSL Output

| # | Requirement |
|---|-------------|
| 1 | F-15.3      |

1. **#1** — Simple add-node graph (Float + Float)
   - **Expected:** Valid GLSL source string

### TC-12.5.6 VirtualResourceStreamer Feedback

| # | Requirement |
|---|-------------|
| 1 | F-12.5      |

1. **#1** — `process_feedback([page_A, page_B])`
   - **Expected:** page_A and page_B enqueued for load

### TC-12.5.7 VirtualResourceStreamer Budget

| # | Requirement |
|---|-------------|
| 1 | F-12.5      |
| 2 | F-12.5      |

1. **#1** — Budget = 100 MB, load pages totaling 90 MB
   - **Expected:** All loaded
2. **#2** — Request page pushing total to 110 MB
   - **Expected:** LRU page evicted first

### TC-12.5.8 VirtualResourceStreamer LRU Eviction

| # | Requirement |
|---|-------------|
| 1 | F-12.5      |
| 2 | F-12.5      |

1. **#1** — Pages A, B, C loaded (A oldest)
   - **Expected:** A is LRU candidate
2. **#2** — Evict under budget pressure
   - **Expected:** A evicted, state = NonResident

### TC-12.5.9 VirtualResourceStreamer Commit Uploads

| # | Requirement |
|---|-------------|
| 1 | F-12.5      |

1. **#1** — Page load completes, `commit_uploads()`
   - **Expected:** Page state transitions to Resident

## Integration Tests

### TC-1.7.4.I1 Handle ECS Interop

| # | Requirement |
|---|-------------|
| 1 | F-1.1.11    |
| 2 | F-1.7.4     |

1. **#1** — Create entities via `Handle<EntityMarker>`, store in `HandleMap`
   - **Expected:** Handles valid
2. **#2** — Access from different system
   - **Expected:** Cross-system handle validity confirmed

### TC-7.6.I1 Grid Plus Spatial Index

| # | Requirement |
|---|-------------|
| 1 | F-7.6       |
| 2 | F-7.6       |

1. **#1** — Populate `UniformGrid` from `SpatialQuery` results
   - **Expected:** Grid populated
2. **#2** — Verify cell contents
   - **Expected:** Match spatial query positions

### TC-13.10.I1 Stat Pipeline End-to-End

| # | Requirement |
|---|-------------|
| 1 | F-13.10     |

1. **#1** — Base=100, ability Flat(+20), item Percent(0.25), buff Override(none)
   - **Expected:** evaluate = (100+20)*1.25 = 150

<!-- Review feedback: VirtualStreamer tests move to
     content-pipeline design -->

### TC-12.5.I1 Virtual Streamer Plus Compio

| # | Requirement |
|---|-------------|
| 1 | F-12.5      |
| 2 | F-12.5      |

1. **#1** — Load pages via compio on main thread
   - **Expected:** Pages loaded, delivered via channel
2. **#2** — Verify residency tracking
   - **Expected:** All loaded pages marked Resident

## Benchmarks

No explicit performance requirements are defined for core algorithms. Performance targets are
inherited from the consuming domain designs (see R-1.7.4, R-1.9.4a, R-1.1.11a).
