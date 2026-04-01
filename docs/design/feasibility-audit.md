# Feasibility Audit Report

Audit date: 2026-03-15, updated 2026-03-31. Covers all design documents across 15 domains.

---

## Summary Risk Matrix

| Risk Category | Resolved | Critical | High | Medium | Low | Total |
|---------------|----------|----------|------|--------|-----|-------|
| Technical Feasibility | 3 | 0 | 3 | 6 | 2 | 14 |
| Rust Ecosystem | 5 | 0 | 0 | 3 | 1 | 9 |
| Scale Feasibility | 0 | 1 | 3 | 3 | 1 | 8 |
| Integration Feasibility | 1 | 1 | 2 | 4 | 1 | 9 |
| Dependency Feasibility | 3 | 0 | 1 | 2 | 2 | 8 |
| **Total** | **12** | **2** | **9** | **18** | **7** | **48** |

---

## 1. Technical Feasibility

### T-1. Custom IoReactor on Three Platforms — RESOLVED

- **Status:** Resolved. The custom IoReactor has been replaced by Tokio `current_thread`. Tokio
  handles all platform I/O internally (epoll on Linux, kqueue on macOS, IOCP on Windows). The game
  loop controls when futures are polled via `runtime.poll()` at frame boundaries.

### T-2. GCD Fibers via swift-bridge — RESOLVED

- **Status:** Resolved. GCD is no longer used for I/O or fibers. Tokio handles all async I/O. Fibers
  on macOS use async/await coroutines that yield at `.await` points.

### T-3. Custom Windowing System (No Winit)

- **File:** `platform/windowing.md`, `constraints.md`
- **Design element:** Platform-native windowing via Win32, NSWindow (swift-bridge), xcb, and Wayland
- **Risk:** Implementing window lifecycle, fullscreen modes, DPI scaling, HDR output, VSync, display
  enumeration, and multi-monitor on four backends. Wayland is particularly complex (xdg-shell,
  fractional-scale, decoration).
- **Likelihood:** Medium
- **Impact:** High
- **Mitigation:** Prioritize Win32 and macOS first. Start with X11/xcb for Linux; defer Wayland.
  Budget 3-6 months.

### T-4. Metal Backend via swift-bridge

- **File:** `rendering/gpu-abstraction.md`
- **Design element:** Metal backend accessed via Swift through swift-bridge (F-2.1.4)
- **Risk:** swift-bridge does not yet cover every Swift type (some generics, complex enums), and
  async/await bridging requires manual shims. Manageable with targeted wrapper types on the Swift
  side.
- **Likelihood:** Low
- **Impact:** Medium
- **Mitigation:** Use swift-bridge for all Metal API bindings. Write thin Swift wrapper structs for
  unsupported types. Pin swift-bridge to a known-good version.

### T-5. GJK/EPA/SAT Physics from Scratch

- **File:** `physics/foundation.md`
- **Design element:** Full narrowphase contact generation, rigid body dynamics, island solver, CCD
  in pure Rust
- **Risk:** GJK edge cases, EPA robustness, and stable contact manifold generation are notoriously
  difficult. Most engines use PhysX or Bullet.
- **Likelihood:** Medium
- **Impact:** High
- **Mitigation:** Use `parry3d` for narrowphase geometry operations. Build ECS integration, island
  solver, and broadphase (shared BVH) in-house.

### T-6. Fluid Simulation (SPH, FLIP/PIC, Eulerian)

- **File:** `physics/advanced.md`
- **Design element:** Three fluid simulation methods with GPU acceleration and two-way rigid body
  coupling
- **Risk:** Each method is research-grade. Real-time GPU fluid with rigid body coupling is frontier
  technology.
- **Likelihood:** High
- **Impact:** Medium
- **Mitigation:** Start with SPH only. Defer FLIP/PIC and Eulerian to post-MVP. Screen-space water
  for large bodies.

### T-7. Neural Denoiser and NRD Integration

- **File:** `rendering/advanced.md`
- **Design element:** Neural network denoiser with NRD fallback (proprietary NVIDIA SDK)
- **Risk:** Neural denoising requires ML inference infrastructure. NRD is NVIDIA-proprietary.
- **Likelihood:** Medium
- **Impact:** Medium
- **Mitigation:** Implement vendor-neutral SVGF in HLSL. Neural denoising is nice-to-have, not a
  dependency.

### T-8. Visibility Buffer 64-bit Atomics

- **File:** `geometry/meshlets.md`
- **Design element:** Visibility buffer with 64-bit atomic writes per pixel
- **Risk:** 64-bit shader atomics are not universally available. Older Intel, pre-RDNA2 AMD, and
  pre-M1 Apple GPUs lack them. No fallback path.
- **Likelihood:** Medium
- **Impact:** High
- **Mitigation:** Design a 32-bit visibility buffer fallback with split-field encoding. Capability
  gating selects path.

### T-9. Sound Propagation via Hybrid Ray-Portal

- **File:** `audio/engine.md`
- **Design element:** Occlusion/obstruction via shared BVH; hybrid ray-portal propagation solver
- **Risk:** Physically-based sound propagation is computationally expensive and research-active.
- **Likelihood:** Medium
- **Impact:** Medium
- **Mitigation:** Start with simple ray-based occlusion and portal propagation for indoor scenes.
  Defer full acoustic simulation.

### T-10. Planetary-Scale Voxel Terrain

- **File:** `geometry/terrain.md`
- **Design element:** Planetary-scale voxel sphere with radial gravity, GPU SDF meshing via dual
  contouring
- **Risk:** Cutting-edge technology. No shipping engine provides this as built-in. GPU dual
  contouring on sparse octrees requires careful memory management.
- **Likelihood:** High
- **Impact:** Medium
- **Mitigation:** Treat as experimental/optional. Heightfield CDLOD is the primary supported path.
  Defer to post-MVP.

### T-11. CRDT Collaborative Editing — RESOLVED

- **Status:** Resolved. CRDTs and real-time collaboration have been removed from the design. Version
  control uses Git + Git LFS with lock-based concurrency for binary assets. Binary conflicts are
  resolved via a structural merge UI that parses both versions into memory for per-property
  comparison.

### T-12. Shape Grammar Building Generation

- **File:** `geometry/procedural-generation.md`
- **Design element:** Shape grammar building generator and L-system road networks
- **Risk:** PhD-level implementations. Complex to implement correctly with intersection handling.
- **Likelihood:** Medium
- **Impact:** Low
- **Mitigation:** Start with modular building assembly via socket snapping. Defer full shape grammar
  evaluation.

### T-13. Adaptive Music Beat-Sync Transitions

- **File:** `audio/dsp-music.md`
- **Design element:** Horizontal re-sequencing via segment directed graph with beat-sync transitions
- **Risk:** Sample-accurate beat-synchronized transitions require a tempo-aware scheduler within <
  0.5 ms budget.
- **Likelihood:** Low
- **Impact:** Medium
- **Mitigation:** Start with bar-boundary transitions and crossfade. Beat-accurate sub-bar
  transitions added later.

### T-14. Stochastic Many-Light Sampling

- **File:** `rendering/lighting.md`
- **Design element:** ReSTIR-style many-light sampling with temporal denoiser
- **Risk:** Recent technique (2020+), requires careful GPU reservoir sampling with disocclusion
  handling.
- **Likelihood:** Medium
- **Impact:** Medium
- **Mitigation:** Tiled/clustered forward+ supports 500+ lights. Stochastic only needed for 10,000+.
  Defer.

### T-15. Volumetric Shadow Maps (Fourier Opacity)

- **File:** `rendering/lighting.md`
- **Design element:** Volumetric shadow maps with Fourier opacity mapping
- **Risk:** Specialized technique with limited adoption. Light leaking at high-frequency boundaries.
- **Likelihood:** Low
- **Impact:** Low
- **Mitigation:** Use standard volumetric ray marching as primary. Fourier is a quality tier option.

---

## 2. Rust Ecosystem Feasibility

### R-1. Custom Async Runtime — RESOLVED

- **Status:** Resolved. Tokio `current_thread` is the sole async runtime. Full Tokio ecosystem
  (reqwest, sqlx, quinn, tokio-tungstenite, hyper, axum) is available. No custom IoReactor, no
  platform-native wrappers needed.

### R-2. GCD Callback Integration — RESOLVED

- **Status:** Resolved. GCD is no longer used for I/O. Tokio handles all I/O. swift-bridge is
  retained only for Metal API access.

### R-3. bevy_reflect-Style Reflection from Scratch

- **File:** `core-runtime/reflection-serialization.md`
- **Design element:** Custom Reflect trait, TypeRegistry, derive macro, property path access,
  dynamic values
- **Risk:** bevy_reflect is ~20,000 lines with complex proc macros. Reimplementing from scratch is
  significant.
- **Likelihood:** Medium
- **Impact:** High
- **Mitigation:** Start with minimal subset (struct reflection only). Budget 2-3 months for derive
  macro and registry. Place in Wave 1.

### R-4. Scoped Async Tasks — RESOLVED

- **Status:** Resolved. Scoped async tasks have been removed from the design. All async tasks use
  `'static` futures with `Arc` for shared data. Scoped synchronous tasks remain for CPU parallel
  compute.

### R-5. ECS Query System Ergonomics

- **File:** `core-runtime/ecs.md`
- **Design element:** Composable archetype queries with parallel iteration and change detection
- **Risk:** Building a sound query system that prevents mutable aliasing across parallel iterators
  is hard. Mitigated by using bevy_ecs as the ECS foundation — leveraging its existing query safety
  model (FilteredAccess, archetype-level conflict detection).
- **Likelihood:** Low
- **Impact:** Medium
- **Mitigation:** Use bevy_ecs as foundation. Build custom extensions (relationship pairs, query
  variables, shared components) on top of bevy's proven query system.

### R-6. Proc Macro for ECS Component Derive

- **File:** `core-runtime/ecs.md`, `core-runtime/reflection-serialization.md`
- **Design element:** Derive macros for Component, Reflect, Serialize
- **Risk:** Multiple interacting proc macros are complex. Mitigated by leveraging bevy's existing
  derive macros.
- **Likelihood:** Low
- **Impact:** Medium
- **Mitigation:** Use bevy's Component derive. Build custom derives only for engine-specific traits.

### R-7. Zero-Copy Deserialization Safety

- **File:** `core-runtime/reflection-serialization.md`
- **Design element:** Zero-copy deserialization via mmap, `unsafe fn field_as<T>`
- **Risk:** Alignment violations cause UB. Type confusion can corrupt memory.
- **Likelihood:** Medium
- **Impact:** Medium
- **Mitigation:** Use `bytemuck::Pod` or `zerocopy` traits. Require all zero-copy types to derive
  `Pod`. Keep unsafe internal; expose only safe wrappers.

### R-8. setjmp/longjmp Fibers on Linux

- **File:** `platform/threading.md`
- **Design element:** Linux fibers via custom assembly context switch
- **Risk:** `setjmp`/`longjmp` with explicit stacks is UB when crossing unwound frames. Rust drop
  semantics compound this.
- **Likelihood:** Medium
- **Impact:** Medium
- **Mitigation:** Use the `context` or `minicoroutine` crate. Ensure fibers only yield at controlled
  points with no active Drop guards.

### R-9. HLSL Shader Pipeline — RESOLVED

- **Status:** Resolved. DXC and Metal Shader Converter are invoked as CLI subprocesses. No embedded
  libraries, no COM FFI, no swift-bridge for shaders. The mitigation from the original audit ("use
  DXC CLI for offline compilation") is now the actual design.

### R-10. Async Database Client — RESOLVED

- **Status:** Resolved. `sqlx` with Tokio provides async PostgreSQL with TLS, connection pooling,
  prepared statements, and SASL authentication. No custom wire protocol client needed.

---

## 3. Scale Feasibility

### S-1. 1,381 Features as a Deliverable

- **File:** `plan.md`
- **Design element:** 1,381 features, 1,171 requirements, 5,859 user stories across 15 domains
- **Risk:** Even with a 10-person team, this is 27+ person-years. With overhead, 40+ person-years.
- **Likelihood:** High
- **Impact:** Critical
- **Mitigation:** Define strict MVP (200-300 features). Target single genre (3D action) and single
  platform (Windows). Defer genre-specific, advanced rendering, and social features.

### S-2. Shared BVH at Million-Entity Scale

- **File:** `core-runtime/spatial-index.md`
- **Design element:** Single shared BVH for all subsystems. Target: ray cast < 10 us at 1M entities.
- **Risk:** Contention if physics moves 10K bodies/frame. < 10 us ray cast at 1M is achievable but
  requires SIMD and careful memory layout.
- **Likelihood:** Medium
- **Impact:** High
- **Mitigation:** Grid/octree hybrid alongside BVH. Grid for cell-based queries, BVH for geometric
  queries.

### S-3. MMO: Thousands of Players Per Shard

- **File:** `networking/mmo.md`
- **Design element:** 500 players per shard, dynamic zone splitting, < 100 ms migration
- **Risk:** At 500 players with full ECS simulation, each zone server must handle replication,
  prediction, spatial queries, AI, and physics within 0.5 ms.
- **Likelihood:** Medium
- **Impact:** High
- **Mitigation:** Start with 64-player servers. Scale to MMO incrementally. Validate with synthetic
  load tests.

### S-4. GPU Particle Throughput (1M Particles)

- **File:** `vfx/particles.md`
- **Design element:** 1M+ particles per frame at 60 fps
- **Risk:** Achievable on desktop GPUs. < 0.5 ms GPU sort for 100K particles is tight but feasible.
- **Likelihood:** Low
- **Impact:** Medium
- **Mitigation:** Platform tiers. Validate GPU sort early.

### S-5. AOI Evaluation (1K Clients, 100K Entities)

- **File:** `networking/replication.md`
- **Design element:** Area-of-interest filtering for 1K clients with 100K entities, < 2 ms per tick
- **Risk:** 100M pairwise checks per tick. Even with spatial indexing, < 2 ms is aggressive.
- **Likelihood:** Medium
- **Impact:** Medium
- **Mitigation:** Uniform grid for AOI. Batch relevancy evaluation across ticks (round-robin).

### S-6. Content Pipeline Asset Volume

- **File:** `content-pipeline/asset-import.md`
- **Design element:** Batch import, CAS database, incremental builds for 100K+ assets
- **Risk:** CAS database must handle BLAKE3 lookups and dependency graphs at 100K+ scale.
- **Likelihood:** Low
- **Impact:** Medium
- **Mitigation:** Use SQLite or LMDB for metadata. Validate incremental build at 100K scale early.

### S-7. Foliage: 1M+ Instances

- **File:** `geometry/environment.md`
- **Design element:** 1M+ visible foliage instances with GPU compute culling in < 1 ms
- **Risk:** < 1 ms for 1M instances is feasible but tight. Wind animation adds further cost.
- **Likelihood:** Low
- **Impact:** Low
- **Mitigation:** Hierarchical culling (cluster-level). Meshlet visibility buffer handles foliage
  efficiently.

### S-8. Scripting: 1,000 Active Graphs < 4 ms

- **File:** `game-framework/scripting.md`
- **Design element:** 1,000 active logic graphs with 10 nodes each in < 4 ms
- **Risk:** 10,000 node evaluations at 400 ns each. Graphs reading ECS data add cache miss overhead.
- **Likelihood:** Low
- **Impact:** Medium
- **Mitigation:** Compile to native code (not bytecode VM). Batch same-type graphs for cache
  locality.

---

## 4. Integration Feasibility

### I-1. Shared BVH Contention Model

- **File:** `core-runtime/spatial-index.md`, `physics/foundation.md`
- **Design element:** Single BVH read by all subsystems; updated once per frame
- **Risk:** Physics substeps read stale positions. Semantic mismatch: physics needs current, BVH has
  start-of-frame.
- **Likelihood:** High
- **Impact:** Critical
- **Mitigation:** Physics maintains lightweight internal broadphase alongside shared BVH. BVH is
  inter-subsystem; physics uses narrow-phase cache within substeps.

### I-2. Audio Thread vs ECS Constraint

- **File:** `constraints.md`, `audio/engine.md`
- **Design element:** Audio mixer on dedicated real-time thread with < 0.5 ms budget, bridged to ECS
  via SPSC
- **Risk:** Reverse channel (audio-to-ECS feedback) not fully specified.
- **Likelihood:** Medium
- **Impact:** High
- **Mitigation:** Bidirectional SPSC pair. Game thread reads feedback queue during ECS update.
  One-frame latency is acceptable.

### I-3. Render Proxy Extraction Overhead

- **File:** `rendering/core-rendering.md`
- **Design element:** Extract phase copies changed ECS data into ProxyStore. Target: < 2 ms for 100K
  entities.
- **Risk:** Extraction blocks simulation. 2 ms at 100% dirty is tight.
- **Likelihood:** Medium
- **Impact:** Medium
- **Mitigation:** Double-buffered ECS or pipelined extraction (one frame latency). Standard AAA
  practice.

### I-4. Cross-Domain Event Ordering

- **File:** `core-runtime/events-plugins.md`
- **Design element:** Typed double-buffered event channels with component lifecycle observers
- **Risk:** Observer ordering within sync points is unspecified. Non-deterministic behavior
  possible.
- **Likelihood:** Medium
- **Impact:** Medium
- **Mitigation:** Explicit observer priority ordering. Enforce physics -> AI -> gameplay via
  scheduling DAG.

### I-5. Plugin Hot Reload Safety

- **File:** `core-runtime/events-plugins.md`
- **Design element:** Plugin hot reload with state preservation
- **Risk:** Unloading cdylib invalidates pointers, vtables, type metadata. State preservation
  depends on reflection.
- **Likelihood:** High
- **Impact:** Medium
- **Mitigation:** Editor-only. Serialize plugin state via reflection before unload. Accept restart
  for non-serializable state.

### I-6. Meshlet Pipeline and Traditional Mesh Path

- **File:** `geometry/meshlets.md`, `rendering/core-rendering.md`
- **Design element:** Visibility buffer as primary, indirect draw fallback
- **Risk:** Some geometry (UI, debug, particles, terrain, decals) may not have meshlet
  representations.
- **Likelihood:** Medium
- **Impact:** Medium
- **Mitigation:** Hybrid rendering: meshlet visibility buffer for 3D world, traditional
  rasterization for UI/particles.

### I-7. Transform Propagation and Physics Coupling

- **File:** `core-runtime/scene-transforms.md`, `physics/foundation.md`
- **Design element:** Hierarchical transform propagation as ECS system, physics writes to Transform
- **Risk:** Ordering: physics -> propagation -> rendering must be enforced by scheduling.
- **Likelihood:** Medium
- **Impact:** Medium
- **Mitigation:** Explicit system ordering in scheduling DAG: physics -> propagation -> BVH update
  -> render extract.

### I-8. Logic Graph and ECS System Boundary

- **File:** `tools/logic-graph.md`, `game-framework/scripting.md`
- **Design element:** Visual logic graph compiled to native ECS systems (no bytecode VM)
- **Risk:** Complex gameplay may be unwieldy as visual graphs. Engine/user-logic boundary must be
  clear.
- **Likelihood:** Medium
- **Impact:** High
- **Mitigation:** Macro nodes (visual grouping containers) for common patterns. Keyboard-first
  workflow (Game Maker style) for efficient authoring. Validate with user testing.

### I-9. DCC Plugin Maintenance Burden — RESOLVED

- **Status:** Resolved. DCC plugins have been removed from the design. The engine accepts standard
  intermediate formats (glTF 2.0, Alembic, PNG, EXR, KTX2, WAV, FLAC). Maintenance burden
  eliminated.

---

## 5. Dependency Feasibility

### D-1. NRD (NVIDIA Real-time Denoisers)

- **File:** `rendering/advanced.md`
- **Design element:** NRD as default denoiser
- **Risk:** Proprietary NVIDIA SDK. C++ library requiring C ABI FFI. NVIDIA GPUs only.
- **Likelihood:** High
- **Impact:** High
- **Mitigation:** Implement SVGF in HLSL as vendor-neutral denoiser. NRD as optional NVIDIA-specific
  optimization.

### D-2. Houdini Engine Licensing — RESOLVED

- **Status:** Resolved. DCC plugins removed. No Houdini Engine integration. PCG system uses native
  graph nodes.

### D-3. OpenVDB Integration

- **File:** `rendering/environment-character.md`
- **Design element:** Heterogeneous volumes via OpenVDB
- **Risk:** OpenVDB is 700K+ lines C++ with Boost, TBB, blosc, zlib dependencies.
- **Likelihood:** Medium
- **Impact:** Medium
- **Mitigation:** Minimal VDB file reader in pure Rust. Read pre-computed density grids only.

### D-4. Networking Dependencies — RESOLVED

- **Status:** Resolved. Tokio ecosystem provides all networking: reqwest for HTTP, tokio-tungstenite
  for WebSocket, sqlx for PostgreSQL, quinn for QUIC.

### D-5. DirectStorage and Metal IO

- **File:** `content-pipeline/streaming.md`
- **Design element:** GPU direct storage via DirectStorage (Windows) and Metal IO (macOS)
- **Risk:** DirectStorage is Windows 11 only. Metal IO is macOS 13+. Linux has no equivalent.
- **Likelihood:** Low
- **Impact:** Medium
- **Mitigation:** Standard async I/O + GPU transfer queue as universal fallback. DirectStorage and
  Metal IO are performance optimizations, not requirements.

### D-6. Vendor Upscaler SDKs (DLSS, FSR, XeSS)

- **File:** `rendering/environment-character.md`
- **Design element:** VendorUpscaler trait with DLSS, FSR, XeSS, MetalFX backends
- **Risk:** Each vendor SDK has different requirements. Proprietary, not on crates.io.
- **Likelihood:** Medium
- **Impact:** Low
- **Mitigation:** Implement TSR (temporal super resolution) as vendor-neutral fallback first. Vendor
  integrations are optional quality-of-life features.

### D-7. meshoptimizer FFI

- **File:** `geometry/meshlets.md`
- **Design element:** meshoptimizer C library via `meshopt` Rust crate
- **Risk:** Well-maintained, widely used. Low risk.
- **Likelihood:** Low
- **Impact:** Low
- **Mitigation:** Use `meshopt` crate directly.

### D-8. Substance Runtime Evaluation — RESOLVED

- **Status:** Resolved. DCC plugins removed. No runtime Substance evaluation. Substance textures
  baked during asset import (offline) via standard format export.

---

## Top Remaining Risks

After design revisions, 12 of the original 50 risks are resolved. The remaining highest-impact
risks:

| # | Risk | Category | Likelihood | Impact |
|---|------|----------|------------|--------|
| 1 | 1,381 features scope (S-1) | Scale | High | Critical |
| 2 | Shared BVH contention (I-1) | Integration | High | Critical |
| 3 | Custom windowing (T-3) | Technical | Medium | High |
| 4 | Physics from scratch (T-5) | Technical | Medium | High |
| 5 | Visibility buffer atomics (T-8) | Technical | Medium | High |
| 6 | Logic graph UX (I-8) | Integration | Medium | High |
| 7 | NRD licensing (D-1) | Dependency | High | High |

---

## Recommendations

### Immediate Actions (Before Implementation)

1. **Define the MVP feature set.** Reduce from 1,381 features to 200-300. Cut genre-specific
   features, MMO infrastructure, and advanced rendering.

2. **Validate swift-bridge coverage.** Prototype Metal device creation and command buffer submission
   through swift-bridge to confirm type coverage.

3. **Prototype Tokio game loop integration.** Validate that `current_thread` runtime with `poll()`
   at frame boundaries provides deterministic poll timing without unexpected latency spikes.

### Design Revisions Needed

4. **Shared BVH update model.** Clarify physics substep interaction. Allow physics to maintain
   lightweight internal broadphase alongside shared BVH.

5. **Visibility buffer fallback.** Add a 32-bit fallback path for GPUs without 64-bit atomics.

6. **Audio feedback channel.** Specify the audio-to-game reverse SPSC queue for events and playback
   state.

7. **Fluid simulation scope.** Remove FLIP/PIC and Eulerian. Start with SPH only.

### Resolved by Design Changes

8. ~~Custom IoReactor~~ — replaced by Tokio
9. ~~GCD fibers~~ — replaced by async/await coroutines
10. ~~Custom async runtime ecosystem~~ — Tokio ecosystem
11. ~~Scoped async tasks~~ — removed, all `'static` futures
12. ~~DXC/MSC FFI~~ — CLI subprocesses
13. ~~DCC plugin maintenance~~ — intermediate formats
14. ~~CRDT collaboration~~ — Git + Git LFS
15. ~~Custom database client~~ — sqlx
16. ~~Custom networking~~ — Tokio ecosystem
17. ~~Houdini Engine licensing~~ — no DCC plugins
18. ~~Substance runtime~~ — no DCC plugins
