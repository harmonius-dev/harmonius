# User Stories -- 7.1 Navigation

## NavMesh Generation

| ID          | Persona                 |
|-------------|-------------------------|
| US-7.1.1.1  | level designer (P-6)    |
| US-7.1.1.2  | game designer (P-5)     |
| US-7.1.1.3  | engine developer (P-26) |
| US-7.1.2.1  | level designer (P-6)    |
| US-7.1.2.2  | engine developer (P-26) |

1. **US-7.1.1.1** -- **As a** level designer (P-6), **I want** the engine to generate navigation
   meshes automatically from world geometry, **so that** I do not need to place navigation volumes
   manually.

2. **US-7.1.1.2** -- **As a** game designer (P-5), **I want** to configure agent radius, height,
   step climb, and slope limits per NPC archetype, **so that** different entity types navigate on
   meshes tailored to their size.

3. **US-7.1.1.3** -- **As an** engine developer (P-26), **I want** to implement NavMesh generation
   via heightfield voxelization, watershed partitioning, and contour tracing, **so that** walkable
   surfaces are extracted from arbitrary world geometry.

4. **US-7.1.2.1** -- **As a** level designer (P-6), **I want** NavMesh tiles to stream in and out
   with the world, **so that** navigation works seamlessly in open-world levels without loading the
   full mesh.

5. **US-7.1.2.2** -- **As an** engine developer (P-26), **I want** NavMesh tiles to align with the
   world streaming grid and load asynchronously, **so that** cross-tile pathfinding produces valid
   continuous paths.

## Pathfinding

| ID          | Persona                 |
|-------------|-------------------------|
| US-7.1.3.1  | game designer (P-5)     |
| US-7.1.3.2  | engine developer (P-26) |
| US-7.1.3.3  | engine developer (P-26) |
| US-7.1.4.1  | engine developer (P-26) |
| US-7.1.5.1  | game designer (P-5)     |
| US-7.1.5.2  | engine developer (P-26) |

1. **US-7.1.3.1** -- **As a** game designer (P-5), **I want** to assign per-area-type cost weights
   so AI agents prefer roads over swamps, **so that** pathfinding reflects contextual route
   preferences.

2. **US-7.1.3.2** -- **As an** engine developer (P-26), **I want** to implement A* search over
   NavMesh polygons with a per-tick CPU budget that batches queries across frames, **so that**
   pathfinding never causes frame-time spikes.

3. **US-7.1.3.3** -- **As an** engine developer (P-26), **I want** to cap concurrent path queries
   per platform tier, **so that** mobile devices stay within their CPU budget.

4. **US-7.1.4.1** -- **As an** engine developer (P-26), **I want** to convert NavMesh polygon
   corridors into minimal waypoint sequences using the funnel algorithm, **so that** paths are tight
   and corner-hugging.

5. **US-7.1.5.1** -- **As a** game designer (P-5), **I want** patrol route paths to use Catmull-Rom
   or Bezier interpolation, **so that** NPC movement along scenic routes looks natural and curved.

6. **US-7.1.5.2** -- **As an** engine developer (P-26), **I want** smoothed path points to be
   validated against the NavMesh, **so that** all interpolated positions lie on walkable polygons.

## Dynamic Obstacles and Links

| ID          | Persona                 |
|-------------|-------------------------|
| US-7.1.6.1  | game designer (P-5)     |
| US-7.1.6.2  | engine developer (P-26) |
| US-7.1.7.1  | level designer (P-6)    |
| US-7.1.7.2  | game designer (P-5)     |

1. **US-7.1.6.1** -- **As a** game designer (P-5), **I want** dynamic obstacles like barricades and
   vehicles to carve the NavMesh at runtime, **so that** agents repath around newly blocked areas.

2. **US-7.1.6.2** -- **As an** engine developer (P-26), **I want** obstacle carving to use
   tile-local re-carving that invalidates only affected polygons, **so that** repath storms are
   localized and bounded.

3. **US-7.1.7.1** -- **As a** level designer (P-6), **I want** to place off-mesh connections for
   jumps, ladders, doors, and swimming transitions, **so that** agents traverse disjoint NavMesh
   regions.

4. **US-7.1.7.2** -- **As a** game designer (P-5), **I want** NavMesh links to carry costs,
   animation tags, and optional preconditions, **so that** the pathfinder evaluates traversal
   feasibility per agent.

## Runtime NavMesh Rebuilding

| ID           | Persona                 |
|--------------|-------------------------|
| US-7.1.8.1   | game designer (P-5)     |
| US-7.1.8.2   | engine developer (P-26) |
| US-7.1.9.1   | engine developer (P-26) |
| US-7.1.10.1  | game designer (P-5)     |
| US-7.1.10.2  | engine developer (P-26) |
| US-7.1.11.1  | level designer (P-6)    |
| US-7.1.11.2  | engine developer (P-26) |

1. **US-7.1.8.1** -- **As a** game designer (P-5), **I want** NavMesh tiles to rebuild incrementally
   when terrain or structures change, **so that** navigation stays valid after world modifications.

2. **US-7.1.8.2** -- **As an** engine developer (P-26), **I want** only affected tiles and their
   neighbors to be revoxelized, **so that** rebuild cost is bounded and proportional to the change
   size.

3. **US-7.1.9.1** -- **As an** engine developer (P-26), **I want** NavMesh generation and rebuild to
   run on background worker threads, **so that** the main simulation tick is never blocked.

4. **US-7.1.10.1** -- **As a** game designer (P-5), **I want** building destruction to automatically
   trigger NavMesh updates, **so that** collapsed buildings open new paths and rubble creates new
   obstacles.

5. **US-7.1.10.2** -- **As an** engine developer (P-26), **I want** the destruction system to emit
   NavMesh invalidation events, **so that** rebuild priority scales with the number of affected
   agents.

6. **US-7.1.11.1** -- **As a** level designer (P-6), **I want** player-placed structures to register
   as NavMesh obstacles with area type overrides, **so that** AI navigates around fortifications and
   ramps correctly.

7. **US-7.1.11.2** -- **As an** engine developer (P-26), **I want** auto-generated NavMesh links for
   stairs and ladders on player structures, **so that** agents can traverse multi-level player-built
   buildings.

## Multi-Agent Navigation

| ID           | Persona                 |
|--------------|-------------------------|
| US-7.1.12.1  | game designer (P-5)     |
| US-7.1.12.2  | engine developer (P-26) |
| US-7.1.13.1  | game designer (P-5)     |
| US-7.1.13.2  | game developer (P-15)   |

1. **US-7.1.12.1** -- **As a** game designer (P-5), **I want** separate NavMesh layers for different
   agent size classes, **so that** humanoids, mounts, and large creatures each navigate on meshes
   matching their dimensions.

2. **US-7.1.12.2** -- **As an** engine developer (P-26), **I want** all NavMesh layers to share the
   same spatial grid, **so that** streaming loads all layers for a region together.

3. **US-7.1.13.1** -- **As a** game designer (P-5), **I want** to modify area costs at runtime
   without rebuilding geometry, **so that** flooded zones, danger areas, and road buffs affect
   pathfinding dynamically.

4. **US-7.1.13.2** -- **As a** game developer (P-15), **I want** per-agent cost overrides based on
   faction, **so that** friendly territory is cheaper to traverse.

## Long-Distance Pathfinding

| ID           | Persona                 |
|--------------|-------------------------|
| US-7.1.14.1  | game designer (P-5)     |
| US-7.1.14.2  | engine developer (P-26) |

1. **US-7.1.14.1** -- **As a** game designer (P-5), **I want** NPCs to plan cross-continent paths
   efficiently, **so that** thousands of agents with long-distance travel goals do not spike
   pathfinding cost.

2. **US-7.1.14.2** -- **As an** engine developer (P-26), **I want** to implement hierarchical
   pathfinding that plans on a coarse tile-connectivity graph first and refines to full NavMesh
   resolution locally, **so that** path cost is bounded regardless of world size.

## Debugging and Visualization

| ID           | Persona                 |
|--------------|-------------------------|
| US-7.1.15.1  | level designer (P-6)    |
| US-7.1.15.2  | engine developer (P-26) |

1. **US-7.1.15.1** -- **As a** level designer (P-6), **I want** to visualize NavMesh polygons, area
   types, obstacle carve regions, and off-mesh links as editor overlays, **so that** I can diagnose
   navigation failures in my levels.

2. **US-7.1.15.2** -- **As an** engine developer (P-26), **I want** NavMesh debug visualization to
   be stripped from shipping builds, **so that** release builds carry no debug overhead.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-7.1.1 | level designer (P-6) |
| US-7.1.10 | game designer (P-5) |
| US-7.1.11 | level designer (P-6) |
| US-7.1.12 | game designer (P-5) |
| US-7.1.13 | game designer (P-5) |
| US-7.1.14 | game designer (P-5) |
| US-7.1.15 | level designer (P-6) |
| US-7.1.2 | level designer (P-6) |
| US-7.1.3 | game designer (P-5) |
| US-7.1.4 | engine developer (P-26) |
| US-7.1.5 | game designer (P-5) |
| US-7.1.6 | game designer (P-5) |
| US-7.1.7 | level designer (P-6) |
| US-7.1.8 | game designer (P-5) |
| US-7.1.9 | engine developer (P-26) |

1. **US-7.1.1** -- **As a** level designer (P-6), **I want** the capabilities defined in sub-stories
   US-7.1.1.1 through US-7.1.1.3 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

2. **US-7.1.10** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.1.10.1 through US-7.1.10.2 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

3. **US-7.1.11** -- **As a** level designer (P-6), **I want** the capabilities defined in
   sub-stories US-7.1.11.1 through US-7.1.11.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

4. **US-7.1.12** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.1.12.1 through US-7.1.12.2 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

5. **US-7.1.13** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.1.13.1 through US-7.1.13.2 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

6. **US-7.1.14** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.1.14.1 through US-7.1.14.2 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

7. **US-7.1.15** -- **As a** level designer (P-6), **I want** the capabilities defined in
   sub-stories US-7.1.15.1 through US-7.1.15.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

8. **US-7.1.2** -- **As a** level designer (P-6), **I want** the capabilities defined in sub-stories
   US-7.1.2.1 through US-7.1.2.2 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

9. **US-7.1.3** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.1.3.1 through US-7.1.3.3 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

10. **US-7.1.4** -- **As a** engine developer (P-26), **I want** the capabilities defined in
    sub-stories
US-7.1.4.1 through US-7.1.4.1 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

11. **US-7.1.5** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
US-7.1.5.1 through US-7.1.5.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

12. **US-7.1.6** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
US-7.1.6.1 through US-7.1.6.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

13. **US-7.1.7** -- **As a** level designer (P-6), **I want** the capabilities defined in
    sub-stories
US-7.1.7.1 through US-7.1.7.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

14. **US-7.1.8** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
US-7.1.8.1 through US-7.1.8.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

15. **US-7.1.9** -- **As a** engine developer (P-26), **I want** the capabilities defined in
    sub-stories
US-7.1.9.1 through US-7.1.9.1 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.
