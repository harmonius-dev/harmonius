# R-7.1 -- Navigation User Stories

## US-7.1.1 Recast-Style NavMesh Generation

### US-7.1.1.1
As an **engine dev (P-26)**, I want NavMesh generated via voxelization and watershed
partitioning
so that polygonal navigation meshes are built from world geometry automatically.

### US-7.1.1.2
As a **designer (P-5)**, I want configurable agent radius, height, step climb, and slope
so that different entity archetypes get tailored navigation meshes.

### US-7.1.1.3
As an **engine tester (P-27)**, I want to verify a large-radius agent's mesh excludes
narrow passages
so that per-archetype mesh generation is regression-tested.

---

## US-7.1.2 NavMesh Tiling & Streaming

### US-7.1.2.1
As an **engine dev (P-26)**, I want the NavMesh divided into fixed-size tiles
so that tiles load/unload asynchronously with world streaming.

### US-7.1.2.2
As a **designer (P-5)**, I want seamless cross-tile pathfinding
so that AI navigates the open world without hitting tile boundaries.

### US-7.1.2.3
As a **server admin (P-22)**, I want NavMesh tiles streaming to stay within memory budget
so that server memory is bounded regardless of world size.

### US-7.1.2.4
As an **engine tester (P-27)**, I want to verify only tiles within the streaming window
are loaded
so that tile streaming correctness is regression-tested.

---

## US-7.1.3 A* Pathfinding on NavMesh

### US-7.1.3.1
As a **designer (P-5)**, I want per-area-type cost weights (road, swamp, lava)
so that AI prefers contextually appropriate routes.

### US-7.1.3.2
As a **designer (P-5)**, I want batched path queries spread across frames
so that pathfinding does not spike frame time.

### US-7.1.3.3
As a **player (P-23)**, I want NPCs to take sensible paths avoiding hazards
so that AI movement looks intelligent.

### US-7.1.3.4
As an **engine dev (P-26)**, I want configurable per-tick CPU budget for pathfinding
so that path queries are bounded on each platform.

### US-7.1.3.5
As an **engine tester (P-27)**, I want to verify paths never cross lava (infinite cost)
areas
so that cost-based routing correctness is regression-tested.

---

## US-7.1.4 String Pulling (Funnel Algorithm)

### US-7.1.4.1
As an **engine dev (P-26)**, I want corridors converted to minimal waypoint sequences
so that paths are tight and corner-hugging.

### US-7.1.4.2
As a **player (P-23)**, I want AI to move smoothly around corners without wide detours
so that NPC movement looks natural.

### US-7.1.4.3
As an **engine tester (P-27)**, I want to verify the funnel produces no more than
turn count + 2 waypoints
so that waypoint minimality is regression-tested.

---

## US-7.1.5 Path Smoothing

### US-7.1.5.1
As a **designer (P-5)**, I want Catmull-Rom or Bezier interpolation for curved paths
so that patrol routes look natural and aesthetic.

### US-7.1.5.2
As a **player (P-23)**, I want NPCs to move with smooth curved trajectories
so that movement does not look robotic.

### US-7.1.5.3
As an **engine tester (P-27)**, I want to verify all smoothed path points lie on valid
NavMesh polygons
so that path validity after smoothing is regression-tested.

---

## US-7.1.6 Dynamic Obstacle Carving

### US-7.1.6.1
As a **designer (P-5)**, I want dynamic obstacles (barricades, vehicles) to block paths
so that AI reacts to world changes.

### US-7.1.6.2
As a **player (P-23)**, I want AI to reroute around newly placed obstacles
so that NPC behavior reflects the current world state.

### US-7.1.6.3
As an **engine dev (P-26)**, I want tile-local re-carving for affected polygons only
so that obstacle carving is efficient.

### US-7.1.6.4
As an **engine tester (P-27)**, I want to verify agents repath when their corridor is
carved
so that dynamic repath triggering is regression-tested.

---

## US-7.1.7 NavMesh Links (Off-Mesh Connections)

### US-7.1.7.1
As a **designer (P-5)**, I want off-mesh links for jumping, climbing, and swimming
so that AI can traverse disjoint NavMesh regions.

### US-7.1.7.2
As a **designer (P-5)**, I want links with cost, animation tag, and preconditions
so that the planner evaluates link feasibility per agent.

### US-7.1.7.3
As a **player (P-23)**, I want AI to climb ladders and jump gaps when capable
so that NPC navigation uses all available traversal options.

### US-7.1.7.4
As an **engine tester (P-27)**, I want to verify an agent without the required ability
avoids the link
so that precondition enforcement is regression-tested.

---

## US-7.1.8 Incremental Tile Rebuild

### US-7.1.8.1
As an **engine dev (P-26)**, I want individual tiles rebuilt when geometry changes
so that only affected areas are revoxelized.

### US-7.1.8.2
As a **designer (P-5)**, I want terrain deformation to update navigation automatically
so that AI adapts to player-caused world changes.

### US-7.1.8.3
As an **engine tester (P-27)**, I want to verify only the affected tile and neighbors
are rebuilt
so that rebuild scope is regression-tested.

---

## US-7.1.9 Background NavMesh Generation

### US-7.1.9.1
As an **engine dev (P-26)**, I want NavMesh generation on background worker threads
so that the main simulation tick is never blocked.

### US-7.1.9.2
As a **player (P-23)**, I want AI to continue moving during NavMesh rebuilds
so that navigation never visibly stalls.

### US-7.1.9.3
As an **engine tester (P-27)**, I want to verify main-thread frame time does not
increase more than 5% during generation
so that background generation performance is regression-tested.

---

## US-7.1.10 Destruction-Triggered NavMesh Updates

### US-7.1.10.1
As a **designer (P-5)**, I want collapsed buildings to open new pathways
so that destruction creates new navigation opportunities.

### US-7.1.10.2
As a **player (P-23)**, I want AI to use paths through destroyed structures
so that destruction visibly changes AI behavior.

### US-7.1.10.3
As an **engine dev (P-26)**, I want rebuild priority scaled by active agent count
so that heavily-trafficked areas rebuild first.

### US-7.1.10.4
As an **engine tester (P-27)**, I want to verify NavMeshInvalidation events trigger
rebuilds for the affected region
so that destruction-to-rebuild pipeline is regression-tested.

---

## US-7.1.11 Player-Built Structure Integration

### US-7.1.11.1
As a **designer (P-5)**, I want player-placed structures to register as NavMesh obstacles
so that AI routes around player buildings.

### US-7.1.11.2
As a **designer (P-5)**, I want auto-generated NavMesh links for stairs and ladders
so that AI navigates player-built vertical structures.

### US-7.1.11.3
As a **player (P-23)**, I want AI to respect my fortifications and route around walls
so that building has tactical impact on AI behavior.

### US-7.1.11.4
As an **engine tester (P-27)**, I want to verify a ramp structure generates a NavMesh
link connecting ground to elevated platform
so that auto-link generation is regression-tested.

---

## US-7.1.12 Multi-Size Agent NavMeshes

### US-7.1.12.1
As a **designer (P-5)**, I want separate NavMesh layers for humanoid, mount, and large
creature agents
so that each agent navigates geometry it physically fits through.

### US-7.1.12.2
As an **engine dev (P-26)**, I want all layers sharing the same spatial tile grid
so that streaming loads all layers for a region together.

### US-7.1.12.3
As an **engine tester (P-27)**, I want to verify a large agent fails to pathfind through
a narrow doorway
so that per-size mesh filtering is regression-tested.

---

## US-7.1.13 Dynamic Area Cost Modification

### US-7.1.13.1
As a **designer (P-5)**, I want to modify area costs at runtime without mesh rebuild
so that flooded zones and danger areas affect pathing immediately.

### US-7.1.13.2
As a **designer (P-5)**, I want per-agent cost overrides for faction-specific routing
so that friendly territory is cheaper for allied agents.

### US-7.1.13.3
As an **engine tester (P-27)**, I want to verify agents repath when area costs change
without any mesh rebuild occurring
so that runtime cost modification is regression-tested.

---

## US-7.1.14 Hierarchical Pathfinding (HPA*)

### US-7.1.14.1
As an **engine dev (P-26)**, I want coarse-graph planning for long-distance paths
so that pathfinding cost is bounded regardless of world size.

### US-7.1.14.2
As a **server admin (P-22)**, I want thousands of NPCs with cross-continent goals
so that server-side AI scales to MMO populations.

### US-7.1.14.3
As an **engine tester (P-27)**, I want to verify a 50-tile query completes within
2x of a 5-tile query
so that bounded-cost pathfinding is regression-tested.

---

## US-7.1.15 NavMesh Runtime Visualization

### US-7.1.15.1
As a **designer (P-5)**, I want debug overlays showing NavMesh polygons and area types
so that I can verify walkable regions visually.

### US-7.1.15.2
As a **designer (P-5)**, I want agent-path trails in the overlay
so that I can trace AI movement for debugging.

### US-7.1.15.3
As an **engine dev (P-26)**, I want visualization stripped from shipping builds
so that debug overlays have no runtime cost in production.

### US-7.1.15.4
As an **engine tester (P-27)**, I want to verify obstacle carve regions are visually
distinct in the overlay
so that visualization accuracy is regression-tested.
