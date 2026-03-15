# Harmonius Engine — Design Plan

## Context

The Harmonius game engine has 1,381 features, 1,171
requirements, and 5,859 user stories across 15 domains.
A domain decomposition DAG defines 87 feature-groups in
7 concurrency waves. The project uses Rust (stable) with
cxx.rs FFI to C++ libraries and Swift (Apple APIs).

All project-wide constraints are in
[constraints.md](constraints.md).

## Design Document Template

Each design lives at `docs/design/{domain}/{group}.md`:

```
## Requirements Trace
## Overview
## Architecture        (Mermaid diagrams)
## API Design          (Rust pseudocode)
## Data Flow
## Platform Considerations
## Test Plan
## Open Questions
```

---

## Wave 0 — Platform Root — COMPLETE

| # | Document | Status |
|---|----------|--------|
| 0.1 | `platform/windowing.md` | Done |
| 0.2 | `platform/threading.md` | Done |
| 0.3 | `platform/os-integration.md` | Done |

Key decisions locked in Wave 0:

- IoReactor with controlled frame poll point
- async/await for all async abstractions
- Scoped task execution (borrow from scope)
- AsyncMutex / AsyncRwLock / AsyncBarrier
- Sync + async event handlers
- GCD for macOS concurrency (fibers, I/O, Metal)
- io_uring only on Linux (no epoll)
- Custom windowing (no winit)
- Out-of-process crash handler
- BLAKE3 content hashing
- Async file watcher with typed event stream

---

## Wave 1 — Core Runtime (6 design docs)

**Next wave. ECS design starts first.**

| # | Document | Features | Key Deliverables |
|---|----------|----------|-----------------|
| 1.1 | `core-runtime/ecs.md` | F-1.1.1–38 | Archetype storage, component registration, entity lifecycle (generational indices), query system, parallel iteration, system scheduling, command buffers, observer system, multiple worlds. **Most critical — blocks everything.** |
| 1.2 | `core-runtime/scene-transforms.md` | F-1.2.1–7 | Scene hierarchy as ECS relationships, transform propagation, dirty tracking via change detection. |
| 1.3 | `core-runtime/reflection-serialization.md` | F-1.3.1–10, F-1.4.1–10 | bevy_reflect-style Reflect trait, TypeRegistry, property access, binary/text serialization, mixed textual+binary companion files, schema versioning. |
| 1.4 | `core-runtime/events-plugins.md` | F-1.5.1–7, F-1.6.1–7 | Typed event channels, observer dispatch, plugin registration, dependency resolution, hot reload. |
| 1.5 | `core-runtime/memory-async-io.md` | F-1.7.1–9, F-1.8.1–9 | Arena/pool allocators, generational handles, platform I/O backend trait (wraps IoReactor), buffer pools. |
| 1.6 | `core-runtime/spatial-index.md` | F-1.9.1–9 | Shared BVH, incremental update, grid/octree hybrid, unified query API (ray, shape, overlap, frustum, k-nearest), batch parallelism. |

### Interoperability Contracts (lock before Wave 2)

| Contract | Defined In | Consumed By |
|----------|-----------|-------------|
| `Component` trait + archetype API | 1.1 | All domains |
| `Entity` handle (generational index) | 1.1 | All domains |
| `System` trait + scheduling API | 1.1 | All domains |
| `Event<T>` channel API | 1.4 | All domains |
| `SpatialQuery` trait | 1.6 | Physics, Rendering, Networking, AI, Audio |
| `Reflect` / `Serialize` traits | 1.3 | Networking, Content Pipeline, Game Framework |
| `AsyncIo` trait (wraps IoReactor) | 1.5 | Content Pipeline, Platform |
| `Transform` component schema | 1.2 | Rendering, Physics, Animation, Audio |

### Wave 1 Ordering

1. **ECS first** — start immediately; lock API before
   other Wave 1 designs reference it.
2. **Reflection + Memory/IO** — can proceed in parallel
   with ECS (independent of ECS primitives).
3. **Scene, Events, Spatial** — depend on ECS primitives;
   begin once ECS API is locked.

---

## Wave 2 — Infrastructure Fan-Out (9 design docs)

**All 9 proceed in parallel once Wave 1 contracts lock.**

| # | Document | Features |
|---|----------|----------|
| 2.1 | `rendering/gpu-abstraction.md` | F-2.1.1–12 (Metal, D3D12, Vulkan backends, memory, pipelines) |
| 2.2 | `content-pipeline/asset-import.md` | F-12.1.1–5, F-12.3.1–10 (native format ingestion, CAS database) |
| 2.3 | `physics/foundation.md` | F-4.1.1–10, F-4.2.1–9 (rigid body ECS, broadphase, narrowphase, character controller) |
| 2.4 | `input/devices-actions.md` | F-6.1.1–5, F-6.2.1–11 (device abstraction, action mapping, combos) |
| 2.5 | `audio/engine.md` | F-5.1.1–7, F-5.2.1–7 (mixer, voice management, spatial, HRTF) |
| 2.6 | `animation/skeletal.md` | F-9.1.1–10 (GPU skinning, blending, root motion, retargeting) |
| 2.7 | `networking/transport.md` | F-8.1.1–8 (UDP, reliable/unreliable channels, DTLS) |
| 2.8 | `physics/constraints.md` | F-4.3.1–9 (joints, ragdoll, breakable joints) |
| 2.9 | `physics/advanced.md` | F-4.4–4.8 (spatial queries, vehicles, destruction, cloth, fluids) |

---

## Wave 3 — Intermediate Systems (12 design docs)

| # | Document | Features |
|---|----------|----------|
| 3.1 | `rendering/render-graph.md` | F-2.2.1–13 (DAG, passes, barriers, scheduling) |
| 3.2 | `content-pipeline/streaming.md` | F-12.5.1–10 (VFS, GPU direct storage, streaming) |
| 3.3 | `content-pipeline/processing.md` | F-12.2.1–9 (texture compression, shader compilation) |
| 3.4 | `input/gestures-haptics.md` | F-6.3–6.5 (gestures, haptics, VR input) |
| 3.5 | `audio/dsp-music.md` | F-5.3–5.5 (DSP, adaptive music, voice chat) |
| 3.6 | `animation/state-machine.md` | F-9.4, F-9.2 (state graph, blend spaces, morph) |
| 3.7 | `ai/navigation.md` | F-7.1.1–15 (NavMesh, A*, dynamic obstacles) |
| 3.8 | `networking/replication.md` | F-8.2–8.4 (replication, prediction, rollback) |
| 3.9 | `geometry/meshlets.md` | F-3.1.1–7 (meshlet pipeline, visibility buffer) |
| 3.10 | `ui/widget-framework.md` | F-10.1.1–14 (retained widget tree, layout, styling) |
| 3.11 | `vfx/particles.md` | F-11.1.1–7 (GPU particles, LOD, sub-emitters) |
| 3.12 | `animation/procedural.md` | F-9.3.1–11 (IK, ragdoll blend, foot placement) |

---

## Wave 4 — Integration Layer (19 design docs)

| # | Document | Features |
|---|----------|----------|
| 4.1 | `rendering/core-rendering.md` | F-2.3 (forward+/deferred, PBR, culling) |
| 4.2 | `rendering/lighting.md` | F-2.4 (direct, area, shadows, AO) |
| 4.3 | `rendering/scene-pipeline.md` | F-2.10 (ECS bridge, render proxy, batching) |
| 4.4 | `content-pipeline/hot-reload.md` | F-12.4 (file watcher, hot reload) |
| 4.5 | `ai/behavior.md` | F-7.3–7.5 (behavior trees, utility AI, GOAP) |
| 4.6 | `ai/perception.md` | F-7.6 (sight, hearing, smell, tracking) |
| 4.7 | `ai/steering-crowds.md` | F-7.2, F-7.7 (RVO, formations, crowds) |
| 4.8 | `game-framework/primitives.md` | F-13.1 (game modes, modular systems) |
| 4.9 | `game-framework/save-cinematics.md` | F-13.3, F-13.5 (save/load, sequencer) |
| 4.10 | `geometry/terrain.md` | F-3.2 (heightfield, voxel, streaming) |
| 4.11 | `geometry/environment.md` | F-3.3–3.5 (foliage, water, sky) |
| 4.12 | `ui/hud-widgets.md` | F-10.2–10.4 (widgets, HUD, rendering) |
| 4.13 | `ui/2d-games.md` | F-10.5 (sprites, tilemaps, 2D physics) |
| 4.14 | `ui/accessibility.md` | F-10.6 (screen reader, colorblind, WCAG) |
| 4.15 | `networking/sessions-replay.md` | F-8.5–8.6 (matchmaking, replay) |
| 4.16 | `networking/mmo.md` | F-8.7 (world sharding, persistent state) |
| 4.17 | `networking/anti-cheat.md` | F-8.8 (server-side detection) |
| 4.18 | `vfx/effects.md` | F-11.2–11.5 (decals, screen, weather, destruction) |
| 4.19 | `content-pipeline/dcc-versioning.md` | F-12.6–12.7 (DCC plugins, asset versioning) |

---

## Wave 5 — Advanced Systems (20 design docs)

| # | Document | Features |
|---|----------|----------|
| 5.1 | `rendering/advanced.md` | F-2.5 (ray tracing, GI, path tracing) |
| 5.2 | `rendering/environment-character.md` | F-2.6–2.8 (AA, environment, character) |
| 5.3 | `rendering/post-processing.md` | F-2.9 (bloom, DOF, motion blur, tone) |
| 5.4 | `rendering/stylized-materials.md` | F-2.11–2.12 (toon, glass, fabric) |
| 5.5 | `tools/editor-framework.md` | F-15.1 (panels, viewports, undo, gizmos) |
| 5.6 | `tools/logic-graph.md` | F-15.8 (visual logic graph runtime) |
| 5.7 | `tools/level-world.md` | F-15.2, F-15.6 (level editor, terrain) |
| 5.8 | `tools/material-animation.md` | F-15.3–15.4 (material, animation editors) |
| 5.9 | `tools/profiler.md` | F-15.5 (CPU, GPU, memory profiling) |
| 5.10 | `game-framework/quest-dialogue.md` | F-13.6 (quests, branching dialogue) |
| 5.11 | `game-framework/databases.md` | F-13.7 (tables, loot, stats, currencies) |
| 5.12 | `game-framework/abilities-combat.md` | F-13.10 (abilities, melee, ranged) |
| 5.13 | `game-framework/weapons.md` | F-13.16 (fire modes, ballistics) |
| 5.14 | `game-framework/character.md` | F-13.8–13.9 (customization, inventory) |
| 5.15 | `animation/cloth-hair.md` | F-9.5 (GPU cloth, strand hair) |
| 5.16 | `animation/first-person.md` | F-9.6 (FP camera, weapon sway) |
| 5.17 | `vfx/effect-graph.md` | F-11.6 (node-based VFX editor) |
| 5.18 | `game-framework/destruction.md` | F-4.6 (fracture, runtime destruction) |
| 5.19 | `game-framework/traversal-stealth.md` | F-13.17–13.18 (parkour, stealth) |
| 5.20 | `game-framework/camera.md` | F-13.25 (virtual cameras, spring arm) |

---

## Wave 6 — Capstone Systems (18 design docs)

| # | Document | Features |
|---|----------|----------|
| 6.1 | `geometry/procedural-generation.md` | F-3.6 (PCG graph, planet-scale) |
| 6.2 | `game-framework/npc-simulation.md` | F-13.19 (relationships, schedules) |
| 6.3 | `game-framework/progression-social.md` | F-13.12–13.13 (talents, guilds) |
| 6.4 | `game-framework/building-survival.md` | F-13.14–13.15 (building, survival) |
| 6.5 | `game-framework/genre-specific.md` | F-13.20–22, F-13.26–28 (fog, racing, etc.) |
| 6.6 | `game-framework/monetization.md` | F-13.23–13.24 (battle pass, DLC) |
| 6.7 | `game-framework/selection.md` | F-13.11 (picking, marquee, groups) |
| 6.8 | `game-framework/scripting.md` | F-13.4 (logic graph gameplay) |
| 6.9 | `tools/version-control.md` | F-15.10 (Git, LFS, merge driver) |
| 6.10 | `tools/collaboration.md` | F-15.12 (CRDT, remote rendering) |
| 6.11 | `tools/shared-cache.md` | F-15.11 (compiled asset cache, CI) |
| 6.12 | `tools/ai-governance.md` | F-15.7, F-15.9 (AI governance, assistant) |
| 6.13 | `tools/deployment.md` | F-15.14, F-15.16 (packaging, mod support) |
| 6.14 | `tools/launcher.md` | F-15.15 (version management) |
| 6.15 | `tools/server-infrastructure.md` | F-15.18 (AWS CDK stacks) |
| 6.16 | `tools/asset-store.md` | F-15.17 (marketplace) |
| 6.17 | `tools/localization-docs.md` | F-15.13, F-15.19 (localization, docs) |
| 6.18 | `platform/services-storage.md` | F-14.5 (achievements, cloud save, caches) |

---

## Execution Strategy

### Design Review Lifecycle

1. **Draft** — write from features/requirements/stories
2. **Self-review** — validate requirements traceability
3. **Human review** — present for approval
4. **Lock** — approved; downstream designs may reference
5. **Implementation** — code against locked design

### Contract Locking Protocol

Before Wave 2, these Wave 1 contracts must lock:

1. ECS: `Component`, `Entity`, `System`, `World`
2. Events: `Event<T>`, observer dispatch
3. Spatial: `SpatialQuery` (ray, shape, overlap, frustum)
4. Reflection: `Reflect` trait, `TypeRegistry`
5. Serialization: binary + textual + companion files
6. Async I/O: trait wrapping IoReactor
7. Transform: component schema
8. Memory: allocator traits, generational handles

### MVP Fast Path

13 designs on the critical path to first playable:

| Wave | Designs Required |
|------|-----------------|
| 0 | 0.1 windowing, 0.2 threading (done) |
| 1 | 1.1 ECS, 1.2 scene, 1.5 memory/IO |
| 2 | 2.1 GPU, 2.2 assets, 2.3 physics, 2.4 input, 2.6 animation |
| 3 | 3.1 render graph, 3.3 processing, 3.6 state machine, 3.10 UI |
| 4 | 4.1 core rendering, 4.2 lighting, 4.3 scene pipeline |

### Verification (per wave)

1. Every requirement has a design element
2. Downstream designs reference only locked contracts
3. Every design includes test scenarios
4. All Mermaid diagrams render correctly
5. All content within 100 character line limit

---

## Summary

| Wave | Docs | Status |
|------|------|--------|
| 0 | 3 | **Complete** |
| 1 | 6 | Next |
| 2 | 9 | Blocked on Wave 1 |
| 3 | 12 | Blocked on Wave 2 |
| 4 | 19 | Blocked on Wave 3 |
| 5 | 20 | Blocked on Wave 4 |
| 6 | 18 | Blocked on Wave 5 |
| **Total** | **87** | |
