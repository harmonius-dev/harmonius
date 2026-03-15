# User Stories — 7.7 Crowd Simulation

## F-7.7.1 — Flocking Behaviors (Separation, Alignment, Cohesion)

## US-7.7.1.1 Configure Flocking Weights Per Crowd Archetype
**As a** designer (P-5), **I want to** configure separation, alignment, and cohesion weights
per crowd archetype, **so that** tight military columns, loose herds, and scattered civilians
each have distinct group movement.

## US-7.7.1.2 Set Neighbor Query Radius
**As a** designer (P-5), **I want to** set the neighbor query radius per archetype, **so that**
flocking range matches the intended group tightness.

## US-7.7.1.3 Preview Flocking Forces in Debug View
**As a** designer (P-5), **I want to** see separation, alignment, and cohesion force vectors
as debug arrows on each agent, **so that** I can tune weights visually.

## US-7.7.1.4 See Herds Move as Cohesive Groups
**As a** player (P-23), **I want** animal herds to move as a cohesive group rather than as
disconnected individuals, **so that** wildlife feels alive and natural.

## US-7.7.1.5 Watch Birds Flock in V Formation
**As a** player (P-23), **I want** flocks of birds to move in recognizable patterns with
alignment and cohesion, **so that** aerial wildlife looks realistic.

## US-7.7.1.6 See Crowds Avoid Bumping Into Each Other
**As a** player (P-23), **I want** crowd NPCs to maintain personal space via separation
forces, **so that** dense crowds do not clip through each other.

## US-7.7.1.7 Implement Reynolds Flocking with Three Forces
**As an** engine developer (P-26), **I want to** implement Reynolds flocking with weighted
separation, alignment, and cohesion forces, **so that** group movement emerges from simple
rules.

## US-7.7.1.8 Query Neighbors via Spatial Index
**As an** engine developer (P-26), **I want to** query neighbors via the shared spatial index
for flocking, **so that** neighbor lookups are efficient.

## US-7.7.1.9 Reduce Neighbor Radius on Mobile
**As an** engine developer (P-26), **I want to** reduce the neighbor query radius on mobile (8
m vs. 16 m desktop), **so that** per-agent flocking cost fits the mobile budget.

## US-7.7.1.10 Verify Separation Prevents Agent Overlap
**As an** engine tester (P-27), **I want to** verify that separation forces prevent agents
from overlapping within a flock, **so that** spatial distribution is correct.

## US-7.7.1.11 Test Cohesion Keeps Group Together
**As an** engine tester (P-27), **I want to** verify that cohesion forces keep flock members
within a configurable radius of the group centroid, **so that** groups do not fragment.

## US-7.7.1.12 Benchmark Flocking for Maximum Flock Size
**As an** engine tester (P-27), **I want to** benchmark flocking computation for the maximum
flock size per platform, **so that** flocking cost is within budget.

---

## F-7.7.2 — Flow Field Navigation

## US-7.7.2.1 Generate Flow Fields from Goal Positions
**As a** designer (P-5), **I want to** generate flow fields from goal positions in the editor,
**so that** I can preview mass crowd movement toward points of interest.

## US-7.7.2.2 Set Flow Field Resolution Per Platform
**As a** designer (P-5), **I want to** set flow field cell resolution (1 m desktop, 2 m
mobile), **so that** crowd navigation quality scales with platform capability.

## US-7.7.2.3 Assign Goals to Crowd Groups
**As a** designer (P-5), **I want to** assign goal positions to crowd groups and see them flow
toward the goal, **so that** I can orchestrate mass crowd events.

## US-7.7.2.4 See Thousands of NPCs Move Toward a Goal
**As a** player (P-23), **I want** thousands of crowd NPCs to move smoothly toward a gate,
market, or event, **so that** mass movement looks realistic and purposeful.

## US-7.7.2.5 Watch Crowds Route Around Obstacles via Flow Fields
**As a** player (P-23), **I want** crowd NPCs to flow around buildings and obstacles along
the cheapest path, **so that** mass navigation looks natural.

## US-7.7.2.6 See Crowd Direction Change Smoothly
**As a** player (P-23), **I want** crowd flow direction to change smoothly when goals update,
**so that** redirected crowds do not snap or teleport.

## US-7.7.2.7 Implement Dijkstra-Based Flow Field Generation
**As an** engine developer (P-26), **I want to** implement 2D grid-based flow fields using
Dijkstra propagation from goal positions, **so that** each cell stores the cheapest path
direction.

## US-7.7.2.8 Enable Constant Per-Agent Sampling Cost
**As an** engine developer (P-26), **I want** crowd agents to sample the flow field directly
instead of running individual A* queries, **so that** per-agent navigation cost is constant.

## US-7.7.2.9 Limit Concurrent Flow Fields on Mobile
**As an** engine developer (P-26), **I want to** limit concurrent active flow fields to 4 on
mobile, **so that** flow field memory and computation fit within mobile constraints.

## US-7.7.2.10 Verify Flow Field Directions Point Toward Goal
**As an** engine tester (P-27), **I want to** verify that following flow field directions from
any cell reaches the goal position, **so that** path correctness is confirmed.

## US-7.7.2.11 Test Flow Field Updates When Obstacles Change
**As an** engine tester (P-27), **I want to** verify that flow fields recompute correctly when
dynamic obstacles modify the cost grid, **so that** redirected flows are accurate.

## US-7.7.2.12 Benchmark Flow Field Generation Time
**As an** engine tester (P-27), **I want to** benchmark flow field generation time for the
maximum tile size, **so that** recomputation fits within the per-tick budget.

---

## F-7.7.3 — Flow Field Streaming & Caching

## US-7.7.3.1 Configure Flow Field Cache Size
**As a** designer (P-5), **I want to** configure the flow field cache size per platform,
**so that** frequently used fields are retained without exceeding memory.

## US-7.7.3.2 Preview Cached Flow Fields in Editor
**As a** designer (P-5), **I want to** see which flow fields are currently cached in a debug
panel, **so that** I can verify cache utilization.

## US-7.7.3.3 Set Cache Invalidation on Obstacle Change
**As a** designer (P-5), **I want** cached flow fields to be invalidated when dynamic
obstacles modify the cost grid, **so that** stale directions are never used.

## US-7.7.3.4 See Crowds Navigate Seamlessly Across Zones
**As a** player (P-23), **I want** crowd NPCs to navigate seamlessly across world streaming
boundaries, **so that** crowd flow is continuous.

## US-7.7.3.5 Watch Crowds Redirect Quickly After Obstacle
**As a** player (P-23), **I want** crowds to redirect quickly when a new obstacle appears
(gate closes, barricade placed), **so that** flow field invalidation is responsive.

## US-7.7.3.6 Not See Crowds Freeze During Field Recomputation
**As a** player (P-23), **I want** crowds to keep moving during flow field recomputation,
**so that** cache misses do not cause visible freezes.

## US-7.7.3.7 Implement Tiled Flow Field Streaming
**As an** engine developer (P-26), **I want to** tile flow fields to match the world streaming
grid and load/unload them with world chunks, **so that** flow field data streams with the
world.

## US-7.7.3.8 Cache Fields Keyed by Goal and Cost Version
**As an** engine developer (P-26), **I want to** cache flow fields keyed by (goal cell, cost
layer version), **so that** identical goals reuse cached fields.

## US-7.7.3.9 Limit Cache Size on Mobile
**As an** engine developer (P-26), **I want to** limit the mobile cache to 8 entries (vs. 32
on desktop), **so that** memory usage is bounded with more aggressive eviction.

## US-7.7.3.10 Verify Cache Hit Returns Correct Field
**As an** engine tester (P-27), **I want to** verify that a cache hit returns a field
identical to a freshly computed one, **so that** caching does not alter navigation.

## US-7.7.3.11 Test Cache Invalidation on Cost Grid Change
**As an** engine tester (P-27), **I want to** verify that cached fields are invalidated when
the cost grid changes, **so that** stale fields are never used.

## US-7.7.3.12 Benchmark Cache Hit vs. Miss Performance
**As an** engine tester (P-27), **I want to** benchmark the performance difference between
cache hits and misses, **so that** the caching benefit is quantified.

---

## F-7.7.4 — Mass Entity Simulation

## US-7.7.4.1 Spawn Lightweight Crowd NPCs
**As a** designer (P-5), **I want to** spawn thousands of lightweight crowd NPCs with minimal
per-agent state (position, velocity, archetype), **so that** cities feel populated.

## US-7.7.4.2 Configure Crowd Count Per Platform
**As a** designer (P-5), **I want to** configure the crowd count budget per platform (mobile
100, Switch 500, console 2000, desktop 10000+), **so that** each platform runs at target
frame rate.

## US-7.7.4.3 Set Despawn Radius for Ambient Crowds
**As a** designer (P-5), **I want to** set a despawn radius so crowd NPCs far from players
are removed, **so that** active agent count stays within budget.

## US-7.7.4.4 Walk Through a Bustling City
**As a** player (P-23), **I want** to walk through a city populated with thousands of ambient
inhabitants going about their daily lives, **so that** the world feels alive.

## US-7.7.4.5 See Crowd NPCs Disappear Naturally at Distance
**As a** player (P-23), **I want** distant crowd NPCs to fade out naturally rather than
popping in and out of existence, **so that** despawn is not jarring.

## US-7.7.4.6 Experience Smooth Frame Rate in Crowded Areas
**As a** player (P-23), **I want** the frame rate to remain stable even in the most crowded
city areas, **so that** crowd count does not degrade performance.

## US-7.7.4.7 Implement Mass Entity Processing Pipeline
**As an** engine developer (P-26), **I want to** process crowd NPCs as lightweight entities
driven by flow fields and flocking rather than full behavior trees, **so that** per-agent
cost is bounded.

## US-7.7.4.8 Use Minimal Per-Agent State
**As an** engine developer (P-26), **I want** each crowd entity to store only position,
velocity, archetype ID, and flow field sample, **so that** memory footprint is minimized.

## US-7.7.4.9 Scale Crowd Budget by Platform Tier
**As an** engine developer (P-26), **I want** the crowd budget to scale by platform tier with
configurable caps, **so that** each platform runs within its CPU and memory budget.

## US-7.7.4.10 Verify Crowd Count Stays Within Budget
**As an** engine tester (P-27), **I want to** verify that the active crowd entity count never
exceeds the configured platform budget, **so that** the cap is enforced.

## US-7.7.4.11 Test Frame Time Stability at Maximum Crowd Count
**As an** engine tester (P-27), **I want to** test that frame time remains stable at the
maximum crowd count for each platform tier, **so that** performance is guaranteed.

## US-7.7.4.12 Benchmark Per-Agent CPU Cost for Mass Entities
**As an** engine tester (P-27), **I want to** benchmark the per-agent CPU cost for mass
entity processing, **so that** the lightweight pipeline is validated.

---

## F-7.7.5 — AI Level of Detail (Processing Budget)

## US-7.7.5.1 Configure LOD Tier Distances
**As a** designer (P-5), **I want to** configure LOD tier distances per platform (high-LOD 20
m mobile, 60 m desktop), **so that** AI processing scales with player proximity.

## US-7.7.5.2 Set Agent Count Limits Per LOD Tier
**As a** designer (P-5), **I want to** set the maximum number of agents per LOD tier (8
high-LOD mobile, 64 desktop), **so that** the most expensive agents are capped.

## US-7.7.5.3 Assign LOD Tiers Based on Gameplay Relevance
**As a** designer (P-5), **I want** quest-critical NPCs to remain at high LOD regardless of
distance, **so that** important AI never degrades in quality.

## US-7.7.5.4 See Nearby NPCs Behave Intelligently
**As a** player (P-23), **I want** NPCs near me to exhibit full intelligent behavior (complex
decisions, reactive perception), **so that** AI I interact with feels smart.

## US-7.7.5.5 Not Notice Distant NPCs Are Simplified
**As a** player (P-23), **I want** distant NPCs to use simplified movement without it being
noticeable, **so that** LOD transitions are invisible.

## US-7.7.5.6 See AI Quality Increase as I Approach
**As a** player (P-23), **I want** NPCs to exhibit increasingly complex behavior as I
approach, **so that** AI quality feels consistent with my observation distance.

## US-7.7.5.7 Implement LOD Tier Assignment System
**As an** engine developer (P-26), **I want to** assign each AI agent a LOD tier based on
distance to the nearest player and gameplay relevance, **so that** processing scales.

## US-7.7.5.8 Implement Global Budget Scheduler
**As an** engine developer (P-26), **I want to** implement a global budget scheduler that
distributes CPU time across LOD tiers, **so that** frame rate is stable.

## US-7.7.5.9 Support Low-LOD Flow-Field-Only Movement
**As an** engine developer (P-26), **I want** low-LOD agents to use flow-field-only movement,
skipping BT and perception, **so that** distant agents cost minimal CPU.

## US-7.7.5.10 Verify LOD Tier Transitions Are Correct
**As an** engine tester (P-27), **I want to** verify that agents transition between LOD tiers
at the correct distances, **so that** tier assignment is accurate.

## US-7.7.5.11 Test Budget Scheduler Maintains Frame Rate
**As an** engine tester (P-27), **I want to** verify that the budget scheduler keeps AI
processing within its allocated time slice even at maximum agent count, **so that** frame
rate is stable.

## US-7.7.5.12 Validate LOD Transitions Are Behaviorally Invisible
**As an** engine tester (P-27), **I want to** verify that LOD tier transitions do not produce
visible behavior discontinuities (path snapping, instant state changes), **so that**
transitions are seamless.

---

## F-7.7.6 — Density Management

## US-7.7.6.1 Configure Per-Cell Density Caps
**As a** designer (P-5), **I want to** configure per-cell crowd density caps, **so that**
unrealistic pileups at chokepoints and events are prevented.

## US-7.7.6.2 Set Overflow Redirect Behavior
**As a** designer (P-5), **I want to** configure whether overflow agents are redirected to
alternate routes or despawned, **so that** density management matches the scene's needs.

## US-7.7.6.3 Test Density Management at Chokepoints
**As a** designer (P-5), **I want to** test density management at a narrow gate with hundreds
of crowd NPCs and verify no pileups, **so that** caps work in real scenarios.

## US-7.7.6.4 See Crowds Spread Out at Chokepoints
**As a** player (P-23), **I want** crowds at chokepoints to spread into orderly flow rather
than piling up, **so that** movement looks physically plausible.

## US-7.7.6.5 Watch Crowds Take Alternate Routes When Full
**As a** player (P-23), **I want** crowd NPCs to take alternate routes when the primary path
is full, **so that** crowds self-distribute naturally.

## US-7.7.6.6 Not See Crowds Clip Through Each Other at Events
**As a** player (P-23), **I want** crowd NPCs at a market or event to maintain personal space
without clipping, **so that** gatherings look realistic.

## US-7.7.6.7 Implement Per-Cell Density Monitoring
**As an** engine developer (P-26), **I want to** monitor crowd density per spatial cell and
enforce caps, **so that** overflow triggers redirect or despawn.

## US-7.7.6.8 Redirect Overflow Agents to Alternate Routes
**As an** engine developer (P-26), **I want to** redirect overflow agents to alternate routes
when density exceeds the threshold, **so that** crowds self-balance.

## US-7.7.6.9 Enforce Lower Density Caps on Mobile
**As an** engine developer (P-26), **I want** mobile to enforce lower per-cell density caps
(4 vs. 16 on desktop) and despawn overflow more aggressively, **so that** crowd count stays
within budget.

## US-7.7.6.10 Verify Density Never Exceeds Configured Cap
**As an** engine tester (P-27), **I want to** verify that per-cell density never exceeds the
configured cap, **so that** density management enforcement is correct.

## US-7.7.6.11 Test Redirect Distributes Agents to Alternate Paths
**As an** engine tester (P-27), **I want to** verify that redirected agents successfully reach
their goal via alternate paths, **so that** redirection does not strand agents.

## US-7.7.6.12 Benchmark Density Management Overhead
**As an** engine tester (P-27), **I want to** benchmark the CPU overhead of density monitoring
and management, **so that** it fits within the crowd simulation budget.

---

## F-7.7.7 — Knowledge Sharing and Event Propagation

## US-7.7.7.1 Configure Communication Radius Per Faction
**As a** designer (P-5), **I want to** configure the communication radius for knowledge
sharing per faction, **so that** guard barracks and patrols share alerts within appropriate
range.

## US-7.7.7.2 Define Communication Event Types
**As a** designer (P-5), **I want to** define communication types (immediate alert,
investigation request, all-clear, report), **so that** shared knowledge triggers appropriate
responses.

## US-7.7.7.3 Test Knowledge Propagation Chain
**As a** designer (P-5), **I want to** trigger a detection event and verify it propagates to
all allies within range, **so that** knowledge sharing works end-to-end.

## US-7.7.7.4 See Guard Alert Spread to Barracks
**As a** player (P-23), **I want** the guard who spots me to alert all guards in the barracks
within communication range, **so that** stealth challenges feel realistic.

## US-7.7.7.5 Watch Only Nearest Guard Investigate a Sound
**As a** player (P-23), **I want** only the nearest idle guard to investigate a suspicious
sound while others hold position, **so that** AI teams respond without overcommitting.

## US-7.7.7.6 See Guards Stand Down After All-Clear
**As a** player (P-23), **I want** guards to stand down after an all-clear broadcast when
investigation finds nothing, **so that** alert states resolve naturally.

## US-7.7.7.7 Implement Broadcast Knowledge Event System
**As an** engine developer (P-26), **I want to** implement a knowledge event system that
broadcasts to allied agents within communication radius, **so that** perception data is
shared.

## US-7.7.7.8 Use Faction Affinity for Broadcast Filtering
**As an** engine developer (P-26), **I want to** use the faction affinity system to determine
broadcast recipients, **so that** only same-faction and allied agents receive knowledge.

## US-7.7.7.9 Limit Broadcast Radius and Recipients on Mobile
**As an** engine developer (P-26), **I want** mobile to limit broadcast radius to 15 m (vs.
50 m desktop) and cap recipients to 8, **so that** broadcast cost is bounded.

## US-7.7.7.10 Verify Broadcasts Reach All Allies in Range
**As an** engine tester (P-27), **I want to** verify that knowledge broadcasts reach all
allied agents within communication radius, **so that** no allies are missed.

## US-7.7.7.11 Test Agents Outside Radius Remain Unaware
**As an** engine tester (P-27), **I want to** verify that agents outside the communication
radius do not receive broadcasts, **so that** range limiting works.

## US-7.7.7.12 Validate Shared Knowledge Decays Correctly
**As an** engine tester (P-27), **I want to** verify that shared knowledge decays at the same
rate as direct perception, **so that** second-hand information fades appropriately.

---

## F-7.7.8 — Shared Awareness and Synchronized Group Reactions

## US-7.7.8.1 Configure Group Reaction Type Per Archetype
**As a** designer (P-5), **I want to** configure group reaction types per creature archetype
(birds scatter, herds stampede, civilians flee, guards form line), **so that** each group
reacts characteristically.

## US-7.7.8.2 Set Alarm Propagation Delay
**As a** designer (P-5), **I want to** set the alarm propagation delay per group type,
**so that** the reaction wave looks natural for the creature.

## US-7.7.8.3 Configure Calm-Down and Regroup Timers
**As a** designer (P-5), **I want to** configure how long after the threat clears a group
takes to reassemble, **so that** recovery pace matches the creature's behavior.

## US-7.7.8.4 See Bird Flock Scatter in a Wave
**As a** player (P-23), **I want** a flock of birds to scatter in a wave from nearest to
farthest as I approach, then regroup at a safe distance, **so that** wildlife feels alive.

## US-7.7.8.5 Watch Herd Stampede Away from Predator
**As a** player (P-23), **I want** a herd to stampede as a group when a predator appears,
maintaining cohesion during flight, **so that** herd behavior looks coordinated.

## US-7.7.8.6 See Civilians Panic and Flee Toward Exits
**As a** player (P-23), **I want** civilians to panic and flee toward building exits when
combat breaks out, **so that** bystander reactions feel realistic.

## US-7.7.8.7 Implement Alarm Propagation with Spatial Delay
**As an** engine developer (P-26), **I want to** implement alarm propagation with spatial
delay so members farther from the trigger react later, **so that** a natural wave effect
emerges.

## US-7.7.8.8 Maintain Group Cohesion During Flight
**As an** engine developer (P-26), **I want** individuals that fall behind during group flight
to accelerate and rejoin, **so that** groups remain cohesive during panic.

## US-7.7.8.9 Use Instant-Uniform Alarm on Mobile
**As an** engine developer (P-26), **I want** mobile to use instant-uniform alarm propagation
(skipping wave delay), **so that** group reaction is simpler on constrained devices.

## US-7.7.8.10 Verify Alarm Wave Propagation Order
**As an** engine tester (P-27), **I want to** verify that alarm propagates from nearest to
farthest member with the configured delay, **so that** wave order is correct.

## US-7.7.8.11 Test Group Reassembly After Threat Clears
**As an** engine tester (P-27), **I want to** verify that groups reassemble at a rally point
after the threat clears within the configured calm-down period, **so that** recovery works.

## US-7.7.8.12 Validate All Creature Reaction Types
**As an** engine tester (P-27), **I want to** verify that all configured group reaction types
(scatter, stampede, flee, form line) produce the expected behavior patterns, **so that** each
archetype reacts correctly.

---

## F-7.7.9 — Faction-Based Behavioral Relationships

## US-7.7.9.1 Define Faction Relationship Matrix
**As a** designer (P-5), **I want to** define a faction affinity matrix with relationship
types (aggressive, hostile, wary, neutral, friendly, allied), **so that** NPC behavior is
driven by faction relations.

## US-7.7.9.2 Modify Relationships at Runtime via Gameplay
**As a** designer (P-5), **I want to** modify faction relationships at runtime via gameplay
events (quest completion, betrayal), **so that** the world reacts to player choices.

## US-7.7.9.3 Set Individual NPC Relationship Overrides
**As a** designer (P-5), **I want to** set per-NPC relationship overrides that deviate from
faction defaults (a friendly orc in a hostile faction), **so that** unique characters are
supported.

## US-7.7.9.4 See Hostile NPCs Attack Me on Sight
**As a** player (P-23), **I want** NPCs from a hostile faction to attack me on sight,
**so that** faction hostility is immediately visible.

## US-7.7.9.5 Watch Wary NPCs Warn Before Attacking
**As a** player (P-23), **I want** wary NPCs to warn me before turning hostile if I continue
trespassing, **so that** faction interactions feel nuanced with escalation.

## US-7.7.9.6 See Reputation Change Turn Enemies into Allies
**As a** player (P-23), **I want** NPCs who previously attacked me to become friendly after
my reputation improves, **so that** my actions visibly affect world dynamics.

## US-7.7.9.7 Implement Faction Affinity Matrix
**As an** engine developer (P-26), **I want to** implement a faction affinity matrix storing
relationship types that determine engagement rules, communication, and trade availability,
**so that** behavior is relationship-driven.

## US-7.7.9.8 Support Runtime Matrix Modification
**As an** engine developer (P-26), **I want to** support runtime modification of the affinity
matrix by gameplay systems, **so that** relationships evolve dynamically.

## US-7.7.9.9 Support Per-NPC Relationship Overrides
**As an** engine developer (P-26), **I want to** support per-NPC relationship overrides that
take precedence over faction defaults, **so that** unique NPCs can deviate from their group.

## US-7.7.9.10 Verify All Relationship Types Produce Correct Behavior
**As an** engine tester (P-27), **I want to** verify that each relationship type (aggressive,
hostile, wary, neutral, friendly, allied) produces the expected behavioral response, **so
that** all relationship modes work.

## US-7.7.9.11 Test Runtime Relationship Change Propagation
**As an** engine tester (P-27), **I want to** verify that changing a faction relationship at
runtime immediately changes NPC behavior for that faction, **so that** updates propagate.

## US-7.7.9.12 Validate Individual Override Precedence
**As an** engine tester (P-27), **I want to** verify that a per-NPC override takes precedence
over the faction default, **so that** unique NPCs behave correctly.

---

## F-7.7.10 — Threat Table and Aggro Targeting

## US-7.7.10.1 Configure Threat Sources and Weights
**As a** designer (P-5), **I want to** configure threat sources (damage, healing, taunts,
debuffs, proximity) and their weights, **so that** the threat table reflects my game's
combat balance.

## US-7.7.10.2 Set Aggro Transfer Threshold
**As a** designer (P-5), **I want to** set the aggro transfer threshold (110% melee, 130%
ranged), **so that** tank gameplay has tunable difficulty.

## US-7.7.10.3 Configure AI Archetype Targeting Behavior
**As a** designer (P-5), **I want to** configure targeting behavior per AI archetype
(berserker, tactical, protector, coward), **so that** different enemies require different
strategies.

## US-7.7.10.4 Hold Aggro as Tank with Taunt Abilities
**As a** player (P-23), **I want** enemies to attack me while my threat stays highest, with
taunts generating burst threat, **so that** the tank role functions correctly.

## US-7.7.10.5 See Tactical AI Target Weakest Party Member
**As a** player (P-23), **I want** tactical enemies to target the lowest-HP party member,
**so that** I must protect vulnerable allies.

## US-7.7.10.6 Watch Threat Decay When Out of Range
**As a** player (P-23), **I want** threat to decay when I move out of combat range, **so
that** disengaging from combat is possible over time.

## US-7.7.10.7 Implement Per-Enemy Threat Table
**As an** engine developer (P-26), **I want to** implement a per-enemy threat table tracking
hate from each attacker, selecting the highest-threat target, **so that** aggro targeting is
data-driven.

## US-7.7.10.8 Support Threat Modifiers on Abilities
**As an** engine developer (P-26), **I want to** support threat modifiers on abilities (low
threat heals, high threat taunts), **so that** class roles can manage threat.

## US-7.7.10.9 Expose Threat Table to Behavior Tree and HUD
**As an** engine developer (P-26), **I want to** expose the threat table to behavior trees for
conditional target switching and to the HUD for tank threat indicators, **so that** threat
data is accessible.

## US-7.7.10.10 Verify Highest-Threat Target Is Attacked
**As an** engine tester (P-27), **I want to** verify that the AI always attacks the
highest-threat target, **so that** basic threat table targeting works.

## US-7.7.10.11 Test Aggro Transfer Threshold
**As an** engine tester (P-27), **I want to** verify that aggro does not transfer until the
new threat exceeds the current target by the configured threshold, **so that** aggro is
stable.

## US-7.7.10.12 Validate All Targeting Archetypes
**As an** engine tester (P-27), **I want to** verify that each AI archetype (berserker,
tactical, protector, coward) selects the correct target based on its configured behavior,
**so that** archetype targeting is correct.

---

## F-7.7.11 — Animal Tracking and Hunting Behaviors

## US-7.7.11.1 Configure Predator-Prey Pairs and Success Rates
**As a** designer (P-5), **I want to** configure predator-prey pairs with hunting success
rates, **so that** ecological balance is maintained across wildlife species.

## US-7.7.11.2 Set Predator Behavior Phases
**As a** designer (P-5), **I want to** configure predator phases (patrol, detect, stalk,
ambush, chase, kill, feed), **so that** hunting behavior is rich and sequenced.

## US-7.7.11.3 Configure Prey Defensive Behaviors
**As a** designer (P-5), **I want to** configure prey defensive behaviors (flee, form
protective circle, stand and fight), **so that** different prey species respond differently.

## US-7.7.11.4 Watch Wolf Pack Coordinate a Hunt
**As a** player (P-23), **I want** a wolf pack to stalk downwind, with drivers flushing prey
toward ambushers, **so that** pack hunting feels realistic and coordinated.

## US-7.7.11.5 See Prey Herd Form Protective Circle
**As a** player (P-23), **I want** a deer herd to form protective circles with mothers
shielding young when a predator approaches, **so that** prey behavior is as rich as predator
behavior.

## US-7.7.11.6 Watch Predator Give Up After Prey Outruns It
**As a** player (P-23), **I want** a predator to give up the chase when the prey outruns it,
managing stamina, **so that** hunts have realistic outcomes.

## US-7.7.11.7 Implement Predator Behavior State Machine
**As an** engine developer (P-26), **I want to** implement the predator behavior state machine
(patrol, detect, stalk, ambush, chase, kill, feed), **so that** hunting phases are
well-defined.

## US-7.7.11.8 Implement Pack Coordination with Roles
**As an** engine developer (P-26), **I want to** implement coordinated pack hunting with
driver and ambusher roles, **so that** pack members cooperate during hunts.

## US-7.7.11.9 Simplify Hunting on Mobile
**As an** engine developer (P-26), **I want** mobile to use simplified hunting (sight-only
tracking, no pack coordination, no scent/footprint following), **so that** wildlife
simulation fits the mobile budget.

## US-7.7.11.10 Verify Hunting Success Rate Matches Configuration
**As an** engine tester (P-27), **I want to** run 100+ hunting simulations and verify the
success rate matches the configured value within tolerance, **so that** ecological balance is
correct.

## US-7.7.11.11 Test Predator Gives Up After Stamina Depletion
**As an** engine tester (P-27), **I want to** verify that a predator ends the chase when its
stamina depletes, **so that** stamina management works.

## US-7.7.11.12 Validate Pack Role Assignment
**As an** engine tester (P-27), **I want to** verify that pack members are assigned driver
and ambusher roles and execute their assigned behaviors, **so that** coordination is correct.
