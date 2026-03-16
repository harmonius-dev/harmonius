# Shared Primitives Test Cases

Companion test cases for [shared-primitives.md](shared-primitives.md).

## Unit Tests

### TC-1.7.4.1 Handle Insert Get Remove

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `map.insert(42)` | Returns Handle(idx=0, gen=0) | F-1.1.11 |
| 2 | `map.get(handle)` | Returns `Some(&42)` | F-1.7.4 |
| 3 | `map.remove(handle)` | Returns `Some(42)` | F-1.7.4 |
| 4 | `map.get(handle)` after remove | Returns `None` (stale gen) | F-1.7.4 |

### TC-1.7.4.2 Handle Generation Overflow

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Insert/remove at same slot until gen = u32::MAX | Gen wraps to 0 | F-1.7.4 |
| 2 | Insert new value at wrapped slot | Handle(idx, gen=0) valid | F-1.7.4 |

### TC-1.7.4.3 Handle Iteration Skips Free

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Insert 10, remove indices 2, 5, 8 | 7 entries remain | F-1.7.5 |
| 2 | `map.iter()` | Visits exactly 7 entries | F-1.7.5 |

### TC-1.7.4.4 Handle Type Safety

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `Handle<Mesh>` used in `HandleMap<Texture>` | Compile error (type mismatch) | F-1.7.4 |

### TC-7.6.1 UniformGrid World-to-Cell Mapping

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Grid cell_size=1.0, origin=(0,0,0), pos=(0.5,0.5,0.5) | CellCoord(0, 0, 0) | F-7.6 |
| 2 | pos at cell boundary (1.0, 0, 0) | CellCoord(1, 0, 0) | F-7.6 |
| 3 | pos out of bounds (−1, 0, 0) | Returns `None` | F-7.6 |

### TC-7.6.2 UniformGrid Neighbor Iteration

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3D grid, VonNeumann neighbors of interior cell | 6 neighbors returned | F-7.6 |
| 2 | 3D grid, Moore neighbors of interior cell | 26 neighbors returned | F-7.6 |

### TC-7.6.3 UniformGrid Clear

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set 100 cells to non-default values | 100 cells modified | F-7.6 |
| 2 | `grid.clear()` | All cells == `T::default()` | F-7.6 |

### TC-13.7.1 Curve Linear Evaluate

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Linear keys [(0.0, 0.0), (1.0, 10.0)], evaluate(0.5) | 5.0 | F-13.7 |

### TC-13.7.2 Curve CatmullRom Through Points

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | CatmullRom keys at t=0.0, 0.33, 0.66, 1.0 | evaluate(0.33) == key value (passes through) | F-13.7 |

### TC-13.7.3 Curve Bezier Endpoint

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Bezier key at t=0.0 value=0, t=1.0 value=10 | evaluate(0.0) == 0, evaluate(1.0) == 10 | F-13.7 |

### TC-13.7.4 Curve Step Hold

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Step keys [(0.0, 5), (0.5, 10)], evaluate(0.25) | 5 (holds until next key) | F-13.7 |
| 2 | evaluate(0.75) | 10 | F-13.7 |

### TC-13.7.5 Curve Derivative

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Linear keys [(0.0, 0.0), (1.0, 10.0)], derivative(0.5) | 10.0 (constant slope) | F-13.7 |

### TC-9.6.1 SpringDamper Critically Damped

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | damping=1.0, target=10, current=0, 100 ticks | Converges to 10 without overshoot | F-9.6 |

### TC-9.6.2 SpringDamper Underdamped

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | damping=0.3, target=10, current=0, 200 ticks | Oscillates past 10 before settling | F-9.6 |

### TC-9.6.3 SpringDamper Overdamped

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | damping=3.0, target=10, current=0, 200 ticks | Approaches 10 without overshoot (sluggish) | F-9.6 |

### TC-9.6.4 SpringDamper Zero Dt

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | tick(dt=0.0, target=10, current=5, velocity=1) | position=5, velocity=1 (no change) | F-9.6 |

### TC-9.6.5 SpringDamper Vec3 and Quat

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | SpringDamper<Vec3> with target=(10,0,0) | Converges to (10,0,0) | F-9.6 |
| 2 | SpringDamper<Quat> with target=90 deg Y | Converges to 90 deg Y rotation | F-9.6 |

### TC-13.10.1 StatModifier Flat Only

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Base=100, Flat(+10), Flat(+5) | evaluate(100, 0, 999) = 115 | F-13.10 |

### TC-13.10.2 StatModifier Percent Only

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Base=100, Percent(0.5) | evaluate(100, 0, 999) = 150 | F-13.10 |

### TC-13.10.3 StatModifier Override

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Base=100, Flat(+50), Override(200) | evaluate(100, 0, 999) = 200 | F-13.10 |

### TC-13.10.4 StatModifier Combined Ordering

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Base=100, Flat(+10), Percent(0.5) | 100 + 10 = 110, then 110 * 1.5 = 165 | F-13.10 |

### TC-13.10.5 StatModifier Remove By Source

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3 modifiers from source A, 2 from source B | 5 total | F-13.10 |
| 2 | `remove_by_source(A)` | 2 remaining (source B only) | F-13.10 |

### TC-13.10.6 StatModifier Clamp

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Base=100, Flat(+999), min=0, max=200 | evaluate = 200 (clamped) | F-13.10 |

### TC-13.6.1 ConditionExpr And

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | And([true, true, true]) | evaluate = true | F-13.6 |
| 2 | And([true, false, true]) | evaluate = false | F-13.6 |

### TC-13.6.2 ConditionExpr Or

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Or([false, true, false]) | evaluate = true | F-13.6 |
| 2 | Or([false, false, false]) | evaluate = false | F-13.6 |

### TC-13.6.3 ConditionExpr Not

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Not(Leaf(always_true)) | evaluate = false | F-13.6 |

### TC-13.6.4 ConditionExpr Nested

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | And([Or([true, false]), Not(false)]) | evaluate = true | F-13.6 |

### TC-13.6.5 ConditionExpr Leaf Count

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | And([Leaf(1), Or([Leaf(2), Leaf(3)])]) | leaf_count() = 3 | F-13.6 |

### TC-7.3.1 FrameBudget Priority Execution

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Enqueue low(priority=0) and high(priority=10), budget=100us | High executes first | F-7.3 |

### TC-7.3.2 FrameBudget Exhaustion

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Budget=50us, 3 items each 30us | 1 executes, 2 deferred | F-7.3 |
| 2 | `deferred_count()` | Returns 2 | F-7.3 |

### TC-7.3.3 FrameBudget Reset

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Execute partial, reset | elapsed_us = 0, deferred items remain | F-7.3 |

### TC-7.3.4 FrameBudget Zero Budget

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Budget=0, enqueue 5 items | 0 executed, 5 deferred | F-7.3 |

### TC-15.2.1 FalloffCurve Linear

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Linear max_range=100, evaluate(0) | 1.0 | F-15.2 |
| 2 | Linear max_range=100, evaluate(50) | 0.5 | F-15.2 |
| 3 | Linear max_range=100, evaluate(100) | 0.0 | F-15.2 |

### TC-15.2.2 FalloffCurve Quadratic

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Quadratic max_range=100, evaluate(0) | 1.0 | F-15.2 |
| 2 | Quadratic: verify inverse-square law | 1/(1 + k*d^2) matches expected | F-15.2 |

### TC-15.2.3 FalloffCurve Exponential

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Exponential decay_rate=0.1, evaluate(10) | e^(−1.0) ~= 0.368 | F-15.2 |

### TC-15.2.4 FalloffCurve Custom

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Custom with inner linear Curve, evaluate(0.5 * max_range) | Delegates to curve, returns curve result | F-15.2 |

### TC-15.2.5 FalloffCurve Beyond Max Range

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Any variant, evaluate(max_range + 1) | 0.0 | F-15.2 |

### TC-2.9.1 PlatformTier Ordering

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compare Mobile < Switch < Desktop < HighEnd | All comparisons true | F-2.9 |

### TC-2.9.2 PlatformTier Meets

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Desktop.meets(Desktop) | true | F-2.9 |
| 2 | Desktop.meets(HighEnd) | false | F-2.9 |
| 3 | HighEnd.meets(Mobile) | true | F-2.9 |

### TC-2.9.3 PlatformTier Recommended Values

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | max_draw_distance for each tier | Monotonically increasing Mobile < Switch < Desktop < HighEnd | F-2.9 |

### TC-12.5.1 CompressionCodec None

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compress 1 KiB data with None | Output equals input | F-12.5 |

### TC-12.5.2 CompressionCodec Lz4 Roundtrip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compress 1 KiB data with Lz4 | Compressed bytes produced | F-12.5 |
| 2 | Decompress | Output equals original input | F-12.5 |

### TC-12.5.3 CompressionCodec Zstd Roundtrip

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compress 1 KiB data with Zstd | Compressed bytes produced | F-12.5 |
| 2 | Decompress | Output equals original input | F-12.5 |

### TC-12.5.4 CompressionCodec Empty Input

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compress empty slice with each codec | 0 bytes written (or valid empty frame) | F-12.5 |

### TC-12.5.5 CompressionCodec Large Input

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compress 1 MiB data with Lz4, decompress | Round-trip fidelity | F-12.5 |
| 2 | Compress 1 MiB data with Zstd, decompress | Round-trip fidelity | F-12.5 |

### TC-13.6.6 ConditionalGraph Add and Query

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add nodes A, B, C; add edges A->B, B->C | Graph has 3 nodes, 2 edges | F-13.6 |
| 2 | `topological_iter()` | Order: A, B, C | F-13.6 |

### TC-13.6.7 ConditionalGraph Reachable Filtering

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Edge A->B condition=true, edge A->C condition=false | `reachable_from(A)` returns [B] only | F-13.6 |

### TC-13.6.8 ConditionalGraph Cycle Rejection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Attempt to add edge creating cycle A->B->A | Error (cycle detected) | F-13.6 |

### TC-7.6.4 DecayingStore Insert and Tick

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Insert entry at strength 1.0, decay_rate=0.1, tick dt=5.0 | strength = 0.5 | F-7.6 |

### TC-7.6.5 DecayingStore Below Threshold Removal

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Threshold=0.1, entry at strength 0.05 after tick | Entry removed, tick returns 1 | F-7.6 |

### TC-7.6.6 DecayingStore Refresh

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Entry at strength 0.3, refresh matching predicate | strength restored to 1.0 | F-7.6 |

### TC-7.6.7 DecayingStore Strongest

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3 entries at strength 0.2, 0.8, 0.5 | `strongest()` returns entry with 0.8 | F-7.6 |

### TC-4.6.1 ConnectivityAnalyzer Fully Connected

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10 entities all connected, 1 anchor | connected = 10, disconnected = [] | F-4.6 |

### TC-4.6.2 ConnectivityAnalyzer Anchor Removed

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10 entities, 1 anchor, remove anchor's connection | Cluster detaches | F-4.6 |
| 2 | Analyze | disconnected contains detached cluster | F-4.6 |

### TC-4.6.3 ConnectivityAnalyzer Island Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 2 disconnected groups, anchor in group 1 | Group 2 in disconnected list | F-4.6 |

### TC-4.6.4 ConnectivityAnalyzer Empty Input

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Empty entity list, empty anchors | connected=[], disconnected=[] | F-4.6 |

### TC-13.16.1 TaggedLookupTable Insert Get

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Insert ("fire", "burn", weight=1.0) | Inserted | F-13.16 |
| 2 | `get("fire")` | Returns [WeightedEntry("burn", 1.0)] | F-13.16 |

### TC-13.16.2 TaggedLookupTable Weighted Selection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Insert ("loot", "gold", 9.0) and ("loot", "gem", 1.0) | Both inserted | F-13.16 |
| 2 | `select_weighted("loot")` 10,000 times | ~90% gold, ~10% gem (within 5%) | F-13.16 |

### TC-13.16.3 TaggedLookupTable Missing Key

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `get("nonexistent")` | Returns `None` | F-13.16 |

### TC-13.23.1 LiveOpsResource UpToDate

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Local version=5, remote version=5 | `poll()` returns `UpToDate` | F-13.23 |

### TC-13.23.2 LiveOpsResource Updated

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Local version=5, remote version=6 | `poll()` returns `Updated(Versioned{version:6, ..})` | F-13.23 |

### TC-13.23.3 LiveOpsResource Error

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Network failure during poll | `poll()` returns `Error(message)` | F-13.23 |

### TC-13.23.4 LiveOpsResource Force Refresh

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `force_refresh()` within poll interval | Fetches immediately (bypasses interval) | F-13.23 |

### TC-15.3.1 GraphCompiler Topological Sort

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Nodes A->B->C (linear chain) | Compile output in topological order | F-15.3 |

### TC-15.3.2 GraphCompiler Type Mismatch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Connect Float output to Texture2D input | `CompileError` with node_id and message | F-15.3 |

### TC-15.3.3 GraphCompiler Dead Code Elimination

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph with unreachable node D | HLSL output excludes D's code | F-15.3 |

### TC-15.3.4 GraphCompiler HLSL Output

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Simple add-node graph (Float + Float) | Valid HLSL source string | F-15.3 |

### TC-12.5.6 VirtualResourceStreamer Feedback

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | `process_feedback([page_A, page_B])` | page_A and page_B enqueued for load | F-12.5 |

### TC-12.5.7 VirtualResourceStreamer Budget

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Budget = 100 MB, load pages totaling 90 MB | All loaded | F-12.5 |
| 2 | Request page pushing total to 110 MB | LRU page evicted first | F-12.5 |

### TC-12.5.8 VirtualResourceStreamer LRU Eviction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Pages A, B, C loaded (A oldest) | A is LRU candidate | F-12.5 |
| 2 | Evict under budget pressure | A evicted, state = NonResident | F-12.5 |

### TC-12.5.9 VirtualResourceStreamer Commit Uploads

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Page load completes, `commit_uploads()` | Page state transitions to Resident | F-12.5 |

## Integration Tests

### TC-1.7.4.I1 Handle ECS Interop

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create entities via `Handle<EntityMarker>`, store in `HandleMap` | Handles valid | F-1.1.11 |
| 2 | Access from different system | Cross-system handle validity confirmed | F-1.7.4 |

### TC-7.6.I1 Grid Plus Spatial Index

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Populate `UniformGrid` from `SpatialQuery` results | Grid populated | F-7.6 |
| 2 | Verify cell contents | Match spatial query positions | F-7.6 |

### TC-13.10.I1 Stat Pipeline End-to-End

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Base=100, ability Flat(+20), item Percent(0.25), buff Override(none) | evaluate = (100+20)*1.25 = 150 | F-13.10 |

### TC-12.5.I1 Virtual Streamer Plus IoReactor

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load pages via platform I/O backend | Pages loaded | F-12.5 |
| 2 | Verify residency tracking | All loaded pages marked Resident | F-12.5 |

## Benchmarks

No explicit performance requirements are defined for shared primitives. Performance targets are
inherited from the consuming domain designs (see R-1.7.4, R-1.9.4a, R-1.1.11a).
