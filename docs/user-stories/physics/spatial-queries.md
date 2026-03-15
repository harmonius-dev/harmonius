# User Stories — 4.4 Spatial Queries

## US-4.4.1.1 Cast Ray and Receive Hit Entity
**As a** game developer (P-15), **I want to** cast a ray and receive the hit entity, position,
normal, and distance, **so that** line-of-sight and targeting queries work.

## US-4.4.1.2 Implement RayCast System Parameter
**As an** engine developer (P-26), **I want to** implement RayCast as a system parameter that
queries the shared spatial index, **so that** any system can issue ray casts.

## US-4.4.1.3 Filter Ray Casts by CollisionLayers
**As a** game developer (P-15), **I want** ray casts to filter by CollisionLayers via QueryFilter,
**so that** only relevant entities are hit.

## US-4.4.1.4 See Projectiles Hit Targets Accurately
**As a** player (P-23), **I want** weapons to hit exactly where I aim, **so that** combat targeting
feels precise.

## US-4.4.1.5 Verify Ray Cast Returns Correct Hit Data
**As an** engine tester (P-27), **I want to** verify ray cast results match expected hit position
and normal for known geometry, **so that** accuracy is validated.

## US-4.4.1.6 Use Ray Casts for Line-of-Sight Checks
**As a** game developer (P-15), **I want to** use ray casts for line-of-sight between entities, **so
that** AI visibility and stealth mechanics work.

## US-4.4.1.7 Configure Ray Cast Max Distance
**As a** designer (P-5), **I want to** set maximum ray cast distance per query, **so that** queries
do not waste time on distant geometry.

## US-4.4.1.8 Query Shared Spatial Index for Ray Casts
**As an** engine developer (P-26), **I want** ray casts to traverse the shared BVH spatial index,
**so that** physics, rendering, and AI share the same acceleration structure.

## US-4.4.1.9 Test Ray Casts Against All Shape Types
**As an** engine tester (P-27), **I want to** test ray casts against box, sphere, capsule, convex,
trimesh, and heightfield shapes, **so that** all shape intersections work.

## US-4.4.1.10 Access Surface Normal from Ray Cast Hit
**As a** game developer (P-15), **I want** ray cast results to include the surface normal at the hit
point, **so that** I can orient decals and effects.

## US-4.4.1.11 Read CollisionLayers from Ray Cast Hit
**As a** game developer (P-15), **I want** ray cast results to include the hit entity's
CollisionLayers, **so that** I know what type of surface was hit.

## US-4.4.1.12 Aim at Objects and See Accurate Feedback
**As a** player (P-23), **I want** the targeting reticle to align with the actual hit point, **so
that** aiming feels responsive and trustworthy.

## US-4.4.2.1 Sweep Sphere Shape Along Direction
**As a** game developer (P-15), **I want to** sweep a sphere along a direction and detect the first
collision, **so that** character ground detection works.

## US-4.4.2.2 Sweep Capsule for Character Movement
**As a** game developer (P-15), **I want to** sweep a capsule shape for safe movement queries, **so
that** characters do not clip through walls.

## US-4.4.2.3 Implement ShapeCast System Parameter
**As an** engine developer (P-26), **I want to** implement ShapeCast as a system parameter that
sweeps shapes against ECS entities, **so that** any system can issue shape casts.

## US-4.4.2.4 Report All Contacts Along Sweep
**As an** engine developer (P-26), **I want** shape casts to report either the first or all contacts
along the sweep, **so that** both single-hit and multi-hit queries work.

## US-4.4.2.5 Verify Shape Cast Against Known Geometry
**As an** engine tester (P-27), **I want to** verify shape cast results against analytic solutions
for known scenarios, **so that** sweep accuracy is validated.

## US-4.4.2.6 Move Through Tight Spaces Without Clipping
**As a** player (P-23), **I want** my character to navigate narrow passages without clipping through
walls, **so that** movement feels solid.

## US-4.4.2.7 Sweep Box Shape for Projectile Detection
**As a** game developer (P-15), **I want to** sweep a box shape for wide projectile hit detection,
**so that** area-of-effect projectiles work correctly.

## US-4.4.2.8 Filter Shape Casts by QueryFilter
**As a** game developer (P-15), **I want** shape casts to accept QueryFilter for layer filtering,
**so that** only relevant entities are tested.

## US-4.4.2.9 Sweep Convex Hull for Complex Shapes
**As an** engine developer (P-26), **I want** shape casts to support convex hull sweeps, **so that**
non-primitive shapes can be swept accurately.

## US-4.4.2.10 Test Shape Cast with All Collider Types
**As an** engine tester (P-27), **I want to** test shape casts against all collider types (box,
sphere, capsule, convex, trimesh, heightfield), **so that** all combinations work.

## US-4.4.2.11 Configure Shape Cast Per Game Need
**As a** designer (P-5), **I want to** choose sweep shape type per query use case, **so that**
ground detection, movement, and projectiles each use the best shape.

## US-4.4.2.12 Experience Smooth Character Collision
**As a** player (P-23), **I want** my character to slide along walls smoothly instead of stopping
abruptly, **so that** collision response feels natural.

## US-4.4.3.1 Test Sphere Overlap for Area-of-Effect
**As a** game developer (P-15), **I want to** test a sphere overlap and receive all overlapping
entities, **so that** area-of-effect abilities find targets.

## US-4.4.3.2 Test Box Overlap for Region Queries
**As a** game developer (P-15), **I want to** test box overlaps for rectangular region queries, **so
that** zone-based logic identifies entities in an area.

## US-4.4.3.3 Implement OverlapQuery System Parameter
**As an** engine developer (P-26), **I want to** implement OverlapQuery as a system parameter, **so
that** any system can test shape overlaps.

## US-4.4.3.4 Filter Overlaps by CollisionLayers
**As a** game developer (P-15), **I want** overlap queries to filter by CollisionLayers, **so that**
only relevant entities appear in results.

## US-4.4.3.5 Hit All Enemies in Explosion Radius
**As a** player (P-23), **I want** explosions to damage all enemies within range, **so that** area
attacks feel impactful.

## US-4.4.3.6 Verify Overlap Completeness
**As an** engine tester (P-27), **I want to** verify overlap queries return all overlapping entities
in known scenarios, **so that** no targets are missed.

## US-4.4.3.7 Test Capsule Overlap for Trigger Zones
**As a** game developer (P-15), **I want to** test capsule overlaps for trigger zone detection, **so
that** cylindrical zones work correctly.

## US-4.4.3.8 Configure Overlap Shape Per Use Case
**As a** designer (P-5), **I want to** choose overlap shape type per query, **so that** each
gameplay mechanic uses the most appropriate volume.

## US-4.4.3.9 Traverse Broadphase for Overlap Results
**As an** engine developer (P-26), **I want** overlap queries to traverse the broadphase resource
and return Entity IDs with CollisionLayers, **so that** results are efficient.

## US-4.4.3.10 Test Overlap with ConvexHull Shape
**As an** engine tester (P-27), **I want to** test convex hull overlaps, **so that** arbitrary shape
overlaps are validated.

## US-4.4.3.11 Use Proximity Overlap for Interaction Prompts
**As a** game developer (P-15), **I want to** use overlap queries to detect interactable objects
near the player, **so that** interaction prompts appear contextually.

## US-4.4.3.12 Pick Up Nearby Items Reliably
**As a** player (P-23), **I want** nearby items to be detected accurately for pickup, **so that**
interaction prompts appear when I am close enough.

## US-4.4.4.1 Find Closest Point on Collider Surface
**As a** game developer (P-15), **I want to** query the closest point on any collider surface to a
world-space point, **so that** distance calculations are precise.

## US-4.4.4.2 Implement ClosestPointQuery System Parameter
**As an** engine developer (P-26), **I want to** implement ClosestPointQuery as a system parameter,
**so that** any system can compute closest points.

## US-4.4.4.3 Use Closest Point for Attachment Calculation
**As a** game developer (P-15), **I want** closest point queries for calculating attachment points,
**so that** ropes and cables attach to surfaces precisely.

## US-4.4.4.4 Return Signed Distance from Closest Point
**As an** engine developer (P-26), **I want** closest point results to include signed distance, **so
that** systems know if the query point is inside or outside the surface.

## US-4.4.4.5 Verify Closest Point Accuracy
**As an** engine tester (P-27), **I want to** compare closest point results against analytic
solutions for primitive shapes, **so that** accuracy is validated.

## US-4.4.4.6 Experience Accurate Grappling Point Selection
**As a** player (P-23), **I want** grappling hooks to attach to the nearest valid surface point,
**so that** traversal mechanics feel precise.

## US-4.4.4.7 Use Closest Point for Distance Triggers
**As a** game developer (P-15), **I want** closest point queries for distance-based triggers, **so
that** activation distance is measured to the surface, not center.

## US-4.4.4.8 Include Surface Normal in Results
**As an** engine developer (P-26), **I want** closest point results to include the surface normal,
**so that** effects and decals can be oriented correctly.

## US-4.4.4.9 Test Closest Point on All Shape Types
**As an** engine tester (P-27), **I want to** test closest point queries against box, sphere,
capsule, convex, trimesh, and heightfield shapes, **so that** all work correctly.

## US-4.4.4.10 Use Closest Point for AI Navigation
**As a** game developer (P-15), **I want** AI systems to use closest point queries for navigation
decisions, **so that** agents reason about geometric proximity.

## US-4.4.4.11 Configure Query Search Radius
**As a** designer (P-5), **I want to** limit closest point query search radius, **so that** queries
do not waste time on distant geometry.

## US-4.4.4.12 Snap to Nearest Ledge Accurately
**As a** player (P-23), **I want** my character to grab the nearest ledge when climbing, **so that**
traversal feels responsive.

## US-4.4.5.1 Submit Batch of Ray Casts for Parallel Execution
**As a** game developer (P-15), **I want to** submit a batch of ray cast queries for parallel
execution, **so that** many AI agents can query simultaneously.

## US-4.4.5.2 Implement BatchSpatialQuery
**As an** engine developer (P-26), **I want to** implement BatchSpatialQuery that accepts a slice of
query descriptors and distributes them across worker threads, **so that** batch queries exploit
parallelism.

## US-4.4.5.3 Mix Query Types in a Single Batch
**As a** game developer (P-15), **I want** batches to accept mixed query types (ray casts, shape
casts, overlaps), **so that** different query needs are batched together.

## US-4.4.5.4 Write Batch Results to Caller Buffer
**As an** engine developer (P-26), **I want** batch results written to a caller-provided buffer,
**so that** no allocation occurs during batch execution.

## US-4.4.5.5 Verify Batch Limits Per Platform
**As an** engine tester (P-27), **I want to** confirm mobile caps at 32 queries per batch with
single-threaded fallback, **so that** mobile frame budget is respected.

## US-4.4.5.6 Experience Responsive AI Perception
**As a** player (P-23), **I want** AI enemies to perceive me quickly and accurately, **so that**
enemy behavior feels responsive.

## US-4.4.5.7 Amortize Broadphase Traversal Cost
**As an** engine developer (P-26), **I want** batch queries to amortize broadphase traversal cost
across all queries in the batch, **so that** overhead per query is minimized.

## US-4.4.5.8 Use SIMD Acceleration on High-End PC
**As an** engine developer (P-26), **I want** high-end PC batch queries to use SIMD ray-BVH
acceleration for up to 4096 queries, **so that** server-side AI scales.

## US-4.4.5.9 Test Batch Query Result Correctness
**As an** engine tester (P-27), **I want to** compare batch query results against individual query
results for the same inputs, **so that** batching does not affect accuracy.

## US-4.4.5.10 Support 512 Queries Per Batch on Desktop
**As an** engine developer (P-26), **I want** desktop to support 512 queries per batch with workers
scaling to core count, **so that** large query workloads are handled.

## US-4.4.5.11 Configure Batch Size Per Platform
**As a** designer (P-5), **I want to** configure maximum batch size per platform, **so that** query
workloads match device capability.

## US-4.4.5.12 Verify Parallel Execution Correctness
**As an** engine tester (P-27), **I want to** verify batch parallel execution produces identical
results to serial execution, **so that** parallelism is deterministic.

## US-4.4.6.1 Filter Queries with CollisionLayers Bitmask
**As a** game developer (P-15), **I want** all spatial queries to accept CollisionLayers bitmask
filtering, **so that** only relevant entities are considered.

## US-4.4.6.2 Filter Queries by Component Presence
**As a** game developer (P-15), **I want** queries to accept With and Without ECS query filters,
**so that** entities are filtered by component presence.

## US-4.4.6.3 Implement Custom Predicate Closures
**As an** engine developer (P-26), **I want** queries to accept custom predicate closures receiving
EntityRef, **so that** arbitrary filtering logic runs during traversal.

## US-4.4.6.4 Find Nearest Enemy Not Behind Cover
**As a** game developer (P-15), **I want** custom predicates to filter by cover status during
traversal, **so that** "find nearest visible enemy" works without post-filtering.

## US-4.4.6.5 Verify Custom Predicates Execute During Traversal
**As an** engine tester (P-27), **I want to** verify custom predicates reject entities during
traversal rather than after, **so that** performance benefit is real.

## US-4.4.6.6 Target Only Valid Enemies
**As a** player (P-23), **I want** targeting to find the nearest valid target, **so that** I do not
target dead, friendly, or immune entities.

## US-4.4.6.7 Combine Bitmask and Component Filters
**As a** game developer (P-15), **I want to** combine CollisionLayers bitmask filtering with
component presence filters, **so that** queries are both layer- and type-aware.

## US-4.4.6.8 Implement QueryFilter Struct
**As an** engine developer (P-26), **I want** QueryFilter to encapsulate bitmask, component filters,
and optional predicate, **so that** all filtering is unified.

## US-4.4.6.9 Test Predicate Filtering Reduces Result Count
**As an** engine tester (P-27), **I want to** verify custom predicate filtering reduces candidate
count during traversal, **so that** performance scales.

## US-4.4.6.10 Filter Queries to Exclude Dead Entities
**As a** game developer (P-15), **I want** queries to exclude entities with a Dead component via
Without filter, **so that** dead entities are ignored.

## US-4.4.6.11 Configure Filters in Visual Editor
**As a** designer (P-5), **I want to** configure query filters in the visual editor, **so that**
spatial query filtering is set up without code.

## US-4.4.6.12 Verify Filter Combinations Work Correctly
**As an** engine tester (P-27), **I want to** test all combinations of bitmask, component, and
custom predicate filters, **so that** all filter paths are validated.
