# R-7.7 -- Crowd Simulation Requirements

## R-7.7.1 Flocking Behaviors (Separation, Alignment, Cohesion)

The engine **SHALL** implement Reynolds flocking with three independently weighted forces --
separation (avoid crowding), alignment (match average heading), and cohesion (steer toward
average position) -- tunable per crowd archetype.

- **Derived from:** [F-7.7.1](../../features/ai/crowd-simulation.md)
- **Rationale:** Flocking produces emergent group behaviors (herds, columns, panicked mobs)
  from simple per-agent rules without hand-scripted group movement.
- **Verification:** Configure a flock of 50 agents with equal weights. Verify separation keeps
  minimum pairwise distance above 0.5 m. Verify alignment converges the heading variance to
  below 15 degrees within 100 ticks. Verify cohesion keeps the flock bounding radius below
  2x the initial radius. Adjust weights to zero out cohesion and verify the flock disperses.

## R-7.7.2 Flow Field Navigation

The engine **SHALL** generate a 2D grid-based flow field from a goal position using Dijkstra
propagation, where each cell stores a direction vector along the cheapest path, enabling
thousands of agents to sample the field at constant per-agent cost instead of running
individual A* queries.

- **Derived from:** [F-7.7.2](../../features/ai/crowd-simulation.md)
- **Rationale:** Per-agent A* is prohibitively expensive at crowd scale; flow fields amortize
  pathfinding cost across all agents sharing the same goal.
- **Verification:** Generate a flow field for a goal on a 100x100 grid with obstacles. Verify
  1000 agents all reach the goal by following the field vectors. Measure per-agent cost and
  verify it is constant regardless of agent count. Verify no agent follows a vector into an
  obstacle cell.

## R-7.7.3 Flow Field Streaming and Caching

The engine **SHALL** tile flow fields to match the world streaming grid, cache computed fields
keyed by (goal cell, cost layer version), invalidate stale fields when dynamic obstacles modify
the cost grid, and share cached fields across multiple goals with overlapping regions.

- **Derived from:** [F-7.7.3](../../features/ai/crowd-simulation.md)
- **Rationale:** Recomputing flow fields every tick is wasteful; caching with invalidation
  provides up-to-date fields at minimal cost.
- **Verification:** Generate a flow field and verify a subsequent request with the same goal
  and cost version returns the cached field (zero recomputation). Add a dynamic obstacle and
  verify the affected field is invalidated and recomputed. Create two goals in overlapping
  regions and verify the overlapping tiles share a single cached computation.

## R-7.7.4 Mass Entity Simulation

The engine **SHALL** process crowd NPCs as lightweight entities with minimal per-agent state
(position, velocity, archetype ID, flow field sample), driven entirely by flow fields and
flocking forces without behavior tree evaluation, supporting tens of thousands of simultaneous
agents within a bounded CPU footprint.

- **Derived from:** [F-7.7.4](../../features/ai/crowd-simulation.md)
- **Rationale:** Full AI evaluation per ambient NPC is too expensive at city scale; lightweight
  entities provide believable crowd density at minimal CPU cost.
- **Verification:** Spawn 10,000 crowd entities and measure per-tick CPU time. Verify the time
  scales linearly with entity count (no super-linear blowup). Verify no behavior tree is
  allocated or ticked for these entities. Verify each entity moves along flow field vectors
  with flocking corrections applied.

## R-7.7.5 AI Level of Detail (Processing Budget)

The engine **SHALL** assign each AI agent a LOD tier based on distance to the nearest player and
gameplay relevance, with high-LOD agents running full behavior trees, mid-LOD agents ticking at
reduced frequency, and low-LOD agents using flow-field-only movement, managed by a global budget
scheduler that maintains a stable frame rate.

- **Derived from:** [F-7.7.5](../../features/ai/crowd-simulation.md)
- **Rationale:** Not all NPCs require full AI processing; LOD tiers allocate CPU where it has
  the most gameplay impact while keeping total AI cost bounded.
- **Verification:** Place agents at distances of 10 m, 50 m, and 200 m from a player. Verify
  the 10 m agent is assigned high LOD (full BT tick every frame), the 50 m agent mid LOD
  (BT tick every 4th frame), and the 200 m agent low LOD (flow-field only). Set the global
  budget to 4 ms and verify total AI processing time does not exceed 4 ms by demoting agents
  to lower tiers as needed.

## R-7.7.6 Density Management

The engine **SHALL** monitor crowd density per spatial cell, enforce configurable density caps,
and redirect or despawn overflow agents when density exceeds the threshold to prevent
unrealistic pileups and maintain server performance.

- **Derived from:** [F-7.7.6](../../features/ai/crowd-simulation.md)
- **Rationale:** Uncapped density at chokepoints causes both visual implausibility (agent
  stacking) and server performance degradation from excessive collision processing.
- **Verification:** Set a density cap of 20 agents per cell. Spawn 30 agents targeting the
  same cell. Verify the cell never contains more than 20 agents at any tick. Verify overflow
  agents are either redirected to adjacent cells or despawned (for ambient entities). Verify
  the density check runs in O(1) per cell via spatial hashing.

## R-7.7.7 Knowledge Sharing and Event Propagation

The engine **SHALL** allow AI agents to broadcast perception and investigation events to
allied agents within a configurable communication radius. Recipients **SHALL** update their
perception state with shared knowledge. Knowledge **SHALL** decay using the same aging rules
as direct perception. At least 4 communication types **SHALL** be supported (alert,
investigation request, all-clear, deferred report).

- **Derived from:** [F-7.7.7](../../features/ai/crowd-simulation.md)
- **Rationale:** Knowledge sharing creates coordinated AI responses where guards alert each
  other, making stealth gameplay more challenging and rewarding.
- **Verification:** Have one guard spot an intruder; verify all guards within communication
  radius enter alert state within 2 ticks. Verify knowledge decays at the same rate as
  direct perception.

## R-7.7.8 Shared Awareness and Synchronized Group Reactions

The engine **SHALL** propagate alarm stimuli through animal/NPC groups with a spatial delay
producing a wave effect. At least 5 group reaction patterns **SHALL** be supported (bird
scatter, fish school turn, herd stampede, civilian panic, guard formation). Group members
**SHALL** maintain cohesion during flight and reassemble after the threat clears.

- **Derived from:** [F-7.7.8](../../features/ai/crowd-simulation.md)
- **Rationale:** Synchronized group reactions (bird flocks scattering, herds stampeding)
  create immersive wildlife and crowd behavior.
- **Verification:** Trigger an alarm near a flock of 50 birds; verify they scatter with a
  visible wave delay from nearest to farthest. Verify they regroup at a safe distance
  within 30 seconds.

## R-7.7.9 Faction-Based Behavioral Relationships

The engine **SHALL** determine AI behavior based on a runtime faction relationship matrix
with at least 6 relationship types (aggressive, hostile, wary, neutral, friendly, allied).
Relationships **SHALL** be modifiable at runtime by gameplay systems. Individual NPC
overrides **SHALL** take precedence over faction defaults.

- **Derived from:** [F-7.7.9](../../features/ai/crowd-simulation.md)
- **Rationale:** Dynamic faction relationships enable emergent political gameplay where
  player actions reshape the world's social fabric.
- **Verification:** Set faction A to hostile toward the player; verify faction A NPCs attack
  on sight. Complete a quest; shift faction A to friendly; verify the same NPCs no longer
  attack. Override one NPC to hostile; verify only that NPC attacks.

## R-7.7.10 Threat Table and Aggro Targeting

The engine **SHALL** maintain a per-enemy threat table tracking hate from damage, healing,
taunts, debuffs, and proximity. The highest-threat target **SHALL** be attacked. Aggro
transfer **SHALL** require exceeding current target's threat by a configurable threshold
(default: 110% melee, 130% ranged). Threat **SHALL** decay over time out of combat range.
At least 4 AI targeting archetypes **SHALL** be supported (berserker, tactical, protector,
coward).

- **Derived from:** [F-7.7.10](../../features/ai/crowd-simulation.md)
- **Rationale:** Threat-based targeting is the foundation of MMO tank/healer/DPS combat
  roles and creates strategic depth.
- **Verification:** Have a tank generate 1000 threat; have a DPS generate 1050 threat;
  verify aggro does NOT transfer (below 110% threshold). DPS generates 1100 threat; verify
  aggro transfers. Verify threat decays when the source moves out of range.

## R-7.7.11 Animal Tracking and Hunting Behaviors

The engine **SHALL** provide predator-prey AI with: stalking (approach using cover,
downwind), ambush (wait at prey paths/water), chase (with stamina), pack hunting
(coordinated driver/ambusher roles), prey flee (stamina-aware, terrain-aware), herd defense
(protective circles, stand-and-fight), and evidence-based tracking (scent, footprints,
visual). Hunting success rate **SHALL** be configurable per predator-prey pair.

- **Derived from:** [F-7.7.11](../../features/ai/crowd-simulation.md)
- **Rationale:** Realistic wildlife hunting creates immersive ecosystems and meaningful
  survival gameplay.
- **Verification:** Spawn a wolf pack and deer herd; verify wolves stalk downwind,
  coordinate a chase, and achieve kills within the configured success rate over 100 trials.
  Verify deer flee as a herd with protective behavior.
