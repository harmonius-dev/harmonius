# Harmonius

A modern cross-platform game engine for real-time 2D, 3D, and XR games. Written in Rust with
platform-native async I/O and mesh-shader/ray-tracing-first rendering.

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
2. **Ship every genre** — provide built-in game framework modules (weapons, NPCs, quests, racing,
   tower defense, building, pets, monetization, etc.) so teams start from gameplay, not boilerplate
3. **Minimize integration friction** — native DCC plugins for Houdini, Maya, and Blender export
   assets directly into the engine pipeline; no intermediate format conversion
4. **Maximize parallelism** — 100% ECS with shared spatial index, async/await game loop, scoped task
   graph, and data-parallel `parallel_for` to saturate all cores every frame
5. **Modern GPU baseline** — require mesh shaders + ray tracing (Metal 4, D3D12, Vulkan 1.4); no
   legacy fallback to maintain
6. **Platform-native I/O** — IOCP, GCD, io_uring with custom IoReactor; no third-party async runtime
   anywhere
7. **Full design coverage** — 87 design documents, 1,381 features, 1,171 requirements, 5,859 user
   stories across 27 personas before implementation begins

## Key Technical Highlights

- **100% ECS architecture** — all simulation data as components, all logic as systems, no parallel
  stores
- **Rust stable only** — no nightly, no unsafe without `// SAFETY:` justification
- **Platform-native async** — IOCP (Windows), GCD (macOS), io_uring (Linux); no tokio, no external
  runtime
- **Custom windowing** — NSWindow, Win32, xcb/Wayland directly; no winit
- **Mesh shaders + ray tracing required** — no legacy pipeline fallback
- **No-code authoring** — all user logic via visual graphs, all assets via visual editors
- **Shared spatial index** — single BVH across physics, rendering, networking, AI, audio, and
  gameplay

## Supported Platforms

| OS | Graphics | Async I/O | Status |
|----|----------|-----------|--------|
| macOS | Metal 4 | GCD / Dispatch IO | Design complete |
| Windows | Direct3D 12 | IOCP | Design complete |
| Linux | Vulkan 1.4 | io_uring | Design complete |
| iOS | Metal 4 | GCD / Dispatch IO | Design complete |
| Android | Vulkan 1.4 | io_uring | Design complete |
| Switch | Platform SDK | Platform SDK | Planned |
| Xbox | Direct3D 12 | Platform SDK | Planned |

## Architecture

The engine has 15 subsystems across 6 layers. See [docs/architecture.md](docs/architecture.md) for
clickable diagrams, frame data flow, and links to every design document, test case, feature spec,
requirement, and user story per module.

## Documentation

| Directory                                                |
|----------------------------------------------------------|
| [docs/architecture.md](docs/architecture.md)             |
| [docs/design/](docs/design/)                             |
| [docs/features/](docs/features/)                         |
| [docs/requirements/](docs/requirements/)                 |
| [docs/user-stories/](docs/user-stories/)                 |
| [docs/standards/](docs/standards/)                       |
| [docs/design/constraints.md](docs/design/constraints.md) |

1. **[docs/architecture.md](docs/architecture.md)** — Engine overview with Mermaid diagrams and
   per-module cross-references
2. **[docs/design/](docs/design/)** — 87 design documents with API pseudocode, class diagrams, and
   companion test cases
3. **[docs/features/](docs/features/)** — 1,381 feature definitions organized by domain
4. **[docs/requirements/](docs/requirements/)** — 1,171 functional and non-functional requirements
5. **[docs/user-stories/](docs/user-stories/)** — 5,859 user stories across 27 personas
6. **[docs/standards/](docs/standards/)** — Coding standards for Rust, C++, Swift, TypeScript, HLSL,
   Markdown, JSON, TOML, YAML
7. **[docs/design/constraints.md](docs/design/constraints.md)** — Project-wide design constraints
   (language, platform, async model)
