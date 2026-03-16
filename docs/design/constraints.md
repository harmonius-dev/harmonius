# Project-Wide Design Constraints

These constraints apply to all designs and implementations
across every domain in the Harmonius engine.

## Language and Toolchain

| Constraint | Detail |
|------------|--------|
| Primary language | Rust (stable only — no nightly features) |
| C++ FFI | cxx.rs for C++ library interop (DXC, Metal Shader Converter) |
| Swift FFI | Swift wrappers accessed through cxx.rs for Apple APIs (NSWindow, GCD) |
| C FFI | bindgen for C header interop (xcb, Wayland, Linux syscalls) |
| No C++ or Obj-C authored | We do not write C++ or Objective-C source. C++ wrappers are thin FFI bridges only. |

## Shader Pipeline

| Constraint | Detail |
|------------|--------|
| Shader IL | HLSL is the sole shader intermediate language |
| HLSL → DXIL/SPIR-V | DXC (C++ via cxx.rs) compiles HLSL |
| DXIL → MSL | Metal Shader Converter (C++ via cxx.rs) |

## Async and Concurrency

- **async/await everywhere.** All asynchronous abstractions
  (I/O, GPU sync, long waits, frame-boundary yields) use
  Rust's `async`/`await`. No callbacks.
- **No Rust stdlib file I/O.** All file operations use
  platform-native async I/O: IOCP on Windows, Dispatch IO
  (GCD) on macOS, io_uring on Linux.
- **Controlled reactor poll point.** The game loop owns an
  `IoReactor`. I/O completions are processed only when
  explicitly polled — the OS never fires callbacks
  asynchronously.
- **Synchronous blocking only for sub-microsecond critical
  sections.** Even 1 ms of blocking has significant
  performance impact. Use `AsyncMutex`/`AsyncRwLock` for
  any contended lock.
- **Scoped execution.** Thread pool supports scoped tasks
  that borrow from the calling scope without `'static` or
  `Arc` overhead.
- **Event handlers support both sync and async variants.**
  Async handlers are dispatched as tasks on the thread pool.

## Platform I/O Backends

| Platform | I/O Backend | Notes |
|----------|-------------|-------|
| Windows | IOCP | `CreateIoCompletionPort`, `GetQueuedCompletionStatusEx` via `windows-sys` |
| macOS | GCD / Dispatch IO | Accessed through C++ wrappers via cxx.rs. We control when dispatch callbacks fire (controlled drain at poll point). |
| Linux | io_uring | Minimum kernel 5.1+. `IORING_OP_POLL_ADD` for fd readiness. No epoll. |

## macOS and Apple Platforms

- **GCD is the concurrency primitive on macOS.** Fibers,
  async I/O, and cooperative scheduling all use GCD
  dispatch queues and blocks.
- **Metal uses Dispatch.** Command buffer completion
  handlers are dispatch blocks. GCD integration is a hard
  requirement for GPU synchronization.
- **All GCD/Dispatch IO accessed through C++ wrappers via
  cxx.rs.**

## Architecture

- **Static dispatch preferred.** No virtual methods,
  vtables, abstract base classes, or dynamic dispatch
  unless absolutely necessary. Use enum dispatch,
  generics, or function pointers instead. `dyn` is
  greatly discouraged and requires explicit
  justification. Acceptable uses:
  - `dyn Reflect` — inherent to the reflection API
  - `dyn FnOnce` / `dyn Fn` — command buffer closures
  - `dyn Plugin` — cold initialization path only
  - `dyn` in editor/tool paths (not game runtime)
- **100% ECS-based.** All simulation data lives as
  components, all logic as systems. No separate physics
  world or other parallel data stores.
  - **Exception: audio runtime.** The audio mixer,
    voice pool, and DSP chain run on a dedicated
    real-time thread with < 0.5 ms latency budget.
    ECS components (`AudioSource`, `AudioListener`)
    bridge game state to the audio runtime via a
    lock-free SPSC command queue. The audio thread
    owns its own mix buffers and effect chains.
- **Shared spatial index.** A single BVH/octree is shared
  across physics, rendering, networking, AI, audio, and
  gameplay.
- **Dependency injection.** Components receive dependencies
  through constructors or method parameters. Never create
  or locate them internally.
- **No singletons.** All dependencies are explicitly passed
  via constructor injection or function parameters.
- **Composition over inheritance.** Separate concerns
  between components and modules.
- **Domain-driven design.** Model the problem space with
  ubiquitous language. Define clear boundaries between
  domains.

## Windowing

- **Custom windowing system.** No winit. Platform-native
  windowing implemented directly:
  - Windows: Win32 `CreateWindowEx` via `windows-sys`
  - macOS: `NSWindow` via Swift wrappers through cxx.rs
  - Linux: xcb (X11) and Wayland via C FFI / bindgen

## Reflection and Serialization

- **bevy_reflect-style reflection.** The `Reflect` trait,
  `TypeRegistry`, and property access patterns are modeled
  after bevy_reflect.
- **Mixed textual+binary serialization.** Textual formats
  (RON, TOML) may reference binary content stored in
  separate companion `.bin` files.

## User-Facing Authoring

- **No-code engine.** All user-facing authoring surfaces
  are visual (logic graphs, material editors, animation
  editors). Users never write code.

## Infrastructure

- **Self-hosted AWS.** All server infrastructure uses
  self-hosted AWS with open-source CDK stacks.
- **Multiplatform.** Code works consistently across
  Windows, macOS, Linux, consoles, iOS, and Android.
  Platform-specific functionality gated via `cfg`
  attributes.

## Dependencies

- Seek approval before adding or changing dependencies.
- Prefer low-level, well-maintained libraries accessible
  through the Rust ecosystem (crates.io) or vcpkg.
- No frameworks — only libraries.
- Always use latest versions.
