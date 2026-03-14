# 4.8 — Fluid Simulation

## Particle-Based Methods

### F-4.8.1 SPH Fluid Simulation

Each SPH fluid instance is an entity with a `FluidVolume` component (solver type set to SPH,
domain bounds, viscosity, surface tension) and a `FluidParticleBuffer` component that stores
particle positions and velocities as a GPU buffer resource. The `SPHSystem` queries all
`(FluidVolume, FluidParticleBuffer, Transform)` entities where the solver type is SPH,
evaluates density and pressure kernels, and writes updated particle state back to the
`FluidParticleBuffer`. Particle counts are bounded per `FluidVolume` to maintain frame budgets.

- **Requirements:** R-4.8.1
- **Dependencies:** F-1.1.1, F-1.1.2
- **Platform notes:** None

### F-4.8.2 FLIP/PIC Hybrid Simulation

Entities requiring FLIP/PIC simulation carry a `FluidVolume` component (solver type set to
FLIP/PIC), a `FluidParticleBuffer` for particle data, and a `FluidGrid` component for the
background pressure-projection grid. The `FLIPSystem` queries all entities with this component
triple, transfers particle velocities to the grid, runs pressure projection on the `FluidGrid`,
then updates `FluidParticleBuffer` particles from the corrected grid velocities. This combines
grid stability with particle detail preservation for large-scale flooding and river flow.

- **Requirements:** R-4.8.2
- **Dependencies:** F-4.8.1, F-1.1.1
- **Platform notes:** None

## Grid-Based Methods

### F-4.8.3 Eulerian Grid Fluid Solver

Bounded water volumes (lakes, harbors, moats) are entities with a `FluidVolume` component
(solver type set to Eulerian) and a `FluidGrid` component storing velocity, pressure, and
boundary condition data on a uniform or adaptive grid. The `EulerianSystem` queries all
`(FluidVolume, FluidGrid, Transform)` entities where the solver type is Eulerian and computes
velocity advection, pressure projection, and boundary enforcement. Grid resolution is
configured per entity on the `FluidGrid` component.

- **Requirements:** R-4.8.3
- **Dependencies:** F-1.1.1, F-1.1.2
- **Platform notes:** None

## Surface and Rendering Integration

### F-4.8.4 Fluid Surface Reconstruction

The `SurfaceReconstructionSystem` queries all entities with a `FluidParticleBuffer` component
and reconstructs a renderable triangle mesh using marching cubes or a screen-space approach.
Reconstructed mesh data is written to a `FluidRenderer` component on the same entity, which
bridges fluid data to the rendering pipeline for refraction, reflection, and foam effects.
Surface reconstruction runs at interactive rates and produces watertight meshes with smooth
normals.

- **Requirements:** R-4.8.4
- **Dependencies:** F-4.8.1, F-1.1.1
- **Platform notes:** None

### F-4.8.5 Water Surface Simulation

Ocean, river, and lake entities carry a `WaterSurface` component with a `WaveConfig` that
stores FFT parameters, Gerstner wave descriptors, and flow map references. The
`WaterSurfaceSystem` queries all `(WaterSurface, Transform)` entities and evaluates wave
synthesis plus localized displacement from nearby `FluidVolume` entities. The `WaterSurface`
component produces seamless tiling across streaming zones and supports river flow fields and
shoreline wave breaking through the `WaveConfig` parameters.

- **Requirements:** R-4.8.5
- **Dependencies:** F-4.8.3, F-1.1.1
- **Platform notes:** None

## Rigid Body Interaction

### F-4.8.6 Buoyancy and Drag Forces

The `BuoyancySystem` queries all `(RigidBody, Collider, Transform)` entities and tests their
bounding shapes against every `FluidVolume` entity's domain. For overlapping pairs, the system
computes buoyancy from submerged volume approximation using sample points on the `Collider`,
and drag from the entity's `Velocity` relative to the fluid. Resulting forces are written as
`ExternalForce` components on the rigid body entities. Two-way coupling is achieved by the
fluid systems reading `RigidBody` and `Velocity` components on nearby entities.

- **Requirements:** R-4.8.6
- **Dependencies:** F-4.8.5, F-4.1.1, F-1.1.1
- **Platform notes:** None

### F-4.8.7 Two-Way Fluid-Rigid Body Coupling

Fluid simulation systems (`SPHSystem`, `FLIPSystem`, `EulerianSystem`) read `RigidBody`,
`Velocity`, and `Collider` components on nearby entities to apply displacement forces from
submerged bodies into the fluid. The `BuoyancySystem` writes `ExternalForce` components on
rigid body entities to push them back. Object splashes, ship wakes, and debris carried by
currents emerge from this bidirectional ECS query pattern. A `FluidRenderer` component on
each fluid entity bridges simulation state to the rendering pipeline for visual feedback.
Coupling degrades gracefully when simulation budget is exceeded by reducing query radius.

- **Requirements:** R-4.8.7
- **Dependencies:** F-4.8.6, F-4.8.1, F-1.1.1
- **Platform notes:** None
