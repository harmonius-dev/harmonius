# 4.6 — Destruction & Fracture

## Fracture Generation

| ID      | Feature                      | Requirements |
|---------|------------------------------|--------------|
| F-4.6.1 | Voronoi Fracture Generation  | R-4.6.1      |
| F-4.6.2 | Pre-Fractured Mesh Authoring | R-4.6.2      |

1. **F-4.6.1** — Generate fracture patterns using 3D Voronoi decomposition at build time. Fracture
   point placement supports random, impact-directed, and artist-guided seeding. The output is a
   fracture asset containing convex hull geometry per fragment, a connectivity graph, and joint
   configurations. At runtime, the `Destructible` component on an entity references this asset by
   handle. No runtime mesh generation occurs; all fragment data is loaded from the pre-computed
   asset.
   - **Deps:** F-1.1.1, F-4.2.3
   - **Platform:** Mobile: max 8 fragments per fracture asset. Switch: max 16 fragments. Desktop:
     max 64 fragments. High-end PC: max 256 fragments. Platform-specific LOD variants baked at build
     time to reduce runtime fragment count.
2. **F-4.6.2** — Support pre-fractured meshes authored in DCC tools or generated offline by the
   content pipeline. The resulting fracture asset stores fragment geometry, connectivity graph, and
   joint break thresholds in a single file. At runtime, an entity with a `Destructible` component
   references this asset. When fracture activates, the system spawns fragment entities with
   `RigidBody`, `Collider`, and `DebrisLifetime` components, and Joint entities (F-4.3.4) connecting
   adjacent fragments. This enables art-directed destruction for hero objects like castle walls,
   towers, and bridges.
   - **Deps:** F-1.1.1, F-4.6.1, F-4.3.4

## Runtime Destruction

| ID      | Feature                         | Requirements |
|---------|---------------------------------|--------------|
| F-4.6.3 | Runtime Fracture and Activation | R-4.6.3      |
| F-4.6.4 | Progressive Damage Model        | R-4.6.4      |

1. **F-4.6.3** — `FractureActivationSystem` queries all `(Destructible, DamageHealth)` entities and
   triggers fracture when cumulative damage exceeds the threshold stored in `Destructible`. On
   fracture, the system despawns the intact entity and spawns one entity per fragment with
   `RigidBody`, `Collider`, `DebrisLifetime`, and `Transform` components initialized from the
   fracture asset. Adjacent fragments are connected by Joint entities using breakable joints from
   F-4.3.4, which separate based on impact propagation direction and magnitude. Fragment spawning is
   budgeted per frame to avoid hitches during large-scale battles.
   - **Deps:** F-1.1.1, F-4.6.1, F-4.6.2, F-4.3.4
   - **Platform:** Mobile: max 1 fracture activation per frame, 8 fragments max. Switch: max 2
     activations per frame, 16 fragments. Desktop: max 8 activations per frame, 64 fragments.
     High-end PC: max 16 activations, 256 fragments. Spawning staggered across frames on constrained
     platforms.
2. **F-4.6.4** — The `DamageHealth` component tracks cumulative damage as a scalar integrity value
   that decreases with each impact. `DamageAccumulationSystem` queries all
   `(DamageHealth, Collider)` entities, processes contact events, and subtracts damage based on
   impact impulse magnitude. Visual cracking stages are driven by thresholds in the `DamageHealth`
   component, providing gameplay feedback before full fracture. The server maintains authoritative
   `DamageHealth` state via the ECS state replication system to prevent client-side cheating.
   - **Deps:** F-1.1.1, F-4.6.3

## Structural Analysis

| ID      | Feature                                    | Requirements |
|---------|--------------------------------------------|--------------|
| F-4.6.5 | Stress Propagation and Structural Collapse | R-4.6.5      |

1. **F-4.6.5** — `StructuralAnalysisSystem` queries all fragment entities connected by Joint
   entities and performs a graph traversal to identify fragments without a path to a grounded
   anchor. When a load-bearing fragment is destroyed or its Joint entity breaks, the system
   re-evaluates connectivity. Unsupported fragments have their Joint entities despawned, causing
   them to fall as independent `RigidBody` entities under gravity. This enables cascading collapses
   of buildings and fortifications using only ECS queries over fragment and Joint entities.
   - **Deps:** F-1.1.1, F-4.6.3, F-4.3.4
   - **Platform:** Mobile: disabled by default; fracture uses pre-baked collapse sequences. Switch:
     simplified graph traversal, max 32 fragments per structure. Desktop: full stress propagation,
     max 256 fragments. High-end PC: full propagation, max 1024 fragments with parallel graph
     traversal.

## Debris Management

| ID      | Feature                         | Requirements |
|---------|---------------------------------|--------------|
| F-4.6.6 | Debris Simulation and Lifecycle | R-4.6.6      |
| F-4.6.7 | Debris Pooling and LOD          | R-4.6.7      |

1. **F-4.6.6** — Fragment entities spawned during fracture carry a `DebrisLifetime` component with a
   configurable time-to-live value. `DebrisLifetimeSystem` queries all `(DebrisLifetime, Transform)`
   entities, decrements the timer each frame, and despawns entities whose lifetime has expired. A
   maximum debris count is enforced by querying all `DebrisLifetime` entities and despawning the
   oldest when the cap is exceeded. Fragments transition through active, settling, and sleeping
   states using the standard `RigidBody` sleep logic.
   - **Deps:** F-1.1.1, F-4.1.6, F-4.6.3
   - **Platform:** Mobile: max 32 debris entities, 3-second TTL. Switch: max 64 debris, 5-second
     TTL. Desktop: max 512 debris, 10-second TTL. High-end PC: max 2048 debris, 30-second TTL.
     Shorter lifetimes on constrained platforms free simulation budget faster.
2. **F-4.6.7** — Debris pooling uses ECS entity recycling: despawned fragment entities are returned
   to a pool and reused by resetting their `RigidBody`, `Collider`, `DebrisLifetime`, and
   `Transform` components with new fragment data, eliminating allocation churn during destruction
   events. `DebrisLodSystem` queries all `(DebrisLifetime, Transform)` entities and reduces
   collision shape complexity and simulation fidelity for distant fragments. Beyond a configurable
   distance, debris entities have their `RigidBody` and `Collider` components removed and are
   represented as visual-only particles with no simulation cost.
   - **Deps:** F-1.1.1, F-4.6.6
   - **Platform:** Mobile: pool size 32, LOD transition at 10m, particle fallback at 20m. Switch:
     pool 64, LOD at 20m, particle at 40m. Desktop: pool 512, LOD at 50m, particle at 100m. High-end
     PC: pool 2048, extended distances.

## Voxel and SDF Integration

| ID       | Feature                       | Requirements |
|----------|-------------------------------|--------------|
| F-4.6.8  | Voxel Collision Generation    | R-4.6.8      |
| F-4.6.9  | Destruction Fragment Physics  | R-4.6.9      |
| F-4.6.10 | SDF Collision Queries         | R-4.6.10     |
| F-4.6.11 | Voxel Destruction Integration | R-4.6.11     |

1. **F-4.6.8** — Auto-generate convex hull or trimesh colliders from voxel SDF meshes produced by
   the meshing pipeline (F-3.2.12). When voxel terrain is edited at runtime (F-3.2.13), the
   `VoxelColliderUpdateSystem` incrementally regenerates colliders for only the modified chunks
   rather than rebuilding the entire volume. Generated colliders are registered in the shared
   spatial index (F-1.9.1) and carry `PhysicsMaterial` assignments derived from voxel material IDs.
   Collider shape selection (convex hull vs. trimesh) is automatic based on chunk geometry
   complexity — simple chunks use convex decomposition, complex chunks use trimesh.
   - **Deps:** F-3.2.9, F-3.2.12, F-3.2.13, F-1.9.1, F-4.2.3, F-4.2.4
   - **Platform:** Mobile: convex hull only (no trimesh from voxels), max 16 vertices per hull.
     Switch: convex hull preferred, trimesh for complex chunks up to 8K triangles. Desktop: full
     trimesh support up to 64K triangles per chunk.
2. **F-4.6.9** — Fragment entities spawned by the fracture system (F-4.6.3) inherit the parent
   entity's linear and angular velocity plus an additional impulse derived from the impact point and
   direction. The `FragmentImpulseSystem` distributes mass across fragments proportionally to their
   volume (computed from convex hull geometry). Fragments auto-sleep after settling using the
   standard sleep system (F-4.1.6) with configurable per-fragment sleep thresholds. Large fragments
   receive higher sleep thresholds to settle faster, reducing simulation cost during sustained
   destruction sequences.
   - **Deps:** F-4.6.3, F-4.1.6, F-4.1.1
   - **Platform:** Mobile: fragments inherit velocity only (no impact impulse distribution). Switch:
     simplified mass distribution (uniform). Desktop: full volume-based mass distribution with
     per-fragment sleep tuning.
3. **F-4.6.10** — Physics raycasts and shape casts query SDF volumes directly without requiring
   meshed geometry, using the `SdfQuerySystem`. The system evaluates the signed distance field at
   sample points along the ray or sweep path via sphere-tracing (ray marching), returning hit
   position, surface normal (from SDF gradient), and distance. SDF queries serve as fast pre-checks
   before precise mesh collision — if the SDF query reports no hit, the expensive mesh narrowphase
   is skipped entirely. SDF queries integrate with the unified spatial query API (F-1.9.4) and
   accept standard `QueryFilter` parameters.
   - **Deps:** F-3.2.9, F-4.4.1, F-4.4.2, F-1.9.4
   - **Platform:** Mobile: SDF queries disabled (mesh collision only). Switch: SDF ray queries only
     (no shape sweeps against SDF). Desktop: full SDF ray and shape cast support with configurable
     march step count (default 64).
4. **F-4.6.11** — Physics damage events trigger voxel SDF subtraction operations on the terrain
   volume. The `VoxelDestructionSystem` listens for `CollisionEnded` events on entities with both
   `Collider` and `VoxelVolume` components, computes an impact point and blast radius from the
   impulse magnitude, and issues an SDF subtract operation (sphere or custom shape) to the voxel
   volume (F-3.2.13). After subtraction, the system performs a structural integrity check on
   remaining voxel connectivity — disconnected voxel islands are converted to physics fragments with
   `RigidBody` components and fall under gravity. Edit operations are throttled per frame to
   maintain performance.
   - **Deps:** F-3.2.9, F-3.2.13, F-4.2.7, F-4.6.3, F-1.1.1
   - **Platform:** Mobile: disabled by default; voxel destruction requires explicit opt-in with
     reduced blast radius (max 2m). Switch: max blast radius 5m, 1 voxel destruction per frame.
     Desktop: max blast radius 20m, 4 voxel destructions per frame. Structural integrity check
     disabled on mobile.
