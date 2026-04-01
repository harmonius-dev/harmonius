# Project-Wide Design Constraints

These constraints apply to all designs and implementations across every domain in the Harmonius
engine.

## Language and Toolchain

| Constraint | Detail |
|------------|--------|
| Primary language | Rust (stable only — no nightly features) |
| Windows APIs | `windows-rs` for all Windows APIs (Win32, COM, D3D12) |
| Apple APIs | Swift via `swift-bridge`; generates Rust ↔ Swift bindings |
| Linux APIs | Rust crates (`ash`, `x11rb`, `wayland-client`) |
| Swift build | SwiftPM for libraries; XcodeGen + xcodebuild for app packaging |
| Swift interop | `swift-bridge` for all Rust ↔ Swift FFI (macOS/iOS only) |
| No C/C++ source | No `.c`, `.cpp`, `.h` files anywhere in the project |
| No CMake | SwiftPM replaces CMake for all Swift builds |
| No manual C FFI | No hand-written `@_cdecl` / `extern "C"` for Swift interop |
| No Obj-C/Obj-C++ | No Objective-C or Objective-C++ source anywhere |
| No metal-cpp | Metal is accessed directly from Swift |

### Backend Language Matrix

| Backend | Language | FFI Surface |
|---------|----------|-------------|
| Direct3D 12 | Rust | `windows-rs` COM bindings |
| Metal | Swift | `swift-bridge` generated bindings |
| Vulkan | Rust | `ash` (thin Vulkan bindings) |
| DXC | CLI | `dxc` subprocess (offline + hot-reload) |
| Metal Shader Converter | CLI | `metal-shaderconverter` subprocess |
| Win32 | Rust | `windows-rs` |
| NSWindow / AppKit | Swift | `swift-bridge` generated bindings |
| X11 | Rust | `x11rb` |
| Wayland | Rust | `wayland-client` |

## Shader Pipeline

| Constraint | Detail |
|------------|--------|
| Shader IL | HLSL is the sole shader intermediate language |
| HLSL → DXIL/SPIR-V | `dxc` CLI invoked as subprocess |
| DXIL → MSL | `metal-shaderconverter` CLI invoked as subprocess |

No embedded DXC or Metal Shader Converter libraries. All shader compilation is offline (asset
processing time) or via subprocess calls for hot-reload.

## Async and Concurrency

- **async/await everywhere.** All asynchronous abstractions (I/O, GPU sync, long waits,
  frame-boundary yields) use Rust's `async`/`await`. No callbacks.
- **Tokio `current_thread` runtime.** The game loop thread owns a Tokio `current_thread` runtime.
  All futures (file I/O, network, timers) are polled only when the game loop explicitly calls
  `runtime.poll()` (non-blocking drain) or `runtime.block_on()` (blocking wait). No I/O handlers
  fire at arbitrary times.
- **Dedicated game loop thread.** The game loop runs on a dedicated thread that owns the Tokio
  runtime. On desktop (macOS, Windows, Linux) this is typically thread 0. On mobile (iOS, Android)
  the game loop thread is separate from the OS main thread. Platform UI events (UIKit, Activity
  lifecycle) arrive on the OS main thread and are forwarded to the game loop thread via a lock-free
  SPSC queue.
- **Controlled poll point.** I/O completions are delivered only when the game loop polls the Tokio
  runtime — the OS never fires callbacks asynchronously into game logic.
- **File I/O via Tokio.** `tokio::fs` provides async file operations backed by an internal thread
  pool. Results are delivered to the game loop thread at the next poll point.
- **Network I/O via Tokio.** `tokio::net` provides async TCP/UDP. Tokio internally uses epoll
  (Linux), kqueue (macOS), or IOCP (Windows). Engine code does not interact with these directly.
- **Synchronous blocking only for sub-microsecond critical sections.** Even 1 ms of blocking has
  significant performance impact. Use `AsyncMutex`/`AsyncRwLock` for any contended lock.
- **All async tasks use `'static` futures.** Shared data is passed via `Arc`. Scoped synchronous
  tasks (thread pool `scope()`) remain available for data-parallel CPU work.
- **Event handlers support both sync and async variants.** Async handlers are dispatched as tasks on
  the thread pool.

## macOS and Apple Platforms

- **Tokio handles all I/O on Apple platforms.** No GCD or Dispatch IO for file or network I/O. Tokio
  internally uses kqueue for socket readiness and a thread pool for file operations.
- **Metal GPU sync via `MTLSharedEvent`.** GPU completion is detected by polling
  `MTLSharedEvent.signaledValue` at the frame boundary poll point. No GCD dispatch blocks needed for
  GPU synchronization.
- **Metal via Swift.** Metal is accessed directly from Swift, with `swift-bridge` generating the
  Rust ↔ Swift bindings. No metal-cpp, no objc2-metal, no manual C ABI.
- **iOS: UIKit owns the OS main thread.** `UIApplicationMain` runs the `CFRunLoop` on thread 0. The
  game loop runs on a dedicated thread. UIKit input events (touch, accelerometer, keyboard) are
  forwarded to the game loop thread via a lock-free SPSC queue. The render thread presents
  independently via Metal when frames are ready.

## Architecture

- **Static dispatch preferred.** No virtual methods, vtables, abstract base classes, or dynamic
  dispatch unless absolutely necessary. Use enum dispatch, generics, or function pointers instead.
  `dyn` is greatly discouraged and requires explicit justification. Acceptable uses:
  - `dyn Reflect` — inherent to the reflection API
  - `dyn FnOnce` / `dyn Fn` — command buffer closures
  - `dyn Plugin` — cold initialization path only
  - `dyn` in editor/tool paths (not game runtime)
- **ECS-primary (~90%).** All simulation data lives as components, all gameplay logic as systems.
  Exceptions:
  - **Audio runtime.** Dedicated real-time thread with < 0.5 ms latency budget. ECS components
    (`AudioSource`, `AudioListener`) bridge game state via lock-free SPSC command queue.
  - **GPU resource management.** Descriptor heaps, command allocators, and swap chains are managed
    internally by the rendering backend.
  - **Windowing and platform event loops.** OS event loops are not ECS systems but forward events
    into ECS.
  - **Shader compilation pipeline.** CLI subprocess invocations are not ECS systems.
  - **Asset processing internals.** Import and processing pipelines manage internal state but expose
    results as ECS components.
- **Shared spatial index.** A single BVH/octree is shared across physics, rendering, networking, AI,
  audio, and gameplay.
- **Dependency injection.** Components receive dependencies through constructors or method
  parameters. Never create or locate them internally.
- **No singletons.** All dependencies are explicitly passed via constructor injection or function
  parameters.
- **Composition over inheritance.** Separate concerns between components and modules.
- **Domain-driven design.** Model the problem space with ubiquitous language. Define clear
  boundaries between domains.

## Windowing

- **Custom windowing system.** No winit. Platform-native windowing implemented directly:
  - Windows: Win32 `CreateWindowEx` via `windows-rs`
  - macOS: `NSWindow` via Swift (`swift-bridge`)
  - Linux: X11 via `x11rb`, Wayland via `wayland-client`

## Reflection and Serialization

- **bevy_reflect-style reflection.** The `Reflect` trait, `TypeRegistry`, and property access
  patterns are modeled after bevy_reflect.
- **Mixed textual+binary serialization.** Textual formats (RON, TOML) may reference binary content
  stored in separate companion `.bin` files.

## User-Facing Authoring

- **No-code engine.** All user-facing authoring surfaces are visual (logic graphs, material editors,
  animation editors). Users never write code.

## Infrastructure

- **Self-hosted AWS.** All server infrastructure uses self-hosted AWS with open-source CDK stacks.
- **Multiplatform.** Code works consistently across Windows, macOS, Linux, consoles, iOS, and
  Android. Platform-specific functionality gated via `cfg` attributes.

## Dependencies

- Seek approval before adding or changing dependencies.
- Prefer low-level, well-maintained libraries accessible through the Rust ecosystem (crates.io) or
  vcpkg.
- No frameworks — only libraries.
- Always use latest versions.

## Async Runtime Policy

Tokio `current_thread` is the sole async runtime for all processes: engine, editor, tool servers,
and cloud services. The game loop thread owns the runtime and polls it explicitly at frame
boundaries.

- **Full Tokio ecosystem.** Crates that depend on Tokio (reqwest, sqlx, quinn, tokio-tungstenite,
  hyper, axum) are permitted.
- **HTTP:** `reqwest` or `hyper` for HTTP clients; `axum` for HTTP servers.
- **WebSocket:** `tokio-tungstenite` for async WebSocket.
- **Database:** `sqlx` with Tokio for PostgreSQL.
- **QUIC:** `quinn` with the `tokio` feature for QUIC transport.
- **No unsafe minimization.** Do not use `unsafe` methods or functions unless absolutely necessary.
  When a performant, safe alternative exists, use it instead. Unsafe is still allowed where
  necessary (graphics APIs, FFI, ECS internals). Use `bytemuck`/`zerocopy` for zero-copy safety.
