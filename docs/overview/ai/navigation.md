# Navigation

Pathfinding, movement execution, and obstacle avoidance.

## What it covers

- Navigation meshes: simplified geometry for pathfinding.
- Pathfinding: A* search from start to goal on nav mesh.
- Path smoothing: simplifying waypoints to curved paths.
- Local steering: velocity steering around dynamic obstacles.
- Separation: avoiding nearby characters and obstacles.
- Wall sliding: smooth movement along barriers.
- Ramps and elevation: ascending and descending slopes.
- Jump detection: identifying climbable heights.
- Climbing: transitioning from ground to climbing surface.
- Root motion following: character movement driven by animation.

## Concepts

### Navigation Meshes

Navigation meshes (nav meshes) represent walkable surfaces in the world as a graph of connected
polygons. Pathfinding occurs on nav mesh polygons rather than fine voxel grids, reducing search
space. Nav meshes update at authoring time (level design) or dynamically (destruction changes
walkability). Characters query the nav mesh to find paths.

### Pathfinding and Waypoints

Pathfinding (A*) searches on the nav mesh from start position to goal, producing a path of
waypoints. Each waypoint is a nav mesh polygon. The character follows waypoints in sequence. Path
smoothing simplifies waypoints to curved paths, removing intermediate waypoints the character can
skip. Smooth paths reduce jittering and enable natural motion.

### Local Steering and Obstacle Avoidance

Pathfinding provides global waypoints; local steering handles immediate movement. Steering computes
desired velocity toward the next waypoint, then adjusts for obstacles: separation from nearby
characters, avoidance of walls. Velocity steering applies steering corrections to the velocity each
frame without recomputing the global path.

### Elevation and Complex Terrain

Navigation meshes handle slopes: characters walk on ramps at varied elevations. Jump detection
identifies climbable heights: a 1-meter wall is not jumpable; a 0.5-meter wall is. Climbing
transitions the character from ground to wall surface; a separate climbing state machine takes over.
Ledge walking allows characters to balance on narrow ledges.

### Root Motion Following

Characters follow nav mesh paths by moving toward waypoints. Animation-based movement via root
motion enables animation-accurate paths: vault animations propel the character over obstacles.
Root motion combines with steering to follow animations while avoiding obstacles.

## How it fits

- See [perception.md](./perception.md) for sensing goals and threats to navigate toward.
- See [crowds-and-tactics.md](./crowds-and-tactics.md) for group movement and flocking.
- See [../physics/queries-and-vehicles.md](../physics/queries-and-vehicles.md) for raycasting
  and queries.
