# 7.7 — Crowd Simulation

## Flocking

### F-7.7.1 Flocking Behaviors (Separation, Alignment, Cohesion)

Implements Reynolds flocking with three weighted forces: separation (avoid crowding neighbors),
alignment (steer toward average heading), and cohesion (steer toward average position). Weights
are tunable per crowd archetype to produce varied group behaviors — tight military columns, loose
wandering herds, or scattered panicked civilians.

- **Requirements:** R-7.7.1
- **Dependencies:** F-7.2.3
- **Platform notes:** None

## Flow Fields

### F-7.7.2 Flow Field Navigation

Generates a 2D grid-based flow field from a goal position using Dijkstra propagation. Each cell
stores a direction vector pointing along the cheapest path to the goal. Thousands of crowd agents
sample the field directly instead of running individual A* queries, enabling mass movement at
constant per-agent cost.

- **Requirements:** R-7.7.2
- **Dependencies:** F-7.1.1
- **Platform notes:** None

### F-7.7.3 Flow Field Streaming & Caching

Tiles flow fields to match the world streaming grid and caches recently computed fields keyed by
(goal cell, cost layer version). Stale fields are invalidated when dynamic obstacles modify the
cost grid. Multiple goals share cached fields when their regions overlap, reducing recomputation
in city districts with many points of interest.

- **Requirements:** R-7.7.3
- **Dependencies:** F-7.7.2, F-7.1.2
- **Platform notes:** None

## Mass Simulation

### F-7.7.4 Mass Entity Simulation

Processes crowd NPCs as lightweight entities with minimal per-agent state (position, velocity,
archetype ID, flow field sample). Avoids full behavior tree evaluation by driving movement entirely
from flow fields and flocking forces. Supports tens of thousands of ambient city inhabitants on the
server with a bounded CPU footprint.

- **Requirements:** R-7.7.4
- **Dependencies:** F-7.7.1, F-7.7.2
- **Platform notes:** None

### F-7.7.5 AI Level of Detail (Processing Budget)

Assigns each AI agent a LOD tier based on distance to the nearest player and gameplay relevance.
High-LOD agents run full behavior trees and per-tick perception; mid-LOD agents tick at reduced
frequency; low-LOD agents use flow-field-only movement. A global budget scheduler distributes
available CPU time across tiers to maintain a stable frame rate.

- **Requirements:** R-7.7.5
- **Dependencies:** F-7.3.1, F-7.7.4
- **Platform notes:** None

### F-7.7.6 Density Management

Monitors crowd density per spatial cell and enforces configurable caps to prevent unrealistic
pileups at chokepoints, event locations, and spawn areas. When density exceeds the threshold,
overflow agents are redirected to alternative routes or despawned if they are ambient crowd entities,
maintaining both simulation plausibility and server performance.

- **Requirements:** R-7.7.6
- **Dependencies:** F-7.7.4, F-7.2.1
- **Platform notes:** None
