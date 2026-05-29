# Harmonius Engine Documentation

Welcome to the Harmonius game engine documentation. This is the starting point
for understanding the engine's design, capabilities, and how to use it.

## Quick Start

- **Just getting started?** → [Engine Overview](./overview/README.md) covers
  all 17 engine domains with topic breakdowns and integration guidance.
- **Working locally?** → [Environment Setup](./environment-setup.md) and
  [Testing](./testing.md) cover the Bash, SwiftPM, XcodeGen, and Appium flow.
- **Using an LLM agent?** → [Agent Workflows](./agent-workflows.md) lists the
  public VS Code tasks and matching Bash commands.
- **Looking for a specific topic?** → Navigate the domain READMEs in
  [overview/](./overview/).

## Documentation Structure

```text
docs/
  README.md                — this file
  overview/
    README.md              — index of all 17 domains and 62 topics
    {domain}/
      README.md            — domain overview + key takeaways + risks
      topic-1.md
      topic-2.md
      ...
```

## The 17 Domains

All engine functionality is organized into 17 independent domains:

1. **Core Runtime** — World model, simulation loop, entity-component system.
2. **Data Systems** — Generic building blocks (graphs, tables, containers, attributes).
3. **Simulation** — Spatial grids, environmental awareness, timelines.
4. **Platform** — OS integration, threading, file I/O, crash handling.
5. **Content Pipeline** — Asset import, versioning, streaming, hot reload.
6. **Rendering** — Rasterization, lighting, materials, effects, antialiasing.
7. **Physics** — Rigid bodies, joints, constraints, destruction, raycasts.
8. **Audio** — Playback, mixing, spatial audio, effects.
9. **Input** — Devices, actions, gestures, haptics.
10. **Animation** — Skeletal rigs, blending, IK, motion matching.
11. **AI** — Behavior trees, perception, pathfinding, crowds.
12. **Geometry** — Meshes, terrain, foliage, procedural generation.
13. **VFX** — Particles, decals, weather, destruction effects.
14. **UI** — Widgets, layouts, HUD, accessibility.
15. **Networking** — Replication, prediction, RPC, anti-cheat.
16. **Game Framework** — Camera, saves, scripting, world streaming, gameplay.
17. **Tools** — Editor, profiling, CI/CD, content services.

See [Engine Overview](./overview/README.md) for the complete index and navigation guide.

## Key Concepts Across Domains

- **Composition over inheritance** — Features layer generic primitives (graphs + attributes +
  containers) rather than rigid hierarchies.
- **Decoupling via messaging** — Systems communicate through events, not direct references.
- **Determinism** — Physics and AI produce reproducible results given identical inputs (enables
  replay, rollback).
- **Streaming** — Content loads on-demand; memory is bounded regardless of world size.
- **Platform abstraction** — Single codebase runs on Windows, macOS, Linux, consoles.

## Integration Points

Cross-domain integration happens through well-defined contracts:

- **Input → Animation** — Player actions trigger state transitions.
- **Animation → Rendering** — Skeletal animation drives mesh deformation.
- **Physics → Audio** — Impact velocities drive sound volume and pitch.
- **AI → Rendering** — Perception results influence behavior; behavior drives visual effects.
- **Networking → Simulation** — Client prediction mirrors server authority;
  reconciliation corrects divergence.

Each domain README lists "Integration risks" for fragile contracts requiring
careful coordination. See [overview/README.md](./overview/README.md) for
guidance on managing these risks.

## Reading Recommendations

- **Entity-component model** →
  [Core Runtime — World and Entities](./overview/core-runtime/world-and-entities.md)
- **Rendering pipeline** → [Rendering — Pipeline](./overview/rendering/pipeline.md)
- **Physics simulation** → [Physics — Dynamics](./overview/physics/dynamics.md)
- **Gameplay systems** → [Game Framework](./overview/game-framework/README.md)
- **Performance optimization** → [Tools —
  Profiling](./overview/tools/profiling-and-collaboration.md)

## Contributing

All documentation lives in `docs/` as Markdown. See [AGENTS.md](../AGENTS.md)
for rules on formatting, line lengths, and link conventions.
