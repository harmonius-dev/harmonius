# User Stories — 7.2 Steering & Avoidance

## F-7.2.1 — RVO/ORCA Local Avoidance

## US-7.2.1.1 Configure ORCA Neighbor Count in Editor
**As a** designer (P-5), **I want to** configure the ORCA neighbor count and query radius per
agent archetype, **so that** avoidance quality balances with performance for each NPC type.

## US-7.2.1.2 Test Crowd Flow at Chokepoints
**As a** designer (P-5), **I want to** spawn many agents at a narrow chokepoint and observe
smooth flow without pileups, **so that** I can tune ORCA parameters for dense areas.

## US-7.2.1.3 Adjust Avoidance Aggressiveness Per Archetype
**As a** designer (P-5), **I want to** adjust how aggressively agents avoid each other (tight
for guards, loose for civilians), **so that** different NPCs have distinct movement feel.

## US-7.2.1.4 See NPCs Navigate Crowded Streets Without Overlapping
**As a** player (P-23), **I want** NPCs in crowded city streets to navigate smoothly without
walking through each other, **so that** crowds feel realistic.

## US-7.2.1.5 Watch Guards File Through Doorways Orderly
**As a** player (P-23), **I want** groups of guards to file through doorways one at a time
instead of clumping, **so that** tight spaces feel physically plausible.

## US-7.2.1.6 See Enemies Spread Out During Approach
**As a** player (P-23), **I want** approaching enemies to spread out instead of stacking on
top of each other, **so that** combat encounters look tactically distributed.

## US-7.2.1.7 Implement ORCA Velocity Obstacle Computation
**As an** engine developer (P-26), **I want to** implement Optimal Reciprocal Collision
Avoidance with velocity obstacle computation, **so that** agents produce deadlock-free
movement.

## US-7.2.1.8 Batch ORCA Neighbor Queries via Spatial Index
**As an** engine developer (P-26), **I want to** batch neighbor queries using the shared
spatial index, **so that** ORCA scales efficiently with agent count.

## US-7.2.1.9 Limit ORCA Neighbors on Mobile
**As an** engine developer (P-26), **I want to** limit ORCA neighbor count to 4 on mobile (vs.
12 on desktop), **so that** per-agent avoidance cost fits within the mobile CPU budget.

## US-7.2.1.10 Verify No Agent Overlap in Dense Scenarios
**As an** engine tester (P-27), **I want to** verify that no two agents overlap positions in a
stress test with 200+ agents in a confined area, **so that** ORCA prevents interpenetration.

## US-7.2.1.11 Test Deadlock Resolution at Narrow Passages
**As an** engine tester (P-27), **I want to** verify that two groups approaching from opposite
directions through a narrow passage resolve without deadlock, **so that** ORCA is robust.

## US-7.2.1.12 Benchmark ORCA Cost Per Agent
**As an** engine tester (P-27), **I want to** benchmark per-agent ORCA computation time across
platform tiers, **so that** avoidance cost stays within the per-agent CPU budget.

---

## F-7.2.2 — Obstacle Avoidance (Static Geometry)

## US-7.2.2.1 Configure Feeler Ray Count Per Agent
**As a** designer (P-5), **I want to** configure the number of feeler rays per agent archetype,
**so that** avoidance precision matches the agent's movement requirements.

## US-7.2.2.2 Test Wall-Hugging Behavior in Corridors
**As a** designer (P-5), **I want to** test agents walking through corridors and verify they
do not clip through walls, **so that** geometry avoidance works for indoor environments.

## US-7.2.2.3 Tune Feeler Ray Length for Large Agents
**As a** designer (P-5), **I want to** tune feeler ray length per agent size class, **so that**
large agents detect walls earlier and steer with wider clearance.

## US-7.2.2.4 See NPCs Avoid Walls and Pillars
**As a** player (P-23), **I want** NPCs to smoothly steer around walls and pillars without
bumping into them, **so that** indoor movement feels natural.

## US-7.2.2.5 Watch AI Navigate Around Terrain Edges
**As a** player (P-23), **I want** AI agents to steer away from cliff edges and terrain drops,
**so that** they do not walk off ledges unnaturally.

## US-7.2.2.6 Not See NPCs Get Stuck on Geometry Corners
**As a** player (P-23), **I want** NPCs to slide around sharp geometry corners without getting
stuck, **so that** movement remains fluid in complex interiors.

## US-7.2.2.7 Implement Feeler Ray Obstacle Detection
**As an** engine developer (P-26), **I want to** cast short-range feeler rays against nearby
static geometry, **so that** agents receive correction forces before contacting walls.

## US-7.2.2.8 Layer Obstacle Avoidance After ORCA
**As an** engine developer (P-26), **I want to** apply static obstacle avoidance as a final
correction layer after ORCA, **so that** both agent-agent and agent-geometry avoidance work.

## US-7.2.2.9 Reduce Feeler Count on Mobile
**As an** engine developer (P-26), **I want to** reduce feeler rays to 2 on mobile (vs. 5 on
desktop), **so that** raycast cost fits within the mobile per-agent budget.

## US-7.2.2.10 Verify Agents Never Clip Through Static Geometry
**As an** engine tester (P-27), **I want to** verify that agents never clip through static
geometry on a reference level with tight corridors, **so that** obstacle avoidance is airtight.

## US-7.2.2.11 Test Low-LOD Agents Skip Obstacle Avoidance
**As an** engine tester (P-27), **I want to** confirm that low-LOD agents skip obstacle
avoidance entirely as designed, **so that** the LOD system correctly gates expensive behaviors.

## US-7.2.2.12 Benchmark Feeler Ray Cost Across Platforms
**As an** engine tester (P-27), **I want to** benchmark feeler ray computation cost per agent
across platforms, **so that** the raycast budget scales appropriately.

---

## F-7.2.3 — Core Steering Behaviors

## US-7.2.3.1 Assign Steering Behaviors to Agent Archetypes
**As a** designer (P-5), **I want to** assign steering behaviors (seek, flee, arrive, wander,
pursuit, evade) to agent archetypes in the editor, **so that** each NPC type moves
distinctively.

## US-7.2.3.2 Tune Arrive Deceleration Radius
**As a** designer (P-5), **I want to** tune the arrive behavior's deceleration radius per
agent, **so that** NPCs slow down gracefully when reaching their destination.

## US-7.2.3.3 Configure Wander Randomness Parameters
**As a** designer (P-5), **I want to** configure wander jitter, radius, and distance
parameters, **so that** idle NPCs meander naturally without leaving their assigned area.

## US-7.2.3.4 See Guards Patrol and Wander Naturally
**As a** player (P-23), **I want** town guards to patrol with natural wandering when idle,
**so that** NPC movement looks organic rather than robotic.

## US-7.2.3.5 Watch Enemies Pursue with Prediction
**As a** player (P-23), **I want** pursuing enemies to intercept my predicted position instead
of chasing where I was, **so that** combat AI feels smart and challenging.

## US-7.2.3.6 See Fleeing NPCs Run Away Intelligently
**As a** player (P-23), **I want** fleeing NPCs to run away from threats using predictive
evasion, **so that** their escape behavior looks intentional.

## US-7.2.3.7 Implement Steering Primitives as Composable Forces
**As an** engine developer (P-26), **I want to** implement seek, flee, arrive, wander, pursuit,
and evade as composable steering force functions, **so that** they can be combined flexibly.

## US-7.2.3.8 Return Steering Force Vectors for Blending
**As an** engine developer (P-26), **I want** each steering behavior to return a force vector,
**so that** the blending system can combine multiple active behaviors per agent.

## US-7.2.3.9 Optimize Steering with SIMD Vector Math
**As an** engine developer (P-26), **I want to** use SIMD-optimized vector math for steering
computations, **so that** thousands of agents can be processed per tick efficiently.

## US-7.2.3.10 Verify Arrive Behavior Stops at Destination
**As an** engine tester (P-27), **I want to** verify that the arrive behavior brings agents
to a complete stop at the target position within a tolerance, **so that** arrival is precise.

## US-7.2.3.11 Test Pursuit Intercepts Moving Target
**As an** engine tester (P-27), **I want to** verify that pursuit behavior intercepts a
linearly moving target faster than naive seek, **so that** prediction works correctly.

## US-7.2.3.12 Validate Wander Stays Within Bounds
**As an** engine tester (P-27), **I want to** verify that wander behavior keeps agents within
a configurable area over extended simulation, **so that** wandering does not drift unbounded.

---

## F-7.2.4 — Steering Behavior Blending & Priority

## US-7.2.4.1 Configure Blending Weights Per Behavior
**As a** designer (P-5), **I want to** set blending weights for each active steering behavior
per agent archetype, **so that** avoidance can outweigh seek when needed.

## US-7.2.4.2 Choose Between Weighted and Priority Blending
**As a** designer (P-5), **I want to** choose between weighted blending and priority pipeline
modes per agent, **so that** different NPCs use the strategy best suited to their role.

## US-7.2.4.3 Preview Blended Steering Vectors in Debug View
**As a** designer (P-5), **I want to** visualize each steering force component and the final
blended vector as debug arrows, **so that** I can diagnose conflicting behaviors.

## US-7.2.4.4 See NPCs Balance Pursuit and Avoidance
**As a** player (P-23), **I want** pursuing enemies to avoid obstacles while chasing me instead
of running into walls, **so that** pursuit feels intelligent.

## US-7.2.4.5 Watch Fleeing NPCs Avoid Crowds While Escaping
**As a** player (P-23), **I want** fleeing NPCs to avoid other NPCs while running away, not
bulldoze through crowds, **so that** escape behavior looks realistic.

## US-7.2.4.6 See Smooth Transitions Between Behaviors
**As a** player (P-23), **I want** NPC movement transitions (idle to pursuit to arrival) to be
smooth without jerky direction changes, **so that** blending looks natural.

## US-7.2.4.7 Implement Weighted and Priority Blending Systems
**As an** engine developer (P-26), **I want to** implement both weighted-sum and priority
pipeline blending, **so that** designers can choose the best mode per scenario.

## US-7.2.4.8 Allocate Remaining Magnitude in Priority Mode
**As an** engine developer (P-26), **I want** priority blending to allocate remaining steering
magnitude to lower-priority layers, **so that** high-priority forces are satisfied first.

## US-7.2.4.9 Limit Active Steering Layers on Mobile
**As an** engine developer (P-26), **I want to** limit active steering layers to 3 on mobile
(vs. 5 on desktop), **so that** per-agent blending cost is reduced.

## US-7.2.4.10 Verify Priority Blending Prevents Force Cancellation
**As an** engine tester (P-27), **I want to** verify that priority blending prevents
conflicting forces from canceling out, **so that** agents always produce meaningful movement.

## US-7.2.4.11 Test Weighted Blending Produces Expected Vectors
**As an** engine tester (P-27), **I want to** test weighted blending with known inputs and
verify the output vector matches the expected weighted sum, **so that** blending math is
correct.

## US-7.2.4.12 Benchmark Blending Cost with Maximum Active Layers
**As an** engine tester (P-27), **I want to** benchmark blending cost with the maximum number
of active layers per agent, **so that** worst-case performance is within budget.

---

## F-7.2.5 — Formation Movement

## US-7.2.5.1 Assign Formation Shapes in Editor
**As a** designer (P-5), **I want to** assign formation shapes (line, wedge, column, circle)
to agent groups in the editor, **so that** squads move in organized patterns.

## US-7.2.5.2 Configure Formation Spacing and Slot Count
**As a** designer (P-5), **I want to** configure spacing between slots and the number of slots
per formation, **so that** formations fit different group sizes and terrain widths.

## US-7.2.5.3 Test Formation Adjustment in Narrow Terrain
**As a** designer (P-5), **I want to** observe formations automatically adjust spacing when
terrain narrows, **so that** agents do not clip through walls to maintain position.

## US-7.2.5.4 See Guard Squads March in Formation
**As a** player (P-23), **I want** guard squads to march in wedge or column formation through
the city, **so that** military NPCs look organized and disciplined.

## US-7.2.5.5 Watch Formation Reform After Disruption
**As a** player (P-23), **I want** a formation to reform after being disrupted by combat or
obstacles, **so that** group movement recovers gracefully.

## US-7.2.5.6 See Followers Maintain Offset Behind Leader
**As a** player (P-23), **I want** formation members to maintain their offset positions
relative to the leader as the group turns, **so that** formation integrity is visually
convincing.

## US-7.2.5.7 Implement Parameterized Formation Shapes
**As an** engine developer (P-26), **I want to** implement formation shapes as parameterized
slot layouts with a leader and follower offsets, **so that** shapes are data-driven and
extensible.

## US-7.2.5.8 Drive Followers with Arrival Steering
**As an** engine developer (P-26), **I want** followers to use arrival steering to maintain
their formation offset, **so that** slot tracking is smooth and responsive.

## US-7.2.5.9 Handle Partial Formation Destruction
**As an** engine developer (P-26), **I want** formations to automatically reassign slots when
members are lost, **so that** the remaining members maintain a coherent reduced formation.

## US-7.2.5.10 Verify Formation Spacing Matches Configuration
**As an** engine tester (P-27), **I want to** verify that formation member spacing matches the
configured values within tolerance, **so that** formations look as designed.

## US-7.2.5.11 Test Formation Through Narrow Passage
**As an** engine tester (P-27), **I want to** test a formation passing through a narrow
passage and verify that spacing adjusts without clipping, **so that** adaptation works.

## US-7.2.5.12 Validate Formation Recovery After Member Loss
**As an** engine tester (P-27), **I want to** remove formation members and verify remaining
members reassign to valid slots, **so that** partial formations remain coherent.

---

## F-7.2.6 — Group Steering & Cohesion

## US-7.2.6.1 Create Named Agent Groups in Editor
**As a** designer (P-5), **I want to** create named agent groups in the editor and assign
members, **so that** group steering applies to designated NPC teams.

## US-7.2.6.2 Tune Group Cohesion and Alignment Strength
**As a** designer (P-5), **I want to** tune cohesion and alignment correction strength per
group, **so that** tight squads stay close while loose patrols allow more spread.

## US-7.2.6.3 Preview Group Centroid and Heading in Debug View
**As a** designer (P-5), **I want to** see the group centroid and average heading as debug
gizmos, **so that** I can verify group steering is working correctly.

## US-7.2.6.4 See Patrol Groups Stay Together Around Corners
**As a** player (P-23), **I want** patrol groups to stay together when rounding corners instead
of fragmenting, **so that** group movement looks cohesive.

## US-7.2.6.5 Watch Escort Groups Match Player Speed
**As a** player (P-23), **I want** escort NPC groups to maintain cohesion while matching my
movement speed, **so that** the escort feels responsive and unified.

## US-7.2.6.6 See Scattered Group Members Regroup
**As a** player (P-23), **I want** scattered group members to gradually regroup after being
separated by obstacles, **so that** groups reform naturally.

## US-7.2.6.7 Implement Group Centroid and Heading Tracker
**As an** engine developer (P-26), **I want to** maintain a lightweight group centroid and
average heading tracker, **so that** cohesion and alignment corrections are computed
efficiently.

## US-7.2.6.8 Apply Cohesion Without Full Flocking Model
**As an** engine developer (P-26), **I want to** apply cohesion and alignment corrections
using the simplified group tracker rather than full flocking, **so that** group steering is
cheaper than crowd simulation.

## US-7.2.6.9 Scale Group Steering with Platform Crowd Budget
**As an** engine developer (P-26), **I want** group size to be limited by the platform crowd
budget, **so that** group steering cost scales appropriately per platform.

## US-7.2.6.10 Verify Group Does Not Fragment During Turns
**As an** engine tester (P-27), **I want to** verify that a group navigating a series of turns
maintains cohesion within a configurable radius, **so that** fragmentation does not occur.

## US-7.2.6.11 Test Group Velocity Goal Convergence
**As an** engine tester (P-27), **I want to** verify that all group members converge to a
shared velocity goal within a bounded time, **so that** group cohesion stabilizes.

## US-7.2.6.12 Benchmark Group Steering Overhead Per Member
**As an** engine tester (P-27), **I want to** benchmark the CPU cost of group steering per
member, **so that** I can verify it is cheaper than full flocking on equivalent group sizes.
