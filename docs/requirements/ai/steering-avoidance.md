# R-7.2 -- Steering & Avoidance Requirements

## Local Avoidance

### R-7.2.1 RVO/ORCA Local Avoidance

The engine **SHALL** implement Optimal Reciprocal Collision Avoidance so that agents compute
deadlock-free velocities in dense areas, producing smooth crowd flow with zero agent overlap at
chokepoints.

- **Derived from:**
  [F-7.2.1](../../features/ai/steering-avoidance.md)
- **Rationale:** Dense crowd scenarios (city streets, battle melees) require collision avoidance
  that scales to hundreds of agents without deadlocks or interpenetration.
- **Verification:** Spawn 200 agents at a chokepoint with opposing destinations. Verify zero agent
  overlap throughout the simulation. Verify all agents reach their destinations without deadlock.
  Verify per-agent avoidance computation completes within the configured tick budget.

### R-7.2.2 Obstacle Avoidance (Static Geometry)

The engine **SHALL** cast short-range feeler rays against static geometry to steer agents away from
walls, pillars, and terrain edges, operating as a correction layer after ORCA and maintaining at
least agent-radius clearance from all static surfaces.

- **Derived from:**
  [F-7.2.2](../../features/ai/steering-avoidance.md)
- **Rationale:** ORCA handles agent-agent avoidance but not static geometry; feeler rays prevent
  agents from clipping into walls and terrain features.
- **Verification:** Navigate an agent along a wall and verify it maintains at least agent-radius
  distance from the surface at all times. Navigate through a pillar field and verify no collision
  with any static geometry.

## Steering Behaviors

### R-7.2.3 Core Steering Behaviors

The engine **SHALL** provide composable steering primitives for seek, flee, arrive (with
configurable deceleration radius), wander (constrained random heading), pursuit (predictive
interception), and evade (predictive escape), each returning a steering force vector.

- **Derived from:**
  [F-7.2.3](../../features/ai/steering-avoidance.md)
- **Rationale:** Steering primitives are the building blocks for all AI movement; composability
  enables complex behaviors from simple, testable units.
- **Verification:** Test arrive: verify the agent decelerates and stops within 0.1 m of the
  destination. Test pursuit: verify the agent intercepts a moving target rather than chasing its
  current position. Test wander: verify the agent's heading changes over time but stays within the
  configured wander constraint.

### R-7.2.4 Steering Behavior Blending and Priority

The engine **SHALL** combine multiple active steering behaviors per agent using either weighted
blending or a priority pipeline that allocates remaining steering magnitude to lower-priority
layers, preventing conflicting forces from producing zero net movement.

- **Derived from:**
  [F-7.2.4](../../features/ai/steering-avoidance.md)
- **Rationale:** Agents simultaneously pursuing and avoiding obstacles need a blending strategy that
  prevents stalling when forces cancel out.
- **Verification:** Activate simultaneous pursuit and obstacle avoidance behaviors. Verify the agent
  continues moving (non-zero velocity) when forces partially conflict. Verify higher-priority
  avoidance is fully satisfied before lower-priority pursuit receives remaining magnitude.

## Formation and Group Movement

### R-7.2.5 Formation Movement

The engine **SHALL** assign agents to slots in parameterized formation shapes (line, wedge, column,
circle) with a leader driving the group along the path and followers maintaining offsets via arrival
steering, automatically adjusting spacing when terrain narrows.

- **Derived from:**
  [F-7.2.5](../../features/ai/steering-avoidance.md)
- **Rationale:** Tactical games require squads to maintain recognizable formations during movement
  while adapting to environmental constraints.
- **Verification:** Move a 6-agent wedge formation through open terrain and verify all agents
  maintain their offsets within 0.5 m tolerance. Move the formation through a narrow corridor and
  verify spacing compresses automatically without agents overlapping. Verify formation re-expands
  after exiting the narrow terrain.

### R-7.2.6 Group Steering and Cohesion

The engine **SHALL** coordinate steering across named groups so members share a common velocity
goal, using a lightweight centroid and heading tracker to apply cohesion and alignment corrections
that prevent group fragmentation during turns.

- **Derived from:**
  [F-7.2.6](../../features/ai/steering-avoidance.md)
- **Rationale:** Groups of NPCs (patrols, herds, squads) must stay together during navigation
  without the overhead of full flocking simulation.
- **Verification:** Create a 10-agent group and navigate it through a sharp turn. Verify the group's
  bounding radius does not exceed 1.5x its initial radius during the turn. Verify all agents remain
  within communication range of the group centroid throughout the maneuver.

---

## User Stories

## US-7.2.1 RVO/ORCA Local Avoidance

### US-7.2.1.1

As an **engine dev (P-26)**, I want ORCA-based collision avoidance so that agents compute
deadlock-free velocities in dense areas.

### US-7.2.1.2

As a **player (P-23)**, I want crowd NPCs flowing smoothly through chokepoints so that city streets
feel alive and believable.

### US-7.2.1.3

As a **designer (P-5)**, I want deadlock-free crowd flow so that agents do not get stuck in
face-to-face standoffs.

### US-7.2.1.4

As an **engine tester (P-27)**, I want to verify zero agent overlap at chokepoints with 200+ agents
so that collision avoidance is regression-tested under load.

---

## US-7.2.2 Obstacle Avoidance (Static Geometry)

### US-7.2.2.1

As an **engine dev (P-26)**, I want feeler rays against static geometry so that agents steer away
from walls and terrain edges.

### US-7.2.2.2

As a **player (P-23)**, I want NPCs to avoid walking into walls and pillars so that AI movement
looks spatially aware.

### US-7.2.2.3

As an **engine tester (P-27)**, I want to verify agents maintain radius distance from all static
surfaces so that obstacle avoidance is regression-tested.

---

## US-7.2.3 Core Steering Behaviors

### US-7.2.3.1

As a **designer (P-5)**, I want seek, flee, arrive, wander, pursuit, and evade primitives so that I
can compose varied AI movement patterns.

### US-7.2.3.2

As a **player (P-23)**, I want NPCs to pursue with predictive interception so that enemies
anticipate my movement.

### US-7.2.3.3

As a **player (P-23)**, I want NPCs to decelerate smoothly near destinations so that AI does not
overshoot waypoints.

### US-7.2.3.4

As an **engine tester (P-27)**, I want to verify arrive behavior stops within tolerance of the
destination so that arrival precision is regression-tested.

---

## US-7.2.4 Steering Behavior Blending & Priority

### US-7.2.4.1

As a **designer (P-5)**, I want weighted blending of multiple steering behaviors so that agents
combine pursuit and avoidance smoothly.

### US-7.2.4.2

As an **engine dev (P-26)**, I want priority pipeline for steering magnitude allocation so that
high-priority behaviors are satisfied first.

### US-7.2.4.3

As an **engine tester (P-27)**, I want to verify conflicting forces do not produce zero net movement
so that priority blending prevents stalling.

---

## US-7.2.5 Formation Movement

### US-7.2.5.1

As a **designer (P-5)**, I want parameterized formation shapes (line, wedge, column) so that squads
maintain tactical arrangements.

### US-7.2.5.2

As a **player (P-23)**, I want squads to move in formation so that military units look organized.

### US-7.2.5.3

As a **designer (P-5)**, I want formations adjusting spacing in narrow terrain so that squads adapt
to environment constraints.

### US-7.2.5.4

As an **engine tester (P-27)**, I want to verify formation offsets stay within 0.5m tolerance so
that formation coherence is regression-tested.

---

## US-7.2.6 Group Steering & Cohesion

### US-7.2.6.1

As a **designer (P-5)**, I want group cohesion so members share velocity goals so that named groups
move together without fragmenting.

### US-7.2.6.2

As a **player (P-23)**, I want groups of NPCs to stay together during turns so that group movement
looks coordinated.

### US-7.2.6.3

As an **engine tester (P-27)**, I want to verify group bounding radius does not exceed 1.5x initial
radius after turns so that group cohesion is regression-tested.
