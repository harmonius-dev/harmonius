# R-4.2 -- Collision Detection Requirements

1. **R-4.2.1** -- The engine **SHALL** cull collision pairs using the shared BVH spatial index
   filtered by CollisionLayers, writing results to a BroadphasePairs resource. Broadphase **SHALL**
   process at least 50,000 entities within 1 ms on minimum-spec hardware.
   - **Rationale:** A shared BVH eliminates redundant spatial structures across physics, rendering,
     and AI.
   - **Verification:** Populate 50K entities. Assert broadphase completes within 1 ms. Assert zero
     false negatives against brute-force AABB overlap.

2. **R-4.2.2** -- The engine **SHALL** generate contact points and penetration depths using GJK,
   EPA, and SAT, producing ContactManifold components. Contact generation **SHALL** be bit-identical
   given identical inputs.
   - **Rationale:** Multiple algorithms are needed for general convex, primitive, and penetration
     cases.
   - **Verification:** Position known primitive pairs. Assert penetration depth accuracy within 1
     mm. Assert bit-identical results across runs.

3. **R-4.2.3** -- The engine **SHALL** support box, sphere, capsule, and convex hull shapes via a
   Collider component with specialized fast paths for primitive pairs achieving at least 2x speedup
   over generic GJK.
   - **Rationale:** Primitive fast paths optimize the most frequent collision type.
   - **Verification:** Benchmark sphere-sphere via fast path and generic GJK. Assert at least 2x
     speedup.

4. **R-4.2.4** -- The engine **SHALL** support triangle mesh and heightfield shapes with
   per-triangle material indices integrated with the shared BVH.
   - **Rationale:** Static geometry is concave; per-triangle materials enable surface-specific
     responses.
   - **Verification:** Drop a sphere onto a heightfield with two materials. Assert correct friction
     per region.

5. **R-4.2.5** -- The engine **SHALL** support compound colliders combining multiple primitives with
   per-child CollisionLayers and PhysicsMaterial.
   - **Rationale:** Complex silhouettes need compound shapes without concave decomposition.
   - **Verification:** Create a compound with 3 children. Ray cast hitting one child. Assert the hit
     reports that child's layer and material.

6. **R-4.2.6** -- The engine **SHALL** filter collisions via CollisionLayers bitmasks at broadphase
   and support custom filter callbacks for runtime rules.
   - **Rationale:** Layer filtering is O(1) and prevents wasted narrowphase on non-interacting
     groups.
   - **Verification:** Overlap two entities on non-interacting layers. Assert no ContactManifold.
     Register a callback rejecting same-team pairs. Assert no contact.

7. **R-4.2.7** -- The engine **SHALL** emit CollisionStarted, CollisionPersisted, and CollisionEnded
   events with contact data available the same frame they are detected.
   - **Rationale:** Same-frame delivery prevents desyncs between physics and gameplay feedback.
   - **Verification:** Move two spheres into contact for 5 frames then separate. Assert correct
     event sequence.

8. **R-4.2.8** -- The engine **SHALL** support trigger volumes with TriggerEnter, TriggerStay, and
   TriggerExit events, including one-shot, persistent, and filtered modes.
   - **Rationale:** Quest regions, hazards, and loading zones need spatial detection without forces.
   - **Verification:** Move a body through a trigger. Assert correct event lifecycle. Assert
     one-shot fires once.

9. **R-4.2.9** -- The engine **SHALL** support PhysicsMaterial assets with friction, restitution,
   density, surface tags, and configurable combine modes.
   - **Rationale:** Data-driven materials decouple surface properties from code.
   - **Verification:** Create two materials. Collide. Assert combined friction matches the
     configured combine mode.

## Voxel Terrain, Decomposition, and Layers

10. **R-4.2.10** -- The engine **SHALL** regenerate per-chunk triangle mesh colliders for voxel
    terrain on modification, rebuilding only affected chunks on a job worker and atomically swapping
    the collider in the physics BVH at the next `FixedUpdate` step.
    - **Rationale:** Runtime voxel edits require physics collision to update without blocking the
      game loop or regenerating unaffected chunks.
    - **Verification:** Carve a voxel chunk at runtime. Assert the affected chunk rebuilds on a
      worker thread. Assert the new collider is installed at the next FixedUpdate. Assert unaffected
      chunks remain unchanged.
11. **R-4.2.11** -- The engine **SHALL** decompose non-convex meshes into compound convex hulls at
    asset import time via V-HACD with configurable maximum hull count and per-hull vertex limit,
    storing the result as a baked `CompoundCollider`.
    - **Rationale:** Offline V-HACD trades import cost for cheap runtime collision without manual
      DCC decomposition.
    - **Verification:** Import a concave mesh with V-HACD enabled. Assert the baked CompoundCollider
      contains at most the configured hull count. Assert each hull respects the vertex limit.
12. **R-4.2.12** -- The engine **SHALL** generate convex hulls from arbitrary vertex sets at runtime
    via quickhull in O(n log n) for destructible fragments and procedural geometry.
    - **Rationale:** Destruction and procedural systems need cheap convex hull generation without an
      offline bake step.
    - **Verification:** Feed 1000 random vertices to the runtime quickhull. Assert the returned hull
      is convex. Assert runtime stays within the O(n log n) envelope.
13. **R-4.2.13** -- The engine **SHALL** expose an editor-configurable 32x32 collision layer
    interaction matrix that generates layer filter bitmasks at build time, supporting hitbox/hurtbox
    overlap without physics response and per-layer budgeting.
    - **Rationale:** Designers need to toggle layer pair interaction from the editor without writing
      filter code, and combat hitboxes need overlap without contact response.
    - **Verification:** Toggle a layer pair off in the editor. Assert the generated bitmask excludes
      the pair. Assert a hitbox layer overlaps its target without generating contact impulses.

## Non-Functional Requirements

14. **R-4.2.NF1** -- The engine **SHALL** complete broadphase for 50,000 AABBs within 1 ms on
    minimum-spec hardware.
    - **Rationale:** Broadphase must not dominate the physics frame budget in dense scenes.
    - **Verification:** Populate 50K entities with random extents. Assert broadphase completes
      within 1 ms.

15. **R-4.2.NF2** -- The engine **SHALL** complete narrowphase for 10,000 primitive pairs within 2
    ms on minimum-spec hardware.
    - **Rationale:** Narrowphase must scale to large contact counts without exceeding frame budget.
    - **Verification:** Benchmark 10K overlapping primitive pairs. Assert narrowphase completes
      within 2 ms.

16. **R-4.2.NF3** -- The engine **SHALL** deliver collision events in the same frame they are
    detected with zero latency.
    - **Rationale:** Same-frame delivery prevents desync between physics and gameplay feedback.
    - **Verification:** Move two spheres into contact. Assert CollisionStarted fires the same frame
      contact is detected.
