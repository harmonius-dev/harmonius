# User Stories — 7.7 Crowd Simulation

## F-7.7.1 — Flocking Behaviors (Separation, Alignment, Cohesion)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.7.1.1 | designer (P-5) | I want to configure separation, alignment, and cohesion weights per crowd archetype | tight military columns, loose herds, and scattered civilians each have distinct group movement | F-7.7.1 | R-7.7.1 |
| US-7.7.1.2 | designer (P-5) | I want to set the neighbor query radius per archetype | flocking range matches the intended group tightness | F-7.7.1 | R-7.7.1 |
| US-7.7.1.3 | designer (P-5) | I want to see separation, alignment, and cohesion force vectors as debug arrows on each agent | I can tune weights visually | F-7.7.1 | R-7.7.1 |
| US-7.7.1.4 | player (P-23) | I want animal herds to move as a cohesive group rather than as disconnected individuals | wildlife feels alive and natural | F-7.7.1 | R-7.7.1 |
| US-7.7.1.5 | player (P-23) | I want flocks of birds to move in recognizable patterns with alignment and cohesion | aerial wildlife looks realistic | F-7.7.1 | R-7.7.1 |
| US-7.7.1.6 | player (P-23) | I want crowd NPCs to maintain personal space via separation forces | dense crowds do not clip through each other | F-7.7.1 | R-7.7.1 |
| US-7.7.1.7 | engine developer (P-26) | I want to implement Reynolds flocking with weighted separation, alignment, and cohesion forces | group movement emerges from simple rules | F-7.7.1 | R-7.7.1 |
| US-7.7.1.8 | engine developer (P-26) | I want to query neighbors via the shared spatial index for flocking | neighbor lookups are efficient | F-7.7.1 | R-7.7.1 |
| US-7.7.1.9 | engine developer (P-26) | I want to reduce the neighbor query radius on mobile (8 m vs. 16 m desktop) | per-agent flocking cost fits the mobile budget | F-7.7.1 | R-7.7.1 |
| US-7.7.1.10 | engine tester (P-27) | I want to verify that separation forces prevent agents from overlapping within a flock | spatial distribution is correct | F-7.7.1 | R-7.7.1 |
| US-7.7.1.11 | engine tester (P-27) | I want to verify that cohesion forces keep flock members within a configurable radius of the group centroid | groups do not fragment | F-7.7.1 | R-7.7.1 |
| US-7.7.1.12 | engine tester (P-27) | I want to benchmark flocking computation for the maximum flock size per platform | flocking cost is within budget. --- | F-7.7.1 | R-7.7.1 |

## F-7.7.2 — Flow Field Navigation

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.7.2.1 | designer (P-5) | I want to generate flow fields from goal positions in the editor | I can preview mass crowd movement toward points of interest | F-7.7.2 | R-7.7.2 |
| US-7.7.2.2 | designer (P-5) | I want to set flow field cell resolution (1 m desktop, 2 m mobile) | crowd navigation quality scales with platform capability | F-7.7.2 | R-7.7.2 |
| US-7.7.2.3 | designer (P-5) | I want to assign goal positions to crowd groups and see them flow toward the goal | I can orchestrate mass crowd events | F-7.7.2 | R-7.7.2 |
| US-7.7.2.4 | player (P-23) | I want thousands of crowd NPCs to move smoothly toward a gate, market, or event | mass movement looks realistic and purposeful | F-7.7.2 | R-7.7.2 |
| US-7.7.2.5 | player (P-23) | I want crowd NPCs to flow around buildings and obstacles along the cheapest path | mass navigation looks natural | F-7.7.2 | R-7.7.2 |
| US-7.7.2.6 | player (P-23) | I want crowd flow direction to change smoothly when goals update | redirected crowds do not snap or teleport | F-7.7.2 | R-7.7.2 |
| US-7.7.2.7 | engine developer (P-26) | I want to implement 2D grid-based flow fields using Dijkstra propagation from goal positions | each cell stores the cheapest path direction | F-7.7.2 | R-7.7.2 |
| US-7.7.2.8 | engine developer (P-26) | I want crowd agents to sample the flow field directly instead of running individual A* queries | per-agent navigation cost is constant | F-7.7.2 | R-7.7.2 |
| US-7.7.2.9 | engine developer (P-26) | I want to limit concurrent active flow fields to 4 on mobile | flow field memory and computation fit within mobile constraints | F-7.7.2 | R-7.7.2 |
| US-7.7.2.10 | engine tester (P-27) | I want to verify that following flow field directions from any cell reaches the goal position | path correctness is confirmed | F-7.7.2 | R-7.7.2 |
| US-7.7.2.11 | engine tester (P-27) | I want to verify that flow fields recompute correctly when dynamic obstacles modify the cost grid | redirected flows are accurate | F-7.7.2 | R-7.7.2 |
| US-7.7.2.12 | engine tester (P-27) | I want to benchmark flow field generation time for the maximum tile size | recomputation fits within the per-tick budget. --- | F-7.7.2 | R-7.7.2 |

## F-7.7.3 — Flow Field Streaming & Caching

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.7.3.1 | designer (P-5) | I want to configure the flow field cache size per platform | frequently used fields are retained without exceeding memory | F-7.7.3 | R-7.7.3 |
| US-7.7.3.2 | designer (P-5) | I want to see which flow fields are currently cached in a debug panel | I can verify cache utilization | F-7.7.3 | R-7.7.3 |
| US-7.7.3.3 | designer (P-5) | I want cached flow fields to be invalidated when dynamic obstacles modify the cost grid | stale directions are never used | F-7.7.3 | R-7.7.3 |
| US-7.7.3.4 | player (P-23) | I want crowd NPCs to navigate seamlessly across world streaming boundaries | crowd flow is continuous | F-7.7.3 | R-7.7.3 |
| US-7.7.3.5 | player (P-23) | I want crowds to redirect quickly when a new obstacle appears (gate closes, barricade placed) | flow field invalidation is responsive | F-7.7.3 | R-7.7.3 |
| US-7.7.3.6 | player (P-23) | I want crowds to keep moving during flow field recomputation | cache misses do not cause visible freezes | F-7.7.3 | R-7.7.3 |
| US-7.7.3.7 | engine developer (P-26) | I want to tile flow fields to match the world streaming grid and load/unload them with world chunks | flow field data streams with the world | F-7.7.3 | R-7.7.3 |
| US-7.7.3.8 | engine developer (P-26) | I want to cache flow fields keyed by (goal cell, cost layer version) | identical goals reuse cached fields | F-7.7.3 | R-7.7.3 |
| US-7.7.3.9 | engine developer (P-26) | I want to limit the mobile cache to 8 entries (vs. 32 on desktop) | memory usage is bounded with more aggressive eviction | F-7.7.3 | R-7.7.3 |
| US-7.7.3.10 | engine tester (P-27) | I want to verify that a cache hit returns a field identical to a freshly computed one | caching does not alter navigation | F-7.7.3 | R-7.7.3 |
| US-7.7.3.11 | engine tester (P-27) | I want to verify that cached fields are invalidated when the cost grid changes | stale fields are never used | F-7.7.3 | R-7.7.3 |
| US-7.7.3.12 | engine tester (P-27) | I want to benchmark the performance difference between cache hits and misses | the caching benefit is quantified. --- | F-7.7.3 | R-7.7.3 |

## F-7.7.4 — Mass Entity Simulation

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.7.4.1 | designer (P-5) | I want to spawn thousands of lightweight crowd NPCs with minimal per-agent state (position, velocity, archetype) | cities feel populated | F-7.7.4 | R-7.7.4 |
| US-7.7.4.2 | designer (P-5) | I want to configure the crowd count budget per platform (mobile 100, Switch 500, console 2000, desktop 10000+) | each platform runs at target frame rate | F-7.7.4 | R-7.7.4 |
| US-7.7.4.3 | designer (P-5) | I want to set a despawn radius so crowd NPCs far from players are removed | active agent count stays within budget | F-7.7.4 | R-7.7.4 |
| US-7.7.4.4 | player (P-23) | I want walk through a city populated with thousands of ambient inhabitants going about their daily lives | the world feels alive | F-7.7.4 | R-7.7.4 |
| US-7.7.4.5 | player (P-23) | I want distant crowd NPCs to fade out naturally rather than popping in and out of existence | despawn is not jarring | F-7.7.4 | R-7.7.4 |
| US-7.7.4.6 | player (P-23) | I want the frame rate to remain stable even in the most crowded city areas | crowd count does not degrade performance | F-7.7.4 | R-7.7.4 |
| US-7.7.4.7 | engine developer (P-26) | I want to process crowd NPCs as lightweight entities driven by flow fields and flocking rather than full behavior trees | per-agent cost is bounded | F-7.7.4 | R-7.7.4 |
| US-7.7.4.8 | engine developer (P-26) | I want each crowd entity to store only position, velocity, archetype ID, and flow field sample | memory footprint is minimized | F-7.7.4 | R-7.7.4 |
| US-7.7.4.9 | engine developer (P-26) | I want the crowd budget to scale by platform tier with configurable caps | each platform runs within its CPU and memory budget | F-7.7.4 | R-7.7.4 |
| US-7.7.4.10 | engine tester (P-27) | I want to verify that the active crowd entity count never exceeds the configured platform budget | the cap is enforced | F-7.7.4 | R-7.7.4 |
| US-7.7.4.11 | engine tester (P-27) | I want to test that frame time remains stable at the maximum crowd count for each platform tier | performance is guaranteed | F-7.7.4 | R-7.7.4 |
| US-7.7.4.12 | engine tester (P-27) | I want to benchmark the per-agent CPU cost for mass entity processing | the lightweight pipeline is validated. --- | F-7.7.4 | R-7.7.4 |

## F-7.7.5 — AI Level of Detail (Processing Budget)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.7.5.1 | designer (P-5) | I want to configure LOD tier distances per platform (high-LOD 20 m mobile, 60 m desktop) | AI processing scales with player proximity | F-7.7.5 | R-7.7.5 |
| US-7.7.5.2 | designer (P-5) | I want to set the maximum number of agents per LOD tier (8 high-LOD mobile, 64 desktop) | the most expensive agents are capped | F-7.7.5 | R-7.7.5 |
| US-7.7.5.3 | designer (P-5) | I want quest-critical NPCs to remain at high LOD regardless of distance | important AI never degrades in quality | F-7.7.5 | R-7.7.5 |
| US-7.7.5.4 | player (P-23) | I want NPCs near me to exhibit full intelligent behavior (complex decisions, reactive perception) | AI I interact with feels smart | F-7.7.5 | R-7.7.5 |
| US-7.7.5.5 | player (P-23) | I want distant NPCs to use simplified movement without it being noticeable | LOD transitions are invisible | F-7.7.5 | R-7.7.5 |
| US-7.7.5.6 | player (P-23) | I want NPCs to exhibit increasingly complex behavior as I approach | AI quality feels consistent with my observation distance | F-7.7.5 | R-7.7.5 |
| US-7.7.5.7 | engine developer (P-26) | I want to assign each AI agent a LOD tier based on distance to the nearest player and gameplay relevance | processing scales | F-7.7.5 | R-7.7.5 |
| US-7.7.5.8 | engine developer (P-26) | I want to implement a global budget scheduler that distributes CPU time across LOD tiers | frame rate is stable | F-7.7.5 | R-7.7.5 |
| US-7.7.5.9 | engine developer (P-26) | I want low-LOD agents to use flow-field-only movement, skipping BT and perception | distant agents cost minimal CPU | F-7.7.5 | R-7.7.5 |
| US-7.7.5.10 | engine tester (P-27) | I want to verify that agents transition between LOD tiers at the correct distances | tier assignment is accurate | F-7.7.5 | R-7.7.5 |
| US-7.7.5.11 | engine tester (P-27) | I want to verify that the budget scheduler keeps AI processing within its allocated time slice even at maximum agent count | frame rate is stable | F-7.7.5 | R-7.7.5 |
| US-7.7.5.12 | engine tester (P-27) | I want to verify that LOD tier transitions do not produce visible behavior discontinuities (path snapping, instant state changes) | transitions are seamless. --- | F-7.7.5 | R-7.7.5 |

## F-7.7.6 — Density Management

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.7.6.1 | designer (P-5) | I want to configure per-cell crowd density caps | unrealistic pileups at chokepoints and events are prevented | F-7.7.6 | R-7.7.6 |
| US-7.7.6.2 | designer (P-5) | I want to configure whether overflow agents are redirected to alternate routes or despawned | density management matches the scene's needs | F-7.7.6 | R-7.7.6 |
| US-7.7.6.3 | designer (P-5) | I want to test density management at a narrow gate with hundreds of crowd NPCs and verify no pileups | caps work in real scenarios | F-7.7.6 | R-7.7.6 |
| US-7.7.6.4 | player (P-23) | I want crowds at chokepoints to spread into orderly flow rather than piling up | movement looks physically plausible | F-7.7.6 | R-7.7.6 |
| US-7.7.6.5 | player (P-23) | I want crowd NPCs to take alternate routes when the primary path is full | crowds self-distribute naturally | F-7.7.6 | R-7.7.6 |
| US-7.7.6.6 | player (P-23) | I want crowd NPCs at a market or event to maintain personal space without clipping | gatherings look realistic | F-7.7.6 | R-7.7.6 |
| US-7.7.6.7 | engine developer (P-26) | I want to monitor crowd density per spatial cell and enforce caps | overflow triggers redirect or despawn | F-7.7.6 | R-7.7.6 |
| US-7.7.6.8 | engine developer (P-26) | I want to redirect overflow agents to alternate routes when density exceeds the threshold | crowds self-balance | F-7.7.6 | R-7.7.6 |
| US-7.7.6.9 | engine developer (P-26) | I want mobile to enforce lower per-cell density caps (4 vs. 16 on desktop) and despawn overflow more aggressively | crowd count stays within budget | F-7.7.6 | R-7.7.6 |
| US-7.7.6.10 | engine tester (P-27) | I want to verify that per-cell density never exceeds the configured cap | density management enforcement is correct | F-7.7.6 | R-7.7.6 |
| US-7.7.6.11 | engine tester (P-27) | I want to verify that redirected agents successfully reach their goal via alternate paths | redirection does not strand agents | F-7.7.6 | R-7.7.6 |
| US-7.7.6.12 | engine tester (P-27) | I want to benchmark the CPU overhead of density monitoring and management | it fits within the crowd simulation budget. --- | F-7.7.6 | R-7.7.6 |

## F-7.7.7 — Knowledge Sharing and Event Propagation

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.7.7.1 | designer (P-5) | I want to configure the communication radius for knowledge sharing per faction | guard barracks and patrols share alerts within appropriate range | F-7.7.7 | R-7.7.7 |
| US-7.7.7.2 | designer (P-5) | I want to define communication types (immediate alert, investigation request, all-clear, report) | shared knowledge triggers appropriate responses | F-7.7.7 | R-7.7.7 |
| US-7.7.7.3 | designer (P-5) | I want to trigger a detection event and verify it propagates to all allies within range | knowledge sharing works end-to-end | F-7.7.7 | R-7.7.7 |
| US-7.7.7.4 | player (P-23) | I want the guard who spots me to alert all guards in the barracks within communication range | stealth challenges feel realistic | F-7.7.7 | R-7.7.7 |
| US-7.7.7.5 | player (P-23) | I want only the nearest idle guard to investigate a suspicious sound while others hold position | AI teams respond without overcommitting | F-7.7.7 | R-7.7.7 |
| US-7.7.7.6 | player (P-23) | I want guards to stand down after an all-clear broadcast when investigation finds nothing | alert states resolve naturally | F-7.7.7 | R-7.7.7 |
| US-7.7.7.7 | engine developer (P-26) | I want to implement a knowledge event system that broadcasts to allied agents within communication radius | perception data is shared | F-7.7.7 | R-7.7.7 |
| US-7.7.7.8 | engine developer (P-26) | I want to use the faction affinity system to determine broadcast recipients | only same-faction and allied agents receive knowledge | F-7.7.7 | R-7.7.7 |
| US-7.7.7.9 | engine developer (P-26) | I want mobile to limit broadcast radius to 15 m (vs. 50 m desktop) and cap recipients to 8 | broadcast cost is bounded | F-7.7.7 | R-7.7.7 |
| US-7.7.7.10 | engine tester (P-27) | I want to verify that knowledge broadcasts reach all allied agents within communication radius | no allies are missed | F-7.7.7 | R-7.7.7 |
| US-7.7.7.11 | engine tester (P-27) | I want to verify that agents outside the communication radius do not receive broadcasts | range limiting works | F-7.7.7 | R-7.7.7 |
| US-7.7.7.12 | engine tester (P-27) | I want to verify that shared knowledge decays at the same rate as direct perception | second-hand information fades appropriately. --- | F-7.7.7 | R-7.7.7 |

## F-7.7.8 — Shared Awareness and Synchronized Group Reactions

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.7.8.1 | designer (P-5) | I want to configure group reaction types per creature archetype (birds scatter, herds stampede, civilians flee, guards form line) | each group reacts characteristically | F-7.7.8 | R-7.7.8 |
| US-7.7.8.2 | designer (P-5) | I want to set the alarm propagation delay per group type | the reaction wave looks natural for the creature | F-7.7.8 | R-7.7.8 |
| US-7.7.8.3 | designer (P-5) | I want to configure how long after the threat clears a group takes to reassemble | recovery pace matches the creature's behavior | F-7.7.8 | R-7.7.8 |
| US-7.7.8.4 | player (P-23) | I want a flock of birds to scatter in a wave from nearest to farthest as I approach, then regroup at a safe distance | wildlife feels alive | F-7.7.8 | R-7.7.8 |
| US-7.7.8.5 | player (P-23) | I want a herd to stampede as a group when a predator appears, maintaining cohesion during flight | herd behavior looks coordinated | F-7.7.8 | R-7.7.8 |
| US-7.7.8.6 | player (P-23) | I want civilians to panic and flee toward building exits when combat breaks out | bystander reactions feel realistic | F-7.7.8 | R-7.7.8 |
| US-7.7.8.7 | engine developer (P-26) | I want to implement alarm propagation with spatial delay so members farther from the trigger react later | a natural wave effect emerges | F-7.7.8 | R-7.7.8 |
| US-7.7.8.8 | engine developer (P-26) | I want individuals that fall behind during group flight to accelerate and rejoin | groups remain cohesive during panic | F-7.7.8 | R-7.7.8 |
| US-7.7.8.9 | engine developer (P-26) | I want mobile to use instant-uniform alarm propagation (skipping wave delay) | group reaction is simpler on constrained devices | F-7.7.8 | R-7.7.8 |
| US-7.7.8.10 | engine tester (P-27) | I want to verify that alarm propagates from nearest to farthest member with the configured delay | wave order is correct | F-7.7.8 | R-7.7.8 |
| US-7.7.8.11 | engine tester (P-27) | I want to verify that groups reassemble at a rally point after the threat clears within the configured calm-down period | recovery works | F-7.7.8 | R-7.7.8 |
| US-7.7.8.12 | engine tester (P-27) | I want to verify that all configured group reaction types (scatter, stampede, flee, form line) produce the expected behavior patterns | each archetype reacts correctly. --- | F-7.7.8 | R-7.7.8 |

## F-7.7.9 — Faction-Based Behavioral Relationships

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.7.9.1 | designer (P-5) | I want to define a faction affinity matrix with relationship types (aggressive, hostile, wary, neutral, friendly, allied) | NPC behavior is driven by faction relations | F-7.7.9 | R-7.7.9 |
| US-7.7.9.2 | designer (P-5) | I want to modify faction relationships at runtime via gameplay events (quest completion, betrayal) | the world reacts to player choices | F-7.7.9 | R-7.7.9 |
| US-7.7.9.3 | designer (P-5) | I want to set per-NPC relationship overrides that deviate from faction defaults (a friendly orc in a hostile faction) | unique characters are supported | F-7.7.9 | R-7.7.9 |
| US-7.7.9.4 | player (P-23) | I want NPCs from a hostile faction to attack me on sight | faction hostility is immediately visible | F-7.7.9 | R-7.7.9 |
| US-7.7.9.5 | player (P-23) | I want wary NPCs to warn me before turning hostile if I continue trespassing | faction interactions feel nuanced with escalation | F-7.7.9 | R-7.7.9 |
| US-7.7.9.6 | player (P-23) | I want NPCs who previously attacked me to become friendly after my reputation improves | my actions visibly affect world dynamics | F-7.7.9 | R-7.7.9 |
| US-7.7.9.7 | engine developer (P-26) | I want to implement a faction affinity matrix storing relationship types that determine engagement rules, communication, and trade availability | behavior is relationship-driven | F-7.7.9 | R-7.7.9 |
| US-7.7.9.8 | engine developer (P-26) | I want to support runtime modification of the affinity matrix by gameplay systems | relationships evolve dynamically | F-7.7.9 | R-7.7.9 |
| US-7.7.9.9 | engine developer (P-26) | I want to support per-NPC relationship overrides that take precedence over faction defaults | unique NPCs can deviate from their group | F-7.7.9 | R-7.7.9 |
| US-7.7.9.10 | engine tester (P-27) | I want to verify that each relationship type (aggressive, hostile, wary, neutral, friendly, allied) produces the expected behavioral response | all relationship modes work | F-7.7.9 | R-7.7.9 |
| US-7.7.9.11 | engine tester (P-27) | I want to verify that changing a faction relationship at runtime immediately changes NPC behavior for that faction | updates propagate | F-7.7.9 | R-7.7.9 |
| US-7.7.9.12 | engine tester (P-27) | I want to verify that a per-NPC override takes precedence over the faction default | unique NPCs behave correctly. --- | F-7.7.9 | R-7.7.9 |

## F-7.7.10 — Threat Table and Aggro Targeting

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.7.10.1 | designer (P-5) | I want to configure threat sources (damage, healing, taunts, debuffs, proximity) and their weights | the threat table reflects my game's combat balance | F-7.7.10 | R-7.7.10 |
| US-7.7.10.2 | designer (P-5) | I want to set the aggro transfer threshold (110% melee, 130% ranged) | tank gameplay has tunable difficulty | F-7.7.10 | R-7.7.10 |
| US-7.7.10.3 | designer (P-5) | I want to configure targeting behavior per AI archetype (berserker, tactical, protector, coward) | different enemies require different strategies | F-7.7.10 | R-7.7.10 |
| US-7.7.10.4 | player (P-23) | I want enemies to attack me while my threat stays highest, with taunts generating burst threat | the tank role functions correctly | F-7.7.10 | R-7.7.10 |
| US-7.7.10.5 | player (P-23) | I want tactical enemies to target the lowest-HP party member | I must protect vulnerable allies | F-7.7.10 | R-7.7.10 |
| US-7.7.10.6 | player (P-23) | I want threat to decay when I move out of combat range | disengaging from combat is possible over time | F-7.7.10 | R-7.7.10 |
| US-7.7.10.7 | engine developer (P-26) | I want to implement a per-enemy threat table tracking hate from each attacker, selecting the highest-threat target | aggro targeting is data-driven | F-7.7.10 | R-7.7.10 |
| US-7.7.10.8 | engine developer (P-26) | I want to support threat modifiers on abilities (low threat heals, high threat taunts) | class roles can manage threat | F-7.7.10 | R-7.7.10 |
| US-7.7.10.9 | engine developer (P-26) | I want to expose the threat table to behavior trees for conditional target switching and to the HUD for tank threat indicators | threat data is accessible | F-7.7.10 | R-7.7.10 |
| US-7.7.10.10 | engine tester (P-27) | I want to verify that the AI always attacks the highest-threat target | basic threat table targeting works | F-7.7.10 | R-7.7.10 |
| US-7.7.10.11 | engine tester (P-27) | I want to verify that aggro does not transfer until the new threat exceeds the current target by the configured threshold | aggro is stable | F-7.7.10 | R-7.7.10 |
| US-7.7.10.12 | engine tester (P-27) | I want to verify that each AI archetype (berserker, tactical, protector, coward) selects the correct target based on its configured behavior | archetype targeting is correct. --- | F-7.7.10 | R-7.7.10 |

## F-7.7.11 — Animal Tracking and Hunting Behaviors

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.7.11.1 | designer (P-5) | I want to configure predator-prey pairs with hunting success rates | ecological balance is maintained across wildlife species | F-7.7.11 | R-7.7.11 |
| US-7.7.11.2 | designer (P-5) | I want to configure predator phases (patrol, detect, stalk, ambush, chase, kill, feed) | hunting behavior is rich and sequenced | F-7.7.11 | R-7.7.11 |
| US-7.7.11.3 | designer (P-5) | I want to configure prey defensive behaviors (flee, form protective circle, stand and fight) | different prey species respond differently | F-7.7.11 | R-7.7.11 |
| US-7.7.11.4 | player (P-23) | I want a wolf pack to stalk downwind, with drivers flushing prey toward ambushers | pack hunting feels realistic and coordinated | F-7.7.11 | R-7.7.11 |
| US-7.7.11.5 | player (P-23) | I want a deer herd to form protective circles with mothers shielding young when a predator approaches | prey behavior is as rich as predator behavior | F-7.7.11 | R-7.7.11 |
| US-7.7.11.6 | player (P-23) | I want a predator to give up the chase when the prey outruns it, managing stamina | hunts have realistic outcomes | F-7.7.11 | R-7.7.11 |
| US-7.7.11.7 | engine developer (P-26) | I want to implement the predator behavior state machine (patrol, detect, stalk, ambush, chase, kill, feed) | hunting phases are well-defined | F-7.7.11 | R-7.7.11 |
| US-7.7.11.8 | engine developer (P-26) | I want to implement coordinated pack hunting with driver and ambusher roles | pack members cooperate during hunts | F-7.7.11 | R-7.7.11 |
| US-7.7.11.9 | engine developer (P-26) | I want mobile to use simplified hunting (sight-only tracking, no pack coordination, no scent/footprint following) | wildlife simulation fits the mobile budget | F-7.7.11 | R-7.7.11 |
| US-7.7.11.10 | engine tester (P-27) | I want to run 100+ hunting simulations and verify the success rate matches the configured value within tolerance | ecological balance is correct | F-7.7.11 | R-7.7.11 |
| US-7.7.11.11 | engine tester (P-27) | I want to verify that a predator ends the chase when its stamina depletes | stamina management works | F-7.7.11 | R-7.7.11 |
| US-7.7.11.12 | engine tester (P-27) | I want to verify that pack members are assigned driver and ambusher roles and execute their assigned behaviors | coordination is correct | F-7.7.11 | R-7.7.11 |
