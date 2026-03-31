# R-4.4 -- Spatial Queries Requirements

1. **R-4.4.1** -- The engine **SHALL** provide ray casts via a RayCast system parameter querying the
   shared spatial index, returning hit entity, position, normal, distance, and CollisionLayers.
   - **Rationale:** Ray casts are foundational for line-of-sight, targeting, and AI perception.
   - **Verification:** Cast a ray at a known target. Assert correct hit position and normal. Assert
     layer filtering excludes ignored entities.

2. **R-4.4.2** -- The engine **SHALL** provide shape casts (sphere, capsule, box, convex hull) via a
   ShapeCast system parameter, reporting first or all contacts.
   - **Rationale:** Shape casts power character ground detection and projectile hit detection.
   - **Verification:** Sweep a capsule along a direction. Assert contact point and normal are
     correct. Assert first-hit and all-hits modes both work.

3. **R-4.4.3** -- The engine **SHALL** provide overlap queries returning all entities whose
   colliders overlap a given shape, with CollisionLayers filtering.
   - **Rationale:** Overlap tests power area-of-effect abilities and proximity triggers.
   - **Verification:** Place a sphere overlapping 5 entities. Assert all 5 are returned. Assert
     layer-filtered entities are excluded.

4. **R-4.4.4** -- The engine **SHALL** provide closest- point queries returning the nearest surface
   point, signed distance, and normal on any Collider.
   - **Rationale:** Closest-point queries enable distance triggers and attachment calculations.
   - **Verification:** Query closest point to a known position. Assert result within 1 mm of the
     analytic answer.

5. **R-4.4.5** -- The engine **SHALL** support batch spatial queries executed in parallel across
   worker threads, processing at least 512 queries per batch on desktop.
   - **Rationale:** Server-side AI with hundreds of agents requires amortized query cost.
   - **Verification:** Submit 512 ray casts as a batch. Assert results match individual queries.
     Assert parallel execution on multiple threads.

6. **R-4.4.6** -- The engine **SHALL** support query filters combining CollisionLayers masks, ECS
   component filters, and custom predicates receiving an EntityRef.
   - **Rationale:** Custom predicates enable contextual queries without post-filtering.
   - **Verification:** Register a predicate excluding entities with a specific component. Assert
     filtered entities are skipped during traversal.
