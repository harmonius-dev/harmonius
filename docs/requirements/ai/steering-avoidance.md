# R-7.2 -- Steering & Avoidance Requirements

## R-7.2.1 RVO/ORCA Local Avoidance

The engine **SHALL** implement Optimal Reciprocal Collision Avoidance (ORCA) so that agents
compute velocities outside the combined velocity obstacles of their neighbors, producing
deadlock-free crowd flow without agent overlap.

- **Derived from:** [F-7.2.1](../../features/ai/steering-avoidance.md)
- **Rationale:** Naive steering causes agents to overlap or deadlock in dense areas; ORCA
  provides mathematically guaranteed collision-free velocities.
- **Verification:** Simulate 200 agents converging on a single chokepoint. Verify zero agent
  overlaps (minimum pairwise distance >= combined radii) over a 60-second run. Confirm no
  agent remains deadlocked (stationary for more than 3 seconds while its goal is reachable).

## R-7.2.2 Obstacle Avoidance (Static Geometry)

The engine **SHALL** cast short-range feeler rays against nearby static geometry to steer agents
away from walls, pillars, and terrain edges, operating as a final correction layer after ORCA.

- **Derived from:** [F-7.2.2](../../features/ai/steering-avoidance.md)
- **Rationale:** ORCA handles agent-agent avoidance but not static geometry; feeler rays
  prevent agents from clipping through walls and environmental features.
- **Verification:** Navigate an agent along a corridor with pillars and 90-degree corners.
  Verify the agent maintains at least its configured radius distance from all static geometry
  surfaces throughout the path. Confirm no geometry penetration occurs in a 1000-frame
  simulation.

## R-7.2.3 Core Steering Behaviors

The engine **SHALL** provide composable steering behavior primitives (seek, flee, arrive with
deceleration radius, wander, pursuit with predictive interception, and evade with predictive
escape), each returning a steering force vector for blending.

- **Derived from:** [F-7.2.3](../../features/ai/steering-avoidance.md)
- **Rationale:** A library of reusable steering primitives avoids reimplementing common movement
  patterns for every NPC type and enables consistent, composable behavior.
- **Verification:** For each behavior: (1) seek -- verify agent reaches target within expected
  time. (2) flee -- verify agent moves away from threat. (3) arrive -- verify agent decelerates
  to zero velocity within the deceleration radius. (4) wander -- verify heading variance over
  100 ticks falls within configured bounds. (5) pursuit -- verify interception angle reduces
  time-to-contact vs. naive seek. (6) evade -- verify agent escapes the pursuer's projected
  intercept cone.

## R-7.2.4 Steering Behavior Blending and Priority

The engine **SHALL** combine multiple active steering behaviors per agent using either weighted
blending or a priority pipeline that allocates remaining steering magnitude to lower-priority
layers, preventing conflicting forces from canceling out.

- **Derived from:** [F-7.2.4](../../features/ai/steering-avoidance.md)
- **Rationale:** Agents performing simultaneous pursuit and obstacle avoidance need a structured
  combiner to prevent opposing forces from producing zero net movement.
- **Verification:** Configure an agent with high-priority obstacle avoidance and low-priority
  seek. Place a wall between the agent and its target. Verify the agent steers around the wall
  (avoidance dominates) and still reaches the target (seek contributes residual force). Confirm
  the resultant velocity magnitude never exceeds the agent's max speed.

## R-7.2.5 Formation Movement

The engine **SHALL** assign agents to slots in a parameterized formation shape (line, wedge,
column, circle, custom), with a formation leader driving the group along a path while followers
maintain their offsets using arrival steering, automatically adjusting spacing when terrain
narrows.

- **Derived from:** [F-7.2.5](../../features/ai/steering-avoidance.md)
- **Rationale:** Coordinated group movement in formation is essential for military units,
  patrols, and escort scenarios to appear organized rather than chaotic.
- **Verification:** Create a 5-agent wedge formation and move it along a path that narrows
  from 10 m to 3 m width. Verify all agents maintain their relative offsets (within 0.5 m
  tolerance) in the wide section. Verify the formation compresses in the narrow section with
  no agent overlaps. Remove one agent and confirm remaining agents redistribute to fill the
  gap.

## R-7.2.6 Group Steering and Cohesion

The engine **SHALL** coordinate steering across a named group so members share a common velocity
goal, applying cohesion and alignment corrections via a group centroid and heading tracker to
prevent fragmentation during turns.

- **Derived from:** [F-7.2.6](../../features/ai/steering-avoidance.md)
- **Rationale:** Without group-level coordination, groups fragment during sharp turns as
  outer agents overshoot and inner agents bunch up.
- **Verification:** Move a 10-agent group through three 90-degree turns. Measure the group's
  bounding radius after each turn and verify it does not exceed 1.5x the initial radius. Verify
  no agent falls more than 2x the configured spacing distance behind the group centroid.
