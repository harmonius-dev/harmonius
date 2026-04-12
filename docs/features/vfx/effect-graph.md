# 11.6 — Effect Graph

## Authoring

| ID       | Feature                        |
|----------|--------------------------------|
| F-11.6.1 | Node-Based Effect Graph Editor |
| F-11.6.2 | Custom Effect Graph Nodes      |
| F-11.6.3 | Effect Graph Parameter System  |
| F-11.6.4 | Event-Driven Effect Spawning   |
| F-11.6.5 | VFX LOD and Performance Budget |

1. **F-11.6.1** — A visual node-graph editor for authoring VFX systems without writing code. Effect
   graphs compose spawn, update, and render modules as nodes connected by typed data flow edges.
   Spawn modules define emission shapes, rates, and burst patterns. Update modules define forces,
   turbulence, color/size curves, collision, and custom behavior. Render modules select rendering
   mode (sprite, mesh, ribbon, decal) and material. The graph compiles to GPU compute dispatches
   (F-11.1.1) for runtime execution. The editor provides real-time preview with scrubbing, looping,
   and performance statistics.
   - **Deps:** F-15.8.1 (Universal Logic Graph Runtime), F-11.1.1 (GPU Particle Simulation)
   - **Platform:** Editor is desktop-only. Compiled effect graphs run on all platforms. Mobile
     graphs are compiled with node count limits (32 vs. 128 on desktop).
2. **F-11.6.2** — Extend the effect graph with custom nodes authored in the logic graph system
   (F-15.8.4). Custom nodes define typed inputs, outputs, and a visual graph body that executes
   per-particle or per-emitter. Examples: sample a noise field, read game state (health percentage
   to particle color), apply custom forces (black hole attractor, wind tunnel), or trigger gameplay
   events when particles collide. Custom nodes are packaged as reusable library assets that appear
   in the node palette alongside built-in nodes.
   - **Deps:** F-11.6.1, F-15.8.4 (Gameplay Logic Graphs), F-15.8.10 (Graph Node Library)
   - **Platform:** Per-particle custom nodes are expensive; mobile restricts custom nodes to
     per-emitter execution only to avoid GPU compute overhead.
3. **F-11.6.3** — Expose effect graph parameters to the outside world: material-style parameter
   slots (float, vector, color, curve, gradient, texture) that designers set per-instance in the
   level editor or bind to game state via reactive data binding (F-10.1.7). Parameters drive spawn
   rate multipliers, color tints, force magnitudes, and render modes. Effect templates define
   parameter defaults; instances override individual parameters. Parameters are animated via the
   Cinematics Editor (F-13.5.1) for cinematic VFX choreography.
   - **Deps:** F-11.6.1, F-10.1.7 (Reactive Data Binding)
   - **Platform:** Lightweight parameter binding; runs identically on all platforms. Mobile uses
     platform-variant defaults (lower spawn rate multipliers).
4. **F-11.6.4** — Spawn VFX in response to gameplay events through the ECS observer system
   (F-1.1.30). Event sources include: animation notifies (F-9.1.9 -- sword slash spawns sparks),
   physics collisions (F-4.2.4 -- impact spawns dust), ability activation (gameplay events to muzzle
   flash), destruction events (F-4.6.1 -- fracture spawns debris VFX), and custom logic graph
   events. Each event carries context data (position, normal, velocity, surface material) that
   parameterizes the spawned effect. Effects auto-attach to moving entities or spawn at world
   positions.
   - **Deps:** F-11.6.1, F-1.1.30 (Observers), F-9.1.9 (Animation Events)
   - **Platform:** Mobile throttles event-driven spawns: low-priority effects (ambient dust, distant
     impacts) are skipped under budget pressure.
5. **F-11.6.5** — Automatic VFX quality scaling based on distance, screen coverage, and a global VFX
   performance budget. LOD tiers reduce particle counts, simulation fidelity, and rendering
   complexity for distant or partially occluded effects. A global budget manager tracks total
   particle count, GPU compute time, and overdraw contribution, scaling down lower-priority effects
   when the budget is exceeded. Priority is assigned per-effect (hero abilities > ambient particles
   > distant environment VFX). The budget integrates with the dynamic resolution system to maintain
   frame rate targets.
   - **Deps:** F-11.1.1, F-1.9.1 (Shared BVH)
   - **Platform:** Mobile platforms use aggressive LOD defaults with lower particle caps.

## Simulation Forces and Outputs

| ID       | Feature                      |
|----------|------------------------------|
| F-11.6.6 | Force Field Volumes          |
| F-11.6.7 | VFX Audio Emission           |
| F-11.6.8 | VFX Volumetric Density Output |

1. **F-11.6.6** — `ForceField` components expose configurable shapes (sphere, box, capsule,
   cylinder), force kinds (radial, vortex, directional, drag), strength, and falloff curves. Force
   fields influence both particle simulation and debris trajectories, applying accumulated
   accelerations during the GPU particle update step. Multiple overlapping force fields compose
   additively. Used for wind tunnels, black hole attractors, updrafts, repulsor explosions, and
   environmental drag zones (water, dense fog).
   - **Deps:** F-11.1.1, F-11.1.2
   - **Platform:** Mobile caps concurrent force fields per particle system (4 vs 16 on desktop). All
     shapes supported on all platforms.
2. **F-11.6.7** — Effect graph `AudioEmit` output nodes emit spatial audio events triggered by
   particle lifecycle events: spawn, death, and collision. Each emission carries the particle
   position, velocity, and surface context (for collision) which parameterize the resulting audio
   source. Used to play drip sounds at rain puddle formation, sparking hits on metal collisions, and
   crackle for fire particles. Audio events flow into the spatial audio system for HRTF and
   occlusion processing.
   - **Deps:** F-11.6.1, F-5.1.3 (Spatial Audio)
   - **Platform:** Mobile caps audio event emission rate per emitter to stay within the audio voice
     budget (16 voices). All platforms share the same emission semantics.
3. **F-11.6.8** — Effect graph `VolumetricDensity` output nodes inject per-particle density and
   color into the volumetric fog froxel grid (F-2.7.2), enabling smoke and steam effects to
   participate in volumetric light scattering and shadow receiving. Density contributions are
   accumulated per froxel with temporal reprojection for stability. Used for battlefield smoke,
   magical fog clouds, steam vents, and weather-driven volumetric fog enhancements.
   - **Deps:** F-11.6.1, F-2.7.2 (Ray-Marched Volumetric Fog)
   - **Platform:** Mobile: disabled; volumetric fog uses screen-space height fog fallback. Switch:
     reduced froxel grid resolution. Desktop/High-end: full froxel injection with temporal
     reprojection.
