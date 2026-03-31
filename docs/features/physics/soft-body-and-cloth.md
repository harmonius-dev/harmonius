# 4.7 — Soft Body & Cloth

## Simulation Core

| ID      | Feature |
|---------|-------------------------------- |
| F-4.7.1 | Position-Based Dynamics Solver |
| F-4.7.2 | Cloth Simulation |

1. **F-4.7.1** — The `XpbdSolverSystem` queries all entities with `(ClothSimulation, Transform)`
   components and runs Extended Position-Based Dynamics constraint resolution each physics tick. The
   `ClothSimulation` component references GPU buffer handles for particle positions, velocities, and
   constraint descriptors (distance, bending, volume preservation, shape-matching). Constraint
   compliance is configured per-entity, enabling stiffness-independent solving without a separate
   physics world. This feature shares scope with animation/cloth-hair.md (F-9.5.1, F-9.5.5): the
   physics domain owns the simulation; the animation domain owns character garment authoring and
   LOD.
   - **Deps:** F-1.1.1, F-1.1.2, F-9.5.1 (Cloth Simulation — Animation)
   - **Platform:** Mobile: max 2 solver iterations, 128 particles per cloth. Switch: max 4
     iterations, 256 particles. Desktop: max 8 iterations, 1024 particles. High-end PC: max 16
     iterations, 4096 particles. GPU compute preferred on desktop+.
2. **F-4.7.2** — Each cloth instance is an entity with a `ClothSimulation` component that stores
   particle and constraint buffer handles, material parameters (stretch, shear, bend resistance),
   and mesh topology. A `ClothAttachment` component links cloth particles to skeleton bone entities
   via ECS entity references, enabling capes, banners, and sails to follow animated characters. The
   `ClothSimulationSystem` steps the particle-constraint mesh each tick by reading `ClothSimulation`
   and writing updated particle positions back to the same component's buffers. This feature shares
   scope with animation/cloth-hair.md (F-9.5.1, F-9.5.5): the physics domain owns the simulation;
   the animation domain owns character garment authoring and LOD.
   - **Deps:** F-4.7.1, F-1.1.1
   - **Platform:** Mobile: max 2 active cloth instances, player character only. Switch: max 4
     instances. Desktop: max 16 instances. High-end PC: max 64 instances. Distant cloth replaced
     with animation-driven fallback on all platforms.

## Collision and Interaction

| ID      | Feature |
|---------|----------------------------- |
| F-4.7.3 | Cloth Self-Collision |
| F-4.7.4 | Two-Way Rigid Body Coupling |

1. **F-4.7.3** — Entities with both `ClothSimulation` and a `SelfCollisionEnabled` marker component
   participate in self-collision detection. The `ClothSelfCollisionSystem` queries all
   `(ClothSimulation, SelfCollisionEnabled)` entities and runs spatial hashing or BVH-based
   continuous collision detection on their particles. Configurable thickness and skip-distance
   parameters live on the `SelfCollisionEnabled` component to manage per-instance budgets.
   - **Deps:** F-4.7.2, F-1.1.1
   - **Platform:** Mobile: disabled by default (too expensive). Switch: enabled for player cloth
     only, coarse spatial hash. Desktop: enabled for nearby cloth instances. High-end PC: full
     BVH-based self-collision for all active cloth.
2. **F-4.7.4** — The `XpbdSolverSystem` reads `Collider` and `Transform` components on nearby rigid
   body entities to generate contact constraints against cloth particles. Reaction forces are
   written as `ExternalForce` components on the rigid body entities, enabling two-way coupling where
   cloth drapes over objects and rigid bodies deflect from soft body contact. All interaction is
   resolved through ECS component queries with no separate collision world.
   - **Deps:** F-4.7.1, F-4.1.3, F-1.1.1

## Environmental Effects

| ID      | Feature |
|---------|------------------ |
| F-4.7.5 | Wind Interaction |
| F-4.7.6 | Cloth Tearing |

1. **F-4.7.5** — Wind sources are entities with a `WindSource` component specifying type
   (directional, point, vortex), position, direction, strength, radius, and turbulence noise
   parameters. The `WindFieldGenerationSystem` queries all active `WindSource` entities each frame
   and samples their contributions into a shared 3D wind field texture. All wind consumers (cloth,
   hair, foliage, particles) sample wind forces from this shared texture rather than querying
   `WindSource` entities directly. The `ClothWindSystem` reads the shared wind field texture and
   applies the sampled wind force to every `(ClothSimulation, Transform)` entity within range. This
   brings banners, flags, and sails to life through pure ECS system scheduling with a unified wind
   representation.
   - **Deps:** F-4.7.2, F-1.1.1
   - **Platform:** Mobile: wind field texture 32x32x16, max 2 wind sources, no turbulence noise.
     Switch: 64x64x32, max 4 sources. Desktop: 128x128x64, max 16 sources with turbulence. High-end
     PC: 256x256x128, unlimited sources.
2. **F-4.7.6** — The `ClothTearingSystem` queries all `ClothSimulation` entities each tick and
   checks constraint strain against a configurable threshold stored on the component. When strain
   exceeds the limit, the system splits the `ClothSimulation` mesh topology, updates constraint
   buffers, and spawns new cloth entities for separated patches. Torn edges generate proper boundary
   normals for rendering, enabling destructible sails and battle-damaged banners.
   - **Deps:** F-4.7.2, F-1.1.1
   - **Platform:** Mobile: disabled by default; use pre-authored torn mesh swap. Switch: max 1 tear
     event per frame, max 2 patches. Desktop: max 4 tears per frame, 8 patches. High-end PC:
     unlimited tears with dynamic topology updates.

## Performance

| ID      | Feature |
|---------|----------------------- |
| F-4.7.7 | Cloth Level of Detail |

1. **F-4.7.7** — A `ClothLod` component on cloth entities controls simulation fidelity parameters:
   particle count, constraint iterations, and update frequency. The `ClothLodSystem` queries the
   active camera entity's `Transform` and computes distance to each
   `(ClothSimulation, ClothLod, Transform)` entity, then adjusts the `ClothLod` tier accordingly. At
   extreme distances the system replaces physics stepping with an animation-driven fallback at zero
   simulation cost. LOD transitions interpolate particle positions to avoid visual popping.
   - **Deps:** F-4.7.2, F-1.1.1
   - **Platform:** Mobile: 2 LOD tiers (full, animation fallback), transition at 5m. Switch: 3
     tiers, transitions at 10m/20m. Desktop: 4 tiers, transitions at 15m/30m/60m. High-end PC: 4
     tiers with extended distances (25m/50m/100m).
