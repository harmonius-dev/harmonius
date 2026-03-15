# R-4.8 — Fluid Simulation Requirements

## R-4.8.1 SPH Fluid Simulation

The engine **SHALL** simulate SPH fluid volumes as ECS entities with `FluidVolume` and
`FluidParticleBuffer` components, maintaining fluid incompressibility such that density
variation across particles remains below 5% of the rest density.

- **Derived from:** [F-4.8.1](../../features/physics/fluid-simulation.md)
- **Rationale:** SPH is the primary particle-based fluid method; incompressibility enforcement
  prevents unrealistic compression artifacts and volume loss.
- **Verification:** Unit test — initialize a 1,000-particle SPH volume at rest density;
  simulate 500 ticks under gravity in a closed container; measure per-particle density each
  frame; assert no particle's density deviates from rest density by more than 5%.

## R-4.8.2 FLIP/PIC Hybrid Simulation

The engine **SHALL** support FLIP/PIC hybrid fluid simulation on entities carrying
`FluidVolume` (solver type FLIP/PIC), `FluidParticleBuffer`, and `FluidGrid` components,
producing visually stable results with less than 10% kinetic energy loss per second compared
to the initial state in a closed system.

- **Derived from:** [F-4.8.2](../../features/physics/fluid-simulation.md)
- **Rationale:** FLIP/PIC combines grid stability with particle detail preservation, enabling
  large-scale flooding and river flow that pure SPH cannot handle efficiently.
- **Verification:** Unit test — initialize a FLIP/PIC fluid in a sealed box with known initial
  kinetic energy; simulate for 5 seconds with no external forces; measure total kinetic energy
  at each second; assert loss does not exceed 10% per second.

## R-4.8.3 Eulerian Grid Fluid Solver

The engine **SHALL** solve velocity advection and pressure projection on `FluidGrid` entities
with Eulerian solver type, producing a divergence-free velocity field with residual divergence
below 1e-4 after the pressure solve.

- **Derived from:** [F-4.8.3](../../features/physics/fluid-simulation.md)
- **Rationale:** Eulerian solvers are optimal for bounded water volumes (lakes, moats); a
  divergence-free velocity field is the physical correctness criterion for incompressible flow.
- **Verification:** Unit test — initialize a 64x64x64 Eulerian grid with a known velocity
  field; run one pressure projection step; measure divergence at each cell; assert maximum
  divergence is below 1e-4.

## R-4.8.4 Fluid Surface Reconstruction

The engine **SHALL** reconstruct a watertight triangle mesh from `FluidParticleBuffer` data
that can be rendered with refraction and reflection effects, completing surface reconstruction
within 4 ms for volumes of up to 10,000 particles.

- **Derived from:** [F-4.8.4](../../features/physics/fluid-simulation.md)
- **Rationale:** Rendering fluid requires a surface mesh; reconstruction must be fast enough
  to run every frame alongside simulation and rendering workloads.
- **Verification:** Benchmark — reconstruct a surface mesh from a 10,000-particle SPH volume;
  measure wall-clock time; assert it completes within 4 ms. Unit test — verify the output mesh
  is watertight (no boundary edges) and produces correct normals for rendering.

## R-4.8.5 Water Surface Simulation

The engine **SHALL** evaluate FFT-based and Gerstner wave synthesis on `WaterSurface` entities,
producing seamless tiling across streaming zone boundaries with less than 1 mm of visible seam
displacement at tile edges.

- **Derived from:** [F-4.8.5](../../features/physics/fluid-simulation.md)
- **Rationale:** Water surfaces span large open worlds; visible seams at zone or tile boundaries
  break immersion.
- **Verification:** Integration test — place two adjacent `WaterSurface` tiles at a streaming
  zone boundary; evaluate wave displacement at 100 points along the shared edge; assert the
  maximum displacement difference between matching points on adjacent tiles is below 1 mm.

## R-4.8.6 Buoyancy and Drag Forces

The engine **SHALL** compute buoyancy forces on submerged `RigidBody` entities such that a body
with density equal to the fluid density reaches neutral buoyancy (vertical acceleration below
0.01 m/s^2) within 5 seconds of simulation.

- **Derived from:** [F-4.8.6](../../features/physics/fluid-simulation.md)
- **Rationale:** Buoyancy and drag are required for boats, floating debris, and any object
  interacting with water volumes.
- **Verification:** Unit test — place a rigid body with density equal to the fluid rest density
  into a `FluidVolume`; simulate for 5 seconds; assert the body's vertical acceleration is
  below 0.01 m/s^2, indicating neutral buoyancy.

## R-4.8.7 Two-Way Fluid-Rigid Body Coupling

The engine **SHALL** displace fluid particles away from submerged `RigidBody` entities and
apply equal-and-opposite reaction forces, producing visible splash displacement when a rigid
body enters a fluid volume at speed.

- **Derived from:** [F-4.8.7](../../features/physics/fluid-simulation.md)
- **Rationale:** One-way buoyancy without fluid displacement produces unrealistic behavior;
  two-way coupling creates splashes, wakes, and current-carried debris.
- **Verification:** Integration test — drop a 10 kg sphere into a resting SPH fluid volume at
  5 m/s; measure peak particle displacement in the first 0.5 seconds; assert displacement is
  at least 10x the particle rest spacing. Verify the sphere's deceleration is consistent with
  fluid reaction forces (slower than freefall).

---

## Non-Functional Requirements

### R-4.8.NF1 SPH Particle Budget

SPH simulation **SHALL** support up to 50,000 active particles per `FluidVolume` entity
while completing its simulation step within 4 ms on minimum-spec hardware (GPU compute).

- **Derived from:** R-4.8.1
- **Rationale:** Visually convincing fluid requires sufficient particle density; the budget
  must balance visual quality with frame-time constraints.
- **Verification:** Benchmark — simulate an SPH volume with 50,000 particles; measure GPU
  compute time per step; assert it completes within 4 ms.

### R-4.8.NF2 Fluid Memory Budget

Total GPU memory consumption for fluid simulation (particle buffers, grid data, surface
reconstruction meshes) **SHALL NOT** exceed 128 MB across all active `FluidVolume` entities
in a scene.

- **Derived from:** R-4.8.1, R-4.8.2, R-4.8.3, R-4.8.4
- **Rationale:** Fluid simulation competes with rendering, cloth, and other GPU workloads for
  VRAM; a fixed budget prevents fluid from starving other systems.
- **Verification:** Profile — create 4 active fluid volumes (2 SPH, 1 FLIP/PIC, 1 Eulerian)
  at maximum configured particle/grid counts; measure total GPU allocation; assert it does
  not exceed 128 MB.

### R-4.8.NF3 Water Surface Evaluation Cost

Water surface wave synthesis (FFT + Gerstner) **SHALL** evaluate within 0.5 ms per frame
for a water surface covering a 1 km x 1 km area at the highest detail LOD.

- **Derived from:** R-4.8.5
- **Rationale:** Water surfaces are always visible in outdoor scenes and must evaluate cheaply
  to avoid competing with fluid particle simulation for GPU time.
- **Verification:** Benchmark — evaluate wave synthesis for a 1 km x 1 km water surface at
  full LOD; measure GPU compute time; assert it completes within 0.5 ms.
