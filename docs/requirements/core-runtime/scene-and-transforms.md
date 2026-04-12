# R-1.2 — Scene & Transforms Requirements

## Scene Hierarchy

1. **R-1.2.1** — The engine **SHALL** represent scene hierarchy as parent-child relationships stored
   in ECS components, where each entity has at most one parent and an ordered list of children, with
   hierarchy modifications batched through command buffers.
   - **Rationale:** ECS-native hierarchy enables efficient parallel traversal for transform
     propagation, culling, and serialization.
   - **Verification:** Build 1,000-entity hierarchy; verify each has exactly one parent (except
     root); verify child ordering preserved after insert/remove. Verify modifications during
     parallel iteration are deferred.
2. **R-1.2.2** — The engine **SHALL** provide allocation-free depth-first and breadth-first
   traversal iterators with early termination and subtree skipping, supporting trees up to 256
   levels without heap allocation. For deeper trees, the iterator **SHALL** transparently fall back
   to heap allocation. A diagnostic warning **SHALL** fire when depth exceeds 128 levels.
   - **Rationale:** Allocation-free traversal is critical for per-frame culling and LOD selection.
   - **Verification:** Traverse 5-level hierarchy with DFS and BFS; verify visit order. Verify zero
     heap allocations for trees under 64 levels. Traverse 300-level tree; verify completion via
     fallback. Verify warning at depth 129.

## Parent-Child Relationships

1. **R-1.2.3** — The engine **SHALL** recursively despawn all descendants when a parent is
   despawned, with an optional orphan-on-delete mode reparenting children to the world root.
   - **Rationale:** Automatic cascade prevents orphaned entities from leaking resources.
   - **Verification:** Build 3-level hierarchy; despawn root in cascade mode; verify all descendants
     destroyed. Repeat with orphan-on-delete; verify children reparented to root.

## Transform Propagation

1. **R-1.2.4** — The engine **SHALL** compute world-space transforms by composing local transforms
   in a top-down parallel system processing independent subtrees concurrently, handling chains of
   arbitrary depth without stack overflow. Propagation **SHALL** process at least 2 million entities
   per millisecond on 4 cores.
   - **Rationale:** Parallel propagation is essential for frame-time budgets with deep hierarchies.
   - **Verification:** Propagate 100,000 entities at mixed depths (1-50); verify correct results
     against serial reference. Verify no stack overflow at depth 1,000. Benchmark 2M entities on 4
     cores under 1 ms.
2. **R-1.2.5** — Propagation **SHALL** introduce no more than 1 frame of latency between a local
   transform modification and its world-space result being visible to other systems.
   - **Rationale:** Latency directly affects visual fidelity and physics accuracy.
   - **Verification:** Modify local transform in frame N; verify world-space result available to all
     systems in frame N.

## Dirty Tracking

1. **R-1.2.6** — The engine **SHALL** use ECS change detection to mark transforms dirty and skip
   world-space recomputation for subtrees with no dirty ancestors.
   - **Rationale:** In open worlds where most entities are stationary, dirty tracking reduces
     propagation cost by orders of magnitude.
   - **Verification:** 100,000 entities with 1% moving; verify propagation time proportional to
     dirty count. Modify a leaf; verify only its subtree recomputed.

## Spatial Partitioning

1. **R-1.2.7** — The engine **SHALL** delegate spatial partitioning to the shared BVH spatial index.
   The scene module **SHALL** register transform-bearing entities with the shared index and wrap the
   unified spatial query API for scene-level queries.
   - **Rationale:** A shared index amortizes update costs and eliminates redundant spatial
     structures.
   - **Verification:** Insert 2M entities; perform 100 frustum queries per frame; verify all
     complete within 1 ms total. Verify incremental update cost proportional to moved entities.

## Scene Queries

1. **R-1.2.8** — The engine **SHALL** provide a spatial query API supporting point containment, ray
   intersection, sphere/box overlap, and k-nearest-neighbor queries, combining spatial filtering
   with ECS archetype filtering.
   - **Rationale:** Combined filtering avoids expensive post-filtering and powers proximity triggers
     and AoE.
   - **Verification:** Populate scene with 10,000 entities; perform sphere overlap with With<Enemy>
     filter; verify only enemies within sphere returned. Verify ray hits sorted by distance.

## 2D Transforms

1. **R-1.2.9** — The engine **SHALL** provide 2D transform components (Transform2D,
   GlobalTransform2D, PreviousGlobalTransform2D) using Vec2 position, f32 rotation, Vec2 scale, with
   Mat3 propagation through the hierarchy.
   - **Rationale:** 2D and 2.5D games need efficient 2D math without the overhead of unused 3D
     transform components.
   - **Verification:** Build a 3-level 2D hierarchy; propagate; verify GlobalTransform2D matches
     manual Mat3 composition. Verify no 3D transform components are required.

## Previous-Frame Transforms

1. **R-1.2.10** — The engine **SHALL** store the previous frame's GlobalTransform for each entity,
   copying current to previous before each propagation pass, enabling render interpolation between
   fixed-timestep positions.
   - **Rationale:** Fixed-timestep simulation produces discrete positions; previous-frame storage
     enables smooth interpolation for rendering at variable frame rates.
   - **Verification:** Run simulation at 30 Hz with rendering at 60 Hz; verify
     PreviousGlobalTransform contains frame N-1 values during frame N propagation. Verify copy
     occurs before propagation, not after.

## Interpolated Transforms

1. **R-1.2.11** — The engine **SHALL** compute interpolated transforms as lerp(previous, current,
   alpha) during the Frame Snapshot phase for smooth rendering between fixed-timestep simulation
   ticks.
   - **Rationale:** Interpolation eliminates visual stutter when the render rate differs from the
     simulation rate.
   - **Verification:** Simulate at 30 Hz, render at 60 Hz; verify interpolated position at alpha=0.5
     is midpoint between previous and current. Verify alpha=0.0 matches previous and alpha=1.0
     matches current.

## Hierarchical Scene Spawning

1. **R-1.2.12** — The engine **SHALL** support spawning a scene as children of a specified parent
   entity, with all root entities in the scene becoming children of the parent.
   - **Rationale:** Scene composition requires attaching spawned content to existing hierarchy nodes
     for equipment, vehicle passengers, and UI panels.
   - **Verification:** Spawn a 5-entity scene as children of an existing parent; verify all scene
     roots are children of the parent. Verify transforms propagate through the attachment point.

## Scene Instance Tracking

1. **R-1.2.13** — The engine **SHALL** track each spawned scene instance with an opaque
   SceneInstanceId, enabling bulk despawn of all entities from a single scene spawn.
   - **Rationale:** Unloading a streamed zone or removing a spawned prefab requires despawning all
     entities from that instance without affecting other entities.
   - **Verification:** Spawn a scene twice; despawn one instance by SceneInstanceId; verify only
     that instance's entities are removed and the other instance is intact.

## Ordered Child Insertion

1. **R-1.2.14** — The engine **SHALL** support inserting a child at a specific index in the parent's
   children list for ordered siblings in UI and editor.
   - **Rationale:** UI widget ordering and editor entity lists require deterministic sibling order
     with insertion at arbitrary positions.
   - **Verification:** Insert children A, B, C; insert D at index 1; verify order is A, D, B, C.
     Verify parent's children list reflects the insertion index.
