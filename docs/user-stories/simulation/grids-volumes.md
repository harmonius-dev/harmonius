# Grids and Volumes User Stories

## Uniform Grids

| ID        | Persona                 |
|-----------|-------------------------|
| US-17.2.1 | engine developer (P-26) |
| US-17.2.2 | engine developer (P-26) |
| US-17.2.3 | engine developer (P-26) |
| US-17.2.4 | game designer (P-5)     |
| US-17.2.5 | game designer (P-5)     |

1. **US-17.2.1** — **As an** engine developer (P-26), **I want** a generic UniformGrid<T> primitive
   usable with Transform2D and Transform, **so that** fog of war, tactical maps, height fields,
   scent grids, and influence maps share one type.
2. **US-17.2.2** — **As an** engine developer (P-26), **I want** 256x256 grid propagation per tick
   to complete within 1 ms, **so that** per-tick fire spread, fluid flow, and influence updates
   never blow the frame budget.
3. **US-17.2.3** — **As an** engine developer (P-26), **I want** 128-cell line-of-sight queries
   under 0.01 ms each, **so that** AI perception and fog reveal can run many queries per tick
   cheaply.
4. **US-17.2.4** — **As a** game designer (P-5), **I want** flow fields for RTS unit movement
   computed via Dijkstra propagation, **so that** hundreds of units can path toward a goal without
   per-unit A* queries.
5. **US-17.2.5** — **As a** game designer (P-5), **I want** influence map propagation with decay and
   obstacle blocking, **so that** AI decision making can query threat zones and resource attraction
   without bespoke code.

## Voxel Volumes

| ID        | Persona                 |
|-----------|-------------------------|
| US-17.2.6 | game developer (P-15)   |
| US-17.2.7 | engine developer (P-26) |
| US-17.2.8 | engine developer (P-26) |

1. **US-17.2.6** — **As a** game developer (P-15), **I want** VoxelVolume chunk storage with palette
   compression, **so that** block worlds and density fields consume the minimum memory required by
   the present block variety.
2. **US-17.2.7** — **As an** engine developer (P-26), **I want** voxel raycasts over 512 steps to
   complete within 2 ms, **so that** block place/destroy and shooting rays stay interactive at full
   world scale.
3. **US-17.2.8** — **As an** engine developer (P-26), **I want** signed distance fields computed
   from voxel surfaces on dirty regions only, **so that** SDFs feed shadows, AO, and collision
   without recomputing the entire volume.

## GPU Sync

| ID         | Persona                 |
|------------|-------------------------|
| US-17.2.9  | engine developer (P-26) |
| US-17.2.10 | engine developer (P-26) |

1. **US-17.2.9** — **As an** engine developer (P-26), **I want** dirty grid regions to upload to the
   GPU within 1 ms per tick, **so that** renderer overlays reflect simulation state with at most one
   frame of lag.
2. **US-17.2.10** — **As an** engine developer (P-26), **I want** large-scale grid propagation to
   offload to GPU compute when CPU budget is exceeded, **so that** 1024x1024 grids remain
   interactive.

## Hierarchical Grids

| ID         | Persona               |
|------------|-----------------------|
| US-17.2.11 | game developer (P-15) |

1. **US-17.2.11** — **As a** game developer (P-15), **I want** HierarchicalGrid with
   multi-resolution cells, **so that** open-world 2D games can keep detail near the camera and
   coarse state far away without paying for full-resolution grids.

## Area of Interest

| ID         | Persona                 |
|------------|-------------------------|
| US-17.2.12 | engine developer (P-26) |

1. **US-17.2.12** — **As an** engine developer (P-26), **I want** a networking relevancy grid of
   entity sets for area-of-interest filtering, **so that** the replication system can query nearby
   cells per observer instead of scanning the global BVH every tick.
