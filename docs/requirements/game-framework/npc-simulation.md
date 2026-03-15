# R-13.19 -- NPC Simulation Requirements

## R-13.19.1 NPC Relationship and Affinity System

The engine **SHALL** maintain per-NPC numeric affinity values (friendship, romance, trust, fear) as
ECS components, modifying them from player actions, dialogue choices, quest outcomes, and NPC-specific
gift preference tables, and advancing relationship tiers that gate dialogue options, quests, and perks.

- **Derived from:** [F-13.19.1](../../features/game-framework/npc-simulation.md)
- **Rationale:** Relationship tracking with data-driven preference tables enables socially rich
  RPG systems where player choices meaningfully shape NPC interactions and story progression.
- **Verification:** Configure two NPCs with distinct gift preference tables (loved, liked, neutral,
  disliked, hated). Give each NPC loved and hated items; verify affinity shifts match table values
  within tolerance. Advance affinity past tier thresholds and confirm new dialogue options and quests
  unlock at each tier. Save and reload; verify affinity values and tier persist.

## R-13.19.2 NPC Personality and Emotion Model

The engine **SHALL** assign per-NPC personality traits from data tables and compute dynamic emotional
states (happy, angry, afraid, grateful, jealous, sad) that evolve from witnessed events and dialogue,
decay toward personality-determined baselines over time, and influence animations, dialogue tone, and
available interactions.

- **Derived from:** [F-13.19.2](../../features/game-framework/npc-simulation.md)
- **Rationale:** Personality and emotion give NPCs behavioral depth, making interactions feel
  varied and responsive to player actions rather than static and scripted.
- **Verification:** Configure an NPC with high courage and low compassion. Trigger a threatening
  event and verify the NPC's fear emotion stays below that of a low-courage NPC in the same
  scenario. Verify emotion decays to baseline within the configured time window. Confirm the
  animation state machine receives the current emotional state and selects the matching animation.

## R-13.19.3a NPC Deed Memory

The engine **SHALL** record player and NPC actions witnessed within perception range as deed
memories with emotional weight and time-based decay, judging them against personality values
to adjust affinity, and evicting memories when they decay below a minimum weight threshold.

- **Derived from:** [F-13.19.3a](../../features/game-framework/npc-simulation.md)
- **Rationale:** Deed memory gives NPCs persistent awareness of player behavior, enabling
  consequences for actions taken in an NPC's presence.
- **Verification:** Perform a positive deed (rescue) witnessed by NPC-A. Verify NPC-A's
  affinity increases. Wait for memory decay and confirm emotional weight decreases to zero
  within the configured decay period. Verify the memory is evicted after reaching minimum
  weight.

## R-13.19.3b Gossip Propagation Network

The engine **SHALL** propagate deed memories between NPCs during social interactions as gossip
with degrading accuracy and emotional weight per retelling, at a configurable propagation
rate per NPC archetype.

- **Derived from:** [F-13.19.3b](../../features/game-framework/npc-simulation.md)
- **Rationale:** Gossip propagation creates emergent social dynamics where information about
  player behavior spreads organically through the NPC social network.
- **Verification:** Allow NPC-A (who witnessed a deed) to gossip with NPC-B; verify NPC-B's
  affinity increases by a lesser amount. Allow NPC-B to gossip with NPC-C; verify NPC-C's
  affinity increase is further reduced. Verify gossip frequency matches the configured
  archetype rate.

## R-13.19.3c Emergent Reputation Aggregation

The engine **SHALL** aggregate per-NPC affinity values across social groups to produce
emergent reputation scores that gate group-wide behaviors such as shop discounts, hostile
reactions, and quest availability.

- **Derived from:** [F-13.19.3c](../../features/game-framework/npc-simulation.md)
- **Rationale:** Aggregated reputation rewards consistent positive behavior across a
  community and creates meaningful social consequences for player actions.
- **Verification:** Improve affinity with several NPCs in a village through gossip
  propagation. Verify the village-wide reputation score crosses a threshold and unlocks
  a group-wide behavior (e.g., shop discount). Verify negative deeds reduce the aggregate
  score accordingly.

## R-13.19.4a Schedule Data Model

The engine **SHALL** define NPC daily schedules as data tables mapping time-of-day ranges to
locations, activities, arrival animations, idle behaviors, interaction availability flags, and
variation rules by day of the week and season.

- **Derived from:** [F-13.19.4a](../../features/game-framework/npc-simulation.md)
- **Rationale:** A structured data model enables designers to author diverse NPC routines
  through the visual editor without code changes.
- **Verification:** Configure a shopkeeper NPC with a schedule: home (6:00-8:00), shop
  (8:00-17:00), tavern (17:00-20:00), home (20:00-6:00). Verify the schedule data
  serializes and deserializes correctly. Change the day of the week and verify the
  alternate schedule entry activates.

## R-13.19.4b Schedule Execution

The engine **SHALL** drive NPC movement between schedule locations via the pathfinding system
when the in-game clock crosses schedule boundaries, playing configured arrival animations and
entering idle behaviors at each destination.

- **Derived from:** [F-13.19.4b](../../features/game-framework/npc-simulation.md)
- **Rationale:** Executed schedules make the game world feel alive with NPCs that follow
  believable daily routines rather than standing idle at fixed positions.
- **Verification:** Advance the in-game clock and verify the NPC navigates to each location
  at the correct time. Verify arrival animations play on reaching the destination. Verify
  NPCs that miss a transition skip to the current slot.

## R-13.19.4c Schedule-Gated Interactions

The engine **SHALL** gate NPC interactions based on the current schedule slot, exposing
availability status to the dialogue system and UI so players can see why an NPC is
unavailable.

- **Derived from:** [F-13.19.4c](../../features/game-framework/npc-simulation.md)
- **Rationale:** Schedule-gated interactions encourage players to learn NPC routines and
  plan activities around the in-game clock, adding depth to the simulation.
- **Verification:** Attempt to trade with a shopkeeper outside work hours and verify the
  interaction is rejected with a reason displayed. Verify trade is available during
  configured shop hours.

## R-13.19.5 Ambient Bark System

The engine **SHALL** trigger contextual one-liner NPC dialogue (barks) from priority-sorted,
cooldown-gated bark pools filtered by proximity, world events, combat state, time of day, and
weather, displaying barks as floating text bubbles with optional audio without entering a full
conversation or pausing gameplay.

- **Derived from:** [F-13.19.5](../../features/game-framework/npc-simulation.md)
- **Rationale:** Ambient barks give NPCs incidental personality and contextual awareness without
  requiring the overhead of full dialogue interactions.
- **Verification:** Configure a guard NPC with two barks: low-priority weather comment
  (60 s cooldown) and high-priority enemy warning (5 s cooldown). Trigger both conditions
  simultaneously; verify the high-priority bark plays. Trigger the low-priority bark and attempt
  to trigger it again within the cooldown window; verify it is suppressed. Confirm barks render
  as floating text and do not pause the game loop.

## R-13.19.6 Threat and Aggro Table System

The engine **SHALL** maintain per-enemy threat tables accumulating hate from damage, healing, taunts,
debuffs, and proximity, with configurable per-ability threat modifiers and time-based out-of-combat
decay, targeting the highest-threat actor and exposing the threat table to AI behavior trees for
target-switching decisions.

- **Derived from:** [F-13.19.6](../../features/game-framework/npc-simulation.md)
- **Rationale:** Threat tables enable tank/healer/DPS role differentiation in combat by giving
  players explicit control over enemy targeting through ability choice.
- **Verification:** Spawn an enemy and three player units: tank (taunt ability), healer, and DPS.
  Have all three engage; verify the enemy targets the highest-threat unit. Activate the tank's
  taunt; verify the tank becomes the target. Have the healer generate healing threat; verify
  threat distributes among engaged enemies. Disengage the DPS for the configured decay duration;
  verify its threat drops to zero.

## Non-Functional Requirements

### NFR-13.19.1 NPC Simulation Scalability

The NPC simulation system **SHALL** support up to 200 scheduled NPCs in the active area with
relationship tracking, personality evaluation, emotion updates, and bark triggers completing
within a total budget of 4ms per frame. Gossip propagation **SHALL** be amortized across
frames, processing no more than 10 gossip events per frame.

- **Rationale:** Towns and cities may contain hundreds of NPCs. Simulation must scale without
  degrading frame rate in populated areas.
- **Verification:** Spawn 200 NPCs with daily schedules, personality, and emotion. Measure
  total NPC simulation time per frame. Verify it stays under 4ms. Trigger 50 simultaneous
  gossip events and verify they amortize across frames at 10 per frame.

### NFR-13.19.2 Deed Memory Storage Efficiency

Per-NPC deed memory **SHALL** consume no more than 256 bytes per deed entry. Total deed
memory across all NPCs **SHALL NOT** exceed 4 MB at any time, enforced by evicting the
oldest low-weight memories when the limit is reached.

- **Rationale:** Unbounded memory accumulation from deed tracking could exhaust memory in
  long play sessions. Capped storage with eviction prevents memory growth.
- **Verification:** Generate 1,000 deeds across 200 NPCs. Measure total memory consumption.
  Verify it stays under 4 MB. Generate additional deeds beyond the cap and verify the oldest
  entries are evicted.
