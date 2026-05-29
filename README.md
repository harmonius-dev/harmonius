# Harmonius

An open-source, cross-platform game engine for real-time 2D, 3D, and XR games.
Built in Rust around a single ECS, a single spatial index, and a single job
system — because harmony is what emerges when every part composes cleanly into
every other part.

## Building and Testing

Harmonius uses SwiftPM as the source of truth for Swift targets, shader
artifacts, package tests, render snapshots, and Appium UI tests. XcodeGen still
owns Apple app bundles, resources, signing, schemes, archives, and destinations.

Bash is the canonical automation layer. VS Code tasks, CI jobs, and LLM agents
should call `scripts/dev.sh` instead of hand-expanding dependency order.

### Prerequisites

For macOS and iOS development:

- macOS 26 + Xcode 26
- XcodeGen: `brew install xcodegen`
- `swift-format`, `jq`, `pkg-config`, and `appium`

For Linux package development:

- Swift 6.3 or newer
- `pkg-config`, Git, certificates, and a C/C++ compiler toolchain
- Vulkan and X11 development packages for future native Linux rendering work

Run `./scripts/setup_environment.sh` to install supported host dependencies.
See [docs/environment-setup.md](docs/environment-setup.md) for details.

### Common Commands

```bash
./scripts/dev.sh bootstrap macos
./scripts/dev.sh package-graph
./scripts/dev.sh compile-spm macos debug
./scripts/dev.sh bundle macos debug
./scripts/dev.sh test
./scripts/dev.sh full-check
```

VS Code exposes the same flow through aggregate tasks such as `dev:bootstrap`,
`dev:compile`, `dev:test`, `dev:run:macos`, and `dev:full-check`.

See [docs/testing.md](docs/testing.md) for test targets and snapshots. See
[docs/agent-workflows.md](docs/agent-workflows.md) for LLM workflow cards.

## Who Harmonius Is For

### Artists

Author everything visually. Materials, VFX, animations, UI, lighting, and characters have dedicated
node-based editors with real-time previews and hot reload. Import via industry formats (glTF,
Alembic, PNG, EXR, KTX2) — no DCC plugins required. Mesh shaders, ray tracing, dynamic global
illumination, and a GPU-driven render pipeline are baseline, not premium features.

### Game Designers

Build gameplay without writing code. Quests, dialogue, abilities, inventory, and AI behavior are
composed in visual graphs and data tables, then compiled to native Rust via a bundled toolchain.
Hot-reload in the editor lets you iterate without restarting. The engine ships with generic
primitives (directed graphs, data tables, attributes/effects, containers/slots, timelines, event
logs) — not opinionated "quest systems" you have to fight. Every genre is supported: puzzle, 2D
platformer, 3D RPG, RTS, FPS, co-op, local multiplayer, and VR.

### Game Developers

Stable Rust, zero reflection, static codegen, custom ECS with AoSoA tiled storage, custom job system
with work-stealing, and platform-native I/O (io_uring, IOCP, GCD). Mesh shaders and ray tracing are
required — no legacy pipeline to support. Every subsystem has a design document, test plan, and
pair-wise integration spec written before any code. Apache 2.0, no royalties, no seat fees, no
vendor lock-in.

## What Makes Harmonius Different

- **Everything compiles to Rust** — visual graphs for logic, materials, AI,
  animation, and VFX become real Rust source, compiled via bundled rustc into a
  shared middleman `.dylib` for editor hot-reload and statically linked into
  ship builds. One language, one compiler, one type system.
- **Composition over inheritance** — the engine has no "quest" or "inventory" subsystem, only
  generic primitives that compose into any mechanic. Puzzle games and MMO shooters built from the
  same blocks.
- **2D and 3D share one engine** — sprites, tilemaps, 2D physics, 2D lighting, skeletal 2D
  animation, vector graphics, and full 3D rendering share the same ECS, render graph, and spatial
  index.
- **Zero reflection, zero async runtimes** — no `dyn Reflect`, no `TypeRegistry`, no tokio, no
  `Future`. Static dispatch everywhere; platform-native I/O with the main thread polling
  completions.
- **Design-first** — 1,200+ features, 1,500+ requirements, pair-wise integration specs, and test
  cases written before implementation begins.
- **Accessibility and localization are core** — screen reader bridges, colorblind filters, WCAG
  compliance, TTS, subtitles, input remapping, and `LocalizedStringId` replacing hardcoded strings
  are core runtime services, not UI add-ons.

## Under The Hood

- **Custom ECS** — archetype-based with AoSoA tiled storage for SIMD, compiled query plans with
  bloom filters, parallel iteration
- **Custom job system** — Chase-Lev work-stealing deques via crossbeam, data-parallel primitives
  (par_iter, join, scope, par_sort), QoS-based scheduling for hybrid CPUs
- **GPU-driven rendering** — Nanite-style cluster LOD, compute culling, HZB occlusion, indirect
  draw, render graph with automatic barriers, per-frame ring buffers
- **Platform-native I/O** — io_uring (Linux), IOCP + DirectStorage (Windows), GCD + Metal I/O
  (Apple); main thread polls completions; zero blocking operations
- **Custom windowing** — NSWindow, Win32, X11/Wayland directly; no winit
- **HLSL shader pipeline** — all shaders authored in HLSL, compiled via DXC
  and Metal Shader Converter CLI
- **Deterministic fixed timestep** — 30/60/120 fps tiers with rollback-friendly semantics for
  replays, netcode, and testing
- **Pair-wise integration specs** — 50 subsystem-pair contracts define every cross-domain edge

## Philosophy

**Harmonius is about composition.** An engine should not impose structure on games — it should
provide primitives that harmonize and compose into whatever the creator imagines.

- **Generic primitives, not game mechanics** — graphs, tables, attributes, and containers compose
  into quests, inventory, dialogue, and progression. The engine does not know what a "quest" is. It
  knows what a directed graph with conditional edges is.
- **ECS everywhere** — entities are IDs, components are data, systems are functions.
- **No-code means no barriers** — creators compose behavior visually; the engine compiles it to
  optimized native code through the middleman `.dylib`.
- **Platform-native everything** — harmonize with each platform's strengths (GCD, io_uring, IOCP,
  Metal, D3D12) rather than fighting them with abstraction layers.

**Harmonius = the harmony of systems.** 15 subsystems, one spatial index, one ECS, one job system,
one frame. Everything in concert.

## Goals

1. **Universal game engine** — support every genre in a single engine
2. **No-code-first** — all user-facing authoring is visual
3. **Production-grade Rust** — stable Rust only, zero nightly, modern GPU graphics
4. **Open source** — Apache 2.0 with open asset store and community marketplace
5. **Self-hosted infrastructure** — all services open source with 1-click AWS
   deployment; customers pay AWS directly, no vendor lock-in
6. **Privacy-respecting AI** — cloud AI backends (Claude, Cursor, Copilot) use the customer's own
   API keys; engine is a thin client, never a proxy
