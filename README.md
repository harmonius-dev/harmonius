# Harmonius

A modern cross-platform game engine for real-time 2D, 3D, and XR games. Written in Rust with
platform-native async I/O and mesh-shader/ray-tracing-first rendering.

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

| Directory | What you will find |
|-----------|-------------------|
| [docs/architecture.md](docs/architecture.md) | Engine overview with Mermaid diagrams and per-module cross-references |
| [docs/design/](docs/design/) | 87 design documents with API pseudocode, class diagrams, and companion test cases |
| [docs/features/](docs/features/) | 1,381 feature definitions organized by domain |
| [docs/requirements/](docs/requirements/) | 1,171 functional and non-functional requirements |
| [docs/user-stories/](docs/user-stories/) | 5,859 user stories across 27 personas |
| [docs/standards/](docs/standards/) | Coding standards for Rust, C++, Swift, TypeScript, HLSL, Markdown, JSON, TOML, YAML |
| [docs/design/constraints.md](docs/design/constraints.md) | Project-wide design constraints (language, platform, async model) |
| [docs/design/plan.md](docs/design/plan.md) | Design wave schedule and dependency DAG |
