# User Stories — 7.2 Steering & Avoidance

## F-7.2.1 — RVO/ORCA Local Avoidance

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.2.1.1  | designer (P-5)          | F-7.2.1  | R-7.2.1      |
| US-7.2.1.2  | designer (P-5)          | F-7.2.1  | R-7.2.1      |
| US-7.2.1.3  | designer (P-5)          | F-7.2.1  | R-7.2.1      |
| US-7.2.1.4  | player (P-23)           | F-7.2.1  | R-7.2.1      |
| US-7.2.1.5  | player (P-23)           | F-7.2.1  | R-7.2.1      |
| US-7.2.1.6  | player (P-23)           | F-7.2.1  | R-7.2.1      |
| US-7.2.1.7  | engine developer (P-26) | F-7.2.1  | R-7.2.1      |
| US-7.2.1.8  | engine developer (P-26) | F-7.2.1  | R-7.2.1      |
| US-7.2.1.9  | engine developer (P-26) | F-7.2.1  | R-7.2.1      |
| US-7.2.1.10 | engine tester (P-27)    | F-7.2.1  | R-7.2.1      |
| US-7.2.1.11 | engine tester (P-27)    | F-7.2.1  | R-7.2.1      |
| US-7.2.1.12 | engine tester (P-27)    | F-7.2.1  | R-7.2.1      |

1. **US-7.2.1.1** — I want to configure the ORCA neighbor count and query radius per agent archetype
   - **Acceptance:** avoidance quality balances with performance for each NPC type
2. **US-7.2.1.2** — I want to spawn many agents at a narrow chokepoint and observe smooth flow
   without pileups
   - **Acceptance:** I can tune ORCA parameters for dense areas
3. **US-7.2.1.3** — I want to adjust how aggressively agents avoid each other (tight for guards,
   loose for civilians)
   - **Acceptance:** different NPCs have distinct movement feel
4. **US-7.2.1.4** — I want NPCs in crowded city streets to navigate smoothly without walking through
   each other
   - **Acceptance:** crowds feel realistic
5. **US-7.2.1.5** — I want groups of guards to file through doorways one at a time instead of
   clumping
   - **Acceptance:** tight spaces feel physically plausible
6. **US-7.2.1.6** — I want approaching enemies to spread out instead of stacking on top of each
   other
   - **Acceptance:** combat encounters look tactically distributed
7. **US-7.2.1.7** — I want to implement Optimal Reciprocal Collision Avoidance with velocity
   obstacle computation
   - **Acceptance:** agents produce deadlock-free movement
8. **US-7.2.1.8** — I want to batch neighbor queries using the shared spatial index
   - **Acceptance:** ORCA scales efficiently with agent count
9. **US-7.2.1.9** — I want to limit ORCA neighbor count to 4 on mobile (vs. 12 on desktop)
   - **Acceptance:** per-agent avoidance cost fits within the mobile CPU budget
10. **US-7.2.1.10** — I want to verify that no two agents overlap positions in a stress test with
    200+ agents in a confined area
    - **Acceptance:** ORCA prevents interpenetration
11. **US-7.2.1.11** — I want to verify that two groups approaching from opposite directions through
    a narrow passage resolve without deadlock
    - **Acceptance:** ORCA is robust
12. **US-7.2.1.12** — I want to benchmark per-agent ORCA computation time across platform tiers
    - **Acceptance:** avoidance cost stays within the per-agent CPU budget. ---

## F-7.2.2 — Obstacle Avoidance (Static Geometry)

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.2.2.1  | designer (P-5)          | F-7.2.2  | R-7.2.2      |
| US-7.2.2.2  | designer (P-5)          | F-7.2.2  | R-7.2.2      |
| US-7.2.2.3  | designer (P-5)          | F-7.2.2  | R-7.2.2      |
| US-7.2.2.4  | player (P-23)           | F-7.2.2  | R-7.2.2      |
| US-7.2.2.5  | player (P-23)           | F-7.2.2  | R-7.2.2      |
| US-7.2.2.6  | player (P-23)           | F-7.2.2  | R-7.2.2      |
| US-7.2.2.7  | engine developer (P-26) | F-7.2.2  | R-7.2.2      |
| US-7.2.2.8  | engine developer (P-26) | F-7.2.2  | R-7.2.2      |
| US-7.2.2.9  | engine developer (P-26) | F-7.2.2  | R-7.2.2      |
| US-7.2.2.10 | engine tester (P-27)    | F-7.2.2  | R-7.2.2      |
| US-7.2.2.11 | engine tester (P-27)    | F-7.2.2  | R-7.2.2      |
| US-7.2.2.12 | engine tester (P-27)    | F-7.2.2  | R-7.2.2      |

1. **US-7.2.2.1** — I want to configure the number of feeler rays per agent archetype
   - **Acceptance:** avoidance precision matches the agent's movement requirements
2. **US-7.2.2.2** — I want to test agents walking through corridors and verify they do not clip
   through walls
   - **Acceptance:** geometry avoidance works for indoor environments
3. **US-7.2.2.3** — I want to tune feeler ray length per agent size class
   - **Acceptance:** large agents detect walls earlier and steer with wider clearance
4. **US-7.2.2.4** — I want NPCs to smoothly steer around walls and pillars without bumping into them
   - **Acceptance:** indoor movement feels natural
5. **US-7.2.2.5** — I want AI agents to steer away from cliff edges and terrain drops
   - **Acceptance:** they do not walk off ledges unnaturally
6. **US-7.2.2.6** — I want NPCs to slide around sharp geometry corners without getting stuck
   - **Acceptance:** movement remains fluid in complex interiors
7. **US-7.2.2.7** — I want to cast short-range feeler rays against nearby static geometry
   - **Acceptance:** agents receive correction forces before contacting walls
8. **US-7.2.2.8** — I want to apply static obstacle avoidance as a final correction layer after ORCA
   - **Acceptance:** both agent-agent and agent-geometry avoidance work
9. **US-7.2.2.9** — I want to reduce feeler rays to 2 on mobile (vs. 5 on desktop)
   - **Acceptance:** raycast cost fits within the mobile per-agent budget
10. **US-7.2.2.10** — I want to verify that agents never clip through static geometry on a reference
    level with tight corridors
    - **Acceptance:** obstacle avoidance is airtight
11. **US-7.2.2.11** — I want to confirm that low-LOD agents skip obstacle avoidance entirely as
    designed
    - **Acceptance:** the LOD system correctly gates expensive behaviors
12. **US-7.2.2.12** — I want to benchmark feeler ray computation cost per agent across platforms
    - **Acceptance:** the raycast budget scales appropriately. ---

## F-7.2.3 — Core Steering Behaviors

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.2.3.1  | designer (P-5)          | F-7.2.3  | R-7.2.3      |
| US-7.2.3.2  | designer (P-5)          | F-7.2.3  | R-7.2.3      |
| US-7.2.3.3  | designer (P-5)          | F-7.2.3  | R-7.2.3      |
| US-7.2.3.4  | player (P-23)           | F-7.2.3  | R-7.2.3      |
| US-7.2.3.5  | player (P-23)           | F-7.2.3  | R-7.2.3      |
| US-7.2.3.6  | player (P-23)           | F-7.2.3  | R-7.2.3      |
| US-7.2.3.7  | engine developer (P-26) | F-7.2.3  | R-7.2.3      |
| US-7.2.3.8  | engine developer (P-26) | F-7.2.3  | R-7.2.3      |
| US-7.2.3.9  | engine developer (P-26) | F-7.2.3  | R-7.2.3      |
| US-7.2.3.10 | engine tester (P-27)    | F-7.2.3  | R-7.2.3      |
| US-7.2.3.11 | engine tester (P-27)    | F-7.2.3  | R-7.2.3      |
| US-7.2.3.12 | engine tester (P-27)    | F-7.2.3  | R-7.2.3      |

1. **US-7.2.3.1** — I want to assign steering behaviors (seek, flee, arrive, wander, pursuit, evade)
   to agent archetypes in the editor
   - **Acceptance:** each NPC type moves distinctively
2. **US-7.2.3.2** — I want to tune the arrive behavior's deceleration radius per agent
   - **Acceptance:** NPCs slow down gracefully when reaching their destination
3. **US-7.2.3.3** — I want to configure wander jitter, radius, and distance parameters
   - **Acceptance:** idle NPCs meander naturally without leaving their assigned area
4. **US-7.2.3.4** — I want town guards to patrol with natural wandering when idle
   - **Acceptance:** NPC movement looks organic rather than robotic
5. **US-7.2.3.5** — I want pursuing enemies to intercept my predicted position instead of chasing
   where I was
   - **Acceptance:** combat AI feels smart and challenging
6. **US-7.2.3.6** — I want fleeing NPCs to run away from threats using predictive evasion
   - **Acceptance:** their escape behavior looks intentional
7. **US-7.2.3.7** — I want to implement seek, flee, arrive, wander, pursuit, and evade as composable
   steering force functions
   - **Acceptance:** they can be combined flexibly
8. **US-7.2.3.8** — I want each steering behavior to return a force vector
   - **Acceptance:** the blending system can combine multiple active behaviors per agent
9. **US-7.2.3.9** — I want to use SIMD-optimized vector math for steering computations
   - **Acceptance:** thousands of agents can be processed per tick efficiently
10. **US-7.2.3.10** — I want to verify that the arrive behavior brings agents to a complete stop at
    the target position within a tolerance
    - **Acceptance:** arrival is precise
11. **US-7.2.3.11** — I want to verify that pursuit behavior intercepts a linearly moving target
    faster than naive seek
    - **Acceptance:** prediction works correctly
12. **US-7.2.3.12** — I want to verify that wander behavior keeps agents within a configurable area
    over extended simulation
    - **Acceptance:** wandering does not drift unbounded. ---

## F-7.2.4 — Steering Behavior Blending & Priority

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.2.4.1  | designer (P-5)          | F-7.2.4  | R-7.2.4      |
| US-7.2.4.2  | designer (P-5)          | F-7.2.4  | R-7.2.4      |
| US-7.2.4.3  | designer (P-5)          | F-7.2.4  | R-7.2.4      |
| US-7.2.4.4  | player (P-23)           | F-7.2.4  | R-7.2.4      |
| US-7.2.4.5  | player (P-23)           | F-7.2.4  | R-7.2.4      |
| US-7.2.4.6  | player (P-23)           | F-7.2.4  | R-7.2.4      |
| US-7.2.4.7  | engine developer (P-26) | F-7.2.4  | R-7.2.4      |
| US-7.2.4.8  | engine developer (P-26) | F-7.2.4  | R-7.2.4      |
| US-7.2.4.9  | engine developer (P-26) | F-7.2.4  | R-7.2.4      |
| US-7.2.4.10 | engine tester (P-27)    | F-7.2.4  | R-7.2.4      |
| US-7.2.4.11 | engine tester (P-27)    | F-7.2.4  | R-7.2.4      |
| US-7.2.4.12 | engine tester (P-27)    | F-7.2.4  | R-7.2.4      |

1. **US-7.2.4.1** — I want to set blending weights for each active steering behavior per agent
   archetype
   - **Acceptance:** avoidance can outweigh seek when needed
2. **US-7.2.4.2** — I want to choose between weighted blending and priority pipeline modes per agent
   - **Acceptance:** different NPCs use the strategy best suited to their role
3. **US-7.2.4.3** — I want to visualize each steering force component and the final blended vector
   as debug arrows
   - **Acceptance:** I can diagnose conflicting behaviors
4. **US-7.2.4.4** — I want pursuing enemies to avoid obstacles while chasing me instead of running
   into walls
   - **Acceptance:** pursuit feels intelligent
5. **US-7.2.4.5** — I want fleeing NPCs to avoid other NPCs while running away, not bulldoze through
   crowds
   - **Acceptance:** escape behavior looks realistic
6. **US-7.2.4.6** — I want NPC movement transitions (idle to pursuit to arrival) to be smooth
   without jerky direction changes
   - **Acceptance:** blending looks natural
7. **US-7.2.4.7** — I want to implement both weighted-sum and priority pipeline blending
   - **Acceptance:** designers can choose the best mode per scenario
8. **US-7.2.4.8** — I want priority blending to allocate remaining steering magnitude to
   lower-priority layers
   - **Acceptance:** high-priority forces are satisfied first
9. **US-7.2.4.9** — I want to limit active steering layers to 3 on mobile (vs. 5 on desktop)
   - **Acceptance:** per-agent blending cost is reduced
10. **US-7.2.4.10** — I want to verify that priority blending prevents conflicting forces from
    canceling out
    - **Acceptance:** agents always produce meaningful movement
11. **US-7.2.4.11** — I want to test weighted blending with known inputs and verify the output
    vector matches the expected weighted sum
    - **Acceptance:** blending math is correct
12. **US-7.2.4.12** — I want to benchmark blending cost with the maximum number of active layers per
    agent
    - **Acceptance:** worst-case performance is within budget. ---

## F-7.2.5 — Formation Movement

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.2.5.1  | designer (P-5)          | F-7.2.5  | R-7.2.5      |
| US-7.2.5.2  | designer (P-5)          | F-7.2.5  | R-7.2.5      |
| US-7.2.5.3  | designer (P-5)          | F-7.2.5  | R-7.2.5      |
| US-7.2.5.4  | player (P-23)           | F-7.2.5  | R-7.2.5      |
| US-7.2.5.5  | player (P-23)           | F-7.2.5  | R-7.2.5      |
| US-7.2.5.6  | player (P-23)           | F-7.2.5  | R-7.2.5      |
| US-7.2.5.7  | engine developer (P-26) | F-7.2.5  | R-7.2.5      |
| US-7.2.5.8  | engine developer (P-26) | F-7.2.5  | R-7.2.5      |
| US-7.2.5.9  | engine developer (P-26) | F-7.2.5  | R-7.2.5      |
| US-7.2.5.10 | engine tester (P-27)    | F-7.2.5  | R-7.2.5      |
| US-7.2.5.11 | engine tester (P-27)    | F-7.2.5  | R-7.2.5      |
| US-7.2.5.12 | engine tester (P-27)    | F-7.2.5  | R-7.2.5      |

1. **US-7.2.5.1** — I want to assign formation shapes (line, wedge, column, circle) to agent groups
   in the editor
   - **Acceptance:** squads move in organized patterns
2. **US-7.2.5.2** — I want to configure spacing between slots and the number of slots per formation
   - **Acceptance:** formations fit different group sizes and terrain widths
3. **US-7.2.5.3** — I want to observe formations automatically adjust spacing when terrain narrows
   - **Acceptance:** agents do not clip through walls to maintain position
4. **US-7.2.5.4** — I want guard squads to march in wedge or column formation through the city
   - **Acceptance:** military NPCs look organized and disciplined
5. **US-7.2.5.5** — I want a formation to reform after being disrupted by combat or obstacles
   - **Acceptance:** group movement recovers gracefully
6. **US-7.2.5.6** — I want formation members to maintain their offset positions relative to the
   leader as the group turns
   - **Acceptance:** formation integrity is visually convincing
7. **US-7.2.5.7** — I want to implement formation shapes as parameterized slot layouts with a leader
   and follower offsets
   - **Acceptance:** shapes are data-driven and extensible
8. **US-7.2.5.8** — I want followers to use arrival steering to maintain their formation offset
   - **Acceptance:** slot tracking is smooth and responsive
9. **US-7.2.5.9** — I want formations to automatically reassign slots when members are lost
   - **Acceptance:** the remaining members maintain a coherent reduced formation
10. **US-7.2.5.10** — I want to verify that formation member spacing matches the configured values
    within tolerance
    - **Acceptance:** formations look as designed
11. **US-7.2.5.11** — I want to test a formation passing through a narrow passage and verify that
    spacing adjusts without clipping
    - **Acceptance:** adaptation works
12. **US-7.2.5.12** — I want to remove formation members and verify remaining members reassign to
    valid slots
    - **Acceptance:** partial formations remain coherent. ---

## F-7.2.6 — Group Steering & Cohesion

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.2.6.1  | designer (P-5)          | F-7.2.6  | R-7.2.6      |
| US-7.2.6.2  | designer (P-5)          | F-7.2.6  | R-7.2.6      |
| US-7.2.6.3  | designer (P-5)          | F-7.2.6  | R-7.2.6      |
| US-7.2.6.4  | player (P-23)           | F-7.2.6  | R-7.2.6      |
| US-7.2.6.5  | player (P-23)           | F-7.2.6  | R-7.2.6      |
| US-7.2.6.6  | player (P-23)           | F-7.2.6  | R-7.2.6      |
| US-7.2.6.7  | engine developer (P-26) | F-7.2.6  | R-7.2.6      |
| US-7.2.6.8  | engine developer (P-26) | F-7.2.6  | R-7.2.6      |
| US-7.2.6.9  | engine developer (P-26) | F-7.2.6  | R-7.2.6      |
| US-7.2.6.10 | engine tester (P-27)    | F-7.2.6  | R-7.2.6      |
| US-7.2.6.11 | engine tester (P-27)    | F-7.2.6  | R-7.2.6      |
| US-7.2.6.12 | engine tester (P-27)    | F-7.2.6  | R-7.2.6      |

1. **US-7.2.6.1** — I want to create named agent groups in the editor and assign members
   - **Acceptance:** group steering applies to designated NPC teams
2. **US-7.2.6.2** — I want to tune cohesion and alignment correction strength per group
   - **Acceptance:** tight squads stay close while loose patrols allow more spread
3. **US-7.2.6.3** — I want to see the group centroid and average heading as debug gizmos
   - **Acceptance:** I can verify group steering is working correctly
4. **US-7.2.6.4** — I want patrol groups to stay together when rounding corners instead of
   fragmenting
   - **Acceptance:** group movement looks cohesive
5. **US-7.2.6.5** — I want escort NPC groups to maintain cohesion while matching my movement speed
   - **Acceptance:** the escort feels responsive and unified
6. **US-7.2.6.6** — I want scattered group members to gradually regroup after being separated by
   obstacles
   - **Acceptance:** groups reform naturally
7. **US-7.2.6.7** — I want to maintain a lightweight group centroid and average heading tracker
   - **Acceptance:** cohesion and alignment corrections are computed efficiently
8. **US-7.2.6.8** — I want to apply cohesion and alignment corrections using the simplified group
   tracker rather than full flocking
   - **Acceptance:** group steering is cheaper than crowd simulation
9. **US-7.2.6.9** — I want group size to be limited by the platform crowd budget
   - **Acceptance:** group steering cost scales appropriately per platform
10. **US-7.2.6.10** — I want to verify that a group navigating a series of turns maintains cohesion
    within a configurable radius
    - **Acceptance:** fragmentation does not occur
11. **US-7.2.6.11** — I want to verify that all group members converge to a shared velocity goal
    within a bounded time
    - **Acceptance:** group cohesion stabilizes
12. **US-7.2.6.12** — I want to benchmark the CPU cost of group steering per member
    - **Acceptance:** I can verify it is cheaper than full flocking on equivalent group sizes
