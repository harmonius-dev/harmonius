# Engine Overview

Complete reference for all engine systems organized by domain. Each domain is a self-contained
subsystem; understanding cross-domain integration points is essential for system design.

## All 17 Domains

| Domain | Purpose | Topics |
|--------|---------|--------|
| [Core Runtime](./core-runtime/README.md) | Foundation every system sits on | 5 topics |
| [Data Systems](./data-systems/README.md) | Generic building blocks for game data | 5 topics |
| [Simulation](./simulation/README.md) | Grids, spatial awareness, timelines | 4 topics |
| [Platform](./platform/README.md) | OS integration and threading | 4 topics |
| [Content Pipeline](./content-pipeline/README.md) | Source to runtime asset flow | 3 topics |
| [Rendering](./rendering/README.md) | Drawing, lighting, effects | 5 topics |
| [Physics](./physics/README.md) | Motion, forces, collisions | 4 topics |
| [Audio](./audio/README.md) | Playback, mixing, spatial audio | 4 topics |
| [Input](./input/README.md) | Devices and player actions | 3 topics |
| [Animation](./animation/README.md) | Skeletal, procedural, physical | 4 topics |
| [AI](./ai/README.md) | Perception, decision-making, movement | 4 topics |
| [Geometry](./geometry/README.md) | Meshes, terrain, procedural generation | 4 topics |
| [VFX](./vfx/README.md) | Particles, decals, weather | 3 topics |
| [UI](./ui/README.md) | Widgets, layouts, accessibility | 3 topics |
| [Networking](./networking/README.md) | Multiplayer synchronization | 4 topics |
| [Game Framework](./game-framework/README.md) | Camera, saves, scripting, world | 6 topics |
| [Tools](./tools/README.md) | Editor, profiling, deployment | 5 topics |

## Reading Guide

**New to the engine?** Start with [Core Runtime](./core-runtime/README.md). The world model,
entity-component system, and frame loop are foundation concepts all other systems depend on.

**Implementing a specific feature?** Each domain README lists key takeaways and integration risks.
Integration risks are fragile contracts between domains; read linked topic files for detailed
guidance.

**System design question?** Check "Key takeaways" sections across related domains. For example,
character control involves [Input](./input/README.md), [Animation](./animation/README.md),
[Physics](./physics/README.md), and [Game Framework](./game-framework/README.md).

**Performance issue?** See
[Tools — profiling-and-collaboration](./tools/profiling-and-collaboration.md) and corresponding
domain topic for optimization guidance.

**Cross-domain integration?** Read the "Integration risks" section in each domain README; risks are
linked to topics explaining resolution.

## Topic Files

Each domain contains 3–6 topic files covering key concepts:

```text
docs/overview/
  {domain}/
    README.md                 — overview + key takeaways + risks
    topic-1.md               — focused concept breakdown
    topic-2.md               — ...
```

Topic files follow the same structure:

- **What it covers** — bullet list of key points.
- **Concepts** — detailed explanations with subsections.
- **How it fits** — links to related topics (cross-domain integration).

Topic links use relative paths: `./sibling.md` for same domain, `../other-domain/topic.md` for
cross-domain.
