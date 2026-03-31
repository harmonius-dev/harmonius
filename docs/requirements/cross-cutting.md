# Cross-Cutting Requirements

Requirements that span multiple domains, covering inter-system integration, performance budgets,
threading, error handling, determinism, and other concerns that no single domain owns.

---

## Performance Budgets

| ID | Requirement |
|----|-------------|
| R-X.1.1 | Per-system frame time budgets |
| R-X.1.2 | GPU memory subsystem pools |
| R-X.1.3 | System RAM pressure response |

1. **R-X.1.1** — The engine **SHALL** define per-system frame time budgets for 30fps (33.3ms), 60fps
   (16.67ms), and 120fps (8.33ms) targets as configurable project settings. Default allocations at
   60fps: rendering 8ms, physics 2ms, animation 1.5ms, AI 1ms, audio 0.5ms, networking 0.5ms, game
   framework 2ms, overhead 1.67ms. Each system **SHALL** report its per-frame CPU and GPU time via
   the profiling API. When total frame time exceeds the target, the budget arbiter **SHALL** request
   load-shedding from systems in priority order: VFX first, then rendering quality (dynamic
   resolution), then AI LOD, then physics substep reduction.
   - **Rationale:** Without coordinated budgets, individual systems shed load independently and may
     still exceed the frame target. A central arbiter ensures the most impactful reductions happen
     first.
   - **Verification:** Integration test: artificially overload physics to 4ms at 60fps target;
     verify the arbiter reduces physics substeps before reducing rendering quality.
2. **R-X.1.2** — The engine **SHALL** partition GPU memory into subsystem pools with configurable
   sizes per platform tier (8GB, 12GB, 16GB VRAM targets). Default at 12GB: texture streaming pool
   4GB, mesh streaming pool 1GB, render targets 2GB, acceleration structures 1GB, particle buffers
   512MB, UI atlases 256MB, shadow maps 1GB, reserve 2.25GB. Each pool **SHALL** enforce its limit —
   allocations exceeding the pool size trigger eviction of lowest-priority resident data. The
   streaming system **SHALL** coordinate with the rendering system to evict textures before render
   targets when under global VRAM pressure.
   - **Rationale:** Without coordinated VRAM budgets, the texture streaming system can starve the
     particle buffer or render target pools, causing rendering failures.
   - **Verification:** Stress test: load a scene requiring 14GB of texture data on a 12GB GPU;
     verify the streaming pool caps at 4GB and evicts lowest-priority textures without affecting
     render target allocation.
3. **R-X.1.3** — The engine **SHALL** track system RAM usage per subsystem (ECS storage, audio
   decode buffers, NavMesh tiles, physics islands, animation bone palettes, UI widget trees) and
   expose current usage through the profiling API. When total system RAM exceeds 80% of available
   physical memory, the engine **SHALL** trigger memory pressure callbacks in each subsystem, which
   respond by releasing cached data (decoded audio, uncompressed textures, distant NavMesh tiles).
   - **Rationale:** OOM crashes are unrecoverable. Proactive pressure response prevents them.
   - **Verification:** Benchmark: allocate memory in a test subsystem until 80% threshold; verify
     pressure callbacks fire and other subsystems release cached data within 100ms.

---

## Threading Model

| ID | Requirement |
|----|-------------|
| R-X.2.1 | Subsystem thread assignment |
| R-X.2.2 | Pipelined frame execution |
| R-X.2.3 | Real-time audio thread constraints |

1. **R-X.2.1** — The engine **SHALL** assign each subsystem's main loop to a designated thread or
   thread pool: game loop thread (ECS system scheduling, game framework logic, input processing),
   render thread (render graph execution, GPU command encoding), audio thread (mixer callback, DSP
   processing), network thread (packet I/O, serialization), and worker pool (physics substeps, AI
   pathfinding, animation evaluation, asset processing, procedural generation). No subsystem
   **SHALL** execute its primary workload on a thread owned by another subsystem without explicit
   synchronization.
   - **Rationale:** Implicit thread sharing causes priority inversion and unpredictable latency.
     Explicit ownership enables targeted profiling and priority tuning.
   - **Verification:** Profiling test: capture a frame trace; verify each subsystem's work executes
     only on its designated thread(s) with no unexpected cross-thread execution.
2. **R-X.2.2** — The engine **SHALL** support pipelined frame execution where simulation frame N+1
   overlaps with rendering of frame N. The pipeline **SHALL** use double-buffered ECS extraction:
   the render thread reads from a snapshot of the previous frame's render-relevant components while
   the game loop thread advances simulation. GPU frames in flight **SHALL** be configurable (1-3)
   with default of 2. The frame pipeline **SHALL** guarantee that no simulation write races with a
   render read — extraction produces an immutable snapshot consumed by the render thread.
   - **Rationale:** Overlapping simulation and rendering is essential for hitting 60fps+ on complex
     scenes. Without explicit pipelining, the render thread idles during simulation.
   - **Verification:** Integration test: run a physics-heavy scene at 60fps; verify render thread
     utilization exceeds 80% (not blocked waiting for simulation).
3. **R-X.2.3** — The audio callback thread **SHALL NOT** perform heap allocations, blocking I/O,
   mutex contention, or system calls during the audio callback. All audio data consumed by the
   callback (decoded samples, DSP parameters, spatial positions) **SHALL** be communicated via
   lock-free ring buffers or atomic variables. Violation of real-time constraints **SHALL** be
   detectable by a debug-mode watchdog that logs warnings when the callback exceeds its deadline.
   - **Rationale:** Audio glitches (pops, dropouts) are immediately perceptible. Real-time audio
     requires hard guarantees that no operation blocks the callback.
   - **Verification:** Stress test: run 128 simultaneous voices with DSP chains; verify zero audio
     underruns over 60 seconds of playback. Debug watchdog reports zero deadline violations.

---

## Error Propagation

| ID | Requirement |
|----|-------------|
| R-X.3.1 | GPU device loss recovery |
| R-X.3.2 | Network disconnection resilience |
| R-X.3.3 | Fallback assets for failed loads |

1. **R-X.3.1** — The engine **SHALL** detect GPU device loss (D3D12 device removed, Vulkan device
   lost, Metal command buffer error) and orchestrate recovery: (1) notify the render thread to
   abandon in-flight frames, (2) release all GPU resources, (3) recreate the device and swapchain,
   (4) re-upload persistent resources (shaders, pipelines, static buffers), (5) resume rendering
   from the next simulation frame. Game state **SHALL NOT** be lost during recovery. The recovery
   sequence **SHALL** complete within 5 seconds on reference hardware.
   - **Rationale:** GPU device loss occurs from driver crashes, TDR timeouts, and hot-plug events.
     Without recovery, the application must terminate.
   - **Verification:** Integration test: simulate device removal via debug layer; verify rendering
     resumes within 5 seconds with no game state corruption.
2. **R-X.3.2** — The engine **SHALL** detect network disconnection within 5 seconds (no keepalive
   response) and transition affected systems to offline mode: physics continues locally without
   server reconciliation, AI continues with last-known state, animation plays locally without
   network sync, UI displays a reconnection indicator. On reconnection, the engine **SHALL**
   resynchronize state within 10 seconds without requiring a full reload. Reconnection **SHALL**
   preserve the player's position, inventory, and quest state.
   - **Rationale:** Network disruptions are common. Players must not lose progress or be forced to
     restart the game.
   - **Verification:** Integration test: sever the network connection for 30 seconds; verify game
     continues locally, reconnects, and resynchronizes within 10 seconds of restoration.
3. **R-X.3.3** — The engine **SHALL** provide fallback assets (magenta checkerboard texture, unit
   cube mesh, silent audio clip, identity animation pose) for every asset type. When an asset fails
   to load (corrupt data, missing file, format version mismatch), the engine **SHALL** substitute
   the fallback asset, log an error with the asset path and failure reason, and continue execution
   without crashing. The error **SHALL** be surfaced in the editor's asset browser as a visual
   warning badge on the failed asset.
   - **Rationale:** A single corrupt asset must not crash the entire engine. Fallback assets make
     errors visible without halting production.
   - **Verification:** Unit test: attempt to load a deliberately corrupt texture, mesh, audio, and
     animation asset; verify each returns the fallback and logs the error.

---

## Startup and Shutdown

| ID | Requirement |
|----|-------------|
| R-X.4.1 | System initialization order |
| R-X.4.2 | Reverse shutdown with flush |

1. **R-X.4.1** — The engine **SHALL** initialize systems in this order: (1) Platform (windowing,
   threading, filesystem), (2) Core Runtime (ECS, reflection, memory, async I/O, spatial index), (3)
   Content Pipeline (asset database, streaming), (4) Rendering (GPU device, render graph), (5) Audio
   (mixer, output device), (6) Input (device enumeration), (7) Physics, (8) AI, (9) Animation, (10)
   Networking, (11) VFX, (12) UI, (13) Game Framework, (14) Tools/Editor. Each system **SHALL**
   verify its dependencies are initialized before proceeding. Initialization failure **SHALL** be
   reported with the failed system name and error, and the engine **SHALL** attempt to continue
   without the failed system if it is optional.
   - **Rationale:** Undefined initialization order causes race conditions and inscrutable startup
     crashes. Explicit ordering makes failures diagnosable.
   - **Verification:** Integration test: disable a mid-level system (audio); verify the engine
     starts successfully with audio-dependent features gracefully degraded.
2. **R-X.4.2** — The engine **SHALL** shut down systems in reverse initialization order. Before
   shutting down each system, the engine **SHALL** (1) cancel in-flight async operations with a
   2-second timeout, (2) flush pending writes (save data, network packets, log buffers), (3) release
   system resources. If a system fails to shut down within 5 seconds, the engine **SHALL** log the
   timeout and proceed with the next system. The shutdown sequence **SHALL** guarantee that no data
   loss occurs for save files or persistent state.
   - **Rationale:** Unclean shutdown can corrupt save files, leak GPU resources, and leave orphaned
     network connections.
   - **Verification:** Integration test: trigger shutdown during active gameplay with pending saves,
     in-flight network packets, and streaming operations; verify all save data is flushed and no
     resource leaks are reported.

---

## Determinism

| ID | Requirement |
|----|-------------|
| R-X.5.1 | Cross-platform physics determinism |
| R-X.5.2 | Per-system seeded RNG streams |

1. **R-X.5.1** — The engine **SHALL** produce bit-identical physics simulation results across
   Windows (x86-64), macOS (ARM64), and Linux (x86-64) given identical initial state and input
   sequence. This **SHALL** be achieved by using strict floating-point ordering (no reassociation),
   deterministic iteration order over ECS archetypes, and platform-independent math functions (no
   platform libc transcendentals for physics). The determinism guarantee **SHALL** cover rigid body
   integration, collision detection, constraint solving, and character controller movement.
   - **Rationale:** Server-authoritative networking and replay systems require that the same inputs
     produce the same outputs on any platform. Non-determinism causes prediction mispredictions and
     replay divergence.
   - **Verification:** Cross-platform test: run identical 60-second physics simulation on all 3
     platforms; compare per-frame state snapshots for bit-exact equality.
2. **R-X.5.2** — All systems that use random number generation (procedural generation, loot tables,
   AI decision variance, particle spawning, damage rolls) **SHALL** use seeded pseudo-random number
   generators derived from a per-session master seed. The master seed **SHALL** be logged and
   reproducible. Each system **SHALL** maintain its own RNG stream to prevent cross-system ordering
   dependencies from affecting determinism. The seeding strategy **SHALL** ensure that adding or
   removing entities in one system does not change the RNG sequence in another system.
   - **Rationale:** Shared RNG state creates fragile determinism that breaks when any system's
     evaluation order changes. Per-system streams isolate randomness.
   - **Verification:** Replay test: play a 5-minute session, record the seed; replay with the same
     seed; verify all procedural generation, drops, and AI decisions are identical.

---

## Hot Reload Coordination

| ID | Requirement |
|----|-------------|
| R-X.6.1 | Sync-point asset hot reload |

1. **R-X.6.1** — Asset hot reload **SHALL** only take effect at designated sync points between
   frames — never mid-frame or mid-system-execution. The hot reload coordinator **SHALL** collect
   all pending asset changes during a frame, then apply them atomically at the sync point. Systems
   affected by hot reload **SHALL** be notified via the ECS observer system with the set of changed
   asset handles, enabling them to refresh their derived state (re-bake lightmaps, rebuild NavMesh
   tiles, recompile shaders, refresh UI layout).
   - **Rationale:** Mid-frame hot reload causes data races and visual artifacts. Sync-point
     application ensures all systems see a consistent asset state.
   - **Verification:** Integration test: modify a material asset during rendering; verify the change
     applies at the next frame boundary (not mid-draw) and all dependent systems update atomically.

---

## Save/Load Scope

| ID | Requirement |
|----|-------------|
| R-X.7.1 | Serialized vs reconstructed state |

1. **R-X.7.1** — Each system **SHALL** document which runtime state is serialized on save and which
   is reconstructed on load. The following **SHALL** be serialized: ECS world state (all persistent
   entity components), player inventory, quest progress, character customization, ability cooldowns,
   world time, weather state, building placements, NPC relationship values, faction standings, and
   achievement progress. The following **SHALL** be reconstructed from assets on load (not
   serialized): NavMesh tiles, streaming residency, audio playback positions, particle system state,
   render state (pipelines, buffers), and cached spatial index data.
   - **Rationale:** Over-serializing bloats save files; under-serializing loses player progress.
     Explicit scope prevents both.
   - **Verification:** Round-trip test: save game state, load into a fresh session; verify all
     serialized state matches the saved state and all reconstructed state is functional.

---

## Platform Parity

| ID | Requirement |
|----|-------------|
| R-X.8.1 | Cross-platform feature parity matrix |

1. **R-X.8.1** — The engine **SHALL** maintain a platform parity matrix documenting which features
   are identical across platforms, which have platform-specific implementations with identical
   behavior, and which are platform-exclusive. At minimum: ECS behavior, physics simulation, save
   format, and networking protocol **SHALL** be bit-identical across Windows, macOS, and Linux. GPU
   rendering **SHALL** produce visually equivalent results (not bit-identical due to GPU hardware
   differences). Platform-exclusive features (DirectStorage, Metal I/O, console certification)
   **SHALL NOT** be dependencies of platform-agnostic systems.
   - **Rationale:** Platform-specific code that leaks into platform-agnostic systems creates
     maintenance burden and prevents cross-platform testing.
   - **Verification:** Cross-platform CI: run the full test suite on Windows, macOS, and Linux;
     verify identical pass/fail results for all platform-agnostic tests.

---

## Inter-System Integration

| ID | Requirement |
|----|-------------|
| R-X.9.1 | Unified wind system |
| R-X.9.2 | Single cloth and ragdoll owner |
| R-X.9.3 | Voice chat single jitter buffer |
| R-X.9.4 | HLSL shader compilation pipeline |
| R-X.9.5 | Visual-only authoring surfaces |
| R-X.9.6 | Cross-domain accessibility hooks |

1. **R-X.9.1** — The engine **SHALL** use a single wind system architecture serving all consumers
   (foliage, cloth, particles, hair, audio). Wind **SHALL** be modeled as ECS entities with
   `WindSource` components defining position, direction, strength, radius, turbulence, and falloff.
   A global wind field texture **SHALL** be generated each frame by sampling all active `WindSource`
   entities into a 3D grid. Foliage shaders, cloth simulation, particle forces, hair dynamics, and
   audio wind effects **SHALL** all sample from this shared wind field texture. No system **SHALL**
   maintain a separate wind model.
   - **Rationale:** Three contradictory wind architectures (per-entity sources, global field
     texture, global singleton) were found in the requirements review. A unified architecture
     prevents inconsistent wind behavior across systems.
   - **Verification:** Integration test: place a wind source; verify foliage, cloth, particles, and
     hair all respond consistently to the same wind at the same position.
2. **R-X.9.2** — The engine **SHALL** provide a single cloth simulation system and a single ragdoll
   system, each with a clearly defined owner domain. Cloth **SHALL** be owned by the physics domain
   (XPBD solver) with the animation domain providing character-specific authoring, LOD, and skinning
   integration. Ragdoll **SHALL** be owned by the physics domain (joint-based ragdoll) with the
   animation domain providing blend weights, partial ragdoll, and animated-to-ragdoll transitions.
   No duplicate simulation system **SHALL** exist. Animation requirements that reference cloth or
   ragdoll **SHALL** delegate to the physics system for simulation and provide only the
   animation-side integration layer.
   - **Rationale:** Dual cloth and ragdoll systems with different algorithms create ambiguity about
     which system is canonical. A single system eliminates redundant work and behavioral
     inconsistencies.
   - **Verification:** Architecture review: verify no two simulation implementations exist for cloth
     or ragdoll in the compiled engine.
3. **R-X.9.3** — Voice chat audio packets **SHALL** pass through exactly one jitter buffer — the
   audio-domain jitter buffer. The network-domain jitter buffer **SHALL NOT** process voice packets;
   it **SHALL** only buffer game state updates. Voice packets **SHALL** be identified by a channel
   type field in the packet header, and the network layer **SHALL** route voice packets directly to
   the audio system's jitter buffer, bypassing the game state buffer entirely.
   - **Rationale:** Double-buffering voice packets introduces unnecessary latency and conflicting
     adaptation algorithms.
   - **Verification:** Integration test: send voice packets over a simulated 100ms jitter link;
     verify packets pass through exactly one buffer (audio) and arrive with latency under 200ms.
4. **R-X.9.4** — The shader graph system **SHALL** compile material and shader graphs to HLSL, which
   DXC (Direct3D Shader Compiler) compiles to DXIL and SPIR-V, and Metal Shader Converter translates
   DXIL to MSL. HLSL is the sole shader intermediate language. WGSL **SHALL NOT** be generated as
   web is not a target platform. All shader compilation paths in the engine **SHALL** use DXC and
   Metal Shader Converter as the shader compilation backend, with no SPIRV-Cross dependencies. DXC
   is accessed via `windows-rs` COM on Windows and C API on Linux. Metal Shader Converter is
   accessed via Swift `@_cdecl` wrappers.
   - **Rationale:** HLSL is the universal shader intermediate representation. DXC provides
     industry-standard compilation to DXIL and SPIR-V, and Metal Shader Converter provides Apple's
     official DXIL-to-MSL translation.
   - **Verification:** Build test: compile all engine shaders; verify HLSL is the sole intermediate
     format, DXC produces DXIL and SPIR-V, and Metal Shader Converter produces MSL.
5. **R-X.9.5** — All user-facing authoring surfaces — gameplay logic, material graphs, animation
   state machines, VFX effect graphs, audio mixing, AI behavior trees, UI layout, quest/dialogue
   trees, formula definitions, and gesture recognition — **SHALL** be authored exclusively through
   visual editors and data-driven assets. No authoring surface **SHALL** require text-based code,
   scripting languages, DSLs, or command-line tools. The formula system **SHALL** use visual formula
   nodes within the logic graph, not a textual DSL. The hot reload system **SHALL** reload logic
   graph assets, not text-based scripts.
   - **Rationale:** The requirements review found formula DSL definitions using textual syntax and
     hot reload referencing text scripts, both violating the no-code constraint. This requirement
     enforces consistency.
   - **Verification:** Audit: enumerate all user-facing authoring surfaces; verify each has a visual
     editor and no text-based alternative is required for any workflow.
6. **R-X.9.6** — Each simulation domain (physics, AI, audio, animation) **SHALL** expose
   accessibility hooks that the UI accessibility system can consume. Audio **SHALL** provide visual
   directional indicators for spatial sound cues when the user enables visual audio mode. Physics
   **SHALL** provide simplified difficulty modes that reduce timing precision requirements for
   motor-impaired players. AI **SHALL** respect per-player difficulty modifiers for perception
   ranges, reaction times, and accuracy. The game framework **SHALL** expose a unified accessibility
   settings panel that configures all cross-domain accessibility hooks from a single location.
   - **Rationale:** Accessibility is a cross-cutting concern. UI-only accessibility misses gameplay
     barriers in physics timing, AI difficulty, and audio-dependent mechanics.
   - **Verification:** Accessibility test suite: enable each accessibility mode; verify audio visual
     indicators, simplified physics, and reduced AI difficulty all activate correctly.

---

## Redundancy Resolution

| ID | Requirement |
|----|-------------|
| R-X.10.1 | Feature ownership for shared capabilities |

1. **R-X.10.1** — The engine **SHALL** resolve feature ownership for capabilities that appear in
   multiple domains according to these rules: (1) rendering-domain features (ocean, clouds, culling,
   hair rendering) own the GPU rendering implementation; geometry/animation domain features own the
   data generation and simulation; (2) the core-runtime spatial indexing features are the sole
   spatial acceleration structure — scene-and-transforms spatial features delegate to them; (3) ECS
   observers, command buffers, and resources are defined in the ECS domain — events-and-messaging
   features extend but do not redefine them; (4) terrain and vegetation painting are defined once in
   world-building — level-editor features reference them.
   - **Rationale:** 15 redundancies were identified across domain boundaries. Explicit ownership
     prevents duplicate implementations and conflicting behavior.
   - **Verification:** Architecture review: for each redundant pair identified in the review, verify
     exactly one implementation exists and the other delegates to it.

---

## Animation Non-Functional Requirements

| ID | Requirement |
|----|-------------|
| R-X.11.1 | Animation evaluation budget |
| R-X.11.2 | Cloth/hair simulation stability |

1. **R-X.11.1** — The engine **SHALL** complete all skeletal animation evaluation (keyframe
   sampling, blending, IK solving, state machine ticking) within 1.5ms of CPU time and 1.0ms of GPU
   compute time per frame at 60fps, for up to 500 visible animated entities. When the budget is
   exceeded, the animation LOD system **SHALL** reduce update rates and bone counts for distant
   entities before dropping any visible animation.
   - **Rationale:** Animation evaluation competes with physics and AI for CPU time. Without a hard
     budget, crowd scenes overwhelm the frame budget.
   - **Verification:** Benchmark: spawn 500 animated characters with full skeletal evaluation, IK,
     and state machines; verify combined CPU+GPU animation time stays within budget at 60fps.
2. **R-X.11.2** — The engine **SHALL** guarantee that cloth and hair simulation never produces NaN,
   infinite, or exploding vertex positions under any input condition (extreme wind, rapid skeletal
   movement, teleportation). When a simulation instability is detected, the system **SHALL** reset
   the affected simulation to its rest pose within one frame and log a warning.
   - **Rationale:** Simulation instability produces visual artifacts (stretched cloth, exploding
     hair) that break immersion and cannot be hidden by LOD.
   - **Verification:** Stress test: teleport a character 1000 units, apply 100 m/s wind, and play a
     180-degree spine rotation simultaneously; verify no NaN or infinite vertex positions.

---

## UI and 2D Non-Functional Requirements

| ID | Requirement |
|----|-------------|
| R-X.12.1 | UI rendering budget |
| R-X.12.2 | Cross-platform 2D physics determinism |

1. **R-X.12.1** — The engine **SHALL** render the full UI (widget tree, text, overlays, nameplates,
   minimaps) within 2ms of GPU time per frame at 60fps. The UI batching system **SHALL** produce
   fewer than 50 draw calls for a typical HUD with action bars, raid frames, minimap, chat, and buff
   icons visible simultaneously.
   - **Rationale:** UI rendering must not compete with 3D scene rendering for GPU time. Draw call
     overhead from un-batched widgets causes frame drops on complex HUDs.
   - **Verification:** Benchmark: render a full HUD with 40 raid bars, 8 action bars, minimap, chat
     window, and 100 nameplates; verify GPU time under 2ms and draw calls under 50.
2. **R-X.12.2** — The 2D physics simulation **SHALL** produce bit-identical results across all
   supported platforms given identical initial state and input sequence, using the same
   deterministic guarantees as the 3D physics system (R-X.5.1). This is required for rollback
   netcode in competitive 2D fighting and action games.
   - **Rationale:** Rollback netcode requires re-simulating frames from confirmed input.
     Non-deterministic 2D physics causes rollback desyncs and visual jitter.
   - **Verification:** Cross-platform test: run a 2D physics simulation with 100 bodies for 1000
     fixed timesteps on all platforms; compare final state for bit-exact equality.

---

## VFX Non-Functional Requirements

| ID | Requirement |
|----|-------------|
| R-X.13.1 | VFX GPU compute budget |
| R-X.13.2 | Decal count and memory budget |

1. **R-X.13.1** — The engine **SHALL** allocate a configurable GPU compute budget for VFX (default:
   2ms at 60fps). The VFX budget manager **SHALL** track per-emitter compute cost and enforce the
   budget by scaling down lower-priority effects before hero effects are affected. Total alive
   particle count **SHALL** be capped at a configurable limit (default: 500,000).
   - **Rationale:** Unbounded particle simulation during spell-heavy combat causes GPU frame spikes.
     A hard budget with priority-based shedding keeps VFX within frame time.
   - **Verification:** Stress test: spawn 600,000 particles across 200 emitters; verify the budget
     manager caps alive particles at 500,000 and GPU compute stays within 2ms.
2. **R-X.13.2** — The engine **SHALL** cap the total number of active decals at a configurable limit
   (default: 2,048) and the decal atlas memory at a configurable budget (default: 128 MB). When
   limits are exceeded, the decal lifecycle system **SHALL** reclaim the oldest low-priority decals
   first.
   - **Rationale:** Large-scale battles accumulate thousands of blood, scorch, and footprint decals.
     Without a budget, decal memory grows unbounded.
   - **Verification:** Stress test: spawn 3,000 decals in a battle scenario; verify active count
     stays at 2,048 and atlas memory stays within 128 MB.

---

## Content Pipeline Non-Functional Requirements

| ID | Requirement |
|----|-------------|
| R-X.14.1 | Asset import throughput |
| R-X.14.2 | Hot reload latency targets |

1. **R-X.14.1** — The engine **SHALL** import and process assets at a sustained rate of at least 100
   assets per minute during batch import on a machine with 16 CPU cores. Incremental builds
   **SHALL** process only changed assets, achieving at least 10x faster build times than full
   rebuilds for changes affecting less than 1% of the asset library.
   - **Rationale:** Large-scale asset libraries contain hundreds of thousands of assets. Slow builds
     block artist iteration and CI pipelines.
   - **Verification:** Benchmark: batch-import 1,000 assets and verify throughput exceeds 100/min.
     Modify 10 assets in a 10,000-asset library and verify incremental build completes in under 10%
     of full build time.
2. **R-X.14.2** — The engine **SHALL** complete asset hot reload — from file change detection
   through in-memory swap — within 2 seconds for textures, 3 seconds for meshes, and 500ms for logic
   graphs. Shader hot reload **SHALL** complete recompilation and pipeline swap within 5 seconds.
   - **Rationale:** Hot reload is the primary iteration mechanism for artists and designers. Latency
     above these thresholds breaks the interactive feedback loop.
   - **Verification:** Integration test: modify a texture, mesh, shader, and logic graph file;
     measure time from file write to in-engine update and verify within the specified limits.

---

## Platform Non-Functional Requirements

| ID | Requirement |
|----|-------------|
| R-X.15.1 | Async I/O bandwidth utilization |
| R-X.15.2 | Crash dump reliability |

1. **R-X.15.1** — The engine **SHALL** achieve at least 80% of raw sequential disk bandwidth through
   the platform-native async I/O layer on all supported platforms. GPU direct storage **SHALL**
   achieve at least 3 GB/s throughput on NVMe hardware where supported.
   - **Rationale:** Streaming open-world content requires saturating storage bandwidth. Inefficient
     I/O causes texture pop-in and mesh LOD delays.
   - **Verification:** Benchmark per platform: sequential read 1 GB via async I/O and verify
     throughput exceeds 80% of raw disk bandwidth. Verify GPU direct storage exceeds 3 GB/s on NVMe.
2. **R-X.15.2** — The engine **SHALL** successfully generate a crash dump for at least 99% of
   crashes, including crashes caused by stack overflow, heap corruption, and GPU device loss. Crash
   reports **SHALL** be uploaded to the aggregation service within 60 seconds of next application
   launch.
   - **Rationale:** Missing crash reports leave developers blind to player-facing stability issues.
     Out-of-process capture maximizes reliability even when the faulting process is corrupted.
   - **Verification:** Inject 100 crashes (null deref, stack overflow, heap corruption, GPU hang);
     verify at least 99 produce valid dump files. Verify upload completes within 60 seconds of the
     subsequent launch.
