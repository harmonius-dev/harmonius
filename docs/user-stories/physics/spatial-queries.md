# User Stories — 4.4 Spatial Queries

## F-4.4.1 Ray Casting

**US-4.4.1a** As a gameplay programmer, I want to cast a ray and receive the hit entity,
position, normal, distance, and collision layers so that I can implement line-of-sight
checks, weapon targeting, and ground detection with a single API call.

**US-4.4.1b** As a designer, I want ray casts to use the shared spatial index so that
line-of-sight queries account for all collidable objects in the scene without maintaining
a separate query structure.

## F-4.4.2 Shape Casting (Sweep Tests)

**US-4.4.2a** As a gameplay programmer, I want to sweep a sphere, capsule, or box along a
direction and get the first contact so that character controllers, projectile hit volumes,
and safe-movement checks work reliably for shapes wider than a ray.

**US-4.4.2b** As a QA engineer, I want to verify that shape casts report the correct first
contact point for all supported shape types so that no false-negative misses occur during
gameplay.

## F-4.4.3 Overlap Tests

**US-4.4.3a** As a gameplay programmer, I want to query all entities overlapping a given
shape at a position so that area-of-effect abilities, proximity checks, and trigger
implementations have zero false negatives.

## F-4.4.4 Closest Point Queries

**US-4.4.4a** As a gameplay programmer, I want to compute the closest point on any collider
surface to a world-space position so that I can implement distance-based triggers, magnet
effects, and attachment-point snapping.

## F-4.4.5 Batch Query Execution

**US-4.4.5a** As a gameplay programmer, I want to submit batches of spatial queries for
parallel execution so that server-side AI agents issuing hundreds of ray casts per tick do
not bottleneck the frame.

**US-4.4.5b** As a QA engineer, I want to benchmark batch queries and confirm they scale
linearly with core count so that server hardware is fully utilized during peak load.

## F-4.4.6 Query Filtering and Custom Predicates

**US-4.4.6a** As a gameplay programmer, I want to attach a `QueryFilter` with collision
layer masks, ECS structural filters, and custom predicates to any spatial query so that
irrelevant entities are excluded before narrowphase, avoiding expensive post-filtering.

**US-4.4.6b** As a designer, I want queries to support "find nearest enemy not behind
cover" semantics so that AI perception and ability targeting work correctly without
additional gameplay filtering passes.
