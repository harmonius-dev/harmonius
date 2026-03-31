# R-11.1 -- Particle System Requirements

## GPU Particle Simulation

1. **R-11.1.1** — The engine **SHALL** simulate particles on the GPU using compute shaders,
   supporting point, sphere, box, cone, mesh surface, and skinned mesh spawn shapes, with persistent
   GPU buffer allocation, free-list management, and indirect dispatch for variable particle counts.
   - **Rationale:** GPU-driven simulation eliminates CPU bottlenecks, enabling millions of particles
     per frame and skinned-mesh emission without CPU readback.
   - **Verification:** Spawn particles from each shape type and confirm all simulation runs on
     compute with zero CPU readback. Measure throughput exceeds one million particles at 60 fps on
     reference hardware.

2. **R-11.1.2** — The engine **SHALL** provide composable per-particle simulation modules --
   velocity over life, gravity, orbital velocity, curl noise, vector fields, drag, vortex,
   turbulence, attract/repel, color/size/rotation over life, and collision (depth-buffer and SDF) --
   compiled into a single compute dispatch per emitter.
   - **Rationale:** Fusing modules into one dispatch minimizes GPU overhead while giving artists
     full creative control.
   - **Verification:** Configure an emitter with every module active and verify correct per-particle
     output in a single dispatch. Validate depth-buffer and SDF collision with configurable
     restitution and friction.

## Rendering

3. **R-11.1.3** — The engine **SHALL** support three rendering modes: camera-facing sprite
   billboards with atlas flipbook and blend modes (additive, alpha, premultiplied) with
   soft-particle depth fade; spline-based ribbon geometry; and GPU-instanced mesh particles with
   full material support and distance-based LOD.
   - **Rationale:** Different effects require fundamentally different geometry strategies; all three
     cover the full range of VFX art needs.
   - **Verification:** Render emitters in each mode and confirm correct billboard orientation,
     ribbon connectivity, mesh instancing, blend modes, flipbook playback, and LOD.

4. **R-11.1.4** — The engine **SHALL** implement hierarchical LOD with at least four tiers (full
   sim, reduced spawn, billboard impostor, culled), hysteresis, a global budget manager capping
   alive particles by priority, and GPU radix sorting of transparent particles by camera distance.
   - **Rationale:** Unbounded particles cause frame drops; LOD and budget culling keep GPU cost
     predictable while prioritizing gameplay-critical effects.
   - **Verification:** Spawn emitters exceeding the budget and confirm lower-priority culled first.
     Verify LOD transitions without popping. Confirm sort correctness from multiple angles.

## Advanced Features

5. **R-11.1.5** — The engine **SHALL** support event-driven child emitter spawning from particle
   birth, death, collision, and manual trigger events, with sub-emitters inheriting parent transform
   and velocity.
   - **Rationale:** Cascading effects (fireworks, impact sparks) require child emitters reacting to
     parent lifecycle.
   - **Verification:** Configure sub-emitters for each event type and verify child particles spawn
     at correct parent position and velocity. Confirm cascading chains work.

6. **R-11.1.6** — The engine **SHALL** spawn dynamic point lights from particle positions into the
   clustered light buffer with a stochastic sampling strategy capping per-tile light evaluation.
   - **Rationale:** Emissive particles must illuminate geometry for consistency, but uncapped lights
     would overwhelm the lighting budget.
   - **Verification:** Emit light-producing particles and verify clustered light buffer entries.
     Confirm per-tile cap is enforced and cost stays bounded as count increases.

7. **R-11.1.7** — The engine **SHALL** provide a grid-based Eulerian fluid simulation on GPU for
   fire, smoke, and liquid via compute shaders, ray-marching the density field for volumetric
   rendering with emission, absorption, and scattering, scaling grid resolution with LOD distance.
   - **Rationale:** Volumetric fire and smoke require fluid dynamics that point particles cannot
     approximate; LOD-scaled grids keep cost viable in open worlds.
   - **Verification:** Run fluid sims for fire, smoke, and liquid. Confirm advection, emission,
     absorption, and scattering are plausible. Verify grid resolution decreases at greater LOD
     distances without visible artifacts.
