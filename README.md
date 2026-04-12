# Harmonius

A modern cross-platform game engine for real-time 2D, 3D, and XR games. Written in Rust with
platform-native I/O, GPU-driven rendering, and static codegen for zero-reflection performance.

## Key Features

- **No-code visual authoring** — all gameplay logic, formulas, materials, VFX, animations, and UI
  are authored in visual editors. Users never write code.
- **Everything compiles to Rust** — all visual graphs (gameplay logic, formulas, AI behavior, quest
  conditions, dialogue branching) codegen actual Rust source. Bundled rustc compiles it into the
  middleman `.dylib`. Inline data in tables and graphs uses `include_bytes!`. Shipped games
  statically link all code into one binary; assets stay on disk. One language, one compiler, one
  type system.
- **Rust expression semantics** — every visual node maps to a Rust expression. Case analysis
  (`match`), let-binding, Option/Result handling, explicit `as` casts, iterator chains. No implicit
  coercion. This is part of being Harmonius.
- **Custom ECS** — archetype-based with AoSoA tiled storage, compiled query plans, bloom filter
  cache, parallel iteration, and zero reflection
- **Composition over inheritance** — generic primitives (directed graphs, data tables, attributes,
  containers, timelines, spatial indices) compose into any game mechanic. The engine has no concept
  of "quest" or "inventory" — only graphs, tables, and conditions.
- **GPU-driven rendering** — Nanite-style cluster LOD, mesh shaders, compute culling, indirect draw,
  render graph with automatic barrier insertion, per-frame ring-buffered resources
- **2D and 3D in one engine** — sprites, tilemaps, 2D physics, 2D lighting, skeletal 2D animation,
  vector graphics, and full 3D rendering share the same ECS, render graph, and spatial index
- **Platform-native I/O** — io_uring (Linux), IOCP + DirectStorage (Windows), GCD + Metal I/O
  (Apple). Main thread polls completions. Zero async runtimes.
- **Static codegen pipeline** — visual editor schemas compile to Rust structs, typed accessors, ECS
  binding functions, and validation logic in the middleman `.dylib`. Hot-reload in editor, static
  linking for shipping.
- **Zero reflection** — no `dyn Reflect`, no `TypeRegistry`, no `TypeId` dispatch. All type metadata
  generated statically by the codegen pipeline.
- **Full accessibility** — screen reader bridges (NSAccessibility, UI Automation, AT-SPI),
  colorblind filters, WCAG compliance, TTS, subtitles, input remapping, high contrast, reduced
  motion
- **Engine-wide localization** — `LocalizedStringId` replaces all hardcoded strings. Locale
  detection, plural rules, message interpolation, time zones, per-locale asset bundles. Localization
  is a core-runtime service, not a UI feature.
- **HLSL shader pipeline** — all shaders authored in HLSL, compiled via DXC + Metal Shader Converter
  CLI. No GLSL, no WGSL.
- **Custom windowing** — NSWindow, Win32, X11/Wayland directly. No winit.
- **All game genres** — puzzle, 2D platformer, 3D RPG, RTS, FPS, co-op, local multiplayer, VR. One
  engine, one set of primitives.
- **57 design documents** — features, requirements, user stories, and test cases written before
  implementation begins

## Goals

1. **Universal game engine** — support every genre (puzzle, 2D, 3D, RPG, RTS, FPS, co-op, local
   multiplayer, VR) in a single engine
2. **No-code-first** — all user-facing authoring is visual; users never write code
3. **Production-grade Rust** — stable Rust only, zero nightly, modern GPU graphics
4. **Open source** — Apache 2.0 license with open asset store and community marketplace
5. **Self-hosted infrastructure** — all services open source with 1-click AWS deployment; customers
   pay AWS directly, no vendor lock-in
6. **Privacy-respecting AI** — cloud AI backends (Claude, Cursor, Copilot) use customer's own API
   keys; engine is a thin client, never a proxy

## Objectives

1. **Replace legacy engines** — deliver a credible alternative to Unity and Unreal for creators who
   value open-source infrastructure and no-code authoring
2. **Minimize integration friction** — import via intermediate formats (glTF, Alembic, PNG, EXR,
   KTX2); no DCC plugins required
3. **Maximize parallelism** — custom ECS with AoSoA tiled storage, custom job system with
   work-stealing, and data-parallel primitives to saturate all cores every frame
4. **Modern GPU baseline** — require mesh shaders + ray tracing (Metal 4, D3D12, Vulkan 1.4); no
   legacy fallback to maintain
5. **Platform-native I/O** — io_uring, IOCP, GCD with DirectStorage and Metal I/O for disk-to-GPU
   DMA; no third-party async runtime
6. **Zero reflection** — all type metadata generated statically via codegen; no runtime type
   registry, no `dyn Reflect`
7. **Full design coverage** — 57 design documents with features, requirements, and user stories
   before implementation begins

## Key Technical Highlights

- **Custom ECS** — archetype-based with AoSoA tiled storage for SIMD, compiled query plans with
  bloom filters, zero reflection
- **Custom job system** — Chase-Lev work-stealing deques via crossbeam, data-parallel primitives
  (par_iter, join, scope, par_sort), QoS-based core scheduling for hybrid CPUs
- **Static codegen** — visual editor schemas compile to Rust via bundled toolchain; plugins are data
  packages, not DLLs; all dispatch is static
- **Platform-native I/O** — io_uring (Linux), IOCP + DirectStorage (Windows), GCD + Metal I/O
  (Apple); main thread polls I/O completions; zero blocking operations
- **GPU-driven rendering** — Nanite-style cluster LOD, compute shader culling, indirect draw; CPU
  BVH for gameplay queries only
- **Custom windowing** — NSWindow, Win32, X11/Wayland directly; no winit
- **Mesh shaders + ray tracing required** — no legacy pipeline fallback
- **No-code authoring** — all user logic via visual graphs compiled to native Rust; all assets via
  visual editors
- **Spatial indices** — BVH for AI/gameplay, separate BVH for physics, grid for networking

## Supported Platforms

| OS | Graphics | I/O | GPU Assets | Status |
|----|----------|-----|------------|--------|
| macOS | Metal 4 | GCD dispatch_io | Metal I/O | Design |
| Windows | Direct3D 12 | IOCP | DirectStorage | Design |
| Linux | Vulkan 1.4 | io_uring | Staging buffer | Design |
| iOS | Metal 4 | GCD dispatch_io | Metal I/O | Design |
| Android | Vulkan 1.4 | io_uring | Staging buffer | Design |
| Switch | Platform SDK | Platform SDK | Platform SDK | Planned |
| Xbox | Direct3D 12 | Platform SDK | Platform SDK | Planned |

## Philosophy

**Harmonius is about composition.** Complex things emerge from simple parts working together:

- **Generic primitives, not game mechanics** — graphs, tables, attributes, and containers compose
  into quests, inventory, dialogue, and progression. The engine does not know what a "quest" is. It
  knows what a directed graph with conditional edges is.
- **ECS everywhere** — entities are IDs, components are data, systems are functions. Composition
  replaces inheritance.
- **Visual graphs compile to native code** — users compose behavior visually; the engine compiles it
  to optimized machine code through the middleman `.dylib`.
- **No-code means no barriers** — the engine bridges the gap between creator intent and machine
  execution.
- **Platform-native everything** — rather than fighting platforms with abstraction layers, we
  harmonize with each platform's strengths (GCD, io_uring, IOCP, Metal, D3D12).

An engine should not impose structure on games. It should provide **primitives that harmonize** —
that naturally compose into whatever the creator imagines. Puzzle games and MMO shooters built from
the same building blocks, because the blocks are generic enough and the composition model is
powerful enough.

**Harmonius = the harmony of systems.** 15 subsystems, one spatial index, one ECS, one job system,
one frame. Everything in concert.

## Architecture

The engine has 15 subsystems across 6 layers. See [docs/architecture.md](docs/architecture.md) for
clickable diagrams, frame data flow, and links to every design document, test case, feature spec,
requirement, and user story per module.

## Documentation

| Directory | Contents |
|-----------|----------|
| [docs/architecture.md](docs/architecture.md) | Engine overview with Mermaid diagrams |
| [docs/design/](docs/design/) | Design documents with API pseudocode and test cases |
| [docs/features/](docs/features/) | Feature definitions organized by domain |
| [docs/requirements/](docs/requirements/) | Functional and non-functional requirements |
| [docs/user-stories/](docs/user-stories/) | User stories across personas |
| [docs/design/constraints.md](docs/design/constraints.md) | Project-wide design constraints |
