# User Stories — 7.2 Steering & Avoidance

## F-7.2.1 — RVO/ORCA Local Avoidance

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.2.1.1 | designer (P-5) | I want to configure the ORCA neighbor count and query radius per agent archetype | avoidance quality balances with performance for each NPC type | F-7.2.1 | R-7.2.1 |
| US-7.2.1.2 | designer (P-5) | I want to spawn many agents at a narrow chokepoint and observe smooth flow without pileups | I can tune ORCA parameters for dense areas | F-7.2.1 | R-7.2.1 |
| US-7.2.1.3 | designer (P-5) | I want to adjust how aggressively agents avoid each other (tight for guards, loose for civilians) | different NPCs have distinct movement feel | F-7.2.1 | R-7.2.1 |
| US-7.2.1.4 | player (P-23) | I want NPCs in crowded city streets to navigate smoothly without walking through each other | crowds feel realistic | F-7.2.1 | R-7.2.1 |
| US-7.2.1.5 | player (P-23) | I want groups of guards to file through doorways one at a time instead of clumping | tight spaces feel physically plausible | F-7.2.1 | R-7.2.1 |
| US-7.2.1.6 | player (P-23) | I want approaching enemies to spread out instead of stacking on top of each other | combat encounters look tactically distributed | F-7.2.1 | R-7.2.1 |
| US-7.2.1.7 | engine developer (P-26) | I want to implement Optimal Reciprocal Collision Avoidance with velocity obstacle computation | agents produce deadlock-free movement | F-7.2.1 | R-7.2.1 |
| US-7.2.1.8 | engine developer (P-26) | I want to batch neighbor queries using the shared spatial index | ORCA scales efficiently with agent count | F-7.2.1 | R-7.2.1 |
| US-7.2.1.9 | engine developer (P-26) | I want to limit ORCA neighbor count to 4 on mobile (vs. 12 on desktop) | per-agent avoidance cost fits within the mobile CPU budget | F-7.2.1 | R-7.2.1 |
| US-7.2.1.10 | engine tester (P-27) | I want to verify that no two agents overlap positions in a stress test with 200+ agents in a confined area | ORCA prevents interpenetration | F-7.2.1 | R-7.2.1 |
| US-7.2.1.11 | engine tester (P-27) | I want to verify that two groups approaching from opposite directions through a narrow passage resolve without deadlock | ORCA is robust | F-7.2.1 | R-7.2.1 |
| US-7.2.1.12 | engine tester (P-27) | I want to benchmark per-agent ORCA computation time across platform tiers | avoidance cost stays within the per-agent CPU budget. --- | F-7.2.1 | R-7.2.1 |

## F-7.2.2 — Obstacle Avoidance (Static Geometry)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.2.2.1 | designer (P-5) | I want to configure the number of feeler rays per agent archetype | avoidance precision matches the agent's movement requirements | F-7.2.2 | R-7.2.2 |
| US-7.2.2.2 | designer (P-5) | I want to test agents walking through corridors and verify they do not clip through walls | geometry avoidance works for indoor environments | F-7.2.2 | R-7.2.2 |
| US-7.2.2.3 | designer (P-5) | I want to tune feeler ray length per agent size class | large agents detect walls earlier and steer with wider clearance | F-7.2.2 | R-7.2.2 |
| US-7.2.2.4 | player (P-23) | I want NPCs to smoothly steer around walls and pillars without bumping into them | indoor movement feels natural | F-7.2.2 | R-7.2.2 |
| US-7.2.2.5 | player (P-23) | I want AI agents to steer away from cliff edges and terrain drops | they do not walk off ledges unnaturally | F-7.2.2 | R-7.2.2 |
| US-7.2.2.6 | player (P-23) | I want NPCs to slide around sharp geometry corners without getting stuck | movement remains fluid in complex interiors | F-7.2.2 | R-7.2.2 |
| US-7.2.2.7 | engine developer (P-26) | I want to cast short-range feeler rays against nearby static geometry | agents receive correction forces before contacting walls | F-7.2.2 | R-7.2.2 |
| US-7.2.2.8 | engine developer (P-26) | I want to apply static obstacle avoidance as a final correction layer after ORCA | both agent-agent and agent-geometry avoidance work | F-7.2.2 | R-7.2.2 |
| US-7.2.2.9 | engine developer (P-26) | I want to reduce feeler rays to 2 on mobile (vs. 5 on desktop) | raycast cost fits within the mobile per-agent budget | F-7.2.2 | R-7.2.2 |
| US-7.2.2.10 | engine tester (P-27) | I want to verify that agents never clip through static geometry on a reference level with tight corridors | obstacle avoidance is airtight | F-7.2.2 | R-7.2.2 |
| US-7.2.2.11 | engine tester (P-27) | I want to confirm that low-LOD agents skip obstacle avoidance entirely as designed | the LOD system correctly gates expensive behaviors | F-7.2.2 | R-7.2.2 |
| US-7.2.2.12 | engine tester (P-27) | I want to benchmark feeler ray computation cost per agent across platforms | the raycast budget scales appropriately. --- | F-7.2.2 | R-7.2.2 |

## F-7.2.3 — Core Steering Behaviors

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.2.3.1 | designer (P-5) | I want to assign steering behaviors (seek, flee, arrive, wander, pursuit, evade) to agent archetypes in the editor | each NPC type moves distinctively | F-7.2.3 | R-7.2.3 |
| US-7.2.3.2 | designer (P-5) | I want to tune the arrive behavior's deceleration radius per agent | NPCs slow down gracefully when reaching their destination | F-7.2.3 | R-7.2.3 |
| US-7.2.3.3 | designer (P-5) | I want to configure wander jitter, radius, and distance parameters | idle NPCs meander naturally without leaving their assigned area | F-7.2.3 | R-7.2.3 |
| US-7.2.3.4 | player (P-23) | I want town guards to patrol with natural wandering when idle | NPC movement looks organic rather than robotic | F-7.2.3 | R-7.2.3 |
| US-7.2.3.5 | player (P-23) | I want pursuing enemies to intercept my predicted position instead of chasing where I was | combat AI feels smart and challenging | F-7.2.3 | R-7.2.3 |
| US-7.2.3.6 | player (P-23) | I want fleeing NPCs to run away from threats using predictive evasion | their escape behavior looks intentional | F-7.2.3 | R-7.2.3 |
| US-7.2.3.7 | engine developer (P-26) | I want to implement seek, flee, arrive, wander, pursuit, and evade as composable steering force functions | they can be combined flexibly | F-7.2.3 | R-7.2.3 |
| US-7.2.3.8 | engine developer (P-26) | I want each steering behavior to return a force vector | the blending system can combine multiple active behaviors per agent | F-7.2.3 | R-7.2.3 |
| US-7.2.3.9 | engine developer (P-26) | I want to use SIMD-optimized vector math for steering computations | thousands of agents can be processed per tick efficiently | F-7.2.3 | R-7.2.3 |
| US-7.2.3.10 | engine tester (P-27) | I want to verify that the arrive behavior brings agents to a complete stop at the target position within a tolerance | arrival is precise | F-7.2.3 | R-7.2.3 |
| US-7.2.3.11 | engine tester (P-27) | I want to verify that pursuit behavior intercepts a linearly moving target faster than naive seek | prediction works correctly | F-7.2.3 | R-7.2.3 |
| US-7.2.3.12 | engine tester (P-27) | I want to verify that wander behavior keeps agents within a configurable area over extended simulation | wandering does not drift unbounded. --- | F-7.2.3 | R-7.2.3 |

## F-7.2.4 — Steering Behavior Blending & Priority

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.2.4.1 | designer (P-5) | I want to set blending weights for each active steering behavior per agent archetype | avoidance can outweigh seek when needed | F-7.2.4 | R-7.2.4 |
| US-7.2.4.2 | designer (P-5) | I want to choose between weighted blending and priority pipeline modes per agent | different NPCs use the strategy best suited to their role | F-7.2.4 | R-7.2.4 |
| US-7.2.4.3 | designer (P-5) | I want to visualize each steering force component and the final blended vector as debug arrows | I can diagnose conflicting behaviors | F-7.2.4 | R-7.2.4 |
| US-7.2.4.4 | player (P-23) | I want pursuing enemies to avoid obstacles while chasing me instead of running into walls | pursuit feels intelligent | F-7.2.4 | R-7.2.4 |
| US-7.2.4.5 | player (P-23) | I want fleeing NPCs to avoid other NPCs while running away, not bulldoze through crowds | escape behavior looks realistic | F-7.2.4 | R-7.2.4 |
| US-7.2.4.6 | player (P-23) | I want NPC movement transitions (idle to pursuit to arrival) to be smooth without jerky direction changes | blending looks natural | F-7.2.4 | R-7.2.4 |
| US-7.2.4.7 | engine developer (P-26) | I want to implement both weighted-sum and priority pipeline blending | designers can choose the best mode per scenario | F-7.2.4 | R-7.2.4 |
| US-7.2.4.8 | engine developer (P-26) | I want priority blending to allocate remaining steering magnitude to lower-priority layers | high-priority forces are satisfied first | F-7.2.4 | R-7.2.4 |
| US-7.2.4.9 | engine developer (P-26) | I want to limit active steering layers to 3 on mobile (vs. 5 on desktop) | per-agent blending cost is reduced | F-7.2.4 | R-7.2.4 |
| US-7.2.4.10 | engine tester (P-27) | I want to verify that priority blending prevents conflicting forces from canceling out | agents always produce meaningful movement | F-7.2.4 | R-7.2.4 |
| US-7.2.4.11 | engine tester (P-27) | I want to test weighted blending with known inputs and verify the output vector matches the expected weighted sum | blending math is correct | F-7.2.4 | R-7.2.4 |
| US-7.2.4.12 | engine tester (P-27) | I want to benchmark blending cost with the maximum number of active layers per agent | worst-case performance is within budget. --- | F-7.2.4 | R-7.2.4 |

## F-7.2.5 — Formation Movement

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.2.5.1 | designer (P-5) | I want to assign formation shapes (line, wedge, column, circle) to agent groups in the editor | squads move in organized patterns | F-7.2.5 | R-7.2.5 |
| US-7.2.5.2 | designer (P-5) | I want to configure spacing between slots and the number of slots per formation | formations fit different group sizes and terrain widths | F-7.2.5 | R-7.2.5 |
| US-7.2.5.3 | designer (P-5) | I want to observe formations automatically adjust spacing when terrain narrows | agents do not clip through walls to maintain position | F-7.2.5 | R-7.2.5 |
| US-7.2.5.4 | player (P-23) | I want guard squads to march in wedge or column formation through the city | military NPCs look organized and disciplined | F-7.2.5 | R-7.2.5 |
| US-7.2.5.5 | player (P-23) | I want a formation to reform after being disrupted by combat or obstacles | group movement recovers gracefully | F-7.2.5 | R-7.2.5 |
| US-7.2.5.6 | player (P-23) | I want formation members to maintain their offset positions relative to the leader as the group turns | formation integrity is visually convincing | F-7.2.5 | R-7.2.5 |
| US-7.2.5.7 | engine developer (P-26) | I want to implement formation shapes as parameterized slot layouts with a leader and follower offsets | shapes are data-driven and extensible | F-7.2.5 | R-7.2.5 |
| US-7.2.5.8 | engine developer (P-26) | I want followers to use arrival steering to maintain their formation offset | slot tracking is smooth and responsive | F-7.2.5 | R-7.2.5 |
| US-7.2.5.9 | engine developer (P-26) | I want formations to automatically reassign slots when members are lost | the remaining members maintain a coherent reduced formation | F-7.2.5 | R-7.2.5 |
| US-7.2.5.10 | engine tester (P-27) | I want to verify that formation member spacing matches the configured values within tolerance | formations look as designed | F-7.2.5 | R-7.2.5 |
| US-7.2.5.11 | engine tester (P-27) | I want to test a formation passing through a narrow passage and verify that spacing adjusts without clipping | adaptation works | F-7.2.5 | R-7.2.5 |
| US-7.2.5.12 | engine tester (P-27) | I want to remove formation members and verify remaining members reassign to valid slots | partial formations remain coherent. --- | F-7.2.5 | R-7.2.5 |

## F-7.2.6 — Group Steering & Cohesion

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.2.6.1 | designer (P-5) | I want to create named agent groups in the editor and assign members | group steering applies to designated NPC teams | F-7.2.6 | R-7.2.6 |
| US-7.2.6.2 | designer (P-5) | I want to tune cohesion and alignment correction strength per group | tight squads stay close while loose patrols allow more spread | F-7.2.6 | R-7.2.6 |
| US-7.2.6.3 | designer (P-5) | I want to see the group centroid and average heading as debug gizmos | I can verify group steering is working correctly | F-7.2.6 | R-7.2.6 |
| US-7.2.6.4 | player (P-23) | I want patrol groups to stay together when rounding corners instead of fragmenting | group movement looks cohesive | F-7.2.6 | R-7.2.6 |
| US-7.2.6.5 | player (P-23) | I want escort NPC groups to maintain cohesion while matching my movement speed | the escort feels responsive and unified | F-7.2.6 | R-7.2.6 |
| US-7.2.6.6 | player (P-23) | I want scattered group members to gradually regroup after being separated by obstacles | groups reform naturally | F-7.2.6 | R-7.2.6 |
| US-7.2.6.7 | engine developer (P-26) | I want to maintain a lightweight group centroid and average heading tracker | cohesion and alignment corrections are computed efficiently | F-7.2.6 | R-7.2.6 |
| US-7.2.6.8 | engine developer (P-26) | I want to apply cohesion and alignment corrections using the simplified group tracker rather than full flocking | group steering is cheaper than crowd simulation | F-7.2.6 | R-7.2.6 |
| US-7.2.6.9 | engine developer (P-26) | I want group size to be limited by the platform crowd budget | group steering cost scales appropriately per platform | F-7.2.6 | R-7.2.6 |
| US-7.2.6.10 | engine tester (P-27) | I want to verify that a group navigating a series of turns maintains cohesion within a configurable radius | fragmentation does not occur | F-7.2.6 | R-7.2.6 |
| US-7.2.6.11 | engine tester (P-27) | I want to verify that all group members converge to a shared velocity goal within a bounded time | group cohesion stabilizes | F-7.2.6 | R-7.2.6 |
| US-7.2.6.12 | engine tester (P-27) | I want to benchmark the CPU cost of group steering per member | I can verify it is cheaper than full flocking on equivalent group sizes | F-7.2.6 | R-7.2.6 |
