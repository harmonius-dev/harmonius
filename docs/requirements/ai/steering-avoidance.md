# R-7.2 -- Steering and Avoidance Requirements

## Local Avoidance

1. **R-7.2.1** -- The engine **SHALL** implement Optimal Reciprocal Collision Avoidance so agents
   navigate dense areas without overlapping, producing deadlock-free crowd flow.
   - **Rationale:** ORCA provides mathematically guaranteed collision-free velocities for large
     agent counts in confined spaces.
   - **Verification:** Spawn 200+ agents in a confined area and verify no two agents overlap. Send
     two groups through a narrow passage from opposite directions and verify deadlock-free
     resolution.

2. **R-7.2.2** -- The engine **SHALL** cast short-range feeler rays against static geometry as a
   correction layer after ORCA, handling walls and terrain edges not represented as agents.
   - **Rationale:** ORCA handles agent-agent avoidance but not agent-geometry; feeler rays fill this
     gap.
   - **Verification:** Send agents through corridors and verify no clipping through walls. Verify
     low-LOD agents skip obstacle avoidance as designed.

## Steering Behaviors

1. **R-7.2.3** -- The engine **SHALL** provide seek, flee, arrive, wander, pursuit, and evade as
   composable steering force functions, each returning a force vector.
   - **Rationale:** Composable steering primitives enable rich movement behavior through combination
     rather than per-scenario coding.
   - **Verification:** Verify arrive brings agents to a stop within tolerance. Verify pursuit
     intercepts a linearly moving target faster than naive seek. Verify wander keeps agents within a
     configurable area.

2. **R-7.2.4** -- The engine **SHALL** combine multiple active steering behaviors using either
   weighted-sum blending or a priority pipeline that allocates remaining magnitude to lower-priority
   layers.
   - **Rationale:** Conflicting forces must be resolved without cancellation; priority blending
     ensures high-priority behaviors are satisfied first.
   - **Verification:** Verify priority blending prevents conflicting forces from canceling out.
     Verify weighted blending with known inputs produces the expected weighted sum vector.

## Formation and Group Movement

1. **R-7.2.5** -- The engine **SHALL** assign agents to slots in parameterized formation shapes with
   automatic spacing adjustment when terrain narrows and slot reassignment when members are lost.
   - **Rationale:** Military and escort formations require structured group movement that adapts to
     geometry and casualties.
   - **Verification:** Verify formation spacing matches configured values within tolerance. Test a
     formation through a narrow passage and verify spacing adjusts without clipping. Remove members
     and verify remaining agents reassign to valid slots.

2. **R-7.2.6** -- The engine **SHALL** coordinate steering across named groups using a shared
   velocity goal with cohesion and alignment corrections.
   - **Rationale:** Groups must maintain cohesion during turns without the full computational cost
     of crowd simulation flocking.
   - **Verification:** Navigate a group through turns and verify cohesion within a configurable
     radius. Verify all members converge to a shared velocity goal within a bounded time.
