# 11.1 — Particle System

## GPU Particle Simulation

| ID       | Feature                            | Requirements |
|----------|------------------------------------|--------------|
| F-11.1.1 | Compute Shader Particle Simulation | R-11.1.1     |
| F-11.1.2 | Particle Simulation Modules        | R-11.1.2     |

1. **F-11.1.1** — GPU-driven particle simulation using compute shaders to update millions of
   particles per frame. Each emitter defines a spawn shape (point, sphere, box, cone, mesh surface,
   skinned mesh), spawn rate, lifetime, and a chain of simulation modules evaluated per-particle.
   Particles are stored in persistent GPU buffers with free-list allocation and indirect dispatch
   for variable particle counts. Skinned mesh emitters sample vertex buffers each frame, enabling
   particles to spawn on animated characters in 40-player raids without CPU readback.
   - **Platform:** Requires async compute queue support; falls back to graphics queue where
     unavailable
2. **F-11.1.2** — Composable simulation modules evaluated per-particle per-frame on the GPU:
   velocity over life, gravity, orbital velocity, curl noise, vector fields, drag, vortex,
   turbulence, attract/repel, color over life, size over life, rotation over life, and collision.
   Collision supports depth-buffer (screen-space) and signed-distance-field (world-space) modes with
   configurable restitution and friction. Modules are compiled into a single compute dispatch per
   emitter to minimize overhead.
   - **Deps:** F-11.1.1
   - **Platform:** Mobile disables SDF collision and curl noise; uses depth-buffer collision only.
     Turbulence and vector fields use lower resolution on mobile.

## Rendering

| ID       | Feature                                   | Requirements |
|----------|-------------------------------------------|--------------|
| F-11.1.3 | Particle Rendering Modes                  | R-11.1.3     |
| F-11.1.4 | Particle LOD, Sorting, and Budget Culling | R-11.1.4     |

1. **F-11.1.3** — Three rendering modes driven by particle state. Sprites use camera-facing
   billboards with atlas flipbook animation, blend modes (additive, alpha, premultiplied), and
   soft-particle depth fade. Ribbons generate spline-based geometry connecting sequential particles
   for sword trails, spell streaks, and projectile trails. Mesh particles use GPU instancing with
   full material support, per-particle transforms, and LOD selection by camera distance.
   - **Deps:** F-11.1.1
   - **Platform:** Mobile favors sprites over mesh particles due to draw call cost. Soft-particle
     depth fade disabled on low-end mobile GPUs.
2. **F-11.1.4** — Hierarchical LOD system that reduces simulation and rendering cost based on camera
   distance, screen coverage, and a global particle budget. Emitters transition through LOD tiers
   (full simulation, reduced spawn rate, billboard impostor, culled) with hysteresis to avoid
   popping. A global budget manager caps total alive particles, prioritizing player-local and
   gameplay-critical effects. Transparent particles are GPU radix-sorted by camera distance with
   per-emitter sort mode selection.
   - **Deps:** F-11.1.1
   - **Platform:** Global particle budget: mobile 10K, Switch 50K, console 200K, desktop 500K+.
     Mobile uses aggressive LOD distances (near/cull thresholds halved).

## Advanced Features

| ID       | Feature                 | Requirements |
|----------|-------------------------|--------------|
| F-11.1.5 | Sub-Emitters and Events | R-11.1.5     |
| F-11.1.6 | Particle Light Emission | R-11.1.6     |
| F-11.1.7 | GPU Fluid Simulation    | R-11.1.7     |

1. **F-11.1.5** — Event-driven child emitter spawning triggered by particle lifecycle events: birth,
   death, collision, or manual trigger. Sub-emitters inherit parent particle transforms and
   velocity, enabling cascading effects like fireworks, impact sparks, or spell detonations without
   authoring monolithic particle systems.
   - **Deps:** F-11.1.1, F-11.1.2
   - **Platform:** Mobile limits sub-emitter depth to 1 (no nested sub-emitters) and caps child
     particle count to prevent cascade-driven budget spikes.
2. **F-11.1.6** — Dynamic point lights spawned from particle positions. Each emitting particle
   contributes a light with configurable color, intensity, and attenuation radius to the clustered
   light buffer. A stochastic sampling strategy caps the number of particle lights evaluated per
   tile to keep lighting cost bounded during spell-heavy combat.
   - **Deps:** F-11.1.1
   - **Platform:** Mobile caps particle lights to 4 per emitter (vs. 16 on desktop) and uses smaller
     attenuation radii. Disabled entirely on low-end mobile GPUs.
3. **F-11.1.7** — Grid-based Eulerian fluid simulation on the GPU for fire, smoke, and liquid
   surface effects. A 3D velocity/density/temperature grid is advected via compute shaders. The
   resulting density field is ray-marched for volumetric rendering with emission, absorption, and
   scattering. Grid resolution scales with LOD distance for large open-world deployments.
   - **Deps:** F-11.1.1
   - **Platform:** Requires async compute queue support. Disabled on mobile GPUs that lack compute.
     Switch uses 32^3 grid (vs. 128^3 desktop). Mobile falls back to sprite particles.
