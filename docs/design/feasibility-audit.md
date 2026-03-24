# Feasibility Audit Report

Audit date: 2026-03-15. Covers all 87 design documents across 15 domains and 7 waves.

---

## Summary Risk Matrix

| Risk Category | Critical | High | Medium | Low | Total |
|---------------|----------|------|--------|-----|-------|
| Technical Feasibility | 2 | 5 | 6 | 2 | 15 |
| Rust Ecosystem | 2 | 4 | 3 | 1 | 10 |
| Scale Feasibility | 1 | 3 | 3 | 1 | 8 |
| Integration Feasibility | 1 | 3 | 4 | 1 | 9 |
| Dependency Feasibility | 1 | 2 | 3 | 2 | 8 |
| **Total** | **7** | **17** | **19** | **7** | **50** |

---

## 1. Technical Feasibility

### T-1. Custom IoReactor on Three Platforms

- **File:** `platform/threading.md`, `core-runtime/memory-async-io.md`
- **Design element:** IoReactor wrapping IOCP, GCD controlled drain, and io_uring with a single
  poll-point-per-frame model
- **Risk:** Building a production-quality async I/O reactor from scratch on three platforms is a
  multi-person-year effort. The GCD "controlled drain" pattern (routing dispatch_io callbacks to a
  serial queue and draining synchronously) is not a standard GCD pattern and may interact poorly
  with Metal command buffer completion handlers. The io_uring backend requires kernel 5.1+ minimum
  but advanced features (fixed buffers, registered files) need 5.6+; the design does not resolve
  this.
- **Likelihood:** High
- **Impact:** Critical
- **Mitigation:** Prototype the GCD controlled drain pattern first. Validate that `dispatch_sync` on
  a serial queue reliably captures all pending completions without races. Define a minimum kernel
  version policy for io_uring. Consider a phased approach: IOCP first (best documented), then
  io_uring, then GCD.

### T-2. GCD Fibers via C ABI

- **File:** `platform/threading.md`, `constraints.md`
- **Design element:** macOS fibers implemented via GCD dispatch queues and blocks, accessed through
  C ABI wrappers
- **Risk:** GCD dispatch blocks are not true cooperative fibers. They are fire-and-forget work items
  that cannot be suspended mid-execution and resumed on a different thread. The design's
  `FiberYielder::yield_now()` semantics require saving and restoring execution context, which GCD
  dispatch queues do not support. The design says "No custom assembly" for macOS but GCD blocks
  cannot yield mid-execution.
- **Likelihood:** High
- **Impact:** Critical
- **Mitigation:** Reassess the macOS fiber strategy. Options: (a) use `ucontext` (deprecated on
  macOS but still functional), (b) use `setjmp/longjmp` with explicit stacks (same as Linux), (c)
  redefine "fibers" on macOS as cooperative async tasks that yield only at `.await` points rather
  than arbitrary suspension points. Option (c) aligns best with the async/await-first constraint.

### T-3. Custom Windowing System (No Winit)

- **File:** `platform/windowing.md`, `constraints.md`
- **Design element:** Platform-native windowing via Win32, NSWindow (Swift C ABI), xcb, and Wayland
- **Risk:** The windowing design requires implementing: window lifecycle, fullscreen modes, DPI
  scaling, HDR output, VSync/presentation modes, display enumeration, and multi-monitor support on
  four backends (Win32, Cocoa, X11, Wayland). Wayland is particularly complex due to its compositor
  protocol (xdg-shell, fractional-scale, decoration negotiation). This is what winit's 50+
  maintainers have spent years on.
- **Likelihood:** Medium
- **Impact:** High
- **Mitigation:** Prioritize Win32 and macOS backends first (best documented APIs). For Linux, start
  with X11/xcb only; defer Wayland to a later phase. Wayland's fractional scaling protocol
  (`wp_fractional_scale_v1`) is still unstable in some compositors. Budget 3-6 months for windowing
  alone.

### T-4. Metal Backend via Swift-to-C-to-Bindgen

- **File:** `rendering/gpu-abstraction.md`
- **Design element:** Metal backend accessed via Swift wrappers through C ABI (F-2.1.4)
- **Risk:** The FFI chain is: Rust -> C ABI -> C++ -> Swift. C ABI does not natively support
  Swift; the C++ layer must bridge between C ABI and Swift. Swift interop with C++ (Swift/C++
  interoperability) was stabilized in Swift 5.9 but has limitations: no support for Swift generics
  in C++, limited enum bridging, and no async/await bridging. Every Metal API call traverses three
  language boundaries.
- **Likelihood:** High
- **Impact:** High
- **Mitigation:** Consider using Metal's C API (`MTLDevice`, etc. via Objective-C runtime functions)
  accessed through bindgen rather than Swift. The `metal-rs` or `objc2-metal` crates provide direct
  Objective-C FFI without the Swift intermediate layer. Alternatively, use a thin Objective-C++
  bridge instead of Swift.

### T-5. GJK/EPA/SAT Physics from Scratch

- **File:** `physics/foundation.md`
- **Design element:** Full narrowphase contact generation (GJK/EPA/SAT), rigid body dynamics, island
  solver, CCD, all written in pure Rust with no PhysX dependency
- **Risk:** A production physics engine is one of the hardest systems to write correctly. GJK edge
  cases (degenerate simplexes, numerical precision), EPA robustness (face enumeration on the
  Minkowski difference), and stable contact manifold generation are notoriously difficult. Most
  commercial engines (Unreal, Unity, Godot) use PhysX or Bullet rather than rolling their own.
- **Likelihood:** Medium
- **Impact:** High
- **Mitigation:** Use the `parry3d` crate for narrowphase geometry operations (GJK, EPA, contact
  manifolds). It is well-tested, pure Rust, and actively maintained by the Rapier physics team.
  Build the ECS integration, island solver, and broadphase (shared BVH) in-house but delegate
  geometric primitives to a proven library.

### T-6. Fluid Simulation (SPH, FLIP/PIC, Eulerian)

- **File:** `physics/advanced.md`
- **Design element:** Three distinct fluid simulation methods (SPH, FLIP/PIC, Eulerian grid) with
  GPU acceleration and two-way rigid body coupling
- **Risk:** Each fluid simulation method is a research-grade system. Real-time GPU fluid simulation
  with two-way rigid body coupling is at the frontier of game physics. No shipping game engine
  provides all three methods. FLIP/PIC in particular requires a particle-to-grid transfer that is
  non-trivial to parallelize on the GPU.
- **Likelihood:** High
- **Impact:** Medium
- **Mitigation:** Start with SPH only (best understood, most GPU-friendly). Defer FLIP/PIC and
  Eulerian solvers to post-MVP. Use SPH for small-scale effects (splashing, flowing) and
  screen-space water simulation for large bodies. Evaluate whether Eulerian is needed at all given
  FFT ocean already exists in the environment systems.

### T-7. Neural Denoiser and NRD Integration

- **File:** `rendering/advanced.md`
- **Design element:** Neural network denoiser (requires tensor cores) with NRD fallback (proprietary
  NVIDIA SDK)
- **Risk:** Neural denoising requires ML inference infrastructure that does not exist in the engine.
  NRD is proprietary NVIDIA SDK with licensing restrictions. The design acknowledges this requires
  "separate approval" but provides no fallback for non-NVIDIA hardware beyond NRD SVGF, which itself
  is NVIDIA-proprietary.
- **Likelihood:** Medium
- **Impact:** Medium
- **Mitigation:** Implement a vendor-neutral temporal denoiser (SVGF algorithm is published, not
  proprietary; NRD is the NVIDIA implementation). A custom SVGF implementation in HLSL would run on
  all backends. Neural denoising should be a nice-to-have tier, not a dependency.

### T-8. Visibility Buffer 64-bit Atomics

- **File:** `geometry/meshlets.md`
- **Design element:** Visibility buffer rendering with 64-bit atomic writes per pixel
- **Risk:** 64-bit shader atomics are not universally available. The design's GPU feature matrix
  lists `MTLGPUFamilyApple7+` (M1 and later), SM 6.6 `InterlockedCompareExchange64`, and
  `shaderBufferInt64Atomics`. Older Intel integrated GPUs, AMD GPUs before RDNA2, and pre-M1 Apple
  GPUs lack 64-bit atomics. The design has no fallback path for this capability.
- **Likelihood:** Medium
- **Impact:** High
- **Mitigation:** Design a 32-bit visibility buffer fallback that uses split-field encoding (e.g.,
  24-bit instance + 8-bit triangle, or two-pass approaches). The capability gating system (F-2.2.2)
  should select the appropriate path.

### T-9. Sound Propagation via Hybrid Ray-Portal

- **File:** `audio/engine.md`
- **Design element:** Occlusion/obstruction via shared BVH with material transmission loss; sound
  propagation via hybrid ray-portal solver (async)
- **Risk:** Physically-based sound propagation (diffraction, multi-path) is computationally
  expensive and a research-active area. The "hybrid ray-portal" solver is not a standard algorithm
  name; the design does not specify implementation details. Real-time propagation at game frame
  rates requires careful budgeting.
- **Likelihood:** Medium
- **Impact:** Medium
- **Mitigation:** Start with simple ray-based occlusion (one ray per source-listener pair via shared
  BVH) and portal-based propagation for indoor scenes. Defer full acoustic simulation. Steam Audio
  and Resonance Audio provide reference implementations that could inform the design.

### T-10. Planetary-Scale Voxel Terrain

- **File:** `geometry/terrain.md`
- **Design element:** Planetary-scale voxel sphere with radial gravity (F-3.2.11), GPU SDF meshing
  via dual contouring (F-3.2.12)
- **Risk:** Planetary-scale voxel terrain with GPU dual contouring and seamless LOD is cutting-edge
  technology. No shipping game engine provides this as a built-in feature. GPU dual contouring on
  sparse octrees requires careful memory management and produces non-manifold geometry at LOD seams.
- **Likelihood:** High
- **Impact:** Medium
- **Mitigation:** Treat planetary voxel terrain as an experimental/optional feature. The heightfield
  CDLOD terrain system should be the primary supported path. Defer voxel planet to post-MVP.

### T-11. CRDT Collaborative Editing Per Asset Type

- **File:** `tools/collaboration.md`
- **Design element:** CRDT document model per asset type (F-15.12.8) for real-time co-editing of
  scenes, materials, animations, logic graphs
- **Risk:** CRDTs for structured game assets (scene graphs, material networks, animation state
  machines) are not a solved problem. Text CRDTs (e.g., Yrs/Y.js) are mature, but tree-structured
  CRDTs for scene hierarchies with cross-references require custom conflict resolution. Merge
  semantics for visual graphs are an open research question.
- **Likelihood:** High
- **Impact:** Medium
- **Mitigation:** Start with whole-asset locking (pessimistic concurrency) as the baseline. Add CRDT
  support incrementally, starting with the simplest asset types (text, key-value configs). Scene
  graph CRDT should be a stretch goal.

### T-12. Shape Grammar Building Generation

- **File:** `geometry/procedural-generation.md`
- **Design element:** Shape grammar building generator (F-3.6.18) and L-system road networks
  (F-3.6.15)
- **Risk:** Shape grammars for architectural generation (CGA Shape, CityEngine style) are complex to
  implement correctly. L-system road networks with proper intersection handling and terrain
  integration add further complexity. These are typically PhD-level implementations.
- **Likelihood:** Medium
- **Impact:** Low
- **Mitigation:** Implement simple rule-based generation first (modular building assembly using
  socket snapping). Defer full shape grammar evaluation to a later phase. L-system roads can start
  as simple spline networks without automatic intersection handling.

### T-13. Adaptive Music Beat-Sync Transitions

- **File:** `audio/dsp-music.md`
- **Design element:** Horizontal re-sequencing via segment directed graph with beat-sync transitions
  (F-5.4.2, F-5.4.3), tempo/beat clock (F-5.4.4)
- **Risk:** Sample-accurate beat-synchronized music transitions require a tempo-aware scheduler that
  can pre-compute crossfade points. The real-time audio thread has a < 0.5 ms budget. Beat tracking
  across variable-tempo music segments adds implementation complexity.
- **Likelihood:** Low
- **Impact:** Medium
- **Mitigation:** Start with bar-boundary transitions (quantize to next bar) and crossfade-based
  transitions. Beat-accurate sub-bar transitions can be added incrementally.

### T-14. Stochastic Many-Light Sampling

- **File:** `rendering/lighting.md`
- **Design element:** Stochastic many-light sampling with temporal denoiser (F-2.4.10)
- **Risk:** ReSTIR-style many-light sampling is a recent technique (2020+) that requires careful
  implementation of reservoir sampling on the GPU. The temporal denoiser must handle disocclusion
  correctly. This is production-proven at NVIDIA but not widely implemented in other engines.
- **Likelihood:** Medium
- **Impact:** Medium
- **Mitigation:** The tiled/clustered forward+ path already supports 500+ lights. Stochastic
  sampling is needed only for extreme light counts (10,000+). Defer to post-MVP unless content
  requires it.

### T-15. Volumetric Shadow Maps (Fourier Opacity)

- **File:** `rendering/lighting.md`
- **Design element:** Volumetric shadow maps with Fourier opacity mapping (F-2.4.19)
- **Risk:** Fourier opacity mapping is a specialized technique with limited adoption outside
  research papers. Light leaking artifacts at high-frequency opacity boundaries are a known issue.
- **Likelihood:** Low
- **Impact:** Low
- **Mitigation:** Use standard volumetric ray marching with shadow map sampling as the primary path.
  Fourier opacity mapping is a quality tier option for high-end GPUs.

---

## 2. Rust Ecosystem Feasibility

### R-1. Custom Async Runtime — No Ecosystem Crates

- **File:** `platform/threading.md`, `core-runtime/memory-async-io.md`
- **Design element:** Custom `IoReactor` + work-stealing thread pool as the sole async runtime for
  all binaries (engine, editor, servers, tools)
- **Risk:** The Rust async ecosystem assumes a third-party runtime. The custom runtime means no
  runtime-dependent HTTP clients, no runtime-dependent database drivers, and no runtime-dependent
  TLS integration. Every network client library must be replaced or adapted. The design identifies
  this conflict in `tools/launcher.md`, `tools/collaboration.md`, and `tools/version-control.md`.
- **Likelihood:** High
- **Impact:** Critical
- **Mitigation:** All binaries use the IoReactor. Replacements per protocol:
  - **HTTP:** Platform-native clients (NSURLSession, WinHTTP, libcurl) wrapped via IoReactor.
  - **WebSocket:** `tungstenite` (non-async) with manual I/O integration via IoReactor.
  - **Database:** Custom PostgreSQL wire protocol client on IoReactor, or synchronous clients on
    dedicated I/O threads.
  - **QUIC:** `quinn` with `AsyncUdpSocket` trait implemented on the IoReactor.
  - **REST server:** Custom HTTP server on IoReactor using raw TCP with HTTP/1.1 parsing.

### R-2. C ABI Limitations for GCD Callbacks

- **File:** `platform/threading.md`, all macOS platform paths
- **Design element:** GCD Dispatch IO and dispatch blocks accessed through C ABI wrappers
- **Risk:** C ABI does not support C++ lambdas, std::function, or function pointers as callback
  parameters. GCD's core API pattern is `dispatch_async(queue, block)` where `block` is an
  Objective-C block (not a C++ lambda). The C++ wrappers must capture Rust state in dispatch blocks,
  which requires converting Rust function pointers to C-compatible callbacks. This is feasible but
  complex and error-prone. C ABI also does not support Objective-C blocks directly.
- **Likelihood:** High
- **Impact:** High
- **Mitigation:** Use the `block2` crate for creating Objective-C blocks from Rust closures,
  bypassing the C++ layer entirely. Alternatively, define a C callback interface (function pointer +
  void* context) in the C++ wrapper that C ABI can bridge, and use `dispatch_async_f` (the
  function-pointer variant of dispatch_async) instead of block-based dispatch.

### R-3. bevy_reflect-Style Reflection from Scratch

- **File:** `core-runtime/reflection-serialization.md`
- **Design element:** Custom Reflect trait, TypeRegistry, derive macro, property path access,
  dynamic values
- **Risk:** bevy_reflect is ~20,000 lines of code with complex proc macros. Reimplementing it from
  scratch requires: attribute parsing, derive expansion for structs/enums/tuples, collection
  reflection, trait object registration, and FromReflect conversion. The derive macro alone
  (supporting all annotations: skip, rename, default, custom attributes) is a significant project.
- **Likelihood:** Medium
- **Impact:** High
- **Mitigation:** This is feasible but should be one of the earliest implementation efforts (the
  design correctly places it in Wave 1). Consider starting with a minimal subset (struct reflection
  only, no enum/tuple/collection reflection) and expanding incrementally. Budget 2-3 months for the
  derive macro and registry.

### R-4. Scoped Async Tasks with Borrow Checker

- **File:** `platform/threading.md`
- **Design element:** `Scope::spawn_async` -- scoped async tasks that borrow from the calling frame
  without `'static` or `Arc`
- **Risk:** Rust's borrow checker makes scoped async tasks very difficult. A scoped async task that
  borrows `&world` must guarantee the future does not outlive the scope, but futures are typically
  `'static` to be stored in task queues. The standard library's `std::thread::scope` works for
  synchronous tasks, but the async equivalent requires either: (a) unsafe lifetime erasure, (b)
  `Pin`-based self-referential types, or (c) a custom executor that proves to the compiler the scope
  is respected. `async-scoped` crate exists but requires careful unsafe usage.
- **Likelihood:** High
- **Impact:** High
- **Mitigation:** Implement scoped synchronous tasks first (well-understood pattern). For async
  tasks, use `Arc` for shared data and accept the overhead, or use unsafe lifetime transmutation
  with a runtime check that the scope completes before the borrowed data is dropped. Document the
  safety invariants carefully.

### R-5. ECS Query System Ergonomics

- **File:** `core-runtime/ecs.md`
- **Design element:** Composable archetype queries with parallel iteration, change detection, and
  `unsafe trait WorldQuery`
- **Risk:** The ECS query system design requires `unsafe trait WorldQuery` (line 1440 of ecs.md),
  `unsafe fn fetch`, and `unsafe fn get_unchecked`. Building a sound query system that prevents
  mutable aliasing across parallel iterators while remaining ergonomic is one of the hardest
  problems in Rust ECS design. Bevy's query system has evolved through multiple major rewrites.
- **Likelihood:** Medium
- **Impact:** High
- **Mitigation:** Study Bevy's query safety model carefully (FilteredAccess, archetype-level
  conflict detection). Invest in formal safety documentation and extensive miri/TSAN testing.
  Consider starting with single-threaded queries and adding parallel iteration incrementally.

### R-6. Proc Macro for ECS Component Derive

- **File:** `core-runtime/ecs.md`, `core-runtime/reflection-serialization.md`
- **Design element:** Derive macros for Component, Reflect, Serialize, and associated traits
- **Risk:** Multiple interacting proc macros (Component, Reflect, Serialize) that generate trait
  impls based on attribute annotations are complex to maintain and debug. Compile-time impact of
  proc macros on large codebases is significant.
- **Likelihood:** Medium
- **Impact:** Medium
- **Mitigation:** Use a single unified derive macro that generates all trait implementations based
  on attribute flags, rather than separate macros that each parse the same type definition. This
  reduces compile time and maintenance burden.

### R-7. Zero-Copy Deserialization Safety

- **File:** `core-runtime/reflection-serialization.md`
- **Design element:** Zero-copy deserialization via memory mapping (F-1.4.2),
  `unsafe fn field_as<T>`
- **Risk:** The design includes `unsafe fn field_as` that reinterprets bytes as typed references.
  This is a correctness and safety hazard: alignment violations cause undefined behavior, and type
  confusion can corrupt memory. The open questions section (line 2173) explicitly asks whether this
  should remain unsafe.
- **Likelihood:** Medium
- **Impact:** Medium
- **Mitigation:** Use `bytemuck::Pod` or `zerocopy` crate traits to validate that types are safe for
  zero-copy reinterpretation. Require all zero-copy types to derive `Pod` (guarantees alignment and
  no padding). Keep the unsafe API internal and expose only safe wrappers.

### R-8. setjmp/longjmp Fibers on Linux

- **File:** `platform/threading.md`
- **Design element:** Linux fibers via custom assembly context switch using `setjmp`/`longjmp` with
  explicit stacks
- **Risk:** `setjmp`/`longjmp` with explicit stacks is undefined behavior in C (and by extension
  Rust) when the jump crosses stack frames that have been unwound. Rust's drop semantics compound
  this: if a fiber yields inside a function with local Drop types, resuming may skip destructors.
  The stable Rust constraint means `asm!` is available (stabilized in 1.59), but custom context
  switching requires deep platform ABI knowledge.
- **Likelihood:** Medium
- **Impact:** Medium
- **Mitigation:** Use the `context` or `minicoroutine` crate for context switching rather than raw
  setjmp/longjmp. Alternatively, use `ucontext_t` (POSIX, available on Linux). Ensure that fibers
  only yield at controlled points where no Rust Drop guards are active.

### R-9. HLSL Shader Pipeline via DXC + MSC

- **File:** `constraints.md`, `rendering/gpu-abstraction.md`
- **Design element:** HLSL compiled by DXC (C++ via C ABI) to DXIL/SPIR-V, then Metal Shader
  Converter (C++ via C ABI) for MSL
- **Risk:** DXC's C++ API is complex (COM-based on Windows, dxcompiler shared library on other
  platforms). Wrapping DXC's COM interfaces through C ABI requires careful handling of reference
  counting and memory management. Metal Shader Converter is an Apple tool with limited documentation
  for programmatic use.
- **Likelihood:** Medium
- **Impact:** Medium
- **Mitigation:** Use DXC's command-line interface for offline compilation (during asset processing)
  rather than embedding the DXC library. Runtime shader compilation (hot reload) can use a
  subprocess call. This avoids the COM FFI complexity entirely.

### R-10. Async Database Client Without Ecosystem Crates

- **File:** `networking/mmo.md`
- **Design element:** DatabaseAccessLayer with async PostgreSQL queries via platform-native I/O
- **Risk:** All mature Rust PostgreSQL clients depend on a third-party async runtime. Building an
  async PostgreSQL client on the custom IoReactor requires implementing the PostgreSQL wire protocol
  from scratch, including TLS, connection pooling, prepared statements, and SASL authentication.
- **Likelihood:** High
- **Impact:** Medium
- **Mitigation:** Two options, both using the IoReactor: (a) implement a custom PostgreSQL wire
  protocol client using async socket I/O via the IoReactor, or (b) use synchronous PostgreSQL
  clients (e.g., `postgres` crate) on dedicated I/O threads managed by the IoReactor's thread pool.
  All server infrastructure uses the same IoReactor as the engine.

---

## 3. Scale Feasibility

### S-1. 1,381 Features as a Deliverable

- **File:** `plan.md`
- **Design element:** "1,381 features, 1,171 requirements, and 5,859 user stories across 15 domains"
- **Risk:** Even with a 10-person team working full time, 1,381 features at an average of 1 week
  each would take ~27 person-years. With realistic overhead (testing, integration, iteration), 40+
  person-years is more likely. This exceeds what most game engine teams deliver in a single
  development cycle.
- **Likelihood:** High
- **Impact:** Critical
- **Mitigation:** Define a strict MVP feature set (the plan's "MVP Fast Path" of 13 designs is a
  start). Prioritize ruthlessly: the MVP should target a single genre (e.g., 3D action) and a single
  platform (Windows). Defer genre-specific features (RTS fog of war, racing vehicles, survival
  crafting), advanced rendering (path tracing, neural denoising), and social features (guilds,
  auction house, battle pass) to post-MVP phases. Target 200-300 features for MVP.

### S-2. Shared BVH at Million-Entity Scale

- **File:** `core-runtime/spatial-index.md`
- **Design element:** Single shared BVH for physics, rendering, networking, AI, audio, and gameplay.
  Performance target: ray cast < 10 us at 1M entities, frustum cull < 500 us at 1M entities.
- **Risk:** A single BVH serving all subsystems with incremental updates proportional to moved
  entities is feasible in theory, but contention becomes an issue. If physics moves 10K bodies per
  frame, the BVH must incrementally update 10K entries, then serve queries from rendering, AI,
  audio, and networking concurrently. The < 10 us ray cast target at 1M entities is achievable with
  a well-tuned BVH but requires SIMD acceleration and careful memory layout.
- **Likelihood:** Medium
- **Impact:** High
- **Mitigation:** The design already includes a grid/octree hybrid alongside the BVH (F-1.9.3). Use
  the grid for cell-based queries (networking relevancy, AI perception) and the BVH for geometric
  queries (ray cast, frustum cull). This reduces contention. Benchmark incremental BVH update cost
  early and set a per-frame budget for moved entities.

### S-3. MMO: Thousands of Players Per Shard

- **File:** `networking/mmo.md`
- **Design element:** 500 players per shard, dynamic zone splitting, < 100 ms migration, 10,000+ TPS
  database writes
- **Risk:** The scaling tiers show 500 players per shard with 20-40 zones. At 500 players with full
  ECS simulation, each zone server must handle replication, prediction, spatial queries, AI, and
  physics for 2,000 entities (500 players + NPCs) at 60 Hz within a 0.5 ms networking budget. The <
  100 ms migration target requires serializing a full player entity (inventory, buffs, cooldowns,
  prediction history), transferring over the network, and deserializing -- all under 100 ms. This is
  aggressive but achievable if the serialization is efficient.
- **Likelihood:** Medium
- **Impact:** High
- **Mitigation:** Start with 64-player servers (FPS scale) as the first milestone. Scale to MMO
  density incrementally. The zone splitting and migration features should be validated with
  synthetic load tests before real content. The 0.5 ms networking budget may need to be relaxed to
  1-2 ms for MMO scenarios.

### S-4. GPU Particle Throughput (1M Particles)

- **File:** `vfx/particles.md`
- **Design element:** 1M+ particles per frame at 60 fps, GPU sort of 100K particles in < 0.5 ms
- **Risk:** 1M particles at 60 fps is achievable on desktop GPUs with GPU compute simulation and
  indirect rendering. The < 0.5 ms GPU sort target for 100K particles (bitonic or radix sort) is
  tight but feasible on modern GPUs. Mobile targets (50K-100K particles) are more constrained.
- **Likelihood:** Low
- **Impact:** Medium
- **Mitigation:** The design already includes platform tiers. Validate GPU sort performance on
  target hardware early. Consider using HLSL `WaveActiveCountBits` for more efficient prefix sums on
  supported hardware.

### S-5. AOI Evaluation (1K Clients, 100K Entities)

- **File:** `networking/replication.md`
- **Design element:** Area-of-interest filtering via shared BVH for 1K clients with 100K entities, <
  2 ms per tick
- **Risk:** Evaluating relevancy for 1K clients against 100K entities is 100M pairwise checks per
  tick. Even with spatial indexing (grid-based AOI), the per-tick budget of < 2 ms is aggressive.
  The design notes that "snapshot memory for 100K entities could be" a concern.
- **Likelihood:** Medium
- **Impact:** Medium
- **Mitigation:** Use a uniform grid for AOI (not the BVH -- grid is O(1) per cell lookup). Batch
  relevancy evaluation across ticks (evaluate 1/N clients per tick in a round-robin). Most entities
  are dormant and do not change relevancy frequently.

### S-6. Content Pipeline Asset Volume

- **File:** `content-pipeline/asset-import.md`, `content-pipeline/processing.md`
- **Design element:** Batch import with parallelism, CAS database, incremental builds
- **Risk:** A AAA game project may have 100K+ assets totaling 500+ GB of source data. The CAS
  (content-addressable storage) database must handle BLAKE3 hash lookups, dependency graphs, and
  incremental invalidation at this scale. The build farm must process texture compression, meshlet
  baking, and shader compilation in parallel.
- **Likelihood:** Low
- **Impact:** Medium
- **Mitigation:** The CAS design is sound (BLAKE3-keyed, bottom-up invalidation). Use a local SQLite
  or LMDB database for the metadata store rather than a custom implementation. Validate incremental
  build performance at 100K asset scale early.

### S-7. Foliage: 1M+ Instances

- **File:** `geometry/environment.md`
- **Design element:** 1M+ visible foliage instances on desktop, GPU-driven compute culling in < 1 ms
- **Risk:** 1M foliage instances require efficient GPU instanced rendering with compute-based
  culling. The < 1 ms GPU compute budget for culling 1M instances is feasible with a well-optimized
  compute shader but leaves little margin. Wind animation for 1M instances adds further cost.
- **Likelihood:** Low
- **Impact:** Low
- **Mitigation:** Use hierarchical culling (cluster of instances culled together) rather than
  per-instance culling. The meshlet pipeline's visibility buffer can handle foliage rendering
  efficiently if foliage meshlets are small.

### S-8. Scripting: 1,000 Active Graphs < 4 ms

- **File:** `game-framework/scripting.md`
- **Design element:** 1,000 active logic graphs with 10 nodes each in < 4 ms per frame at 60 fps
  (R-13.4.NF1)
- **Risk:** 1,000 graphs at 10 nodes each = 10,000 node evaluations per frame. At 4 ms budget, that
  is 400 ns per node. A bytecode VM with instruction budgets should achieve this, but graphs that
  read ECS data or emit events add overhead from data access and cache misses.
- **Likelihood:** Low
- **Impact:** Medium
- **Mitigation:** Compile frequently-executed graphs to a flat bytecode representation (the design
  includes AOT compilation). Use data-oriented design for the VM: batch all graphs of the same type
  and evaluate them together for better cache locality.

---

## 4. Integration Feasibility

### I-1. Shared BVH Contention Model

- **File:** `core-runtime/spatial-index.md`, `physics/foundation.md`, `rendering/core-rendering.md`,
  `networking/replication.md`, `ai/perception.md`, `audio/engine.md`
- **Design element:** Single BVH read by physics, rendering, networking, AI, and audio; updated once
  per frame
- **Risk:** The "updated once per frame" constraint means all consumers read stale spatial data.
  Physics computes contacts using positions from the previous frame's BVH update. If physics runs
  multiple substeps per frame, the BVH does not reflect intermediate positions. This creates a
  semantic mismatch: physics needs current positions, the BVH provides start-of-frame positions.
- **Likelihood:** High
- **Impact:** Critical
- **Mitigation:** Allow the physics broadphase to use a lightweight sweep-and-prune or incremental
  update within substeps, with the shared BVH as the initial broadphase seed. The BVH provides the
  inter-subsystem spatial index; physics can maintain a narrow-phase cache that updates within
  substeps.

### I-2. Audio Thread vs ECS Constraint

- **File:** `constraints.md`, `audio/engine.md`
- **Design element:** Audio mixer runs on a dedicated real-time thread with < 0.5 ms latency budget,
  bridged to ECS via lock-free SPSC command queue. Exception to "100% ECS-based" rule.
- **Risk:** The exception is well-documented, but the boundary between ECS-owned audio state and
  audio-thread-owned state must be carefully defined. Commands from ECS to audio thread (play, stop,
  set parameter) are fire-and-forget via SPSC queue. But audio-to-ECS feedback (current playback
  position, completion events, propagation results) requires a reverse channel. The design does not
  fully specify this reverse path.
- **Likelihood:** Medium
- **Impact:** High
- **Mitigation:** Define a bidirectional SPSC pair: game-to-audio for commands, audio-to-game for
  events/feedback. The game thread reads the feedback queue during the ECS update phase. Latency of
  one frame for feedback is acceptable for audio events.

### I-3. Render Proxy Extraction Overhead

- **File:** `rendering/core-rendering.md`
- **Design element:** Extract phase copies changed ECS data into renderer-owned ProxyStore. Target:
  < 2.0 ms for 100K entities (full extraction), < 0.5 ms for 1% dirty.
- **Risk:** The extraction phase runs on the game loop thread and blocks simulation from advancing.
  At 100K entities with 100% dirty (scene load), the 2.0 ms budget is tight. The design says
  extraction uses "immutable ECS queries" on a "dedicated thread," but the simulation cannot advance
  while extraction borrows the ECS world immutably.
- **Likelihood:** Medium
- **Impact:** Medium
- **Mitigation:** Use double-buffered ECS worlds or copy-on-write snapshots to allow simulation and
  extraction to overlap. Alternatively, accept one frame of latency between simulation and rendering
  (pipelined extraction), which is standard practice in AAA engines.

### I-4. Cross-Domain Event Ordering

- **File:** `core-runtime/events-plugins.md`
- **Design element:** Typed double-buffered event channels with per-type isolation, component
  lifecycle observers at sync points
- **Risk:** Event ordering across domains can cause subtle bugs. If physics emits a collision event
  that AI observes, the processing order depends on system scheduling. The design uses deterministic
  flush at sync points, but the order of observers within a sync point is not specified. This can
  cause non-deterministic behavior in physics-AI-gameplay interaction chains.
- **Likelihood:** Medium
- **Impact:** Medium
- **Mitigation:** Define explicit observer priority ordering within sync points. Use the system
  scheduling DAG to enforce that physics observers run before AI observers, which run before
  gameplay observers. Document the observer execution order as a contract.

### I-5. Plugin Hot Reload Safety

- **File:** `core-runtime/events-plugins.md`
- **Design element:** Plugin hot reload with state preservation (F-1.6.5)
- **Risk:** Hot reloading Rust dynamic libraries (`cdylib`) requires unloading the old library and
  loading the new one. Any pointers, vtables, or type metadata from the old library become dangling.
  The `dyn Plugin` trait objects stored in the registry would point to freed memory. State
  preservation across reload requires serializing all plugin state, which depends on the reflection
  system working perfectly.
- **Likelihood:** High
- **Impact:** Medium
- **Mitigation:** Limit hot reload to editor mode only (not shipping builds). Use the reflection
  system to serialize plugin state before unload and deserialize after load. Accept that hot reload
  may fail for plugins with non-serializable state and require a full restart.

### I-6. Meshlet Pipeline and Traditional Mesh Path

- **File:** `geometry/meshlets.md`, `rendering/core-rendering.md`
- **Design element:** Visibility buffer rendering as the primary path, with indirect draw fallback
- **Risk:** The design assumes all geometry flows through the meshlet pipeline. But some geometry
  types (UI meshes, debug visualization, particle billboards, terrain patches, decals) may not have
  meshlet representations. The relationship between the meshlet visibility buffer and traditional
  rasterization is not fully specified.
- **Likelihood:** Medium
- **Impact:** Medium
- **Mitigation:** Define a hybrid rendering path: meshlet visibility buffer for 3D world geometry,
  traditional rasterization for UI, particles, and debug overlays. The render graph should compose
  both paths into the same frame.

### I-7. Transform Propagation and Physics Coupling

- **File:** `core-runtime/scene-transforms.md`, `physics/foundation.md`
- **Design element:** Hierarchical transform propagation as an ECS system, physics integration
  writing to Transform component
- **Risk:** Transform propagation traverses the scene hierarchy top-down to compute
  `GlobalTransform`. Physics writes to local `Transform` after integration. If transform propagation
  runs before physics, the global transforms are stale. If physics runs before propagation, physics
  reads stale parent transforms. The correct order is: physics -> propagation -> rendering, but the
  system scheduling must enforce this.
- **Likelihood:** Medium
- **Impact:** Medium
- **Mitigation:** Define explicit system ordering: physics integration -> transform propagation ->
  BVH update -> rendering extraction. Document this as a hard constraint in the scheduling DAG.

### I-8. Logic Graph and ECS System Boundary

- **File:** `tools/logic-graph.md`, `game-framework/scripting.md`
- **Design element:** Visual logic graph as the sole authoring mechanism for all game logic,
  compiled to bytecode and executed in ECS systems
- **Risk:** The "no-code" constraint means all game logic must be expressible as visual graphs.
  Complex gameplay systems (inventory management, quest state machines, ability cooldown tracking)
  require many graph nodes and connections, which may become unwieldy for designers. The boundary
  between "engine system" (Rust code) and "user logic" (visual graph) must be carefully drawn.
- **Likelihood:** Medium
- **Impact:** High
- **Mitigation:** Provide high-level "macro nodes" that encapsulate common gameplay patterns (timer,
  state machine, inventory operation) as single nodes. The logic graph should call into engine APIs
  via well-defined node types, not replicate engine logic. Validate with user testing that complex
  gameplay can be authored visually.

### I-9. DCC Plugin Maintenance Burden

- **File:** `content-pipeline/dcc-versioning.md`
- **Design element:** Native plugins for Houdini, Maya, Blender, Marvelous Designer, Substance,
  Photoshop (15 plugin features across 6 DCC tools)
- **Risk:** Each DCC tool has its own plugin API, versioning scheme, and breaking changes. Houdini
  updates its HDK annually, Maya's SDK changes between major versions, and Blender's Python API
  breaks frequently. Maintaining 6 DCC plugins across multiple versions is a continuous engineering
  cost.
- **Likelihood:** High
- **Impact:** Medium
- **Mitigation:** Prioritize the three most common DCC tools (Blender, Maya, Houdini) for MVP. Use
  the C API plugin SDK with stable ABI to minimize version-specific maintenance. Defer Marvelous
  Designer, Substance runtime, and Photoshop plugins to post-MVP.

---

## 5. Dependency Feasibility

### D-1. NRD (NVIDIA Real-time Denoisers)

- **File:** `rendering/advanced.md`
- **Design element:** NRD as the default denoiser for ray-traced effects
- **Risk:** NRD is proprietary NVIDIA software with license restrictions. It is not available on
  crates.io and requires direct integration of the NRD SDK (C++ library). Integration via C ABI
  adds FFI complexity. NRD only runs on NVIDIA GPUs; AMD and Intel GPUs need a different denoiser.
- **Likelihood:** High
- **Impact:** High
- **Mitigation:** Implement SVGF (spatiotemporal variance-guided filtering) as a vendor-neutral
  denoiser in HLSL. This is the same algorithm family as NRD but implemented directly without the
  proprietary SDK. NRD integration can be an optional NVIDIA-specific optimization path.

### D-2. Houdini Engine Licensing

- **File:** `content-pipeline/dcc-versioning.md`, `geometry/procedural-generation.md`
- **Design element:** Houdini Engine HDA hosting in-process or out-of-process (F-12.6.3)
- **Risk:** Houdini Engine requires a commercial license
  ($4,495/year) or Houdini Engine Indie ($269/year with revenue cap). CI/CD build machines running
  Houdini Engine for procedural content need licenses per machine. The licensing cost for a studio
  scales linearly with build farm size.
- **Likelihood:** High
- **Impact:** Medium
- **Mitigation:** Run Houdini Engine out-of-process on a dedicated build server with a single
  license. Queue HDA cook requests from CI. Document the licensing requirement prominently. Ensure
  the PCG system works without Houdini for studios that do not use it (native PCG graph nodes).

### D-3. OpenVDB Integration

- **File:** `rendering/environment-character.md`
- **Design element:** Heterogeneous volumes via OpenVDB with volumetric BSDF (F-2.7.7)
- **Risk:** OpenVDB is a large C++ library (700K+ lines) with dependencies on Boost, TBB, blosc, and
  zlib. Integrating it via C ABI would require wrapping a significant API surface. The design
  notes: "OpenVDB is a large C++ library" and suggests a minimal reader. Even a minimal VDB reader
  must handle the sparse grid traversal and compression formats.
- **Likelihood:** Medium
- **Impact:** Medium
- **Mitigation:** Implement a minimal VDB file reader in pure Rust (the VDB file format is
  documented). Only support reading pre-computed density grids; do not attempt to replicate
  OpenVDB's editing or CSG capabilities. Use the `openvdb-reader` crate if it exists, or write a
  focused parser.

### D-4. Networking Dependencies in Tool Processes

- **File:** `tools/launcher.md`, `tools/collaboration.md`, `tools/version-control.md`,
  `tools/shared-cache.md`
- **Design element:** Multiple tool subsystems need HTTP, WebSocket, and database clients
- **Risk:** Four separate design documents independently flag the need for custom networking
  clients. Without a centralized strategy, different tools may adopt inconsistent workarounds
  (custom HTTP client in one, synchronous client in another, platform-native wrapper in a third).
- **Likelihood:** Medium
- **Impact:** Medium
- **Mitigation:** All binaries use the IoReactor. The project-wide policy (see R-1) provides a
  single set of replacements: platform-native HTTP clients, `tungstenite` for WebSocket, `quinn` for
  QUIC, and custom or synchronous database clients. Shared wrapper libraries ensure consistency
  across all tool processes.

### D-5. DirectStorage and Metal IO

- **File:** `content-pipeline/streaming.md`
- **Design element:** GPU direct storage via DirectStorage (Windows) and Metal IO (macOS) for
  file-to-GPU DMA
- **Risk:** DirectStorage 1.2+ is Windows 11 only (Windows 10 support is limited to CPU
  decompression only). Metal IO is macOS 13+. Linux has no GPU direct storage equivalent. The design
  requires three completely different code paths: DirectStorage on Windows, Metal IO on macOS,
  standard async I/O + GPU upload on Linux.
- **Likelihood:** Low
- **Impact:** Medium
- **Mitigation:** Use standard async I/O + GPU transfer queue as the universal fallback path.
  DirectStorage and Metal IO are performance optimizations, not requirements. Gate them behind
  runtime capability checks.

### D-6. Vendor Upscaler SDKs (DLSS, FSR, XeSS)

- **File:** `rendering/environment-character.md`
- **Design element:** VendorUpscaler trait with DLSS, FSR, XeSS, MetalFX backends
- **Risk:** Each vendor SDK has different integration requirements: DLSS requires the NVIDIA NGX
  library (proprietary, not on crates.io), FSR requires AMD's FidelityFX SDK (open source but C++),
  XeSS requires Intel's SDK. Each SDK update may break the integration. MetalFX is Apple-only.
- **Likelihood:** Medium
- **Impact:** Low
- **Mitigation:** The design correctly includes TSR (temporal super resolution) as the
  vendor-neutral fallback. Implement TSR first. Vendor integrations are optional quality-of-life
  features. Gate each behind a feature flag and a runtime GPU vendor check.

### D-7. meshoptimizer FFI

- **File:** `geometry/meshlets.md`
- **Design element:** meshoptimizer C library via bindgen for meshlet partitioning, vertex cache
  optimization, and mesh simplification
- **Risk:** meshoptimizer is well-maintained and widely used. The `meshopt` Rust crate provides safe
  bindings. Low risk.
- **Likelihood:** Low
- **Impact:** Low
- **Mitigation:** Use the `meshopt` crate directly. This is a well-tested dependency.

### D-8. Substance Runtime Evaluation

- **File:** `content-pipeline/dcc-versioning.md`
- **Design element:** Runtime .sbsar evaluation on background thread (F-12.6.14) via Substance C API
- **Risk:** The Substance Engine runtime requires a commercial license from Adobe. The C API is
  available but not publicly documented to the same degree as the Substance Designer UI. Runtime
  evaluation on a background thread requires careful memory management of the Substance context.
- **Likelihood:** Medium
- **Impact:** Low
- **Mitigation:** Defer runtime Substance evaluation to post-MVP. Bake all Substance textures during
  asset import (offline). Runtime evaluation is a nice-to-have for dynamic material customization.

---

## Top 7 Critical and High-Impact Risks

These require immediate attention before implementation begins.

| # | Risk | Category | Likelihood | Impact |
|---|------|----------|------------|--------|
| 1 | Custom IoReactor on 3 platforms (T-1) | Technical | High | Critical |
| 2 | GCD fibers cannot yield (T-2) | Technical | High | Critical |
| 3 | 1,381 features scope (S-1) | Scale | High | Critical |
| 4 | Shared BVH contention model (I-1) | Integration | High | Critical |
| 5 | Custom runtime = limited ecosystem (R-1) | Rust Ecosystem | High | Critical |
| 6 | Metal FFI chain depth (T-4) | Technical | High | High |
| 7 | Scoped async tasks + borrow checker (R-4) | Rust Ecosystem | High | High |

---

## Recommendations

### Immediate Actions (Before Implementation)

1. **Prototype IoReactor** on all three platforms. Validate the GCD controlled drain pattern on
   macOS with Metal command buffer completion handlers before committing to the design.

2. **Resolve the macOS fiber strategy.** The current GCD-based fiber design is not implementable as
   specified. Choose between `ucontext`, `setjmp/longjmp`, or redefining fibers as async tasks.

3. **Build shared IoReactor networking libraries.** Implement platform-native HTTP wrappers,
   `tungstenite` integration, and `quinn` custom socket adapter. This unblocks all tool and server
   infrastructure design.

4. **Define the MVP feature set.** Reduce from 1,381 features to 200-300 for the first deliverable.
   Cut genre-specific features, MMO infrastructure, and advanced rendering techniques.

5. **Simplify the Metal FFI path.** Evaluate `objc2-metal` or direct Objective-C runtime bindings
   instead of the Swift intermediate layer.

### Design Revisions Needed

6. **Shared BVH update model.** Clarify the physics substep interaction. Allow physics to maintain a
   lightweight internal broadphase alongside the shared BVH.

7. **Visibility buffer fallback.** Add a 32-bit fallback path for GPUs without 64-bit atomics.

8. **Scoped async tasks.** Either remove `Scope::spawn_async` from the design or document the unsafe
   implementation strategy with safety proofs.

9. **Audio feedback channel.** Specify the audio-to-game reverse SPSC queue for events and playback
   state.

10. **Fluid simulation scope.** Remove FLIP/PIC and Eulerian from the initial design. Start with SPH
    only.
