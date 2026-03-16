# R-7.1 -- Navigation Requirements

## NavMesh Generation

### R-7.1.1 Recast-Style NavMesh Generation

The engine **SHALL** generate polygonal navigation meshes from
world geometry via heightfield voxelization, watershed
partitioning, and contour tracing, with configurable agent
radius, height, step climb, and slope limits per agent
archetype.

- **Derived from:**
  [F-7.1.1](../../features/ai/navigation.md)
- **Rationale:** Automated NavMesh generation from geometry
  eliminates manual placement and ensures meshes match the
  world across dynamic terrain changes.
- **Verification:** Generate a NavMesh for a test scene with
  stairs, ramps, and walls. Verify all walkable surfaces
  within the configured slope limit are covered. Configure
  two agent archetypes (radius 0.3 m, radius 1.0 m) and
  verify the large-radius mesh excludes narrow passages the
  small-radius mesh includes.

### R-7.1.2 NavMesh Tiling and Streaming

The engine **SHALL** divide the NavMesh into fixed-size tiles
aligned to the world streaming grid, loading and unloading
tiles asynchronously as the simulation window moves, with
seamless cross-tile pathfinding that produces valid paths
across tile boundaries.

- **Derived from:**
  [F-7.1.2](../../features/ai/navigation.md)
- **Rationale:** Open-world navigation requires bounded memory
  usage; streaming tiles in and out keeps the active NavMesh
  within memory budget regardless of total world size.
- **Verification:** Load a world with 100+ tiles. Verify only
  tiles within the streaming window are resident in memory.
  Request a path crossing 5 tile boundaries and verify the
  path is valid and continuous. Unload a tile and verify its
  memory is reclaimed.

## Pathfinding

### R-7.1.3 A* Pathfinding on NavMesh

The engine **SHALL** perform A* search over NavMesh polygons
with configurable per-area-type cost weights and a per-tick
CPU budget that batches queries across frames, preventing
frame-time spikes.

- **Derived from:**
  [F-7.1.3](../../features/ai/navigation.md)
- **Rationale:** Cost-weighted pathfinding enables contextual
  route preferences (roads vs. swamps) while batching
  prevents pathfinding from monopolizing frame time.
- **Verification:** Configure lava areas with infinite cost
  and verify no path crosses them. Configure road areas with
  cost 0.5 and verify paths prefer roads over default terrain.
  Issue 200 simultaneous path queries and verify frame time
  does not exceed the configured per-tick budget.

### R-7.1.4 String Pulling (Funnel Algorithm)

The engine **SHALL** convert NavMesh polygon corridors into
minimal waypoint sequences using the funnel algorithm,
producing tight corner-hugging paths with no more than
turn_count + 2 waypoints.

- **Derived from:**
  [F-7.1.4](../../features/ai/navigation.md)
- **Rationale:** Raw polygon corridors produce inefficient
  zig-zag movement; the funnel algorithm yields direct paths
  that look natural and reduce steering corrections.
- **Verification:** Generate a corridor through an L-shaped
  hallway and verify the funnel produces exactly 3 waypoints
  (start, corner, end). Verify all waypoints lie within
  valid NavMesh polygons. Verify waypoint count never exceeds
  turn_count + 2 across 100 randomized paths.

### R-7.1.5 Path Smoothing

The engine **SHALL** post-process waypoint paths with raycast
validation and optional Catmull-Rom or Bezier interpolation,
producing curved trajectories where all interpolated points
lie on valid NavMesh polygons.

- **Derived from:**
  [F-7.1.5](../../features/ai/navigation.md)
- **Rationale:** Smoothed paths eliminate robotic movement and
  improve visual quality for patrol routes and cinematics.
- **Verification:** Smooth a 10-waypoint path with
  Catmull-Rom interpolation. Sample 100 points along the
  curve and verify each lies on a valid NavMesh polygon.
  Verify the smoothed path has fewer waypoints than the
  original.

## Dynamic Obstacles and Links

### R-7.1.6 Dynamic Obstacle Carving

The engine **SHALL** mark NavMesh regions as blocked when
dynamic obstacles appear, using tile-local re-carving that
invalidates only affected polygons and triggers localized
repath requests for agents whose corridors intersect the
modified area.

- **Derived from:**
  [F-7.1.6](../../features/ai/navigation.md)
- **Rationale:** Agents must reroute around runtime obstacles
  (barricades, vehicles) without rebuilding the entire
  NavMesh, which would be prohibitively expensive.
- **Verification:** Place a dynamic obstacle on an agent's
  active path. Verify the agent receives a repath request
  within 2 ticks. Verify only the affected tile's polygons
  are re-carved while neighboring tiles remain unchanged.
  Remove the obstacle and verify the carved region is
  restored.

### R-7.1.7 NavMesh Links (Off-Mesh Connections)

The engine **SHALL** define off-mesh connections between
disjoint NavMesh regions carrying a traversal cost, animation
tag, and optional preconditions, and the pathfinder **SHALL**
evaluate link feasibility per agent before including a link
in a path.

- **Derived from:**
  [F-7.1.7](../../features/ai/navigation.md)
- **Rationale:** Ladders, jump gaps, doors, and swimming
  transitions connect geometry that is not contiguous on the
  NavMesh; preconditions prevent agents from using links they
  cannot physically traverse.
- **Verification:** Create a link with precondition
  "has_climb_ability." Verify an agent with the ability
  includes the link in its path. Verify an agent without the
  ability routes around, producing a longer but valid path.
  Verify the link's cost is included in the total path cost.

## Runtime NavMesh Rebuilding

### R-7.1.8 Incremental Tile Rebuild

The engine **SHALL** rebuild individual NavMesh tiles at
runtime when world geometry changes, revoxelizing only the
affected tile and its immediate neighbors while leaving all
other tiles untouched.

- **Derived from:**
  [F-7.1.8](../../features/ai/navigation.md)
- **Rationale:** Full NavMesh regeneration is too expensive
  for runtime; incremental rebuild bounds the cost to the
  changed region.
- **Verification:** Modify geometry in a single tile. Verify
  only that tile and its direct neighbors are rebuilt. Verify
  all other tiles remain unchanged by comparing their data
  hashes before and after the rebuild.

### R-7.1.9 Background NavMesh Generation

The engine **SHALL** run NavMesh tile generation and rebuild
on background worker threads, never blocking the main
simulation tick. Agents with corridors through tiles under
construction **SHALL** receive a temporary fallback path.

- **Derived from:**
  [F-7.1.9](../../features/ai/navigation.md)
- **Rationale:** Blocking the main thread during NavMesh
  generation causes frame-time spikes that are unacceptable
  in a live game.
- **Verification:** Trigger a tile rebuild and verify
  main-thread frame time does not increase by more than 5%
  during generation. Verify an agent navigating through a
  pending tile receives a fallback path and continues
  moving without stopping.

### R-7.1.10 Destruction-Triggered NavMesh Updates

The engine **SHALL** emit a `NavMeshInvalidation` event when
a destructible entity fractures, enqueuing incremental tile
rebuilds for the affected bounding region with priority
scaled by active agent count in the area.

- **Derived from:**
  [F-7.1.10](../../features/ai/navigation.md)
- **Rationale:** Destroyed buildings open new pathways and
  create new obstacles; the NavMesh must reflect these
  changes for AI to route correctly through destruction.
- **Verification:** Destroy a building and verify a
  `NavMeshInvalidation` event is emitted containing the
  correct bounding region. Verify the affected tiles are
  enqueued for rebuild. Verify a region with 50 active
  agents rebuilds before a region with 5 active agents.

### R-7.1.11 Player-Built Structure Integration

The engine **SHALL** register player-placed structures as
NavMesh obstacles via a `NavMeshObstacle` ECS component,
triggering incremental tile rebuilds on placement and
removal, and auto-generating NavMesh links for stairs and
ladders on walkable structures.

- **Derived from:**
  [F-7.1.11](../../features/ai/navigation.md)
- **Rationale:** AI must respect player fortifications and
  route around or through player-built structures for
  building to have tactical impact.
- **Verification:** Place a wall structure and verify the
  affected tile is rebuilt with the wall carved out. Place a
  ramp structure and verify a NavMesh link is auto-generated
  connecting ground to the elevated platform. Remove the
  structure and verify the NavMesh reverts to its
  pre-placement state.

## Multi-Agent Navigation

### R-7.1.12 Multi-Size Agent NavMeshes

The engine **SHALL** maintain separate NavMesh layers for
each agent size class defined by a `NavMeshAgentConfig`, with
all layers sharing the same spatial tile grid so streaming
loads all layers for a region together.

- **Derived from:**
  [F-7.1.12](../../features/ai/navigation.md)
- **Rationale:** Different-sized agents (humanoid, mount,
  siege golem) need meshes that reflect what geometry they
  physically fit through.
- **Verification:** Generate NavMesh layers for humanoid
  (radius 0.3 m) and large creature (radius 1.5 m). Verify
  the large-creature layer excludes a 1 m wide doorway that
  the humanoid layer includes. Verify streaming a tile loads
  both layers simultaneously.

### R-7.1.13 Dynamic Area Cost Modification

The engine **SHALL** modify NavMesh polygon area costs at
runtime without rebuilding geometry, stored in a
`NavMeshAreaCosts` ECS resource, with support for per-agent
cost overrides for faction-specific routing.

- **Derived from:**
  [F-7.1.13](../../features/ai/navigation.md)
- **Rationale:** Runtime cost changes (flooded zones, danger
  areas, road buffs) must affect pathfinding immediately
  without the expense of a mesh rebuild.
- **Verification:** Increase a zone's area cost at runtime
  and verify agents repath to avoid the zone without any
  mesh rebuild occurring. Apply a per-agent cost override
  making friendly territory cheaper and verify the agent
  prefers that territory over neutral terrain.

## Long-Distance Pathfinding

### R-7.1.14 Hierarchical Pathfinding (HPA*)

The engine **SHALL** build a coarse navigation graph from
NavMesh tile connectivity enabling bounded-cost path queries
across the entire world, refining to full NavMesh resolution
only for tiles the agent is currently traversing.

- **Derived from:**
  [F-7.1.14](../../features/ai/navigation.md)
- **Rationale:** Full-resolution A* across an open world is
  too expensive for thousands of NPCs with cross-continent
  goals; hierarchical planning bounds cost regardless of
  world size.
- **Verification:** Query a path spanning 50 tiles and verify
  it completes within 2x the time of a 5-tile query. Verify
  the hierarchical path produces a valid route when refined
  to full resolution. Verify 1,000 concurrent long-distance
  queries complete within the per-tick pathfinding budget.

## Debugging and Visualization

### R-7.1.15 NavMesh Runtime Visualization

The engine **SHALL** render NavMesh polygons, tile boundaries,
area types, obstacle carve regions, off-mesh links, and
pending rebuild zones as debug overlays, stripped from
shipping builds with zero runtime cost in production.

- **Derived from:**
  [F-7.1.15](../../features/ai/navigation.md)
- **Rationale:** Visual debugging is essential for designers
  to verify walkable regions and diagnose navigation
  failures in complex geometry.
- **Verification:** Enable the debug overlay and verify
  NavMesh polygons, area types, and carve regions are
  visually distinct. Verify agent-path trails appear in the
  overlay. Verify the overlay is absent from a shipping
  build and adds zero CPU or GPU cost.

---

## User Stories

## US-7.1.1 Recast-Style NavMesh Generation

### US-7.1.1.1
As an **engine dev (P-26)**, I want NavMesh generated via
voxelization and watershed partitioning so that polygonal
navigation meshes are built from world geometry automatically.

### US-7.1.1.2
As a **designer (P-5)**, I want configurable agent radius,
height, step climb, and slope so that different entity
archetypes get tailored navigation meshes.

### US-7.1.1.3
As an **engine tester (P-27)**, I want to verify a
large-radius agent's mesh excludes narrow passages so that
per-archetype mesh generation is regression-tested.

---

## US-7.1.2 NavMesh Tiling & Streaming

### US-7.1.2.1
As an **engine dev (P-26)**, I want the NavMesh divided into
fixed-size tiles so that tiles load/unload asynchronously
with world streaming.

### US-7.1.2.2
As a **designer (P-5)**, I want seamless cross-tile
pathfinding so that AI navigates the open world without
hitting tile boundaries.

### US-7.1.2.3
As a **server admin (P-22)**, I want NavMesh tiles streaming
to stay within memory budget so that server memory is bounded
regardless of world size.

### US-7.1.2.4
As an **engine tester (P-27)**, I want to verify only tiles
within the streaming window are loaded so that tile streaming
correctness is regression-tested.

---

## US-7.1.3 A* Pathfinding on NavMesh

### US-7.1.3.1
As a **designer (P-5)**, I want per-area-type cost weights
(road, swamp, lava) so that AI prefers contextually
appropriate routes.

### US-7.1.3.2
As a **designer (P-5)**, I want batched path queries spread
across frames so that pathfinding does not spike frame time.

### US-7.1.3.3
As a **player (P-23)**, I want NPCs to take sensible paths
avoiding hazards so that AI movement looks intelligent.

### US-7.1.3.4
As an **engine dev (P-26)**, I want configurable per-tick CPU
budget for pathfinding so that path queries are bounded on
each platform.

### US-7.1.3.5
As an **engine tester (P-27)**, I want to verify paths never
cross lava (infinite cost) areas so that cost-based routing
correctness is regression-tested.

---

## US-7.1.4 String Pulling (Funnel Algorithm)

### US-7.1.4.1
As an **engine dev (P-26)**, I want corridors converted to
minimal waypoint sequences so that paths are tight and
corner-hugging.

### US-7.1.4.2
As a **player (P-23)**, I want AI to move smoothly around
corners without wide detours so that NPC movement looks
natural.

### US-7.1.4.3
As an **engine tester (P-27)**, I want to verify the funnel
produces no more than turn count + 2 waypoints so that
waypoint minimality is regression-tested.

---

## US-7.1.5 Path Smoothing

### US-7.1.5.1
As a **designer (P-5)**, I want Catmull-Rom or Bezier
interpolation for curved paths so that patrol routes look
natural and aesthetic.

### US-7.1.5.2
As a **player (P-23)**, I want NPCs to move with smooth
curved trajectories so that movement does not look robotic.

### US-7.1.5.3
As an **engine tester (P-27)**, I want to verify all smoothed
path points lie on valid NavMesh polygons so that path
validity after smoothing is regression-tested.

---

## US-7.1.6 Dynamic Obstacle Carving

### US-7.1.6.1
As a **designer (P-5)**, I want dynamic obstacles
(barricades, vehicles) to block paths so that AI reacts to
world changes.

### US-7.1.6.2
As a **player (P-23)**, I want AI to reroute around newly
placed obstacles so that NPC behavior reflects the current
world state.

### US-7.1.6.3
As an **engine dev (P-26)**, I want tile-local re-carving for
affected polygons only so that obstacle carving is efficient.

### US-7.1.6.4
As an **engine tester (P-27)**, I want to verify agents
repath when their corridor is carved so that dynamic repath
triggering is regression-tested.

---

## US-7.1.7 NavMesh Links (Off-Mesh Connections)

### US-7.1.7.1
As a **designer (P-5)**, I want off-mesh links for jumping,
climbing, and swimming so that AI can traverse disjoint
NavMesh regions.

### US-7.1.7.2
As a **designer (P-5)**, I want links with cost, animation
tag, and preconditions so that the planner evaluates link
feasibility per agent.

### US-7.1.7.3
As a **player (P-23)**, I want AI to climb ladders and jump
gaps when capable so that NPC navigation uses all available
traversal options.

### US-7.1.7.4
As an **engine tester (P-27)**, I want to verify an agent
without the required ability avoids the link so that
precondition enforcement is regression-tested.

---

## US-7.1.8 Incremental Tile Rebuild

### US-7.1.8.1
As an **engine dev (P-26)**, I want individual tiles rebuilt
when geometry changes so that only affected areas are
revoxelized.

### US-7.1.8.2
As a **designer (P-5)**, I want terrain deformation to update
navigation automatically so that AI adapts to player-caused
world changes.

### US-7.1.8.3
As an **engine tester (P-27)**, I want to verify only the
affected tile and neighbors are rebuilt so that rebuild scope
is regression-tested.

---

## US-7.1.9 Background NavMesh Generation

### US-7.1.9.1
As an **engine dev (P-26)**, I want NavMesh generation on
background worker threads so that the main simulation tick is
never blocked.

### US-7.1.9.2
As a **player (P-23)**, I want AI to continue moving during
NavMesh rebuilds so that navigation never visibly stalls.

### US-7.1.9.3
As an **engine tester (P-27)**, I want to verify main-thread
frame time does not increase more than 5% during generation
so that background generation performance is
regression-tested.

---

## US-7.1.10 Destruction-Triggered NavMesh Updates

### US-7.1.10.1
As a **designer (P-5)**, I want collapsed buildings to open
new pathways so that destruction creates new navigation
opportunities.

### US-7.1.10.2
As a **player (P-23)**, I want AI to use paths through
destroyed structures so that destruction visibly changes AI
behavior.

### US-7.1.10.3
As an **engine dev (P-26)**, I want rebuild priority scaled by
active agent count so that heavily-trafficked areas rebuild
first.

### US-7.1.10.4
As an **engine tester (P-27)**, I want to verify
NavMeshInvalidation events trigger rebuilds for the affected
region so that destruction-to-rebuild pipeline is
regression-tested.

---

## US-7.1.11 Player-Built Structure Integration

### US-7.1.11.1
As a **designer (P-5)**, I want player-placed structures to
register as NavMesh obstacles so that AI routes around player
buildings.

### US-7.1.11.2
As a **designer (P-5)**, I want auto-generated NavMesh links
for stairs and ladders so that AI navigates player-built
vertical structures.

### US-7.1.11.3
As a **player (P-23)**, I want AI to respect my
fortifications and route around walls so that building has
tactical impact on AI behavior.

### US-7.1.11.4
As an **engine tester (P-27)**, I want to verify a ramp
structure generates a NavMesh link connecting ground to
elevated platform so that auto-link generation is
regression-tested.

---

## US-7.1.12 Multi-Size Agent NavMeshes

### US-7.1.12.1
As a **designer (P-5)**, I want separate NavMesh layers for
humanoid, mount, and large creature agents so that each agent
navigates geometry it physically fits through.

### US-7.1.12.2
As an **engine dev (P-26)**, I want all layers sharing the
same spatial tile grid so that streaming loads all layers for
a region together.

### US-7.1.12.3
As an **engine tester (P-27)**, I want to verify a large
agent fails to pathfind through a narrow doorway so that
per-size mesh filtering is regression-tested.

---

## US-7.1.13 Dynamic Area Cost Modification

### US-7.1.13.1
As a **designer (P-5)**, I want to modify area costs at
runtime without mesh rebuild so that flooded zones and danger
areas affect pathing immediately.

### US-7.1.13.2
As a **designer (P-5)**, I want per-agent cost overrides for
faction-specific routing so that friendly territory is
cheaper for allied agents.

### US-7.1.13.3
As an **engine tester (P-27)**, I want to verify agents
repath when area costs change without any mesh rebuild
occurring so that runtime cost modification is
regression-tested.

---

## US-7.1.14 Hierarchical Pathfinding (HPA*)

### US-7.1.14.1
As an **engine dev (P-26)**, I want coarse-graph planning for
long-distance paths so that pathfinding cost is bounded
regardless of world size.

### US-7.1.14.2
As a **server admin (P-22)**, I want thousands of NPCs with
cross-continent goals so that server-side AI scales to MMO
populations.

### US-7.1.14.3
As an **engine tester (P-27)**, I want to verify a 50-tile
query completes within 2x of a 5-tile query so that
bounded-cost pathfinding is regression-tested.

---

## US-7.1.15 NavMesh Runtime Visualization

### US-7.1.15.1
As a **designer (P-5)**, I want debug overlays showing
NavMesh polygons and area types so that I can verify walkable
regions visually.

### US-7.1.15.2
As a **designer (P-5)**, I want agent-path trails in the
overlay so that I can trace AI movement for debugging.

### US-7.1.15.3
As an **engine dev (P-26)**, I want visualization stripped
from shipping builds so that debug overlays have no runtime
cost in production.

### US-7.1.15.4
As an **engine tester (P-27)**, I want to verify obstacle
carve regions are visually distinct in the overlay so that
visualization accuracy is regression-tested.
