# User Stories -- 7.2 Steering and Avoidance

## Local Avoidance

| ID          | Persona                 |
|-------------|-------------------------|
| US-7.2.1.1  | game designer (P-5)     |
| US-7.2.1.2  | game designer (P-5)     |
| US-7.2.1.3  | engine developer (P-26) |
| US-7.2.1.4  | engine developer (P-26) |
| US-7.2.2.1  | game designer (P-5)     |
| US-7.2.2.2  | engine developer (P-26) |
| US-7.2.2.3  | engine developer (P-26) |

1. **US-7.2.1.1** -- **As a** game designer (P-5), **I want** to configure ORCA neighbor count and
   query radius per agent archetype, **so that** avoidance quality balances with performance for
   each NPC type.

2. **US-7.2.1.2** -- **As a** game designer (P-5), **I want** agents in crowded chokepoints to flow
   smoothly without overlapping, **so that** dense crowd movement looks realistic.

3. **US-7.2.1.3** -- **As an** engine developer (P-26), **I want** to implement Optimal Reciprocal
   Collision Avoidance with velocity obstacle computation, **so that** agents produce deadlock-free
   movement in dense areas.

4. **US-7.2.1.4** -- **As an** engine developer (P-26), **I want** to batch ORCA neighbor queries
   using the shared spatial index, **so that** avoidance scales efficiently with agent count.

5. **US-7.2.2.1** -- **As a** game designer (P-5), **I want** agents to steer around walls, pillars,
   and terrain edges without clipping, **so that** indoor movement feels natural and physically
   plausible.

6. **US-7.2.2.2** -- **As an** engine developer (P-26), **I want** to cast short-range feeler rays
   against static geometry as a final correction layer after ORCA, **so that** agent-geometry
   avoidance complements agent-agent avoidance.

7. **US-7.2.2.3** -- **As an** engine developer (P-26), **I want** low-LOD agents to skip obstacle
   avoidance entirely, **so that** distant agents consume minimal CPU resources.

## Steering Behaviors

| ID          | Persona                 |
|-------------|-------------------------|
| US-7.2.3.1  | game designer (P-5)     |
| US-7.2.3.2  | game designer (P-5)     |
| US-7.2.3.3  | engine developer (P-26) |
| US-7.2.3.4  | engine developer (P-26) |
| US-7.2.4.1  | game designer (P-5)     |
| US-7.2.4.2  | game designer (P-5)     |
| US-7.2.4.3  | engine developer (P-26) |
| US-7.2.4.4  | engine developer (P-26) |

1. **US-7.2.3.1** -- **As a** game designer (P-5), **I want** to assign steering behaviors like
   seek, flee, arrive, wander, pursuit, and evade to agent archetypes, **so that** each NPC type
   moves distinctively.

2. **US-7.2.3.2** -- **As a** game designer (P-5), **I want** to tune the arrive behavior's
   deceleration radius per agent, **so that** NPCs slow down gracefully when reaching their
   destination.

3. **US-7.2.3.3** -- **As an** engine developer (P-26), **I want** each steering behavior to return
   a force vector, **so that** the blending system can combine multiple active behaviors per agent.

4. **US-7.2.3.4** -- **As an** engine developer (P-26), **I want** to use SIMD-optimized vector math
   for steering computations, **so that** thousands of agents can be processed per tick efficiently.

5. **US-7.2.4.1** -- **As a** game designer (P-5), **I want** to choose between weighted blending
   and priority pipeline modes per agent archetype, **so that** different NPCs use the strategy best
   suited to their role.

6. **US-7.2.4.2** -- **As a** game designer (P-5), **I want** to visualize each steering force
   component and the final blended vector as debug arrows, **so that** I can diagnose conflicting
   behaviors.

7. **US-7.2.4.3** -- **As an** engine developer (P-26), **I want** to implement both weighted-sum
   and priority pipeline blending, **so that** high-priority forces are satisfied first without
   cancellation.

8. **US-7.2.4.4** -- **As an** engine developer (P-26), **I want** to limit active steering layers
   to 3 on mobile, **so that** per-agent blending cost stays within the mobile CPU budget.

## Formation and Group Movement

| ID          | Persona                 |
|-------------|-------------------------|
| US-7.2.5.1  | game designer (P-5)     |
| US-7.2.5.2  | game designer (P-5)     |
| US-7.2.5.3  | engine developer (P-26) |
| US-7.2.5.4  | engine developer (P-26) |
| US-7.2.6.1  | game designer (P-5)     |
| US-7.2.6.2  | game designer (P-5)     |
| US-7.2.6.3  | engine developer (P-26) |

1. **US-7.2.5.1** -- **As a** game designer (P-5), **I want** to assign formation shapes like line,
   wedge, column, and circle to agent groups, **so that** squads move in organized patterns.

2. **US-7.2.5.2** -- **As a** game designer (P-5), **I want** formations to automatically adjust
   spacing when terrain narrows, **so that** agents do not clip through walls to maintain position.

3. **US-7.2.5.3** -- **As an** engine developer (P-26), **I want** to implement formation shapes as
   parameterized slot layouts with a leader and follower offsets, **so that** shapes are data-driven
   and extensible.

4. **US-7.2.5.4** -- **As an** engine developer (P-26), **I want** formations to automatically
   reassign slots when members are lost, **so that** remaining members maintain a coherent reduced
   formation.

5. **US-7.2.6.1** -- **As a** game designer (P-5), **I want** to tune cohesion and alignment
   correction strength per group, **so that** tight squads stay close while loose patrols allow more
   spread.

6. **US-7.2.6.2** -- **As a** game designer (P-5), **I want** patrol groups to stay together when
   rounding corners, **so that** group movement looks cohesive without fragmentation.

7. **US-7.2.6.3** -- **As an** engine developer (P-26), **I want** to maintain a lightweight group
   centroid and heading tracker for cohesion corrections, **so that** group steering is cheaper than
   full crowd simulation.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-7.2.1 | game designer (P-5) |
| US-7.2.2 | game designer (P-5) |
| US-7.2.3 | game designer (P-5) |
| US-7.2.4 | game designer (P-5) |
| US-7.2.5 | game designer (P-5) |
| US-7.2.6 | game designer (P-5) |

1. **US-7.2.1** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.2.1.1 through US-7.2.1.4 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

2. **US-7.2.2** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.2.2.1 through US-7.2.2.3 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

3. **US-7.2.3** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.2.3.1 through US-7.2.3.4 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

4. **US-7.2.4** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.2.4.1 through US-7.2.4.4 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

5. **US-7.2.5** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.2.5.1 through US-7.2.5.4 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.

6. **US-7.2.6** -- **As a** game designer (P-5), **I want** the capabilities defined in sub-stories
   US-7.2.6.1 through US-7.2.6.3 combined into a single umbrella feature, **so that** I have a
   coherent parent story covering the refined child stories.
