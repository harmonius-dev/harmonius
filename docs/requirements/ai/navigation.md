# R-7.1 -- Navigation Requirements

## NavMesh Generation

| ID      | Derived From                               |
|---------|--------------------------------------------|
| R-7.1.1 | [F-7.1.1](../../features/ai/navigation.md) |
| R-7.1.2 | [F-7.1.2](../../features/ai/navigation.md) |

1. **R-7.1.1** — The engine **SHALL** generate polygonal navigation meshes from world geometry via
   heightfield voxelization, watershed partitioning, and contour tracing, with configurable agent
   radius, height, step climb, and slope limits per agent archetype.
   - **Rationale:** Automated NavMesh generation from geometry eliminates manual placement and
     ensures meshes match the world across dynamic terrain changes.
   - **Verification:** Generate a NavMesh for a test scene with stairs, ramps, and walls. Verify all
     walkable surfaces within the configured slope limit are covered. Configure two agent archetypes
     (radius 0.3 m, radius 1.0 m) and verify the large-radius mesh excludes narrow passages the
     small-radius mesh includes.
2. **R-7.1.2** — The engine **SHALL** divide the NavMesh into fixed-size tiles aligned to the world
   streaming grid, loading and unloading tiles asynchronously as the simulation window moves, with
   seamless cross-tile pathfinding that produces valid paths across tile boundaries.
   - **Rationale:** Open-world navigation requires bounded memory usage; streaming tiles in and out
     keeps the active NavMesh within memory budget regardless of total world size.
   - **Verification:** Load a world with 100+ tiles. Verify only tiles within the streaming window
     are resident in memory. Request a path crossing 5 tile boundaries and verify the path is valid
     and continuous. Unload a tile and verify its memory is reclaimed.

## Pathfinding

| ID      | Derived From                               |
|---------|--------------------------------------------|
| R-7.1.3 | [F-7.1.3](../../features/ai/navigation.md) |
| R-7.1.4 | [F-7.1.4](../../features/ai/navigation.md) |
| R-7.1.5 | [F-7.1.5](../../features/ai/navigation.md) |

1. **R-7.1.3** — The engine **SHALL** perform A* search over NavMesh polygons with configurable
   per-area-type cost weights and a per-tick CPU budget that batches queries across frames,
   preventing frame-time spikes.
   - **Rationale:** Cost-weighted pathfinding enables contextual route preferences (roads vs.
     swamps) while batching prevents pathfinding from monopolizing frame time.
   - **Verification:** Configure lava areas with infinite cost and verify no path crosses them.
     Configure road areas with cost 0.5 and verify paths prefer roads over default terrain. Issue
     200 simultaneous path queries and verify frame time does not exceed the configured per-tick
     budget.
2. **R-7.1.4** — The engine **SHALL** convert NavMesh polygon corridors into minimal waypoint
   sequences using the funnel algorithm, producing tight corner-hugging paths with no more than
   turn_count + 2 waypoints.
   - **Rationale:** Raw polygon corridors produce inefficient zig-zag movement; the funnel algorithm
     yields direct paths that look natural and reduce steering corrections.
   - **Verification:** Generate a corridor through an L-shaped hallway and verify the funnel
     produces exactly 3 waypoints (start, corner, end). Verify all waypoints lie within valid
     NavMesh polygons. Verify waypoint count never exceeds turn_count + 2 across 100 randomized
     paths.
3. **R-7.1.5** — The engine **SHALL** post-process waypoint paths with raycast validation and
   optional Catmull-Rom or Bezier interpolation, producing curved trajectories where all
   interpolated points lie on valid NavMesh polygons.
   - **Rationale:** Smoothed paths eliminate robotic movement and improve visual quality for patrol
     routes and cinematics.
   - **Verification:** Smooth a 10-waypoint path with Catmull-Rom interpolation. Sample 100 points
     along the curve and verify each lies on a valid NavMesh polygon. Verify the smoothed path has
     fewer waypoints than the original.

## Dynamic Obstacles and Links

| ID      | Derived From                               |
|---------|--------------------------------------------|
| R-7.1.6 | [F-7.1.6](../../features/ai/navigation.md) |
| R-7.1.7 | [F-7.1.7](../../features/ai/navigation.md) |

1. **R-7.1.6** — The engine **SHALL** mark NavMesh regions as blocked when dynamic obstacles appear,
   using tile-local re-carving that invalidates only affected polygons and triggers localized repath
   requests for agents whose corridors intersect the modified area.
   - **Rationale:** Agents must reroute around runtime obstacles (barricades, vehicles) without
     rebuilding the entire NavMesh, which would be prohibitively expensive.
   - **Verification:** Place a dynamic obstacle on an agent's active path. Verify the agent receives
     a repath request within 2 ticks. Verify only the affected tile's polygons are re-carved while
     neighboring tiles remain unchanged. Remove the obstacle and verify the carved region is
     restored.
2. **R-7.1.7** — The engine **SHALL** define off-mesh connections between disjoint NavMesh regions
   carrying a traversal cost, animation tag, and optional preconditions, and the pathfinder
   **SHALL** evaluate link feasibility per agent before including a link in a path.
   - **Rationale:** Ladders, jump gaps, doors, and swimming transitions connect geometry that is not
     contiguous on the NavMesh; preconditions prevent agents from using links they cannot physically
     traverse.
   - **Verification:** Create a link with precondition "has_climb_ability." Verify an agent with the
     ability includes the link in its path. Verify an agent without the ability routes around,
     producing a longer but valid path. Verify the link's cost is included in the total path cost.

## Runtime NavMesh Rebuilding

| ID       | Derived From                                |
|----------|---------------------------------------------|
| R-7.1.8  | [F-7.1.8](../../features/ai/navigation.md)  |
| R-7.1.9  | [F-7.1.9](../../features/ai/navigation.md)  |
| R-7.1.10 | [F-7.1.10](../../features/ai/navigation.md) |
| R-7.1.11 | [F-7.1.11](../../features/ai/navigation.md) |

1. **R-7.1.8** — The engine **SHALL** rebuild individual NavMesh tiles at runtime when world
   geometry changes, revoxelizing only the affected tile and its immediate neighbors while leaving
   all other tiles untouched.
   - **Rationale:** Full NavMesh regeneration is too expensive for runtime; incremental rebuild
     bounds the cost to the changed region.
   - **Verification:** Modify geometry in a single tile. Verify only that tile and its direct
     neighbors are rebuilt. Verify all other tiles remain unchanged by comparing their data hashes
     before and after the rebuild.
2. **R-7.1.9** — The engine **SHALL** run NavMesh tile generation and rebuild on background worker
   threads, never blocking the main simulation tick. Agents with corridors through tiles under
   construction **SHALL** receive a temporary fallback path.
   - **Rationale:** Blocking the game loop thread during NavMesh generation causes frame-time spikes
     that are unacceptable in a live game.
   - **Verification:** Trigger a tile rebuild and verify game-loop-thread frame time does not
     increase by more than 5% during generation. Verify an agent navigating through a pending tile
     receives a fallback path and continues moving without stopping.
3. **R-7.1.10** — The engine **SHALL** emit a `NavMeshInvalidation` event when a destructible entity
   fractures, enqueuing incremental tile rebuilds for the affected bounding region with priority
   scaled by active agent count in the area.
   - **Rationale:** Destroyed buildings open new pathways and create new obstacles; the NavMesh must
     reflect these changes for AI to route correctly through destruction.
   - **Verification:** Destroy a building and verify a `NavMeshInvalidation` event is emitted
     containing the correct bounding region. Verify the affected tiles are enqueued for rebuild.
     Verify a region with 50 active agents rebuilds before a region with 5 active agents.
4. **R-7.1.11** — The engine **SHALL** register player-placed structures as NavMesh obstacles via a
   `NavMeshObstacle` ECS component, triggering incremental tile rebuilds on placement and removal,
   and auto-generating NavMesh links for stairs and ladders on walkable structures.
   - **Rationale:** AI must respect player fortifications and route around or through player-built
     structures for building to have tactical impact.
   - **Verification:** Place a wall structure and verify the affected tile is rebuilt with the wall
     carved out. Place a ramp structure and verify a NavMesh link is auto-generated connecting
     ground to the elevated platform. Remove the structure and verify the NavMesh reverts to its
     pre-placement state.

## Multi-Agent Navigation

| ID       | Derived From                                |
|----------|---------------------------------------------|
| R-7.1.12 | [F-7.1.12](../../features/ai/navigation.md) |
| R-7.1.13 | [F-7.1.13](../../features/ai/navigation.md) |

1. **R-7.1.12** — The engine **SHALL** maintain separate NavMesh layers for each agent size class
   defined by a `NavMeshAgentConfig`, with all layers sharing the same spatial tile grid so
   streaming loads all layers for a region together.
   - **Rationale:** Different-sized agents (humanoid, mount, siege golem) need meshes that reflect
     what geometry they physically fit through.
   - **Verification:** Generate NavMesh layers for humanoid (radius 0.3 m) and large creature
     (radius 1.5 m). Verify the large-creature layer excludes a 1 m wide doorway that the humanoid
     layer includes. Verify streaming a tile loads both layers simultaneously.
2. **R-7.1.13** — The engine **SHALL** modify NavMesh polygon area costs at runtime without
   rebuilding geometry, stored in a `NavMeshAreaCosts` ECS resource, with support for per-agent cost
   overrides for faction-specific routing.
   - **Rationale:** Runtime cost changes (flooded zones, danger areas, road buffs) must affect
     pathfinding immediately without the expense of a mesh rebuild.
   - **Verification:** Increase a zone's area cost at runtime and verify agents repath to avoid the
     zone without any mesh rebuild occurring. Apply a per-agent cost override making friendly
     territory cheaper and verify the agent prefers that territory over neutral terrain.

## Long-Distance Pathfinding

| ID       | Derived From                                |
|----------|---------------------------------------------|
| R-7.1.14 | [F-7.1.14](../../features/ai/navigation.md) |

1. **R-7.1.14** — The engine **SHALL** build a coarse navigation graph from NavMesh tile
   connectivity enabling bounded-cost path queries across the entire world, refining to full NavMesh
   resolution only for tiles the agent is currently traversing.
   - **Rationale:** Full-resolution A* across an open world is too expensive for thousands of NPCs
     with cross-continent goals; hierarchical planning bounds cost regardless of world size.
   - **Verification:** Query a path spanning 50 tiles and verify it completes within 2x the time of
     a 5-tile query. Verify the hierarchical path produces a valid route when refined to full
     resolution. Verify 1,000 concurrent long-distance queries complete within the per-tick
     pathfinding budget.

## Debugging and Visualization

| ID       | Derived From                                |
|----------|---------------------------------------------|
| R-7.1.15 | [F-7.1.15](../../features/ai/navigation.md) |

1. **R-7.1.15** — The engine **SHALL** render NavMesh polygons, tile boundaries, area types,
   obstacle carve regions, off-mesh links, and pending rebuild zones as debug overlays, stripped
   from shipping builds with zero runtime cost in production.
   - **Rationale:** Visual debugging is essential for designers to verify walkable regions and
     diagnose navigation failures in complex geometry.
   - **Verification:** Enable the debug overlay and verify NavMesh polygons, area types, and carve
     regions are visually distinct. Verify agent-path trails appear in the overlay. Verify the
     overlay is absent from a shipping build and adds zero CPU or GPU cost.

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/ai/navigation.md](../../user-stories/ai/navigation.md). Requirements in this document
are derived from those user stories.
