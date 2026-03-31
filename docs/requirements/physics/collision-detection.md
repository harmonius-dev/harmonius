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
