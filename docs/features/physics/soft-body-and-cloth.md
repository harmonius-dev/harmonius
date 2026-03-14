# 4.7 — Soft Body & Cloth

## Simulation Core

### F-4.7.1 Position-Based Dynamics Solver

The `XpbdSolverSystem` queries all entities with `(ClothSimulation, Transform)` components
and runs Extended Position-Based Dynamics constraint resolution each physics tick. The
`ClothSimulation` component references GPU buffer handles for particle positions, velocities,
and constraint descriptors (distance, bending, volume preservation, shape-matching). Constraint
compliance is configured per-entity, enabling stiffness-independent solving without a separate
physics world.

- **Requirements:** R-4.7.1
- **Dependencies:** F-1.1.1, F-1.1.2
- **Platform notes:** None

### F-4.7.2 Cloth Simulation

Each cloth instance is an entity with a `ClothSimulation` component that stores particle and
constraint buffer handles, material parameters (stretch, shear, bend resistance), and mesh
topology. A `ClothAttachment` component links cloth particles to skeleton bone entities via
ECS entity references, enabling capes, banners, and sails to follow animated characters. The
`ClothSimulationSystem` steps the particle-constraint mesh each tick by reading
`ClothSimulation` and writing updated particle positions back to the same component's buffers.

- **Requirements:** R-4.7.2
- **Dependencies:** F-4.7.1, F-1.1.1
- **Platform notes:** None

## Collision and Interaction

### F-4.7.3 Cloth Self-Collision

Entities with both `ClothSimulation` and a `SelfCollisionEnabled` marker component participate
in self-collision detection. The `ClothSelfCollisionSystem` queries all
`(ClothSimulation, SelfCollisionEnabled)` entities and runs spatial hashing or BVH-based
continuous collision detection on their particles. Configurable thickness and skip-distance
parameters live on the `SelfCollisionEnabled` component to manage per-instance budgets.

- **Requirements:** R-4.7.3
- **Dependencies:** F-4.7.2, F-1.1.1
- **Platform notes:** None

### F-4.7.4 Two-Way Rigid Body Coupling

The `XpbdSolverSystem` reads `Collider` and `Transform` components on nearby rigid body
entities to generate contact constraints against cloth particles. Reaction forces are written
as `ExternalForce` components on the rigid body entities, enabling two-way coupling where
cloth drapes over objects and rigid bodies deflect from soft body contact. All interaction
is resolved through ECS component queries with no separate collision world.

- **Requirements:** R-4.7.4
- **Dependencies:** F-4.7.1, F-4.1.3, F-1.1.1
- **Platform notes:** None

## Environmental Effects

### F-4.7.5 Wind Interaction

Wind sources are entities with a `WindSource` component specifying type (directional, point,
vortex), strength, and turbulence noise parameters. The `ClothWindSystem` queries all
`WindSource` entities to accumulate forces, then applies the resultant wind force to every
`(ClothSimulation, Transform)` entity within range. This brings banners, flags, and sails
to life through pure ECS system scheduling with no global wind field singleton.

- **Requirements:** R-4.7.5
- **Dependencies:** F-4.7.2, F-1.1.1
- **Platform notes:** None

### F-4.7.6 Cloth Tearing

The `ClothTearingSystem` queries all `ClothSimulation` entities each tick and checks constraint
strain against a configurable threshold stored on the component. When strain exceeds the limit,
the system splits the `ClothSimulation` mesh topology, updates constraint buffers, and spawns
new cloth entities for separated patches. Torn edges generate proper boundary normals for
rendering, enabling destructible sails and battle-damaged banners.

- **Requirements:** R-4.7.6
- **Dependencies:** F-4.7.2, F-1.1.1
- **Platform notes:** None

## Performance

### F-4.7.7 Cloth Level of Detail

A `ClothLod` component on cloth entities controls simulation fidelity parameters: particle
count, constraint iterations, and update frequency. The `ClothLodSystem` queries the active
camera entity's `Transform` and computes distance to each `(ClothSimulation, ClothLod,
Transform)` entity, then adjusts the `ClothLod` tier accordingly. At extreme distances the
system replaces physics stepping with an animation-driven fallback at zero simulation cost.
LOD transitions interpolate particle positions to avoid visual popping.

- **Requirements:** R-4.7.7
- **Dependencies:** F-4.7.2, F-1.1.1
- **Platform notes:** None
