# R-7.2 -- Steering & Avoidance Requirements

## Local Avoidance

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|-------------|-----------|--------------|
| R-7.2.1 | The engine **SHALL** implement Optimal Reciprocal Collision Avoidance so that agents compute deadlock-free velocities in dense areas, producing smooth crowd flow with zero agent overlap at chokepoints. | [F-7.2.1](../../features/ai/steering-avoidance.md) | Dense crowd scenarios (city streets, battle melees) require collision avoidance that scales to hundreds of agents without deadlocks or interpenetration. | Spawn 200 agents at a chokepoint with opposing destinations. Verify zero agent overlap throughout the simulation. Verify all agents reach their destinations without deadlock. Verify per-agent avoidance computation completes within the configured tick budget. |
| R-7.2.2 | The engine **SHALL** cast short-range feeler rays against static geometry to steer agents away from walls, pillars, and terrain edges, operating as a correction layer after ORCA and maintaining at least agent-radius clearance from all static surfaces. | [F-7.2.2](../../features/ai/steering-avoidance.md) | ORCA handles agent-agent avoidance but not static geometry; feeler rays prevent agents from clipping into walls and terrain features. | Navigate an agent along a wall and verify it maintains at least agent-radius distance from the surface at all times. Navigate through a pillar field and verify no collision with any static geometry. |

## Steering Behaviors

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|-------------|-----------|--------------|
| R-7.2.3 | The engine **SHALL** provide composable steering primitives for seek, flee, arrive (with configurable deceleration radius), wander (constrained random heading), pursuit (predictive interception), and evade (predictive escape), each returning a steering force vector. | [F-7.2.3](../../features/ai/steering-avoidance.md) | Steering primitives are the building blocks for all AI movement; composability enables complex behaviors from simple, testable units. | Test arrive: verify the agent decelerates and stops within 0.1 m of the destination. Test pursuit: verify the agent intercepts a moving target rather than chasing its current position. Test wander: verify the agent's heading changes over time but stays within the configured wander constraint. |
| R-7.2.4 | The engine **SHALL** combine multiple active steering behaviors per agent using either weighted blending or a priority pipeline that allocates remaining steering magnitude to lower-priority layers, preventing conflicting forces from producing zero net movement. | [F-7.2.4](../../features/ai/steering-avoidance.md) | Agents simultaneously pursuing and avoiding obstacles need a blending strategy that prevents stalling when forces cancel out. | Activate simultaneous pursuit and obstacle avoidance behaviors. Verify the agent continues moving (non-zero velocity) when forces partially conflict. Verify higher-priority avoidance is fully satisfied before lower-priority pursuit receives remaining magnitude. |

## Formation and Group Movement

| ID | Requirement | Derived From | Rationale | Verification |
|----|-------------|-------------|-----------|--------------|
| R-7.2.5 | The engine **SHALL** assign agents to slots in parameterized formation shapes (line, wedge, column, circle) with a leader driving the group along the path and followers maintaining offsets via arrival steering, automatically adjusting spacing when terrain narrows. | [F-7.2.5](../../features/ai/steering-avoidance.md) | Tactical games require squads to maintain recognizable formations during movement while adapting to environmental constraints. | Move a 6-agent wedge formation through open terrain and verify all agents maintain their offsets within 0.5 m tolerance. Move the formation through a narrow corridor and verify spacing compresses automatically without agents overlapping. Verify formation re-expands after exiting the narrow terrain. |
| R-7.2.6 | The engine **SHALL** coordinate steering across named groups so members share a common velocity goal, using a lightweight centroid and heading tracker to apply cohesion and alignment corrections that prevent group fragmentation during turns. | [F-7.2.6](../../features/ai/steering-avoidance.md) | Groups of NPCs (patrols, herds, squads) must stay together during navigation without the overhead of full flocking simulation. | Create a 10-agent group and navigate it through a sharp turn. Verify the group's bounding radius does not exceed 1.5x its initial radius during the turn. Verify all agents remain within communication range of the group centroid throughout the maneuver. |

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/ai/steering-avoidance.md](../../user-stories/ai/steering-avoidance.md). Requirements
in this document are derived from those user stories.
