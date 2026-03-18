# R-11.1 — Particle System Requirements

## R-11.1.1–R-11.1.7 Particle System

| ID       | Derived From                                |
|----------|---------------------------------------------|
| R-11.1.1 | [F-11.1.1](../../features/vfx/particles.md) |
| R-11.1.2 | [F-11.1.2](../../features/vfx/particles.md) |
| R-11.1.3 | [F-11.1.3](../../features/vfx/particles.md) |
| R-11.1.4 | [F-11.1.4](../../features/vfx/particles.md) |
| R-11.1.5 | [F-11.1.5](../../features/vfx/particles.md) |
| R-11.1.6 | [F-11.1.6](../../features/vfx/particles.md) |
| R-11.1.7 | [F-11.1.7](../../features/vfx/particles.md) |

1. **R-11.1.1** — The engine **SHALL** simulate particles entirely on the GPU using compute shaders,
   supporting at least point, sphere, box, cone, mesh surface, and skinned mesh spawn shapes, with
   persistent GPU buffer allocation, free-list management, and indirect dispatch for variable
   particle counts.
   - **Rationale:** GPU-driven simulation eliminates CPU bottlenecks, enabling millions of particles
     per frame and allowing skinned-mesh emission without CPU readback.
   - **Verification:** Spawn particles from each supported shape type and confirm all simulation
     runs on compute shaders with zero CPU readback; measure throughput exceeds one million
     particles at 60 fps on reference hardware.
2. **R-11.1.2** — The engine **SHALL** provide composable per-particle simulation modules —
   including velocity over life, gravity, orbital velocity, curl noise, vector fields, drag, vortex,
   turbulence, attract/repel, color over life, size over life, rotation over life, and collision
   (depth-buffer and SDF modes) — compiled into a single compute dispatch per emitter.
   - **Rationale:** Fusing modules into one dispatch minimizes GPU overhead while giving artists
     full creative control over particle behavior.
   - **Verification:** Configure an emitter with every listed module active and verify each module
     produces correct per-particle output in a single dispatch; validate depth-buffer and SDF
     collision with configurable restitution and friction values.
3. **R-11.1.3** — The engine **SHALL** support three particle rendering modes: camera-facing sprite
   billboards with atlas flipbook animation and blend modes (additive, alpha, premultiplied) with
   soft-particle depth fade; spline-based ribbon geometry connecting sequential particles; and
   GPU-instanced mesh particles with full material support, per-particle transforms, and
   distance-based LOD selection.
   - **Rationale:** Different visual effects require fundamentally different geometry generation
     strategies; supporting all three covers the full range of particle art needs.
   - **Verification:** Render test emitters in each mode and confirm correct billboard orientation,
     ribbon connectivity, and mesh instancing; validate blend modes, flipbook playback, soft-depth
     fade, and mesh LOD transitions.
4. **R-11.1.4** — The engine **SHALL** implement hierarchical LOD for particle emitters with at
   least four tiers (full simulation, reduced spawn rate, billboard impostor, culled), hysteresis to
   prevent popping, a global budget manager that caps total alive particles by priority, and GPU
   radix sorting of transparent particles by camera distance with per-emitter sort mode selection.
   - **Rationale:** Unbounded particle counts cause frame drops; tiered LOD and budget culling keep
     GPU cost predictable while prioritizing gameplay-critical effects.
   - **Verification:** Spawn emitters exceeding the global budget and confirm lower-priority
     emitters are culled first; verify LOD tier transitions occur at configured distances without
     visible popping; confirm transparent particles are sorted correctly from multiple camera
     angles.
5. **R-11.1.5** — The engine **SHALL** support event-driven child emitter spawning triggered by
   particle birth, death, collision, and manual trigger events, with sub-emitters inheriting parent
   particle transform and velocity.
   - **Rationale:** Cascading effects such as fireworks and impact sparks require child emitters
     that react to parent particle lifecycle without monolithic system authoring.
   - **Verification:** Configure sub-emitters for each event type (birth, death, collision, manual
     trigger) and verify child particles spawn at the correct parent position and velocity; confirm
     cascading chains of sub-emitters function correctly.
6. **R-11.1.6** — The engine **SHALL** spawn dynamic point lights from particle positions,
   contributing color, intensity, and attenuation radius to the clustered light buffer, with a
   stochastic sampling strategy that caps the number of particle lights evaluated per tile.
   - **Rationale:** Emissive particles must illuminate surrounding geometry for visual consistency,
     but uncapped particle lights would overwhelm the lighting budget during dense combat.
   - **Verification:** Emit light-producing particles and confirm they appear in the clustered light
     buffer with correct color and attenuation; verify per-tile light cap is enforced and lighting
     cost remains bounded as particle count increases.
7. **R-11.1.7** — The engine **SHALL** provide a grid-based Eulerian fluid simulation on the GPU for
   fire, smoke, and liquid surface effects, advecting a 3D velocity/density/temperature grid via
   compute shaders, ray-marching the density field for volumetric rendering with emission,
   absorption, and scattering, and scaling grid resolution with LOD distance.
   - **Rationale:** Volumetric fire, smoke, and liquid effects require fluid dynamics that cannot be
     approximated by point particles alone; LOD-scaled grids keep cost viable in open worlds.
   - **Verification:** Run fluid simulations for fire, smoke, and liquid and confirm advection,
     emission, absorption, and scattering produce physically plausible results; verify grid
     resolution decreases at greater LOD distances without visible artifacts.
