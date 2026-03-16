# R-7.7 -- Crowd Simulation Requirements

## Flocking

### R-7.7.1 Flocking Behaviors (Separation, Alignment, Cohesion)

The engine **SHALL** implement Reynolds flocking with separation, alignment, and cohesion forces,
each with per-archetype tunable weights, maintaining minimum pairwise distance above a configurable
threshold (default 0.5 m).

- **Derived from:**
  [F-7.7.1](../../features/ai/crowd-simulation.md)
- **Rationale:** Flocking is the foundation for natural group movement in wildlife, herds, and
  ambient crowds; tunable weights enable varied group personalities.
- **Verification:** Configure a flock of 50 agents with default weights. Verify minimum pairwise
  distance stays above 0.5 m throughout a 60-second simulation. Adjust cohesion weight to 0.0 and
  verify the flock disperses. Adjust separation weight to 0.0 and verify agents cluster below the
  minimum distance threshold.

## Flow Fields

### R-7.7.2 Flow Field Navigation

The engine **SHALL** generate 2D grid-based flow fields from goal positions using Dijkstra
propagation, where each cell stores a direction vector and agents sample the field at constant O(1)
per-agent cost regardless of agent count.

- **Derived from:**
  [F-7.7.2](../../features/ai/crowd-simulation.md)
- **Rationale:** Individual A* queries per crowd agent do not scale to thousands of entities; flow
  fields amortize the cost of pathfinding across the entire crowd.
- **Verification:** Generate a flow field for a goal position. Verify 1,000 agents following the
  field all converge on the goal. Benchmark per-agent sampling cost at 1K, 5K, and 10K agents and
  verify it remains constant (within 5% variance).

### R-7.7.3 Flow Field Streaming and Caching

The engine **SHALL** tile flow fields to match the world streaming grid and cache computed fields
keyed by (goal cell, cost layer version), invalidating stale fields when dynamic obstacles modify
the cost grid.

- **Derived from:**
  [F-7.7.3](../../features/ai/crowd-simulation.md)
- **Rationale:** Recomputing flow fields every tick is too expensive; caching by goal and cost
  version avoids redundant computation while invalidation ensures correctness after obstacle
  changes.
- **Verification:** Request the same flow field twice with identical goal and cost version. Verify
  the second request returns the cached result with zero recomputation. Modify a dynamic obstacle
  and verify the affected field is invalidated and recomputed on next request.

## Mass Simulation

### R-7.7.4 Mass Entity Simulation

The engine **SHALL** process crowd NPCs as lightweight entities with minimal per-agent state
(position, velocity, archetype ID, flow field sample), driven by flow fields and flocking forces
without full behavior tree evaluation, supporting platform-scaled crowd counts.

- **Derived from:**
  [F-7.7.4](../../features/ai/crowd-simulation.md)
- **Rationale:** Full AI evaluation per crowd entity is too expensive for tens of thousands of
  ambient inhabitants; minimal-state entities keep CPU cost bounded.
- **Verification:** Simulate 10,000 crowd entities and verify CPU usage scales linearly with count
  (2x entities = 2x cost, within 10% tolerance). Verify per-entity memory does not exceed 64 bytes
  for low-LOD crowd entities.

### R-7.7.5 AI Level of Detail (Processing Budget)

The engine **SHALL** assign each AI agent a LOD tier based on distance to the nearest player and
gameplay relevance, with a global budget scheduler distributing available CPU across tiers
(high-LOD: full BT + perception, mid-LOD: reduced tick rate, low-LOD: flow field only) to maintain
stable frame rate.

- **Derived from:**
  [F-7.7.5](../../features/ai/crowd-simulation.md)
- **Rationale:** Not all agents need full AI evaluation every tick; LOD tiers ensure CPU is spent on
  gameplay-critical agents while distant ones use cheap movement.
- **Verification:** Place agents at varying distances from the player. Verify the closest agents are
  assigned high-LOD and tick at full rate. Verify distant agents are assigned low-LOD and use
  flow-field-only movement. Verify total AI processing stays within the configured frame budget.

### R-7.7.6 Density Management

The engine **SHALL** monitor crowd density per spatial cell and enforce configurable caps,
redirecting overflow agents to alternative routes or despawning ambient crowd entities when density
exceeds the threshold.

- **Derived from:**
  [F-7.7.6](../../features/ai/crowd-simulation.md)
- **Rationale:** Unrestricted crowd density at chokepoints causes agent pileups that break immersion
  and degrade simulation performance.
- **Verification:** Set a per-cell density cap of 16. Spawn 32 agents in a single cell. Verify 16
  agents are redirected or despawned. Verify no cell ever exceeds the configured cap during a
  60-second stress test.

## Social and Group Behaviors

### R-7.7.7 Knowledge Sharing and Event Propagation

The engine **SHALL** broadcast knowledge events (alert, investigation request, all-clear, report)
from AI agents to all allied agents within a configurable communication radius, with recipients
updating their perception state using the faction affinity system and shared knowledge decaying
under the same memory aging rules as direct perception.

- **Derived from:**
  [F-7.7.7](../../features/ai/crowd-simulation.md)
- **Rationale:** Coordinated AI response (guards alerting each other) requires knowledge sharing;
  decay prevents stale information from persisting indefinitely.
- **Verification:** Have a guard detect an intruder and verify all allied guards within the
  communication radius enter alert state within 2 ticks. Verify guards outside the radius remain
  unaware. Verify shared knowledge decays at the same rate as directly perceived stimuli.

### R-7.7.8 Shared Awareness and Synchronized Group Reactions

The engine **SHALL** propagate alarm events through groups with spatial delay (wave effect from
nearest to farthest member) and support configurable reaction patterns (scatter, school turn,
stampede, panic, formation), with group reassembly at rally points after threats clear.

- **Derived from:**
  [F-7.7.8](../../features/ai/crowd-simulation.md)
- **Rationale:** Instant uniform group reactions look artificial; spatial delay creates
  natural-looking wave responses that differ by creature type.
- **Verification:** Trigger an alarm near one member of a 20-agent group. Verify the nearest member
  reacts first and the farthest reacts last, with delay proportional to distance. Verify the
  configured reaction pattern (e.g., scatter) activates. Verify the group reassembles at the rally
  point after the threat clears.

### R-7.7.9 Faction-Based Behavioral Relationships

The engine **SHALL** determine AI behavior from a runtime faction affinity matrix supporting 6
relationship types (aggressive, hostile, wary, neutral, friendly, allied), with individual NPC
overrides taking precedence over faction defaults and runtime modification of relationships via
gameplay systems.

- **Derived from:**
  [F-7.7.9](../../features/ai/crowd-simulation.md)
- **Rationale:** NPCs must behave differently toward different factions (attack enemies, trade with
  friends); runtime modification supports reputation and quest systems.
- **Verification:** Configure faction A as hostile to B and friendly to C. Verify agent A attacks B
  on sight and trades with C. Set an individual override making one A agent friendly to B. Verify
  that specific agent does not attack B while all other A agents still do. Modify the faction matrix
  at runtime and verify behavior changes on the next tick.

### R-7.7.10 Threat Table and Aggro Targeting

The engine **SHALL** track per-entity threat generated by damage, healing, taunts, debuffs, and
proximity in a threat table, with the highest-threat target selected for attack and aggro transfer
occurring when a new entity exceeds the current target's threat by a configurable threshold (default
110% melee, 130% ranged).

- **Derived from:**
  [F-7.7.10](../../features/ai/crowd-simulation.md)
- **Rationale:** Threat-based targeting enables the tank/healer/DPS role system by giving players
  agency over which character the enemy attacks.
- **Verification:** Have a tank deal 100 threat and a DPS deal 105 threat. Verify aggro does NOT
  transfer at 105% (below 110% threshold). Have the DPS deal 111 threat and verify aggro transfers.
  Verify a taunt ability generates instant high-threat and captures aggro. Verify threat decays when
  the source leaves combat range.

### R-7.7.11 Animal Tracking and Hunting Behaviors

The engine **SHALL** provide predator AI with stalk, ambush, and chase behaviors using perception
senses (sight, smell, hearing), prey AI with flee and herd defense behaviors, and pack hunting with
driver/ambusher coordination, with configurable hunting success rate per predator-prey pair.

- **Derived from:**
  [F-7.7.11](../../features/ai/crowd-simulation.md)
- **Rationale:** Realistic wildlife simulation requires predator-prey dynamics with tracking, pack
  coordination, and configurable ecological balance.
- **Verification:** Place a predator and prey in range. Verify the predator detects the prey via
  sight and initiates stalking. Verify the prey flees when it detects the predator. Configure a 50%
  hunting success rate and verify the actual rate matches within 10% over 100 trials. Verify pack
  hunting coordinates drivers and ambushers.

---

## User Story Traceability

User stories for this domain are maintained in
[user-stories/ai/crowd-simulation.md](../../user-stories/ai/crowd-simulation.md).
Requirements in this document are derived from those
user stories.
