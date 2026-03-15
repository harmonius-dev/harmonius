# 11.6 — Effect Graph

## Authoring

### F-11.6.1 Node-Based Effect Graph Editor

A visual node-graph editor for authoring VFX systems without writing code. Effect graphs compose
spawn, update, and render modules as nodes connected by typed data flow edges. Spawn modules define
emission shapes, rates, and burst patterns. Update modules define forces, turbulence, color/size
curves, collision, and custom behavior. Render modules select rendering mode (sprite, mesh, ribbon,
decal) and material. The graph compiles to GPU compute dispatches (F-11.1.1) for runtime execution.
The editor provides real-time preview with scrubbing, looping, and performance statistics.

- **Requirements:** R-11.6.1
- **Dependencies:** F-15.8.1 (Universal Logic Graph Runtime), F-11.1.1 (GPU Particle Simulation)
- **Platform notes:** Editor is desktop-only. Compiled effect graphs run on all platforms. Mobile
  graphs are compiled with node count limits (32 vs. 128 on desktop).

### F-11.6.2 Custom Effect Graph Nodes

Extend the effect graph with custom nodes authored in the logic graph system (F-15.8.4). Custom
nodes define typed inputs, outputs, and a visual graph body that executes per-particle or
per-emitter. Examples: sample a noise field, read game state (health percentage to particle color),
apply custom forces (black hole attractor, wind tunnel), or trigger gameplay events when particles
collide. Custom nodes are packaged as reusable library assets that appear in the node palette
alongside built-in nodes.

- **Requirements:** R-11.6.2
- **Dependencies:** F-11.6.1, F-15.8.4 (Gameplay Logic Graphs), F-15.8.10 (Graph Node Library)
- **Platform notes:** Per-particle custom nodes are expensive; mobile restricts custom nodes to
  per-emitter execution only to avoid GPU compute overhead.

### F-11.6.3 Effect Graph Parameter System

Expose effect graph parameters to the outside world: material-style parameter slots (float, vector,
color, curve, gradient, texture) that designers set per-instance in the level editor or bind to game
state via reactive data binding (F-10.1.7). Parameters drive spawn rate multipliers, color tints,
force magnitudes, and render modes. Effect templates define parameter defaults; instances override
individual parameters. Parameters are animated via the sequencer (F-13.5.1) for cinematic VFX
choreography.

- **Requirements:** R-11.6.3
- **Dependencies:** F-11.6.1, F-10.1.7 (Reactive Data Binding)
- **Platform notes:** Lightweight parameter binding; runs identically on all platforms. Mobile uses
  platform-variant defaults (lower spawn rate multipliers).

### F-11.6.4 Event-Driven Effect Spawning

Spawn VFX in response to gameplay events through the ECS observer system (F-1.1.30). Event sources
include: animation notifies (F-9.1.9 -- sword slash spawns sparks), physics collisions (F-4.2.4 --
impact spawns dust), ability activation (gameplay events to muzzle flash), destruction events
(F-4.6.1 -- fracture spawns debris VFX), and custom logic graph events. Each event carries context
data (position, normal, velocity, surface material) that parameterizes the spawned effect. Effects
auto-attach to moving entities or spawn at world positions.

- **Requirements:** R-11.6.4
- **Dependencies:** F-11.6.1, F-1.1.30 (Observers), F-9.1.9 (Animation Events)
- **Platform notes:** Mobile throttles event-driven spawns: low-priority effects (ambient dust,
  distant impacts) are skipped under budget pressure.

### F-11.6.5 VFX LOD and Performance Budget

Automatic VFX quality scaling based on distance, screen coverage, and a global VFX performance
budget. LOD tiers reduce particle counts, simulation fidelity, and rendering complexity for distant
or partially occluded effects. A global budget manager tracks total particle count, GPU compute
time, and overdraw contribution, scaling down lower-priority effects when the budget is exceeded.
Priority is assigned per-effect (hero abilities > ambient particles > distant environment VFX). The
budget integrates with the dynamic resolution system to maintain frame rate targets.

- **Requirements:** R-11.6.5
- **Dependencies:** F-11.1.1, F-1.9.1 (Shared BVH)
- **Platform notes:** Mobile platforms use aggressive LOD defaults with lower particle caps.
