# 7.2 — Steering & Avoidance

## Local Avoidance

### F-7.2.1 RVO/ORCA Local Avoidance

Implements Optimal Reciprocal Collision Avoidance so thousands of agents navigate dense areas
without overlapping. Each agent computes a velocity that lies outside the combined velocity
obstacles of its neighbors, producing smooth, deadlock-free crowd flow in city streets and
chokepoints.

- **Requirements:** R-7.2.1
- **Dependencies:** F-7.1.3
- **Platform notes:** Mobile limits ORCA neighbor count to 4 (vs. 12 on desktop) and uses a
  smaller query radius to reduce per-agent avoidance cost.

### F-7.2.2 Obstacle Avoidance (Static Geometry)

Casts short-range feeler rays against nearby static geometry to steer agents away from walls,
pillars, and terrain edges. Operates as a final correction layer after ORCA, handling geometry
that is not represented as other agents.

- **Requirements:** R-7.2.2
- **Dependencies:** F-7.2.1
- **Platform notes:** Mobile uses 2 feeler rays (vs. 5 on desktop) for reduced raycast cost.
  Low-LOD agents skip obstacle avoidance entirely.

## Steering Behaviors

### F-7.2.3 Core Steering Behaviors

Provides a library of composable steering primitives: seek, flee, arrive (with deceleration
radius), wander (constrained random heading), pursuit (predictive interception), and evade
(predictive escape). Each returns a steering force vector that is blended by a priority or
weighted-sum combiner.

- **Requirements:** R-7.2.3
- **Dependencies:** None
- **Platform notes:** Lightweight vector math; runs identically on all platforms.

### F-7.2.4 Steering Behavior Blending & Priority

Combines multiple active steering behaviors per agent using either weighted blending or a priority
pipeline that allocates remaining steering magnitude to lower-priority layers. Prevents conflicting
forces from canceling out in complex scenarios like simultaneous pursuit and obstacle avoidance.

- **Requirements:** R-7.2.4
- **Dependencies:** F-7.2.3
- **Platform notes:** Mobile limits active steering layers to 3 (vs. 5 on desktop) to
  reduce per-agent blending cost.

## Formation & Group Movement

### F-7.2.5 Formation Movement

Assigns agents to slots in a parameterized formation shape (line, wedge, column, circle, custom).
A formation leader drives the group along the path while followers maintain their offsets using
arrival steering, automatically adjusting spacing when terrain narrows or the formation is
partially destroyed.

- **Requirements:** R-7.2.5
- **Dependencies:** F-7.2.3, F-7.2.1
- **Platform notes:** Mobile limits formation size to 4 slots (vs. 8+ on desktop). Custom
  formation shapes disabled on mobile; only preset shapes available.

### F-7.2.6 Group Steering & Cohesion

Coordinates steering across a named group so members share a common velocity goal and avoid
fragmenting during turns. Uses a lightweight group centroid and heading tracker to apply cohesion
and alignment corrections without the full flocking model used for crowds.

- **Requirements:** R-7.2.6
- **Dependencies:** F-7.2.3
- **Platform notes:** Group size limited by the platform crowd budget (see F-7.7.4).
  Centroid computation is lightweight and scales linearly with group size.
