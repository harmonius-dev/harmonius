# R-4.8 — Fluid Simulation Requirements

## F-4.8.1 SPH Fluid Simulation

### R-4.8.1 ECS SPH Fluid Entities

Each SPH fluid instance **SHALL** be an ECS entity with a `FluidVolume` component (solver type SPH,
domain bounds, viscosity, surface tension) and a `FluidParticleBuffer` component storing particle
positions and velocities as a GPU buffer resource. The `SPHSystem` **SHALL** evaluate density and
pressure kernels on GPU compute.

- **Derived from:**
  [F-4.8.1](../../features/physics/fluid-simulation.md)
- **Rationale:** GPU-accelerated SPH is required for visually convincing fluid at interactive frame
  rates.
- **Verification:** Initialize a 1000-particle SPH volume at rest density. Simulate 500 ticks under
  gravity. Assert no particle density deviates from rest by more than 5%.

### R-4.8.1a SPH Particle Budget

SPH simulation **SHALL** support up to 50,000 active particles per `FluidVolume` while completing
its step within 4 ms on minimum-spec hardware (GPU compute).

- **Derived from:**
  [F-4.8.1](../../features/physics/fluid-simulation.md)
- **Rationale:** Visual fluid quality scales with particle density; the budget balances quality with
  frame-time.
- **Verification:** Benchmark: simulate 50,000 particles. Measure GPU compute time. Assert
  completion within 4 ms.

---

## US-4.8.1.1 Set Up SPH Fluid Volumes

**As a** game developer (P-15), **I want to** create entities with `FluidVolume` (solver type SPH)
and `FluidParticleBuffer` components, **so that** particle-based fluid instances are standard ECS
entities.

## US-4.8.1.2 Configure SPH Parameters in Editor

**As a** designer (P-5), **I want to** set domain bounds, viscosity, surface tension, and particle
count per fluid volume in the editor, **so that** fluid behavior is tuned visually.

## US-4.8.1.3 Configure SPH Particle Budget Per Platform

**As a** designer (P-5), **I want to** set platform-specific particle limits (1K Switch, 256K
high-end PC) in the build settings, **so that** fluid complexity matches platform capability.

## US-4.8.1.4 Verify SPH Incompressibility

**As an** engine tester (P-27), **I want to** initialize a 1000-particle SPH volume at rest density,
simulate 500 ticks under gravity, and assert no particle density deviates from rest by more than 5%,
**so that** incompressibility enforcement is confirmed.

## US-4.8.1.5 Benchmark SPH Performance

**As an** engine tester (P-27), **I want to** simulate a 50000-particle SPH volume and assert GPU
compute time is within 4ms, **so that** SPH performance meets the particle budget.

## US-4.8.1.6 Implement SPH Solver System

**As an** engine developer (P-26), **I want to** implement `SPHSystem` that evaluates density and
pressure kernels on GPU buffers and writes updated particle state, **so that** SPH simulation is
GPU-accelerated.

## US-4.8.1.7 Experience Liquid Flowing and Splashing

**As a** player (P-23), **I want** water and other liquids to flow, splash, and pool realistically,
**so that** fluid interactions are visually impressive.

---

## F-4.8.2 FLIP/PIC Hybrid Simulation

### R-4.8.2 FLIP/PIC Hybrid Solver

The engine **SHALL** support FLIP/PIC fluid entities with `FluidVolume`, `FluidParticleBuffer`, and
`FluidGrid` components. The `FLIPSystem` **SHALL** transfer particle velocities to the grid, run
pressure projection, and update particles from corrected grid velocities.

- **Derived from:**
  [F-4.8.2](../../features/physics/fluid-simulation.md)
- **Rationale:** FLIP/PIC combines grid stability with particle detail preservation, enabling
  large-scale flooding and river flow.
- **Verification:** Initialize FLIP/PIC fluid in a sealed box. Simulate 5 seconds with no external
  forces. Assert kinetic energy loss does not exceed 10% per second.

---

## US-4.8.2.1 Set Up FLIP/PIC Fluid Volumes

**As a** game developer (P-15), **I want to** create entities with `FluidVolume` (solver type
FLIP/PIC), `FluidParticleBuffer`, and `FluidGrid` components, **so that** hybrid fluid simulation
uses both particles and grids.

## US-4.8.2.2 Configure FLIP/PIC Grid Resolution in Editor

**As a** designer (P-5), **I want to** set grid resolution and particle count per FLIP/PIC volume in
the editor, **so that** simulation quality versus performance is balanced.

## US-4.8.2.3 Verify Energy Conservation

**As an** engine tester (P-27), **I want to** initialize a FLIP/PIC fluid in a sealed box, simulate
5 seconds with no external forces, and assert kinetic energy loss does not exceed 10% per second,
**so that** energy conservation is within tolerance.

## US-4.8.2.4 Implement FLIP/PIC System

**As an** engine developer (P-26), **I want to** implement `FLIPSystem` that transfers particle
velocities to the grid, runs pressure projection, and updates particles from the corrected grid,
**so that** FLIP/PIC combines grid stability with particle detail.

## US-4.8.2.5 Experience Large-Scale Flooding

**As a** player (P-23), **I want** flooding events and river flow to look convincing with volume and
momentum, **so that** large-scale water scenarios are immersive.

---

## F-4.8.3 Eulerian Grid Fluid Solver

### R-4.8.3 Eulerian Grid Solver

The engine **SHALL** support Eulerian fluid entities with `FluidVolume` (solver type Eulerian) and
`FluidGrid` components. The `EulerianSystem` **SHALL** compute velocity advection, pressure
projection, and boundary enforcement on the grid, producing a divergence-free velocity field.

- **Derived from:**
  [F-4.8.3](../../features/physics/fluid-simulation.md)
- **Rationale:** Grid-based methods are ideal for bounded volumes (lakes, harbors) where stable
  pressure projection is more important than free-surface detail.
- **Verification:** Initialize a 64x64x64 grid with a known velocity field. Run pressure projection.
  Assert residual divergence is below 1e-4.

---

## US-4.8.3.1 Set Up Eulerian Fluid Volumes

**As a** game developer (P-15), **I want to** create entities with `FluidVolume` (solver type
Eulerian) and `FluidGrid` components for bounded water volumes, **so that** lakes and moats use
grid-based simulation.

## US-4.8.3.2 Configure Grid Resolution in Editor

**As a** designer (P-5), **I want to** set grid resolution per Eulerian volume in the editor,
**so that** simulation quality matches the scene's visual importance.

## US-4.8.3.3 Verify Divergence-Free Velocity Field

**As an** engine tester (P-27), **I want to** initialize a 64x64x64 grid with a known velocity
field, run pressure projection, and assert residual divergence is below 1e-4, **so that**
incompressible flow correctness is confirmed.

## US-4.8.3.4 Implement Eulerian Solver System

**As an** engine developer (P-26), **I want to** implement `EulerianSystem` that computes velocity
advection, pressure projection, and boundary enforcement on `FluidGrid` components, **so that**
grid-based fluid simulation is available.

## US-4.8.3.5 Experience Calm Lake and Moat Water

**As a** player (P-23), **I want** lakes and moats to have gentle surface motion with realistic wave
patterns, **so that** bounded water looks natural.

## US-4.8.3.6 Place Bounded Water Volumes in Levels

**As a** level designer (P-6), **I want to** place lake and moat water volumes in the editor with
configurable boundaries, **so that** bounded water is part of level design.

---

## F-4.8.4 Fluid Surface Reconstruction

### R-4.8.4 Watertight Surface Mesh Reconstruction

The `SurfaceReconstructionSystem` **SHALL** reconstruct a watertight triangle mesh from
`FluidParticleBuffer` data using marching cubes or a screen-space approach and write it to a
`FluidRenderer` component for the rendering pipeline. The mesh **SHALL** have no boundary edges and
correct normals.

- **Derived from:**
  [F-4.8.4](../../features/physics/fluid-simulation.md)
- **Rationale:** Particles alone cannot be rendered as a continuous fluid surface; reconstruction
  bridges simulation to rendering.
- **Verification:** Reconstruct a surface from 10,000 particles. Assert the mesh is watertight (no
  boundary edges) with correct normals.

### R-4.8.4a Surface Reconstruction Budget

Surface reconstruction **SHALL** complete within 4 ms for 10,000 particles.

- **Derived from:**
  [F-4.8.4](../../features/physics/fluid-simulation.md)
- **Rationale:** Reconstruction runs every frame; it must fit within the rendering budget.
- **Verification:** Benchmark: reconstruct from 10,000 particles. Assert completion within 4 ms.

---

## US-4.8.4.1 Set Up Fluid Rendering Integration

**As a** game developer (P-15), **I want** the `SurfaceReconstructionSystem` to write a renderable
mesh to a `FluidRenderer` component, **so that** fluid data bridges to the rendering pipeline.

## US-4.8.4.2 Verify Watertight Surface Mesh

**As an** engine tester (P-27), **I want to** reconstruct a surface mesh from a 10000-particle SPH
volume and assert it is watertight (no boundary edges) with correct normals, **so that** mesh
quality is confirmed.

## US-4.8.4.3 Benchmark Surface Reconstruction Performance

**As an** engine tester (P-27), **I want to** reconstruct a surface from 10000 particles and assert
completion within 4ms, **so that** reconstruction fits within the frame budget.

## US-4.8.4.4 Implement Surface Reconstruction System

**As an** engine developer (P-26), **I want to** implement `SurfaceReconstructionSystem` using
marching cubes or screen-space approach to produce watertight meshes with smooth normals,
**so that** fluid particles can be rendered as a continuous surface.

## US-4.8.4.5 Experience Beautiful Water Surfaces

**As a** player (P-23), **I want** fluid to have smooth, reflective surfaces with refraction and
foam effects, **so that** water looks visually stunning.

---

## F-4.8.5 Water Surface Simulation

### R-4.8.5 FFT and Gerstner Wave Synthesis

The engine **SHALL** support `WaterSurface` entities with `WaveConfig` components storing FFT
parameters, Gerstner wave descriptors, and flow map references. The `WaterSurfaceSystem` **SHALL**
evaluate wave synthesis and produce seamless tiling across streaming zones.

- **Derived from:**
  [F-4.8.5](../../features/physics/fluid-simulation.md)
- **Rationale:** Ocean and river surfaces need physically driven wave patterns that tile seamlessly
  across large areas.
- **Verification:** Place two adjacent water surface tiles at a zone boundary. Assert maximum
  displacement difference is below 1 mm at the shared edge.

### R-4.8.5a Water Surface Evaluation Cost

Wave synthesis (FFT + Gerstner) **SHALL** evaluate within 0.5 ms per frame for a 1 km x 1 km water
surface at the highest detail LOD.

- **Derived from:**
  [F-4.8.5](../../features/physics/fluid-simulation.md)
- **Rationale:** Water surfaces are visible in every outdoor frame; they must evaluate cheaply.
- **Verification:** Benchmark: evaluate wave synthesis for 1 km x 1 km at full LOD. Assert GPU time
  within 0.5 ms.

---

## US-4.8.5.1 Set Up Water Surface Entities

**As a** game developer (P-15), **I want to** create entities with `WaterSurface` and `WaveConfig`
components storing FFT parameters and Gerstner wave descriptors, **so that** ocean and river
surfaces are ECS entities.

## US-4.8.5.2 Configure Wave Parameters in Editor

**As a** designer (P-5), **I want to** set wave height, frequency, wind direction, and flow map
parameters in the editor, **so that** water surface appearance is tuned visually.

## US-4.8.5.3 Verify Seamless Tiling Across Zones

**As an** engine tester (P-27), **I want to** place two adjacent water surface tiles at a zone
boundary and assert maximum displacement difference is below 1mm at the shared edge, **so that**
seamless tiling is confirmed.

## US-4.8.5.4 Benchmark Water Surface Evaluation

**As an** engine tester (P-27), **I want to** evaluate wave synthesis for a 1km x 1km water surface
and assert GPU time is within 0.5ms, **so that** water surface cost meets the budget.

## US-4.8.5.5 Implement Water Surface System

**As an** engine developer (P-26), **I want to** implement `WaterSurfaceSystem` that evaluates FFT
and Gerstner wave synthesis with flow maps and shoreline breaking, **so that** water surfaces are
physically driven.

## US-4.8.5.6 Experience Ocean Waves and River Currents

**As a** player (P-23), **I want** oceans to have rolling waves and rivers to have visible currents,
**so that** water environments feel alive and immersive.

## US-4.8.5.7 Place Ocean and River Surfaces in Levels

**As a** level designer (P-6), **I want to** place water surface entities for oceans, rivers, and
lakes in the editor with configurable wave and flow parameters, **so that** water is part of the
level design.

---

## F-4.8.6 Buoyancy and Drag Forces

### R-4.8.6 Automatic Buoyancy and Drag

The `BuoyancySystem` **SHALL** test all `(RigidBody, Collider, Transform)` entities against
`FluidVolume` domains. For overlapping pairs, the system **SHALL** compute buoyancy from submerged
volume approximation and drag from relative velocity, writing results as `ExternalForce` components.
No special setup **SHALL** be required per entity.

- **Derived from:**
  [F-4.8.6](../../features/physics/fluid-simulation.md)
- **Rationale:** Automatic buoyancy from collider overlap means any rigid body can interact with
  water without designer intervention.
- **Verification:** Place a body with density equal to the fluid. Simulate 5 seconds. Assert
  vertical acceleration is below 0.01 m/s^2 (neutral buoyancy).

---

## US-4.8.6.1 Set Up Buoyant Rigid Bodies

**As a** game developer (P-15), **I want** any `RigidBody` entity with a `Collider` to automatically
receive buoyancy and drag forces when overlapping a `FluidVolume`, **so that** water interaction
requires no special setup.

## US-4.8.6.2 Configure Buoyancy Sample Points

**As a** designer (P-5), **I want to** set the number of buoyancy sample points per collider in the
editor, **so that** buoyancy precision versus cost is balanced.

## US-4.8.6.3 Verify Neutral Buoyancy

**As an** engine tester (P-27), **I want to** place a body with density equal to the fluid in a
volume, simulate 5 seconds, and assert vertical acceleration is below 0.01 m/s^2, **so that**
neutral buoyancy is achieved correctly.

## US-4.8.6.4 Benchmark Buoyancy Cost

**As an** engine tester (P-27), **I want to** simulate 64 buoyant bodies on desktop and assert the
system fits within the physics frame budget, **so that** buoyancy scales to the platform limit.

## US-4.8.6.5 Implement Buoyancy System

**As an** engine developer (P-26), **I want to** implement `BuoyancySystem` that tests rigid body
colliders against fluid volume domains, computes buoyancy from submerged volume and drag from
relative velocity, and writes `ExternalForce` components, **so that** water-body interaction is
automated.

## US-4.8.6.6 Experience Objects Floating and Sinking

**As a** player (P-23), **I want** barrels to float and metal crates to sink, **so that** objects
interact with water based on their density.

---

## F-4.8.7 Two-Way Fluid-Rigid Body Coupling

### R-4.8.7 Bidirectional Fluid-Rigid Body Interaction

Fluid simulation systems **SHALL** read `RigidBody`, `Velocity`, and `Collider` components on nearby
entities to apply displacement forces into the fluid, while the `BuoyancySystem` writes
`ExternalForce` components on rigid bodies to push them back, producing splashes and wakes.

- **Derived from:**
  [F-4.8.7](../../features/physics/fluid-simulation.md)
- **Rationale:** One-way coupling produces lifeless water; two-way coupling creates splashes and
  wakes that make water interactions feel physical.
- **Verification:** Drop a 10 kg sphere into resting SPH at 5 m/s. Assert peak particle displacement
  is at least 10x rest spacing. Assert the sphere decelerates consistent with fluid reaction forces.

### R-4.8.7a Fluid Memory Budget

Total GPU memory for fluid simulation (particle buffers, grid data, surface reconstruction meshes)
**SHALL NOT** exceed 128 MB across all active `FluidVolume` entities.

- **Derived from:**
  [F-4.8.7](../../features/physics/fluid-simulation.md)
- **Rationale:** Fluid competes with rendering, cloth, and other GPU workloads for VRAM; a fixed
  budget prevents fluid from starving other systems.
- **Verification:** Create 4 active fluid volumes at max configured counts. Measure total GPU
  allocation. Assert it does not exceed 128 MB.

---

## US-4.8.7.1 Set Up Two-Way Coupling

**As a** game developer (P-15), **I want** fluid simulation systems to read rigid body components
and apply displacement forces into the fluid, while the buoyancy system pushes back, **so that**
two-way coupling produces splashes and wakes.

## US-4.8.7.2 Verify Splash Displacement

**As an** engine tester (P-27), **I want to** drop a 10 kg sphere into a resting SPH volume at 5 m/s
and assert peak particle displacement is at least 10x rest spacing, **so that** splash magnitude is
physically proportional.

## US-4.8.7.3 Verify Rigid Body Deceleration in Fluid

**As an** engine tester (P-27), **I want to** verify the sphere's deceleration is consistent with
fluid reaction forces (slower than freefall), **so that** two-way coupling slows objects in water.

## US-4.8.7.4 Implement Two-Way Fluid-Rigid Coupling

**As an** engine developer (P-26), **I want to** implement displacement force injection from
submerged rigid bodies into the fluid particle/grid systems, **so that** object entry creates
splashes and wakes.

## US-4.8.7.5 Experience Splashes When Objects Hit Water

**As a** player (P-23), **I want** objects dropping into water to create splashes and ripples,
**so that** water interactions look physically realistic.

## US-4.8.7.6 Experience Ship Wakes

**As a** player (P-23), **I want** boats moving through water to leave visible wake trails,
**so that** naval travel feels impactful.

---

## Non-Functional Requirements

### R-4.8.NF1 SPH Particle Budget

SPH simulation **SHALL** support up to 50,000 active particles per `FluidVolume` entity while
completing its simulation step within 4 ms on minimum-spec hardware (GPU compute).

- **Derived from:** R-4.8.1
- **Rationale:** Visually convincing fluid requires sufficient particle density; the budget must
  balance visual quality with frame-time constraints.
- **Verification:** Benchmark -- simulate an SPH volume with 50,000 particles; measure GPU compute
  time per step; assert it completes within 4 ms.

### R-4.8.NF2 Fluid Memory Budget

Total GPU memory consumption for fluid simulation (particle buffers, grid data, surface
reconstruction meshes) **SHALL NOT** exceed 128 MB across all active `FluidVolume` entities in a
scene.

- **Derived from:** R-4.8.1, R-4.8.2, R-4.8.3, R-4.8.4
- **Rationale:** Fluid simulation competes with rendering, cloth, and other GPU workloads for VRAM;
  a fixed budget prevents fluid from starving other systems.
- **Verification:** Profile -- create 4 active fluid volumes (2 SPH, 1 FLIP/PIC, 1 Eulerian) at
  maximum configured particle/grid counts; measure total GPU allocation; assert it does not exceed
  128 MB.

### R-4.8.NF3 Water Surface Evaluation Cost

Water surface wave synthesis (FFT + Gerstner) **SHALL** evaluate within 0.5 ms per frame for a water
surface covering a 1 km x 1 km area at the highest detail LOD.

- **Derived from:** R-4.8.5
- **Rationale:** Water surfaces are always visible in outdoor scenes and must evaluate cheaply to
  avoid competing with fluid particle simulation for GPU time.
- **Verification:** Benchmark -- evaluate wave synthesis for a 1 km x 1 km water surface at full
  LOD; measure GPU compute time; assert it completes within 0.5 ms.
