# R-4.8 — Fluid Simulation Requirements

## F-4.8.1 SPH Fluid Simulation

| ID       | Derived From                                          |
|----------|-------------------------------------------------------|
| R-4.8.1  | [F-4.8.1](../../features/physics/fluid-simulation.md) |
| R-4.8.1a | [F-4.8.1](../../features/physics/fluid-simulation.md) |

1. **R-4.8.1** — ECS SPH Fluid Entities: Each SPH fluid instance **SHALL** be an ECS entity with a
   `FluidVolume` component (solver type SPH, domain bounds, viscosity, surface tension) and a
   `FluidParticleBuffer` component storing particle positions and velocities as a GPU buffer
   resource. The `SPHSystem` **SHALL** evaluate density and pressure kernels on GPU compute.
   - **Rationale:** GPU-accelerated SPH is required for visually convincing fluid at interactive
     frame rates.
   - **Verification:** Initialize a 1000-particle SPH volume at rest density. Simulate 500 ticks
     under gravity. Assert no particle density deviates from rest by more than 5%.
2. **R-4.8.1a** — SPH Particle Budget: SPH simulation **SHALL** support up to 50,000 active
   particles per `FluidVolume` while completing its step within 4 ms on minimum-spec hardware (GPU
   compute).
   - **Rationale:** Visual fluid quality scales with particle density; the budget balances quality
     with frame-time.
   - **Verification:** Benchmark: simulate 50,000 particles. Measure GPU compute time. Assert
     completion within 4 ms.

## F-4.8.2 FLIP/PIC Hybrid Simulation

| ID      | Derived From                                          |
|---------|-------------------------------------------------------|
| R-4.8.2 | [F-4.8.2](../../features/physics/fluid-simulation.md) |

1. **R-4.8.2** — FLIP/PIC Hybrid Solver: The engine **SHALL** support FLIP/PIC fluid entities with
   `FluidVolume`, `FluidParticleBuffer`, and `FluidGrid` components. The `FLIPSystem` **SHALL**
   transfer particle velocities to the grid, run pressure projection, and update particles from
   corrected grid velocities.
   - **Rationale:** FLIP/PIC combines grid stability with particle detail preservation, enabling
     large-scale flooding and river flow.
   - **Verification:** Initialize FLIP/PIC fluid in a sealed box. Simulate 5 seconds with no
     external forces. Assert kinetic energy loss does not exceed 10% per second.

## F-4.8.3 Eulerian Grid Fluid Solver

| ID      | Derived From                                          |
|---------|-------------------------------------------------------|
| R-4.8.3 | [F-4.8.3](../../features/physics/fluid-simulation.md) |

1. **R-4.8.3** — Eulerian Grid Solver: The engine **SHALL** support Eulerian fluid entities with
   `FluidVolume` (solver type Eulerian) and `FluidGrid` components. The `EulerianSystem` **SHALL**
   compute velocity advection, pressure projection, and boundary enforcement on the grid, producing
   a divergence-free velocity field.
   - **Rationale:** Grid-based methods are ideal for bounded volumes (lakes, harbors) where stable
     pressure projection is more important than free-surface detail.
   - **Verification:** Initialize a 64x64x64 grid with a known velocity field. Run pressure
     projection. Assert residual divergence is below 1e-4.

## F-4.8.4 Fluid Surface Reconstruction

| ID       | Derived From                                          |
|----------|-------------------------------------------------------|
| R-4.8.4  | [F-4.8.4](../../features/physics/fluid-simulation.md) |
| R-4.8.4a | [F-4.8.4](../../features/physics/fluid-simulation.md) |

1. **R-4.8.4** — Watertight Surface Mesh Reconstruction: The `SurfaceReconstructionSystem` **SHALL**
   reconstruct a watertight triangle mesh from `FluidParticleBuffer` data using marching cubes or a
   screen-space approach and write it to a `FluidRenderer` component for the rendering pipeline. The
   mesh **SHALL** have no boundary edges and correct normals.
   - **Rationale:** Particles alone cannot be rendered as a continuous fluid surface; reconstruction
     bridges simulation to rendering.
   - **Verification:** Reconstruct a surface from 10,000 particles. Assert the mesh is watertight
     (no boundary edges) with correct normals.
2. **R-4.8.4a** — Surface Reconstruction Budget: Surface reconstruction **SHALL** complete within 4
   ms for 10,000 particles.
   - **Rationale:** Reconstruction runs every frame; it must fit within the rendering budget.
   - **Verification:** Benchmark: reconstruct from 10,000 particles. Assert completion within 4 ms.

## F-4.8.5 Water Surface Simulation

| ID       | Derived From                                          |
|----------|-------------------------------------------------------|
| R-4.8.5  | [F-4.8.5](../../features/physics/fluid-simulation.md) |
| R-4.8.5a | [F-4.8.5](../../features/physics/fluid-simulation.md) |

1. **R-4.8.5** — FFT and Gerstner Wave Synthesis: The engine **SHALL** support `WaterSurface`
   entities with `WaveConfig` components storing FFT parameters, Gerstner wave descriptors, and flow
   map references. The `WaterSurfaceSystem` **SHALL** evaluate wave synthesis and produce seamless
   tiling across streaming zones.
   - **Rationale:** Ocean and river surfaces need physically driven wave patterns that tile
     seamlessly across large areas.
   - **Verification:** Place two adjacent water surface tiles at a zone boundary. Assert maximum
     displacement difference is below 1 mm at the shared edge.
2. **R-4.8.5a** — Water Surface Evaluation Cost: Wave synthesis (FFT + Gerstner) **SHALL** evaluate
   within 0.5 ms per frame for a 1 km x 1 km water surface at the highest detail LOD.
   - **Rationale:** Water surfaces are visible in every outdoor frame; they must evaluate cheaply.
   - **Verification:** Benchmark: evaluate wave synthesis for 1 km x 1 km at full LOD. Assert GPU
     time within 0.5 ms.

## F-4.8.6 Buoyancy and Drag Forces

| ID      | Derived From                                          |
|---------|-------------------------------------------------------|
| R-4.8.6 | [F-4.8.6](../../features/physics/fluid-simulation.md) |

1. **R-4.8.6** — Automatic Buoyancy and Drag: The `BuoyancySystem` **SHALL** test all
   `(RigidBody, Collider, Transform)` entities against `FluidVolume` domains. For overlapping pairs,
   the system **SHALL** compute buoyancy from submerged volume approximation and drag from relative
   velocity, writing results as `ExternalForce` components. No special setup **SHALL** be required
   per entity.
   - **Rationale:** Automatic buoyancy from collider overlap means any rigid body can interact with
     water without designer intervention.
   - **Verification:** Place a body with density equal to the fluid. Simulate 5 seconds. Assert
     vertical acceleration is below 0.01 m/s^2 (neutral buoyancy).

## F-4.8.7 Two-Way Fluid-Rigid Body Coupling

| ID       | Derived From                                          |
|----------|-------------------------------------------------------|
| R-4.8.7  | [F-4.8.7](../../features/physics/fluid-simulation.md) |
| R-4.8.7a | [F-4.8.7](../../features/physics/fluid-simulation.md) |

1. **R-4.8.7** — Bidirectional Fluid-Rigid Body Interaction: Fluid simulation systems **SHALL** read
   `RigidBody`, `Velocity`, and `Collider` components on nearby entities to apply displacement
   forces into the fluid, while the `BuoyancySystem` writes `ExternalForce` components on rigid
   bodies to push them back, producing splashes and wakes.
   - **Rationale:** One-way coupling produces lifeless water; two-way coupling creates splashes and
     wakes that make water interactions feel physical.
   - **Verification:** Drop a 10 kg sphere into resting SPH at 5 m/s. Assert peak particle
     displacement is at least 10x rest spacing. Assert the sphere decelerates consistent with fluid
     reaction forces.
2. **R-4.8.7a** — Fluid Memory Budget: Total GPU memory for fluid simulation (particle buffers, grid
   data, surface reconstruction meshes) **SHALL NOT** exceed 128 MB across all active `FluidVolume`
   entities.
   - **Rationale:** Fluid competes with rendering, cloth, and other GPU workloads for VRAM; a fixed
     budget prevents fluid from starving other systems.
   - **Verification:** Create 4 active fluid volumes at max configured counts. Measure total GPU
     allocation. Assert it does not exceed 128 MB.

## Non-Functional Requirements

| ID        | Derived From                       |
|-----------|------------------------------------|
| R-4.8.NF1 | R-4.8.1                            |
| R-4.8.NF2 | R-4.8.1, R-4.8.2, R-4.8.3, R-4.8.4 |
| R-4.8.NF3 | R-4.8.5                            |

1. **R-4.8.NF1** — SPH Particle Budget: SPH simulation **SHALL** support up to 50,000 active
   particles per `FluidVolume` entity while completing its simulation step within 4 ms on
   minimum-spec hardware (GPU compute).
   - **Rationale:** Visually convincing fluid requires sufficient particle density; the budget must
     balance visual quality with frame-time constraints.
   - **Verification:** Benchmark -- simulate an SPH volume with 50,000 particles; measure GPU
     compute time per step; assert it completes within 4 ms.
2. **R-4.8.NF2** — Fluid Memory Budget: Total GPU memory consumption for fluid simulation (particle
   buffers, grid data, surface reconstruction meshes) **SHALL NOT** exceed 128 MB across all active
   `FluidVolume` entities in a scene.
   - **Rationale:** Fluid simulation competes with rendering, cloth, and other GPU workloads for
     VRAM; a fixed budget prevents fluid from starving other systems.
   - **Verification:** Profile -- create 4 active fluid volumes (2 SPH, 1 FLIP/PIC, 1 Eulerian) at
     maximum configured particle/grid counts; measure total GPU allocation; assert it does not
     exceed 128 MB.
3. **R-4.8.NF3** — Water Surface Evaluation Cost: Water surface wave synthesis (FFT + Gerstner)
   **SHALL** evaluate within 0.5 ms per frame for a water surface covering a 1 km x 1 km area at the
   highest detail LOD.
   - **Rationale:** Water surfaces are always visible in outdoor scenes and must evaluate cheaply to
     avoid competing with fluid particle simulation for GPU time.
   - **Verification:** Benchmark -- evaluate wave synthesis for a 1 km x 1 km water surface at full
     LOD; measure GPU compute time; assert it completes within 0.5 ms.
