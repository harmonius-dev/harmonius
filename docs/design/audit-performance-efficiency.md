# Performance & Efficiency Audit Report

## Methodology

All design files under `docs/design/` were read systematically. Each finding identifies the source
file, section, issue, estimated impact, and recommended optimization.

---

## 1. Algorithmic Efficiency

### 1.1 Octree `remove()` is O(n) Linear Scan

- **File:** `core-runtime/spatial-index.md`, Octree API
- **Issue:** `OctreeIndex::remove(entity: Entity)` takes only an `Entity` with no cell hint. The
  implementation must search all cells to find and remove the entity, making it O(total entities).
  Compare with `update()` which correctly takes `old_cell: OctreeCell`.
- **Impact:** 0.1-1.0 ms per frame if many entities are despawned from the octree (e.g.,
  destruction, despawn waves).
- **Recommendation:** Change signature to `remove(entity: Entity, cell: OctreeCell)` to match the
  `update()` pattern. The `SpatialMembership` component already stores
  `octree_cell: Option<OctreeCell>`, so the caller has this information.

### 1.2 Octree/Grid `query_range()` Returns `Vec`

- **File:** `core-runtime/spatial-index.md`, Octree and UniformGrid API
- **Issue:** `OctreeIndex::query_range()`, `query_bounds()`, `UniformGrid::query_radius()`, and
  `query_rect()` all return `Vec<Entity>`, allocating on every call. These are called per-frame by
  networking, AI, and gameplay systems.
- **Impact:** Hundreds of heap allocations per frame. At 1000+ AI agents each doing neighbor
  queries, this is 0.5-2.0 ms of allocation overhead.
- **Recommendation:** Accept a `&mut Vec<Entity>` output parameter or return a callback-based
  iterator. Callers can reuse a pre-allocated buffer across frames.

### 1.3 SpatialQuery Trait Returns `Vec<SpatialHit>`

- **File:** `core-runtime/spatial-index.md`, SpatialQuery trait
- **Issue:** Every method on the `SpatialQuery` trait (`query()`, `ray_cast()`, `overlap_aabb()`,
  `overlap_sphere()`, `frustum_query()`, `k_nearest()`) returns `Vec<SpatialHit>`. This trait is the
  primary interop contract consumed by physics, rendering, networking, AI, audio, and gameplay.
  Every spatial query allocates.
- **Impact:** Potentially thousands of allocations per frame across all consumers. Major GC
  pressure. Estimated 1-3 ms of allocation overhead in complex scenes.
- **Recommendation:** Provide arena-allocated or callback-based variants. Add
  `query_into(shape, config, &mut Vec<SpatialHit>)` and `query_visitor(shape, config, &mut F)` where
  `F: FnMut(SpatialHit) -> bool` methods to the trait.

### 1.4 BVH `batch_insert()` Returns `Vec<BvhHandle>`

- **File:** `core-runtime/spatial-index.md`, BVH API
- **Issue:** `BvhIndex::batch_insert()` returns `Vec<BvhHandle>`. When spawning many entities (scene
  load, destruction), this allocates a potentially large vector.
- **Impact:** One-off allocation during spawn bursts. Low per-frame impact but causes hitches during
  level transitions. 0.1-0.5 ms per batch.
- **Recommendation:** Accept a pre-allocated `&mut Vec<BvhHandle>` or return handles via a
  caller-provided slice.

### 1.5 Destruction Structural Analysis is

  Connectivity Traversal

- **File:** `game-framework/destruction.md`, StructuralAnalysisSystem
- **Issue:** The `StructuralAnalysisSystem` traverses the connectivity graph of fragment entities to
  find unsupported fragments. With many simultaneous destructions (e.g., building collapse with 100+
  fragments), the connectivity traversal over ECS entities involves random-access entity lookups.
- **Impact:** 0.5-2.0 ms for large destruction events with 100+ fragments per object.
- **Recommendation:** Cache the connectivity graph as a contiguous adjacency list (not ECS entity
  traversal) within the `FractureAsset`. Use union-find for connected component detection, which is
  nearly O(n) with path compression.

### 1.6 Plugin Dependency Graph Uses HashMap for Edges

- **File:** `core-runtime/ecs.md`, ArchetypeGraph
- **Issue:** `ArchetypeGraph` uses `HashMap<ArchetypeId, EdgeMap>` where each `EdgeMap` contains two
  more `HashMap`s (`add_edges`, `remove_edges`). While lookups are amortized O(1), the constant
  factor of `HashMap` (hashing + probing) is high for what could be a flat array indexed by
  `ArchetypeId`.
- **Impact:** Low per-frame impact (edge traversals are infrequent after warmup), but adds overhead
  during entity spawn bursts when new archetypes are created. 0.05-0.2 ms during heavy archetype
  churn.
- **Recommendation:** Since `ArchetypeId` is a `u32` index, use `Vec<EdgeMap>` indexed by
  `ArchetypeId.0`. Within `EdgeMap`, use a sorted `SmallVec<[(ComponentId, ArchetypeId); N]>` since
  most archetypes have few edge transitions.

### 1.7 Event Channel `Vec<T>` Grows Unbounded

- **File:** `core-runtime/events-plugins.md`, EventChannel
- **Issue:** `EventChannel<T>` uses `Vec<T>` for both front and back buffers. While there is a
  `flood_threshold` for diagnostic warnings, there is no capacity limit or backpressure mechanism. A
  misbehaving system can flood an event channel causing unbounded memory growth.
- **Impact:** Unbounded memory growth under event floods. Could cause OOM or multi-millisecond
  allocation stalls.
- **Recommendation:** Add a configurable hard cap per channel. When exceeded, either drop oldest
  events or apply backpressure to the writer. Pre-allocate a reasonable capacity at registration
  time based on expected throughput.

---

## 2. Memory Efficiency

### 2.1 Archetype Uses `HashMap` for Columns

- **File:** `core-runtime/ecs.md`, Archetype struct
- **Issue:** `Archetype` stores `columns: HashMap<ComponentId, Column>` and
  `shared_values: HashMap<ComponentId, Vec<SharedValue>>`. These HashMaps are accessed on every
  component fetch during iteration. The hash lookup adds overhead to the innermost loop of every
  query.
- **Impact:** 5-15 ns overhead per component access due to hashing. For 100K entities with 3
  components, this adds 1.5-4.5 ms per full iteration pass.
- **Recommendation:** Store columns in a `Vec<Column>` sorted by `ComponentId`, with a separate
  `Vec<ComponentId>` for binary search lookups. Or use a flat array indexed by a per-archetype
  column index (computed once at archetype creation). This removes hashing from the hot path
  entirely.

### 2.2 `BvhNode` is 36 Bytes, Not 32

- **File:** `core-runtime/spatial-index.md`, BvhNode
- **Issue:** The design states "32 bytes on 64-bit platforms. Two nodes fit in one cache line." But
  the actual layout is: `Aabb` (24 bytes) + `left` (4) + `right` (4) + `parent` (4) + `is_leaf` (1)
  = 37 bytes, which with alignment rounds to 40 bytes. Two nodes do NOT fit in a 64-byte cache line.
- **Impact:** 25% more cache misses during BVH traversal than claimed. For frustum culling 1M
  entities, this could add 0.1-0.3 ms.
- **Recommendation:** Remove the `parent` field from the hot node struct (store parent links in a
  separate array used only during mutations). Pack `is_leaf` into the high bit of `left` or `right`.
  This yields 24 + 4 + 4 = 32 bytes, achieving the stated two-per-cache-line goal.

### 2.3 `LeafEntry` is Oversized

- **File:** `core-runtime/spatial-index.md`, LeafEntry struct
- **Issue:** `LeafEntry` contains `entity` (8 bytes), `aabb` (24 bytes), `layers` (4 bytes), and
  `fat_aabb` (24 bytes) = 60 bytes. Each leaf entry nearly fills an entire cache line, and the
  `fat_aabb` is only needed during update checks, not during queries.
- **Impact:** Query traversal loads fat_aabb data that is never read, wasting cache bandwidth. For
  1M entities, this wastes ~24 MB of cache traffic during full-tree traversal.
- **Recommendation:** Split into hot (query) and cold (update) data. Hot: entity + aabb + layers =
  36 bytes. Cold: fat_aabb stored in a parallel array indexed by leaf index.

### 2.4 `GlobalTransform` Stores Full `Mat4`

- **File:** `core-runtime/scene-transforms.md`, GlobalTransform struct
- **Issue:** `GlobalTransform` stores a full `Mat4` (64 bytes). For entities with only uniform scale
  (the majority), a `Vec3 + Quat + f32` (32 bytes) representation would halve the memory and improve
  cache utilization during transform propagation.
- **Impact:** 2x cache pressure for transform propagation passes. With 1M entities, this is 64 MB vs
  32 MB of data to stream through cache. Estimated 0.5-1.0 ms additional propagation time.
- **Recommendation:** Store `GlobalTransform` as `Vec3 + Quat + Vec3` (40 bytes with padding) for
  the common case. Provide `to_matrix()` for consumers that need a `Mat4`. Alternatively, use a 4x3
  affine matrix (48 bytes), which is already used by `GpuMeshletInstance` for GPU upload.

### 2.5 `ComponentRegistry` Uses `HashMap<String, ComponentId>`

- **File:** `core-runtime/ecs.md`, ComponentRegistry
- **Issue:** `ComponentRegistry` stores `name_map: HashMap<String, ComponentId>`. Each entry
  heap-allocates a `String` for the component name. With 1000+ registered component types, this
  creates 1000+ small heap allocations.
- **Impact:** Negligible per-frame impact (registry is read-only after init), but adds to startup
  allocation pressure and fragmentation.
- **Recommendation:** Use interned string IDs (`StringId` or `&'static str`) instead of owned
  `String`. Component names from derive macros are already `&'static str`.

### 2.6 Transform Component is 40 bytes

- **File:** `core-runtime/scene-transforms.md`, Transform struct
- **Issue:** `Transform` stores `Vec3` (12) + `Quat` (16) + `Vec3` (12) = 40 bytes. With 4-byte
  alignment this is fine, but it means 1.6 transforms per cache line. Entities that are purely
  positional (no rotation, uniform scale) still pay the full 40 bytes.
- **Impact:** Minor. 40 bytes is reasonable for most engines. Only notable for extremely
  entity-dense scenarios (1M+ entities).
- **Recommendation:** Consider a compressed `Transform2D` (16 bytes: Vec2 + f32 angle + f32 scale)
  for 2D game entities that do not need 3D transforms.

---

## 3. GPU Efficiency

### 3.1 Meshlet Streaming Feedback Readback Stall Risk

- **File:** `geometry/meshlets.md`, Streaming Feedback Loop
- **Issue:** The design states "The feedback buffer is read back to the CPU one frame later (no
  stall)." However, the readback mechanism is not fully specified. If the readback fence is waited
  on synchronously (even one frame delayed), there is a risk of a GPU-CPU sync point if the GPU
  falls behind.
- **Impact:** Potential 1-3 ms stall if the GPU pipeline bubble causes the readback to not be ready
  when polled.
- **Recommendation:** Use triple-buffered readback (N-2 frame data). Add a fence check with a
  fallback to use stale data if the readback is not yet complete.

### 3.2 Visibility Buffer 64-bit Atomics on Mobile

- **File:** `geometry/meshlets.md`, VisibilityBuffer
- **Issue:** The visibility buffer requires 64-bit atomic writes (`InterlockedCompareExchange64`).
  The mobile tier in the scaling table lists 4,096 max instances and 65,536 max meshlets. However,
  many mobile GPUs lack 64-bit atomic support.
- **Impact:** Complete pipeline failure on GPUs without 64-bit atomics. No fallback path defined for
  mobile.
- **Recommendation:** Define a 32-bit visibility buffer fallback that packs a reduced
  instance+triangle ID into 32 bits (e.g., 16 bits each, supporting 65,536 instances and 65,536
  triangles). This fits the mobile tier limits.

### 3.3 Virtual Shadow Maps Unbounded Page Count

- **File:** `rendering/lighting.md`, Virtual Shadow Maps (F-2.4.14)
- **Issue:** Virtual shadow maps with on-demand page allocation are listed without a page budget or
  eviction strategy in the lighting design. Page counts can grow unbounded with many shadow-casting
  lights.
- **Impact:** Unbounded GPU memory consumption. Desktop scenes with 100+ shadow-casting lights could
  exceed VRAM budgets.
- **Recommendation:** The streaming design (`content-pipeline/streaming.md`) defines a
  `VirtualResourceStreamer` with page budgets and LRU eviction. Explicitly reference this framework
  in the VSM design and define per-platform page budgets.

### 3.4 Post-Processing Bloom Chain Multiple

  Intermediate Textures

- **File:** `rendering/post-processing.md`, Bloom downsample/upsample chain
- **Issue:** Bloom uses a multi-pass downsample/upsample blur chain. Each mip level requires a
  separate intermediate texture. At 1080p with 6 mip levels, this is 6 additional textures. The
  design does not mention resource aliasing with other post-process intermediates.
- **Impact:** 2-4 MB of GPU memory for bloom intermediates alone. Multiplied by resolution scaling
  and HDR format (R16G16B16A16), this grows.
- **Recommendation:** The render graph's resource aliasing (F-2.2.4) should be explicitly leveraged
  for bloom mip chain textures. Ensure the graph compiler can alias bloom downsample textures with
  DOF or motion blur intermediates that are not simultaneously live.

### 3.5 TLAS Rebuild Every Frame

- **File:** `rendering/advanced.md`, BLAS/TLAS (F-2.5.1)
- **Issue:** "per-frame TLAS rebuild/refit" is listed. For scenes with 100K+ instances, a full TLAS
  rebuild every frame is expensive (0.5-2.0 ms GPU).
- **Impact:** 0.5-2.0 ms GPU per frame for ray tracing acceleration structure maintenance.
- **Recommendation:** Use incremental TLAS refit for frames where only a small percentage of
  instances moved. Full rebuild only when structural changes (spawns/despawns) exceed a threshold.
  The BVH spatial index already uses this strategy.

### 3.6 Hair OIT Compositing Overdraw

- **File:** `animation/cloth-hair.md`, Hair rendering
- **Issue:** Hair rendering uses order-independent transparency (OIT). With 64-256 guide strands per
  character and multiple characters on screen, the OIT per-pixel linked list can grow very deep,
  causing excessive fragment shader overdraw.
- **Impact:** 1-4 ms GPU for scenes with 4+ characters with strand-based hair at close range.
- **Recommendation:** Cap the OIT linked list depth per pixel (e.g., 8 layers). Use weighted blended
  OIT as a cheaper fallback for mid-distance hair. The hair LOD system should aggressively
  transition to card-based rendering beyond a close threshold.

---

## 4. Async/IO Efficiency

### 4.1 Single Reactor Poll Point Adds Up to 16 ms

  I/O Latency

- **File:** `platform/threading.md`, Open Questions #3
- **Issue:** The design acknowledges this in Open Questions: "One poll per frame adds up to 16 ms
  I/O latency at 60 fps." The sequence diagram shows two poll points (start of frame and mid-frame),
  but the mid-frame poll is marked "optional."
- **Impact:** 8-16 ms I/O completion latency. For streaming assets that need to appear within 1-2
  frames, this is a full frame of unnecessary delay.
- **Recommendation:** Make the mid-frame poll mandatory. Consider a third poll point after GPU
  submission (while waiting for present). This reduces worst-case I/O latency to ~5 ms at 60 fps.

### 4.2 PersistentStream Ring Buffer Lacks Size Limit

- **File:** `core-runtime/events-plugins.md`, PersistentStream
- **Issue:** `PersistentStream<T>` has a `capacity` field but the design does not specify what
  happens when a slow reader falls behind and the ring buffer wraps. If the write head overtakes a
  reader's cursor, the reader would read corrupted data.
- **Impact:** Data corruption in slow readers (AI systems reading at 10 Hz from a 60 Hz stream).
- **Recommendation:** Explicitly define overflow semantics: either block the writer (backpressure),
  advance the slow reader's cursor (skip events), or return a `Lagged` error to the reader. The
  cursor should detect wrap-around.

### 4.3 Audio Stream Prefetch Buffer Sizing

- **File:** `audio/engine.md`, Streaming playback
- **Issue:** Audio streaming uses ring-buffer chunks with prefetch, but the design does not specify
  the prefetch depth relative to the audio callback period. At 48 kHz / 512 samples, the callback
  fires every 10.67 ms. If disk I/O latency exceeds the prefetch buffer depth, audio will glitch.
- **Impact:** Audio glitches under I/O contention (level loads, streaming heavy scenes).
- **Recommendation:** Size the prefetch buffer to at least 4x the callback period (~43 ms = 4
  chunks). Use the `IoPriority` system to give audio stream reads the highest priority in the
  `PriorityScheduler`.

### 4.4 BufferPool Exhaustion Under Load

- **File:** `platform/threading.md`, BufferPool
- **Issue:** `BufferPool::acquire()` returns `Option<BufferSlot>`. When the pool is exhausted,
  callers receive `None`. The design does not specify recovery behavior. With many concurrent async
  I/O operations, pool exhaustion is possible during heavy streaming.
- **Impact:** I/O operations fail when buffer pool is exhausted. Streaming stalls or errors.
- **Recommendation:** Implement a grow-on-demand strategy up to a configured maximum. When at
  maximum, queue I/O requests until a buffer is returned. The `PriorityScheduler` should throttle
  low-priority I/O when buffers are scarce.

---

## 5. Frame Budget

### 5.1 Physics + Rendering + AI Can Exceed 16.6 ms

- **File:** Multiple files
- **Issue:** Summing the stated budgets across domains:
  - Physics: 4 ms (R-4.3.NF2: 500 joints in 4 ms)
  - Rendering culling: 1 ms (NFR-2.3.1)
  - Rendering extraction: 2 ms (NFR-2.10.1)
  - Post-processing: 3 ms (NFR-2.9.1)
  - AI perception: budget-scheduled but unspecified
  - Audio sync: < 0.5 ms
  - Networking: 0.5 ms (R-X.1.1)
  - Transform propagation: 0.5 ms (R-1.2.4a)
  - Spatial index update: unspecified
  - UI layout: 0.5 ms (widget framework targets)
  - Total minimum: ~12 ms CPU + GPU overlap varies

This leaves only ~4.6 ms for gameplay logic, animation evaluation, and GPU rendering. The GPU budget
is separate but post-processing alone takes 3 ms GPU.

- **Impact:** Frame budget overrun on complex scenes with many systems active simultaneously.
- **Recommendation:** Define an explicit frame budget allocation document with per-subsystem CPU and
  GPU time slices. Add time-slicing to the AI budget scheduler. Ensure the render graph's budget
  culling (F-2.2.8) aggressively prunes GPU passes when the GPU budget is exceeded. The existing
  `FrameBudget` shared primitive should be wired into every subsystem.

### 5.2 BVH Full Rebuild is Unbounded

- **File:** `core-runtime/spatial-index.md`, `rebuild_full()`
- **Issue:** `BvhIndex::rebuild_full()` performs a complete SAH rebuild from all leaves. For 1M
  entities, this is an O(n log n) operation that could take 10-50 ms.
- **Impact:** Multi-frame hitch when SAH quality degrades past the threshold. The design says
  "Called when incremental quality degrades" but does not specify time-slicing.
- **Recommendation:** The design mentions "Full rebuilds happen in the background only when SAH
  quality degrades past a threshold." Explicitly run the rebuild on a background worker thread,
  double-buffering the BVH so the old tree remains available for queries while the new one is built.
  The `BvhRebuildSystem` in the file layout suggests this intent but the API lacks a
  `rebuild_async()` method.

### 5.3 NavMesh Tile Rebuild Can Spike

- **File:** `ai/navigation.md`, Incremental tile rebuild (F-7.1.8)
- **Issue:** NavMesh tile rebuilds on geometry change use voxelization which is CPU-intensive. While
  the design specifies background generation on worker threads (F-7.1.9), the interaction with the
  frame budget is not specified. Multiple simultaneous tile rebuilds (destruction events triggering
  NavMesh invalidation across several tiles) could saturate the thread pool.
- **Impact:** 2-10 ms per tile rebuild. Multiple simultaneous rebuilds could consume all worker
  threads temporarily.
- **Recommendation:** Cap concurrent NavMesh tile rebuilds to 1-2 per frame. Queue excess rebuilds
  and process over multiple frames. Use the `TaskPriority::Low` setting for NavMesh rebuilds so they
  do not compete with frame-critical work.

### 5.4 Cloth PBD Solver Iterations Unbudgeted

- **File:** `animation/cloth-hair.md`, ClothPanel
- **Issue:** `ClothPanel` has a `solver_iterations` field with no specified limit. More iterations
  produce stiffer cloth but increase GPU compute time linearly. With 16 simultaneous cloth panels
  (desktop target) each running multiple iterations, GPU compute time could be significant.
- **Impact:** 0.5-2.0 ms GPU per panel at high iteration counts. 16 panels could total 8-32 ms.
- **Recommendation:** Cap `solver_iterations` per platform tier (e.g., 4 on mobile, 8 on desktop, 16
  on high-end). Include cloth simulation in the render graph's budget culling. The `ClothLodTier`
  already reduces complexity but needs explicit iteration caps.

### 5.5 Scent Grid Propagation Per-Frame Cost

- **File:** `ai/perception.md`, Scent Propagation
- **Issue:** The scent grid propagates values every frame using wind drift, decay, and blocking. For
  large grids (e.g., 256x256 cells), this is a 65,536-cell update per frame. The propagation
  includes neighbor reads (wind drift), making it memory-bandwidth-bound.
- **Impact:** 0.5-1.0 ms for large scent grids without SIMD optimization.
- **Recommendation:** Update the scent grid at a reduced frequency (e.g., 10 Hz instead of 60 Hz)
  with interpolation between updates. Use SIMD for the decay and drift operations. Consider a
  hierarchical grid that only propagates at full resolution near active scent sources.

### 5.6 Flow Field Dijkstra Generation Cost

- **File:** `ai/steering-crowds.md`, FlowField generation
- **Issue:** Flow field generation uses Dijkstra propagation. For a 128x128 grid (16,384 cells),
  Dijkstra is O(n log n) with a priority queue, costing ~0.5-1.0 ms per field.
- **Impact:** If the cache misses and a new flow field must be generated, this blocks the AI
  pipeline for 0.5-1.0 ms.
- **Recommendation:** The design already specifies LRU caching. Ensure flow field generation is
  dispatched as a `TaskPriority::Low` background task, not inline in the crowd system. Provide stale
  flow field data to agents while the new field is being computed.

---

## 6. Missing SIMD Opportunities

### 6.1 Transform Propagation

- **File:** `core-runtime/scene-transforms.md`
- **Issue:** Transform propagation computes `parent.GlobalTransform * child.Transform` for every
  entity in a dirty subtree. The design mentions parallel processing of independent subtrees but
  does not mention SIMD batching of matrix multiplications within a subtree.
- **Impact:** 0.2-0.5 ms missed optimization at 1M entities with 1% dirty.
- **Recommendation:** When multiple siblings share the same parent GlobalTransform, batch the
  TRS-to-Mat4 conversions using 4-wide SIMD (process 4 children simultaneously). The SoA chunk
  layout of the ECS is designed for this.

### 6.2 AABB Intersection Tests in BVH Traversal

- **File:** `core-runtime/spatial-index.md`, BVH traversal
- **Issue:** BVH traversal performs AABB-ray and AABB-AABB intersection tests at every node. These
  are 6 component-wise comparisons that map directly to SIMD. The design mentions "SIMD
  acceleration" for batch queries but not for single-query traversal.
- **Impact:** 0.1-0.3 ms improvement for frustum culling at 1M entities.
- **Recommendation:** Use 4-wide SIMD for AABB min/max comparisons. Test 4 child nodes
  simultaneously during BVH traversal (requires a 4-ary BVH tree, not binary). This is a known
  optimization in production renderers.

### 6.3 ORCA Half-Plane Solver

- **File:** `ai/steering-crowds.md`, ORCA algorithm
- **Issue:** The ORCA solver computes velocity obstacles and half-plane intersections for each
  agent-neighbor pair. With `max_neighbors: u8` (up to 255 neighbors), this is a significant
  per-agent computation that maps well to SIMD.
- **Impact:** 0.1-0.5 ms per 1000 agents.
- **Recommendation:** Batch the velocity obstacle computation using 4-wide SIMD. Process 4 neighbor
  pairs simultaneously. The linear programming fallback can also be vectorized.

---

## 7. Cache-Unfriendly Patterns

### 7.1 Sparse-Set Lookup is Two Indirections

- **File:** `core-runtime/ecs.md`, SparseSet
- **Issue:** `SparseSet` uses a `PagedSparseArray` for the sparse-to-dense mapping. Each lookup
  requires: (1) page lookup from entity index, (2) read dense index from page, (3) read component
  from dense array. Steps 1 and 2 are pointer chases through different memory regions.
- **Impact:** Cache miss on each sparse-set component access. For `#[sparse]` components accessed
  frequently (e.g., enableable AI states), this adds 3-5 ns per access vs 1-2 ns for table storage.
- **Recommendation:** Document this tradeoff clearly. Ensure `#[sparse]` is only used for components
  that are rarely iterated (high churn + rare access). Consider a flat sparse array for components
  with bounded entity counts.

### 7.2 Relationship Pair Encoding Creates

  Archetype Fragmentation

- **File:** `core-runtime/ecs.md`, Relationship pairs
- **Issue:** Each unique `(Relationship, Target)` pair is a distinct `ComponentId`. Entities with
  different parents have different archetypes (since `(ChildOf, parent_A)` and `(ChildOf, parent_B)`
  are different component IDs). This fragments entities across many small archetypes.
- **Impact:** Reduced cache locality during query iteration. Instead of iterating one large
  archetype chunk, the query must visit many small archetypes. For 10K entities with distinct
  parents, this could mean 10K single-entity archetypes.
- **Recommendation:** This is an inherent tradeoff of flecs-style relationship encoding. Consider
  storing relationship targets as an array component rather than as archetype-level data for
  high-cardinality relationships (many distinct targets). The `ChildOf` relationship is the primary
  concern; document the expected archetype count and ensure the query cache handles large archetype
  lists efficiently.

---

## 8. Summary of Critical Findings

Priority-ranked by estimated frame-time impact:

| # | Finding | Est. Impact | Category |
|---|---------|-------------|----------|
| 1 | SpatialQuery returns Vec (alloc per query) | 1-3 ms/frame | Algorithmic |
| 2 | Archetype columns stored in HashMap | 1.5-4.5 ms/frame | Memory |
| 3 | GlobalTransform full Mat4 (64 bytes) | 0.5-1.0 ms/frame | Memory |
| 4 | BvhNode is 40 bytes, not 32 as claimed | 0.1-0.3 ms/frame | Memory |
| 5 | LeafEntry cold data mixed with hot | 0.1-0.3 ms/frame | Memory |
| 6 | BVH full rebuild unbounded, no async | 10-50 ms spike | Frame Budget |
| 7 | Frame budget sum approaches 16.6 ms | Risk of overrun | Frame Budget |
| 8 | TLAS full rebuild every frame | 0.5-2.0 ms GPU | GPU |
| 9 | Octree remove() is O(n) | 0.1-1.0 ms/frame | Algorithmic |
| 10 | Single reactor poll adds 16 ms latency | Latency, not FPS | Async/IO |
| 11 | Cloth solver iterations unbudgeted | 8-32 ms GPU risk | Frame Budget |
| 12 | Vis buffer 64-bit atomics no mobile fallback | Pipeline failure | GPU |
