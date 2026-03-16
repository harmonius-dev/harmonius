# User Stories — 4.4 Spatial Queries

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-4.4.1.1 | game developer (P-15) | As a game developer (P-15), I want to cast a ray and receive the hit entity, position, normal, and distance, so that line-of-sight and targeting queries work. | — | — | — |
| US-4.4.1.2 | engine developer (P-26) | As an engine developer (P-26), I want to implement RayCast as a system parameter that queries the shared spatial index, so that any system can issue ray casts. | — | — | — |
| US-4.4.1.3 | game developer (P-15) | As a game developer (P-15), I want ray casts to filter by CollisionLayers via QueryFilter, so that only relevant entities are hit. | — | — | — |
| US-4.4.1.4 | player (P-23) | As a player (P-23), I want weapons to hit exactly where I aim, so that combat targeting feels precise. | — | — | — |
| US-4.4.1.5 | engine tester (P-27) | As an engine tester (P-27), I want to verify ray cast results match expected hit position and normal for known geometry, so that accuracy is validated. | — | — | — |
| US-4.4.1.6 | game developer (P-15) | As a game developer (P-15), I want to use ray casts for line-of-sight between entities, so that AI visibility and stealth mechanics work. | — | — | — |
| US-4.4.1.7 | designer (P-5) | As a designer (P-5), I want to set maximum ray cast distance per query, so that queries do not waste time on distant geometry. | — | — | — |
| US-4.4.1.8 | engine developer (P-26) | As an engine developer (P-26), I want ray casts to traverse the shared BVH spatial index, so that physics, rendering, and AI share the same acceleration structure. | — | — | — |
| US-4.4.1.9 | engine tester (P-27) | As an engine tester (P-27), I want to test ray casts against box, sphere, capsule, convex, trimesh, and heightfield shapes, so that all shape intersections work. | — | — | — |
| US-4.4.1.10 | game developer (P-15) | As a game developer (P-15), I want ray cast results to include the surface normal at the hit point, so that I can orient decals and effects. | — | — | — |
| US-4.4.1.11 | game developer (P-15) | As a game developer (P-15), I want ray cast results to include the hit entity's CollisionLayers, so that I know what type of surface was hit. | — | — | — |
| US-4.4.1.12 | player (P-23) | As a player (P-23), I want the targeting reticle to align with the actual hit point, so that aiming feels responsive and trustworthy. | — | — | — |
| US-4.4.2.1 | game developer (P-15) | As a game developer (P-15), I want to sweep a sphere along a direction and detect the first collision, so that character ground detection works. | — | — | — |
| US-4.4.2.2 | game developer (P-15) | As a game developer (P-15), I want to sweep a capsule shape for safe movement queries, so that characters do not clip through walls. | — | — | — |
| US-4.4.2.3 | engine developer (P-26) | As an engine developer (P-26), I want to implement ShapeCast as a system parameter that sweeps shapes against ECS entities, so that any system can issue shape casts. | — | — | — |
| US-4.4.2.4 | engine developer (P-26) | As an engine developer (P-26), I want shape casts to report either the first or all contacts along the sweep, so that both single-hit and multi-hit queries work. | — | — | — |
| US-4.4.2.5 | engine tester (P-27) | As an engine tester (P-27), I want to verify shape cast results against analytic solutions for known scenarios, so that sweep accuracy is validated. | — | — | — |
| US-4.4.2.6 | player (P-23) | As a player (P-23), I want my character to navigate narrow passages without clipping through walls, so that movement feels solid. | — | — | — |
| US-4.4.2.7 | game developer (P-15) | As a game developer (P-15), I want to sweep a box shape for wide projectile hit detection, so that area-of-effect projectiles work correctly. | — | — | — |
| US-4.4.2.8 | game developer (P-15) | As a game developer (P-15), I want shape casts to accept QueryFilter for layer filtering, so that only relevant entities are tested. | — | — | — |
| US-4.4.2.9 | engine developer (P-26) | As an engine developer (P-26), I want shape casts to support convex hull sweeps, so that non-primitive shapes can be swept accurately. | — | — | — |
| US-4.4.2.10 | engine tester (P-27) | As an engine tester (P-27), I want to test shape casts against all collider types (box, sphere, capsule, convex, trimesh, heightfield), so that all combinations work. | — | — | — |
| US-4.4.2.11 | designer (P-5) | As a designer (P-5), I want to choose sweep shape type per query use case, so that ground detection, movement, and projectiles each use the best shape. | — | — | — |
| US-4.4.2.12 | player (P-23) | As a player (P-23), I want my character to slide along walls smoothly instead of stopping abruptly, so that collision response feels natural. | — | — | — |
| US-4.4.3.1 | game developer (P-15) | As a game developer (P-15), I want to test a sphere overlap and receive all overlapping entities, so that area-of-effect abilities find targets. | — | — | — |
| US-4.4.3.2 | game developer (P-15) | As a game developer (P-15), I want to test box overlaps for rectangular region queries, so that zone-based logic identifies entities in an area. | — | — | — |
| US-4.4.3.3 | engine developer (P-26) | As an engine developer (P-26), I want to implement OverlapQuery as a system parameter, so that any system can test shape overlaps. | — | — | — |
| US-4.4.3.4 | game developer (P-15) | As a game developer (P-15), I want overlap queries to filter by CollisionLayers, so that only relevant entities appear in results. | — | — | — |
| US-4.4.3.5 | player (P-23) | As a player (P-23), I want explosions to damage all enemies within range, so that area attacks feel impactful. | — | — | — |
| US-4.4.3.6 | engine tester (P-27) | As an engine tester (P-27), I want to verify overlap queries return all overlapping entities in known scenarios, so that no targets are missed. | — | — | — |
| US-4.4.3.7 | game developer (P-15) | As a game developer (P-15), I want to test capsule overlaps for trigger zone detection, so that cylindrical zones work correctly. | — | — | — |
| US-4.4.3.8 | designer (P-5) | As a designer (P-5), I want to choose overlap shape type per query, so that each gameplay mechanic uses the most appropriate volume. | — | — | — |
| US-4.4.3.9 | engine developer (P-26) | As an engine developer (P-26), I want overlap queries to traverse the broadphase resource and return Entity IDs with CollisionLayers, so that results are efficient. | — | — | — |
| US-4.4.3.10 | engine tester (P-27) | As an engine tester (P-27), I want to test convex hull overlaps, so that arbitrary shape overlaps are validated. | — | — | — |
| US-4.4.3.11 | game developer (P-15) | As a game developer (P-15), I want to use overlap queries to detect interactable objects near the player, so that interaction prompts appear contextually. | — | — | — |
| US-4.4.3.12 | player (P-23) | As a player (P-23), I want nearby items to be detected accurately for pickup, so that interaction prompts appear when I am close enough. | — | — | — |
| US-4.4.4.1 | game developer (P-15) | As a game developer (P-15), I want to query the closest point on any collider surface to a world-space point, so that distance calculations are precise. | — | — | — |
| US-4.4.4.2 | engine developer (P-26) | As an engine developer (P-26), I want to implement ClosestPointQuery as a system parameter, so that any system can compute closest points. | — | — | — |
| US-4.4.4.3 | game developer (P-15) | As a game developer (P-15), I want closest point queries for calculating attachment points, so that ropes and cables attach to surfaces precisely. | — | — | — |
| US-4.4.4.4 | engine developer (P-26) | As an engine developer (P-26), I want closest point results to include signed distance, so that systems know if the query point is inside or outside the surface. | — | — | — |
| US-4.4.4.5 | engine tester (P-27) | As an engine tester (P-27), I want to compare closest point results against analytic solutions for primitive shapes, so that accuracy is validated. | — | — | — |
| US-4.4.4.6 | player (P-23) | As a player (P-23), I want grappling hooks to attach to the nearest valid surface point, so that traversal mechanics feel precise. | — | — | — |
| US-4.4.4.7 | game developer (P-15) | As a game developer (P-15), I want closest point queries for distance-based triggers, so that activation distance is measured to the surface, not center. | — | — | — |
| US-4.4.4.8 | engine developer (P-26) | As an engine developer (P-26), I want closest point results to include the surface normal, so that effects and decals can be oriented correctly. | — | — | — |
| US-4.4.4.9 | engine tester (P-27) | As an engine tester (P-27), I want to test closest point queries against box, sphere, capsule, convex, trimesh, and heightfield shapes, so that all work correctly. | — | — | — |
| US-4.4.4.10 | game developer (P-15) | As a game developer (P-15), I want AI systems to use closest point queries for navigation decisions, so that agents reason about geometric proximity. | — | — | — |
| US-4.4.4.11 | designer (P-5) | As a designer (P-5), I want to limit closest point query search radius, so that queries do not waste time on distant geometry. | — | — | — |
| US-4.4.4.12 | player (P-23) | As a player (P-23), I want my character to grab the nearest ledge when climbing, so that traversal feels responsive. | — | — | — |
| US-4.4.5.1 | game developer (P-15) | As a game developer (P-15), I want to submit a batch of ray cast queries for parallel execution, so that many AI agents can query simultaneously. | — | — | — |
| US-4.4.5.2 | engine developer (P-26) | As an engine developer (P-26), I want to implement BatchSpatialQuery that accepts a slice of query descriptors and distributes them across worker threads, so that batch queries exploit parallelism. | — | — | — |
| US-4.4.5.3 | game developer (P-15) | As a game developer (P-15), I want batches to accept mixed query types (ray casts, shape casts, overlaps), so that different query needs are batched together. | — | — | — |
| US-4.4.5.4 | engine developer (P-26) | As an engine developer (P-26), I want batch results written to a caller-provided buffer, so that no allocation occurs during batch execution. | — | — | — |
| US-4.4.5.5 | engine tester (P-27) | As an engine tester (P-27), I want to confirm mobile caps at 32 queries per batch with single-threaded fallback, so that mobile frame budget is respected. | — | — | — |
| US-4.4.5.6 | player (P-23) | As a player (P-23), I want AI enemies to perceive me quickly and accurately, so that enemy behavior feels responsive. | — | — | — |
| US-4.4.5.7 | engine developer (P-26) | As an engine developer (P-26), I want batch queries to amortize broadphase traversal cost across all queries in the batch, so that overhead per query is minimized. | — | — | — |
| US-4.4.5.8 | engine developer (P-26) | As an engine developer (P-26), I want high-end PC batch queries to use SIMD ray-BVH acceleration for up to 4096 queries, so that server-side AI scales. | — | — | — |
| US-4.4.5.9 | engine tester (P-27) | As an engine tester (P-27), I want to compare batch query results against individual query results for the same inputs, so that batching does not affect accuracy. | — | — | — |
| US-4.4.5.10 | engine developer (P-26) | As an engine developer (P-26), I want desktop to support 512 queries per batch with workers scaling to core count, so that large query workloads are handled. | — | — | — |
| US-4.4.5.11 | designer (P-5) | As a designer (P-5), I want to configure maximum batch size per platform, so that query workloads match device capability. | — | — | — |
| US-4.4.5.12 | engine tester (P-27) | As an engine tester (P-27), I want to verify batch parallel execution produces identical results to serial execution, so that parallelism is deterministic. | — | — | — |
| US-4.4.6.1 | game developer (P-15) | As a game developer (P-15), I want all spatial queries to accept CollisionLayers bitmask filtering, so that only relevant entities are considered. | — | — | — |
| US-4.4.6.2 | game developer (P-15) | As a game developer (P-15), I want queries to accept With and Without ECS query filters, so that entities are filtered by component presence. | — | — | — |
| US-4.4.6.3 | engine developer (P-26) | As an engine developer (P-26), I want queries to accept custom predicate closures receiving EntityRef, so that arbitrary filtering logic runs during traversal. | — | — | — |
| US-4.4.6.4 | game developer (P-15) | As a game developer (P-15), I want custom predicates to filter by cover status during traversal, so that "find nearest visible enemy" works without post-filtering. | — | — | — |
| US-4.4.6.5 | engine tester (P-27) | As an engine tester (P-27), I want to verify custom predicates reject entities during traversal rather than after, so that performance benefit is real. | — | — | — |
| US-4.4.6.6 | player (P-23) | As a player (P-23), I want targeting to find the nearest valid target, so that I do not target dead, friendly, or immune entities. | — | — | — |
| US-4.4.6.7 | game developer (P-15) | As a game developer (P-15), I want to combine CollisionLayers bitmask filtering with component presence filters, so that queries are both layer- and type-aware. | — | — | — |
| US-4.4.6.8 | engine developer (P-26) | As an engine developer (P-26), I want QueryFilter to encapsulate bitmask, component filters, and optional predicate, so that all filtering is unified. | — | — | — |
| US-4.4.6.9 | engine tester (P-27) | As an engine tester (P-27), I want to verify custom predicate filtering reduces candidate count during traversal, so that performance scales. | — | — | — |
| US-4.4.6.10 | game developer (P-15) | As a game developer (P-15), I want queries to exclude entities with a Dead component via Without filter, so that dead entities are ignored. | — | — | — |
| US-4.4.6.11 | designer (P-5) | As a designer (P-5), I want to configure query filters in the visual editor, so that spatial query filtering is set up without code. | — | — | — |
| US-4.4.6.12 | engine tester (P-27) | As an engine tester (P-27), I want to test all combinations of bitmask, component, and custom predicate filters, so that all filter paths are validated. | — | — | — |
