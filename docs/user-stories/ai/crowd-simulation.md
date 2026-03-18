# User Stories — 7.7 Crowd Simulation

## F-7.7.1 — Flocking Behaviors (Separation, Alignment, Cohesion)

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.7.1.1  | designer (P-5)          | F-7.7.1  | R-7.7.1      |
| US-7.7.1.2  | designer (P-5)          | F-7.7.1  | R-7.7.1      |
| US-7.7.1.3  | designer (P-5)          | F-7.7.1  | R-7.7.1      |
| US-7.7.1.4  | player (P-23)           | F-7.7.1  | R-7.7.1      |
| US-7.7.1.5  | player (P-23)           | F-7.7.1  | R-7.7.1      |
| US-7.7.1.6  | player (P-23)           | F-7.7.1  | R-7.7.1      |
| US-7.7.1.7  | engine developer (P-26) | F-7.7.1  | R-7.7.1      |
| US-7.7.1.8  | engine developer (P-26) | F-7.7.1  | R-7.7.1      |
| US-7.7.1.9  | engine developer (P-26) | F-7.7.1  | R-7.7.1      |
| US-7.7.1.10 | engine tester (P-27)    | F-7.7.1  | R-7.7.1      |
| US-7.7.1.11 | engine tester (P-27)    | F-7.7.1  | R-7.7.1      |
| US-7.7.1.12 | engine tester (P-27)    | F-7.7.1  | R-7.7.1      |

1. **US-7.7.1.1** — I want to configure separation, alignment, and cohesion weights per crowd
   archetype
   - **Acceptance:** tight military columns, loose herds, and scattered civilians each have distinct
     group movement
2. **US-7.7.1.2** — I want to set the neighbor query radius per archetype
   - **Acceptance:** flocking range matches the intended group tightness
3. **US-7.7.1.3** — I want to see separation, alignment, and cohesion force vectors as debug arrows
   on each agent
   - **Acceptance:** I can tune weights visually
4. **US-7.7.1.4** — I want animal herds to move as a cohesive group rather than as disconnected
   individuals
   - **Acceptance:** wildlife feels alive and natural
5. **US-7.7.1.5** — I want flocks of birds to move in recognizable patterns with alignment and
   cohesion
   - **Acceptance:** aerial wildlife looks realistic
6. **US-7.7.1.6** — I want crowd NPCs to maintain personal space via separation forces
   - **Acceptance:** dense crowds do not clip through each other
7. **US-7.7.1.7** — I want to implement Reynolds flocking with weighted separation, alignment, and
   cohesion forces
   - **Acceptance:** group movement emerges from simple rules
8. **US-7.7.1.8** — I want to query neighbors via the shared spatial index for flocking
   - **Acceptance:** neighbor lookups are efficient
9. **US-7.7.1.9** — I want to reduce the neighbor query radius on mobile (8 m vs. 16 m desktop)
   - **Acceptance:** per-agent flocking cost fits the mobile budget
10. **US-7.7.1.10** — I want to verify that separation forces prevent agents from overlapping within
    a flock
    - **Acceptance:** spatial distribution is correct
11. **US-7.7.1.11** — I want to verify that cohesion forces keep flock members within a configurable
    radius of the group centroid
    - **Acceptance:** groups do not fragment
12. **US-7.7.1.12** — I want to benchmark flocking computation for the maximum flock size per
    platform
    - **Acceptance:** flocking cost is within budget. ---

## F-7.7.2 — Flow Field Navigation

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.7.2.1  | designer (P-5)          | F-7.7.2  | R-7.7.2      |
| US-7.7.2.2  | designer (P-5)          | F-7.7.2  | R-7.7.2      |
| US-7.7.2.3  | designer (P-5)          | F-7.7.2  | R-7.7.2      |
| US-7.7.2.4  | player (P-23)           | F-7.7.2  | R-7.7.2      |
| US-7.7.2.5  | player (P-23)           | F-7.7.2  | R-7.7.2      |
| US-7.7.2.6  | player (P-23)           | F-7.7.2  | R-7.7.2      |
| US-7.7.2.7  | engine developer (P-26) | F-7.7.2  | R-7.7.2      |
| US-7.7.2.8  | engine developer (P-26) | F-7.7.2  | R-7.7.2      |
| US-7.7.2.9  | engine developer (P-26) | F-7.7.2  | R-7.7.2      |
| US-7.7.2.10 | engine tester (P-27)    | F-7.7.2  | R-7.7.2      |
| US-7.7.2.11 | engine tester (P-27)    | F-7.7.2  | R-7.7.2      |
| US-7.7.2.12 | engine tester (P-27)    | F-7.7.2  | R-7.7.2      |

1. **US-7.7.2.1** — I want to generate flow fields from goal positions in the editor
   - **Acceptance:** I can preview mass crowd movement toward points of interest
2. **US-7.7.2.2** — I want to set flow field cell resolution (1 m desktop, 2 m mobile)
   - **Acceptance:** crowd navigation quality scales with platform capability
3. **US-7.7.2.3** — I want to assign goal positions to crowd groups and see them flow toward the
   goal
   - **Acceptance:** I can orchestrate mass crowd events
4. **US-7.7.2.4** — I want thousands of crowd NPCs to move smoothly toward a gate, market, or event
   - **Acceptance:** mass movement looks realistic and purposeful
5. **US-7.7.2.5** — I want crowd NPCs to flow around buildings and obstacles along the cheapest path
   - **Acceptance:** mass navigation looks natural
6. **US-7.7.2.6** — I want crowd flow direction to change smoothly when goals update
   - **Acceptance:** redirected crowds do not snap or teleport
7. **US-7.7.2.7** — I want to implement 2D grid-based flow fields using Dijkstra propagation from
   goal positions
   - **Acceptance:** each cell stores the cheapest path direction
8. **US-7.7.2.8** — I want crowd agents to sample the flow field directly instead of running
   individual A* queries
   - **Acceptance:** per-agent navigation cost is constant
9. **US-7.7.2.9** — I want to limit concurrent active flow fields to 4 on mobile
   - **Acceptance:** flow field memory and computation fit within mobile constraints
10. **US-7.7.2.10** — I want to verify that following flow field directions from any cell reaches
    the goal position
    - **Acceptance:** path correctness is confirmed
11. **US-7.7.2.11** — I want to verify that flow fields recompute correctly when dynamic obstacles
    modify the cost grid
    - **Acceptance:** redirected flows are accurate
12. **US-7.7.2.12** — I want to benchmark flow field generation time for the maximum tile size
    - **Acceptance:** recomputation fits within the per-tick budget. ---

## F-7.7.3 — Flow Field Streaming & Caching

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.7.3.1  | designer (P-5)          | F-7.7.3  | R-7.7.3      |
| US-7.7.3.2  | designer (P-5)          | F-7.7.3  | R-7.7.3      |
| US-7.7.3.3  | designer (P-5)          | F-7.7.3  | R-7.7.3      |
| US-7.7.3.4  | player (P-23)           | F-7.7.3  | R-7.7.3      |
| US-7.7.3.5  | player (P-23)           | F-7.7.3  | R-7.7.3      |
| US-7.7.3.6  | player (P-23)           | F-7.7.3  | R-7.7.3      |
| US-7.7.3.7  | engine developer (P-26) | F-7.7.3  | R-7.7.3      |
| US-7.7.3.8  | engine developer (P-26) | F-7.7.3  | R-7.7.3      |
| US-7.7.3.9  | engine developer (P-26) | F-7.7.3  | R-7.7.3      |
| US-7.7.3.10 | engine tester (P-27)    | F-7.7.3  | R-7.7.3      |
| US-7.7.3.11 | engine tester (P-27)    | F-7.7.3  | R-7.7.3      |
| US-7.7.3.12 | engine tester (P-27)    | F-7.7.3  | R-7.7.3      |

1. **US-7.7.3.1** — I want to configure the flow field cache size per platform
   - **Acceptance:** frequently used fields are retained without exceeding memory
2. **US-7.7.3.2** — I want to see which flow fields are currently cached in a debug panel
   - **Acceptance:** I can verify cache utilization
3. **US-7.7.3.3** — I want cached flow fields to be invalidated when dynamic obstacles modify the
   cost grid
   - **Acceptance:** stale directions are never used
4. **US-7.7.3.4** — I want crowd NPCs to navigate seamlessly across world streaming boundaries
   - **Acceptance:** crowd flow is continuous
5. **US-7.7.3.5** — I want crowds to redirect quickly when a new obstacle appears (gate closes,
   barricade placed)
   - **Acceptance:** flow field invalidation is responsive
6. **US-7.7.3.6** — I want crowds to keep moving during flow field recomputation
   - **Acceptance:** cache misses do not cause visible freezes
7. **US-7.7.3.7** — I want to tile flow fields to match the world streaming grid and load/unload
   them with world chunks
   - **Acceptance:** flow field data streams with the world
8. **US-7.7.3.8** — I want to cache flow fields keyed by (goal cell, cost layer version)
   - **Acceptance:** identical goals reuse cached fields
9. **US-7.7.3.9** — I want to limit the mobile cache to 8 entries (vs. 32 on desktop)
   - **Acceptance:** memory usage is bounded with more aggressive eviction
10. **US-7.7.3.10** — I want to verify that a cache hit returns a field identical to a freshly
    computed one
    - **Acceptance:** caching does not alter navigation
11. **US-7.7.3.11** — I want to verify that cached fields are invalidated when the cost grid changes
    - **Acceptance:** stale fields are never used
12. **US-7.7.3.12** — I want to benchmark the performance difference between cache hits and misses
    - **Acceptance:** the caching benefit is quantified. ---

## F-7.7.4 — Mass Entity Simulation

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.7.4.1  | designer (P-5)          | F-7.7.4  | R-7.7.4      |
| US-7.7.4.2  | designer (P-5)          | F-7.7.4  | R-7.7.4      |
| US-7.7.4.3  | designer (P-5)          | F-7.7.4  | R-7.7.4      |
| US-7.7.4.4  | player (P-23)           | F-7.7.4  | R-7.7.4      |
| US-7.7.4.5  | player (P-23)           | F-7.7.4  | R-7.7.4      |
| US-7.7.4.6  | player (P-23)           | F-7.7.4  | R-7.7.4      |
| US-7.7.4.7  | engine developer (P-26) | F-7.7.4  | R-7.7.4      |
| US-7.7.4.8  | engine developer (P-26) | F-7.7.4  | R-7.7.4      |
| US-7.7.4.9  | engine developer (P-26) | F-7.7.4  | R-7.7.4      |
| US-7.7.4.10 | engine tester (P-27)    | F-7.7.4  | R-7.7.4      |
| US-7.7.4.11 | engine tester (P-27)    | F-7.7.4  | R-7.7.4      |
| US-7.7.4.12 | engine tester (P-27)    | F-7.7.4  | R-7.7.4      |

1. **US-7.7.4.1** — I want to spawn thousands of lightweight crowd NPCs with minimal per-agent state
   (position, velocity, archetype)
   - **Acceptance:** cities feel populated
2. **US-7.7.4.2** — I want to configure the crowd count budget per platform (mobile 100, Switch 500,
   console 2000, desktop 10000+)
   - **Acceptance:** each platform runs at target frame rate
3. **US-7.7.4.3** — I want to set a despawn radius so crowd NPCs far from players are removed
   - **Acceptance:** active agent count stays within budget
4. **US-7.7.4.4** — I want walk through a city populated with thousands of ambient inhabitants going
   about their daily lives
   - **Acceptance:** the world feels alive
5. **US-7.7.4.5** — I want distant crowd NPCs to fade out naturally rather than popping in and out
   of existence
   - **Acceptance:** despawn is not jarring
6. **US-7.7.4.6** — I want the frame rate to remain stable even in the most crowded city areas
   - **Acceptance:** crowd count does not degrade performance
7. **US-7.7.4.7** — I want to process crowd NPCs as lightweight entities driven by flow fields and
   flocking rather than full behavior trees
   - **Acceptance:** per-agent cost is bounded
8. **US-7.7.4.8** — I want each crowd entity to store only position, velocity, archetype ID, and
   flow field sample
   - **Acceptance:** memory footprint is minimized
9. **US-7.7.4.9** — I want the crowd budget to scale by platform tier with configurable caps
   - **Acceptance:** each platform runs within its CPU and memory budget
10. **US-7.7.4.10** — I want to verify that the active crowd entity count never exceeds the
    configured platform budget
    - **Acceptance:** the cap is enforced
11. **US-7.7.4.11** — I want to test that frame time remains stable at the maximum crowd count for
    each platform tier
    - **Acceptance:** performance is guaranteed
12. **US-7.7.4.12** — I want to benchmark the per-agent CPU cost for mass entity processing
    - **Acceptance:** the lightweight pipeline is validated. ---

## F-7.7.5 — AI Level of Detail (Processing Budget)

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.7.5.1  | designer (P-5)          | F-7.7.5  | R-7.7.5      |
| US-7.7.5.2  | designer (P-5)          | F-7.7.5  | R-7.7.5      |
| US-7.7.5.3  | designer (P-5)          | F-7.7.5  | R-7.7.5      |
| US-7.7.5.4  | player (P-23)           | F-7.7.5  | R-7.7.5      |
| US-7.7.5.5  | player (P-23)           | F-7.7.5  | R-7.7.5      |
| US-7.7.5.6  | player (P-23)           | F-7.7.5  | R-7.7.5      |
| US-7.7.5.7  | engine developer (P-26) | F-7.7.5  | R-7.7.5      |
| US-7.7.5.8  | engine developer (P-26) | F-7.7.5  | R-7.7.5      |
| US-7.7.5.9  | engine developer (P-26) | F-7.7.5  | R-7.7.5      |
| US-7.7.5.10 | engine tester (P-27)    | F-7.7.5  | R-7.7.5      |
| US-7.7.5.11 | engine tester (P-27)    | F-7.7.5  | R-7.7.5      |
| US-7.7.5.12 | engine tester (P-27)    | F-7.7.5  | R-7.7.5      |

1. **US-7.7.5.1** — I want to configure LOD tier distances per platform (high-LOD 20 m mobile, 60 m
   desktop)
   - **Acceptance:** AI processing scales with player proximity
2. **US-7.7.5.2** — I want to set the maximum number of agents per LOD tier (8 high-LOD mobile, 64
   desktop)
   - **Acceptance:** the most expensive agents are capped
3. **US-7.7.5.3** — I want quest-critical NPCs to remain at high LOD regardless of distance
   - **Acceptance:** important AI never degrades in quality
4. **US-7.7.5.4** — I want NPCs near me to exhibit full intelligent behavior (complex decisions,
   reactive perception)
   - **Acceptance:** AI I interact with feels smart
5. **US-7.7.5.5** — I want distant NPCs to use simplified movement without it being noticeable
   - **Acceptance:** LOD transitions are invisible
6. **US-7.7.5.6** — I want NPCs to exhibit increasingly complex behavior as I approach
   - **Acceptance:** AI quality feels consistent with my observation distance
7. **US-7.7.5.7** — I want to assign each AI agent a LOD tier based on distance to the nearest
   player and gameplay relevance
   - **Acceptance:** processing scales
8. **US-7.7.5.8** — I want to implement a global budget scheduler that distributes CPU time across
   LOD tiers
   - **Acceptance:** frame rate is stable
9. **US-7.7.5.9** — I want low-LOD agents to use flow-field-only movement, skipping BT and
   perception
   - **Acceptance:** distant agents cost minimal CPU
10. **US-7.7.5.10** — I want to verify that agents transition between LOD tiers at the correct
    distances
    - **Acceptance:** tier assignment is accurate
11. **US-7.7.5.11** — I want to verify that the budget scheduler keeps AI processing within its
    allocated time slice even at maximum agent count
    - **Acceptance:** frame rate is stable
12. **US-7.7.5.12** — I want to verify that LOD tier transitions do not produce visible behavior
    discontinuities (path snapping, instant state changes)
    - **Acceptance:** transitions are seamless. ---

## F-7.7.6 — Density Management

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.7.6.1  | designer (P-5)          | F-7.7.6  | R-7.7.6      |
| US-7.7.6.2  | designer (P-5)          | F-7.7.6  | R-7.7.6      |
| US-7.7.6.3  | designer (P-5)          | F-7.7.6  | R-7.7.6      |
| US-7.7.6.4  | player (P-23)           | F-7.7.6  | R-7.7.6      |
| US-7.7.6.5  | player (P-23)           | F-7.7.6  | R-7.7.6      |
| US-7.7.6.6  | player (P-23)           | F-7.7.6  | R-7.7.6      |
| US-7.7.6.7  | engine developer (P-26) | F-7.7.6  | R-7.7.6      |
| US-7.7.6.8  | engine developer (P-26) | F-7.7.6  | R-7.7.6      |
| US-7.7.6.9  | engine developer (P-26) | F-7.7.6  | R-7.7.6      |
| US-7.7.6.10 | engine tester (P-27)    | F-7.7.6  | R-7.7.6      |
| US-7.7.6.11 | engine tester (P-27)    | F-7.7.6  | R-7.7.6      |
| US-7.7.6.12 | engine tester (P-27)    | F-7.7.6  | R-7.7.6      |

1. **US-7.7.6.1** — I want to configure per-cell crowd density caps
   - **Acceptance:** unrealistic pileups at chokepoints and events are prevented
2. **US-7.7.6.2** — I want to configure whether overflow agents are redirected to alternate routes
   or despawned
   - **Acceptance:** density management matches the scene's needs
3. **US-7.7.6.3** — I want to test density management at a narrow gate with hundreds of crowd NPCs
   and verify no pileups
   - **Acceptance:** caps work in real scenarios
4. **US-7.7.6.4** — I want crowds at chokepoints to spread into orderly flow rather than piling up
   - **Acceptance:** movement looks physically plausible
5. **US-7.7.6.5** — I want crowd NPCs to take alternate routes when the primary path is full
   - **Acceptance:** crowds self-distribute naturally
6. **US-7.7.6.6** — I want crowd NPCs at a market or event to maintain personal space without
   clipping
   - **Acceptance:** gatherings look realistic
7. **US-7.7.6.7** — I want to monitor crowd density per spatial cell and enforce caps
   - **Acceptance:** overflow triggers redirect or despawn
8. **US-7.7.6.8** — I want to redirect overflow agents to alternate routes when density exceeds the
   threshold
   - **Acceptance:** crowds self-balance
9. **US-7.7.6.9** — I want mobile to enforce lower per-cell density caps (4 vs. 16 on desktop) and
   despawn overflow more aggressively
   - **Acceptance:** crowd count stays within budget
10. **US-7.7.6.10** — I want to verify that per-cell density never exceeds the configured cap
    - **Acceptance:** density management enforcement is correct
11. **US-7.7.6.11** — I want to verify that redirected agents successfully reach their goal via
    alternate paths
    - **Acceptance:** redirection does not strand agents
12. **US-7.7.6.12** — I want to benchmark the CPU overhead of density monitoring and management
    - **Acceptance:** it fits within the crowd simulation budget. ---

## F-7.7.7 — Knowledge Sharing and Event Propagation

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.7.7.1  | designer (P-5)          | F-7.7.7  | R-7.7.7      |
| US-7.7.7.2  | designer (P-5)          | F-7.7.7  | R-7.7.7      |
| US-7.7.7.3  | designer (P-5)          | F-7.7.7  | R-7.7.7      |
| US-7.7.7.4  | player (P-23)           | F-7.7.7  | R-7.7.7      |
| US-7.7.7.5  | player (P-23)           | F-7.7.7  | R-7.7.7      |
| US-7.7.7.6  | player (P-23)           | F-7.7.7  | R-7.7.7      |
| US-7.7.7.7  | engine developer (P-26) | F-7.7.7  | R-7.7.7      |
| US-7.7.7.8  | engine developer (P-26) | F-7.7.7  | R-7.7.7      |
| US-7.7.7.9  | engine developer (P-26) | F-7.7.7  | R-7.7.7      |
| US-7.7.7.10 | engine tester (P-27)    | F-7.7.7  | R-7.7.7      |
| US-7.7.7.11 | engine tester (P-27)    | F-7.7.7  | R-7.7.7      |
| US-7.7.7.12 | engine tester (P-27)    | F-7.7.7  | R-7.7.7      |

1. **US-7.7.7.1** — I want to configure the communication radius for knowledge sharing per faction
   - **Acceptance:** guard barracks and patrols share alerts within appropriate range
2. **US-7.7.7.2** — I want to define communication types (immediate alert, investigation request,
   all-clear, report)
   - **Acceptance:** shared knowledge triggers appropriate responses
3. **US-7.7.7.3** — I want to trigger a detection event and verify it propagates to all allies
   within range
   - **Acceptance:** knowledge sharing works end-to-end
4. **US-7.7.7.4** — I want the guard who spots me to alert all guards in the barracks within
   communication range
   - **Acceptance:** stealth challenges feel realistic
5. **US-7.7.7.5** — I want only the nearest idle guard to investigate a suspicious sound while
   others hold position
   - **Acceptance:** AI teams respond without overcommitting
6. **US-7.7.7.6** — I want guards to stand down after an all-clear broadcast when investigation
   finds nothing
   - **Acceptance:** alert states resolve naturally
7. **US-7.7.7.7** — I want to implement a knowledge event system that broadcasts to allied agents
   within communication radius
   - **Acceptance:** perception data is shared
8. **US-7.7.7.8** — I want to use the faction affinity system to determine broadcast recipients
   - **Acceptance:** only same-faction and allied agents receive knowledge
9. **US-7.7.7.9** — I want mobile to limit broadcast radius to 15 m (vs. 50 m desktop) and cap
   recipients to 8
   - **Acceptance:** broadcast cost is bounded
10. **US-7.7.7.10** — I want to verify that knowledge broadcasts reach all allied agents within
    communication radius
    - **Acceptance:** no allies are missed
11. **US-7.7.7.11** — I want to verify that agents outside the communication radius do not receive
    broadcasts
    - **Acceptance:** range limiting works
12. **US-7.7.7.12** — I want to verify that shared knowledge decays at the same rate as direct
    perception
    - **Acceptance:** second-hand information fades appropriately. ---

## F-7.7.8 — Shared Awareness and Synchronized Group Reactions

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.7.8.1  | designer (P-5)          | F-7.7.8  | R-7.7.8      |
| US-7.7.8.2  | designer (P-5)          | F-7.7.8  | R-7.7.8      |
| US-7.7.8.3  | designer (P-5)          | F-7.7.8  | R-7.7.8      |
| US-7.7.8.4  | player (P-23)           | F-7.7.8  | R-7.7.8      |
| US-7.7.8.5  | player (P-23)           | F-7.7.8  | R-7.7.8      |
| US-7.7.8.6  | player (P-23)           | F-7.7.8  | R-7.7.8      |
| US-7.7.8.7  | engine developer (P-26) | F-7.7.8  | R-7.7.8      |
| US-7.7.8.8  | engine developer (P-26) | F-7.7.8  | R-7.7.8      |
| US-7.7.8.9  | engine developer (P-26) | F-7.7.8  | R-7.7.8      |
| US-7.7.8.10 | engine tester (P-27)    | F-7.7.8  | R-7.7.8      |
| US-7.7.8.11 | engine tester (P-27)    | F-7.7.8  | R-7.7.8      |
| US-7.7.8.12 | engine tester (P-27)    | F-7.7.8  | R-7.7.8      |

1. **US-7.7.8.1** — I want to configure group reaction types per creature archetype (birds scatter,
   herds stampede, civilians flee, guards form line)
   - **Acceptance:** each group reacts characteristically
2. **US-7.7.8.2** — I want to set the alarm propagation delay per group type
   - **Acceptance:** the reaction wave looks natural for the creature
3. **US-7.7.8.3** — I want to configure how long after the threat clears a group takes to reassemble
   - **Acceptance:** recovery pace matches the creature's behavior
4. **US-7.7.8.4** — I want a flock of birds to scatter in a wave from nearest to farthest as I
   approach, then regroup at a safe distance
   - **Acceptance:** wildlife feels alive
5. **US-7.7.8.5** — I want a herd to stampede as a group when a predator appears, maintaining
   cohesion during flight
   - **Acceptance:** herd behavior looks coordinated
6. **US-7.7.8.6** — I want civilians to panic and flee toward building exits when combat breaks out
   - **Acceptance:** bystander reactions feel realistic
7. **US-7.7.8.7** — I want to implement alarm propagation with spatial delay so members farther from
   the trigger react later
   - **Acceptance:** a natural wave effect emerges
8. **US-7.7.8.8** — I want individuals that fall behind during group flight to accelerate and rejoin
   - **Acceptance:** groups remain cohesive during panic
9. **US-7.7.8.9** — I want mobile to use instant-uniform alarm propagation (skipping wave delay)
   - **Acceptance:** group reaction is simpler on constrained devices
10. **US-7.7.8.10** — I want to verify that alarm propagates from nearest to farthest member with
    the configured delay
    - **Acceptance:** wave order is correct
11. **US-7.7.8.11** — I want to verify that groups reassemble at a rally point after the threat
    clears within the configured calm-down period
    - **Acceptance:** recovery works
12. **US-7.7.8.12** — I want to verify that all configured group reaction types (scatter, stampede,
    flee, form line) produce the expected behavior patterns
    - **Acceptance:** each archetype reacts correctly. ---

## F-7.7.9 — Faction-Based Behavioral Relationships

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-7.7.9.1  | designer (P-5)          | F-7.7.9  | R-7.7.9      |
| US-7.7.9.2  | designer (P-5)          | F-7.7.9  | R-7.7.9      |
| US-7.7.9.3  | designer (P-5)          | F-7.7.9  | R-7.7.9      |
| US-7.7.9.4  | player (P-23)           | F-7.7.9  | R-7.7.9      |
| US-7.7.9.5  | player (P-23)           | F-7.7.9  | R-7.7.9      |
| US-7.7.9.6  | player (P-23)           | F-7.7.9  | R-7.7.9      |
| US-7.7.9.7  | engine developer (P-26) | F-7.7.9  | R-7.7.9      |
| US-7.7.9.8  | engine developer (P-26) | F-7.7.9  | R-7.7.9      |
| US-7.7.9.9  | engine developer (P-26) | F-7.7.9  | R-7.7.9      |
| US-7.7.9.10 | engine tester (P-27)    | F-7.7.9  | R-7.7.9      |
| US-7.7.9.11 | engine tester (P-27)    | F-7.7.9  | R-7.7.9      |
| US-7.7.9.12 | engine tester (P-27)    | F-7.7.9  | R-7.7.9      |

1. **US-7.7.9.1** — I want to define a faction affinity matrix with relationship types (aggressive,
   hostile, wary, neutral, friendly, allied)
   - **Acceptance:** NPC behavior is driven by faction relations
2. **US-7.7.9.2** — I want to modify faction relationships at runtime via gameplay events (quest
   completion, betrayal)
   - **Acceptance:** the world reacts to player choices
3. **US-7.7.9.3** — I want to set per-NPC relationship overrides that deviate from faction defaults
   (a friendly orc in a hostile faction)
   - **Acceptance:** unique characters are supported
4. **US-7.7.9.4** — I want NPCs from a hostile faction to attack me on sight
   - **Acceptance:** faction hostility is immediately visible
5. **US-7.7.9.5** — I want wary NPCs to warn me before turning hostile if I continue trespassing
   - **Acceptance:** faction interactions feel nuanced with escalation
6. **US-7.7.9.6** — I want NPCs who previously attacked me to become friendly after my reputation
   improves
   - **Acceptance:** my actions visibly affect world dynamics
7. **US-7.7.9.7** — I want to implement a faction affinity matrix storing relationship types that
   determine engagement rules, communication, and trade availability
   - **Acceptance:** behavior is relationship-driven
8. **US-7.7.9.8** — I want to support runtime modification of the affinity matrix by gameplay
   systems
   - **Acceptance:** relationships evolve dynamically
9. **US-7.7.9.9** — I want to support per-NPC relationship overrides that take precedence over
   faction defaults
   - **Acceptance:** unique NPCs can deviate from their group
10. **US-7.7.9.10** — I want to verify that each relationship type (aggressive, hostile, wary,
    neutral, friendly, allied) produces the expected behavioral response
    - **Acceptance:** all relationship modes work
11. **US-7.7.9.11** — I want to verify that changing a faction relationship at runtime immediately
    changes NPC behavior for that faction
    - **Acceptance:** updates propagate
12. **US-7.7.9.12** — I want to verify that a per-NPC override takes precedence over the faction
    default
    - **Acceptance:** unique NPCs behave correctly. ---

## F-7.7.10 — Threat Table and Aggro Targeting

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-7.7.10.1  | designer (P-5)          | F-7.7.10 | R-7.7.10     |
| US-7.7.10.2  | designer (P-5)          | F-7.7.10 | R-7.7.10     |
| US-7.7.10.3  | designer (P-5)          | F-7.7.10 | R-7.7.10     |
| US-7.7.10.4  | player (P-23)           | F-7.7.10 | R-7.7.10     |
| US-7.7.10.5  | player (P-23)           | F-7.7.10 | R-7.7.10     |
| US-7.7.10.6  | player (P-23)           | F-7.7.10 | R-7.7.10     |
| US-7.7.10.7  | engine developer (P-26) | F-7.7.10 | R-7.7.10     |
| US-7.7.10.8  | engine developer (P-26) | F-7.7.10 | R-7.7.10     |
| US-7.7.10.9  | engine developer (P-26) | F-7.7.10 | R-7.7.10     |
| US-7.7.10.10 | engine tester (P-27)    | F-7.7.10 | R-7.7.10     |
| US-7.7.10.11 | engine tester (P-27)    | F-7.7.10 | R-7.7.10     |
| US-7.7.10.12 | engine tester (P-27)    | F-7.7.10 | R-7.7.10     |

1. **US-7.7.10.1** — I want to configure threat sources (damage, healing, taunts, debuffs,
   proximity) and their weights
   - **Acceptance:** the threat table reflects my game's combat balance
2. **US-7.7.10.2** — I want to set the aggro transfer threshold (110% melee, 130% ranged)
   - **Acceptance:** tank gameplay has tunable difficulty
3. **US-7.7.10.3** — I want to configure targeting behavior per AI archetype (berserker, tactical,
   protector, coward)
   - **Acceptance:** different enemies require different strategies
4. **US-7.7.10.4** — I want enemies to attack me while my threat stays highest, with taunts
   generating burst threat
   - **Acceptance:** the tank role functions correctly
5. **US-7.7.10.5** — I want tactical enemies to target the lowest-HP party member
   - **Acceptance:** I must protect vulnerable allies
6. **US-7.7.10.6** — I want threat to decay when I move out of combat range
   - **Acceptance:** disengaging from combat is possible over time
7. **US-7.7.10.7** — I want to implement a per-enemy threat table tracking hate from each attacker,
   selecting the highest-threat target
   - **Acceptance:** aggro targeting is data-driven
8. **US-7.7.10.8** — I want to support threat modifiers on abilities (low threat heals, high threat
   taunts)
   - **Acceptance:** class roles can manage threat
9. **US-7.7.10.9** — I want to expose the threat table to behavior trees for conditional target
   switching and to the HUD for tank threat indicators
   - **Acceptance:** threat data is accessible
10. **US-7.7.10.10** — I want to verify that the AI always attacks the highest-threat target
    - **Acceptance:** basic threat table targeting works
11. **US-7.7.10.11** — I want to verify that aggro does not transfer until the new threat exceeds
    the current target by the configured threshold
    - **Acceptance:** aggro is stable
12. **US-7.7.10.12** — I want to verify that each AI archetype (berserker, tactical, protector,
    coward) selects the correct target based on its configured behavior
    - **Acceptance:** archetype targeting is correct. ---

## F-7.7.11 — Animal Tracking and Hunting Behaviors

| ID           | Persona                 | Features | Requirements |
|--------------|-------------------------|----------|--------------|
| US-7.7.11.1  | designer (P-5)          | F-7.7.11 | R-7.7.11     |
| US-7.7.11.2  | designer (P-5)          | F-7.7.11 | R-7.7.11     |
| US-7.7.11.3  | designer (P-5)          | F-7.7.11 | R-7.7.11     |
| US-7.7.11.4  | player (P-23)           | F-7.7.11 | R-7.7.11     |
| US-7.7.11.5  | player (P-23)           | F-7.7.11 | R-7.7.11     |
| US-7.7.11.6  | player (P-23)           | F-7.7.11 | R-7.7.11     |
| US-7.7.11.7  | engine developer (P-26) | F-7.7.11 | R-7.7.11     |
| US-7.7.11.8  | engine developer (P-26) | F-7.7.11 | R-7.7.11     |
| US-7.7.11.9  | engine developer (P-26) | F-7.7.11 | R-7.7.11     |
| US-7.7.11.10 | engine tester (P-27)    | F-7.7.11 | R-7.7.11     |
| US-7.7.11.11 | engine tester (P-27)    | F-7.7.11 | R-7.7.11     |
| US-7.7.11.12 | engine tester (P-27)    | F-7.7.11 | R-7.7.11     |

1. **US-7.7.11.1** — I want to configure predator-prey pairs with hunting success rates
   - **Acceptance:** ecological balance is maintained across wildlife species
2. **US-7.7.11.2** — I want to configure predator phases (patrol, detect, stalk, ambush, chase,
   kill, feed)
   - **Acceptance:** hunting behavior is rich and sequenced
3. **US-7.7.11.3** — I want to configure prey defensive behaviors (flee, form protective circle,
   stand and fight)
   - **Acceptance:** different prey species respond differently
4. **US-7.7.11.4** — I want a wolf pack to stalk downwind, with drivers flushing prey toward
   ambushers
   - **Acceptance:** pack hunting feels realistic and coordinated
5. **US-7.7.11.5** — I want a deer herd to form protective circles with mothers shielding young when
   a predator approaches
   - **Acceptance:** prey behavior is as rich as predator behavior
6. **US-7.7.11.6** — I want a predator to give up the chase when the prey outruns it, managing
   stamina
   - **Acceptance:** hunts have realistic outcomes
7. **US-7.7.11.7** — I want to implement the predator behavior state machine (patrol, detect, stalk,
   ambush, chase, kill, feed)
   - **Acceptance:** hunting phases are well-defined
8. **US-7.7.11.8** — I want to implement coordinated pack hunting with driver and ambusher roles
   - **Acceptance:** pack members cooperate during hunts
9. **US-7.7.11.9** — I want mobile to use simplified hunting (sight-only tracking, no pack
   coordination, no scent/footprint following)
   - **Acceptance:** wildlife simulation fits the mobile budget
10. **US-7.7.11.10** — I want to run 100+ hunting simulations and verify the success rate matches
    the configured value within tolerance
    - **Acceptance:** ecological balance is correct
11. **US-7.7.11.11** — I want to verify that a predator ends the chase when its stamina depletes
    - **Acceptance:** stamina management works
12. **US-7.7.11.12** — I want to verify that pack members are assigned driver and ambusher roles and
    execute their assigned behaviors
    - **Acceptance:** coordination is correct
