# Crowds and Tactics

Group movement, flocking, formation control, and tactical coordination.

## What it covers

- Flocking: cohesion, alignment, separation behaviors.
- Boid simulation: velocity-based flocking with simple rules.
- Formation control: holding specific spatial arrangements.
- Leader-follower: units following a designated leader.
- Swarm tactics: units coordinating attacks or defenses.
- Crowd density: density-based pathfinding and movement prioritization.
- Navigation lanes: preferred paths reducing congestion.
- Tactical positioning: cover selection, flanking, retreat routes.
- Threat prioritization: focusing fire on high-value targets.
- Communication: non-verbal coordination via perception and stimulus spread.

## Concepts

### Flocking and Boid Simulation

Flocking models group movement with three simple behaviors: cohesion (steer toward group center),
alignment (match group velocity), separation (avoid crowding neighbors). Boid simulation applies
these rules to each unit; emergent behavior produces schooling, herding, and panic dispersal without
explicit coordination. Flocking is inexpensive per unit, enabling hundreds of units with simple
steering.

### Formation Control

Formations arrange units in specific spatial patterns: line, column, wedge. Each unit maintains a
position offset from a formation reference (often the leader). Leader moves toward an objective;
followers move to maintain offset. Formation slots hold positions; if a unit dies, survivors compact
to fill gaps. Dynamic formations adjust based on terrain: tight columns in corridors, spreads in
open fields.

### Crowd Density and Navigation

Crowd density map tracks unit concentration. High-density areas trigger alternative pathfinding:
units prefer routes avoiding congestion. Navigation lanes (preferred paths) reduce collision and
clustering. Units at intersections prioritize entering lanes, reducing deadlock.

### Tactical Positioning

Tactical AI assesses the environment: finds cover spots (behind walls, breakable objects), identifies
flanking routes (side paths around enemy position), and selects retreat routes. Threat assessment
focuses fire on high-priority targets (leaders, artillery). Units maintain formation while adapting
to threats: spreading if under fire, bunching if moving through constricted areas.

### Swarm Tactics and Communication

Swarm tactics enable simple units to coordinate without centralized control. Boid flocking with
cohesion creates natural clustering around threats. Perception propagation alerts nearby units to
danger. Units moving in formation and using line-of-sight steering produce emergent tactical
behavior without hard-coded scripts.

## How it fits

- See [navigation.md](./navigation.md) for pathfinding and local steering foundations.
- See [perception.md](./perception.md) for threat awareness and stimulus propagation.
- See [../simulation/spatial-awareness.md](../simulation/spatial-awareness.md) for spatial
  density and proximity queries.
