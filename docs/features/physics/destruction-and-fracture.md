# 4.6 — Destruction & Fracture

## Fracture Generation

### F-4.6.1 Voronoi Fracture Generation

Generate fracture patterns using 3D Voronoi decomposition at build time. Fracture point
placement supports random, impact-directed, and artist-guided seeding. The output is a
fracture asset containing convex hull geometry per fragment, a connectivity graph, and
joint configurations. At runtime, the `Destructible` component on an entity references
this asset by handle. No runtime mesh generation occurs; all fragment data is loaded from
the pre-computed asset.

- **Requirements:** R-4.6.1
- **Dependencies:** F-1.1.1, F-4.2.3
- **Platform notes:** None

### F-4.6.2 Pre-Fractured Mesh Authoring

Support pre-fractured meshes authored in DCC tools or generated offline by the content
pipeline. The resulting fracture asset stores fragment geometry, connectivity graph, and
joint break thresholds in a single file. At runtime, an entity with a `Destructible`
component references this asset. When fracture activates, the system spawns fragment
entities with `RigidBody`, `Collider`, and `DebrisLifetime` components, and Joint
entities (F-4.3.4) connecting adjacent fragments. This enables art-directed destruction
for hero objects like castle walls, towers, and bridges.

- **Requirements:** R-4.6.2
- **Dependencies:** F-1.1.1, F-4.6.1, F-4.3.4
- **Platform notes:** None

## Runtime Destruction

### F-4.6.3 Runtime Fracture and Activation

`FractureActivationSystem` queries all `(Destructible, DamageHealth)` entities and
triggers fracture when cumulative damage exceeds the threshold stored in `Destructible`.
On fracture, the system despawns the intact entity and spawns one entity per fragment
with `RigidBody`, `Collider`, `DebrisLifetime`, and `Transform` components initialized
from the fracture asset. Adjacent fragments are connected by Joint entities using
breakable joints from F-4.3.4, which separate based on impact propagation direction and
magnitude. Fragment spawning is budgeted per frame to avoid hitches during large-scale
battles.

- **Requirements:** R-4.6.3
- **Dependencies:** F-1.1.1, F-4.6.1, F-4.6.2, F-4.3.4
- **Platform notes:** None

### F-4.6.4 Progressive Damage Model

The `DamageHealth` component tracks cumulative damage as a scalar integrity value that
decreases with each impact. `DamageAccumulationSystem` queries all
`(DamageHealth, Collider)` entities, processes contact events, and subtracts damage
based on impact impulse magnitude. Visual cracking stages are driven by thresholds in
the `DamageHealth` component, providing gameplay feedback before full fracture. The
server maintains authoritative `DamageHealth` state via the ECS state replication system
to prevent client-side cheating.

- **Requirements:** R-4.6.4
- **Dependencies:** F-1.1.1, F-4.6.3
- **Platform notes:** None

## Structural Analysis

### F-4.6.5 Stress Propagation and Structural Collapse

`StructuralAnalysisSystem` queries all fragment entities connected by Joint entities
and performs a graph traversal to identify fragments without a path to a grounded anchor.
When a load-bearing fragment is destroyed or its Joint entity breaks, the system re-
evaluates connectivity. Unsupported fragments have their Joint entities despawned, causing
them to fall as independent `RigidBody` entities under gravity. This enables cascading
collapses of buildings and fortifications using only ECS queries over fragment and Joint
entities.

- **Requirements:** R-4.6.5
- **Dependencies:** F-1.1.1, F-4.6.3, F-4.3.4
- **Platform notes:** None

## Debris Management

### F-4.6.6 Debris Simulation and Lifecycle

Fragment entities spawned during fracture carry a `DebrisLifetime` component with a
configurable time-to-live value. `DebrisLifetimeSystem` queries all
`(DebrisLifetime, Transform)` entities, decrements the timer each frame, and despawns
entities whose lifetime has expired. A maximum debris count is enforced by querying all
`DebrisLifetime` entities and despawning the oldest when the cap is exceeded. Fragments
transition through active, settling, and sleeping states using the standard `RigidBody`
sleep logic.

- **Requirements:** R-4.6.6
- **Dependencies:** F-1.1.1, F-4.1.6, F-4.6.3
- **Platform notes:** None

### F-4.6.7 Debris Pooling and LOD

Debris pooling uses ECS entity recycling: despawned fragment entities are returned to a
pool and reused by resetting their `RigidBody`, `Collider`, `DebrisLifetime`, and
`Transform` components with new fragment data, eliminating allocation churn during
destruction events. `DebrisLodSystem` queries all `(DebrisLifetime, Transform)` entities
and reduces collision shape complexity and simulation fidelity for distant fragments.
Beyond a configurable distance, debris entities have their `RigidBody` and `Collider`
components removed and are represented as visual-only particles with no simulation cost.

- **Requirements:** R-4.6.7
- **Dependencies:** F-1.1.1, F-4.6.6
- **Platform notes:** None
