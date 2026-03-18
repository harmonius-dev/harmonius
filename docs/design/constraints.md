# Project-Wide Design Constraints

These constraints apply to all designs and implementations across every domain in the Harmonius
engine.

## Language and Toolchain

| Constraint | Detail |
|------------|--------|
| Primary language | Rust (stable only — no nightly features) |
| C++ FFI | cxx.rs for C++ library interop (DXC, Metal Shader Converter) |
| Swift FFI | Swift with C++ interop exposes Apple APIs via cxx.rs to Rust |
| Swift build | CMake builds Swift libraries; XcodeGen + xcodebuild packages macOS/iOS apps |
| C FFI | bindgen for C header interop (xcb, Wayland, Linux syscalls) |
| No Obj-C/Obj-C++ | No Objective-C or Objective-C++ source. Use Swift with C++ interop instead. |
| No objc2-metal | Use metal-cpp (Apple's C++ Metal wrapper) for Metal FFI, not objc2-metal. |

## Shader Pipeline

| Constraint | Detail |
|------------|--------|
| Shader IL | HLSL is the sole shader intermediate language |
| HLSL → DXIL/SPIR-V | DXC (C++ via cxx.rs) compiles HLSL |
| DXIL → MSL | Metal Shader Converter (C++ via cxx.rs) |

## Async and Concurrency

- **async/await everywhere.** All asynchronous abstractions (I/O, GPU sync, long waits,
  frame-boundary yields) use Rust's `async`/`await`. No callbacks.
- **No Rust stdlib file I/O.** All file operations use platform-native async I/O: IOCP on Windows,
  Dispatch IO (GCD) on macOS, io_uring on Linux.
- **Controlled reactor poll point.** The game loop owns an `IoReactor`. I/O completions are
  processed only when explicitly polled — the OS never fires callbacks asynchronously.
- **Synchronous blocking only for sub-microsecond critical sections.** Even 1 ms of blocking has
  significant performance impact. Use `AsyncMutex`/`AsyncRwLock` for any contended lock.
- **Scoped execution.** Thread pool supports scoped tasks that borrow from the calling scope without
  `'static` or `Arc` overhead.
- **Scoped async limitation.** Scoped async tasks (`Scope::spawn_async`) that borrow from the
  calling scope must be CPU-only (no I/O in flight). Tasks requiring I/O must use `'static` futures.
  This avoids unsafe lifetime erasure and aligns with Rust's future model.
- **Event handlers support both sync and async variants.** Async handlers are dispatched as tasks on
  the thread pool.

## Platform I/O Backends

| Platform | I/O Backend       |
|----------|-------------------|
| Windows  | IOCP              |
| macOS    | GCD / Dispatch IO |
| Linux    | io_uring          |

1. **Windows** — `CreateIoCompletionPort`, `GetQueuedCompletionStatusEx` via `windows-sys`
2. **macOS** — Accessed through C++ wrappers via cxx.rs. We control when dispatch callbacks fire
   (controlled drain at poll point).
3. **Linux** — Minimum kernel 5.1+. `IORING_OP_POLL_ADD` for fd readiness. No epoll.

## macOS and Apple Platforms

- **GCD is the concurrency primitive on macOS.** Fibers, async I/O, and cooperative scheduling all
  use GCD dispatch queues and blocks.
- **Metal uses Dispatch.** Command buffer completion handlers are dispatch blocks. GCD integration
  is a hard requirement for GPU synchronization.
- **All GCD/Dispatch IO accessed through Swift wrappers via cxx.rs.** Swift uses C++ interop to
  implement the cxx.rs bridge interface, exposing GCD to Rust.
- **Metal via metal-cpp.** Metal is accessed through metal-cpp (Apple's C++ wrapper) exposed to Rust
  via cxx.rs. No objc2-metal.

## Architecture

- **Static dispatch preferred.** No virtual methods, vtables, abstract base classes, or dynamic
  dispatch unless absolutely necessary. Use enum dispatch, generics, or function pointers instead.
  `dyn` is greatly discouraged and requires explicit justification. Acceptable uses:
  - `dyn Reflect` — inherent to the reflection API
  - `dyn FnOnce` / `dyn Fn` — command buffer closures
  - `dyn Plugin` — cold initialization path only
  - `dyn` in editor/tool paths (not game runtime)
- **100% ECS-based.** All simulation data lives as components, all logic as systems. No separate
  physics world or other parallel data stores.
  - **Exception: audio runtime.** The audio mixer, voice pool, and DSP chain run on a dedicated
    real-time thread with < 0.5 ms latency budget. ECS components (`AudioSource`, `AudioListener`)
    bridge game state to the audio runtime via a lock-free SPSC command queue. The audio thread owns
    its own mix buffers and effect chains.
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
  - Windows: Win32 `CreateWindowEx` via `windows-sys`
  - macOS: `NSWindow` via Swift wrappers through cxx.rs
  - Linux: xcb (X11) and Wayland via C FFI / bindgen

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

All processes -- engine, editor, tool servers, and cloud services -- use the custom `IoReactor`
built on platform-native I/O primitives (IOCP on Windows, GCD/Dispatch IO on macOS, io_uring on
Linux). There is no third-party async runtime anywhere in the project.

- **No third-party runtimes.** No external async runtimes in any binary. No crates that depend on or
  assume a third-party async runtime.
- **HTTP:** Platform-native HTTP clients (NSURLSession on macOS, WinHTTP on Windows, libcurl on
  Linux) wrapped via the `IoReactor`.
- **WebSocket:** The `tungstenite` crate (non-async, no runtime dependency) with manual I/O
  integration via `IoReactor`.
- **Database:** Platform-native async socket I/O via `IoReactor` with a custom PostgreSQL wire
  protocol client, or synchronous database clients on dedicated I/O threads.
- **QUIC:** The `quinn` crate with its `AsyncUdpSocket` trait implemented on the `IoReactor` (quinn
  supports custom runtimes).
- **REST APIs:** Custom HTTP server built on the `IoReactor` using raw TCP accept/read/write with
  HTTP/1.1 parsing, or a minimal HTTP library without runtime dependencies.
