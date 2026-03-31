# Spatial Indexing User Stories

## Acceleration Structures

| ID       | Persona                 |
|----------|-------------------------|
| US-1.9.1 | engine developer (P-26) |
| US-1.9.2 | engine tester (P-27)    |
| US-1.9.3 | engine developer (P-26) |
| US-1.9.4 | engine tester (P-27)    |
| US-1.9.5 | game developer (P-15)   |

1. **US-1.9.1** — **As an** engine developer (P-26), **I want** a single BVH maintained as an ECS
   resource shared across physics, rendering, networking, AI, and gameplay, **so that** all
   subsystems use one spatial structure instead of maintaining redundant copies.
2. **US-1.9.2** — **As an** engine tester (P-27), **I want** to benchmark BVH memory usage and query
   latency at platform entity limits, **so that** I can verify the spatial index meets memory and
   performance budgets.
3. **US-1.9.3** — **As an** engine developer (P-26), **I want** the BVH updated incrementally using
   ECS change detection on Transform components, **so that** update cost is proportional to moved
   entities rather than total entity count.
4. **US-1.9.4** — **As an** engine tester (P-27), **I want** to benchmark incremental BVH update
   cost with varying ratios of stationary to moving entities, **so that** I can verify update time
   scales with moved entity count.
5. **US-1.9.5** — **As a** game developer (P-15), **I want** an optional coarse-grained spatial
   index (uniform grid or octree) alongside the BVH, **so that** network area-of-interest filtering
   and AI crowd density queries use the most efficient structure.

## Query Interface

| ID       | Persona                 |
|----------|-------------------------|
| US-1.9.6 | game developer (P-15)   |
| US-1.9.7 | engine developer (P-26) |
| US-1.9.8 | engine tester (P-27)    |

1. **US-1.9.6** — **As a** game developer (P-15), **I want** a single API for ray casts, shape
   casts, overlap tests, nearest-neighbor, and frustum queries that accepts ECS component filters,
   **so that** all subsystems use a consistent query interface.
2. **US-1.9.7** — **As an** engine developer (P-26), **I want** to submit batches of spatial queries
   that execute in parallel across worker threads with SIMD acceleration, **so that** server ticks
   with hundreds of AI and ability checks complete within frame budgets.
3. **US-1.9.8** — **As an** engine tester (P-27), **I want** to benchmark batch spatial query
   throughput at platform limits, **so that** I can verify server tick budgets are met during
   worst-case processing.

## Consumer Integration

| ID        | Persona                 |
|-----------|-------------------------|
| US-1.9.9  | engine developer (P-26) |
| US-1.9.10 | engine tester (P-27)    |
| US-1.9.11 | engine developer (P-26) |
| US-1.9.12 | game developer (P-15)   |
| US-1.9.13 | engine tester (P-27)    |
| US-1.9.14 | game developer (P-15)   |
| US-1.9.15 | engine tester (P-27)    |

1. **US-1.9.9** — **As an** engine developer (P-26), **I want** the physics broadphase to read from
   the shared BVH filtered by collision layers, **so that** collision detection uses the same
   spatial data as rendering and no separate broadphase structure is allocated.
2. **US-1.9.10** — **As an** engine tester (P-27), **I want** to verify that physics broadphase and
   rendering culling produce consistent results from the shared BVH within the same frame,
   **so that** no entity is visible but non-collidable or vice versa.
3. **US-1.9.11** — **As an** engine developer (P-26), **I want** frustum culling to read from the
   shared BVH for all views, **so that** the renderer shares incremental updates and does not
   rebuild a separate culling hierarchy.
4. **US-1.9.12** — **As a** game developer (P-15), **I want** the network relevancy system to use
   the shared spatial index for area-of-interest filtering, **so that** only entities within each
   player's relevancy radius are replicated.
5. **US-1.9.13** — **As an** engine tester (P-27), **I want** to verify that network relevancy sets
   are spatially consistent with physics and rendering, **so that** replicated entities match what
   the player sees and collides with.
6. **US-1.9.14** — **As a** game developer (P-15), **I want** AI perception systems and gameplay
   spatial queries to read from the shared index, **so that** AI and gameplay systems agree with
   physics on entity positions.
7. **US-1.9.15** — **As an** engine tester (P-27), **I want** to verify that AI sight cone and
   hearing radius queries return exactly the entities within the perception volume, **so that** AI
   does not perceive entities outside its range.
