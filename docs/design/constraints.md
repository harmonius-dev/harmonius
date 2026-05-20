# Project-Wide Design Constraints

These constraints apply to all designs and implementations across every domain in the Harmonius
engine.

## Language and Toolchain

| Constraint | Detail |
|------------|--------|
| Primary language | Rust (stable only — no nightly features) |
| Windows APIs | `windows-rs` for all Windows APIs (Win32, COM) |
| Apple APIs | `objc2` crate for Objective-C runtime bindings |
| Linux APIs | Rust crates (`ash`, `x11rb`, `wayland-client`) |
| App packaging | XcodeGen + xcodebuild for macOS/iOS app packaging |
| No C/C++ source | No `.c`, `.cpp`, `.h` files anywhere in the project |
| No Obj-C/Obj-C++ | No Objective-C or Objective-C++ source anywhere |
| No Swift | No Swift source anywhere in the project |
| GPU API | Vulkan via `ash` on all platforms |

### Backend Language Matrix

| Backend | Language | FFI Surface |
|---------|----------|-------------|
| Vulkan | Rust | `ash` (thin Vulkan bindings) |
| naga | Rust | bundled crate (build, asset cook, hot-reload) |
| Win32 | Rust | `windows-rs` |
| NSWindow / AppKit | Rust | `objc2-app-kit` bindings |
| X11 | Rust | `x11rb` |
| Wayland | Rust | `wayland-client` |

## Shader Pipeline

| Constraint | Detail |
|------------|--------|
| Shader IL | GLSL is the sole shader intermediate language |
| GLSL → SPIR-V | `naga` (`glsl-in`, `spv-out`) in-process |

`naga` is the bundled GLSL→SPIR-V compiler. Shader compilation is offline (asset processing, build
scripts) or in-process on the main thread for hot-reload. No external shader compiler executables.

## Threading Model

Three thread roles communicate via crossbeam-channel. No shared mutable state.

| Thread | Owns | Handles |
|--------|------|---------|
| Main | OS event loop, platform I/O | Window, input, timers, all I/O |
| Workers (N) | Job system, ECS, game loop | Simulation, all compute |
| Render | GPU | Draw calls, GPU uploads |

- **Main thread is sole OS interface.** All platform APIs (windowing, input, timers, clipboard, file
  dialogs, notifications) run on the main thread. Workers touch zero OS APIs.
- **Workers include the game loop.** The game loop runs on a worker thread. It participates in
  work-stealing via `scope()`. No dedicated game loop thread.
- **Render thread is pure GPU.** The render thread only submits GPU commands and uploads resources.
  It never performs file I/O.
- **Channels between threads.** All inter-thread communication uses crossbeam-channel. No shared
  mutable state, no mutexes, no `Arc` for cross-thread sharing.

## I/O Model

- **Platform-native I/O.** Each platform uses its optimal I/O framework:
  - Linux: io_uring via rustix
  - Windows: IOCP via windows-rs, Vulkan staging buffers for GPU assets
  - Apple: GCD dispatch_io via dispatch2, Vulkan staging buffers for GPU assets,
    Networking.framework
- **Main thread polls I/O.** The main thread owns the OS event loop and polls I/O completions as
  part of that loop. No separate I/O thread.
- **Zero blocking operations.** All I/O is non-blocking via platform async APIs.

## User-Facing API Principle

- **All user-facing APIs are synchronous.** Users write purely synchronous ECS system code. Async is
  an invisible implementation detail inside the engine.
- **Request/handle pattern for I/O.** User calls a sync method (e.g., `assets.load("sword.glb")`)
  which returns a handle immediately. The I/O completes asynchronously on the main thread. Systems
  react when the handle becomes ready.
- **Cached OS state.** Main-thread-only OS data (clipboard, locale, power state, display info) is
  cached as ECS resources (`Res<ClipboardState>`, `Res<PowerState>`). The main thread updates these
  automatically. Users read them as plain synchronous data.
- **Fire-and-forget writes.** Save game, log writes, and network sends are queued to the main thread
  and executed via platform I/O. The user never waits for completion.

## CPU Parallelism

- **Custom job system for all parallelism.** Built on crossbeam-deque (Chase-Lev work-stealing),
  crossbeam-channel, and crossbeam-utils. Provides both task parallelism (job graphs) and data
  parallelism (par_iter, join, scope).
- **Game loop runs on a worker thread.** No dedicated game loop thread. The game loop driver
  participates in work-stealing via scope().
- **QoS-based core scheduling.** Apple QoS classes, Windows Thread Director, Linux nice + EAS. No
  explicit core pinning.
- **No async/await in engine, editor, or game runtime.** No `async fn`, no `.await`, no `Future`, no
  async runtimes in the game engine, editor, or headless game server. All engine code is
  synchronous. I/O is submitted to the main thread via channels and completions arrive as jobs. No
  `AsyncMutex`, `AsyncRwLock`, `AsyncBarrier`. Channels eliminate shared mutable state.
- **Async/await permitted in backend services.** Backend services (GameDb, matchmaking, control
  panel, K8s operator) may use Tokio and async/await. These are standard server workloads that
  benefit from async I/O. The game server communicates with backend services via QUIC — the game
  side is synchronous, the backend side is async.

## macOS and Apple Platforms

- **GCD for Apple I/O.** GCD dispatch_io for file I/O, Networking.framework for networking, Vulkan
  staging I/O for GPU assets. Accessed from Rust via dispatch2 and objc2.
- **Vulkan GPU sync via `VkSemaphore`.** GPU completion is detected by polling
  `VkSemaphore.signaledValue`. No GCD dispatch blocks needed for GPU synchronization.
- **Target Vulkan.** Use Vulkan API when available for explicit memory management, placement
  sparse resources, and improved shader compilation.
- **Vulkan via `ash`.** Vulkan is accessed from Rust via the `ash` crate. No Swift, no manual C ABI.
- **iOS: UIKit owns the OS main thread.** `UIApplicationMain` runs the `CFRunLoop` on thread 0. The
  game loop runs on a dedicated thread. UIKit input events (touch, accelerometer, keyboard) are
  forwarded to the game loop thread via channel. The render thread presents independently via
  Vulkan.

## Architecture

- **Static dispatch preferred.** No virtual methods, vtables, abstract base classes, or dynamic
  dispatch unless absolutely necessary. Use enum dispatch, generics, or function pointers instead.
  `dyn` is greatly discouraged and requires explicit justification. Acceptable uses:
  - `dyn FnOnce` / `dyn Fn` — command buffer closures
  - `dyn Plugin` — cold initialization path only
  - `dyn` in editor/tool paths (not game runtime)
- **ECS-primary (~90%).** All simulation data lives as components, all gameplay logic as systems.
  Exceptions:
  - **Audio runtime.** Dedicated real-time thread with < 0.5 ms latency budget. ECS components
    (`AudioSource`, `AudioListener`) bridge game state via lock-free SPSC command queue.
  - **GPU resource management.** Descriptor pools, command allocators, and swap chains are managed
    internally by the rendering backend.
  - **Windowing and platform event loops.** OS event loops are not ECS systems but forward events
    into ECS.
  - **Shader compilation pipeline.** In-process `naga` compilation is not an ECS system.
  - **Asset processing internals.** Import and processing pipelines manage internal state but expose
    results as ECS components.
- **Spatial indices.** Three spatial structures serve different subsystems:
  - **Shared BVH** for AI, audio, and gameplay queries (e.g., "what's near me?")
  - **Physics-private BVH** owned by the physics engine. Not shared — physics needs its own topology
    for broadphase collision and raycasts.
  - **GPU visibility** handled by GPU-driven culling (compute shader + HZB). The shared BVH is not
    used for rendering visibility.
  - **Grid** for networking relevancy (interest management, area-of-interest).
- **Collision layers.** `u32` bitmask on physics shapes. Determines which shapes can collide with
  each other. Systems filter by layer for selective collision queries.
- **Render layers.** `u32` bitmask on renderable entities. Determines which camera sees which
  objects. Used for split-screen, minimap, editor overlays, and selective rendering.
- **Dependency injection.** Components receive dependencies through constructors or method
  parameters. Never create or locate them internally.
- **No singletons.** All dependencies are explicitly passed via constructor injection or function
  parameters.
- **Composition over inheritance.** Separate concerns between components and modules.
- **Domain-driven design.** Model the problem space with ubiquitous language. Define clear
  boundaries between domains.

## 2D and 2.5D Support

- **First-class 2.5D.** The engine supports side-scrollers, isometric, and top-down games alongside
  full 3D. Every subsystem must work in 2D, 2.5D, and 3D modes.
- **2D transforms.** `Transform2D` component with `Vec2` position, `f32` rotation, `Vec2` scale.
  Used alongside or instead of 3D `Transform` depending on game type.
- **2D physics.** 2D rigid bodies, 2D collision shapes (circle, rectangle, capsule, polygon), 2D
  physics BVH. Same solver, 2D specialization.
- **2D rendering.** Sprite batching, tilemap rendering, parallax layers, 2D lighting (point, spot,
  global), pixel-perfect rendering mode. Runs through the same render graph as 3D.
- **2D spatial index.** 2D BVH for gameplay/AI queries. Grid already supports 2D for networking.
- **Tilemap geometry.** Chunked tilemaps with auto-tiling, animated tiles, collision generation from
  tile properties.
- **Shared subsystems.** Audio, input, ECS, asset pipeline, events, serialization, job system, and
  platform I/O work identically in 2D and 3D — no separate code paths.

## Windowing

- **Custom windowing system.** No winit. Platform-native windowing implemented directly:
  - Windows: Win32 `CreateWindowEx` via `windows-rs`
  - macOS: `NSWindow` via `objc2-app-kit`
  - Linux: X11 via `x11rb`, Wayland via `wayland-client`

## Serialization

- **Zero reflection.** No runtime type registry, no `dyn Reflect`, no TypeId-based dispatch. All
  type metadata is generated statically by the codegen pipeline.
- **Binary serialization via rkyv only.** Baked assets and save files use rkyv for zero-copy mmap
  access without deserialization. No serde. No other binary serialization libraries.
- **Custom scene text format.** A custom text format designed for line-based diffing and merge
  conflict resolution. Not BSN, not RON — a Harmonius-specific format. Deterministic output, stable
  entity IDs, flat structure.
- **Mixed textual+binary serialization.** Text scene files may reference binary content stored in
  separate companion `.bin` files.

## Codegen and Hot-Reload Architecture

- **Middleman .dylib.** All codegen'd types (components, enums, systems, events, type descriptors,
  property panels, blueprint functions, serialization derives) live in a single middleman .dylib.
- **Engine binary is a thin shell.** The engine binary contains the job system, platform I/O, GPU
  abstraction, and render graph. It loads the middleman .dylib via `libloading`. It never contains
  user-defined types.
- **Plugin .dylibs depend on the middleman.** Plugins link against the middleman for shared types.
  The middleman is the single source of truth for all types that cross the engine/plugin boundary.
- **Hot-reload recompiles the middleman.** When users change components, materials, or blueprints,
  only the middleman .dylib is recompiled and hot-reloaded. The engine binary stays stable.
- **Bundled Rust toolchain.** The engine installer includes `rustc` and `cargo`. Users never see
  them. Incremental compilation targets sub-3-second reload.
- **Codegen is the preferred method for all dynamic content in the editor.** When users create or
  modify components, materials, shading models, blueprints, or any other type-level content, the
  engine generates Rust code, compiles it, and hot-reloads it. No runtime interpretation, no
  reflection, no dynamic dispatch for user-defined types. If something can be codegen'd, it must be.
- **Visual graph nodes codegen Rust source.** All visual graph nodes (logic graphs, material graphs,
  blueprints) generate actual Rust source code. The bundled `rustc` compiles that source into the
  middleman `.dylib`. No bytecode VM, no interpreter.
- **`include_bytes!` scope.** `include_bytes!` is only for inline data directly embedded in tables
  and logic graphs. Large assets (meshes, textures, audio) remain on disk and are loaded via the
  asset pipeline.
- **Static linking for shipping.** Middleman + all plugins are statically linked into a single
  binary with LTO. No .dylibs shipped. Dead code elimination removes unused plugin code.

## User-Facing Authoring

- **No-code engine.** All user-facing authoring surfaces are visual (logic graphs, material editors,
  animation editors). Users never write code.

## Localization

- **Localization is a core-runtime service, not a UI feature.** All user-visible strings use
  `LocalizedStringId`. The localization table is resolved at runtime by the core-runtime service. No
  hardcoded strings anywhere in the engine, editor, or game runtime.

## Networking

- **QUIC unified transport.** All network communication uses QUIC. No TCP, no custom UDP, no
  separate DTLS or HTTPS connections. Platform implementations:
  - Linux: `quinn-proto` (pure Rust QUIC)
  - Apple: `Networking.framework` (system QUIC via objc2)
  - Windows: MsQuic via `windows-rs`
- **QUIC replaces everything.** Game state replication, asset streaming, HTTP API calls, WebSocket
  equivalents, and matchmaking all run over QUIC streams. One protocol, one connection.

## Infrastructure

- **Kubernetes-native.** All server infrastructure runs on Kubernetes with Helm charts. Custom Rust
  K8s operator (kube-rs) handles game-aware deployment, canary validation, and graceful player
  drain.
- **TiKV sole database.** TiKV is the only database. Replaces PostgreSQL, Redis, Valkey, and
  DynamoDB. Provides distributed KV with transactions, range queries, and multi-region replication.
- **Full OSS stack.** All engine services are fully open source. Console SDK builds run on shared
  build servers (server-side only). The marketplace, analytics, audit, and AI routing are all OSS
  components deployable by anyone. No proprietary cloud services:
  - TiKV — database
  - Garage — S3-compatible object storage (replaces MinIO)
  - Pingora — CDN / reverse proxy (replaces CloudFlare)
  - Vector — log/metric collection
  - Prometheus + Grafana — monitoring and dashboards
  - Loki — log aggregation
- **Customer-owned AI API keys.** Cloud AI uses the customer's own API keys. The engine is a thin
  client. No Harmonius proxy for billing. Supported providers (OpenAI, Anthropic, etc.) are
  configured per-project.
- **Game-aware GitOps.** Custom Rust K8s operator replaces ArgoCD. Understands game server state:
  graceful player drain before pod termination, canary validation with production metrics, automatic
  rollback on metric regression.
- **Multiplatform.** Code works consistently across Windows, macOS, Linux, consoles, iOS, and
  Android. Platform-specific functionality gated via `cfg` attributes.

## Performance Patterns

- **No HashMap on deterministic hot paths.** Hash maps are non-deterministic (iteration order varies
  across runs). Use sorted `Vec`, `BTreeMap`, or index-based lookup on hot paths. HashMap is fine
  for cold paths (asset loading, editor).
- **Bulk simulation data in GPU buffers.** Particles, cloth vertices, fluid cells, and other bulk
  simulation data live in GPU buffers, not as individual ECS entities. ECS holds the emitter/system
  entity; the GPU holds the per-element data.
- **Per-thread arenas for hot-path allocations.** Allocate from thread-local arenas during frame
  processing. Reset arenas at frame boundaries. Avoids global allocator contention.
- **SmallVec for small inline allocations.** Use `SmallVec` for collections that are typically small
  (< 8 elements) to avoid heap allocation in the common case.

## Documentation Standards

- **Algorithm references required.** Every non-trivial algorithm in a design document must cite the
  source with a direct URL (paper, blog post, GDC talk, reference implementation).

## Dependencies

- Seek approval before adding or changing dependencies.
- Prefer low-level, well-maintained libraries accessible through the Rust ecosystem (crates.io) or
  vcpkg.
- No frameworks — only libraries.
- Always use latest versions.

## Core Dependencies

| Dependency | Purpose |
|------------|---------|
| `crossbeam-deque` | Work-stealing deques for job system |
| `crossbeam-channel` | Lock-free channels between threads |
| `crossbeam-utils` | CachePadded, scoped threads, Backoff |
| `rustix` | Linux syscalls (io_uring) |
| `windows-rs` | Win32 windowing, input, IOCP, Vulkan staging buffers |
| `objc2` + `dispatch2` | Apple APIs (GCD, Networking.framework) |
| `ash` | Thin Vulkan bindings |
| `x11rb` | X11 windowing (Linux) |
| `wayland-client` | Wayland windowing (Linux) |
| `glam` | SIMD math types |
| `smallvec` | Inline-allocated small vectors |

Removed: `tokio`, `mio`, `rayon`, `compio`. Tokio ecosystem crates (`reqwest`, `sqlx`, `quinn`,
`tokio-tungstenite`, `hyper`, `axum`) are not permitted.

## Unsafe Policy

- Do not use `unsafe` unless absolutely necessary.
- When a performant, safe alternative exists, use it instead.
- Unsafe is allowed where necessary: graphics APIs, FFI, ECS internals.
- Use `bytemuck`/`zerocopy` for zero-copy safety.
